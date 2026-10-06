#!/usr/bin/env node
// Theme palette generator for Moonpool.
//
// Source of truth: src/lib/themes.json (the shared Themes pattern's palettes,
// compact vars) + src/lib/ansi.ts (the 16-slot color sets). Outputs, both
// committed so a theme paints before first paint with no async read:
//
//   src/themes.generated.css     one :root[data-theme="id"] block per palette
//   src/lib/themes.generated.ts  THEMES / GROUP_ORDER for the picker + browser
//
// Each block carries the pattern's compact vars (--bg-rgb, --fg, --muted,
// --track, --edge, --panel, --live, --green/--yellow/--red, --g0..2) AND the
// "semantic bridge": Moonpool's long semantic ramp (--bg, --bg-panel, --text-dim,
// --accent, --link, --danger, ...) derived from them (accent/focus <- ANSI blue,
// link <- ANSI bright blue, danger/success/warning <- red/green/yellow, surfaces
// and the text ramp by color-mix between --bg and --fg). An entry with a
// `semantic` object (the 17 palettes Moonpool shipped before the port) keeps those
// hand-tuned values instead of the derived ones, so saved themes look unchanged.
// Delete an entry's `semantic` to switch it to the derived bridge.
//
// --app-alpha is never emitted: the transparency system owns it, and the single
// tint layer rule means a theme block must not touch it.
//
//   node tools/gen-themes.mjs generate   write both outputs
//   node tools/gen-themes.mjs audit      print contrast of the derived bridge

import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, resolve } from "node:path";
import { ANSI } from "../src/lib/ansi.ts";

const here = dirname(fileURLToPath(import.meta.url));
const repo = resolve(here, "..");
const P = {
  json: resolve(repo, "src/lib/themes.json"),
  css: resolve(repo, "src/themes.generated.css"),
  ts: resolve(repo, "src/lib/themes.generated.ts"),
};

const GROUP_ORDER = [
  "Core", "Neon", "Warm", "Cool", "Greens", "Neutral",
  "Light", "Blush", "Bright", "Light Pastel", "Pastel",
];

// Compact var <-> palette key (the flat color tokens).
const TOKEN_MAP = [
  ["--fg", "fg"], ["--muted", "muted"], ["--track", "track"],
  ["--green", "green"], ["--yellow", "yellow"], ["--red", "red"], ["--live", "live"],
  ["--edge", "edge"], ["--edge-soft", "edgeSoft"], ["--hover", "hover"], ["--panel", "panel"],
];

// --- color helpers (audit only; the CSS itself uses color-mix) -----------------

function hexToRgb(h) {
  const m = h.replace("#", "");
  const n = parseInt(m.length === 3 ? m.replace(/./g, "$&$&") : m.slice(0, 6), 16);
  return [(n >> 16) & 255, (n >> 8) & 255, n & 255];
}
const mix = (a, b, pa) => a.map((v, i) => v * pa + b[i] * (1 - pa));
function lum([r, g, b]) {
  const f = (v) => ((v /= 255) <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4);
  return 0.2126 * f(r) + 0.7152 * f(g) + 0.0722 * f(b);
}
function contrast(a, b) {
  const [x, y] = [lum(a), lum(b)].sort((p, q) => q - p);
  return (x + 0.05) / (y + 0.05);
}

// Text color to put ON the accent fill: whichever of white / near-black reads better.
function onAccent(accentHex) {
  const a = hexToRgb(accentHex);
  return contrast([255, 255, 255], a) >= contrast([11, 15, 20], a) ? "#ffffff" : "#0b0f14";
}

// Text color on a --dot-run fill (the aqua/green "running" chips and update banner).
// Dark ink is the long-standing choice; flip to white when the fill is dark.
function onRun(fillHex) {
  const a = hexToRgb(fillHex);
  return contrast([255, 255, 255], a) > contrast([5, 42, 48], a) ? "#ffffff" : "#052a30";
}

// --- the bridge -------------------------------------------------------------

// Ratios live here so a tweak is one edit. "light" = a light-ground palette.
const R = {
  dark: { inset: "88%", panel: 5, elevated: 9, strong: 82, overlay: "rgba(0, 0, 0, 0.55)", shadow: "rgba(0, 0, 0, 0.5)" },
  light: { inset: "93%", panel: 2, elevated: 4, strong: 82, overlay: "rgba(0, 0, 0, 0.4)", shadow: "rgba(31, 35, 40, 0.15)" },
};

