<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { getVersion } from "@tauri-apps/api/app";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { openUrl, updateCheck, updateApply, setupState } from "./lib/api";
  import { t, tSplit, watchLocale } from "./lib/i18n.svelte";

  const credits = [
    { name: "Tauri", url: "https://tauri.app" },
    { name: "Svelte", url: "https://svelte.dev" },
    { name: "Vite", url: "https://vite.dev" },
    { name: "xterm.js", url: "https://xtermjs.org" },
    { name: "portable-pty", url: "https://crates.io/crates/portable-pty" },
  ];

  let version = $state("");
  // Build date (UTC, YYYY-MM-DD) stamped into the exe at compile time. A version
  // alone doesn't say which build a bug report came from while releases are frequent.
  let built = $state("");
  let status = $state("");
  let checking = $state(false);
  let unlistenLocale: (() => void) | null = null;

  function close() {
    getCurrentWindow()
      .close()
      .catch(() => {});
  }

  // The window is transparent; app.css paints body with a solid theme color (the
  // dark box behind the card). Clear it so only the card shows, floating free.
  onMount(async () => {
    const app = document.getElementById("app");
    const els = [document.documentElement, document.body, app].filter(
      Boolean,
    ) as HTMLElement[];
    for (const e of els) e.style.background = "transparent";
    try {
      version = await getVersion();
    } catch {
      version = "?";
    }
    try {
      built = (await setupState()).buildDate;
    } catch {
      // Leave the build date off if setup state isn't reachable.
    }
    // Follow a language change made in the Settings window while About is open.
    // Assigned rather than returned: an async onMount callback cannot return a
    // cleanup function in Svelte 5 (the return value is a promise, not the fn).
    unlistenLocale = await watchLocale();
  });
  onDestroy(() => unlistenLocale?.());

  async function checkUpdates() {
    checking = true;
    status = t("about.checking");
    try {
      const r = await updateCheck();
      if (r.available) {
        status = t("about.updateDownloading", { version: r.available.version });
        // On success the backend relaunches and exits; this won't return.
        await updateApply(r.available);
        status = t("about.updateInstalled");
      } else {
        status = t("about.upToDate");
        checking = false;
      }
    } catch (e) {
      status = t("about.checkFailed", { error: String(e) });
      checking = false;
    }
  }
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && close()} />

<!-- Whole window drags via data-tauri-drag-region; Tauri auto-excludes interactive
     controls (button, input, a). Decorative text gets pointer-events:none so a
     mousedown there falls through to the card and drags too. -->
<div class="page" data-tauri-drag-region>
  <div class="about" data-tauri-drag-region>
    <div class="head">
      <div class="moon">🌙</div>
      <h1>Moonpool</h1>
      <div class="ver">{t("about.version", { version })}{#if built}&nbsp;· {built}{/if}</div>
      <p class="desc">{t("about.tagline")}</p>
    </div>

    <div class="links">
      <button class="link" onclick={() => openUrl("https://fasterdb.com/software/moonpool/")}>fasterdb.com/software/moonpool</button>
      <span class="dot">·</span>
      <button class="link" onclick={() => openUrl("https://github.com/kjustinkeener/Moonpool")}>github.com/kjustinkeener/Moonpool</button>
      <span class="dot">·</span>
      <button class="link" onclick={() => openUrl("mailto:gofast@fasterdb.com")}>gofast@fasterdb.com</button>
    </div>

    <div class="row">
      <button class="btn primary" disabled={checking} onclick={checkUpdates}>
        {t("about.checkUpdates")}
      </button>
      <button class="btn" onclick={close}>{t("common.close")}</button>
    </div>
    {#if status}<div class="status">{status}</div>{/if}

    <div class="credits">
      <span class="txt">{t("about.builtWith")}</span>
      {#each credits as c, i (c.name)}<button class="link" onclick={() => openUrl(c.url)}
          >{c.name}</button
        >{#if i < credits.length - 1}<span class="dot">·</span>{/if}{/each}
    </div>
    <div class="foot">
      {#each tSplit("about.byLine") as c}{#if "text" in c}<span class="txt">{c.text}</span
        >{:else}<button class="link" onclick={() => openUrl("https://fasterdb.com")}
          >Justin Keener</button
        >{/if}{/each}
    </div>
  </div>
</div>

<style>
  .page {
    height: 100vh;
    width: 100vw;
    background: transparent;
    display: flex;
  }
  .about {
    flex: 1;
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: 26px 26px 20px;
    color: var(--text);
    background:
      radial-gradient(120% 70% at 50% -6%, color-mix(in srgb, var(--accent) 22%, transparent), transparent 60%),
      color-mix(in srgb, var(--bg-panel) calc(var(--app-alpha) * 100%), transparent);
    border: 1px solid var(--border);
    overflow: hidden;
  }
  /* Decorative header: non-interactive, so this whole block drags the window. */
  .head {
    pointer-events: none;
    width: 100%;
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
  /* Plain text sits inside control rows; drop pointer events so it drags, while
     the buttons beside it stay clickable (default pointer-events). */
  .txt {
    pointer-events: none;
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
    pointer-events: none;
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
