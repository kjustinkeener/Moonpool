<script lang="ts">
  import AppEditor from "./lib/AppEditor.svelte";
  import { getApps, getSettings } from "./lib/api";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { ask } from "@tauri-apps/plugin-dialog";
  import { emit, listen } from "@tauri-apps/api/event";
  import { t } from "./lib/i18n.svelte";
  import { onMount, onDestroy } from "svelte";
  import type { AppEntry } from "./lib/types";
  import type { UnlistenFn } from "@tauri-apps/api/event";

  // Same formula as App.svelte / SettingsControls.svelte: each detached window
  // reads --app-alpha off its own document, so it must apply the persisted
  // setting itself rather than inheriting it from the hub.
  function applyTransparency(pct: number) {
    const alpha = Math.max(0.1, 1 - Math.min(90, Math.max(0, pct)) / 100);
    document.documentElement.style.setProperty("--app-alpha", String(alpha));
  }
  let unlistenTransparency: UnlistenFn | null = null;

  // The app id to edit is carried in the hash after a colon (#editor:<id>);
  // a bare #editor means "add a new app".
  const editId = decodeURIComponent(window.location.hash.split(":").slice(1).join(":"));

  let ready = $state(false);
  let entry = $state<AppEntry | null>(null);
  let groups = $state<string[]>([]);
  let dirty = false;

  onMount(async () => {
    const apps = await getApps().catch(() => [] as AppEntry[]);
    groups = [...new Set(apps.map((a) => a.group))];
    entry = editId ? (apps.find((a) => a.id === editId) ?? null) : null;
    ready = true;

    getSettings()
      .then((s) => applyTransparency(s.transparency ?? 0))
      .catch(() => {});
    // The detached Settings window broadcasts changes so this window updates live.
    unlistenTransparency = await listen<number>("settings:transparency", (e) =>
      applyTransparency(e.payload),
    );

    // Native close button (X) respects unsaved-change confirmation. The webview's
    // blocking window.confirm() is suppressed in a Tauri child window, so use the
    // async dialog plugin: always preventDefault, then destroy() on confirm
    // (destroy bypasses this guard, avoiding a second prompt).
    const win = getCurrentWindow();
    await win.onCloseRequested(async (e) => {
      if (!dirty) return;
      e.preventDefault();
      if (await ask(t("editor.discardChanges"), { kind: "warning" })) {
        dirty = false;
        win.destroy().catch(() => {});
      }
    });
  });

  onDestroy(() => unlistenTransparency?.());

  function close() {
    // Clear dirty first so the onCloseRequested guard passes without re-prompting;
    // callers (Cancel/save/delete) have already handled any confirmation.
    dirty = false;
    getCurrentWindow()
      .close()
      .catch(() => {});
  }
  function handleSave(e: AppEntry, originalId?: string) {
    emit("editor://save", { entry: e, originalId });
    dirty = false;
    close();
  }
  function handleDelete(id: string) {
    emit("editor://delete", { id });
    dirty = false;
    close();
  }
</script>

<div class="page">
  {#if ready}
    <AppEditor
      {entry}
      {groups}
      windowed
      onSave={handleSave}
      onDelete={handleDelete}
      onClose={close}
      onDirty={(d: boolean) => (dirty = d)}
    />
  {/if}
</div>

<style>
  .page {
    box-sizing: border-box;
    height: 100vh;
    overflow-y: auto;
    padding: 20px 22px;
    background: color-mix(in srgb, var(--bg-panel) calc(var(--app-alpha) * 100%), transparent);
    color: var(--text);
    /* Wakes this window to full opacity on hover, then eases back to the
       configured transparency once the pointer leaves. Matches Settings/hub. */
    transition: --app-alpha 2s ease;
  }
  .page:hover {
    --app-alpha: 1;
    transition: --app-alpha 0s;
  }
</style>
