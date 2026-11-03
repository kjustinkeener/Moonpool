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
    <p>{t("star.message")}</p>
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
  p {
    margin: 0 0 8px;
    line-height: 1.4;
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
