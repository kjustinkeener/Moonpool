<script lang="ts">
  // Asks once for a GitHub star. Nothing polls: one timer is armed for the
  // moment the card could next be due, and it re-checks when it fires, when
  // the window is shown again, and when it gains focus.
  import { onMount } from "svelte";
  import Icon from "./Icon.svelte";
  import { openUrl } from "./api";
  import { t } from "./i18n.svelte";

  const REPO = "https://github.com/kjustinkeener/Moonpool";
  const KEY = "moonpool.starPrompt";
  const NEEDED_MS = 2 * 60_000;
  const QUIET_MS = 30_000;
  const SNOOZE_MS = 24 * 3_600_000;

  type Saved = { usedMs: number; snoozeUntil: number; done: boolean };

  function load(): Saved {
    try {
      const s = JSON.parse(localStorage.getItem(KEY) ?? "{}");
      return { usedMs: +s.usedMs || 0, snoozeUntil: +s.snoozeUntil || 0, done: !!s.done };
    } catch {
      return { usedMs: 0, snoozeUntil: 0, done: false };
    }
  }

  let saved = load();
  let shown = $state(false);
  let visibleSince: number | null = null;
  let lastInput = 0;
  let timer: ReturnType<typeof setTimeout> | undefined;

  function save() {
    try {
      localStorage.setItem(KEY, JSON.stringify(saved));
    } catch {}
  }

  /** Window-visible time, so a tray app that stays open counts only while shown. */
  function usedNow() {
    return saved.usedMs + (visibleSince === null ? 0 : Date.now() - visibleSince);
  }

  function check() {
    clearTimeout(timer);
    if (saved.done || shown || visibleSince === null) return;
    const now = Date.now();
    const wait = Math.max(saved.snoozeUntil - now, NEEDED_MS - usedNow(), lastInput + QUIET_MS - now);
    if (wait <= 0) {
      shown = true;
      return;
    }
    timer = setTimeout(check, wait);
  }

  function show() {
    if (visibleSince !== null) return;
    visibleSince = Date.now();
    check();
  }

  function hide() {
    clearTimeout(timer);
    if (visibleSince === null) return;
    saved.usedMs += Date.now() - visibleSince;
    visibleSince = null;
    save();
  }

  function sync() {
    if (document.hidden) hide();
    else show();
  }

  function onPointerDown(e: PointerEvent) {
    if (e.target instanceof Element && e.target.closest(".star-card")) return;
    lastInput = Date.now();
  }

  onMount(() => {
    sync();
    document.addEventListener("visibilitychange", sync);
    window.addEventListener("focus", check);
    window.addEventListener("pagehide", hide);
    window.addEventListener("pointerdown", onPointerDown, { capture: true, passive: true });
    return () => {
      hide();
      document.removeEventListener("visibilitychange", sync);
      window.removeEventListener("focus", check);
      window.removeEventListener("pagehide", hide);
      window.removeEventListener("pointerdown", onPointerDown, { capture: true });
    };
  });

  function star() {
    openUrl(REPO);
    never();
  }
  function never() {
    saved.done = true;
    shown = false;
    save();
  }
  function later() {
    saved.snoozeUntil = Date.now() + SNOOZE_MS;
    shown = false;
    save();
    check();
  }
</script>

