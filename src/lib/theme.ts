// Light/dark/auto theme, persisted in localStorage and applied as a
// `data-theme` attribute on <html> so an explicit choice overrides the OS
// preference. "auto" resolves to the OS setting and follows it live.

// A theme id: "auto" follows the OS; "light"/"dark" are the built-in palettes;
// the rest are fixed named palettes defined in app.css.
export type Theme = string;

// Picker options (id + label). Add a theme here and give it a block in app.css.
export const THEMES: { id: string; label: string }[] = [
  { id: "auto", label: "Auto (system)" },
  { id: "dark", label: "Dark" },
  { id: "light", label: "Light" },
  { id: "purple", label: "Midnight Purple" },
  { id: "ocean", label: "Ocean" },
  { id: "matrix", label: "Matrix" },
  { id: "amber", label: "Amber" },
  { id: "rose", label: "Rose" },
  { id: "nord", label: "Nord" },
  { id: "dracula", label: "Dracula" },
  { id: "gruvbox", label: "Gruvbox" },
  { id: "solarized", label: "Solarized" },
  { id: "crimson", label: "Crimson" },
  { id: "mint", label: "Mint" },
  { id: "paper", label: "Paper" },
  { id: "sky", label: "Sky" },
  { id: "lavender", label: "Lavender" },
  { id: "sage", label: "Sage" },
];
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
