<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { Terminal } from "@xterm/xterm";
  import { FitAddon } from "@xterm/addon-fit";
  import "@xterm/xterm/css/xterm.css";
  import { launchApp, termInput, termResize, onTermOutput, onTermExit, runLogBytes } from "./api";
  import { onThemeChange } from "./theme";
  import { scrollFade } from "./scrollfade";
  import { xtermColors } from "./ansi";
  import { t } from "./i18n.svelte";
  import Icon from "./Icon.svelte";
  import { writeText, readText } from "@tauri-apps/plugin-clipboard-manager";
  import { type UnlistenFn } from "@tauri-apps/api/event";

  // autoLaunch: whether mounting this tab should start the app. Reopening a
  // closed tab just to view its log (see App.svelte's handleSelect) passes
  // false - the log is backfilled from disk instead, no process is started.
  let { id, active, autoLaunch = true }: { id: string; active: boolean; autoLaunch?: boolean } =
    $props();

  // Replace each ESC[2J (clear screen) in `arr` - only the first when `all` is false - with a
  // screenful of newlines followed by the clear. The newlines scroll the visible text into
  // scrollback, so the clear that follows blanks an empty screen and nothing is lost.
  function clearsToScrollback(arr: Uint8Array, rows: number, all: boolean): Uint8Array {
    const hits: number[] = [];
    for (let i = 0; i + 3 < arr.length; i++) {
      if (arr[i] === 0x1b && arr[i + 1] === 0x5b && arr[i + 2] === 0x32 && arr[i + 3] === 0x4a) {
        hits.push(i);
        if (!all) break;
      }
    }
    if (hits.length === 0) return arr;
    const scroll = new Uint8Array(rows * 2);
    for (let r = 0; r < rows; r++) scroll.set([0x0d, 0x0a], r * 2);
    const out = new Uint8Array(arr.length + hits.length * scroll.length);
    let src = 0;
    let dst = 0;
    for (const h of hits) {
      out.set(arr.subarray(src, h), dst);
      dst += h - src;
      out.set(scroll, dst);
      dst += scroll.length;
      src = h;
    }
    out.set(arr.subarray(src), dst);
    return out;
  }

  let el: HTMLDivElement;
  let term: Terminal;
  let fit: FitAddon;
  let ro: ResizeObserver | null = null;
  let unlistenOut: UnlistenFn | null = null;
  let unlistenExit: UnlistenFn | null = null;
  let cleanupMouse: (() => void) | null = null;
  let fitTimer: ReturnType<typeof setTimeout> | null = null;
  // One-shot re-fit timers (post-mount settle + on-activate), tracked so they can be
  // cancelled if the view unmounts first (doFit on a disposed term would otherwise run).
  let settleTimer: ReturnType<typeof setTimeout> | null = null;
  let activeFitTimer: ReturnType<typeof setTimeout> | null = null;
  // Set in onDestroy. onMount is async, so the view can unmount mid-await; anything
  // registered (or launched) after that point would leak or run against a dead view.
  let destroyed = false;
  let lastCols = 0;
  let lastRows = 0;
  let unsubTheme: (() => void) | null = null;
  // Brief tick on the floating copy button so the click has visible feedback.
  let copied = $state(false);
  let copiedTimer: ReturnType<typeof setTimeout> | null = null;

  // Copy the entire scrollback. selectAll() then getSelection() is the only way
  // to read xterm's buffer as text with its line-wrapping resolved.
  async function copyAll() {
    if (!term) return;
    const had = term.getSelection();
    term.selectAll();
    const all = term.getSelection();
    term.clearSelection();
    if (!all.trim() && !had) return;
    await writeText(all.replace(/\s+$/, "") + "\n").catch(() => {});
    copied = true;
    if (copiedTimer) clearTimeout(copiedTimer);
    copiedTimer = setTimeout(() => (copied = false), 1200);
  }

  // xterm has no access to CSS variables, so read the themed fg from :root.
  // xterm's OWN background is fully transparent; the single translucent tint
  // layer for the terminal region is the .term box behind the canvas (it also
  // covers .term's padding, so no untinted gap shows above/left of the grid).
  // Alphas don't stack (which darkened the terminal) and the text keeps an
  // opaque-enough backing to stay legible over the desktop.
  function termTheme() {
    const cs = getComputedStyle(document.documentElement);
    const fg = cs.getPropertyValue("--text").trim() || "#c9d1d9";
    // The resolved theme id lives on <html data-theme> (theme.ts sets it); use it
    // to theme the 16 ANSI colors so program output matches the palette.
    const id = document.documentElement.dataset.theme || "dark";
    return {
      background: "rgba(0,0,0,0)",
      foreground: fg,
      cursor: fg,
      // xterm's default selection is translucent white, which vanishes on a
      // light palette; tint it with the theme's own text color instead.
      selectionBackground: /^#[0-9a-f]{6}$/i.test(fg) ? `${fg}40` : "rgba(128,128,128,0.35)",
      ...xtermColors(id),
    };
  }
  function onSchemeChange() {
    if (term) term.options.theme = termTheme();
  }

  function doFit() {
    if (!term || !active) return;
    try {
      fit.fit();
      // Only push a resize to the PTY when the grid actually changed - avoids
      // churn and the ResizeObserver -> resize -> observer feedback loop that
      // could leave the viewport (and its scrollbar) in a broken state.
      if (term.cols !== lastCols || term.rows !== lastRows) {
        lastCols = term.cols;
        lastRows = term.rows;
        termResize(id, term.cols, term.rows);
      }
    } catch {
      /* element not laid out yet */
    }
  }

  // Debounce resize so fit() runs once after the layout settles.
  function scheduleFit() {
    if (fitTimer) clearTimeout(fitTimer);
    fitTimer = setTimeout(doFit, 60);
  }

  onMount(async () => {
    term = new Terminal({
      fontFamily: "Cascadia Code, Consolas, ui-monospace, monospace",
      fontSize: 13,
      cursorBlink: true,
      scrollback: 10000,
      allowTransparency: true,
      theme: termTheme(),
    });
    fit = new FitAddon();
    term.loadAddon(fit);
    term.open(el);
    fit.fit();

    unsubTheme = onThemeChange(onSchemeChange);

    term.onData((d) => termInput(id, d));

    // Linux-style clipboard: copy on select (fires on mouse release), then deselect;
    // middle-click pastes.
    //
    // The mouseup listener is on `window`, not on `el`: dragging a selection to the
    // edge (e.g. selecting the whole terminal) usually releases the button outside
    // the terminal box, and an element-scoped listener never fires then - the
    // selection stayed uncopied. Window scope catches the release wherever it lands;
    // we only act when the drag STARTED inside this terminal.
    let dragging = false;
    function onMouseDown(e: MouseEvent) {
      if (e.button === 0) dragging = true;
    }
    el.addEventListener("mousedown", onMouseDown);
    window.addEventListener("mouseup", onMouseUp);

    async function onMouseUp(e: MouseEvent) {
      if (e.button === 1) {
        // Middle click -> paste. Only when it happened over this terminal.
        if (!el.contains(e.target as Node)) return;
        e.preventDefault();
        const text = await readText().catch(() => "");
        if (text) termInput(id, text);
        return;
      }
      const startedHere = dragging;
      dragging = false;
      if (!startedHere && !el.contains(e.target as Node)) return;
      const sel = term.getSelection();
      if (sel) {
        await writeText(sel).catch(() => {});
        term.clearSelection();
      }
    }
    cleanupMouse = () => {
      el.removeEventListener("mousedown", onMouseDown);
      window.removeEventListener("mouseup", onMouseUp);
    };

    // While the tab backfills the log, live chunks are held here so they land after
    // it, not interleaved with it. Each carries its log offset (`end`) so the part the log
    // read already contained can be dropped instead of written twice.
    let held: { arr: Uint8Array; end?: number }[] | null = [];
    // A fresh run's terminal opens with a clear-screen, which would wipe the earlier output and
    // the restart divider just backfilled. Until the first one goes by, turn it into a scroll
    // so that history moves up into scrollback instead of vanishing.
    let startupClearPending = autoLaunch;
    const liveWrite = (arr: Uint8Array) => {
      if (startupClearPending) {
        const kept = clearsToScrollback(arr, term.rows, false);
        if (kept !== arr) startupClearPending = false;
        term.write(kept);
      } else term.write(arr);
    };
    const uOut = await onTermOutput((o) => {
      if (o.id !== id) return;
      const bin = atob(o.data);
      const arr = new Uint8Array(bin.length);
      for (let i = 0; i < bin.length; i++) arr[i] = bin.charCodeAt(i);
      if (held) held.push({ arr, end: o.end });
      else liveWrite(arr);
    });
    // Unmounted while awaiting: unlisten now (onDestroy already ran with a null handle).
    if (destroyed) return uOut();
    unlistenOut = uOut;

    const uExit = await onTermExit((exitId) => {
      if (exitId === id) term.write(`\r\n\x1b[90m${t("term.processExited")}\x1b[0m\r\n`);
    });
    if (destroyed) return uExit();
    unlistenExit = uExit;

    if (destroyed) return;
    if (autoLaunch) {
      // Launch the app sized to the terminal we just laid out - unless we've
      // since unmounted (e.g. the user switched away before this resolved).
      try {
        await launchApp(id, term.cols, term.rows);
      } catch (err) {
        // Most likely "already running" - just show a note.
        term.write(`\r\n\x1b[33m${err}\x1b[0m\r\n`);
      }
    }
    // Backfill this session's log: earlier runs of this app (a restart rebuilds the tab, which
    // would otherwise clear them) and, for a relaunch, the "restarted" divider launch_app wrote
    // to the log. A view-only reopen starts nothing; live output keeps appending past this.
    {
      try {
        const b64 = await runLogBytes(id);
        let logLen = 0;
        if (b64) {
          const bin = atob(b64);
          const arr = new Uint8Array(bin.length);
          for (let i = 0; i < bin.length; i++) arr[i] = bin.charCodeAt(i);
          logLen = arr.length;
          if (!destroyed) term.write(clearsToScrollback(arr, term.rows, true));
        }
        // Replay what arrived live meanwhile, minus whatever the log read already covered.
        const pending = held ?? [];
        held = null;
        for (const { arr, end } of pending) {
          if (destroyed) break;
          if (end === undefined) liveWrite(arr);
          else if (end > logLen) liveWrite(arr.subarray(Math.max(0, logLen - (end - arr.length))));
        }
      } catch (err) {
        held = null;
        if (!destroyed) term.write(`\r\n\x1b[33m${err}\x1b[0m\r\n`);
      }
    }

    if (destroyed) return;
    ro = new ResizeObserver(() => scheduleFit());
    ro.observe(el);

    // Re-fit once styles (incl. the reserved scrollbar gutter) have settled.
    settleTimer = setTimeout(() => doFit(), 120);
  });

  $effect(() => {
    if (active) {
      if (activeFitTimer) clearTimeout(activeFitTimer);
      activeFitTimer = setTimeout(doFit, 0);
    }
  });

  onDestroy(() => {
    destroyed = true;
    ro?.disconnect();
    if (copiedTimer) clearTimeout(copiedTimer);
    if (fitTimer) clearTimeout(fitTimer);
    if (settleTimer) clearTimeout(settleTimer);
    if (activeFitTimer) clearTimeout(activeFitTimer);
    unsubTheme?.();
    cleanupMouse?.();
    unlistenOut?.();
    unlistenExit?.();
    term?.dispose();
  });
