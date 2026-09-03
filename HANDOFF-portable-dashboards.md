# Handoff: portable verification + installer fixes + dashboard scaffold

Worktree branch: `claude/affectionate-nightingale-d670a3`. Read this first, then
`docs/portable-scaffold.md` for the full dashboard spec.

## Original task (partly done)
Verify the "Create portable copy" feature (commit e1708ff) end-to-end with a real
release build. Along the way the user spotted installer bugs and pivoted into
speccing an example-dashboard bundle.

## Done + committed
- `92445f1` - installer close button (frameless window had no exit; new `quit_app`
  command), "Install portable" now opens a folder picker (`establish_portable_at`,
  stamps copy into chosen folder; own-folder = in-place), monochrome inline save
  icon in the hub menu, and the **relaunch-race fix**: a portable/install relaunch
  passed `--wait-pid <parent>` so the child waits for the parent to exit before
  building (shared `install::relaunch_and_exit`) - fixes "after portable install,
  first run re-shows the installer" (single-instance was routing the copy back into
  the still-alive installer -> `show_main` -> installer window).
- `a8e908e`, `9a1d366` - `docs/portable-scaffold.md` spec.
- Files touched: `src-tauri/src/{install.rs,lib.rs,portable.rs}`,
  `src/{Installer.svelte,lib/api.ts,lib/Sidebar.svelte}`, `docs/portable-scaffold.md`.
- Uncommitted at handoff: latest edits to `docs/portable-scaffold.md` (data-source
  matrix + resolved decisions) - commit them.

