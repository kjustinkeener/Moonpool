// Light/dark/auto theme, persisted in localStorage and applied as a
// `data-theme` attribute on <html> so an explicit choice overrides the OS
// preference. "auto" resolves to the OS setting and follows it live.

// A theme id: "auto" follows the OS; every other id is a named palette defined
// in themes.generated.css (see tools/gen-themes.mjs). An unknown saved id falls
// back to "auto".
export type Theme = string;

// Picker options (id + label + group), generated from src/lib/themes.json.
export { THEMES, GROUP_ORDER } from "./themes.generated";
import { THEMES } from "./themes.generated";
import { t as tr } from "./i18n.svelte";

// Palette names are proper nouns and stay as written; only auto/dark/light have
// a translatable name. Call this from reactive code (a template or $derived) so
// the label follows a live language change.
export function themeLabel(id: string, label: string): string {
  return id === "auto"
    ? tr("common.autoSystem")
    : id === "dark"
      ? tr("theme.dark")
      : id === "light"
        ? tr("theme.light")
        : label;
}

const GROUP_KEYS = {
  Core: "theme.group.core",
  Neon: "theme.group.neon",
  Warm: "theme.group.warm",
  Cool: "theme.group.cool",
  Greens: "theme.group.greens",
  Neutral: "theme.group.neutral",
  Light: "theme.group.light",
  Blush: "theme.group.blush",
  Bright: "theme.group.bright",
  "Light Pastel": "theme.group.lightPastel",
  Pastel: "theme.group.pastel",
} as const;

export function groupLabel(group: string): string {
  return group in GROUP_KEYS ? tr(GROUP_KEYS[group as keyof typeof GROUP_KEYS]) : group;
}
const KNOWN = new Set(THEMES.map((t) => t.id));

const KEY = "moonpool.theme";
const listeners = new Set<() => void>();
let mql: MediaQueryList | null = null;

export function getTheme(): Theme {
  let t: string | null = null;
  try {
    t = localStorage.getItem(KEY);
  } catch {
    /* storage unavailable */
  }
  return t && KNOWN.has(t) ? t : "auto";
}

function resolve(theme: Theme): string {
  if (theme !== "auto") return theme;
  return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

function apply(theme: Theme): void {
  document.documentElement.dataset.theme = resolve(theme);
  for (const fn of listeners) fn();
}

function wireOsListener(theme: Theme): void {
  if (!mql) mql = window.matchMedia("(prefers-color-scheme: dark)");
  // Only track the OS preference while on "auto".
  mql.onchange = theme === "auto" ? () => apply("auto") : null;
}

/** Persist and apply a theme choice immediately. */
export function setTheme(theme: Theme): void {
  try {
    localStorage.setItem(KEY, theme);
  } catch {
    /* storage unavailable */
  }
  wireOsListener(theme);
  apply(theme);
}

/** Apply the saved theme at startup. Call before mounting the app. */
export function initTheme(): void {
  const theme = getTheme();
  wireOsListener(theme);
  apply(theme);
}

/** Subscribe to resolved-theme changes (OS flip or explicit set); returns an unsubscribe fn. */
export function onThemeChange(fn: () => void): () => void {
  listeners.add(fn);
  return () => listeners.delete(fn);
}
