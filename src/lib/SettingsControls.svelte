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
    setShowInTray,
    setShowInTaskbar,
    setShowStatusbar,
    setShowMcpProcesses,
    setLogRetentionMb,
    setCliLogging,
    revealLog,
    manifestDir,
    cliOutputDir,
    revealCliOutputDir,
  } from "./api";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { getTheme, setTheme, THEMES, type Theme } from "./theme";
  import { t, localeChoice, LOCALES, setLocale } from "./i18n.svelte";
  import { emit } from "@tauri-apps/api/event";
  import brandIcon from "../assets/app-icon.png";
  import wordmark from "../assets/moonpool-wordmark-text.png";
  import Icon from "./Icon.svelte";
  import { scrollFade } from "./scrollfade";

  let { onClose }: { onClose: () => void } = $props();

  // Defaults (mirror Settings::default() in src-tauri/src/lib.rs). Right-click
  // any control below to reset just that field.
  const DEFAULTS: {
    closeToTray: boolean;
    minimizeToTray: boolean;
    checkOnStartup: boolean;
    transparency: number;
    alwaysOnTop: boolean;
    showInTray: boolean;
    showInTaskbar: boolean;
    showStatusbar: boolean;
    showMcpProcesses: boolean;
    debugLogging: boolean;
    cliLogging: boolean;
    logRetentionMb: number;
  } = {
    closeToTray: false,
    minimizeToTray: true,
    checkOnStartup: true,
    transparency: 0,
    alwaysOnTop: false,
    showInTray: true,
    showInTaskbar: true,
    showStatusbar: true,
    showMcpProcesses: true,
    debugLogging: false,
    cliLogging: false,
    logRetentionMb: 10,
  };

  let debugLogging = $state(DEFAULTS.debugLogging);
  let cliLogging = $state(DEFAULTS.cliLogging);
  let closeToTray = $state(DEFAULTS.closeToTray);
  let minimizeToTray = $state(DEFAULTS.minimizeToTray);
  let checkOnStartup = $state(DEFAULTS.checkOnStartup);
  let transparency = $state(DEFAULTS.transparency);
  let alwaysOnTop = $state(DEFAULTS.alwaysOnTop);
  let showInTray = $state(DEFAULTS.showInTray);
  let showInTaskbar = $state(DEFAULTS.showInTaskbar);
  let showStatusbar = $state(DEFAULTS.showStatusbar);
  let showMcpProcesses = $state(DEFAULTS.showMcpProcesses);
  let logRetentionMb = $state(DEFAULTS.logRetentionMb);
  let saveError = $state("");
  let dir = $state("");
  let cliDir = $state("");
  const logPathText = $derived(dir ? `${dir}${dir.includes("\\") ? "\\" : "/"}moonpool.log` : "");

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
  function resetTransparency() {
    transparency = DEFAULTS.transparency;
    applyTransparency(transparency);
    emit("settings:transparency", transparency).catch(() => {});
    setTransparency(transparency).catch(() => {});
  }

  let logRetentionSaveTimer: ReturnType<typeof setTimeout>;
  function onLogRetentionInput(e: Event) {
    const raw = Math.max(1, Number((e.target as HTMLInputElement).value) || 1);
    logRetentionMb = raw;
    clearTimeout(logRetentionSaveTimer);
    logRetentionSaveTimer = setTimeout(
      () => setLogRetentionMb(logRetentionMb).catch(() => {}),
      400,
    );
  }
  function resetLogRetention() {
    logRetentionMb = DEFAULTS.logRetentionMb;
    setLogRetentionMb(logRetentionMb).catch(() => {});
  }

  let logPathCopied = $state(false);
  let logPathCopiedTimer: ReturnType<typeof setTimeout> | null = null;
  function copyLogPath() {
    if (!logPathText) return;
    writeText(logPathText)
      .then(() => {
        logPathCopied = true;
        if (logPathCopiedTimer) clearTimeout(logPathCopiedTimer);
        logPathCopiedTimer = setTimeout(() => (logPathCopied = false), 1200);
      })
      .catch(() => {});
  }

  let cliPathCopied = $state(false);
  let cliPathCopiedTimer: ReturnType<typeof setTimeout> | null = null;
  function copyCliOutputPath() {
    if (!cliDir) return;
    writeText(cliDir)
      .then(() => {
        cliPathCopied = true;
        if (cliPathCopiedTimer) clearTimeout(cliPathCopiedTimer);
        cliPathCopiedTimer = setTimeout(() => (cliPathCopied = false), 1200);
      })
      .catch(() => {});
  }

  function selectPath(e: FocusEvent) {
    (e.target as HTMLInputElement).select();
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
      showInTray = s.showInTray ?? true;
      showInTaskbar = s.showInTaskbar ?? true;
      showStatusbar = s.showStatusbar ?? true;
      showMcpProcesses = s.showMcpProcesses ?? true;
      logRetentionMb = s.logRetentionMb ?? 10;
      cliLogging = s.cliLogging ?? false;
      applyTransparency(transparency);
    } catch (e) {
      saveError = `Could not load settings.json: ${String(e)}`;
    }
    manifestDir()
      .then((d) => (dir = d))
      .catch(() => {});
    cliOutputDir()
      .then((d) => (cliDir = d))
      .catch(() => {});
  });

  // Cancel a pending debounced transparency save if the window closes first.
  onDestroy(() => clearTimeout(saveTimer));

  // Every boolean setting follows the same save/rollback/reset shape; this one
  // helper drives all of them instead of a hand-rolled toggle+reset pair per row.
  function boolSetting(
    get: () => boolean,
    set: (v: boolean) => void,
    save: (v: boolean) => Promise<void>,
    fallback: boolean,
  ) {
    async function commit(value: boolean) {
      const previous = get();
      set(value);
      try {
        await save(value);
        saveError = "";
      } catch (e) {
        set(previous);
        saveError = `Could not save settings.json: ${String(e)}`;
      }
    }
    return {
      toggle: () => commit(!get()),
      reset: (ev: Event) => {
        ev.preventDefault();
        commit(fallback);
      },
    };
  }

  const debugLoggingCtl = boolSetting(
    () => debugLogging,
    (v) => (debugLogging = v),
    async (v) => {
      await setDebugLogging(v);
      if (v) {
        // no-op; backend logs its own "enabled" line
      }
    },
    DEFAULTS.debugLogging,
  );
  const cliLoggingCtl = boolSetting(
    () => cliLogging,
    (v) => (cliLogging = v),
    setCliLogging,
    DEFAULTS.cliLogging,
  );
  const closeToTrayCtl = boolSetting(
    () => closeToTray,
    (v) => (closeToTray = v),
    setCloseToTray,
    DEFAULTS.closeToTray,
  );
  const minimizeToTrayCtl = boolSetting(
    () => minimizeToTray,
    (v) => (minimizeToTray = v),
    setMinimizeToTray,
    DEFAULTS.minimizeToTray,
  );
  const checkOnStartupCtl = boolSetting(
    () => checkOnStartup,
    (v) => (checkOnStartup = v),
    setCheckOnStartup,
    DEFAULTS.checkOnStartup,
  );
  const alwaysOnTopCtl = boolSetting(
    () => alwaysOnTop,
    (v) => (alwaysOnTop = v),
    setAlwaysOnTop,
    DEFAULTS.alwaysOnTop,
  );
  const showStatusbarCtl = boolSetting(
    () => showStatusbar,
    (v) => (showStatusbar = v),
    async (v) => {
      await setShowStatusbar(v);
      emit("settings:show-statusbar", v).catch(() => {});
    },
    DEFAULTS.showStatusbar,
  );
  const showMcpProcessesCtl = boolSetting(
    () => showMcpProcesses,
    (v) => (showMcpProcesses = v),
    async (v) => {
      await setShowMcpProcesses(v);
      emit("settings:show-mcp-processes", v).catch(() => {});
    },
    DEFAULTS.showMcpProcesses,
  );

  // Show-in-tray / show-in-taskbar lock together: turning off the one still-on
  // control would leave no way back into a hidden, taskbar-less window.
  const trayLockout = $derived(showInTray && !showInTaskbar);
  const taskbarLockout = $derived(showInTaskbar && !showInTray);
  const showInTrayCtl = boolSetting(
    () => showInTray,
    (v) => (showInTray = v),
    setShowInTray,
    DEFAULTS.showInTray,
  );
  const showInTaskbarCtl = boolSetting(
    () => showInTaskbar,
    (v) => (showInTaskbar = v),
    setShowInTaskbar,
    DEFAULTS.showInTaskbar,
  );
