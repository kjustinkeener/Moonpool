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

## Open decisions
- Build order / first-cut dashboard list.
- Flagship delivery: loopback-token endpoint vs in-app webview view; history-log
  schema + rotation.

## Suggested next step
Pick the first-cut dashboard list + start the shared `dashboards/_lib/` design
system and one source demo (CSV or XLSX) as the template, plus the history-logging
groundwork for the flagship.
