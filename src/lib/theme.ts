// Light/dark/auto theme, persisted in localStorage and applied as a
// `data-theme` attribute on <html> so an explicit choice overrides the OS
// preference. "auto" resolves to the OS setting and follows it live.

export type Theme = "light" | "dark" | "auto";

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
  return t === "light" || t === "dark" || t === "auto" ? t : "auto";
}

function resolve(theme: Theme): "light" | "dark" {
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
