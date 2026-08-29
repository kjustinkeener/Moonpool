<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { Terminal } from "@xterm/xterm";
  import { FitAddon } from "@xterm/addon-fit";
  import "@xterm/xterm/css/xterm.css";
  import { launchApp, termInput, termResize, onTermOutput, onTermExit } from "./api";
  import { onThemeChange } from "./theme";
  import { writeText, readText } from "@tauri-apps/plugin-clipboard-manager";
  import type { UnlistenFn } from "@tauri-apps/api/event";

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

  // xterm has no access to CSS variables, so read the themed bg/fg from :root
  // and refresh them whenever the theme changes (OS flip or explicit choice).
  function termTheme() {
    const cs = getComputedStyle(document.documentElement);
    return {
      background: cs.getPropertyValue("--bg").trim() || "#0d1017",
      foreground: cs.getPropertyValue("--text").trim() || "#c9d1d9",
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
      if (exitId === id) term.write("\r\n\x1b[90m[process exited]\x1b[0m\r\n");
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

<div class="term" bind:this={el} style:display={active ? "block" : "none"}></div>

<style>
  .term {
    width: 100%;
    height: 100%;
    /* No right padding: the scrollbar sits flush at the container's right edge, so
       its paint position and interactive hitbox stay aligned. */
    padding: 6px 0 6px 8px;
    box-sizing: border-box;
    background: var(--bg);
  }

  /* Defining an explicit ::-webkit-scrollbar width forces a classic, space-reserving
     scrollbar in WebView2 (not an overlay), which the fit addon accounts for. No
     scrollbar-gutter and no thumb border - both split the visible bar from its hitbox. */
  :global(.xterm .xterm-viewport)::-webkit-scrollbar {
    width: 12px;
  }
  :global(.xterm .xterm-viewport)::-webkit-scrollbar-track {
    background: transparent;
  }
  :global(.xterm .xterm-viewport)::-webkit-scrollbar-thumb {
    background: var(--border-strong);
    border-radius: 6px;
  }
  :global(.xterm .xterm-viewport)::-webkit-scrollbar-thumb:hover {
    background: var(--text-faint);
  }
</style>
