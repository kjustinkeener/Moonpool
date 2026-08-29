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
  } from "./lib/api";
  import type { AppEntry, AppStatus } from "./lib/types";
  import Sidebar from "./lib/Sidebar.svelte";
  import TermView from "./lib/TermView.svelte";
  import About from "./lib/About.svelte";
  import AppEditor from "./lib/AppEditor.svelte";
  import Settings from "./lib/Settings.svelte";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { open } from "@tauri-apps/plugin-dialog";
  import type { UnlistenFn } from "@tauri-apps/api/event";

  let apps = $state<AppEntry[]>([]);
  let statuses = $state<Record<string, AppStatus>>({});
  let openTabs = $state<string[]>([]);
  let activeTab = $state<string | null>(null);
  let filter = $state("");
  // Bump a tab's generation to force TermView to remount (fresh terminal + relaunch).
  let gen = $state<Record<string, number>>({});
  let unlisten: UnlistenFn | null = null;

  // Resolved app icons (data URI or url), fetched from the backend.
  let iconSrc = $state<Record<string, string>>({});
  async function loadIcon(id: string) {
    const src = await appIcon(id).catch(() => null);
    if (src) iconSrc = { ...iconSrc, [id]: src };
  }
  function loadAllIcons() {
    for (const a of apps) loadIcon(a.id);
  }

  // Modals + AI quickstart.
  let showAbout = $state(false);
  let showSettings = $state(false);
  let showEditor = $state(false);
  let editingEntry = $state<AppEntry | null>(null);
  let cfgDir = $state("");
  // Join filenames onto the OS-native config dir with the right separator
  // (the backend returns a Windows path with `\`, a POSIX path with `/`).
  let cfgSep = $derived(cfgDir.includes("\\") ? "\\" : "/");
  let promptCopied = $state(false);

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
    if (!confirm(`Delete "${app.name}"?`)) return;
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
      title: `Icon for ${app.name}`,
      multiple: false,
      directory: false,
      filters: [{ name: "Images", extensions: ["png", "jpg", "jpeg", "gif", "svg", "webp", "ico"] }],
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

  onMount(async () => {
    const saved = Number(localStorage.getItem("moonpool.sidebarWidth"));
    if (saved >= MIN_W && saved <= MAX_W) sidebarWidth = saved;

    try {
      lastStarted = JSON.parse(localStorage.getItem("moonpool.lastStarted") ?? "{}");
    } catch {
      lastStarted = {};
    }

    apps = await getApps();
    loadAllIcons();
    manifestDir()
      .then((d) => (cfgDir = d))
      .catch(() => {});
    unlisten = await onStatus((list) => {
      const m: Record<string, AppStatus> = {};
      for (const s of list) m[s.id] = s;
      statuses = m;
      // A desktop exe icon / web favicon only becomes resolvable once the app is
      // running, so (re)fetch icons for newly-running apps that don't have one yet.
      for (const s of list) {
        if (s.running && !iconSrc[s.id]) loadIcon(s.id);
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

  async function handleLaunch(app: AppEntry) {
    recordStart(app.id);
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
    loadAllIcons();
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

<div class="app" class:resizing>
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
      onAbout={() => (showAbout = true)}
      onSettings={() => (showSettings = true)}
      {clashes}
    />
  </div>
  <div
    class="resizer"
    role="separator"
    aria-orientation="vertical"
    title="Drag to resize"
    onpointerdown={startResize}
  ></div>

  <main class="main">
    {#if openTabs.length > 0}
      <div class="tabs">
        {#each openTabs as id (id)}
          {@const a = appById(id)}
          <div class="tab" class:active={activeTab === id}>
            <button class="tab-name" onclick={() => (activeTab = id)}>
              <span class="tab-dot" class:on={statuses[id]?.running}></span>
              {a?.name ?? id}
            </button>
            <button class="tab-x" title="Close tab" onclick={() => closeTab(id)}>&times;</button>
          </div>
        {/each}
      </div>
    {/if}

    <section class="terminals">
      {#each openTabs as id (id + "#" + (gen[id] ?? 0))}
        <TermView {id} active={activeTab === id} />
      {/each}
      {#if openTabs.length === 0}
        <div class="placeholder">
          <div class="ph-moon">🌙</div>
          <p class="ph-title">Pick an app on the left to launch it.</p>
          <p class="ph-sub">
            Its console streams here, live &mdash; type into it like a real terminal.
          </p>

          <div class="quickstart">
            <div class="qs-head">
              <span>New here? Hand this to an AI agent to set up your apps:</span>
            </div>
            <div class="qs-box">
              <pre class="qs-prompt">{aiPrompt}</pre>
              <button
                class="qs-copy"
                title={promptCopied ? "Copied!" : "Copy prompt"}
                aria-label="Copy prompt"
                onclick={copyPrompt}
              >
                {promptCopied ? "✓" : "⧉"}
              </button>
            </div>
            <p class="qs-note">
              Or use <strong>+ Add app</strong> above, or <strong>Edit file</strong> to edit
              <code>apps.json</code> directly.
            </p>
          </div>
        </div>
      {/if}
    </section>
  </main>

  {#if showAbout}
    <About onClose={() => (showAbout = false)} />
  {/if}
  {#if showSettings}
    <Settings onClose={() => (showSettings = false)} />
  {/if}
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
    height: 100vh;
    width: 100vw;
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
  .tabs {
    display: flex;
    gap: 2px;
    height: 34px;
    padding: 0 8px;
    background: var(--bg);
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
  .terminals {
    flex: 1;
    position: relative;
    min-height: 0;
    background: var(--bg);
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
  }
  .ph-moon {
    font-size: 42px;
    opacity: 0.5;
    margin-bottom: 8px;
  }
  .ph-title {
    font-size: 14px;
    color: var(--text-secondary);
  }
  .ph-sub {
    font-size: 12px;
    color: var(--text-faint);
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
