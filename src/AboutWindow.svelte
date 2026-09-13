<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { getVersion } from "@tauri-apps/api/app";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { openUrl, updateCheck, updateApply, setupState, getSettings } from "./lib/api";
  import { listen } from "@tauri-apps/api/event";
  import type { UnlistenFn } from "@tauri-apps/api/event";
  import { t, tSplit, watchLocale } from "./lib/i18n.svelte";
  import appIcon from "./assets/app-icon.png";

  // Same formula as App.svelte / SettingsControls.svelte: each detached window
  // reads --app-alpha off its own document, so it must apply the persisted
  // setting itself rather than inheriting it from the hub.
  function applyTransparency(pct: number) {
    const alpha = Math.max(0.1, 1 - Math.min(90, Math.max(0, pct)) / 100);
    document.documentElement.style.setProperty("--app-alpha", String(alpha));
  }
  let unlistenTransparency: UnlistenFn | null = null;

  const credits = [
    { name: "Rust", url: "https://www.rust-lang.org" },
    { name: "Tauri", url: "https://tauri.app" },
    { name: "Svelte", url: "https://svelte.dev" },
    { name: "Vite", url: "https://vite.dev" },
    { name: "xterm.js", url: "https://xtermjs.org" },
    { name: "portable-pty", url: "https://crates.io/crates/portable-pty" },
  ];

  let rippling = $state(false);
  let lastRipple = 0;

  function onIconHover() {
    const now = Date.now();
    if (now - lastRipple < 5000) return;
    lastRipple = now;
    rippling = true;
  }

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
    getSettings()
      .then((s) => applyTransparency(s.transparency ?? 0))
      .catch(() => {});
    // The detached Settings window broadcasts changes so this window updates live.
    unlistenTransparency = await listen<number>("settings:transparency", (e) =>
      applyTransparency(e.payload),
    );
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
  onDestroy(() => {
    unlistenLocale?.();
    unlistenTransparency?.();
  });

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
    <button class="xclose" onclick={close} aria-label={t("common.close")}>✕</button>
    <div class="head">
      <div class="moon-wrap" role="presentation" onmouseenter={onIconHover}>
        <img class="moon" src={appIcon} alt="" width="56" height="56" />
        <span class="ripple" class:active={rippling} onanimationend={() => (rippling = false)}></span>
      </div>
      <h1>Moonpool</h1>
      <div class="ver">{t("about.version", { version })}{#if built}&nbsp;· {built}{/if}</div>
      <p class="desc">{t("about.tagline")}</p>
    </div>

    <div class="rule"></div>

    <div class="links">
      <button class="link" onclick={() => openUrl("https://fasterdb.com/software/moonpool/")}>fasterdb.com/software/moonpool</button>
      <button class="link" onclick={() => openUrl("https://github.com/kjustinkeener/Moonpool")}>github.com/kjustinkeener/Moonpool</button>
      <button class="link email" onclick={() => openUrl("mailto:gofast@fasterdb.com")}>gofast@fasterdb.com</button>
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
      {#each credits as c, i (c.name)}<button class="link" title={c.url} onclick={() => openUrl(c.url)}
          >{c.name}</button
        >{#if i < credits.length - 1}<span class="dot">·</span>{/if}{/each}
    </div>
    <div class="foot">
      {#each tSplit("about.byLine") as c}{#if "text" in c}<span class="txt">{c.text}</span
        >{:else}<button class="link" title="https://fasterdb.com" onclick={() => openUrl("https://fasterdb.com")}
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
    position: relative;
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
    /* Wakes this window to full opacity on hover, then eases back to the
       configured transparency once the pointer leaves. Matches Settings/hub/editor. */
    transition: --app-alpha 2s ease;
  }
  .about:hover {
    --app-alpha: 1;
    transition: --app-alpha 0s;
  }
  /* Frameless window has no native title bar, so supply a close affordance in the
     corner (the Close button below can scroll out of view on a short window). */
  .xclose {
    position: absolute;
    top: 8px;
    right: 8px;
    width: 22px;
    height: 22px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: none;
    border: none;
    border-radius: 5px;
    color: var(--text-dim);
    font-size: 13px;
    line-height: 1;
    cursor: pointer;
  }
  .xclose:hover {
    background: var(--bg-elevated);
    color: var(--text);
  }
  /* Decorative header: non-interactive, so this whole block drags the window. */
  .head {
    pointer-events: none;
    width: 100%;
  }
  .moon-wrap {
    position: relative;
    width: 56px;
    height: 56px;
    margin: 0 auto;
    /* The rest of .head is pointer-events:none so it drags the window; this one
       spot opts back in to catch hover for the glow bump and ripple below. */
    pointer-events: auto;
  }
  .moon {
    display: block;
    width: 56px;
    height: 56px;
    /* Glow in the icon's own center color (#61FCED), matching the intro/titlebar. */
    filter: drop-shadow(0 0 10px rgba(97, 252, 237, 0.49))
      drop-shadow(0 0 22px rgba(97, 252, 237, 0.28));
    transition: filter 0.2s ease;
  }
  .moon-wrap:hover .moon {
    filter: brightness(1.25) drop-shadow(0 0 14px rgba(97, 252, 237, 0.7))
      drop-shadow(0 0 30px rgba(97, 252, 237, 0.42));
  }
  /* One-shot ring on hover, debounced to once every 5s in onIconHover(). */
  .ripple {
    position: absolute;
    inset: 0;
    border-radius: 50%;
    border: 2px solid var(--dot-run);
    opacity: 0;
    pointer-events: none;
  }
  .ripple.active {
    animation: moon-ripple 0.8s ease-out;
  }
  @keyframes moon-ripple {
    from {
      transform: scale(0.8);
      opacity: 0.6;
    }
    to {
      transform: scale(1.9);
      opacity: 0;
    }
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
    margin: 14px 4px 0;
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
    background: #007eb9;
    border-color: #007eb9;
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
  /* Hairline divider setting the links block off from the description above. */
  .rule {
    align-self: center;
    width: 78%;
    height: 1px;
    margin: 16px 0;
    background: color-mix(in srgb, var(--text) 28%, transparent);
    flex: none;
  }
  .links {
    margin-top: 0;
    margin-bottom: 16px;
    font-size: 12px;
    color: var(--text-dim);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 3px;
  }
  .links .link {
    white-space: nowrap;
  }
  /* A touch of extra space sets the contact address off from the two site links. */
  .links .email {
    margin-top: 2px;
  }
  .foot {
    font-size: 11px;
    color: var(--text-faint);
    margin-top: 10px;
  }
</style>
