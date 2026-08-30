<script lang="ts">
  import { onMount } from "svelte";
  import {
    getSettings,
    setDebugLogging,
    setCloseToTray,
    setMinimizeToTray,
    setCheckOnStartup,
    openLog,
    manifestDir,
  } from "./api";
  import { getTheme, setTheme, type Theme } from "./theme";
  import Modal from "./Modal.svelte";

  let { onClose }: { onClose: () => void } = $props();

  let debugLogging = $state(false);
  let closeToTray = $state(true);
  let minimizeToTray = $state(true);
  let checkOnStartup = $state(true);
  let dir = $state("");

  const THEMES: Theme[] = ["auto", "light", "dark"];
  let theme = $state<Theme>(getTheme());
  function pickTheme(t: Theme) {
    theme = t;
    setTheme(t);
  }

  onMount(async () => {
    try {
      const s = await getSettings();
      debugLogging = s.debugLogging;
      closeToTray = s.closeToTray;
      minimizeToTray = s.minimizeToTray;
      checkOnStartup = s.checkOnStartup;
    } catch {
      /* defaults */
    }
    manifestDir()
      .then((d) => (dir = d))
      .catch(() => {});
  });

  async function toggle() {
    debugLogging = !debugLogging;
    await setDebugLogging(debugLogging).catch(() => {});
  }

  async function toggleCloseToTray() {
    closeToTray = !closeToTray;
    await setCloseToTray(closeToTray).catch(() => {});
  }

  async function toggleMinimizeToTray() {
    minimizeToTray = !minimizeToTray;
    await setMinimizeToTray(minimizeToTray).catch(() => {});
  }

  async function toggleCheckOnStartup() {
    checkOnStartup = !checkOnStartup;
    await setCheckOnStartup(checkOnStartup).catch(() => {});
  }
</script>

<Modal {onClose} width="420px">
    <h2>Settings</h2>

    <div class="setting">
      <div class="title">Theme</div>
      <div class="seg" role="group" aria-label="Theme">
        {#each THEMES as t (t)}
          <button class="seg-btn" class:sel={theme === t} onclick={() => pickTheme(t)}>{t}</button>
        {/each}
      </div>
    </div>

    <label class="row">
      <input type="checkbox" checked={closeToTray} onchange={toggleCloseToTray} />
      <div class="text">
        <div class="title">Close to tray</div>
        <div class="sub">
          Closing the window hides Moonpool to the tray (leaves the taskbar). Off: closing quits.
        </div>
      </div>
    </label>

    <label class="row">
      <input type="checkbox" checked={minimizeToTray} onchange={toggleMinimizeToTray} />
      <div class="text">
        <div class="title">Minimize to tray</div>
        <div class="sub">
          Minimizing hides Moonpool to the tray (leaves the taskbar). Off: minimizes to the taskbar.
        </div>
      </div>
    </label>

    <label class="row">
      <input type="checkbox" checked={checkOnStartup} onchange={toggleCheckOnStartup} />
      <div class="text">
        <div class="title">Check for updates on startup</div>
        <div class="sub">
          On launch, quietly checks GitHub for a newer version and shows a banner if one is found.
        </div>
      </div>
    </label>

    <label class="row">
      <input type="checkbox" checked={debugLogging} onchange={toggle} />
      <div class="text">
        <div class="title">Log debug info to a file</div>
        <div class="sub">
          Records manifest loads, launches, and errors to <code>moonpool.log</code>.
        </div>
      </div>
    </label>

    {#if dir}
      <div class="path">{dir}{dir.includes("\\") ? "\\" : "/"}moonpool.log</div>
    {/if}

    <div class="actions">
      <button class="btn" onclick={() => openLog()}>Open log</button>
      <div class="spacer"></div>
      <button class="btn primary" onclick={onClose}>Close</button>
    </div>
</Modal>

<style>
  h2 {
    margin: 0 0 16px;
    font-size: 16px;
    color: var(--text-strong);
  }
  .row {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    cursor: pointer;
    margin-bottom: 14px;
  }
  .row input {
    margin-top: 2px;
  }
  .title {
    font-size: 13px;
    color: var(--text);
  }
  .sub {
    font-size: 11px;
    color: var(--text-dim);
    margin-top: 2px;
  }
  .setting {
    margin-bottom: 16px;
  }
  .setting > .title {
    margin-bottom: 6px;
  }
  .seg {
    display: inline-flex;
    border: 1px solid var(--border);
    border-radius: 6px;
    overflow: hidden;
  }
  .seg-btn {
    background: var(--bg);
    color: var(--text-secondary);
    border: none;
    border-right: 1px solid var(--border);
    padding: 5px 16px;
    font-size: 12px;
    text-transform: capitalize;
    cursor: pointer;
  }
  .seg-btn:last-child {
    border-right: none;
  }
  .seg-btn.sel {
    background: var(--accent);
    color: var(--on-accent);
  }
  .path {
    font-family: "Cascadia Code", Consolas, monospace;
    font-size: 11px;
    color: var(--text-dim);
    background: var(--bg);
    border: 1px solid var(--border-muted);
    border-radius: 6px;
    padding: 7px 9px;
    margin-top: 14px;
    word-break: break-all;
  }
  .actions {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 18px;
  }
  .spacer {
    flex: 1;
  }
  .btn {
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    color: var(--text);
    padding: 7px 14px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 13px;
  }
  .btn.primary {
    background: var(--accent);
    border-color: var(--accent);
    color: var(--on-accent);
  }
</style>
