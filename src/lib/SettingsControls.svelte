<script lang="ts">
  import { onMount, onDestroy } from "svelte";
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
  import { t, tSplit, LOCALES, localeChoice, setLocale } from "./i18n.svelte";
  import { emit } from "@tauri-apps/api/event";

  let { onClose }: { onClose: () => void } = $props();

  let debugLogging = $state(false);
  let closeToTray = $state(false);
  let minimizeToTray = $state(true);
  let checkOnStartup = $state(true);
  let transparency = $state(0);
  let alwaysOnTop = $state(false);
  let saveError = $state("");
  let dir = $state("");

  // Named palettes (Nord, Gruvbox, ...) are proper nouns and stay as written in
  // theme.ts; only the three generic ids have a translatable name.
  const themeLabel = (id: string, label: string) =>
    id === "auto"
      ? t("common.autoSystem")
      : id === "dark"
        ? t("theme.dark")
        : id === "light"
          ? t("theme.light")
          : label;

  let locale = $state(localeChoice());
  async function pickLocale(id: string) {
    locale = id;
    await setLocale(id);
    // Retranslate the hub live, the same way theme changes are broadcast.
    emit("settings:locale", id).catch(() => {});
  }

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
    } catch (e) {
      saveError = `Could not load settings.json: ${String(e)}`;
    }
    manifestDir()
      .then((d) => (dir = d))
      .catch(() => {});
  });

  // Cancel a pending debounced transparency save if the window closes first.
  onDestroy(() => clearTimeout(saveTimer));

  async function toggle() {
    const previous = debugLogging;
    debugLogging = !debugLogging;
    try {
      await setDebugLogging(debugLogging);
      saveError = "";
    } catch (e) {
      debugLogging = previous;
      saveError = `Could not save settings.json: ${String(e)}`;
    }
  }
  async function toggleCloseToTray() {
    const previous = closeToTray;
    closeToTray = !closeToTray;
    try {
      await setCloseToTray(closeToTray);
      saveError = "";
    } catch (e) {
      closeToTray = previous;
      saveError = `Could not save settings.json: ${String(e)}`;
    }
  }
  async function toggleMinimizeToTray() {
    const previous = minimizeToTray;
    minimizeToTray = !minimizeToTray;
    try {
      await setMinimizeToTray(minimizeToTray);
      saveError = "";
    } catch (e) {
      minimizeToTray = previous;
      saveError = `Could not save settings.json: ${String(e)}`;
    }
  }
  async function toggleCheckOnStartup() {
    const previous = checkOnStartup;
    checkOnStartup = !checkOnStartup;
    try {
      await setCheckOnStartup(checkOnStartup);
      saveError = "";
    } catch (e) {
      checkOnStartup = previous;
      saveError = `Could not save settings.json: ${String(e)}`;
    }
  }
  async function toggleAlwaysOnTop() {
    const previous = alwaysOnTop;
    alwaysOnTop = !alwaysOnTop;
    try {
      await setAlwaysOnTop(alwaysOnTop);
      saveError = "";
    } catch (e) {
      alwaysOnTop = previous;
      saveError = `Could not save settings.json: ${String(e)}`;
    }
  }
</script>

<h2>{t("settings.title")}</h2>

{#if saveError}<div class="save-error" role="alert">{saveError}</div>{/if}

<div class="setting">
  <div class="title">{t("settings.language")}</div>
  <div class="sub">{t("settings.languageHint")}</div>
  <select
    class="theme-select"
    aria-label={t("settings.language")}
    value={locale}
    onchange={(e) => pickLocale((e.target as HTMLSelectElement).value)}
  >
    <option value="auto">{t("common.autoSystem")}</option>
    <!-- Language names stay in their own language: someone stuck in the wrong
         one cannot read "Japanese" but can always find "日本語". -->
    {#each LOCALES as l (l.id)}
      <option value={l.id}>{l.label}</option>
    {/each}
  </select>
</div>

<div class="setting">
  <div class="title">{t("settings.theme")}</div>
  <select
    class="theme-select"
    aria-label={t("settings.theme")}
    value={theme}
    onchange={(e) => pickTheme((e.target as HTMLSelectElement).value)}
  >
    {#each THEMES as th (th.id)}
      <option value={th.id}>{themeLabel(th.id, th.label)}</option>
    {/each}
  </select>
</div>

<label class="row">
  <input type="checkbox" checked={closeToTray} onchange={toggleCloseToTray} />
  <div class="text">
    <div class="title">{t("settings.closeToTray")}</div>
    <div class="sub">{t("settings.closeToTrayHint")}</div>
  </div>
</label>

<label class="row">
  <input type="checkbox" checked={minimizeToTray} onchange={toggleMinimizeToTray} />
  <div class="text">
    <div class="title">{t("settings.minimizeToTray")}</div>
    <div class="sub">{t("settings.minimizeToTrayHint")}</div>
  </div>
</label>

<label class="row">
  <input type="checkbox" checked={alwaysOnTop} onchange={toggleAlwaysOnTop} />
  <div class="text">
    <div class="title">{t("settings.alwaysOnTop")}</div>
    <div class="sub">{t("settings.alwaysOnTopHint")}</div>
  </div>
</label>

<div class="setting">
  <div class="title">{t("settings.transparency")}</div>
  <div class="sub">{t("settings.transparencyHint")}</div>
  <div class="slider-row">
    <input
      type="range"
      min="0"
      max="90"
      step="5"
      value={transparency}
      oninput={onTransparencyInput}
      aria-label={t("settings.transparency")}
    />
    <span class="pct">{transparency}%</span>
  </div>
</div>

<label class="row">
  <input type="checkbox" checked={checkOnStartup} onchange={toggleCheckOnStartup} />
  <div class="text">
    <div class="title">{t("settings.checkOnStartup")}</div>
    <div class="sub">{t("settings.checkOnStartupHint")}</div>
  </div>
</label>

<label class="row">
  <input type="checkbox" checked={debugLogging} onchange={toggle} />
  <div class="text">
    <div class="title">{t("settings.debugLogging")}</div>
    <div class="sub">
      {#each tSplit("settings.debugLoggingHint") as c}{#if "text" in c}{c.text}{:else}<code
            >moonpool.log</code
          >{/if}{/each}
    </div>
  </div>
</label>

{#if dir}
  <div class="path">{dir}{dir.includes("\\") ? "\\" : "/"}moonpool.log</div>
{/if}

<div class="actions">
  <button class="btn" onclick={() => openLog()}>{t("settings.openLog")}</button>
  <div class="spacer"></div>
  <button class="btn primary" onclick={onClose}>{t("common.close")}</button>
</div>

<style>
  .save-error {
    margin-bottom: 10px;
    padding: 7px 9px;
    color: #ffd7d7;
    background: #6b2028;
    border-radius: 4px;
    font-size: 11px;
  }
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
