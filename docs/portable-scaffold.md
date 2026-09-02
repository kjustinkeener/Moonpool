# Portable Moonpool: opinionated scaffold + example bundle

Status: **draft spec** (brainstorm in progress). Open decisions listed at the bottom.

Goal: a portable Moonpool bundle that is self-contained, moves as one folder
(zip / USB), and on first launch presents a set of apps and dashboards that look
impressive and **launch immediately on any machine** - no Node/Python/install,
nothing to configure. A separate, differently-seeded config ships for the normal
installed mode.

Builds on existing portable resolution (`src-tauri/src/portable.rs`):
`{MP_HOME}` = bundle folder, `{MP_DATA}` = `{MP_HOME}/moonpool-config`, and a
`./x` path anchors to `{MP_HOME}`. The path classifier (`isNonPortablePath` in
`src/lib/api.ts`) already flags absolute / `%VAR%` / UNC paths, so the scaffold
just makes the relative form the path of least resistance.

## Bundle layout

```
MyMoonpool/                      <- {MP_HOME}, the movable folder
|- moonpool.exe
|- moonpool.portable             <- flag file (mode switch + note)
|- moonpool-config/              <- {MP_DATA}: Moonpool's own config + runtime
|  |- apps.json
|  |- settings.json
|  |- AI-README.md
|  |- icons/
|  |- webview/                   <- WebView2 profile incl. localStorage (regenerable)
|  |- state.json  ,  *.log       <- runtime (regenerable)
|- apps/                         <- one folder per app you bundle
|  |- <app-id>/                  <- payload: scripts, project, static site
|- dashboards/                   <- self-contained static HTML dashboards
|  |- <name>/index.html
|- bin/                          <- bundled portable runtimes/tools + their LICENSE
|- data/                         <- app working data that should travel
```

## Conventions (the opinion)

1. **One folder per app** at `apps/<id>/`; its `cwd` defaults to `./apps/<id>`, so
   an app works with no token at all.
2. **Reference by relative / token only** - `./bin/node/node.exe`,
   `./apps/notes/server.js`, `./dashboards/sales/index.html`. The editor flags the
   non-portable forms; the scaffold makes the portable form the natural one.
3. **Dashboards are static** - `dashboards/<name>/index.html`, opened in Moonpool's
   built-in webview. No runtime build step.
4. **Bundled runtimes/tools live in `bin/`** so a bundle runs on a machine with
   nothing installed. Each tool ships its LICENSE alongside.
5. **Writable app data goes in `data/`** (travels), kept separate from
   `moonpool-config/` (Moonpool state) and `apps/` (read-only payload).
6. **Regenerables** (`webview/`, `state.json`, logs) are the only things safe to
   delete when zipping a slim bundle.

## Example content, in tiers

### Tier 1 - static dashboards (zero dependency, instant, offline)

All `static` type, self-contained HTML (inline CSS/JS, no CDN so it works under
CSP + offline), opened in-app. localStorage persists in the bundle's webview
profile, so per-bundle data travels.

Candidate set (impressive, all instant):
- **Metrics Dashboard** - animated SVG/canvas charts + KPI tiles, seeded data.
- **Sales Dashboard** - makes the existing `{MP_HOME}/dashboards/sales` entry real.
- **Notes** - localStorage notepad.
- **Habit Tracker** - streak grid, localStorage.
- **Kanban** - drag-drop board, localStorage.
- **Pomodoro / Clock** - timer, pure JS.

Shipped under `src-tauri/resources/examples/dashboards/**`, copied into the bundle
`dashboards/` at first run.

### Tier 2 - bundled portable tools (real apps, permissive license only)

A small curated cluster of genuinely-portable, single-binary tools that run from
`bin/` with no install, wired into the portable manifest as `cli` / `web` apps.

**License discipline: permissive only** (MIT / BSD / Apache-2.0 / ISC / zlib /
Unlicense). No GPL/LGPL-with-strings. Each tool ships its LICENSE file in its
`bin/<tool>/` folder; a `bin/THIRD-PARTY.md` lists tool + version + license + source.

Excluded by the rule (examples): Git-portable, Notepad++, SumatraPDF (GPL);
unRAR component of 7-Zip.

Candidate cluster (all MIT/Apache/Unlicense, single static exe, Windows):
- **ripgrep** (rg) - MIT/Unlicense - fast search (cli).
- **fd** - MIT/Apache - file find (cli).
- **bat** - MIT/Apache - pretty cat (cli).
- **fzf** - MIT - fuzzy finder (cli).
- **miniserve** - MIT - instant static file server (web).
- **Caddy** - Apache-2.0 - web server (web).
- **micro** - MIT - terminal editor (cli).
- optional **portable Node** - MIT - powers any JS demo apps (bin/node/).

Size tradeoff: keep the cluster small; each binary is a few MB. Consider an
"apps: lite vs full" bundle choice at export time (see open decisions).

## Two seed variants (branch on portable flag at first run)