## Verified
- Fresh portable BOOT works headlessly with the real build: copied exe + flag ->
  seeds `moonpool-config` beside the exe (apps.json, state.json, webview profile),
  `%APPDATA%\Moonpool` untouched. (That's the dev-impossible detection path.)
- Build is clean: `cd /c/claude-local/MoonPool && npm run tauri build -- --no-bundle`
  (Git Bash). Exe at `src-tauri/target/release/moonpool.exe`.

## NOT yet verified (needs the running hub + user driving the native folder picker)
- The `...` menu **Create portable copy** modal: Fresh vs Clone file results, the
  "Open folder" reveal, and the absolute-path warning listing (add an app with an
  absolute cwd first). Relevant: `src/lib/PortableExport.svelte`, `portable.rs`
  `export_portable`/`reveal_path`, `isNonPortablePath` in `src/lib/api.ts`.
- The installer changes (close button, folder-picker portable, no re-show on first
  run) - eyeball in the running installer.

## Dashboard bundle - locked decisions (see docs/portable-scaffold.md)
- Scope: ONLY self-contained static "fake"/real-data dashboards. NO bundled apps,
  installers, Scoop, package managers.
- Fully self-contained exe: embed `examples/dashboards/**` (include_dir/rust-embed),
  write out on first run, skip existing.
- Cross-platform, offline, no CDN: inline-vendor ECharts (Apache-2.0) + a shared
  dark design system under `dashboards/_lib/`.
- Two seeds: portable = dashboards only; installed = dashboards + teaching
  placeholders. Add `apps.example.portable.json`, branch on `portable::is_portable()`
  in `load_manifest` (`src-tauri/src/lib.rs`).
- **Flagship = live Moonpool control panel**, fed by NEW lightweight history logging
  (append status/launch/stop/restart/crash events to a rolling file in `{MP_DATA}`);
  gives a real time dimension the sidebar can't. Delivery leaning a loopback +
  random-port + per-launch-token, GET-only, read-only endpoint (no firewall prompt,
  no real attack surface). Plain live state.json was rejected as redundant.
- **Real-world "drop your data" dashboards, one per source format**: CSV/TSV
  (PapaParse MIT), JSON (native), JSONL/NDJSON (native), YAML (js-yaml MIT), XLSX
  (SheetJS Apache-2.0), TOML (inline), SQLite (sql.js MIT, stretch), Markdown (docs
  browser). Pattern: `<input type=file>`/drag-drop + FileReader (no server/CORS),
  inline parser -> ECharts. Each ships a baked sample for instant wow AND accepts the
  viewer's own file. The docs browser also hosts Moonpool's own help offline.
- Purpose framing: the dashboards are low-key advertising for the user's consulting,
  so polish matters; they must also show real Moonpool usage.

## BUILT (2026-09-02, commits 7c315d3, 333d6c0)

Shared library + five "drop your data" explorers, all under
`src-tauri/resources/examples/dashboards/`, offline, no CDN, no build step.

- `_lib/theme.css` - dark portfolio-grade design system (tokens, KPI tiles, tabs,
  mapping strip, data table, code pane). `_lib/echarts-theme.js` - matching "moonpool"
  ECharts theme. `_lib/profile.js` - auto-profiler (type + cardinality ->
  `MP.suggestMapping`). `_lib/records.js` - `MP.toRecords()` flattens any
  JSON/YAML/TOML shape to a flat record array. `_lib/explorer.js` -
  `MP.createExplorer(root, cfg)`: tabs, field-mapping strip (X / measures / group /
  chart), aggregation, file-load + drag-drop, parsed-table + raw-source panes.
- `_lib/vendor/`: echarts 5.5.1, papaparse 5.4.1, js-yaml 4.1.0 (global `jsyaml`),
  j-toml 1.38.0 (global `TOML`, parse with `{bigint:false}`).
- Demos: `csv/`, `json/`, `jsonl/`, `yaml/`, `toml/index.html`. Each = 4
  differently-shaped baked sample tabs (analyzer torture test) + bring-your-own-file
  landing in a "Your data" tab. Distinct landing chart per demo (csv pie/donut, json
  line, jsonl area, yaml bar, toml scatter).
- Style rule: NO U+2014 em-dashes anywhere (pre-commit hook blocks); use en-dash.
- Verify visually by opening any `index.html` directly in a browser (plain file://).

## FLAGSHIP CONTROL PANEL - DROPPED (do not build)
Decision reversed after a mockup review. From live state it is redundant with the
sidebar; the only non-redundant version samples real per-process CPU/RAM/latency
(needs `sysinfo` + a lazy loopback endpoint) and was judged not worth the backend
cost. History-logging idea abandoned too (user: needless file writes). Mockup was
built at `dashboards/control-panel/` and REMOVED in 333d6c0. If ever revived: it was
to be an on-demand (not launch-time) loopback server that also serves the page
same-origin, started via a `moonpool://control-panel` tile URL, auto-stopping on idle.

## REMAINING (next session)
1. **XLSX demo** - vendor SheetJS (xlsx, Apache-2.0) inline; `parseFile` via
   `XLSX.read(arrayBuffer)` -> first sheet -> `sheet_to_json` -> `MP.toRecords`.
   Mirror the csv demo; 4 differently-shaped baked sheets (embed as base64 or build
   from JS). Binary file, so read as ArrayBuffer (raw code pane: show a sheet summary,
   not bytes).
2. **Markdown docs browser** - the flagship *teaching* dashboard: a self-contained
   docs browser (vendor a small MD parser, e.g. marked, inline) that renders a folder
   of `.md`; also hosts Moonpool's own help offline. Different pattern from the
   explorer (no charts) - likely its own page, not `createExplorer`.
3. **SQLite (sql.js)** - stretch, heavier (WASM); defer.
4. **Rust embedding + seed split** (decisions 1-2 in the spec, still valid):
   embed `examples/dashboards/**` with `include_dir`, write out on first run skipping
   existing; add `apps.example.portable.json` (dashboards only) and branch on
   `portable::is_portable()` in `load_manifest` (`src-tauri/src/lib.rs`); installed
   seed = dashboards + existing teaching placeholders. Then the install/export paths
   stamp the scaffold (see spec "Install/export paths").
5. Author `dashboards/README` / attribution footer polish; confirm charts render
   inside Moonpool's own webview (CSP + the Tauri drag-drop caveat: Load button is
   primary there).
