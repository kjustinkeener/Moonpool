# Portable mode (design note)

Status: **implemented (v0.1.10-dev), not yet released or end-to-end verified.** Backend
detection + path resolution, installer "Portable" option, and the amber editor warning
are in; compiles clean (`cargo check`, `svelte-check`). Still wants a real bundle-on-a-
stick smoke test. See also [HANDOFF-installer.md](../HANDOFF-installer.md) for the
custom installer/updater this builds on.

Implementation lives in `src-tauri/src/portable.rs` (detection, `mp_home`/`data_dir`,
`resolve_tokens`/`resolve_path`, the `portable_state` + `establish_portable` commands),
wired through `moonpool_dir`, `launch_app`, `open_url`, `app_icon`, the status poller,
and the window-state plugin in `lib.rs`. Frontend: `establishPortable`/`portableState`/
`isNonPortablePath` in `src/lib/api.ts`, the installer link in `src/Installer.svelte`,
and the amber badge in `src/lib/AppEditor.svelte`.

## Why

Two real use cases, one mechanism:

1. **Bundle handoff (non-technical).** Someone builds dashboards with Claude (Python,
   static HTML, whatever) and wants to hand the whole set to a coworker as a zip or on
   a USB stick. Recipient unzips, double-clicks `Moonpool.exe`, and every dashboard is
   ready to launch from relative subfolders. No install, no path fixups.
2. **Personal toolbox (IT).** Someone keeps a collection of Claude-built apps across
   mixed stacks on a stick, plugs into any workstation, runs MoonPool, launches any of
   them in place.

Both need MoonPool to (a) run without installing and (b) keep its data beside the exe
so the whole folder is movable, plus (c) config paths that anchor to the bundle rather
than absolute machine paths.

This is also a deliberate **marketing / website** angle - see "Website" below.

## Mode detection

A flag file **`moonpool.portable`** (lowercase) sitting next to the exe = portable mode.
It carries a human-readable note explaining itself (drafted below). Detection is
presence-based and only governs launch; the mode is *established* by the installer.

- First run of a freshly downloaded exe already enters **installer mode** (it's running
  from outside the install dir - see `install.rs` / `lib.rs` first-run-install branch),
  so there is no bootstrap trap. The installer offers a **"Portable (run from this
  folder)"** choice that drops `moonpool.portable` (with note) and skips relocating the
  exe into `%LOCALAPPDATA%\Moonpool`.
- A dedicated flag file (rather than inferring from the presence of seeded config) keeps
  "am I portable?" separate from "is config seeded yet?".

Delete `moonpool.portable` to return to normal installed mode.

## Data location

The config/data root (`moonpool_dir` in `lib.rs`) becomes mode-aware:

- **Portable:** `{MP_HOME}\moonpool-config\` (a subfolder beside the exe). Named
  `moonpool-config` - not `data` - so it's unambiguous and won't collide with the
  user's own folders in a shared bundle root.
- **Installed:** unchanged - `%APPDATA%\Moonpool` (Roaming).

The first-run seeding already in place (embedded `apps.example.json` -> `apps.json`,
embedded `AI-README.md` written beside it) just targets this folder; nothing extra to
bundle, the single exe carries the templates.

## Path tokens

Config path fields (`cwd`, `command`, `url`, `icon`) support tokens, resolved per mode:

| Token       | Portable                         | Installed                          |
|-------------|----------------------------------|------------------------------------|
| `{MP_HOME}` | the exe folder (bundle root)     | `%LOCALAPPDATA%\Moonpool`          |
| `{MP_DATA}` | `{MP_HOME}\moonpool-config`      | `%APPDATA%\Moonpool` (Roaming)     |

Plus an implicit rule: a **relative `cwd`** (starting `./` or `.\`) resolves against
`{MP_HOME}`, so simple bundles need no token at all.

Tokens work in **all modes**, not just portable - an installed user can anchor a tidy
`{MP_DATA}\apps\...` tree too. The only mode-specific behavior is the warning below.

Example bundle entry:

```json
{
  "id": "sales-dashboard",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/sales/index.html"
}
```

## Non-portable path warning

Absolute or machine-dependent paths still **work** in portable mode (never blocked) but
are flagged so a user prepping a handoff can spot them:

- Flag when, in portable mode, a path is absolute (`C:\...`, UNC `\\...`, POSIX `/...`)
  or contains a machine env var (`%USERPROFILE%`, `%APPDATA%`, ...). Paths under
  `{MP_HOME}`/`{MP_DATA}` or `./`-relative are safe.
- UI: passive **amber** accent + a small "not portable" badge in the app editor / tile,
  tooltip "Absolute path - won't move with this folder." Amber = caveat, not error.
- Installed mode: no warning (absolute paths are normal there).
- Later polish: one-click "portablize" that rewrites an absolute path to `{MP_HOME}\...`
  when the target actually lives under the bundle root.

## Bundle layout

```
USB root  (or unzipped folder)
├─ Moonpool.exe
├─ moonpool.portable          flag file + note
├─ moonpool-config\           {MP_DATA}: apps.json, state.json, AI-README.md, logs
├─ dashboards\                user content, referenced via {MP_HOME}\dashboards\...
└─ apps\
```

## `moonpool.portable` file contents (draft)

```
This file switches MoonPool into PORTABLE mode.

