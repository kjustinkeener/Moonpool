<script lang="ts">
  import type { AppEntry, AppType } from "./types";
  import Modal from "./Modal.svelte";

  let {
    entry,
    groups,
    onSave,
    onDelete,
    onClose,
  }: {
    entry: AppEntry | null;
    groups: string[];
    onSave: (e: AppEntry, originalId?: string) => void;
    onDelete: (id: string) => void;
    onClose: () => void;
  } = $props();

  // Snapshot the prop once; this editor is remounted per open, so capturing the
  // initial value is intentional.
  const initial = entry;
  const isNew = !initial;
  const originalId = initial?.id;

  let f = $state({
    id: initial?.id ?? "",
    name: initial?.name ?? "",
    group: initial?.group ?? "Web apps",
    type: (initial?.type ?? "web") as AppType,
    cwd: initial?.cwd ?? "",
    command: initial?.command ?? "",
    port: initial?.port != null ? String(initial.port) : "",
    url: initial?.url ?? "",
    openBrowser: initial?.openBrowser ?? false,
    processName: initial?.processName ?? "",
    env: initial?.env
      ? Object.entries(initial.env)
          .map(([k, v]) => `${k}=${v}`)
          .join("\n")
      : "",
    note: initial?.note ?? "",
  });
  let error = $state("");

  // id is internal (app key + terminal-tab id + icon filename) and never shown,
  // so it's derived from the name rather than entered. App.svelte de-duplicates.
  function slugify(s: string) {
    return s
      .toLowerCase()
      .trim()
      .replace(/[^a-z0-9]+/g, "-")
      .replace(/^-+|-+$/g, "");
  }

  // Group suggestions: the conventional groups plus any already in use.
  const DEFAULT_GROUPS = ["Desktop apps", "Web apps", "Docs", "CLI tools"];
  let groupOptions = $derived([...new Set([...DEFAULT_GROUPS, ...groups])]);

  // Native <datalist> autocomplete is unreliable in WebKitGTK (Linux), so the
  // group picker is a <select> (works everywhere) plus a "New group" escape hatch
  // into a free-text input.
  let customGroup = $state(false);
  function onGroupSelect() {
    if (f.group === "__new__") {
      f.group = "";
      customGroup = true;
    }
  }
  // Escape hatch back to the group <select> (undoes "+ New group...").
  function backToGroupList() {
    f.group = groupOptions[0] ?? "";
    customGroup = false;
  }

  // Per-type guidance: a one-line hint, which fields the type actually uses, and
  // tailored placeholders. Fields not in `fields` are dimmed (still editable) so
  // it's obvious which ones matter for the selected type.
  type TypeInfo = {
    hint: string;
    fields: string[];
    ph: Partial<Record<"cwd" | "command" | "port" | "url" | "processName", string>>;
  };
  const TYPE_INFO: Record<AppType, TypeInfo> = {
    web: {
      hint: "Runs a dev server in a terminal; shows Running when its port answers, and opens the browser once it's live.",
      fields: ["cwd", "command", "port", "url", "openBrowser"],
      ph: { cwd: "path to the app folder", command: "npm run dev", port: "5173", url: "http://localhost:5173" },
    },
    desktop: {
      hint: "Launches a native app; shows Running when a process named processName is found.",
      fields: ["cwd", "command", "processName"],
      ph: { cwd: "path to the app folder", command: "the app's launch command", processName: "my-app" },
    },
    static: {
      hint: "Just opens url in the browser, no terminal or command.",
      fields: ["url", "openBrowser"],
      ph: { url: "file:///path/to/index.html" },
    },
    cli: {
      hint: "Runs a command and keeps an interactive shell open in cwd.",
      fields: ["cwd", "command"],
      ph: { cwd: "path to the folder", command: "python script.py" },
    },
  };
  let ti = $derived(TYPE_INFO[f.type]);

  // Dirty tracking + safe close: a modified form asks before discarding. Routed
  // through Modal's onClose so overlay-click / Escape also get the confirm.
  const initialJson = JSON.stringify(f);
  let dirty = $derived(JSON.stringify(f) !== initialJson);
  function maybeClose() {
    if (dirty && !confirm("Discard your changes?")) return;
    onClose();
  }

  function save() {
    if (!f.name.trim()) {
      error = "name is required.";
      return;
    }
    const env: Record<string, string> = {};
    for (const line of f.env.split("\n")) {
      const t = line.trim();
      const i = t.indexOf("=");
      if (i > 0) env[t.slice(0, i).trim()] = t.slice(i + 1).trim();
    }
    const e: AppEntry = {
      id: isNew ? slugify(f.name) || "app" : originalId!,
      name: f.name.trim(),
      group: f.group.trim() || "Apps",
      type: f.type,
      cwd: f.cwd.trim() || undefined,
      command: f.command.trim() || undefined,
      port: f.port.trim() ? Number(f.port) : undefined,
      url: f.url.trim() || undefined,
      openBrowser: f.openBrowser,
      processName: f.processName.trim() || undefined,
      env: Object.keys(env).length ? env : undefined,
      note: f.note.trim() || undefined,
    };
    onSave(e, originalId);
  }
</script>

