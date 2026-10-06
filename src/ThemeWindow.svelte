<script lang="ts">
  // The theme browser: a detached window (#themes) with one preview card per
  // palette, each drawn in THAT palette's own colors so the card is a faithful
  // thumbnail. Clicking applies live here, persists (theme.ts), and broadcasts
  // "settings:theme" so every other window retints. The window stays open so
  // themes can be compared. Based on the shared Themes pattern's ThemeBrowser.
  import { onMount, onDestroy } from "svelte";
  import { emit, listen } from "@tauri-apps/api/event";
  import type { UnlistenFn } from "@tauri-apps/api/event";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { getSettings } from "./lib/api";
  import {
    THEMES,
    GROUP_ORDER,
    getTheme,
    setTheme,
    themeLabel,
    groupLabel,
    type Theme,
  } from "./lib/theme";
  import { ansiFor } from "./lib/ansi";
  import { t, watchLocale } from "./lib/i18n.svelte";
  import { scrollFade } from "./lib/scrollfade";
  import Icon from "./lib/Icon.svelte";
  import brandIcon from "./assets/app-icon.png";

  let current = $state<Theme>(getTheme());

  // Same formula as the other detached windows: each reads --app-alpha off its
  // own document, so it applies the persisted setting itself and follows the
  // live broadcast from Settings.
  function applyTransparency(pct: number) {
    const alpha = Math.max(0.1, 1 - Math.min(90, Math.max(0, pct)) / 100);
    document.documentElement.style.setProperty("--app-alpha", String(alpha));
  }

  // The role vars a card is drawn from: Moonpool's semantic ramp (what the real
  // UI uses) plus the gauge stops. Captured per theme so a card can be
  // inline-styled in its own palette without being the active theme.
  const VARS = [
    "--bg", "--bg-panel", "--bg-inset", "--border", "--text", "--text-dim",
    "--accent", "--on-accent", "--link", "--success", "--warning", "--danger",
    "--g0", "--g1", "--g2",
  ] as const;

  type Palette = { vars: Record<string, string>; ansi: readonly string[] };

  // Palettes are `:root[data-theme=id]` blocks, so the only way to read a
  // non-active theme's colors is to stamp it on the root, read the computed
  // vars, then restore. All synchronous in one task, so nothing paints between.
  // "auto" has no block; preview it as what the OS currently resolves it to.
  function capture(): Record<string, Palette> {
    const root = document.documentElement;
    const prev = root.getAttribute("data-theme");
    const cs = getComputedStyle(root);
    const osDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    const out: Record<string, Palette> = {};
    for (const th of THEMES) {
      const id = th.id === "auto" ? (osDark ? "dark" : "light") : th.id;
      root.setAttribute("data-theme", id);
      const vars: Record<string, string> = {};
      for (const v of VARS) vars[v] = cs.getPropertyValue(v).trim();
      out[th.id] = { vars, ansi: ansiFor(id) };
    }
    if (prev) root.setAttribute("data-theme", prev);
    else root.removeAttribute("data-theme");
    return out;
  }

  const pal = capture();

  const gauge = (p: Palette) =>
    `linear-gradient(90deg, ${p.vars["--g0"]} 0%, ${p.vars["--g1"]} 52%, ${p.vars["--g2"]} 100%)`;

  // Apply here (setTheme repaints this window and persists the raw choice), then
  // broadcast so the hub, Settings and the rest retint.
  function pick(id: string) {
    current = id;
    setTheme(id);
    emit("settings:theme", id).catch(() => {});
  }

  function close() {
    getCurrentWindow()
      .close()
      .catch(() => {});
  }

  let unlistenTransparency: UnlistenFn | null = null;
  let unlistenTheme: UnlistenFn | null = null;
  let unlistenLocale: UnlistenFn | null = null;

  onMount(async () => {
    // The page paints its own tint, so the transparent window shows the desktop through.
    const app = document.getElementById("app");
    for (const e of [document.documentElement, document.body, app]) {
      if (e) e.style.background = "transparent";
    }
    getSettings()
      .then((s) => applyTransparency(s.transparency ?? 0))
      .catch(() => {});
    unlistenTransparency = await listen<number>("settings:transparency", (e) =>
      applyTransparency(e.payload),
    );
    // Keep the highlight (and this window's own colors) in step when the theme
    // is changed from the Settings window or anywhere else.
    unlistenTheme = await listen<Theme>("settings:theme", (e) => {
      current = e.payload;
      setTheme(e.payload);
    });
    unlistenLocale = await watchLocale();
  });
  onDestroy(() => {
    unlistenTransparency?.();
    unlistenTheme?.();
    unlistenLocale?.();
  });
</script>

<svelte:window onkeydown={(e) => e.key === "Escape" && close()} />

