# Portable mode - session handoff (2026-09-02)

Portable mode is **built and mostly smoke-tested**. Design + full checklist:
`docs/portable-mode.md`. This file is the short "where things stand" note.

## What's done (committed on `main`, not pushed)

- `e4bbae5` core: `src-tauri/src/portable.rs` (flag-file detection, `mp_home`/`data_dir`,
  `{MP_HOME}`/`{MP_DATA}` + `./` resolver, `portable_state`/`establish_portable` commands);
  wired through `moonpool_dir`, `launch_app`, `open_url`, `app_icon`, status poller.
  Frontend: `establishPortable`/`portableState`/`isNonPortablePath` in `src/lib/api.ts`,
  installer link in `src/Installer.svelte`, amber cwd/url badge in `src/lib/AppEditor.svelte`.
  Example manifest gained a `{MP_HOME}` entry.
- `e6f964c` dropped `tauri-plugin-window-state`, hand-rolled `src-tauri/src/winstate.rs`
  (state file now in `moonpool_dir`, both modes; corrupt/missing -> config default
  1200x780 centered; off-screen guard; maximized preserved).
- `9e6e34f` WebView2 profile -> `{MP_DATA}\webview` via `WEBVIEW2_USER_DATA_FOLDER` set
  early in `run()` (portable only).

## Verified locally (release `--no-bundle` exe, hand-made bundle)

- Window geometry round-trip (resize -> quit -> relaunch) in installed mode.
- Corrupt `window-state.json` -> default size/pos, no crash.
- Portable run boots hub directly, seeds `moonpool-config` beside exe, nothing in
  `%APPDATA%\Moonpool`; WebView2 profile in bundle, 0 data files in `%LOCALAPPDATA%`.
- **Build gotcha (important):** a standalone release exe MUST be built with the Tauri
  CLI (`npm run tauri build -- --no-bundle`). A plain `cargo build --release` produces
  an exe that loads the dev-server URL ("localhost refused to connect") - the frontend
  isn't embedded. Use the CLI for any standalone test.

- `9e6e34f`+: `Install Moonpool` item in the sidebar `⋯` menu (`src/lib/Sidebar.svelte`),
  shown only in portable mode; runs the existing `perform_install(true)` +
  `launch_installed_and_exit`. Installer "Portable" button verified end-to-end by the
  user. The install-from-portable menu item is coded + compiles; NOT yet clicked-tested
  (running bundle still shows the pre-rename label until a rebuild).

## Not yet done / open

- Installer "Portable" button end-to-end (only the backend `establish_portable` path is
  tested, not a real download->install-card->Portable click).
- Token/`./` resolution at launch from a real bundle (the `portable-dashboard` example),
  and the amber editor warning eyeballed in portable mode.
- Updater on read-only media (documented, unhandled).
- Residue (accepted): an empty `%LOCALAPPDATA%\<identifier>` dir (0 files) still gets
  touched on startup in portable mode.
- Not released; version still 0.1.9 in `Cargo.toml`/`tauri.conf.json`.

## Test bundle

`<scratchpad>/portable-test/` (moonpool.exe + moonpool.portable + dashboards/ +
moonpool-config/). Rebuild+restage: CLI build, then copy
`src-tauri/target/release/moonpool.exe` over it. Kill stale instances with
`taskkill //F //IM moonpool.exe` (Git Bash) - single-instance forwards otherwise.
