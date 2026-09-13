<script lang="ts">
  import SettingsControls from "./lib/SettingsControls.svelte";
  import { scrollFade } from "./lib/scrollfade";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  function close() {
    getCurrentWindow()
      .close()
      .catch(() => {});
  }
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && close()} />

<div class="page" use:scrollFade>
  <SettingsControls onClose={close} />
</div>

<style>
  /* `html, body` are `overflow: hidden` app-wide, so a page that outgrows the
     window loses the bottom of itself with no way to reach it. Settings is the
     one page whose height is not ours to predict: the same controls run
     noticeably taller in German, Polish or Russian, where labels wrap that fit
     on one line in English. So this scrolls rather than clips. The window also
     opens with vertical slack (see `openSettingsWindow`) so the scrollbar stays
     a safety net instead of the normal case. */
  .page {
    box-sizing: border-box;
    height: 100vh;
    overflow-y: auto;
    padding: 20px 22px;
    background: color-mix(in srgb, var(--bg-panel) calc(var(--app-alpha) * 100%), transparent);
    color: var(--text);
    /* Wakes this window to full opacity on hover, then eases back to the
       configured transparency once the pointer leaves. Matches the hub
       window's hover-fade in App.svelte; each window fades independently. */
    transition: --app-alpha 2s ease;
  }
  .page:hover {
    --app-alpha: 1;
    transition: --app-alpha 0s;
  }
</style>