<div class="win">
  <div class="bar" data-tauri-drag-region>
    <img class="brandicon" src={brandIcon} alt="" aria-hidden="true" draggable="false" />
    <span class="title" data-tauri-drag-region>{t("themes.title")}</span>
    <span class="hint" data-tauri-drag-region>{t("themes.hint")}</span>
    <span class="spacer" data-tauri-drag-region></span>
    <button class="x" onclick={close} title={t("common.close")} aria-label={t("common.close")}>
      <Icon name="close" size={11} width={2.4} />
    </button>
  </div>

  <div class="scroll" use:scrollFade>
    {#each GROUP_ORDER as g (g)}
      {@const items = THEMES.filter((th) => th.group === g)}
      {#if items.length}
        <div class="grouplabel">{groupLabel(g)}</div>
        <div class="grid">
          {#each items as th (th.id)}
            {@const p = pal[th.id]}
            {@const name = themeLabel(th.id, th.label)}
            <!-- The card ground is the palette's own solid --bg: previews must stay
                 opaque however transparent the window is set. -->
            <button
              class="card"
              class:active={th.id === current}
              style:background={p.vars["--bg"]}
              style:border-color={p.vars["--border"]}
              onclick={() => pick(th.id)}
              title={name}
              aria-pressed={th.id === current}
            >
              <div class="row">
                <div class="name" style:color={p.vars["--text"]}>{name}</div>
                {#if th.id === current}
                  <div class="check" style:color={p.vars["--accent"]}>
                    <Icon name="check" size={12} width={2.6} />
                  </div>
                {/if}
              </div>

              <div class="panel" style:background={p.vars["--bg-panel"]} style:border-color={p.vars["--border"]}>
                <span class="t1" style:color={p.vars["--text"]}>Text</span>
                <span class="t2" style:color={p.vars["--text-dim"]}>dim</span>
                <span class="t2" style:color={p.vars["--link"]}>link</span>
                <span
                  class="field"
                  style:background={p.vars["--bg-inset"]}
                  style:border-color={p.vars["--border"]}
                  style:color={p.vars["--text-dim"]}>input</span
                >
                <span class="btn" style:background={p.vars["--accent"]} style:color={p.vars["--on-accent"]}>Run</span>
              </div>

              <div class="pips">
                <span class="pip" style:background={p.vars["--success"]}></span>
                <span class="pip" style:background={p.vars["--warning"]}></span>
                <span class="pip" style:background={p.vars["--danger"]}></span>
              </div>

              <div class="gaugebar" style:background={gauge(p)}></div>

              <div class="ansi">
                {#each p.ansi as c, i (i)}
                  <span class="sw" style:background={c}></span>
                {/each}
              </div>
            </button>
          {/each}
        </div>
      {/if}
    {/each}
  </div>
</div>

<style>
  :global(html),
  :global(body) {
    background: transparent;
  }
  /* Single tint layer: the bar and the scroll area each carry the tint once (the
     shell itself paints nothing), so alphas never stack. */
  .win {
    display: flex;
    flex-direction: column;
    box-sizing: border-box;
    height: 100vh;
    overflow: hidden;
    color: var(--text);
    border: 1px solid var(--border);
    transition: --app-alpha 2s ease;
  }
  .win:hover {
    --app-alpha: 1;
    transition: --app-alpha 0s;
  }
  .bar {
    display: flex;
    align-items: center;
    gap: 8px;
    height: 32px;
    flex: none;
    padding-left: 10px;
    background: color-mix(in srgb, var(--bg) calc(var(--app-alpha) * 100%), transparent);
    border-bottom: 1px solid var(--border-muted);
    user-select: none;
    -webkit-user-select: none;
  }
  .brandicon {
    height: 16px;
    width: 16px;
    pointer-events: none;
  }
  .title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-strong);
  }
  .hint {
    font-size: 11px;
    color: var(--text-dim);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    min-width: 0;
  }
  .spacer {
    flex: 1;
  }
  .x {
    width: 30px;
    height: 32px;
    flex: none;
    display: grid;
    place-items: center;
    border: none;
    background: transparent;
    color: var(--text);
    cursor: pointer;
    transition: background 0.12s;
  }
  .x:hover {
    background: #e81123;
    color: #fff;
  }
  .scroll {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    padding: 4px 14px 16px;
    background: color-mix(in srgb, var(--bg-panel) calc(var(--app-alpha) * 100%), transparent);
  }
  .grouplabel {
    font-size: 10px;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--text-dim);
    font-weight: 600;
    margin: 14px 2px 6px;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(190px, 1fr));
    gap: 9px;
  }
  .card {
    position: relative;
    text-align: left;
    border: 1px solid;
    border-radius: 9px;
    padding: 9px 10px 10px;
    cursor: pointer;
    display: flex;
    flex-direction: column;
    gap: 7px;
    font: inherit;
    transition:
      transform 0.08s ease,
      box-shadow 0.08s ease;
  }
  .card:hover {
    transform: translateY(-1px);
    box-shadow: 0 5px 16px var(--shadow);
  }
  .card:focus-visible,
  .card.active {
    outline: 2px solid var(--accent);
    outline-offset: 1px;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 6px;
  }
  .name {
    font-weight: 700;
    font-size: 12px;
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .check {
    display: flex;
  }
  .panel {
    display: flex;
    align-items: center;
    gap: 6px;
    border: 1px solid;
    border-radius: 6px;
    padding: 5px 7px;
  }
  .t1 {
    font-weight: 600;
    font-size: 11px;
  }
  .t2 {
    font-size: 10px;
  }
  .field {
    margin-left: auto;
    font-size: 10px;
    padding: 2px 7px;
    border: 1px solid;
    border-radius: 4px;
  }
  .btn {
    font-size: 10px;
    font-weight: 700;
    padding: 2px 8px;
    border-radius: 4px;
  }
  .pips {
    display: flex;
    align-items: center;
    gap: 5px;
  }
  .pip {
    width: 11px;
    height: 11px;
    border-radius: 50%;
  }
  .gaugebar {
    height: 8px;
    border-radius: 4px;
  }
  .ansi {
    display: grid;
    grid-template-columns: repeat(16, 1fr);
    gap: 2px;
  }
  .sw {
    height: 10px;
    border-radius: 2px;
  }
</style>
