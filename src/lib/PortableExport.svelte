<script lang="ts">
  import Modal from "./Modal.svelte";
  import {
    getApps,
    exportPortable,
    revealPath,
    isNonPortablePath,
  } from "./api";
  import { open } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";

  let { onClose }: { onClose: () => void } = $props();

  // Fresh is the primary case: drop an empty Moonpool into a folder and set it up
  // there (e.g. one self-contained copy per department).
  let mode = $state<"fresh" | "clone">("fresh");
  let folder = $state("");
  let busy = $state(false);
  let done = $state(false);
  let error = $state("");

  // Apps whose paths are absolute/machine-specific and so won't travel in the copy.
  let apps = $state<{ name: string; bad: boolean }[]>([]);
  onMount(() => {
    getApps()
      .then((list) =>
        (apps = list.map((a) => ({
          name: a.name,
          bad:
            isNonPortablePath(a.cwd) ||
            isNonPortablePath(a.command) ||
            isNonPortablePath(a.url) ||
            isNonPortablePath(a.icon),
        }))),
      )
      .catch(() => {});
  });
  let warnNames = $derived(apps.filter((a) => a.bad).map((a) => a.name));

  async function choose() {
    error = "";
    try {
      const picked = await open({
        directory: true,
        title: "Choose a folder for the portable copy",
      });
      if (typeof picked === "string") {
        folder = picked;
        done = false;
      }
    } catch (e) {
      error = String(e);
    }
  }

  async function create() {
    if (!folder || busy) return;
    busy = true;
    error = "";
    try {
      await exportPortable(folder, mode === "clone");
      done = true;
    } catch (e) {
      error = String(e);
    } finally {
      busy = false;
    }
  }
</script>

<Modal {onClose} width="460px">
  <h2>Install portable</h2>
  <p class="sub">
    Stamp a self-contained Moonpool into a folder you can zip, copy to a USB
    stick, or run on another machine. This Moonpool stays as-is.
  </p>

  <div class="opts">
    <label class="opt" class:sel={mode === "fresh"}>
      <input type="radio" bind:group={mode} value="fresh" />
      <div>
        <b>Fresh</b>
        <span>Empty copy with the example apps. Set it up from scratch in the folder.</span>
      </div>
    </label>
    <label class="opt" class:sel={mode === "clone"}>
      <input type="radio" bind:group={mode} value="clone" />
      <div>
        <b>Clone current</b>
        <span>Copy your apps, icons and settings into the portable copy.</span>
      </div>
    </label>
  </div>

  <div class="folder">
    <button class="btn" onclick={choose} disabled={busy}>Choose folder…</button>
    {#if folder}<code title={folder}>{folder}</code>{/if}
  </div>

  {#if mode === "clone" && warnNames.length}
    <div class="warn">
      ⚠ These apps use absolute paths and won't work on another machine (only
      {"{MP_HOME}"} / relative paths travel): {warnNames.join(", ")}
    </div>
  {/if}

  {#if error}<div class="err">Failed: {error}</div>{/if}

  <div class="row">
    {#if done}
      <div class="ok">✓ Portable copy created in this folder.</div>
      <button class="btn" onclick={() => revealPath(folder)}>Open folder</button>
      <button class="btn primary" onclick={onClose}>Done</button>
    {:else}
      <button class="btn" onclick={onClose}>Cancel</button>
      <button class="btn primary" disabled={!folder || busy} onclick={create}>
        {busy ? "Creating…" : "Create"}
      </button>
    {/if}
  </div>
</Modal>

<style>
  h2 {
    margin: 0 0 4px;
    font-size: 17px;
    color: var(--text-strong);
  }
  .sub {
    margin: 0 0 16px;
    font-size: 13px;
    color: var(--text-secondary);
    line-height: 1.45;
  }
  .opts {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .opt {
    display: flex;
    gap: 10px;
    align-items: flex-start;
    padding: 10px 12px;
    border: 1px solid var(--border);
    border-radius: 8px;
    cursor: pointer;
  }
  .opt.sel {
    border-color: var(--accent);
    background: color-mix(in srgb, var(--accent) 8%, transparent);
  }
  .opt input {
    margin-top: 2px;
    accent-color: var(--accent);
  }
  .opt b {
    display: block;
    font-size: 13px;
    color: var(--text-strong);
  }
  .opt span {
    display: block;
    font-size: 12px;
    color: var(--text-secondary);
    margin-top: 1px;
  }
  .folder {
    display: flex;
    align-items: center;
    gap: 10px;
    margin: 16px 0 4px;
    flex-wrap: wrap;
  }
  .folder code {
    font-size: 12px;
    color: var(--text-dim);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    max-width: 100%;
  }
  .warn {
    margin-top: 12px;
    font-size: 12px;
    line-height: 1.45;
    color: var(--warn, #d8a24a);
    background: color-mix(in srgb, var(--warn, #d8a24a) 12%, transparent);
    border: 1px solid color-mix(in srgb, var(--warn, #d8a24a) 40%, transparent);
    border-radius: 6px;
    padding: 8px 10px;
  }
  .err {
    margin-top: 12px;
    font-size: 12px;
    color: var(--danger, #f06a6a);
  }
  .ok {
    flex: 1;
    font-size: 13px;
    color: var(--text-strong);
  }
  .row {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
    align-items: center;
    margin-top: 18px;
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
  .btn:disabled {
    opacity: 0.6;
    cursor: default;
  }
</style>
