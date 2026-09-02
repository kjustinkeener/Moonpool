//! Portable mode: run Moonpool from a movable folder (zip / USB stick) with all of
//! its data kept beside the exe instead of in Windows AppData.
//!
//! Portable mode is established by the installer (the "Portable" option drops the
//! `moonpool.portable` flag file beside the exe and skips relocating it into
//! `%LOCALAPPDATA%\Moonpool`). Thereafter detection is purely presence-based: while
//! the flag sits next to the exe, we are portable. Delete it to return to installed
//! mode. See `docs/portable-mode.md` for the full design.

use std::path::{Path, PathBuf};
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

/// Portable-only: where WebView2 should keep its browser profile (cache, cookies,
/// GPU/shader caches) so it lands inside the bundle instead of
/// `%LOCALAPPDATA%\<identifier>\EBWebView`. None in installed mode (leave WebView2's
/// default). Doesn't need an `AppHandle`: portable data always lives beside the exe.
pub fn webview_data_dir() -> Option<PathBuf> {
    if is_portable() {
        exe_dir().map(|d| d.join(DATA_SUBDIR).join("webview"))
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
    crate::install::relaunch_and_exit(&app, &exe);
    Ok(())
}

/// Installer "run portable" with a chosen folder. If `target_dir` is the folder the
/// exe is already sitting in (or empty), this is the in-place case: drop the flag
/// beside the current exe and relaunch it. Otherwise stamp a copy into `target_dir`
/// (exe + flag + empty config) and launch that. Either way the fresh process boots
/// portable and this installer process exits. `clone` is intentionally omitted: a
/// first-run install starts fresh (the example manifest seeds on first boot).
#[tauri::command]
pub fn establish_portable_at(app: AppHandle, target_dir: String) -> Result<(), String> {
    let src_exe = std::env::current_exe().map_err(|e| format!("current_exe: {e}"))?;
    let src_dir = src_exe
        .parent()
        .ok_or("cannot locate exe folder")?
        .to_path_buf();

    let same = target_dir.trim().is_empty() || {
        let t = PathBuf::from(&target_dir);
        if !t.is_dir() {
            return Err("target folder does not exist".into());
        }
        t.canonicalize().ok() == src_dir.canonicalize().ok()
    };

    let exe_to_launch = if same {
        std::fs::write(src_dir.join(FLAG_FILE), FLAG_NOTE).map_err(|e| format!("write flag: {e}"))?;
        let _ = std::fs::create_dir_all(src_dir.join(DATA_SUBDIR));
        src_exe.clone()
    } else {
        let target = PathBuf::from(&target_dir);
        let exe_target = target.join("moonpool.exe");
        std::fs::copy(&src_exe, &exe_target).map_err(|e| format!("copy exe: {e}"))?;
        std::fs::write(target.join(FLAG_FILE), FLAG_NOTE).map_err(|e| format!("write flag: {e}"))?;
        std::fs::create_dir_all(target.join(DATA_SUBDIR))
            .map_err(|e| format!("create config dir: {e}"))?;
        exe_target
    };

    crate::install::relaunch_and_exit(&app, &exe_to_launch);
    Ok(())
}

/// Create a self-contained portable Moonpool in `target_dir`: copy this exe there,
/// drop the `moonpool.portable` flag, and make a `moonpool-config` folder. This is
/// the inverse of installing - it's meant to be run from an INSTALLED Moonpool to
/// stamp out a movable copy (per-department folder, USB stick, a zip to hand off).
///
/// `clone = true` copies the current install's apps.json, icons and settings into the
/// portable copy so it's a working clone; `false` leaves the config empty so the
/// copy boots fresh (its first launch seeds the example manifest). The current
/// install is never touched. Returns the new exe's path.
#[tauri::command]
pub fn export_portable(app: AppHandle, target_dir: String, clone: bool) -> Result<String, String> {
    let target = PathBuf::from(&target_dir);
    if !target.is_dir() {
        return Err("target folder does not exist".into());
    }
    let src_exe = std::env::current_exe().map_err(|e| format!("current_exe: {e}"))?;
    let exe_target = target.join("moonpool.exe");

    // Don't stamp a copy onto the running exe's own folder.
    if src_exe.canonicalize().ok() == exe_target.canonicalize().ok() {
        return Err("pick a different folder - that's this exe's own folder".into());
    }

    std::fs::copy(&src_exe, &exe_target).map_err(|e| format!("copy exe: {e}"))?;
    std::fs::write(target.join(FLAG_FILE), FLAG_NOTE).map_err(|e| format!("write flag: {e}"))?;

    let cfg = target.join(DATA_SUBDIR);
    std::fs::create_dir_all(&cfg).map_err(|e| format!("create config dir: {e}"))?;

    if clone {
        if let Some(src_cfg) = data_dir(&app) {
            // Durable config only; skip runtime/cache files (state.json, logs, webview)
            // that shouldn't travel with the bundle.
            for name in ["apps.json", "settings.json", "AI-README.md"] {
                let s = src_cfg.join(name);
                if s.is_file() {
                    std::fs::copy(&s, cfg.join(name)).map_err(|e| format!("copy {name}: {e}"))?;
                }
            }
            let icons = src_cfg.join("icons");
            if icons.is_dir() {
                copy_dir(&icons, &cfg.join("icons")).map_err(|e| format!("copy icons: {e}"))?;
            }
        }
    }

    Ok(exe_target.display().to_string())
}

/// Recursively copy a directory's files and subdirectories.
fn copy_dir(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let src = entry.path();
        let dst = to.join(entry.file_name());
        if src.is_dir() {
            copy_dir(&src, &dst)?;
        } else {
            std::fs::copy(&src, &dst)?;
        }
    }
    Ok(())
}

/// Open a folder in the OS file manager (used after creating a portable copy so the
/// user can zip it). explorer.exe often returns a non-zero exit even on success, so
/// we only spawn it and don't check the status.
#[tauri::command]
pub fn reveal_path(path: String) -> Result<(), String> {
    let mut c = std::process::Command::new("explorer");
    c.arg(&path);
    crate::platform::hidden(&mut c);
    c.spawn().map_err(|e| format!("open folder: {e}"))?;
    Ok(())
}
