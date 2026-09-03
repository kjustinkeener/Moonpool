<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { Terminal } from "@xterm/xterm";
  import { FitAddon } from "@xterm/addon-fit";
  import "@xterm/xterm/css/xterm.css";
  import { launchApp, termInput, termResize, onTermOutput, onTermExit } from "./api";
  import { onThemeChange } from "./theme";
  import { scrollFade } from "./scrollfade";
  import { ansiFor } from "./ansi";
  import { t } from "./i18n.svelte";
  import { writeText, readText } from "@tauri-apps/plugin-clipboard-manager";
  import { type UnlistenFn } from "@tauri-apps/api/event";

  let { id, active }: { id: string; active: boolean } = $props();

  let el: HTMLDivElement;
  let term: Terminal;
  let fit: FitAddon;
  let ro: ResizeObserver | null = null;
  let unlistenOut: UnlistenFn | null = null;
  let unlistenExit: UnlistenFn | null = null;
  let cleanupMouse: (() => void) | null = null;
  let fitTimer: ReturnType<typeof setTimeout> | null = null;
  let lastCols = 0;
  let lastRows = 0;
  let unsubTheme: (() => void) | null = null;

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
      ...ansiFor(id),
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
    el.addEventListener("mouseup", onMouseUp);

    async function onMouseUp(e: MouseEvent) {
      if (e.button === 1) {
        // Middle click -> paste.
        e.preventDefault();
        const text = await readText().catch(() => "");
        if (text) termInput(id, text);
        return;
      }
      const sel = term.getSelection();
      if (sel) {
        await writeText(sel).catch(() => {});
        term.clearSelection();
      }
    }
    cleanupMouse = () => el.removeEventListener("mouseup", onMouseUp);

    unlistenOut = await onTermOutput((o) => {
      if (o.id !== id) return;
      const bin = atob(o.data);
      const arr = new Uint8Array(bin.length);
      for (let i = 0; i < bin.length; i++) arr[i] = bin.charCodeAt(i);
      term.write(arr);
    });
    unlistenExit = await onTermExit((exitId) => {
      if (exitId === id) term.write(`\r\n\x1b[90m${t("term.processExited")}\x1b[0m\r\n`);
    });

    // Launch the app sized to the terminal we just laid out.
    try {
      await launchApp(id, term.cols, term.rows);
    } catch (err) {
      // Most likely "already running" - just show a note.
      term.write(`\r\n\x1b[33m${err}\x1b[0m\r\n`);
    }

    ro = new ResizeObserver(() => scheduleFit());
    ro.observe(el);

    // Re-fit once styles (incl. the reserved scrollbar gutter) have settled.
    setTimeout(() => doFit(), 120);
  });

  $effect(() => {
    if (active) setTimeout(doFit, 0);
  });

  onDestroy(() => {
    ro?.disconnect();
    if (fitTimer) clearTimeout(fitTimer);
    unsubTheme?.();
    cleanupMouse?.();
    unlistenOut?.();
    unlistenExit?.();
    term?.dispose();
  });
</script>

<div class="term" bind:this={el} use:scrollFade style:display={active ? "block" : "none"}></div>

<style>
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
