# Portable Moonpool: opinionated scaffold + example dashboards

Status: **draft spec**. Open decisions at the bottom.

Goal: a portable Moonpool bundle that is self-contained, moves as one folder
(zip / USB), and on first launch shows a set of dashboards that look impressive and
**launch immediately on any machine** - no Node/Python/install, nothing to
configure, and cross-platform (Windows/macOS/Linux). A separate, differently-seeded
config ships for normal installed mode.

Scope decision: **no bundled third-party apps or installers.** The example content
is only self-contained "fake" dashboards (static HTML with seeded data). This keeps
the bundle small, license-clean, cross-platform, and offline.

Builds on existing portable resolution (`src-tauri/src/portable.rs`): `{MP_HOME}` =
bundle folder, `{MP_DATA}` = `{MP_HOME}/moonpool-config`, and a `./x` path anchors
to `{MP_HOME}`. The path classifier (`isNonPortablePath` in `src/lib/api.ts`) flags
absolute / `%VAR%` / UNC paths, so the scaffold just makes the relative form natural.

## Bundle layout

```
MyMoonpool/                      <- {MP_HOME}, the movable folder
|- moonpool(.exe)
|- moonpool.portable             <- flag file (mode switch + note)
|- moonpool-config/              <- {MP_DATA}: Moonpool's own config + runtime
|  |- apps.json
|  |- settings.json
|  |- AI-README.md
|  |- icons/
|  |- webview/                   <- webview profile incl. localStorage (regenerable)
|  |- state.json ,  *.log        <- runtime (regenerable)
|- dashboards/                   <- self-contained static HTML dashboards
|  |- <name>/index.html
|- apps/                         <- (optional) folder per app the user adds later
|- data/                         <- (optional) app working data that should travel
```

## Conventions (the opinion)

1. **Dashboards are static** - `dashboards/<name>/index.html`, opened in Moonpool's
   built-in webview. No runtime, no build step, no network.
2. **Reference by token/relative only** -
   `url: "file:///{MP_HOME}/dashboards/sales/index.html"` (or a `./`-relative path).
   The editor flags absolute / `%VAR%` / UNC forms; the scaffold makes the portable
   form the natural one.
3. **Cross-platform assets** - inline CSS/JS, no CDN, no OS-specific paths, so the
   same bundle renders identically on Windows/macOS/Linux.
4. **User space is separate** - if the user later adds their own apps, `apps/<id>/`
   and `data/` are the homes; Moonpool's own state stays in `moonpool-config/`.
5. **Regenerables** (`webview/`, `state.json`, logs) are the only things safe to
   delete when zipping a slim bundle.

## Example dashboards (zero dependency, instant, offline, cross-platform)

All `static` type, self-contained HTML (inline CSS/JS, no CDN so it works under CSP
and offline), opened in-app. localStorage persists in the bundle's webview profile,
so per-bundle data travels.

Candidate set (impressive, all instant, all fake/seeded data):
- **Metrics Dashboard** - animated SVG/canvas charts + KPI tiles, seeded data.
- **Sales Dashboard** - makes the existing `{MP_HOME}/dashboards/sales` entry real.
- **Notes** - localStorage notepad.
- **Habit Tracker** - streak grid, localStorage.
- **Kanban** - drag-drop board, localStorage.
- **Pomodoro / Clock** - timer, pure JS.
- **Help / Docs browser** - a self-contained docs browser (the pattern built for
  other projects) that hosts **Moonpool's own help offline**. Serves double duty:
  it's the app's built-in help AND a live, copyable example of the docs-browser
  pattern users can point at their own project docs. All content embedded, no CDN,
  cross-platform. This is the flagship teaching dashboard.

Shipped under `src-tauri/resources/examples/dashboards/**`, embedded in the binary
and written to the bundle `dashboards/` at first run.

## Two seed variants (branch on portable flag at first run)

Today: single `EXAMPLE_MANIFEST = include_str!("../resources/apps.example.json")`,
seeded in `load_manifest` on first run (`src-tauri/src/lib.rs`).

Proposed:
- Add `EXAMPLE_MANIFEST_PORTABLE = include_str!("../resources/apps.example.portable.json")`.
- In the seed path, pick portable vs installed by `portable::is_portable()`.
- **Embed** `examples/dashboards/**` in the binary and write it out on first run
  (decision #1 = self-contained exe: one file, nothing beside it until launch). Use
  `include_dir` (or `rust-embed`) to embed the tree; on first run write each file to
  `{MP_HOME}/dashboards/**` (portable) or `<install dir>/dashboards/**` (installed),
  skipping any that already exist so user edits aren't clobbered.

Content split (same static dashboards, different framing):
- **Portable seed** (`apps.example.portable.json`) - only the launch-immediately
  dashboards, all `{MP_HOME}`/relative. Every tile works offline, nothing to set up.
- **Installed seed** (`apps.example.json`) - the same dashboards **plus** the
  existing teaching entries (dev-server / CLI / desktop with placeholder paths) that
  show a developer how to wire their own real apps.

## Install/export paths that stamp this

- **Installer -> Install portable** (`establish_portable_at`, chosen folder): stamp
  the scaffold + portable seed into the target.
- **Hub -> Create portable copy -> Fresh**: same portable scaffold + seed.
- **Hub -> Create portable copy -> Clone current**: copy the user's real
  apps.json/settings/icons, and normalize any path already under `{MP_HOME}` to
  `./...`; absolute paths outside stay flagged by the existing amber warning.

## Decisions

1. **Asset bundling**: RESOLVED - embed in the binary and write out on first run
   (fully self-contained exe). `include_dir`/`rust-embed`; skip files that exist.

2. **Installed seed**: RESOLVED - dashboards + teaching placeholders. Installed seed
   = the static dashboards plus the dev-server / CLI / desktop teaching entries.

## Open decisions

3. **Dashboard set**: which of the candidates ship, and whether they share one small
   inline design system (tokens/typography) so they look like a coherent suite. The
   Help/Docs browser is confirmed in (flagship teaching dashboard).
