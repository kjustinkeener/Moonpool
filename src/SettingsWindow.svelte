<script lang="ts">
  import SettingsControls from "./lib/SettingsControls.svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  function close() {
    getCurrentWindow()
      .close()
      .catch(() => {});
  }
</script>

<div class="page">
  <SettingsControls onClose={close} />
</div>

<style>
  /* Frameless window: SettingsControls draws its own title bar + scrolling
     content area. This just gives it a full-height, themed, hover-fade shell
     (matching every other detached window's opacity behavior). */
  .page {
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
    height: 100vh;
    overflow: hidden;
    background: color-mix(in srgb, var(--bg-panel) calc(var(--app-alpha) * 100%), transparent);
    color: var(--text);
    transition: --app-alpha 2s ease;
  }
  .page:hover {
    --app-alpha: 1;
    transition: --app-alpha 0s;
  }
</style>