function bridge(t) {
  const light = t.scheme === "light";
  const r = light ? R.light : R.dark;
  const set = ANSI[t.id] ?? ANSI.dark;
  const blue = set[4];
  const brightBlue = set[12];
  const m = (a, pct, b = "var(--bg)") => `color-mix(in srgb, ${a} ${pct}%, ${b})`;
  const strongEnd = light ? "black" : "white";
  return {
    "--bg": "rgb(var(--bg-rgb))",
    "--bg-inset": `color-mix(in srgb, var(--bg) ${r.inset}, black)`,
    "--bg-panel": m("var(--fg)", r.panel),
    "--bg-elevated": m("var(--fg)", r.elevated),
    "--border": m("var(--fg)", 16),
    "--border-muted": m("var(--fg)", 10),
    "--border-strong": m("var(--fg)", 26),
    "--text": "var(--fg)",
    "--text-strong": m("var(--fg)", r.strong, strongEnd),
    "--text-secondary": m("var(--fg)", 80),
    "--text-muted": "var(--muted)",
    "--text-dim": m("var(--fg)", 58),
    "--text-faint": m("var(--fg)", 34),
    "--on-accent": onAccent(blue),
    "--accent": blue,
    "--focus": blue,
    "--link": brightBlue,
    "--danger": set[1],
    "--danger-border": m(set[1], 32),
    "--success": set[2],
    "--warning": set[3],
    "--dot-run": set[2],
    "--on-run": onRun(t.semantic?.["--dot-run"] ?? set[2]),
    "--dot-run-glow": `color-mix(in srgb, ${set[2]} 53%, transparent)`,
    "--dot-warn": set[3],
    "--dot-warn-glow": `color-mix(in srgb, ${set[3]} 53%, transparent)`,
    "--success-bg": m(set[2], 14),
    "--danger-bg": m(set[1], 14),
    "--info-bg": m(blue, 14),
    "--overlay": r.overlay,
    "--shadow": r.shadow,
  };
}

function block(t) {
  const c = t.colors;
  const lines = [`  color-scheme: ${t.scheme};`, `  --bg-rgb: ${c.bg.join(", ")};`];
  for (const [css, key] of TOKEN_MAP) lines.push(`  ${css}: ${c[key]};`);
  lines.push(`  --g0: ${t.gradient[0]};`, `  --g1: ${t.gradient[1]};`, `  --g2: ${t.gradient[2]};`);
  // Derived bridge first, then the legacy hand-tuned values win where present.
  const sem = { ...bridge(t), ...(t.semantic ?? {}) };
  lines.push("");
  for (const [k, v] of Object.entries(sem)) lines.push(`  ${k}: ${v};`);
  return `:root[data-theme="${t.id}"] {\n${lines.join("\n")}\n}`;
}

// --- commands ---------------------------------------------------------------

const BANNER =
  "/* GENERATED FILE - do not edit by hand.\n" +
  "   Source of truth: src/lib/themes.json (palettes) + src/lib/ansi.ts (color sets).\n" +
  "   Regenerate with:  node tools/gen-themes.mjs generate\n" +
  "   Never sets --app-alpha: the transparency system owns it (single tint layer). */";

function load() {
  return JSON.parse(readFileSync(P.json, "utf8"));
}

function generate() {
  const all = load();
  const themes = all.filter((t) => t.id !== "auto");
  const out = [BANNER];
  for (const t of themes) {
    if (t.note) out.push(`/* ${t.note} */`);
    out.push(block(t));
  }
  writeFileSync(P.css, out.join("\n\n") + "\n");

  const rows = all.map((t) => `  { id: ${JSON.stringify(t.id)}, label: ${JSON.stringify(t.label)}, group: ${JSON.stringify(t.group)} },`);
  const ts =
    "// GENERATED by tools/gen-themes.mjs from src/lib/themes.json. Do not edit by hand.\n" +
    "// Labels are palette proper nouns and stay untranslated (auto/dark/light get\n" +
    "// translated names in the UI; see themeLabel in the settings code).\n\n" +
    `export const GROUP_ORDER = ${JSON.stringify(GROUP_ORDER)} as const;\n\n` +
    "export const THEMES: { id: string; label: string; group: string }[] = [\n" +
    rows.join("\n") +
    "\n];\n";
  writeFileSync(P.ts, ts);
  console.log(`generated ${themes.length} palettes (+ auto) -> themes.generated.css, themes.generated.ts`);
}

function audit() {
  const rows = [];
  for (const t of load()) {
    if (t.id === "auto" || t.semantic) continue;
    const bg = t.colors.bg;
    const fg = hexToRgb(t.colors.fg);
    const set = ANSI[t.id] ?? ANSI.dark;
    const at = (p) => mix(fg, bg, p / 100);
    const worst = [
      ["text", contrast(fg, bg)],
      ["secondary", contrast(at(80), bg)],
      ["dim", contrast(at(58), bg)],
      ["accent", contrast(hexToRgb(set[4]), bg)],
      ["link", contrast(hexToRgb(set[12]), bg)],
      ["danger", contrast(hexToRgb(set[1]), bg)],
      ["success", contrast(hexToRgb(set[2]), bg)],
      ["warning", contrast(hexToRgb(set[3]), bg)],
      ["onAccent", contrast(hexToRgb(onAccent(set[4])), hexToRgb(set[4]))],
    ];
    rows.push([t.id, ...worst.map(([, v]) => v)]);
  }
  const names = ["text", "secondary", "dim", "accent", "link", "danger", "success", "warning", "onAccent"];
  console.log(["id", ...names].join("\t"));
  for (const r of rows) console.log([r[0], ...r.slice(1).map((v) => v.toFixed(1))].join("\t"));
  names.forEach((n, i) => {
    const min = rows.reduce((a, r) => (r[i + 1] < a[1] ? [r[0], r[i + 1]] : a), ["", 99]);
    console.log(`min ${n}: ${min[1].toFixed(2)} (${min[0]})`);
  });
}

const cmd = process.argv[2];
if (cmd === "generate") generate();
else if (cmd === "audit") audit();
else {
  console.error("usage: node tools/gen-themes.mjs <generate|audit>");
  process.exit(64);
}