</script>

<!-- Escape closes the window, the same as the X button. This is a
     decorationless window, so without it the only way out is the button. -->
<svelte:window onkeydown={(e) => e.key === "Escape" && onClose()} />

<div class="wrap">
<div class="bar" data-tauri-drag-region>
  <img class="brandicon" src={brandIcon} alt="" aria-hidden="true" draggable="false" />
  <img class="wordmark" src={wordmark} alt="moonpool" draggable="false" />
  <span class="title" data-tauri-drag-region>{t("settings.title")}</span>
  <span class="spacer" data-tauri-drag-region></span>
  <button class="x" onclick={onClose} title={t("common.close")} aria-label={t("common.close")}>
    <Icon name="close" size={11} width={2.4} />
  </button>
</div>

<div class="content" use:scrollFade>
  {#if saveError}<div class="save-error" role="alert">{saveError}</div>{/if}

  <div class="columns">
  <div class="col">
  <div class="setting">
    <div class="title">{t("settings.language")}</div>
    <select
      class="theme-select"
      title={t("settings.languageHint")}
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

  <label class="row" title={t("settings.closeToTrayHint") + " " + t("settings.resetTip")}>
    <input
      type="checkbox"
      checked={closeToTray}
      onchange={closeToTrayCtl.toggle}
      oncontextmenu={closeToTrayCtl.reset}
    />
    <span>{t("settings.closeToTray")}</span>
  </label>

  <label class="row" title={t("settings.minimizeToTrayHint") + " " + t("settings.resetTip")}>
    <input
      type="checkbox"
      checked={minimizeToTray}
      onchange={minimizeToTrayCtl.toggle}
      oncontextmenu={minimizeToTrayCtl.reset}
    />
    <span>{t("settings.minimizeToTray")}</span>
  </label>

  <label class="row" title={t("settings.alwaysOnTopHint") + " " + t("settings.resetTip")}>
    <input
      type="checkbox"
      checked={alwaysOnTop}
      onchange={alwaysOnTopCtl.toggle}
      oncontextmenu={alwaysOnTopCtl.reset}
    />
    <span>{t("settings.alwaysOnTop")}</span>
  </label>

  <label
    class="row"
    title={(trayLockout ? t("settings.lockoutTip") + " " : "") +
      t("settings.showInTrayHint") +
      " " +
      t("settings.resetTip")}
  >
    <input
      type="checkbox"
      checked={showInTray}
      disabled={trayLockout}
      onchange={showInTrayCtl.toggle}
      oncontextmenu={showInTrayCtl.reset}
    />
    <span>{t("settings.showInTray")}</span>
  </label>

  <label
    class="row"
    title={(taskbarLockout ? t("settings.lockoutTip") + " " : "") +
      t("settings.showInTaskbarHint") +
      " " +
      t("settings.resetTip")}
  >
    <input
      type="checkbox"
      checked={showInTaskbar}
      disabled={taskbarLockout}
      onchange={showInTaskbarCtl.toggle}
      oncontextmenu={showInTaskbarCtl.reset}
    />
    <span>{t("settings.showInTaskbar")}</span>
  </label>

  <label class="row" title={t("settings.showStatusbarHint") + " " + t("settings.resetTip")}>
    <input
      type="checkbox"
      checked={showStatusbar}
      onchange={showStatusbarCtl.toggle}
      oncontextmenu={showStatusbarCtl.reset}
    />
    <span>{t("settings.showStatusbar")}</span>
  </label>

  <label class="row" title={t("settings.showMcpProcessesHint") + " " + t("settings.resetTip")}>
    <input
      type="checkbox"
      checked={showMcpProcesses}
      onchange={showMcpProcessesCtl.toggle}
      oncontextmenu={showMcpProcessesCtl.reset}
    />
    <span>{t("settings.showMcpProcesses")}</span>
  </label>

  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div
    class="setting"
    role="group"
    title={t("settings.transparencyHint") + " " + t("settings.resetTip")}
    oncontextmenu={(e) => {
      e.preventDefault();
      resetTransparency();
    }}
  >
    <div class="title">{t("settings.transparency")}</div>
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

  <label class="row" title={t("settings.checkOnStartupHint") + " " + t("settings.resetTip")}>
    <input
      type="checkbox"
      checked={checkOnStartup}
      onchange={checkOnStartupCtl.toggle}
      oncontextmenu={checkOnStartupCtl.reset}
    />
    <span>{t("settings.checkOnStartup")}</span>
  </label>
  </div>

  <div class="col">
  <div class="log-card">
    <label class="row" title={t("settings.cliLoggingHint") + " " + t("settings.resetTip")}>
      <input
        type="checkbox"
        checked={cliLogging}
        onchange={cliLoggingCtl.toggle}
        oncontextmenu={cliLoggingCtl.reset}
      />
      <span>{t("settings.cliLogging")}</span>
    </label>

    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="setting"
      role="group"
      title={t("settings.logRetentionMbHint") + " " + t("settings.resetTip")}
      oncontextmenu={(e) => {
        e.preventDefault();
        resetLogRetention();
      }}
    >
      <div class="title">{t("settings.logRetentionMb")}</div>
      <div class="slider-row">
        <input
          class="num-input"
          type="number"
          min="1"
          step="1"
          disabled={!cliLogging}
          value={logRetentionMb}
          oninput={onLogRetentionInput}
          aria-label={t("settings.logRetentionMb")}
        />
        <span class="pct">MB</span>
      </div>
    </div>

    {#if cliDir}
    <div class="path-row">
      <input
        class="path"
        type="text"
        readonly
        value={cliDir}
        style="width: {cliDir.length}ch"
        onfocus={selectPath}
        aria-label={t("settings.cliLogging")}
      />
      <div class="path-actions">
        <button
          class="icon-btn"
          title={t("settings.revealCliOutputDir")}
          aria-label={t("settings.revealCliOutputDir")}
          onclick={() => revealCliOutputDir()}
        >
          <Icon name="folder" size={14} />
        </button>
        <button
          class="icon-btn"
          title={cliPathCopied ? t("term.copied") : t("settings.copyCliOutputPath")}
          aria-label={t("settings.copyCliOutputPath")}
          onclick={copyCliOutputPath}
        >
          <Icon name={cliPathCopied ? "check" : "copy"} size={14} />
        </button>
      </div>
    </div>
    {/if}
  </div>

  <div class="log-card">
    <label
      class="row"
      title={t("settings.debugLoggingHint", { file: "moonpool.log" }) + " " + t("settings.resetTip")}
    >
      <input
        type="checkbox"
        checked={debugLogging}
        onchange={debugLoggingCtl.toggle}
        oncontextmenu={debugLoggingCtl.reset}
      />
      <span>{t("settings.debugLogging")}</span>
    </label>

    {#if dir}
    <div class="path-row">
      <input
        class="path"
        type="text"
        readonly
        value={logPathText}
        style="width: {logPathText.length}ch"
        onfocus={selectPath}
        aria-label={t("settings.debugLogging")}
      />
      <div class="path-actions">
        <button
          class="icon-btn"
          title={t("settings.openLog")}
          aria-label={t("settings.openLog")}
          onclick={() => revealLog()}
        >
          <Icon name="folder" size={14} />
        </button>
        <button
          class="icon-btn"
          title={logPathCopied ? t("term.copied") : t("settings.copyLogPath")}
          aria-label={t("settings.copyLogPath")}
          onclick={copyLogPath}
        >
          <Icon name={logPathCopied ? "check" : "copy"} size={14} />
        </button>
      </div>
    </div>
    {/if}
  </div>
  </div>
  </div>
</div>
</div>

<style>
  :global(html),
  :global(body) {
    background: transparent;
  }
  .wrap {
    display: flex;
    flex-direction: column;
    height: 100%;
    min-height: 0;
  }
  .content {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
  }
  .bar {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 32px;
    flex: none;
    padding-left: 10px;
    background: color-mix(in srgb, var(--bg) calc(var(--app-alpha) * 100%), transparent);
    border-bottom: 1px solid var(--border-muted);
    user-select: none;
    -webkit-user-select: none;
  }
  .bar .title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-strong);
    padding-left: 8px;
    border-left: 1px solid var(--border-muted);
  }
  .brandicon {
    height: 16px;
    width: 16px;
    pointer-events: none;
    filter: drop-shadow(0 0 4px rgba(97, 252, 237, 0.455))
      drop-shadow(0 0 8px rgba(97, 252, 237, 0.28));
  }
  .wordmark {
    height: 15px;
    width: auto;
    filter: drop-shadow(1px 1px 1px rgba(0, 0, 0, 0.55));
    pointer-events: none;
  }
  .spacer {
    flex: 1;
  }
  .x {
    width: 30px;
    height: 32px;
    display: grid;
    place-items: center;
    border: none;
    background: transparent;
    color: var(--text);
    cursor: pointer;
    transition: background 0.12s;
  }
  .x:hover {
    background: #e81123;
    color: #fff;
  }
  .content {
    box-sizing: border-box;
    padding: 16px 22px 20px;
    color: var(--text);
  }
  .columns {
    display: flex;
    gap: 24px;
    align-items: flex-start;
  }
  .col {
    flex: 1 1 0;
    min-width: 0;
  }
  .save-error {
    margin-bottom: 10px;
    padding: 7px 9px;
    color: #ffd7d7;
    background: #6b2028;
    border-radius: 4px;
    font-size: 11px;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 10px;
    cursor: pointer;
    margin-bottom: 12px;
  }
  .row span {
    font-size: 13px;
    color: var(--text);
  }
  .row input:disabled {
    cursor: not-allowed;
  }
  .row:has(input:disabled) span {
    color: var(--text-dim);
  }
  .setting {
    margin-bottom: 16px;
  }
  .setting > .title {
    margin-bottom: 6px;
    font-size: 13px;
    color: var(--text);
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
  .log-card {
    border: 1px solid var(--border);
    border-radius: 8px;
    background: color-mix(in srgb, var(--bg-elevated) 45%, transparent);
    padding: 12px 14px;
    margin-bottom: 16px;
  }
  .log-card .row,
  .log-card .setting {
    margin-bottom: 12px;
  }
  .log-card .row:last-child,
  .log-card .setting:last-child,
  .log-card .path-row:last-child {
    margin-bottom: 0;
  }
  .path-row {
    position: relative;
    display: inline-flex;
    max-width: 100%;
  }
  .path {
    flex: 0 1 auto;
    min-width: 0;
    box-sizing: content-box;
    font-family: "Cascadia Code", Consolas, monospace;
    font-size: 11px;
    color: var(--text-dim);
    background: var(--bg);
    border: 1px solid var(--border-muted);
    border-radius: 6px;
    padding: 7px 62px 7px 9px;
    text-align: right;
    white-space: nowrap;
    overflow: hidden;
  }
  .path:focus {
    outline: none;
    border-color: var(--focus);
    color: var(--text);
  }
  .path-actions {
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    display: flex;
    align-items: center;
    gap: 2px;
    padding-right: 4px;
  }
  .icon-btn {
    flex: none;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    background: transparent;
    border: 1px solid transparent;
    color: var(--text-secondary);
    border-radius: 5px;
    cursor: pointer;
  }
  .icon-btn:hover {
    color: var(--text);
    background: var(--bg-elevated);
    border-color: var(--border);
  }
  .num-input {
    width: 60px;
    flex: none;
    box-sizing: border-box;
  }
</style>
