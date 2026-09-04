// UI language: catalog lookup, OS-language resolution, and the reactive `t()`
// every component calls.
//
// Why hand-rolled and not svelte-i18n / typesafe-i18n: the whole feature is the
// ~90 lines below. A library would add a build step and a store abstraction to
// do less than this does, and the catalogs are the actual work either way.
//
// This file is `.svelte.ts` (not `.ts`) because it uses runes. `$state` only
// compiles in `.svelte` / `.svelte.ts` modules; in a plain `.ts` the rune is a
// bare undefined identifier and the build fails with a confusing error.

// Deliberately talks to Tauri directly instead of going through ./api.ts, which
// is otherwise this app's single home for `invoke`. This module plus locales/ is
// meant to be liftable into another app as-is; an import of the app's own API
// layer would make that a rewrite instead of a copy.
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { en } from "./locales/en";
import { de } from "./locales/de";
import { es } from "./locales/es";
import { fr } from "./locales/fr";
import { it } from "./locales/it";
import { ja } from "./locales/ja";
import { ko } from "./locales/ko";
import { nl } from "./locales/nl";
import { pl } from "./locales/pl";
import { ptBR } from "./locales/pt-BR";
import { ru } from "./locales/ru";
import { tr } from "./locales/tr";
import { zhHans } from "./locales/zh-Hans";
import { zhHant } from "./locales/zh-Hant";

/** One entry per plural form (CLDR categories) instead of a single string. */
export type Plural = { one?: string; other: string } & Record<string, string | undefined>;
export type Msg = string | Plural;

/**
 * The English catalog is the schema: its keys are the only valid keys, and every
 * other locale is checked against it. Values widen to `Msg` (the catalog is
 * `as const`, so without this a translation would have to equal the English
 * string literally to type-check).
 */
export type Dict = Record<keyof typeof en, Msg>;

// Translations may lag behind English. A missing key falls back to English
// rather than blocking a release on a complete catalog, so `Partial` is the
// honest type - it is what makes "ship the feature, land the translation later"
// a supported state instead of a broken build.
export type PartialDict = Partial<Dict>;

/**
 * A shipped language.
 *
 * `label` is deliberately the *endonym* (the language's own name for itself):
 * someone who has landed in the wrong language cannot read "Japanese", but can
 * always find "日本語". Never localize this list.
 *
 * Order is the picker's order: English first (the source catalog), then by tag.
 * It is also the tie-break for a bare-language match, which takes the first
 * entry sharing the language - see `REGION_SCRIPT` for where that is not good
 * enough.
 *
 * `dir` drives the `dir` attribute on <html>. Every locale here is `ltr` today;
 * the field exists so adding an RTL language is a catalog change plus a CSS
 * pass, not a re-plumbing. See the RTL section of the App-Patterns doc.
 */
export type LocaleMeta = { id: string; label: string; dir: "ltr" | "rtl" };

export const LOCALES: LocaleMeta[] = [
  { id: "en", label: "English", dir: "ltr" },
  { id: "de", label: "Deutsch", dir: "ltr" },
  { id: "es", label: "Español", dir: "ltr" },
  { id: "fr", label: "Français", dir: "ltr" },
  { id: "it", label: "Italiano", dir: "ltr" },
  { id: "ja", label: "日本語", dir: "ltr" },
  { id: "ko", label: "한국어", dir: "ltr" },
  { id: "nl", label: "Nederlands", dir: "ltr" },
  { id: "pl", label: "Polski", dir: "ltr" },
  { id: "pt-BR", label: "Português (Brasil)", dir: "ltr" },
  { id: "ru", label: "Русский", dir: "ltr" },
  { id: "tr", label: "Türkçe", dir: "ltr" },
  { id: "zh-Hans", label: "简体中文", dir: "ltr" },
  { id: "zh-Hant", label: "繁體中文", dir: "ltr" },
];

