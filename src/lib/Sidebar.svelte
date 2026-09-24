<script lang="ts">
  import type { AppEntry, AppStatus, AppType } from "./types";
  import { flip } from "svelte/animate";
  import { scrollFade } from "./scrollfade";
  import Icon from "./Icon.svelte";
  import { openInstallerWindow } from "./api";
  import { t, formatList } from "./i18n.svelte";

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
    onStopMcp,
    onEdit,
    onDelete,
    onRename,
    onSetIcon,
    onAdd,
    onEditFile,
    onReload,
    onAbout,
    onSettings,
    onHelp,
    cliHidden = false,
    onExpandCli,
    updateWaiting = false,
    showMcpProcesses = true,
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
    onStopMcp: (app: AppEntry) => void;
    onEdit: (app: AppEntry) => void;
    onDelete: (app: AppEntry) => void;
    onRename: (app: AppEntry, name: string) => void;
    onSetIcon: (app: AppEntry) => void;
    onAdd: () => void;
    onEditFile: () => void;
    onReload: () => void;
    onAbout: () => void;
    onSettings: () => void;
    onHelp: () => void;
    cliHidden?: boolean;
    onExpandCli?: () => void;
    updateWaiting?: boolean;
    showMcpProcesses?: boolean;
    clashes: { port: number; names: string[] }[];
  } = $props();

  let menuOpen = $state(false);

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

  function dismissMenu(event: PointerEvent) {
    const target = event.target;
    if (menuOpen && (!(target instanceof Element) || !target.closest(".menu-wrap"))) {
      menuOpen = false;
    }
  }
</script>

<svelte:window
  onkeydown={(e) => e.key === "Escape" && (ctx ? closeCtx() : (renamingId = null))}
  onpointerdown={dismissMenu}
/>

