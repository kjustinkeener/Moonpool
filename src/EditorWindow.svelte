<script lang="ts">
  import AppEditor from "./lib/AppEditor.svelte";
  import { getApps } from "./lib/api";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { ask } from "@tauri-apps/plugin-dialog";
  import { emit } from "@tauri-apps/api/event";
  import { t } from "./lib/i18n.svelte";
  import { onMount } from "svelte";
  import type { AppEntry } from "./lib/types";

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
  }
</style>
