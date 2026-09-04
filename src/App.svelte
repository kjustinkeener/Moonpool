<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    getApps,
    onStatus,
    stopApp,
    openUrl,
    reloadManifest,
    openManifest,
    manifestDir,
    saveManifest,
    appIcon,
    getSettings,
    onControl,
    reportOutcome,
    openSettingsWindow,
    openAboutWindow,
    updateCheck,
    updateApply,
    type UpdateInfo,
  } from "./lib/api";
  import { listen } from "@tauri-apps/api/event";
  import { setTheme, type Theme } from "./lib/theme";
  import { t, tSplit, watchLocale } from "./lib/i18n.svelte";
  import type { AppEntry, AppStatus } from "./lib/types";
  import Icon from "./lib/Icon.svelte";
  import Sidebar from "./lib/Sidebar.svelte";
  import TermView from "./lib/TermView.svelte";
  import AppEditor from "./lib/AppEditor.svelte";
  import Titlebar from "./lib/Titlebar.svelte";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { open } from "@tauri-apps/plugin-dialog";
  import type { UnlistenFn } from "@tauri-apps/api/event";
  import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";

  let apps = $state<AppEntry[]>([]);
  let statuses = $state<Record<string, AppStatus>>({});
  let openTabs = $state<string[]>([]);
  let activeTab = $state<string | null>(null);
  let filter = $state("");
  // Bump a tab's generation to force TermView to remount (fresh terminal + relaunch).
  let gen = $state<Record<string, number>>({});
  let unlisten: UnlistenFn | null = null;
  let unlistenControl: UnlistenFn | null = null;
  // Live updates pushed from the detached Settings window.
  let unlistenTransparency: UnlistenFn | null = null;
  let unlistenTheme: UnlistenFn | null = null;
  let unlistenLocale: UnlistenFn | null = null;

  // Resolved app icons (data URI or url), fetched from the backend.
  let iconSrc = $state<Record<string, string>>({});
  // `refresh` forces a fresh re-pull past the exe/favicon caches - used on app
  // startup and on every launch, since icons change between successive builds.
  async function loadIcon(id: string, refresh = false) {
    const src = await appIcon(id, refresh).catch(() => null);
    if (src) iconSrc = { ...iconSrc, [id]: src };
  }
  function loadAllIcons(refresh = false) {
    for (const a of apps) loadIcon(a.id, refresh);
  }

  // Modals + AI quickstart.
  let showEditor = $state(false);
  let editingEntry = $state<AppEntry | null>(null);
  let cfgDir = $state("");
  // Join filenames onto the OS-native config dir with the right separator
  // (the backend returns a Windows path with `\`, a POSIX path with `/`).
  let cfgSep = $derived(cfgDir.includes("\\") ? "\\" : "/");
  let promptCopied = $state(false);

  // On-startup update check (gated by the checkOnStartup setting). If a newer
  // release is found, `update` drives the banner in the terminal area.
  let update = $state<UpdateInfo | null>(null);
  let currentVersion = $state("");
  let updateStatus = $state("");
  let updating = $state(false);
  let updateDone = $state(false);

  async function installUpdate() {
    if (!update || updating || updateDone) return;
    updating = true;
    updateStatus = t("app.downloading", { version: update.version });
    try {
      // On success the backend relaunches and exits, so this call never returns.
      await updateApply(update);
      updateStatus = t("app.updateInstalled");
      updateDone = true;
    } catch (e) {
      updateStatus = t("app.updateFailed", { error: String(e) });
      updating = false;
    }
  }

  let groupNames = $derived([...new Set(apps.map((a) => a.group))]);
  let aiPrompt = $derived(
    `You are setting up Moonpool, a local app launcher. First read the config guide at:\n` +
      `  ${cfgDir}${cfgSep}AI-README.md\n` +
      `Then scan my project folders for launchable apps (web dashboards, small web/desktop apps, ` +
      `static HTML dashboards, and CLI scripts) and register each one by adding an entry to:\n` +
      `  ${cfgDir}${cfgSep}apps.json\n` +
      `following the schema and command rules in the guide. Confirm each app's real launch command ` +
      `and port before adding it, and preserve any existing entries. When you're done, I'll click ` +
      `Reload in Moonpool.`,
  );

  // Drive the app-wide background opacity from the Transparency setting. Shared so
  // the Settings slider can preview live via the window event below.
  function applyTransparency(pct: number) {
    const alpha = Math.max(0.1, 1 - Math.min(90, Math.max(0, pct)) / 100);
    document.documentElement.style.setProperty("--app-alpha", String(alpha));
  }

  async function copyPrompt() {
    await writeText(aiPrompt).catch(() => {});
    promptCopied = true;
    setTimeout(() => (promptCopied = false), 1500);
  }

  function openAdd() {
    editingEntry = null;
    showEditor = true;
  }
  function openEdit(app: AppEntry) {
    editingEntry = app;
    showEditor = true;
  }
  async function handleEditorSave(entry: AppEntry, originalId?: string) {
    let next = [...apps];
    const idx = originalId ? next.findIndex((a) => a.id === originalId) : -1;
    if (idx >= 0) {
      next[idx] = entry;
    } else {
      // New app: the name-derived id may collide, so make it unique.
      const existing = new Set(next.map((a) => a.id));
      if (existing.has(entry.id)) {
        let n = 2;
        while (existing.has(`${entry.id}-${n}`)) n++;
        entry = { ...entry, id: `${entry.id}-${n}` };
      }
      next = next.concat(entry);
    }
    apps = next;
    await saveManifest(next);
    loadIcon(entry.id);
    showEditor = false;
  }
  async function handleEditorDelete(id: string) {
    const next = apps.filter((a) => a.id !== id);
    apps = next;
    await saveManifest(next);
    showEditor = false;
  }

  // Right-click menu actions from the sidebar.
  async function handleDelete(app: AppEntry) {
    if (!confirm(t("app.confirmDelete", { name: app.name }))) return;
    const next = apps.filter((a) => a.id !== app.id);
    apps = next;
    await saveManifest(next);
  }
  async function handleRename(app: AppEntry, name: string) {
    const trimmed = name.trim();
    if (!trimmed || trimmed === app.name) return;
    const next = apps.map((a) => (a.id === app.id ? { ...a, name: trimmed } : a));
    apps = next;
    await saveManifest(next);
  }
  async function handleSetIcon(app: AppEntry) {
    const picked = await open({
      title: t("app.iconDialogTitle", { name: app.name }),
      multiple: false,
      directory: false,
      filters: [{ name: t("app.imagesFilter"), extensions: ["png", "jpg", "jpeg", "gif", "svg", "webp", "ico"] }],
    }).catch(() => null);
    if (typeof picked !== "string" || !picked) return;
    const next = apps.map((a) => (a.id === app.id ? { ...a, icon: picked } : a));
    apps = next;
    await saveManifest(next);
    loadIcon(app.id);
  }

  // Per-app last-started time (persisted); drives recency ordering within each group.
  let lastStarted = $state<Record<string, number>>({});
  // "Moved to top" animation: glow in (1s) -> animate to the top (1s) -> glow out (1s),
  // so a launched app rises to the top of its group gently instead of snapping.
  let highlight = $state<Set<string>>(new Set());
  let slowReorder = $state(false);
  const highlightTimers: Record<string, ReturnType<typeof setTimeout>[]> = {};

  function recordStart(id: string) {
    // Restart the sequence cleanly if this app is launched again mid-animation.
    (highlightTimers[id] ?? []).forEach(clearTimeout);
    highlightTimers[id] = [];
    // Phase 1: glow fades in (1s via CSS) while the row stays in place.
    highlight = new Set(highlight).add(id);
    highlightTimers[id].push(
      setTimeout(() => {
        // Phase 2: reorder now so the row flips to the top over 1s.
        slowReorder = true;
        lastStarted = { ...lastStarted, [id]: Date.now() };
        localStorage.setItem("moonpool.lastStarted", JSON.stringify(lastStarted));
        // flip captures its duration at trigger time, so revert the speed right after.
        highlightTimers[id].push(setTimeout(() => (slowReorder = false), 60));
        // Phase 3: once the move has finished, glow fades out (1s via CSS).
        highlightTimers[id].push(
          setTimeout(() => {
            const s = new Set(highlight);
            s.delete(id);
            highlight = s;
          }, 500),
        );
      }, 250),
    );
  }

  // Per-app busy state: set on click, cleared when status reaches the target (or times out).
  let pending = $state<Set<string>>(new Set());
  const pendingTarget: Record<string, "up" | "down"> = {};
  const pendingTimers: Record<string, ReturnType<typeof setTimeout>> = {};

  function setPending(id: string, target: "up" | "down") {
    const s = new Set(pending);
    s.add(id);
    pending = s;
    pendingTarget[id] = target;
    clearTimeout(pendingTimers[id]);
    pendingTimers[id] = setTimeout(() => clearPending(id), 8000);
  }
  function clearPending(id: string) {
    if (pending.has(id)) {
      const s = new Set(pending);
      s.delete(id);
      pending = s;
    }
    delete pendingTarget[id];
    clearTimeout(pendingTimers[id]);
    delete pendingTimers[id];
  }

  // Resizable sidebar (persisted).
  const MIN_W = 180;
  const MAX_W = 620;
  let sidebarWidth = $state(280);
  let resizing = $state(false);

  // Collapsible CLI pane: hiding it shrinks the window down to just the sidebar;
  // a ">" button by the filter box brings it back to its former width.
  const RESIZER_W = 5;
  let cliVisible = $state(true);
  // Width the main/CLI pane had before it was collapsed, restored on re-open.
  let savedMainWidth = 700;

  async function collapseCli() {
    if (!cliVisible) return;
    const win = getCurrentWindow();
    const innerH = window.innerHeight;
    savedMainWidth = Math.max(300, window.innerWidth - sidebarWidth - RESIZER_W);
    cliVisible = false;
    localStorage.setItem("moonpool.cliVisible", "0");
    localStorage.setItem("moonpool.savedMainWidth", String(savedMainWidth));
    try {
      await win.setSize(new LogicalSize(sidebarWidth, innerH));
    } catch {}
  }

  async function expandCli() {
    if (cliVisible) return;
    const win = getCurrentWindow();
    const innerH = window.innerHeight;
    cliVisible = true;
    localStorage.setItem("moonpool.cliVisible", "1");
    try {
      await win.setSize(
        new LogicalSize(sidebarWidth + RESIZER_W + savedMainWidth, innerH),
      );
    } catch {}
  }

  // Reveal the CLI as soon as there's any room past the sidebar + resizer, not
  // just once a comfortable width opens up. 1px of main-pane space is enough.
  const MIN_MAIN_SHOW = 1;

  // Reconcile the collapsed flag with the actual window width: if the window is
  // wide enough to hold the CLI (e.g. the user dragged it wider while collapsed,
  // or the restored width disagrees with the saved flag), reveal the pane so the
  // extra space is filled instead of left as a blank hole. No setSize here - the
  // window already has the width; we just fill it.
  function revealCliIfRoom() {
    if (cliVisible) return;
    const room = window.innerWidth - sidebarWidth - RESIZER_W;
    if (room >= MIN_MAIN_SHOW) {
      cliVisible = true;
      savedMainWidth = Math.max(300, room);
      localStorage.setItem("moonpool.cliVisible", "1");
      localStorage.setItem("moonpool.savedMainWidth", String(savedMainWidth));
    }
  }

  onMount(async () => {
    const saved = Number(localStorage.getItem("moonpool.sidebarWidth"));
    if (saved >= MIN_W && saved <= MAX_W) sidebarWidth = saved;

    // Restore collapsed CLI state, then reconcile with the real window width: if
    // the flag says collapsed but the window is wide (e.g. the restored width and
    // the saved flag disagree), fill the space with the CLI instead of leaving a
    // blank hole. The window width is the source of truth.
    const savedMain = Number(localStorage.getItem("moonpool.savedMainWidth"));
    if (savedMain > 0) savedMainWidth = savedMain;
    cliVisible = localStorage.getItem("moonpool.cliVisible") !== "0";
    revealCliIfRoom();
    window.addEventListener("resize", revealCliIfRoom);

    try {
      lastStarted = JSON.parse(localStorage.getItem("moonpool.lastStarted") ?? "{}");
    } catch {
      lastStarted = {};
    }

    // Fire-and-forget update check; silent on failure or when up to date.
    getSettings()
      .then((s) => {
        applyTransparency(s.transparency ?? 0);
        if (s.checkOnStartup) return updateCheck();
        return null;
      })
      .then((r) => {
        if (r) {
          currentVersion = r.current;
          if (r.available) update = r.available;
        }
      })
      .catch(() => {});

    // The detached Settings window broadcasts changes so the hub updates live.
    unlistenTransparency = await listen<number>("settings:transparency", (e) =>
      applyTransparency(e.payload),
    );
    unlistenTheme = await listen<Theme>("settings:theme", (e) =>
      setTheme(e.payload),
    );
    unlistenLocale = await watchLocale();

    apps = await getApps();
    loadAllIcons(true);
    manifestDir()
      .then((d) => (cfgDir = d))
      .catch(() => {});
    unlisten = await onStatus((list) => {
      // A desktop exe icon / web favicon only resolves once the app is running,
      // and it may have changed since the last build - so on each stopped->running
      // transition, force a fresh re-pull (not every poll, only the rising edge).
      for (const s of list) {
        const wasRunning = statuses[s.id]?.running ?? false;
        if (s.running && !wasRunning) loadIcon(s.id, true);
      }
      const m: Record<string, AppStatus> = {};
      for (const s of list) m[s.id] = s;
      statuses = m;
    });

    // External control channel (a second `moonpool.exe launch/stop/... <id>` run).
    // When the caller tagged the command with `--ticket <key>`, we wait for the
    // action to actually take effect and report ok/error back under that key
    // (it lands in state.json), so a script/agent can confirm it worked.
    unlistenControl = await onControl(async ({ action, arg, ticket }) => {
      const done = (ok: boolean, detail: string | null) => {
        if (ticket) {
          reportOutcome(ticket, action, arg, ok ? "ok" : "error", detail).catch(
            () => {},
          );
        }
      };
      try {
        const app = arg ? apps.find((a) => a.id === arg) : undefined;
        switch (action) {
          case "launch":
          case "restart": {
            if (!app) return done(false, `unknown app id: ${arg ?? ""}`);
            if (action === "restart") await handleRestart(app);
            else await handleLaunch(app);
            // A pure static entry (URL only, nothing to run) has no running
            // signal - opening it is the whole action, so call it done.
            if (!hasRunSignal(app)) return done(true, "opened");
            const ok = await waitForRunning(app.id, true, 25000);
            return done(ok, ok ? null : "did not reach running in time");
          }
          case "stop": {
            if (!app) return done(false, `unknown app id: ${arg ?? ""}`);
            await handleStop(app);
            if (!hasRunSignal(app)) return done(true, "stopped");
            const ok = await waitForRunning(app.id, false, 15000);
            return done(ok, ok ? null : "still running after stop");
          }
          case "reload":
            await handleReload();
            return done(true, null);
          case "refresh-icons":
            await loadAllIcons(true);
            return done(true, null);
          default:
            return done(false, `unknown command: ${action}`);
        }
      } catch (e) {
        done(false, String(e));
      }
    });
  });

  function startResize(e: PointerEvent) {
    e.preventDefault();
    resizing = true;
    const startX = e.clientX;
    const startW = sidebarWidth;
    const move = (ev: PointerEvent) => {
      sidebarWidth = Math.min(MAX_W, Math.max(MIN_W, startW + (ev.clientX - startX)));
    };
    const up = () => {
      resizing = false;
      window.removeEventListener("pointermove", move);
      window.removeEventListener("pointerup", up);
      localStorage.setItem("moonpool.sidebarWidth", String(sidebarWidth));
    };
    window.addEventListener("pointermove", move);
    window.addEventListener("pointerup", up);
  }

  onDestroy(() => {
    unlisten?.();
    unlistenControl?.();
    unlistenTransparency?.();
    unlistenTheme?.();
    unlistenLocale?.();
    window.removeEventListener("resize", revealCliIfRoom);
    // Clear any outstanding timers so they can't fire and set $state after unmount.
    for (const id of Object.keys(pendingTimers)) clearTimeout(pendingTimers[id]);
    for (const arr of Object.values(highlightTimers)) arr.forEach(clearTimeout);
  });

  function appById(id: string) {
    return apps.find((a) => a.id === id);
  }

  function isActive(id: string) {
    return statuses[id]?.running || statuses[id]?.managed || false;
  }

  // Clear a pending badge once the poller confirms the app reached the target state.
  $effect(() => {
    for (const id of pending) {
      const active = isActive(id); // reads `statuses` -> effect re-runs on status updates
      const t = pendingTarget[id];
      if ((t === "up" && active) || (t === "down" && !active)) clearPending(id);
    }
  });

  // Does this app have a signal the poller can use to tell it's running? A pure
  // static URL entry has none (opening it is the whole action); anything with a
  // command (managed PTY), a port, or a process name does.
  function hasRunSignal(app: AppEntry): boolean {
    return !!(app.command || app.port || app.processName);
  }

  // Poll the live status map until app `id` reaches `want` (running/stopped) or
  // the timeout elapses. Used to resolve a ticketed control command's outcome.
  async function waitForRunning(
    id: string,
    want: boolean,
    timeoutMs: number,
  ): Promise<boolean> {
    const deadline = Date.now() + timeoutMs;
    while (Date.now() < deadline) {
      if ((statuses[id]?.running ?? false) === want) return true;
      await new Promise((r) => setTimeout(r, 300));
    }
    return (statuses[id]?.running ?? false) === want;
  }

  async function handleLaunch(app: AppEntry) {
    recordStart(app.id);
    // Re-pull the icon on every launch - a rebuilt app/site may have a new one.
    loadIcon(app.id, true);
    // Static entries with only a URL just open in the browser - no terminal.
    if (app.type === "static" && !app.command) {
      if (app.url) await openUrl(app.url);
      return;
    }
    if (openTabs.includes(app.id)) {
      // Tab already exists: focus a live one, relaunch a stopped one in place.
      if (!isActive(app.id)) {
        setPending(app.id, "up");
        gen = { ...gen, [app.id]: (gen[app.id] ?? 0) + 1 };
      }
      activeTab = app.id;
      return;
    }
    setPending(app.id, "up");
    openTabs = [...openTabs, app.id];
    activeTab = app.id;
  }

  // Clicking a name focuses its terminal tab if open; it never launches.
  function handleSelect(app: AppEntry) {
    if (openTabs.includes(app.id)) activeTab = app.id;
  }

  async function handleReload() {
    apps = await reloadManifest();
    iconSrc = {};
    loadAllIcons(true);
  }
  // F5 or Ctrl/Cmd+R reloads the manifest from disk (same as the menu Reload),
  // instead of the webview's default page refresh.
  function handleGlobalKey(e: KeyboardEvent) {
    if (e.key === "F5" || ((e.ctrlKey || e.metaKey) && (e.key === "r" || e.key === "R"))) {
      e.preventDefault();
      handleReload();
    }
  }
  async function handleEdit() {
    await openManifest();
  }

  async function handleStop(app: AppEntry) {
    setPending(app.id, "down");
    await stopApp(app.id);
  }

  async function handleRestart(app: AppEntry) {
    if (app.type === "static" && !app.command) {
      if (app.url) await openUrl(app.url);
      return;
    }
    recordStart(app.id);
    setPending(app.id, "up");
    await stopApp(app.id);
    // Let the tree-kill / port-free settle before rebinding.
    await new Promise((r) => setTimeout(r, 900));
    if (!openTabs.includes(app.id)) {
      openTabs = [...openTabs, app.id];
    } else {
      gen = { ...gen, [app.id]: (gen[app.id] ?? 0) + 1 };
    }
    activeTab = app.id;
  }

  function closeTab(id: string) {
    openTabs = openTabs.filter((t) => t !== id);
    if (activeTab === id) activeTab = openTabs[openTabs.length - 1] ?? null;
  }

  // Surface manifest port collisions (two apps configured for the same port).
  let clashes = $derived.by(() => {
    const byPort: Record<number, string[]> = {};
    for (const a of apps) {
      if (a.port) (byPort[a.port] ??= []).push(a.name);
    }
    return Object.entries(byPort)
      .filter(([, names]) => names.length > 1)
      .map(([port, names]) => ({ port: Number(port), names }));
  });