const CATALOGS: Record<string, PartialDict> = {
  en,
  de,
  es,
  fr,
  it,
  ja,
  ko,
  nl,
  pl,
  "pt-BR": ptBR,
  ru,
  tr,
  "zh-Hans": zhHans,
  "zh-Hant": zhHant,
};

// Catalogs are imported statically rather than `import()`ed per locale. The
// whole set is a few tens of KB, it ships inside the exe with no network, and
// static keeps `t()` synchronous - no loading state, no flash of untranslated
// text on a language switch. Revisit only if the catalogs get big.

/** "auto" plus every shipped locale; what `Settings.locale` may hold. */
export type LocaleChoice = string;

let choice = $state<LocaleChoice>("auto");
let active = $state<string>("en");
let dict = $state<PartialDict>(en);

/** The user's setting, which may be "auto". */
export const localeChoice = () => choice;
/** The concrete locale in use ("auto" already resolved). */
export const activeLocale = () => active;

/**
 * Regions whose language is shipped under a script tag, not a region tag.
 *
 * Windows reports Taiwan as `zh-TW`, never `zh-Hant`. Without this, pass 2
 * strips it to `zh` and hands a Taiwanese user Simplified Chinese - a fallback
 * that is technically "the right language" and still wrong. Anything not listed
 * falls through to the ordinary bare-language pass.
 */
const REGION_SCRIPT: Record<string, string> = {
  "zh-tw": "zh-Hant",
  "zh-hk": "zh-Hant",
  "zh-mo": "zh-Hant",
  "zh-cn": "zh-Hans",
  "zh-sg": "zh-Hans",
};

/**
 * Best shipped locale for a list of user-preferred tags (most preferred first).
 *
 * Three passes on purpose: an exact regional match wins over a bare-language one,
 * so a `pt-BR` user gets `pt-BR` and not whichever `pt-*` catalog happens to sit
 * first in `LOCALES`. Then known region-to-script aliases, then bare language.
 * Matching is case-insensitive because the OS and the browser disagree about the
 * casing of script and region subtags ("zh-hans" from one, "zh-Hans" from the
 * other).
 */
export function resolveLocale(preferred: readonly string[]): string {
  const ids = LOCALES.map((l) => l.id);
  const lower = new Map(ids.map((id) => [id.toLowerCase(), id]));
  for (const want of preferred) {
    const w = want.toLowerCase();
    const exact = lower.get(w);
    if (exact) return exact;
  }
  for (const want of preferred) {
    const alias = REGION_SCRIPT[want.toLowerCase()];
    if (alias && CATALOGS[alias]) return alias;
  }
  for (const want of preferred) {
    const base = want.toLowerCase().split("-")[0];
    const hit = ids.find((id) => id.toLowerCase().split("-")[0] === base);
    if (hit) return hit;
  }
  return "en";
}

/** Resolve a stored choice ("auto" -> the OS languages) to a shipped locale. */
function resolveChoice(c: LocaleChoice): string {
  if (c && c !== "auto" && CATALOGS[c]) return c;
  if (c && c !== "auto") return resolveLocale([c]);
  // navigator.languages is the webview's view of the OS language list, which on
  // Windows is the user's "preferred languages" order - exactly what we want.
  const nav = navigator.languages?.length ? navigator.languages : [navigator.language];
  return resolveLocale(nav.filter(Boolean));
}

function apply(c: LocaleChoice): string {
  const resolved = resolveChoice(c);
  choice = c;
  active = resolved;
  dict = CATALOGS[resolved] ?? en;
  const meta = LOCALES.find((l) => l.id === resolved);
  document.documentElement.lang = resolved;
  document.documentElement.dir = meta?.dir ?? "ltr";
  return resolved;
}

/** `{name}` placeholders; a missing value is left as-is so it shows up in test. */
function interpolate(s: string, vars?: Record<string, string | number>): string {
  if (!vars) return s;
  return s.replace(/\{(\w+)\}/g, (m, k) => (k in vars ? String(vars[k]) : m));
}