While it sits next to Moonpool.exe, MoonPool keeps all of its data
(apps.json, state.json, AI-README.md, logs) in the "moonpool-config"
folder beside the exe instead of in your Windows AppData. Nothing is
written outside this folder, so you can move or copy the whole folder
to another PC or a USB stick and run it there.

Paths in apps.json can use {MP_HOME} (this folder) so your apps and
dashboards travel with it. Absolute paths still work but won't move
with the folder.

Delete this file to return to normal installed mode (data goes back
to AppData).
```

## Implementation checklist

- [x] Mode detection: `moonpool.portable` beside exe -> portable. (`portable::is_portable`,
      cached; also short-circuits `install::needs_setup` so a portable exe boots the hub.)
- [x] `moonpool_dir` returns `{MP_HOME}\moonpool-config` in portable mode.
- [x] Point the window-state plugin at the portable data folder. The plugin only exposes
      `with_filename` (not the dir), but it does `app_config_dir().join(filename)`, and
      joining an **absolute** path replaces the base - so we hand it the absolute
      `{MP_DATA}\.window-state.json`. Residual: the plugin still `create_dir_all`s the
      (now empty) `%APPDATA%\com.moonpool.app`; no data lands there, but the empty dir is
      created. Acceptable for the "no data outside the folder" claim.
- [x] Token + relative-path resolver applied to every path field read from `apps.json`
      (`launch_app` cwd/command, `open_url`, poller auto-open url, `app_icon` cwd/url/icon).
      Raw tokens are preserved on disk / in the editor; resolution happens at point of use.
- [x] Installer "Portable" option: `establish_portable` writes `moonpool.portable` (with
      note), creates `moonpool-config`, and relaunches in place (skips exe relocation).
- [x] Amber non-portable-path warning in the app editor (cwd + url fields). Classification
      is `isNonPortablePath` in `api.ts` (TS-side, live as you type); `portable_state`
      reports the mode. Tile badge: not done (editor-only for now).
- [x] Update `apps.example.json` to demonstrate `{MP_HOME}` tokens (added a
      `portable-dashboard` static entry; existing absolute stubs kept for the common
      installed case).
- [ ] Updater note: in-place self-replace works from a writable volume; no-ops on
      read-only media. Acceptable; document it. (Not yet handled/tested.)

## Website

Use this as a headline use case on the site - it's a differentiator, not a checkbox:

- "Hand someone a whole set of dashboards as a zip - they double-click and run."
- "Carry your Claude-built toolbox across stacks on a USB stick."
- Show the bundle-layout tree above and the one-line launch story.

Also feeds the AlternativeTo listing: legitimately earns the **Portable** feature tag
once shipped (do not claim it before then), alongside Open Source, Free, Lightweight,
Support for Themes, Privacy focused.
