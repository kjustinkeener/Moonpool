//! Portable mode: run Moonpool from a movable folder (zip / USB stick) with all of
//! its data kept beside the exe instead of in Windows AppData.
//!
//! Portable mode is established by the installer (the "Portable" option drops the
//! `moonpool.portable` flag file beside the exe and skips relocating it into
//! `%LOCALAPPDATA%\Moonpool`). Thereafter detection is purely presence-based: while
//! the flag sits next to the exe, we are portable. Delete it to return to installed
//! mode. See `docs/portable-mode.md` for the full design.

use std::path::PathBuf;
use std::sync::OnceLock;

use serde::Serialize;
use tauri::{AppHandle, Manager};

/// Flag file that, sitting next to the exe, switches Moonpool into portable mode.
pub const FLAG_FILE: &str = "moonpool.portable";

/// Subfolder (beside the exe) that holds portable-mode data. Named `moonpool-config`
/// rather than `data` so it can't be mistaken for the user's own content in a shared
/// bundle root.
pub const DATA_SUBDIR: &str = "moonpool-config";

/// Human-readable note written into the flag file when portable mode is established,
/// so anyone poking at the bundle understands what the file does.
pub const FLAG_NOTE: &str = "\
This file switches MoonPool into PORTABLE mode.

While it sits next to Moonpool.exe, MoonPool keeps all of its data
(apps.json, state.json, AI-README.md, logs) in the \"moonpool-config\"
folder beside the exe instead of in your Windows AppData. Nothing is
written outside this folder, so you can move or copy the whole folder
to another PC or a USB stick and run it there.

Paths in apps.json can use {MP_HOME} (this folder) so your apps and
dashboards travel with it. Absolute paths still work but won't move
with the folder.

Delete this file to return to normal installed mode (data goes back
to AppData).
";

/// The folder holding the running exe (the bundle root in portable mode).
pub fn exe_dir() -> Option<PathBuf> {
    std::env::current_exe()
        .ok()?
        .parent()
        .map(|p| p.to_path_buf())
}

/// Whether the portable flag file sits next to the exe. Cached for the process: the
/// flag is only meant to change between runs (add/remove it, then relaunch).
pub fn is_portable() -> bool {
    static PORTABLE: OnceLock<bool> = OnceLock::new();
    *PORTABLE.get_or_init(|| {
        exe_dir()
            .map(|d| d.join(FLAG_FILE).is_file())
            .unwrap_or(false)
    })
}

/// The `{MP_HOME}` anchor: the bundle root in portable mode, the install dir otherwise.
pub fn mp_home() -> Option<PathBuf> {
    if is_portable() {
        exe_dir()
    } else {
        crate::install::install_dir()
    }
}

/// The data/config root (`{MP_DATA}`), mode-aware:
/// - portable:  `{exe dir}\moonpool-config`
/// - installed: `%APPDATA%\Moonpool` (Roaming), unchanged from before portable mode.
pub fn data_dir(app: &AppHandle) -> Option<PathBuf> {
    if is_portable() {
        exe_dir().map(|d| d.join(DATA_SUBDIR))
    } else {
        app.path().config_dir().ok().map(|d| d.join("Moonpool"))
    }
}

/// Absolute path the window-state plugin should write to in portable mode, so the
/// window layout travels with the folder too (`{MP_DATA}\.window-state.json`). None
/// in installed mode (keep the plugin's default `%APPDATA%\com.moonpool.app` path).
///
/// The plugin only lets us override the *filename*, not the directory, but it joins
/// that filename onto `app_config_dir()` -- and joining an absolute path replaces the
/// base -- so handing it an absolute path redirects the state file into our folder.
/// Doesn't need an `AppHandle`: portable data always lives beside the exe.
pub fn window_state_filename() -> Option<String> {
    if is_portable() {
        exe_dir().map(|d| {
            d.join(DATA_SUBDIR)
                .join(".window-state.json")
                .to_string_lossy()
                .to_string()
        })
    } else {
        None
    }
}

/// Replace `{MP_HOME}` / `{MP_DATA}` tokens in a raw string with their resolved paths.
/// Tokens work in all modes; unresolved tokens (no home/data dir) are left as-is.
pub fn resolve_tokens(raw: &str, app: &AppHandle) -> String {
    let mut out = raw.to_string();
    if out.contains("{MP_HOME}") {
        if let Some(h) = mp_home() {
            out = out.replace("{MP_HOME}", &h.to_string_lossy());
        }
    }
    if out.contains("{MP_DATA}") {
        if let Some(d) = data_dir(app) {
            out = out.replace("{MP_DATA}", &d.to_string_lossy());
        }
    }
    out
}

/// Resolve a path field from apps.json: expand tokens, then anchor a `./` or `.\`
/// relative path against `{MP_HOME}` (the bundle root) so simple bundles need no
/// token at all. A bare relative path (no `./`) is left untouched.
pub fn resolve_path(raw: &str, app: &AppHandle) -> String {
    let s = resolve_tokens(raw, app);
    let rest = s.strip_prefix("./").or_else(|| s.strip_prefix(".\\"));
    if let Some(rest) = rest {
        if let Some(home) = mp_home() {
            return home.join(rest).to_string_lossy().to_string();
        }
    }
    s
}

/// Snapshot of portable state for the frontend (drives the amber warning UI).
///
/// The "is this path portable?" classification itself lives in the frontend
/// (`isNonPortablePath` in api.ts) so the editor can flag paths as you type without a
/// round-trip; this command just reports the mode + anchors.
#[derive(Serialize)]
pub struct PortableState {
    pub portable: bool,
    pub mp_home: String,
    pub mp_data: String,
}

/// Report portable mode + the resolved anchors to the frontend.
#[tauri::command]
pub fn portable_state(app: AppHandle) -> PortableState {
    PortableState {
        portable: is_portable(),
        mp_home: mp_home()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default(),
        mp_data: data_dir(&app)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default(),
    }
}

/// Establish portable mode from the installer: drop the flag file (with note) beside
/// the current exe, then relaunch that exe so a fresh process boots portable and quit
/// this one. Mirrors `install::launch_installed_and_exit`.
#[tauri::command]
pub fn establish_portable(app: AppHandle) -> Result<(), String> {
    let dir = exe_dir().ok_or("cannot locate exe folder")?;
    std::fs::write(dir.join(FLAG_FILE), FLAG_NOTE).map_err(|e| format!("write flag: {e}"))?;
    // Make sure the data folder exists so the first boot has somewhere to seed into.
    let _ = std::fs::create_dir_all(dir.join(DATA_SUBDIR));

    let exe = std::env::current_exe().map_err(|e| format!("current_exe: {e}"))?;
    let mut c = std::process::Command::new(&exe);
    crate::platform::hidden(&mut c);
    c.spawn().map_err(|e| format!("relaunch: {e}"))?;
    app.exit(0);
    Ok(())
}
