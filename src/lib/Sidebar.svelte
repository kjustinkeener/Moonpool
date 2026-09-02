<script lang="ts">
  import type { AppEntry, AppStatus, AppType } from "./types";
  import { flip } from "svelte/animate";
  import { onMount } from "svelte";
  import { scrollFade } from "./scrollfade";
  import { portableState, performInstall, launchInstalledAndExit } from "./api";

  let {
    apps,
    statuses,
    pending,
    lastStarted,
    highlight,
    slowReorder,
    iconSrc,
    filter = $bindable(""),
    onLaunch,
    onSelect,
    onStop,
    onRestart,
    onEdit,
    onDelete,
    onRename,
    onSetIcon,
    onAdd,
    onEditFile,
    onReload,
    onAbout,
    onSettings,
    cliHidden = false,
    onExpandCli,
    updateWaiting = false,
    clashes,
  }: {
    apps: AppEntry[];
    statuses: Record<string, AppStatus>;
    pending: Set<string>;
    lastStarted: Record<string, number>;
    highlight: Set<string>;
    slowReorder: boolean;
    iconSrc: Record<string, string>;
    filter: string;
    onLaunch: (app: AppEntry) => void;
    onSelect: (app: AppEntry) => void;
    onStop: (app: AppEntry) => void;
    onRestart: (app: AppEntry) => void;
    onEdit: (app: AppEntry) => void;
    onDelete: (app: AppEntry) => void;
    onRename: (app: AppEntry, name: string) => void;
    onSetIcon: (app: AppEntry) => void;
    onAdd: () => void;
    onEditFile: () => void;
    onReload: () => void;
    onAbout: () => void;
    onSettings: () => void;
    cliHidden?: boolean;
    onExpandCli?: () => void;
    updateWaiting?: boolean;
    clashes: { port: number; names: string[] }[];
  } = $props();

  let menuOpen = $state(false);

  // "Install on this machine" is offered only when running portable: it copies this
  // exe into %LOCALAPPDATA%\Moonpool (shortcuts + uninstall entry) and relaunches the
  // installed copy. The portable folder is left untouched.
  let portable = $state(false);
  let installing = $state(false);
  onMount(() => {
    portableState()
      .then((s) => (portable = s.portable))
      .catch(() => {});
  });
  async function doInstall() {
    menuOpen = false;
    if (installing) return;
    if (
      !confirm(
        "Install Moonpool into this PC's AppData and add Start-menu/desktop shortcuts?\n\nYour portable folder stays as-is; the installed copy runs independently.",
      )
    )
      return;
    installing = true;
    try {
      const exe = await performInstall(true);
      await launchInstalledAndExit(exe);
    } catch (e) {
      installing = false;
      alert("Install failed: " + e);
    }
  }

  function pick(fn: () => void) {
    menuOpen = false;
    fn();
  }

  const typeIcon: Record<AppType, string> = {
    desktop: "▢", // window
    web: "◉", // server
    static: "◈", // page
    cli: "›", // prompt
  };

  // Apps whose bundled icon failed to load fall back to the type glyph.
  let iconFailed = $state<Set<string>>(new Set());
  function markFailed(id: string) {
    if (!iconFailed.has(id)) {
      const s = new Set(iconFailed);
      s.add(id);
      iconFailed = s;
    }
  }

  // Collapsed groups (persisted); click a group header to toggle.
  let collapsed = $state<Set<string>>(new Set());
  try {
    collapsed = new Set(JSON.parse(localStorage.getItem("moonpool.collapsedGroups") ?? "[]"));
  } catch {
    collapsed = new Set();
  }
  function toggleGroup(g: string) {
    const s = new Set(collapsed);
    s.has(g) ? s.delete(g) : s.add(g);
    collapsed = s;
    localStorage.setItem("moonpool.collapsedGroups", JSON.stringify([...s]));
  }

  let groups = $derived.by(() => {
    const q = filter.trim().toLowerCase();
    const matched = apps.filter(
      (a) => !q || a.name.toLowerCase().includes(q) || a.group.toLowerCase().includes(q),
    );
    const order: string[] = [];
    const map: Record<string, AppEntry[]> = {};
    for (const a of matched) {
      if (!map[a.group]) {
        map[a.group] = [];
        order.push(a.group);
      }
      map[a.group].push(a);
    }
    // Within each group, most-recently-started first; never-started keep manifest order
    // (stable sort, since their timestamps tie at 0).
    return order.map((g) => ({
      group: g,
      apps: [...map[g]].sort((x, y) => (lastStarted[y.id] ?? 0) - (lastStarted[x.id] ?? 0)),
    }));
  });

  function isRunning(a: AppEntry) {
    return statuses[a.id]?.running ?? false;
  }
  // "Active" = running OR owned by us but not yet detected up (starting/building).
  function isActive(a: AppEntry) {
    const s = statuses[a.id];
    return (s?.running ?? false) || (s?.managed ?? false);
  }
  function isStarting(a: AppEntry) {
    const s = statuses[a.id];
    return (s?.managed ?? false) && !(s?.running ?? false);
  }

  // Right-click context menu, positioned at the cursor.
  let ctx = $state<{ app: AppEntry; x: number; y: number } | null>(null);
  function openCtx(e: MouseEvent, app: AppEntry) {
    e.preventDefault();
    ctx = { app, x: e.clientX, y: e.clientY };
  }
  function closeCtx() {
    ctx = null;
  }
  function ctxEdit() {
    if (ctx) onEdit(ctx.app);
    closeCtx();
  }
  function ctxRename() {
    if (ctx) startRename(ctx.app);
    closeCtx();
  }
  function ctxSetIcon() {
    if (ctx) onSetIcon(ctx.app);
    closeCtx();
  }
  function ctxDelete() {
    if (ctx) onDelete(ctx.app);
    closeCtx();
  }

  // Inline rename (from the context menu): the name becomes an editable field.
  let renamingId = $state<string | null>(null);
  let renameValue = $state("");
  function startRename(app: AppEntry) {
    renamingId = app.id;
    renameValue = app.name;
  }
  function commitRename(app: AppEntry) {
    if (renamingId !== app.id) return;
    renamingId = null;
    onRename(app, renameValue);
  }
  function autofocus(node: HTMLInputElement) {
    node.focus();
    node.select();
  }
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && (ctx ? closeCtx() : (renamingId = null))} />