<Modal onClose={maybeClose} width="560px">
    <h2>{isNew ? "Add app" : "Edit app"}</h2>

    <div class="grid">
      <label title="The display name shown in the sidebar. Required.">name<input bind:value={f.name} placeholder="My App" /></label>
      <label title="The sidebar heading this app is listed under. Pick an existing group or choose '+ New group' to add one.">
        group
        {#if customGroup}
          <div class="new-group">
            <input bind:value={f.group} placeholder="New group name" />
            <button type="button" class="back-link" title="Pick an existing group" onclick={backToGroupList}>back to list</button>
          </div>
        {:else}
          <select bind:value={f.group} onchange={onGroupSelect}>
            {#each groupOptions as g (g)}<option value={g}>{g}</option>{/each}
            <option value="__new__">+ New group...</option>
          </select>
        {/if}
      </label>
      <label title="How Moonpool runs and tracks the app. web = dev server on a port. desktop = native app tracked by process name. static = just opens a URL. cli = runs a command in a terminal.">
        type
        <select bind:value={f.type}>
          <option value="web">web</option>
          <option value="desktop">desktop</option>
          <option value="static">static</option>
          <option value="cli">cli</option>
        </select>
      </label>
      <div class="type-hint">{ti.hint}</div>
      <label class="wide" class:dim={!ti.fields.includes("cwd")} title="The working directory the command runs in - usually the app's project folder. Use an absolute path.">cwd<input bind:value={f.cwd} placeholder={ti.ph.cwd ?? "path to the app folder"} /></label>
      <label class="wide" class:dim={!ti.fields.includes("command")} title="The command run in the embedded terminal to start the app, e.g. 'npm run dev' or 'python app.py'. Leave blank for a static URL-only entry.">command<input bind:value={f.command} placeholder={ti.ph.command ?? "command to run"} /></label>
      <label class:dim={!ti.fields.includes("port")} title="The local TCP port the app listens on. Moonpool shows Running when this port answers, and frees it on Stop. Used by web apps.">port<input bind:value={f.port} placeholder={ti.ph.port ?? "3000"} /></label>
      <label class:dim={!ti.fields.includes("processName")} title="For desktop apps: the process/executable name (without .exe) used to detect Running and to stop it. On Linux it must be 15 characters or fewer.">processName<input bind:value={f.processName} placeholder={ti.ph.processName ?? "my-app"} /></label>
      <label class="wide" class:dim={!ti.fields.includes("url")} title="The URL to open: http://localhost:<port> for a web app, or file:///path/to/index.html for a static page.">url<input bind:value={f.url} placeholder={ti.ph.url ?? "http://localhost:3000"} /></label>
      <label class="check" class:dim={!ti.fields.includes("openBrowser")} title="Automatically open the URL in your default browser when the app becomes reachable."><input type="checkbox" bind:checked={f.openBrowser} /> open browser</label>
      <label class="wide" title="Environment variables passed to the command, one KEY=VALUE per line (e.g. PORT=3000).">env (KEY=VALUE per line)<textarea bind:value={f.env} rows="2" placeholder="PORT=3000"></textarea></label>
      <label class="wide" title="Optional text shown as a tooltip when you hover this app in the sidebar.">note<input bind:value={f.note} placeholder="optional tooltip" /></label>
    </div>

    {#if error}<div class="err">{error}</div>{/if}

    <div class="actions">
      {#if !isNew}
        <button class="btn del" onclick={() => onDelete(originalId!)}>Delete</button>
      {/if}
      <div class="spacer"></div>
      <button class="btn" onclick={maybeClose}>Cancel</button>
      <button class="btn primary" onclick={save}>Save</button>
    </div>
</Modal>

<style>
  h2 {
    margin: 0 0 14px;
    font-size: 16px;
    color: var(--text-strong);
  }
  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
  }
  label {
    display: flex;
    flex-direction: column;
    gap: 4px;
    font-size: 11px;
    color: var(--text-muted);
  }
  label.wide {
    grid-column: 1 / -1;
  }
  label.check {
    flex-direction: row;
    align-items: center;
    gap: 6px;
    font-size: 12px;
  }
  .type-hint {
    grid-column: 1 / -1;
    font-size: 11px;
    line-height: 1.4;
    color: var(--text-muted);
    margin: -2px 0 2px;
  }
  label.dim {
    opacity: 0.45;
  }
  input,
  select,
  textarea {
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 5px;
    color: var(--text);
    padding: 6px 8px;
    font-size: 13px;
    font-family: inherit;
    outline: none;
  }
  input:focus,
  select:focus,
  textarea:focus {
    border-color: var(--focus);
  }
  label.check input {
    width: auto;
  }
  .new-group {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .new-group input {
    flex: 1;
    min-width: 0;
  }
  .back-link {
    flex: none;
    background: none;
    border: none;
    color: var(--link);
    font-size: 11px;
    padding: 2px 4px;
    cursor: pointer;
    white-space: nowrap;
  }
  .back-link:hover {
    text-decoration: underline;
  }
  .err {
    color: var(--danger);
    font-size: 12px;
    margin-top: 10px;
  }
  .actions {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 16px;
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
  .btn.del {
    color: var(--danger);
    border-color: var(--danger-border);
  }
</style>