/**
 * Pick a plural form. Uses `Intl.PluralRules`, not `count === 1`, because the
 * categories are per-language: Japanese has one form, English two, French counts
 * 0 as singular, and Polish/Russian have four. Hard-coding `=== 1` is the single
 * most common way an otherwise-translated app still reads as broken.
 */
function plural(forms: Plural, locale: string, count: number): string {
  const cat = new Intl.PluralRules(locale).select(count);
  return forms[cat] ?? forms.other;
}

/**
 * Translate. Reactive: reading it inside a component re-runs that component when
 * the language changes, because it touches the `dict` rune.
 *
 * An unknown key returns the key itself. That is deliberate - a visible
 * `sidebar.addApp` in the UI is a bug report; a silent empty string is not.
 */
export function t(key: keyof Dict, vars?: Record<string, string | number>): string {
  const raw: Msg | undefined = (dict as Record<string, Msg>)[key] ?? (en as Record<string, Msg>)[key];
  if (raw === undefined) return key;
  const s =
    typeof raw === "string"
      ? raw
      : plural(raw, active, Number(vars?.count ?? 0));
  return interpolate(s, vars);
}

/** One piece of a split translation: literal text, or a named slot to fill. */
export type Chunk = { text: string } | { slot: string };

/**
 * Translate a string whose placeholders must render as *markup*, not text.
 *
 * "Or use **+ Add app** above, or <code>Edit file</code> to edit apps.json" is
 * one sentence wearing three elements. Writing it as three sibling strings in
 * the template is the classic way to make a UI untranslatable: word order is
 * frozen to English, and no translator ever sees the whole sentence. So it stays
 * one catalog entry with `{add}` / `{edit}` slots, and the component walks these
 * chunks, wrapping the slots in whatever element it wants.
 */
export function tSplit(key: keyof Dict, vars?: Record<string, string | number>): Chunk[] {
  const s = t(key, vars);
  const out: Chunk[] = [];
  let last = 0;
  for (const m of s.matchAll(/\{(\w+)\}/g)) {
    if (m.index > last) out.push({ text: s.slice(last, m.index) });
    out.push({ slot: m[1] });
    last = m.index + m[0].length;
  }
  if (last < s.length) out.push({ text: s.slice(last) });
  return out;
}

/**
 * Format a list the way the language does it: "A and B" / "A y B" / "AとB".
 * Never join with a hard-coded ", " or " and " - the separator, the final
 * conjunction and even whether spaces appear are all per-language.
 */
export function formatList(items: readonly string[]): string {
  return new Intl.ListFormat(active, { style: "long", type: "conjunction" }).format(items);
}

/**
 * Load and apply the saved language. Call from `main.ts` *before* mounting, so
 * the first paint is already translated - a flash of English is the one bug a
 * user in another language notices every single launch.
 */
export async function initI18n(): Promise<void> {
  let saved: LocaleChoice = "auto";
  try {
    const s = await invoke<{ locale?: string }>("get_settings");
    saved = s.locale || "auto";
  } catch {
    // Settings unreadable (very early failure): fall back to the OS language.
  }
  const resolved = apply(saved);
  // Tell Rust what "auto" resolved to so the next launch can label the tray
  // before the webview exists. Cheap, idempotent, and fire-and-forget.
  invoke("set_locale", { locale: saved, resolved }).catch(() => {});
}

/** Change language: apply here, persist, and retranslate the tray. */
export async function setLocale(c: LocaleChoice): Promise<void> {
  const resolved = apply(c);
  await invoke("set_locale", { locale: c, resolved }).catch(() => {});
}

/**
 * Follow language changes made in another window.
 *
 * Detached windows (Settings, About) run the same bundle in a separate webview
 * with its own module instance, so a `setLocale` in one does not reach the
 * others. Every window that renders text calls this once; the picker broadcasts.
 * Applying only - no re-persist and no tray rebuild, or every open window would
 * race to write the same setting.
 */
export function watchLocale(): Promise<UnlistenFn> {
  return listen<LocaleChoice>("settings:locale", (e) => apply(e.payload));
}