Today: single `EXAMPLE_MANIFEST = include_str!("../resources/apps.example.json")`,
seeded in `load_manifest` on first run (`src-tauri/src/lib.rs`).

Proposed:
- Add `EXAMPLE_MANIFEST_PORTABLE = include_str!("../resources/apps.example.portable.json")`.
- In the seed path, pick portable vs installed by `portable::is_portable()`.
- Copy the bundled `examples/dashboards/**` (and, portable, `bin/**`) into
  `{MP_HOME}` (portable) or the install dir (installed) at seed time.

Content split (same scaffold, different app sets):
- **Portable seed** (`apps.example.portable.json`) - launch-immediately entries only:
  Tier 1 dashboards + Tier 2 bundled portable tools. Every tile works offline,
  nothing to configure, nothing installed on the host. Self-contained by necessity
  (the bundle may run on a locked-down or offline machine).
- **Installed seed** (`apps.example.json`) - the Tier 1 static set **plus a
  different app set** suited to an installed machine (which has internet + a package
  manager): Tier 3 "auto-install" tiles (below) instead of bundled binaries, plus
  the teaching entries that show a developer how to wire their own apps.

### Tier 3 - auto-installing app tiles (installed mode only)

The Linux-app-installer analog: a tile that installs its app on first click via a
Windows package manager, then launches it. Not bundled (keeps the installer small)
and always current.

- Prefer **winget** (built into Win10/11, no bootstrap) for the default;
  **Scoop** (no-admin, user-folder, closest to the portable ethos) for dev/CLI
  tools; **Chocolatey** as an option.
- Manifest shape: a `cli`/`desktop` entry whose command runs the install if absent
  then launches - e.g. `winget install --id <pkg> -e --silent` guarded by a
  "already installed?" check, then start the app. (Detail TBD: a small wrapper vs a
  new app-entry field like `install: { via: "winget", id: "..." }`.)
- Only permissive-to-redistribute *tiles* are seeded by default; the package
  manager fetches the actual app under its own license at the user's request, so we
  don't redistribute anything heavy.

Portable mode omits Tier 3 (no assumption of internet or a package manager).

GUI reference for this feel: **Ninite** (pick-a-list, one auto-installer) and
**UniGetUI** (a software-center GUI over winget/Scoop/Choco). Ninite installs to
default locations only (can't redirect), so it's a UX reference, not an engine.

### Tier 4 - install-into-the-bundle app store (Scoop-backed)

The strongest form of "bundled installers that install into our own app dirs":
ship a **bundle-local Scoop root** and let tiles install real apps into the bundle.

- Set `SCOOP` (and `SCOOP_GLOBAL`) to `{MP_HOME}/bin/scoop`; every `scoop install
  <x>` lands under the bundle, no admin, and travels with a portable copy.
- App tiles are `scoop install <x>` then launch; Moonpool manages the catalog.
- Scoop bootstrap is MIT; bundling it is fine. Apps it fetches stay under their own
  license, downloaded at the user's request (we don't redistribute them).

Why Scoop specifically: it's the only common Windows manager that installs into an
arbitrary, no-admin, relocatable root by design.
- **winget** `--location` is honored only when the underlying installer supports it
  (unreliable across packages); default installs go to Program Files.
- **Raw vendor installers** can be redirected only per-type via silent flags: NSIS
  `/S /D=<path>` (unquoted, last arg), Inno `/VERYSILENT /DIR="<path>"`, MSI
  `INSTALLDIR=<path> /qn`. No universal switch; **Ninite ignores custom dirs**.
- **Chocolatey** `--install-directory` works for some packages only.

Do NOT bundle third-party vendor installers by default (size + redistribution
rights). Default to: bundle the permissive single-binaries (Tier 2) + a manifest
that fetches the rest on first click (Tier 3/4).

## Install/export paths that stamp this

- **Installer -> Install portable** (`establish_portable_at`, chosen folder): stamp
  the scaffold + portable seed into the target.
- **Hub -> Create portable copy -> Fresh**: same portable scaffold + seed.
- **Hub -> Create portable copy -> Clone current**: copy the user's real
  apps.json/settings/icons, and normalize any path already under `{MP_HOME}` to
  `./...`; absolute paths outside stay flagged by the existing amber warning.

## Open decisions

1. **Installed seed content**: Tier 1 static set + Tier 3 auto-install tiles +
   teaching entries [rec], vs a lighter split. And which package manager is the
   default for Tier 3 (winget vs Scoop) and whether it's one field or a wrapper.
2. **Asset bundling**: (a) copy from Tauri-bundled resources at first run
   [works portable + installed], or (b) embed each HTML in the binary and write out
   (single-file exe until first run, heavier binary).
3. **Tier 2 tools in the default bundle**: always included, or an export-time
   "lite (dashboards only) vs full (with tools)" choice to control bundle size.
4. **Convention enforcement**: convention-only (scaffold + examples, apps still
   declare their own paths), or add an "Add app to bundle" helper that copies an
   app's files into `apps/<id>/` and rewrites its paths to `./apps/<id>/...`.
