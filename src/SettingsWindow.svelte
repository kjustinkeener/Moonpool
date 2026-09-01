<script lang="ts">
  import SettingsControls from "./lib/SettingsControls.svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  function close() {
    getCurrentWindow()
      .close()
      .catch(() => {});
  }
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && close()} />

<div class="page">
  <SettingsControls onClose={close} />
</div>

<style>
  .page {
    box-sizing: border-box;
    min-height: 100vh;
    padding: 20px 22px;
    background: color-mix(in srgb, var(--bg-panel) calc(var(--app-alpha) * 100%), transparent);
    color: var(--text);
  }
</style>