</script>

<svelte:window onkeydown={handleGlobalKey} />

<div class="app" class:resizing>
  <Titlebar />
  <div class="body">
  <div class="sidebar-host" style:width="{sidebarWidth}px">
    <Sidebar
      {apps}
      {statuses}
      {pending}
      {lastStarted}
      {highlight}
      {slowReorder}
      {iconSrc}
      bind:filter
      onLaunch={handleLaunch}
      onSelect={handleSelect}
      onStop={handleStop}
      onRestart={handleRestart}
      onEdit={openEdit}
      onDelete={handleDelete}
      onRename={handleRename}
      onSetIcon={handleSetIcon}
      onAdd={openAdd}
      onEditFile={handleEdit}
      onReload={handleReload}
      onAbout={() => openAboutWindow()}
      onSettings={() => openSettingsWindow()}
      cliHidden={!cliVisible}
      onExpandCli={expandCli}
      updateWaiting={!!update && !updateDone}
      {clashes}
    />
  </div>
  {#if cliVisible}
  <div
    class="resizer"
    role="separator"
    aria-orientation="vertical"
    title={t("app.dragToResize")}
    onpointerdown={startResize}
  ></div>
  {/if}

  <!-- Kept mounted (just hidden) when collapsed, so running terminals and their
       scrollback survive a hide/show. -->
  <main class="main" class:hidden={!cliVisible}>
    <div class="tabs">
      {#each openTabs as id (id)}
        {@const a = appById(id)}
        <div class="tab" class:active={activeTab === id}>
          <button class="tab-name" onclick={() => (activeTab = id)}>
            <span class="tab-dot" class:on={statuses[id]?.running}></span>
            {a?.name ?? id}
          </button>
          <button class="tab-x" title={t("app.closeTab")} onclick={() => closeTab(id)}>&times;</button>
        </div>
      {/each}
      <button
        class="cli-collapse"
        title={t("app.hideCli")}
        aria-label={t("app.hideCli")}
        onclick={collapseCli}>&times;</button
      >
    </div>

    <section class="terminals">
      {#each openTabs as id (id + "#" + (gen[id] ?? 0))}
        <TermView {id} active={activeTab === id} />
      {/each}
      {#if openTabs.length === 0}
        <div class="placeholder">
          <div class="ph-moon" aria-hidden="true"></div>
          <p class="ph-title">{t("app.pickApp")}</p>

          {#if update}
            <div class="update-banner" class:done={updateDone}>
              <span class="ub-icon"><Icon name={updateDone ? "check" : "arrow-up"} size={15} /></span>
              <span class="ub-text">
                {#if updateStatus}
                  {updateStatus}
                {:else}
                  {t("app.updateAvailable", { version: update.version, current: currentVersion })}
                {/if}
              </span>
              {#if !updateDone}
                <button class="ub-btn" disabled={updating} onclick={installUpdate}>
                  {updating ? t("app.installing") : t("app.downloadInstall")}
                </button>
                {#if !updating}
                  <button class="ub-x" title={t("common.dismiss")} aria-label={t("common.dismiss")} onclick={() => (update = null)}>&times;</button>
                {/if}
              {/if}
            </div>
          {/if}

          <div class="quickstart">
            <div class="qs-head">
              <span>{t("app.newHere")}</span>
            </div>
            <div class="qs-box">
              <pre class="qs-prompt">{aiPrompt}</pre>
              <button
                class="qs-copy"
                title={promptCopied ? t("common.copied") : t("app.copyPrompt")}
                aria-label={t("app.copyPrompt")}
                onclick={copyPrompt}
              >
                <Icon name={promptCopied ? "check" : "copy"} size={14} />
              </button>
            </div>
            <!-- One catalog string, three inline elements. tSplit keeps the
                 clause order translatable instead of hard-coding English. -->
            <p class="qs-note">
              {#each tSplit("app.orUseHint") as c}{#if "text" in c}{c.text}{:else if c.slot === "add"}<strong
                    >{t("sidebar.addApp")}</strong
                  >{:else if c.slot === "edit"}<strong>{t("app.editFile")}</strong>{:else}<code
                    >apps.json</code
                  >{/if}{/each}
            </p>
          </div>
        </div>
      {/if}
    </section>
  </main>
  </div>

  {#if showEditor}
    <AppEditor
      entry={editingEntry}
      groups={groupNames}
      onSave={handleEditorSave}
      onDelete={handleEditorDelete}
      onClose={() => (showEditor = false)}
    />
  {/if}
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
    overflow: hidden;
  }
  .body {
    flex: 1;
    display: flex;
    min-height: 0;
    overflow: hidden;
  }
  .app.resizing {
    cursor: col-resize;
    user-select: none;
  }
  .sidebar-host {
    flex: none;
    height: 100%;
    overflow: hidden;
  }
  .resizer {
    flex: none;
    width: 5px;
    cursor: col-resize;
    background: var(--border-muted);
    transition: background 0.12s;
  }
  .resizer:hover,
  .app.resizing .resizer {
    background: var(--focus);
  }
  .main {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  .main.hidden {
    display: none;
  }
  .tabs {
    display: flex;
    gap: 2px;
    height: 34px;
    padding: 0 8px;
    background: color-mix(in srgb, var(--bg) calc(var(--app-alpha) * 100%), transparent);
    border-bottom: 1px solid var(--border-muted);
    overflow-x: auto;
    flex: none;
  }
  .tab {
    display: flex;
    align-items: center;
    background: var(--bg-inset);
    border: 1px solid var(--border-muted);
    border-bottom: none;
    border-radius: 6px 6px 0 0;
    margin-top: 4px;
    opacity: 0.7;
    transition: opacity 0.12s, background 0.12s;
  }
  .tab:hover {
    opacity: 0.9;
  }
  .tab.active {
    background: var(--bg);
    border-color: var(--dot-run);
    opacity: 1;
  }
  .tab-name {
    display: flex;
    align-items: center;
    gap: 6px;
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 12px;
    padding: 5px 4px 5px 10px;
    cursor: pointer;
    white-space: nowrap;
  }
  .tab.active .tab-name {
    color: var(--text-strong);
    font-weight: 600;
  }
  .tab-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: var(--border-strong);
  }
  .tab-dot.on {
    background: var(--dot-run);
  }
  .tab-x {
    background: none;
    border: none;
    color: var(--text-dim);
    cursor: pointer;
    font-size: 15px;
    padding: 2px 8px 2px 2px;
  }
  .tab-x:hover {
    color: var(--danger);
  }
  /* Collapse button pinned to the right end of the tabs strip. */
  .cli-collapse {
    margin-left: auto;
    align-self: center;
    flex: none;
    background: none;
    border: 1px solid transparent;
    border-radius: 6px;
    color: var(--text-dim);
    cursor: pointer;
    font-size: 16px;
    line-height: 1;
    padding: 3px 8px;
  }
  .cli-collapse:hover {
    color: var(--text-strong);
    border-color: var(--border);
    background: var(--bg-inset);
  }
  .terminals {
    flex: 1;
    position: relative;
    min-height: 0;
    /* Transparent: the .term box carries the single tint layer for a running
       CLI, and .placeholder carries the tint for the empty state. Tinting here
       too would stack alphas and darken the terminal. */
    background: transparent;
  }
  .placeholder {
    position: absolute;
    inset: 0;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: var(--text-dim);
    gap: 4px;
    overflow: auto;
    padding: 24px;
    box-sizing: border-box;
    /* Empty-state tint (the running terminal carries its own; see .terminals). */
    background: color-mix(in srgb, var(--bg) calc(var(--app-alpha) * 100%), transparent);
  }
  .ph-moon {
    font-size: 42px;
    opacity: 0.8;
    margin-bottom: 8px;
    position: relative;
    isolation: isolate; /* contain the color blend to the emoji */
  }
  /* Base moon on its own layer so its saturation can be dialed independently
     of the aqua overlay. */
  .ph-moon::before {
    content: "🌙";
    display: block;
    filter: saturate(0.4) brightness(0.95);
  }
  /* A second moon glyph, recolored to aqua, blended over the base (glyph-shaped,
     not a rectangle), so only the moon picks up the tint. */
  .ph-moon::after {
    content: "🌙";
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    filter: sepia(1) saturate(6) hue-rotate(140deg);
    mix-blend-mode: color;
    opacity: 0.32;
    pointer-events: none;
  }
  .ph-title {
    font-size: 14px;
    color: var(--text-secondary);
  }
  .ph-sub {
    font-size: 12px;
    color: var(--text-faint);
  }
  .update-banner {
    width: 100%;
    max-width: 560px;
    margin-top: 22px;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 12px;
    background: var(--bg-inset);
    border: 1px solid var(--dot-run);
    border-radius: 8px;
    color: var(--text);
    font-size: 12.5px;
  }
  .ub-icon {
    flex: none;
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    background: var(--dot-run);
    /* Dark ink, not --on-accent (white): the aqua is light in both themes. */
    color: #052a30;
    font-size: 12px;
    font-weight: 700;
  }
  .ub-text {
    flex: 1;
    line-height: 1.4;
  }
  .ub-btn {
    flex: none;
    background: var(--dot-run);
    border: 1px solid var(--dot-run);
    color: #052a30;
    font-weight: 600;
    padding: 6px 12px;
    border-radius: 6px;
    font-size: 12px;
    cursor: pointer;
  }
  .ub-btn:disabled {
    opacity: 0.6;
    cursor: default;
  }
  .ub-x {
    flex: none;
    background: none;
    border: none;
    color: var(--text-dim);
    cursor: pointer;
    font-size: 16px;
    padding: 0 4px;
  }
  .ub-x:hover {
    color: var(--text-strong);
  }
  .quickstart {
    width: 100%;
    max-width: 560px;
    margin-top: 22px;
    text-align: left;
  }
  .qs-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    font-size: 12px;
    color: var(--text-muted);
    margin-bottom: 6px;
  }
  .qs-box {
    position: relative;
  }
  .qs-copy {
    position: absolute;
    top: 8px;
    right: 8px;
    width: 26px;
    height: 26px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    color: var(--text-secondary);
    font-size: 13px;
    border-radius: 6px;
    cursor: pointer;
    opacity: 0.85;
  }
  .qs-copy:hover {
    opacity: 1;
    color: var(--text-strong);
    border-color: var(--focus);
  }
  .qs-prompt {
    margin: 0;
    background: var(--bg-inset);
    border: 1px solid var(--border-muted);
    border-radius: 8px;
    padding: 12px 40px 12px 14px;
    color: var(--text);
    font-family: "Cascadia Code", Consolas, ui-monospace, monospace;
    font-size: 11.5px;
    line-height: 1.5;
    white-space: pre-wrap;
    word-break: break-word;
    max-height: 40vh;
    overflow: auto;
  }
  .qs-note {
    font-size: 11px;
    color: var(--text-faint);
    margin-top: 10px;
  }
  /* Each TermView fills the terminals area; only the active one is shown. */
  .terminals :global(.term) {
    position: absolute;
    inset: 0;
  }
</style>