{#if shown}
  <div class="star-card" role="region" aria-label={t("star.label")}>
    <button class="x" title={t("star.later")} aria-label={t("star.later")} onclick={later}>
      <Icon name="close" size={13} />
    </button>
    <div class="head">
      <!-- A star making the pleading face of the 🥺 emoji: big shiny eyes,
           raised inner brows and a small frown. -->
      <svg class="face" viewBox="0 0 32 32" aria-hidden="true">
        <defs>
          <!-- Lit from the top centre: white fading through yellow to a
               yellow-orange that meets the darker orange rim. -->
          <radialGradient id="star-face-fill" cx="0.5" cy="0.3" r="0.75">
            <stop offset="0" stop-color="#fffdf0" />
            <stop offset="0.3" stop-color="#ffe45c" />
            <stop offset="0.7" stop-color="#ffc42e" />
            <stop offset="1" stop-color="#f59a1c" />
          </radialGradient>
          <!-- Eye border for the inset look: dark where the socket shades the
               top, fading to a lit white lip at the bottom. -->
          <linearGradient id="star-eye-rim" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0" stop-color="#4a3410" />
            <stop offset="0.5" stop-color="#c9a15a" />
            <stop offset="1" stop-color="#ffffff" />
          </linearGradient>
        </defs>
        <path
          class="rim"
          d="M16 4.5 19.64 12.48 28.36 13.48 21.9 19.42 23.64 28.02 16 23.7 8.36 28.02 10.1 19.42 3.64 13.48 12.36 12.48Z"
        />
        <path
          class="body"
          d="M16 4.5 19.64 12.48 28.36 13.48 21.9 19.42 23.64 28.02 16 23.7 8.36 28.02 10.1 19.42 3.64 13.48 12.36 12.48Z"
        />
        <!-- The whole face sits 0.55 grid units lower than where it was first drawn. -->
        <g transform="translate(0 0.55)">
        <path class="brow" d="M9.1 13.8Q10.7 13.6 12.1 12.1M22.9 13.8Q21.3 13.6 19.9 12.1" />
        <circle class="socket" cx="12.3" cy="17" r="2.7" />
        <circle class="eye" cx="12.3" cy="17" r="2.4" />
        <circle class="socket" cx="19.7" cy="17" r="2.7" />
        <circle class="eye" cx="19.7" cy="17" r="2.4" />
        <ellipse class="shine" cx="11.4" cy="15.8" rx="1.5" ry="0.95" transform="rotate(-45 11.4 15.8)" />
        <ellipse class="shine" cx="18.8" cy="15.8" rx="1.5" ry="0.95" transform="rotate(-45 18.8 15.8)" />
        <circle class="shine" cx="13.3" cy="18.3" r="0.6" />
        <circle class="shine" cx="20.7" cy="18.3" r="0.6" />
        <path class="mouth" d="M14.7 22.55Q16 21.45 17.3 22.55" />
        </g>
      </svg>
      <p>{t("star.message")}</p>
    </div>
    <div class="row">
      <button class="primary" onclick={star}>{t("star.action")}</button>
      <button onclick={later}>{t("star.later")}</button>
      <button onclick={never}>{t("star.never")}</button>
    </div>
  </div>
{/if}

<style>
  .star-card {
    position: relative;
    margin: 10px 10px 0;
    padding: 10px 28px 10px 10px;
    background: var(--bg-elevated);
    border: 1px solid var(--border);
    border-radius: 6px;
    font-size: 12px;
    color: var(--text-secondary);
  }
  .head {
    display: flex;
    align-items: flex-start;
    gap: 9px;
    margin-bottom: 8px;
  }
  p {
    margin: 0;
    line-height: 1.4;
  }
  .face {
    flex: none;
    width: 66px;
    height: 66px;
  }
  .face .rim {
    fill: #d8730c;
    stroke: #d8730c;
    stroke-width: 3;
    stroke-linejoin: round;
  }
  .face .body {
    fill: url(#star-face-fill);
    stroke: url(#star-face-fill);
    stroke-width: 2;
    stroke-linejoin: round;
  }
  .face .eye {
    fill: #2b2110;
  }
  .face .socket {
    fill: none;
    stroke: url(#star-eye-rim);
    stroke-width: 0.6;
  }
  .face .shine {
    fill: #fff;
  }
  .face .brow,
  .face .mouth {
    fill: none;
    stroke: #2b2110;
    stroke-width: 1.1;
    stroke-linecap: round;
  }
  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }
  .row button {
    background: var(--bg);
    border: 1px solid var(--border);
    border-radius: 5px;
    color: var(--text);
    padding: 4px 9px;
    font-size: 12px;
    cursor: pointer;
  }
  .row button:hover {
    border-color: var(--border-strong);
  }
  .row .primary {
    background: var(--accent);
    border-color: var(--accent);
    color: var(--on-accent);
  }
  .x {
    position: absolute;
    top: 6px;
    right: 6px;
    display: inline-flex;
    padding: 3px;
    background: none;
    border: none;
    border-radius: 4px;
    color: var(--text-muted);
    cursor: pointer;
  }
  .x:hover {
    color: var(--text-strong);
  }
</style>
