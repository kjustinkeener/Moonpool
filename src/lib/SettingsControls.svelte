<script lang="ts">
  import { onMount } from "svelte";
  import {
    getSettings,
    setDebugLogging,
    setCloseToTray,
    setMinimizeToTray,
    setCheckOnStartup,
    setTransparency,
    setAlwaysOnTop,
    openLog,
    manifestDir,
  } from "./api";
  import { getTheme, setTheme, THEMES, type Theme } from "./theme";
  import { emit } from "@tauri-apps/api/event";

  let { onClose }: { onClose: () => void } = $props();

  let debugLogging = $state(false);
  let closeToTray = $state(false);
  let minimizeToTray = $state(true);
  let checkOnStartup = $state(true);
  let transparency = $state(0);
  let alwaysOnTop = $state(false);
  let dir = $state("");

  let theme = $state<Theme>(getTheme());
  function pickTheme(t: Theme) {
    theme = t;
    setTheme(t);
    // Sync other windows (e.g. the main hub) live.
    emit("settings:theme", t).catch(() => {});
  }

  // Live-apply the app-wide background opacity in this window, and broadcast so
  // the main window updates without waiting for a reload.
  function applyTransparency(pct: number) {
    const alpha = Math.max(0.1, 1 - Math.min(90, Math.max(0, pct)) / 100);
    document.documentElement.style.setProperty("--app-alpha", String(alpha));
  }
  let saveTimer: ReturnType<typeof setTimeout> | undefined;
  function onTransparencyInput(e: Event) {
    transparency = Number((e.target as HTMLInputElement).value);
    applyTransparency(transparency);
    emit("settings:transparency", transparency).catch(() => {});
    clearTimeout(saveTimer);
    saveTimer = setTimeout(
      () => setTransparency(transparency).catch(() => {}),
      200,
    );
  }

  onMount(async () => {
    try {
      const s = await getSettings();
      debugLogging = s.debugLogging;
      closeToTray = s.closeToTray;
      minimizeToTray = s.minimizeToTray;
      checkOnStartup = s.checkOnStartup;
      transparency = s.transparency ?? 0;
      alwaysOnTop = !!s.alwaysOnTop;
      applyTransparency(transparency);
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
  async function toggleAlwaysOnTop() {
    alwaysOnTop = !alwaysOnTop;
    await setAlwaysOnTop(alwaysOnTop).catch(() => {});
  }
</script>

<h2>Settings</h2>

<div class="setting">
  <div class="title">Theme</div>
  <select
    class="theme-select"
    aria-label="Theme"
    value={theme}
    onchange={(e) => pickTheme((e.target as HTMLSelectElement).value)}
  >
    {#each THEMES as t (t.id)}
      <option value={t.id}>{t.label}</option>
    {/each}
  </select>
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
  <input type="checkbox" checked={alwaysOnTop} onchange={toggleAlwaysOnTop} />
  <div class="text">
    <div class="title">Always on top</div>
    <div class="sub">Keep Moonpool and its Settings/About windows above other windows.</div>
  </div>
</label>

<div class="setting">
  <div class="title">Background transparency</div>
  <div class="sub">See-through window background. 0% is solid.</div>
  <div class="slider-row">
    <input
      type="range"
      min="0"
      max="90"
      step="5"
      value={transparency}
      oninput={onTransparencyInput}
      aria-label="Background transparency"
    />
    <span class="pct">{transparency}%</span>
  </div>
</div>

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
  .slider-row {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-top: 8px;
  }
  .slider-row input[type="range"] {
    flex: 1;
    accent-color: var(--accent);
  }
  .pct {
    font-size: 12px;
    color: var(--text-secondary);
    min-width: 34px;
    text-align: right;
  }
  .theme-select {
    width: 100%;
    box-sizing: border-box;
    background: var(--bg);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 7px 9px;
    font-size: 13px;
    cursor: pointer;
  }
  .theme-select:focus {
    outline: none;
    border-color: var(--focus);
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
