// Locale coverage + placeholder gate. Run via `npm run check:locales` (and as
// part of `npm run check`).
//
// Three things go wrong with hand-maintained catalogs, and none of them are
// caught by the type checker or by looking at the app in one language:
//
//  1. An English key is renamed. The translations keep the old key, silently
//     fall back to English forever, and nobody notices. -> ORPHAN, hard fail.
//  2. A translation drops or mistypes a placeholder ("{versoin}", or the
//     translator just omits "{error}"). The string renders with a literal
//     "{versoin}" in it, or silently loses the value. -> MISMATCH, hard fail.
//  3. A translation lags behind English. That is legal here (missing keys fall
//     back), but it should be visible, not invisible. -> reported, soft.
//
// Placeholders that are literal user-facing text rather than slots are listed in
// LITERAL_TOKENS: `{MP_HOME}` is typed by the user into a path, so it appears in
// the English string AND must survive translation, but it is never substituted.
// It is checked like any other placeholder (it must be carried through), which
// is exactly the behaviour we want.

import { readdirSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const here = dirname(fileURLToPath(import.meta.url));
const localesDir = join(here, "..", "src", "lib", "locales");

/** Placeholders that are literal text, kept here only for the report's sake. */
const LITERAL_TOKENS = new Set(["MP_HOME"]);

const placeholders = (s) =>
  new Set([...String(s).matchAll(/\{(\w+)\}/g)].map((m) => m[1]));

/** A catalog value is a string or a plural-forms object; flatten to strings. */
const strings = (v) => (typeof v === "string" ? [v] : Object.values(v).filter(Boolean));

async function load(file) {
  const mod = await import(`file://${join(localesDir, file)}`);
  const exported = Object.values(mod)[0];
  if (!exported || typeof exported !== "object") {
    throw new Error(`${file}: expected a single exported catalog object`);
  }
  return exported;
}

const { en } = await import(`file://${join(localesDir, "en.ts")}`);
const enKeys = new Set(Object.keys(en));

const files = readdirSync(localesDir)
  .filter((f) => f.endsWith(".ts") && f !== "en.ts")
  .sort();

let failed = false;
const rows = [];

for (const file of files) {
  const dict = await load(file);
  const keys = Object.keys(dict);
  const orphans = keys.filter((k) => !enKeys.has(k));
  const missing = [...enKeys].filter((k) => !(k in dict));
  const mismatched = [];

  for (const k of keys) {
    if (!enKeys.has(k)) continue;
    const want = new Set(strings(en[k]).flatMap((s) => [...placeholders(s)]));
    const got = new Set(strings(dict[k]).flatMap((s) => [...placeholders(s)]));
    const lost = [...want].filter((p) => !got.has(p));
    const extra = [...got].filter((p) => !want.has(p));
    if (lost.length || extra.length) mismatched.push({ k, lost, extra });
  }

  const pct = Math.round(((enKeys.size - missing.length) / enKeys.size) * 100);
  rows.push({ file, pct, missing: missing.length });

  if (orphans.length) {
    failed = true;
    console.error(`\n${file}: ${orphans.length} key(s) not in en.ts (renamed or typo'd):`);
    for (const k of orphans) console.error(`  - ${k}`);
  }
  if (mismatched.length) {
    failed = true;
    console.error(`\n${file}: ${mismatched.length} placeholder mismatch(es):`);
    for (const m of mismatched) {
      const bits = [];
      if (m.lost.length) bits.push(`dropped {${m.lost.join("} {")}}`);
      if (m.extra.length) bits.push(`unexpected {${m.extra.join("} {")}}`);
      console.error(`  - ${m.k}: ${bits.join(", ")}`);
    }
  }
  if (missing.length) {
    console.warn(`\n${file}: ${missing.length} untranslated key(s) (falls back to English):`);
    for (const k of missing) console.warn(`  - ${k}`);
  }
}

console.log(`\nen.ts: ${enKeys.size} keys.`);
for (const r of rows) {
  console.log(`  ${r.file.padEnd(14)} ${String(r.pct).padStart(3)}%  (${r.missing} missing)`);
}
const literals = [...enKeys].filter((k) =>
  strings(en[k]).some((s) => [...placeholders(s)].some((p) => LITERAL_TOKENS.has(p))),
);
if (literals.length) {
  console.log(`\nLiteral (never-substituted) tokens appear in: ${literals.join(", ")}`);
}

if (failed) {
  console.error("\nLocale check FAILED.");
  process.exit(1);
}
console.log("\nLocale check passed.");
