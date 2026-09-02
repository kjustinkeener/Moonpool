<script lang="ts">
  import { onMount } from "svelte";
  import { getVersion } from "@tauri-apps/api/app";
  import { openUrl, updateCheck, updateApply } from "./api";
  import Modal from "./Modal.svelte";

  const credits = [
    { name: "Tauri", url: "https://tauri.app" },
    { name: "Svelte", url: "https://svelte.dev" },
    { name: "Vite", url: "https://vite.dev" },
    { name: "xterm.js", url: "https://xtermjs.org" },
    { name: "portable-pty", url: "https://crates.io/crates/portable-pty" },
  ];

  let { onClose }: { onClose: () => void } = $props();

  let version = $state("");
  let status = $state("");
  let checking = $state(false);

  onMount(async () => {
    try {
      version = await getVersion();
    } catch {
      version = "?";
    }
  });

  async function checkUpdates() {
    checking = true;
    status = "Checking for updates...";
    try {
      const r = await updateCheck();
      if (r.available) {
        status = `Update ${r.available.version} available - downloading...`;
        // On success the backend relaunches and exits; this won't return.
        await updateApply(r.available);
        status = "Update installed. Restarting…";
      } else {
        status = "You're on the latest version.";
        checking = false;
      }
    } catch (e) {
      status = `Update check failed: ${e}`;
      checking = false;
    }
  }
</script>

<Modal {onClose} width="360px">
  <div class="about">
    <div class="moon">🌙</div>
    <h1>Moonpool</h1>
    <div class="ver">version {version}</div>
    <p class="desc">
      A system-tray launcher hub for local apps and dev servers, with an embedded terminal per app.
    </p>

    <div class="links">
      <button class="link" onclick={() => openUrl("https://fasterdb.com/software/moonpool/")}>fasterdb.com/software/moonpool</button>
      <span class="dot">·</span>
      <button class="link" onclick={() => openUrl("https://github.com/kjustinkeener/Moonpool")}>github.com/kjustinkeener/Moonpool</button>
    </div>

    <div class="row">
      <button class="btn primary" disabled={checking} onclick={checkUpdates}>
        Check for updates
      </button>
      <button class="btn" onclick={onClose}>Close</button>
    </div>
    {#if status}<div class="status">{status}</div>{/if}

    <div class="credits">
      Built with
      {#each credits as c, i (c.name)}<button class="link" onclick={() => openUrl(c.url)}
          >{c.name}</button
        >{#if i < credits.length - 1}<span class="dot">·</span>{/if}{/each}
    </div>
    <div class="foot">
      MIT License · by <button class="link" onclick={() => openUrl("https://fasterdb.com")}>Justin Keener</button>
    </div>
  </div>
</Modal>

<style>
  .about {
    text-align: center;
    padding: 6px 2px 0;
  }
  .moon {
    font-size: 48px;
  }
  h1 {
    margin: 6px 0 2px;
    font-size: 20px;
    color: var(--text-strong);
  }
  .ver {
    font-size: 12px;
    color: var(--text-dim);
  }
  .desc {
    font-size: 13px;
    color: var(--text-secondary);
    margin: 14px 4px;
    line-height: 1.45;
  }
  .row {
    display: flex;
    gap: 8px;
    justify-content: center;
    margin-top: 8px;
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
  .status {
    font-size: 12px;
    color: var(--text-secondary);
    margin-top: 12px;
  }
  .credits {
    font-size: 11px;
    color: var(--text-dim);
    margin-top: 16px;
    line-height: 1.7;
  }
  .link {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: inherit;
    font-family: inherit;
    padding: 0 2px;
    cursor: pointer;
  }
  .link:hover {
    color: var(--link);
    text-decoration: underline;
  }
  .dot {
    color: var(--border-strong);
    margin: 0 1px;
  }
  .links {
    margin-top: 14px;
    font-size: 12px;
    color: var(--text-dim);
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-items: center;
    gap: 2px;
  }
  .links .link {
    white-space: nowrap;
  }
  .foot {
    font-size: 11px;
    color: var(--text-faint);
    margin-top: 10px;
  }
</style>