<aside class="sidebar">
  <div class="search">
    <div class="menu-wrap">
      <button class="menu-btn" title="Menu" onclick={() => (menuOpen = !menuOpen)}>⋯</button>
      {#if menuOpen}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <div class="menu-backdrop" role="presentation" onclick={() => (menuOpen = false)}></div>
        <div class="menu">
          <button onclick={() => pick(onAdd)}>Add app</button>
          <button onclick={() => pick(onEditFile)}>Edit apps.json</button>
          <button onclick={() => pick(onReload)}>Reload</button>
          <button onclick={() => pick(onSettings)}>Settings</button>
          <button onclick={() => pick(onAbout)}>About</button>
          {#if portable}
            <button onclick={doInstall} disabled={installing}>
              {installing ? "Installing…" : "Install Moonpool"}
            </button>
          {/if}
          {#each clashes as c (c.port)}
            <div class="menu-warn" title={c.names.join(" and ") + " are both on port " + c.port}>
              ⚠ port {c.port}: {c.names.join(" / ")}
            </div>
          {/each}
        </div>
      {/if}
    </div>
    <input placeholder="Filter apps..." aria-label="Filter apps" bind:value={filter} />
    {#if cliHidden}
      <button
        class="cli-expand"
        class:attention={updateWaiting}
        title={updateWaiting ? "Update available, open to install" : "Show CLI pane"}
        aria-label={updateWaiting ? "Update available, show CLI pane" : "Show CLI pane"}
        onclick={() => onExpandCli?.()}
      >
        <svg viewBox="0 0 24 24" width="15" height="15" aria-hidden="true">
          <path
            d="M9 6l6 6-6 6"
            fill="none"
            stroke="currentColor"
            stroke-width="2.5"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </button>
    {/if}
  </div>
  <div class="scroll" use:scrollFade>
    {#each groups as g (g.group)}
      <button class="group-label" onclick={() => toggleGroup(g.group)}>
        <span class="glabel">{g.group}</span>
        <span class="gcount">{g.apps.length}</span>
      </button>
      {#if !collapsed.has(g.group)}
        {#each g.apps as a (a.id)}
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="row"
          class:running={isRunning(a)}
          class:glowing={highlight.has(a.id)}
          animate:flip={{ duration: slowReorder ? 500 : 250 }}
          oncontextmenu={(e) => openCtx(e, a)}
        >
          <span
            class="dot"
            class:on={isRunning(a)}
            class:starting={isStarting(a)}
            title={isRunning(a) ? "running" : isStarting(a) ? "starting..." : "stopped"}
          ></span>
          {#if iconSrc[a.id] && !iconFailed.has(a.id)}
            <img
              class="app-icon"
              src={iconSrc[a.id]}
              alt=""
              title={a.type}
              onerror={() => markFailed(a.id)}
            />
          {:else}
            <span class="type" title={a.type}>{typeIcon[a.type] ?? "•"}</span>
          {/if}
          {#if renamingId === a.id}
            <input
              class="rename-input"
              value={renameValue}
              oninput={(e) => (renameValue = e.currentTarget.value)}
              onkeydown={(e) => {
                if (e.key === "Enter") commitRename(a);
                else if (e.key === "Escape") renamingId = null;
              }}
              onblur={() => commitRename(a)}
              use:autofocus
            />
          {:else}
            <button class="name" title={a.note ?? a.name} onclick={() => onSelect(a)}>
              {a.name}
              {#if a.port}<span class="port">:{a.port}</span>{/if}
            </button>
          {/if}
          <button class="action edit" title="Edit" onclick={() => onEdit(a)}>&#9998;</button>
          {#if pending.has(a.id)}
            <span class="action spinner" title="Working..."></span>
          {:else if isActive(a)}
            <button class="action restart" title="Restart" onclick={() => onRestart(a)}>&#8635;</button>
            <button class="action stop" title="Stop" onclick={() => onStop(a)}>&#9632;</button>
          {:else}
            <button class="action go" title="Launch" onclick={() => onLaunch(a)}>&#9654;</button>
          {/if}
        </div>
        {/each}
      {/if}
    {/each}
    {#if groups.length === 0}
      <div class="empty">No apps match "{filter}".</div>
    {/if}
  </div>

  {#if ctx}
    <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
    <div
      class="ctx-backdrop"
      role="presentation"
      onclick={closeCtx}
      oncontextmenu={(e) => {
        e.preventDefault();
        closeCtx();
      }}
    ></div>
    <div class="ctx-menu" style="left: {ctx.x}px; top: {ctx.y}px;">
      <button onclick={ctxEdit}>Edit</button>
      <button onclick={ctxRename}>Rename</button>
      <button onclick={ctxSetIcon}>Set icon...</button>
      <button class="danger" onclick={ctxDelete}>Delete</button>
    </div>
  {/if}
</aside>

<style>
  .sidebar {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    background: color-mix(in srgb, var(--bg-panel) calc(var(--app-alpha) * 100%), transparent);
  }
  .search {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px;
    border-bottom: 1px solid var(--border-muted);
  }
  .search input {
    flex: 1;
    min-width: 0;
    box-sizing: border-box;
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 6px;
    color: var(--text);
    padding: 7px 9px;
    font-size: 13px;
    outline: none;
  }
  .search input:focus {
    border-color: var(--focus);
  }
  .cli-expand {
    flex: none;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 6px;
    color: var(--text-secondary);
    cursor: pointer;
    padding: 0;
  }
  .cli-expand:hover {
    color: var(--text-strong);
    border-color: var(--focus);
  }
  /* Update waiting behind the collapsed pane: pulse the button so the user knows
     to open it and install. Uses the same aqua as the update banner. */
  .cli-expand.attention {
    color: #052a30;
    background: var(--dot-run);
    border-color: var(--dot-run);
    animation: cli-pulse 1.6s ease-in-out infinite;
  }
  .cli-expand.attention:hover {
    color: #052a30;
    border-color: var(--dot-run);
  }
  @keyframes cli-pulse {
    0%,
    100% {
      box-shadow: 0 0 0 0 var(--dot-run-glow);
    }
    50% {
      box-shadow: 0 0 8px 3px var(--dot-run-glow);
    }
  }
  @media (prefers-reduced-motion: reduce) {
    .cli-expand.attention {
      animation: none;
      box-shadow: 0 0 8px 2px var(--dot-run-glow);
    }
  }
  .menu-wrap {
    position: relative;
    flex: none;
  }
  .menu-btn {
    background: var(--bg);
    border: 1px solid var(--border);
    color: var(--text-secondary);
    border-radius: 6px;
    width: 30px;
    height: 30px;
    font-size: 16px;
    line-height: 1;
    cursor: pointer;
  }
  .menu-btn:hover {
    color: var(--text-strong);
    border-color: var(--focus);
  }
  .menu-backdrop {
    position: fixed;
    inset: 0;
    z-index: 40;
  }
  .menu {
    position: absolute;
    top: 34px;
    left: 0;
    z-index: 41;
    min-width: 160px;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 4px;
    box-shadow: 0 8px 24px var(--shadow);
  }
  .menu button {
    display: block;
    width: 100%;
    text-align: left;
    background: none;
    border: none;
    color: var(--text);
    font-size: 13px;
    padding: 7px 10px;
    border-radius: 5px;
    cursor: pointer;
  }
  .menu button:hover {
    background: var(--accent);
    color: var(--on-accent);
  }
  .menu-warn {
    font-size: 11px;
    color: var(--warning);
    padding: 6px 10px 2px;
    border-top: 1px solid var(--border);
    margin-top: 4px;
  }
  .scroll {
    flex: 1;
    overflow-y: auto;
    padding: 6px 0 16px;
  }
  .group-label {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    background: none;
    border: none;
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-dim);
    padding: 12px 12px 4px;
    cursor: pointer;
    text-align: left;
  }
  .group-label:hover {
    color: var(--text-secondary);
  }
  .glabel {
    flex: 1;
  }
  .gcount {
    font-size: 10px;
    color: var(--text-faint);
  }
  .row {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 3px 10px 3px 14px;
  }
  .row:hover {
    background: var(--bg-elevated);
  }
  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--border-strong);
    flex: none;
  }
  .dot.on {
    background: var(--dot-run);
    box-shadow: 0 0 6px var(--dot-run-glow);
  }
  .dot.starting {
    background: var(--dot-warn);
    box-shadow: 0 0 6px var(--dot-warn-glow);
    animation: pulse 1s ease-in-out infinite;
  }
  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.35;
    }
  }
  .type {
    color: var(--text-dim);
    font-size: 13px;
    width: 16px;
    text-align: center;
    flex: none;
  }
  .app-icon {
    width: 16px;
    height: 16px;
    border-radius: 3px;
    object-fit: contain;
    flex: none;
    image-rendering: auto;
  }
  .name {
    flex: 1;
    text-align: left;
    background: none;
    border: none;
    color: var(--text);
    font-size: 13px;
    padding: 4px 2px;
    cursor: pointer;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    transition: filter 0.5s ease, color 0.5s ease;
  }
  .name:hover {
    color: var(--text-strong);
  }
  /* "Just launched, rising to the top" glow (fades in/out over 1s, see App.svelte). */
  .row.glowing .name {
    color: #fff;
    filter: drop-shadow(0 0 1px rgba(255, 255, 255, 0.26)) drop-shadow(0 0 2px rgba(255, 255, 255, 0.26));
    /* Faster fade-in than fade-out: the base .name transition (0.5s) governs fade-out. */
    transition: filter 0.25s ease, color 0.25s ease;
  }
  .rename-input {
    flex: 1;
    min-width: 0;
    background: var(--bg);
    border: 1px solid var(--focus);
    border-radius: 4px;
    color: var(--text);
    font-family: inherit;
    font-size: 13px;
    padding: 3px 4px;
    outline: none;
  }
  .ctx-backdrop {
    position: fixed;
    inset: 0;
    z-index: 200;
  }
  .ctx-menu {
    position: fixed;
    z-index: 201;
    min-width: 150px;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 4px;
    box-shadow: 0 8px 24px var(--shadow);
    display: flex;
    flex-direction: column;
  }
  .ctx-menu button {
    text-align: left;
    background: none;
    border: none;
    color: var(--text);
    font-size: 13px;
    font-family: inherit;
    padding: 6px 10px;
    border-radius: 4px;
    cursor: pointer;
  }
  .ctx-menu button:hover {
    background: var(--border-muted);
  }
  .ctx-menu button.danger {
    color: var(--danger);
  }
  .port {
    color: var(--text-dim);
    font-size: 11px;
    margin-left: 4px;
  }
  .action {
    flex: none;
    background: none;
    border: none;
    cursor: pointer;
    font-size: 11px;
    padding: 4px 6px;
    border-radius: 4px;
    color: var(--text-dim);
  }
  .action.go:hover {
    color: var(--success);
    background: var(--success-bg);
  }
  .action.stop:hover {
    color: var(--danger);
    background: var(--danger-bg);
  }
  .action.restart:hover {
    color: var(--link);
    background: var(--info-bg);
  }
  .action.edit {
    opacity: 0;
    transition: opacity 0.1s;
  }
  .row:hover .action.edit {
    opacity: 0.7;
  }
  .action.edit:hover {
    opacity: 1;
    color: var(--link);
    background: var(--info-bg);
  }
  .action.spinner {
    box-sizing: border-box;
    width: 12px;
    height: 12px;
    padding: 0;
    margin: 4px 6px;
    border-radius: 50%;
    border: 2px solid var(--border-strong);
    border-top-color: var(--link);
    animation: spin 0.7s linear infinite;
  }
  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }
  .empty {
    color: var(--text-dim);
    font-size: 13px;
    padding: 20px 14px;
  }
</style>