</script>

<div class="term-wrap" style:display={active ? "block" : "none"}>
  <div class="term" bind:this={el} use:scrollFade></div>
  <button class="copy-all" onclick={copyAll} title={copied ? t("term.copied") : t("term.copyAll")} aria-label={t("term.copyAll")}>
    <Icon name={copied ? "check" : "copy"} size={14} />
  </button>
</div>

<style>
  .term-wrap {
    position: relative;
    width: 100%;
    height: 100%;
  }

  /* Floating over the terminal's top-right corner, inside the scrollbar gutter's
     left edge so it never covers the bar. Faint until the terminal is hovered. */
  .copy-all {
    position: absolute;
    top: 6px;
    right: 16px;
    z-index: 2;
    display: grid;
    place-items: center;
    width: 24px;
    height: 24px;
    padding: 0;
    border: 1px solid color-mix(in srgb, var(--border) 70%, transparent);
    border-radius: 6px;
    background: color-mix(in srgb, var(--bg) 85%, transparent);
    color: var(--text-dim, var(--text));
    cursor: pointer;
    opacity: 0;
    transition: opacity 120ms ease, color 120ms ease, border-color 120ms ease;
  }
  .term-wrap:hover .copy-all,
  .copy-all:focus-visible {
    opacity: 0.75;
  }
  .copy-all:hover {
    opacity: 1;
    color: var(--text);
    border-color: var(--border-strong, var(--border));
  }

  .term {
    width: 100%;
    height: 100%;
    /* No right padding: the scrollbar sits flush at the container's right edge, so
       its paint position and interactive hitbox stay aligned. */
    padding: 6px 0 6px 8px;
    box-sizing: border-box;
    /* This box is the single translucent tint layer for the terminal region:
       it backs xterm's transparent canvas AND fills the padding, so no untinted
       gap shows above/left of the grid. xterm's own bg stays transparent so the
       alphas don't stack (which darkened the terminal). */
    background: color-mix(in srgb, var(--bg) calc(var(--app-alpha) * 100%), transparent);
  }

  /* Defining an explicit ::-webkit-scrollbar width forces a classic, space-reserving
     scrollbar in WebView2 (not an overlay), which the fit addon accounts for. No
     scrollbar-gutter and no thumb border - both split the visible bar from its hitbox. */
  :global(.xterm .xterm-viewport)::-webkit-scrollbar {
    width: 10px;
  }
  :global(.xterm .xterm-viewport)::-webkit-scrollbar-track {
    background: transparent;
  }
  /* Match the app-list scrollbar (width/color) and follow --scroll-alpha, which
     .term sets from transparency and raises to 1 on hover. */
  :global(.xterm .xterm-viewport)::-webkit-scrollbar-thumb {
    background: color-mix(in srgb, var(--border) calc(var(--scroll-alpha) * 100%), transparent);
    border-radius: 5px;
  }
  :global(.xterm .xterm-viewport)::-webkit-scrollbar-thumb:hover {
    background: color-mix(in srgb, var(--border-strong) calc(var(--scroll-alpha) * 100%), transparent);
  }
</style>
