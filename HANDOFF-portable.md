# Portable mode - handoff (read this first)

**What it is:** MoonPool can run from a movable folder (zip / USB stick) with all its
data beside the exe, instead of installed into AppData. Full design + checklist:
`docs/portable-mode.md`. This file = current status + how to build/run/test.

**Status:** implemented on `main`, compiles clean (`cargo check`/`clippy`,
`svelte-check`), lightly smoke-tested. **Not released** (version still 0.1.9). Nothing
pushed. No further code changes are required to *use* it - remaining work is only
verification (see "Open" below).

---

## How portable mode works (the mental model)

- A file named `moonpool.portable` sitting **next to the exe** = portable mode. No
  flag = installed mode. Detection: `portable::is_portable()` (cached per process).
- **Installed** data → `%APPDATA%\Moonpool`. **Portable** data → `moonpool-config\`
  beside the exe. Both chosen by `moonpool_dir()`.
- apps.json path fields (`cwd`, `command`, `url`, `icon`) accept tokens `{MP_HOME}`
  (the exe/bundle folder) and `{MP_DATA}` (the config folder), plus `./`-relative
  paths anchored to `{MP_HOME}`. Raw tokens are stored on disk; they’re resolved only
  when used (launch, open-url, icon lookup).
- Two ways to enter portable mode: the first-run install card has an "or run portable
  from this folder" link, and once portable the `⋯` menu shows "Install Moonpool" to
  install onto the PC.

## Where the code is

- `src-tauri/src/portable.rs` - detection, `mp_home`/`data_dir`/`webview_data_dir`,
  token+`./` resolver, commands `portable_state` and `establish_portable`.
- `src-tauri/src/winstate.rs` - **our own** window size/pos persistence. We DROPPED
  `tauri-plugin-window-state` (it forced a dir under `%APPDATA%\<id>`). State file now
  lives in `moonpool_dir` in both modes; a corrupt/missing file falls back to the
  config default (1200×780, centered); off-screen positions are ignored.
- `src-tauri/src/lib.rs` - `moonpool_dir` delegates to `portable`; token resolution in
  `launch_app`, `open_url`, the status poller, `app_icon`; `winstate::restore` in
  `setup` and `winstate::save` on the eager Resized/Moved hook; and
  `WEBVIEW2_USER_DATA_FOLDER` set early in `run()` so WebView2’s profile lands in the
  bundle in portable mode.
- `src-tauri/src/install.rs` - `needs_setup()` returns false when portable (a portable
  exe boots the hub, not the install card).
- Frontend: `src/lib/api.ts` (`portableState`, `establishPortable`, `isNonPortablePath`),
  `src/Installer.svelte` (portable link), `src/lib/AppEditor.svelte` (amber "not
  portable" badge on absolute cwd/url), `src/lib/Sidebar.svelte` (`⋯` menu icons + the
  portable-only 🖥 "Install Moonpool" item).
- `src-tauri/resources/apps.example.json` - has a `{MP_HOME}` example entry.

## Build & run - IMPORTANT gotchas

Shell = **Git Bash** for these.

- **Standalone exe MUST be built with the Tauri CLI**, not plain cargo:
  ```bash
  cd /c/claude-local/MoonPool && npm run tauri build -- --no-bundle
  ```
  A plain `cargo build --release` produces an exe that loads the dev-server URL and
  shows **"localhost refused to connect"** (the frontend isn’t embedded). Output exe:
  `src-tauri/target/release/moonpool.exe`.

- **Test portable UI WITHOUT a full release build** (frontend hot-reloads): drop the
  flag next to the debug exe, then run dev - debug always boots the hub, and with the
  flag present it runs *portable* (data → `src-tauri/target/debug/moonpool-config`):
  ```bash
  cd /c/claude-local/MoonPool && printf x > src-tauri/target/debug/moonpool.portable && npm run tauri dev
  ```
  Remove the flag afterward so later dev runs are normal:
  ```bash
  rm -f /c/claude-local/MoonPool/src-tauri/target/debug/moonpool.portable
  ```

- **Single-instance:** launching while any `moonpool.exe` is running just forwards to it.
  Kill first: `taskkill //F //IM moonpool.exe` (Git Bash).

- **Make a real portable bundle** from a CLI-built exe:
  ```bash
  cd /c/claude-local/MoonPool && \
  B="$(mktemp -d)/bundle" && mkdir -p "$B/dashboards/sales" && \
  cp src-tauri/target/release/moonpool.exe "$B/" && \
  printf 'portable' > "$B/moonpool.portable" && \
  printf '<h1>Sales</h1>' > "$B/dashboards/sales/index.html" && \
  ( cd "$B" && ./moonpool.exe & ) && echo "$B"
  ```

## Verified

Window geometry round-trip (installed); corrupt window-state → default; portable run
boots the hub, seeds `moonpool-config` beside the exe, writes nothing to
`%APPDATA%\Moonpool`; WebView2 profile lands in the bundle (0 data files in
`%LOCALAPPDATA%`); installer "Portable" link works.

## Open (next session)

1. Confirm the `⋯` → "Install Moonpool" action actually installs: after clicking,
   check `%LOCALAPPDATA%\Moonpool\moonpool.exe`, a Start-menu shortcut, and the
   uninstall registry entry `HKCU\...\Uninstall\Moonpool`.
2. Eyeball a `{MP_HOME}` token launch and the amber editor warning in a real portable run.
3. Updater on read-only media (documented, unhandled).
4. Accepted residue: an empty `%LOCALAPPDATA%\<identifier>` dir (0 files) is still
   touched on startup in portable mode.
5. Not released; when releasing, bump version and run the normal release flow
   (see `HANDOFF-installer.md` / `docs/portable-mode.md`).

## Commits (on `main`, unpushed)

`e4bbae5` core portable · `e6f964c` hand-rolled winstate (drop plugin) ·
`9e6e34f` WebView2 profile in bundle · plus follow-ups: "Install Moonpool" menu item,
menu icons, 🖥 icon. `docs/portable-mode.md` carries the checklist and a
"Verified (2026-09-02)" section.
