<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import wordmark from "../assets/moonpool-wordmark.png";
  import { t } from "./i18n.svelte";

  const win = getCurrentWindow();
  let maximized = $state(false);

  // Keep the maximize/restore glyph in sync with the actual window state.
  $effect(() => {
    let unlisten: (() => void) | undefined;
    win.isMaximized().then((m) => (maximized = m));
    win
      .onResized(() => win.isMaximized().then((m) => (maximized = m)))
      .then((u) => (unlisten = u));
    return () => unlisten?.();
  });

  // Close routes through the window's close request so the Rust handler can
  // honour "close to tray"; it isn't a hard exit.
  const close = () => win.close();
  const minimize = () => win.minimize();
  const toggleMax = () => win.toggleMaximize();
</script>

<div class="titlebar" data-tauri-drag-region>
  <img class="wordmark" src={wordmark} alt="moonpool" draggable="false" />
  <div class="controls">
    <button class="ctl" title={t("titlebar.minimize")} aria-label={t("titlebar.minimize")} onclick={minimize}>
      <svg viewBox="0 0 12 12" width="11" height="11" aria-hidden="true">
        <path d="M2 6h8" stroke="currentColor" stroke-width="1.2" />
      </svg>
    </button>
    <button
      class="ctl"
      title={maximized ? t("titlebar.restore") : t("titlebar.maximize")}
      aria-label={maximized ? t("titlebar.restore") : t("titlebar.maximize")}
      onclick={toggleMax}
    >
      {#if maximized}
        <svg viewBox="0 0 12 12" width="11" height="11" aria-hidden="true">
          <path
            d="M3.5 3.5V2.5h6v6h-1M2.5 4.5h6v6h-6z"
            fill="none"
            stroke="currentColor"
            stroke-width="1.1"
          />
        </svg>
      {:else}
        <svg viewBox="0 0 12 12" width="11" height="11" aria-hidden="true">
          <rect
            x="2.5"
            y="2.5"
            width="7"
            height="7"
            fill="none"
            stroke="currentColor"
            stroke-width="1.1"
          />
        </svg>
      {/if}
    </button>
    <button class="ctl close" title={t("common.close")} aria-label={t("common.close")} onclick={close}>
      <svg viewBox="0 0 12 12" width="11" height="11" aria-hidden="true">
        <path d="M3 3l6 6M9 3l-6 6" stroke="currentColor" stroke-width="1.2" />
      </svg>
    </button>
  </div>
</div>

<style>
  .titlebar {
    flex: none;
    display: flex;
    align-items: center;
    height: 32px;
    padding-left: 10px;
    background: color-mix(in srgb, var(--bg) calc(var(--app-alpha) * 100%), transparent);
    border-bottom: 1px solid var(--border-muted);
    user-select: none;
    -webkit-user-select: none;
  }
  .wordmark {
    height: 15px;
    width: auto;
    filter: drop-shadow(1px 1px 1px rgba(0, 0, 0, 0.55));
    /* The drag region sits behind the image; let clicks fall through to it. */
    pointer-events: none;
  }
  .controls {
    margin-left: auto;
    display: flex;
    height: 100%;
    padding-right: 4px;
  }
  .ctl {
    width: 30px;
    height: 100%;
    display: grid;
    place-items: center;
    border: none;
    background: transparent;
    color: var(--fg);
    cursor: pointer;
    transition: background 0.12s;
  }
  .ctl:hover {
    background: color-mix(in srgb, var(--fg) 12%, transparent);
  }
  .ctl.close:hover {
    background: #e81123;
    color: #fff;
  }
</style>
