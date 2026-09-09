<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import wordmark from "../assets/moonpool-wordmark-text.png";
  import brandIcon from "../assets/app-icon.png";
  import { t } from "./i18n.svelte";
  import Icon from "./Icon.svelte";

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
  <span class="brand" data-tauri-drag-region>
    <img class="brandicon" src={brandIcon} alt="" aria-hidden="true" draggable="false" />
    <img class="wordmark" src={wordmark} alt="moonpool" draggable="false" />
  </span>
  <div class="controls">
    <button class="ctl" title={t("titlebar.minimize")} aria-label={t("titlebar.minimize")} onclick={minimize}>
      <Icon name="minimize" size={11} width={2.4} />
    </button>
    <button
      class="ctl"
      title={maximized ? t("titlebar.restore") : t("titlebar.maximize")}
      aria-label={maximized ? t("titlebar.restore") : t("titlebar.maximize")}
      onclick={toggleMax}
    >
      {#if maximized}
        <Icon name="restore" size={11} width={2.2} />
      {:else}
        <Icon name="maximize" size={11} width={2.2} />
      {/if}
    </button>
    <button class="ctl close" title={t("common.close")} aria-label={t("common.close")} onclick={close}>
      <Icon name="close" size={11} width={2.4} />
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
  .brand {
    display: inline-flex;
    align-items: center;
    gap: 6px;
  }
  .brandicon {
    height: 16px;
    width: 16px;
    pointer-events: none;
    /* Glow in the icon's own center color (#61FCED); scaled for the small size. */
    filter: drop-shadow(0 0 4px rgba(97, 252, 237, 0.455))
      drop-shadow(0 0 8px rgba(97, 252, 237, 0.28));
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