<aside class="sidebar">
  <div class="search">
    <div class="menu-wrap">
      <button class="menu-btn" title={t("sidebar.menu")} onclick={() => (menuOpen = !menuOpen)}><Icon name="more-horizontal" size={16} /></button>
      {#if menuOpen}
        <div class="menu">
          <button onclick={() => pick(onAdd)}><span class="mi"><Icon name="plus" /></span>{t("sidebar.addApp")}</button>
          <button onclick={() => pick(onEditFile)}><span class="mi"><Icon name="pencil" /></span>{t("sidebar.editJson")}</button>
          <button onclick={() => pick(onReload)}><span class="mi"><Icon name="refresh" /></span>{t("common.reload")}</button>
          <button onclick={() => pick(onSettings)}><span class="mi"><Icon name="sliders" /></span>{t("common.settings")}</button>
          <button onclick={() => pick(onHelp)}><span class="mi"><Icon name="help" /></span>{t("common.help")}</button>
          <button onclick={() => pick(onAbout)}><span class="mi"><Icon name="info" /></span>{t("common.about")}</button>
          <button onclick={() => pick(openInstallerWindow)}>
            <span class="mi"><Icon name="monitor" /></span>{t("sidebar.installMoonpool")}
          </button>
          {#each clashes as c (c.port)}
            <!-- formatList, not a hard-coded " and ": the separator and the
                 final conjunction differ per language. -->
            <div class="menu-warn" title={t("sidebar.portConflict", { names: formatList(c.names), port: c.port })}>
              <Icon name="warning" size={13} />
              <span>{t("sidebar.portConflictBadge", { port: c.port, names: c.names.join(" / ") })}</span>
            </div>
          {/each}
        </div>
      {/if}
    </div>
    <input
      placeholder={t("sidebar.filterPlaceholder")}
      aria-label={t("sidebar.filterLabel")}
      bind:value={filter}
    />
    {#if cliHidden}
      <button
        class="cli-expand"
        class:attention={updateWaiting}
        title={updateWaiting ? t("sidebar.updateOpenToInstall") : t("sidebar.showCli")}
        aria-label={updateWaiting ? t("sidebar.updateShowCli") : t("sidebar.showCli")}
        onclick={() => onExpandCli?.()}
      >
        <Icon name="chevron-right" size={15} width={2.5} />
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
        <div class="app-block" animate:flip={{ duration: slowReorder ? 500 : 250 }}>
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div
          class="row"
          class:running={isRunning(a)}
          class:glowing={highlight.has(a.id)}
          oncontextmenu={(e) => openCtx(e, a)}
        >
          <span
            class="dot"
            class:on={isRunning(a)}
            class:starting={isStarting(a)}
            title={isRunning(a)
              ? t("sidebar.statusRunning")
              : isStarting(a)
                ? t("sidebar.statusStarting")
                : t("sidebar.statusStopped")}
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
          <button class="action edit" title={t("common.edit")} onclick={() => onEdit(a)}>&#9998;</button>
          {#if pending.has(a.id)}
            <span class="action spinner" title={t("sidebar.working")}></span>
          {:else}
            <!-- Always available: a restart on a stopped app just launches it,
                 so there's one control that works no matter the app's state. -->
            <button class="action restart" title={t("sidebar.restart")} onclick={() => onRestart(a)}>&#8635;</button>
            {#if isActive(a)}
              <button class="action stop" title={t("sidebar.stop")} onclick={() => onStop(a)}>&#9632;</button>
            {:else}
              <button class="action go" title={t("sidebar.launch")} onclick={() => onLaunch(a)}>&#9654;</button>
            {/if}
          {/if}
        </div>
        {#if showMcpProcesses && statuses[a.id]?.mcpSeen}
          <div class="row mcp-row">
            <span
              class="dot"
              class:on={statuses[a.id]?.mcpRunning}
              title={statuses[a.id]?.mcpRunning ? t("sidebar.mcpRunning") : t("sidebar.statusStopped")}
            ></span>
            <span class="type mcp-type" title={t("sidebar.mcpProcess")}>&#9492;</span>
            <span class="name mcp-name">{t("sidebar.mcpProcess")}</span>
            {#if statuses[a.id]?.mcpRunning}
              <button class="action stop" title={t("sidebar.stop")} onclick={() => onStopMcp(a)}>&#9632;</button>
            {/if}
          </div>
        {/if}
        </div>
        {/each}
      {/if}
    {/each}
    {#if groups.length === 0}
      <div class="empty">{t("sidebar.noMatch", { filter })}</div>
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
      <button onclick={ctxEdit}>{t("common.edit")}</button>
      <button onclick={ctxRename}>{t("common.rename")}</button>
      <button onclick={ctxSetIcon}>{t("sidebar.setIcon")}</button>
      <button class="danger" onclick={ctxDelete}>{t("common.delete")}</button>
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
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
  }
  .menu-btn:hover {
    color: var(--text-strong);
    border-color: var(--focus);
  }
  /* The menu is absolutely positioned inside `.menu-wrap`, which is only as wide
     as the 30px button, so shrink-to-fit would otherwise wrap every label to
     min-content. `width: max-content` sizes it to the longest row instead and
     lets it overhang the button, which is what a dropdown should do anyway. It
     matters more in translation: "Install Moonpool" is two words in English and
     considerably longer in German and Polish. `max-width` keeps a long label
     from running off the window, and only then is wrapping allowed. */
  .menu {
    position: absolute;
    top: 34px;
    left: 0;
    z-index: 41;
    width: max-content;
    min-width: 160px;
    max-width: min(320px, calc(100vw - 24px));
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 4px;
    box-shadow: 0 8px 24px var(--shadow);
  }
  .menu button {
    display: flex;
    align-items: center;
    gap: 9px;
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
  .menu .mi {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 16px;
    flex: none;
    color: var(--text-muted);
  }
  .menu button:hover .mi {
    color: inherit;
  }
  .menu button:hover {
    background: var(--accent);
    color: var(--on-accent);
  }
  .menu-warn {
    display: flex;
    align-items: flex-start;
    gap: 5px;
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
  .app-block {
    display: block;
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
  /* Reads as a child of the row above it: same left padding (so the dot and
     stop button land in the same columns as a normal row's), no top border, a
     faint connector glyph in the icon slot, and a dimmer/smaller label. */
  .mcp-row {
    padding-top: 0;
    margin-top: -2px;
    opacity: 0.7;
  }
  .mcp-row .dot {
    width: 6px;
    height: 6px;
  }
  .mcp-type {
    font-size: 13px;
    color: var(--text-faint);
  }
  .mcp-name {
    cursor: default;
    font-size: 11.5px;
    font-style: italic;
    color: var(--text-dim);
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
