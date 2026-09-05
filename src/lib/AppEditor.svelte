<script lang="ts">
  import type { AppEntry, AppType } from "./types";
  import Modal from "./Modal.svelte";
  import { onMount } from "svelte";
  import { t } from "./i18n.svelte";
  import { portableState, isNonPortablePath } from "./api";

  let {
    entry,
    groups,
    onSave,
    onDelete,
    onClose,
    // When true, render as bare content for a detached OS window (no Modal
    // overlay); the hosting window supplies the frame, padding and scroll.
    windowed = false,
    // Lets a hosting window guard its close button against unsaved changes.
    onDirty,
  }: {
    entry: AppEntry | null;
    groups: string[];
    onSave: (e: AppEntry, originalId?: string) => void;
    onDelete: (id: string) => void;
    onClose: () => void;
    windowed?: boolean;
    onDirty?: (dirty: boolean) => void;
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

  // Portable mode drives the amber "won't move with this folder" hints on path
  // fields. Only relevant when running portable; installed mode never warns.
  let portable = $state(false);
  onMount(() => {
    portableState()
      .then((s) => (portable = s.portable))
      .catch(() => {});
  });
  let cwdWarn = $derived(portable && isNonPortablePath(f.cwd));
  let urlWarn = $derived(portable && isNonPortablePath(f.url));

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
  // NOT translated on purpose: a chosen group is written into apps.json and
  // becomes the user's own data. Localizing the seeds would leave headings in
  // whatever language happened to be active when each app was added.
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
  // $derived, not a plain const: a const captures the strings at component
  // creation and would keep showing the old language if the user switched while
  // this modal is open. Sample commands ("npm run dev", "5173") stay as written -
  // they are literals to copy, not prose to read.
  const TYPE_INFO: Record<AppType, TypeInfo> = $derived({
    web: {
      hint: t("editor.hintWeb"),
      fields: ["cwd", "command", "port", "url", "openBrowser"],
      ph: { cwd: t("editor.phFolder"), command: "npm run dev", port: "5173", url: "http://localhost:5173" },
    },
    desktop: {
      hint: t("editor.hintDesktop"),
      fields: ["cwd", "command", "processName"],
      ph: { cwd: t("editor.phFolder"), command: t("editor.phLaunchCommand"), processName: "my-app" },
    },
    static: {
      hint: t("editor.hintStatic"),
      fields: ["url", "openBrowser"],
      ph: { url: "file:///path/to/index.html" },
    },
    cli: {
      hint: t("editor.hintCli"),
      fields: ["cwd", "command"],
      ph: { cwd: t("editor.phFolder"), command: "python script.py" },
    },
  });
  let ti = $derived(TYPE_INFO[f.type]);

  // Dirty tracking + safe close: a modified form asks before discarding. Routed
  // through Modal's onClose so overlay-click / Escape also get the confirm.
  const initialJson = JSON.stringify(f);
  let dirty = $derived(JSON.stringify(f) !== initialJson);
  // Report dirty state to a hosting window so its close button can confirm.
  $effect(() => onDirty?.(dirty));
  function maybeClose() {
    if (dirty && !confirm(t("editor.discardChanges"))) return;
    onClose();
  }

  function save() {
    if (!f.name.trim()) {
      error = t("editor.nameRequired");
      return;
    }
    const env: Record<string, string> = {};
    for (const line of f.env.split("\n")) {
      const kv = line.trim();
      const i = kv.indexOf("=");
      if (i > 0) env[kv.slice(0, i).trim()] = kv.slice(i + 1).trim();
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

{#snippet body()}
    <h2>{isNew ? t("editor.addApp") : t("editor.editApp")}</h2>

    <div class="grid">
      <!-- Field labels (name, group, cwd, ...) are the literal apps.json keys the
           user edits by hand, so they stay English. The hints beside them do not. -->
      <label title={t("editor.nameHint")}>name<input bind:value={f.name} placeholder={t("editor.namePlaceholder")} /></label>
      <label title={t("editor.groupHint")}>
        group
        {#if customGroup}
          <div class="new-group">
            <input bind:value={f.group} placeholder={t("editor.newGroupPlaceholder")} />
            <button type="button" class="back-link" title={t("editor.pickExistingGroup")} onclick={backToGroupList}>{t("editor.backToList")}</button>
          </div>
        {:else}
          <select bind:value={f.group} onchange={onGroupSelect}>
            {#each groupOptions as g (g)}<option value={g}>{g}</option>{/each}
            <option value="__new__">{t("editor.newGroupOption")}</option>
          </select>
        {/if}
      </label>
      <label title={t("editor.typeHint")}>
        type
        <select bind:value={f.type}>
          <option value="web">web</option>
          <option value="desktop">desktop</option>
          <option value="static">static</option>
          <option value="cli">cli</option>
        </select>
      </label>
      <div class="type-hint">{ti.hint}</div>
      <label class="wide" class:dim={!ti.fields.includes("cwd")} title={t("editor.cwdHint")}>
        <span class="lbl">cwd{#if cwdWarn}<span class="warn" title={t("editor.portableWarn")}>{t("editor.notPortable")}</span>{/if}</span>
        <input bind:value={f.cwd} class:warned={cwdWarn} placeholder={ti.ph.cwd ?? "path to the app folder"} />
      </label>
      <label class="wide" class:dim={!ti.fields.includes("command")} title={t("editor.commandHint")}>command<input bind:value={f.command} placeholder={ti.ph.command ?? t("editor.phCommand")} /></label>
      <label class:dim={!ti.fields.includes("port")} title={t("editor.portHint")}>port<input bind:value={f.port} placeholder={ti.ph.port ?? "3000"} /></label>
      <label class:dim={!ti.fields.includes("processName")} title={t("editor.processNameHint")}>processName<input bind:value={f.processName} placeholder={ti.ph.processName ?? "my-app"} /></label>
      <label class="wide" class:dim={!ti.fields.includes("url")} title={t("editor.urlHint")}>
        <span class="lbl">url{#if urlWarn}<span class="warn" title={t("editor.portableWarn")}>{t("editor.notPortable")}</span>{/if}</span>
        <input bind:value={f.url} class:warned={urlWarn} placeholder={ti.ph.url ?? "http://localhost:3000"} />
      </label>
      <label class="check" class:dim={!ti.fields.includes("openBrowser")} title={t("editor.openBrowserHint")}><input type="checkbox" bind:checked={f.openBrowser} /> {t("editor.openBrowser")}</label>
      <label class="wide" title={t("editor.envHint")}>{t("editor.envLabel")}<textarea bind:value={f.env} rows="2" placeholder="PORT=3000"></textarea></label>
      <label class="wide" title={t("editor.noteHint")}>note<input bind:value={f.note} placeholder={t("editor.notePlaceholder")} /></label>
    </div>

    {#if error}<div class="err">{error}</div>{/if}

    <div class="actions">
      {#if !isNew}
        <button class="btn del" onclick={() => onDelete(originalId!)}>{t("common.delete")}</button>
      {/if}
      <div class="spacer"></div>
      <button class="btn" onclick={maybeClose}>{t("common.cancel")}</button>
      <button class="btn primary" onclick={save}>{t("common.save")}</button>
    </div>
{/snippet}

<svelte:window onkeydown={(e) => windowed && e.key === "Escape" && maybeClose()} />

{#if windowed}
  {@render body()}
{:else}
  <Modal onClose={maybeClose} width="560px">
    {@render body()}
  </Modal>
{/if}

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
  .lbl {
    display: inline-flex;
    align-items: center;
    gap: 8px;
  }
  /* Amber = caveat, not error: the path still works, it just won't travel with a
     portable bundle. Passive badge on the field's label. */
  .warn {
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 0.02em;
    color: #d99a26;
    background: color-mix(in srgb, #d99a26 16%, transparent);
    border: 1px solid color-mix(in srgb, #d99a26 45%, transparent);
    border-radius: 4px;
    padding: 0 5px;
    line-height: 15px;
    cursor: help;
  }
  input.warned {
    border-color: color-mix(in srgb, #d99a26 55%, var(--border));
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
