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
use tauri::AppHandle;

/// Flag file that, sitting next to the exe, switches Moonpool into portable mode.
pub const FLAG_FILE: &str = "moonpool.portable";

/// Subfolder (beside the exe) that holds portable-mode data. Named `moonpool-config`
/// rather than `data` so it can't be mistaken for the user's own content in a shared
/// bundle root.
pub const DATA_SUBDIR: &str = "moonpool-config";

/// The single dotted folder that holds an entire Moonpool: the exe, its config, and its
/// bundled help/data. A portable install stamps this inside the folder the user picks, so
/// the whole app is one movable `.moonpool\` directory. This mirrors the INSTALLED layout
/// (`%USERPROFILE%\.moonpool`), so portable and installed differ only in where `.moonpool`
/// lives, not in what's inside it.
pub const APP_DIR: &str = ".moonpool";

/// Human-readable note written into the flag file when portable mode is established,
/// so anyone poking at the bundle understands what the file does.
pub const FLAG_NOTE: &str = "\
This file switches MoonPool into PORTABLE mode.

While it sits next to Moonpool.exe, MoonPool keeps all of its data
(apps.json, state.json, AI-README.md, logs) in the \"moonpool-config\"
folder beside the exe instead of in your Windows AppData. Everything
lives inside this \".moonpool\" folder, so you can move or copy the
whole \".moonpool\" folder to another PC or a USB stick and run it there.

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
/// `install_dir()` is Windows-only (USERPROFILE-based), so non-Windows falls back to the
/// same XDG_CONFIG_HOME/$HOME/.config pattern `data_dir()` uses - without this, `mp_home()`
/// silently returned `None` on Linux, which made `help::seed()`'s early-return guard skip
/// writing the help tree entirely (404 on every help:// request).
pub fn mp_home() -> Option<PathBuf> {
    if is_portable() {
        return exe_dir();
    }
    #[cfg(windows)]
    {
        crate::install::install_dir()
    }
    #[cfg(not(windows))]
    {
        let base = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .filter(|p| !p.as_os_str().is_empty())
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
        Some(base.join("Moonpool"))
    }
}

/// The data/config root (`{MP_DATA}`), mode-aware. Handle-free so both the hub and
/// the `mcp` subcommand (which never builds a Tauri app) resolve it identically:
/// - portable:  `{exe dir}\moonpool-config`
/// - installed: `{install_dir}\moonpool-config` on Windows (`%USERPROFILE%\.moonpool\
///   moonpool-config` - see `install::install_dir`, deliberately NOT AppData);
///   `$XDG_CONFIG_HOME/Moonpool` else `$HOME/.config/Moonpool` elsewhere.
pub fn data_dir() -> Option<PathBuf> {
    if is_portable() {
        return exe_dir().map(|d| d.join(DATA_SUBDIR));
    }
    #[cfg(windows)]
    {
        crate::install::install_dir().map(|d| d.join(DATA_SUBDIR))
    }
    #[cfg(not(windows))]
    {
        let base = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .filter(|p| !p.as_os_str().is_empty())
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
        Some(base.join("Moonpool"))
    }
}

/// Where WebView2 should keep its browser profile (cache, cookies, GPU/shader caches)
/// so it lands under `{MP_DATA}` instead of `%LOCALAPPDATA%\<identifier>\EBWebView`.
/// Set in BOTH modes on Windows so a Moonpool keeps everything inside its `.moonpool`
/// folder and nothing lives in AppData: portable -> beside the exe, installed ->
/// `%USERPROFILE%\.moonpool\moonpool-config\webview`. On non-Windows WebView2 isn't
/// used (WebKitGTK/WKWebView) and the env var is ignored, so only portable redirects
/// there (keeping a portable bundle self-contained); installed leaves the default.
/// Doesn't need an `AppHandle`: the data root is resolvable handle-free.
pub fn webview_data_dir() -> Option<PathBuf> {
    if is_portable() {
        return exe_dir().map(|d| d.join(DATA_SUBDIR).join("webview"));
    }
    #[cfg(windows)]
    {
        data_dir().map(|d| d.join("webview"))
    }
    #[cfg(not(windows))]
    {
        None
    }
}

/// Replace `{MP_HOME}` / `{MP_DATA}` tokens in a raw string with their resolved paths.
/// Tokens work in all modes; unresolved tokens (no home/data dir) are left as-is.
pub fn resolve_tokens(raw: &str, _app: &AppHandle) -> String {
    let mut out = raw.to_string();
    if out.contains("{MP_HOME}") {
        if let Some(h) = mp_home() {
            out = out.replace("{MP_HOME}", &h.to_string_lossy());
        }
    }
    if out.contains("{MP_DATA}") {
        if let Some(d) = data_dir() {
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
pub fn portable_state() -> PortableState {
    PortableState {
        portable: is_portable(),
        mp_home: mp_home()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default(),
        mp_data: data_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default(),
    }
}

/// The `.moonpool` app folder to stamp inside a chosen root. If the running exe is already
/// inside a `.moonpool` folder (re-establishing an existing portable copy), that folder is
/// the app dir; otherwise create `<root>\.moonpool`.
fn app_dir_in(root: &Path) -> PathBuf {
    if root.file_name().and_then(|n| n.to_str()) == Some(APP_DIR) {
        root.to_path_buf()
    } else {
        root.join(APP_DIR)
    }
}

/// Stamp a portable Moonpool into `<root>\.moonpool\`: create the folder, copy this exe in,
/// drop the flag file (with note), and make an empty config folder. Returns the path of the
/// exe to launch. Never writes outside the `.moonpool` folder. Copying is skipped when the
/// running exe is already the target (re-establishing in place), so a running exe is never
/// copied onto itself.
fn stamp_portable(root: &Path) -> Result<PathBuf, String> {
    let app_dir = app_dir_in(root);
    std::fs::create_dir_all(&app_dir).map_err(|e| format!("create {APP_DIR}: {e}"))?;

    let src_exe = std::env::current_exe().map_err(|e| format!("current_exe: {e}"))?;
    let exe_name = src_exe.file_name().ok_or("exe has no file name")?;
    let exe_target = app_dir.join(exe_name);
    if src_exe.canonicalize().ok() != exe_target.canonicalize().ok() {
        std::fs::copy(&src_exe, &exe_target).map_err(|e| format!("copy exe: {e}"))?;
    }

    std::fs::write(app_dir.join(FLAG_FILE), FLAG_NOTE).map_err(|e| format!("write flag: {e}"))?;
    std::fs::create_dir_all(app_dir.join(DATA_SUBDIR))
        .map_err(|e| format!("create config dir: {e}"))?;
    Ok(exe_target)
}

/// Establish portable mode from the installer, in place: stamp a `.moonpool\` folder in the
/// folder the exe currently sits in, moving the app into it, then relaunch the stamped exe
/// so a fresh process boots portable and quit this one. Mirrors
/// `install::launch_installed_and_exit`.
#[tauri::command]
pub fn establish_portable(app: AppHandle) -> Result<(), String> {
    let dir = exe_dir().ok_or("cannot locate exe folder")?;
    let exe = stamp_portable(&dir)?;
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
    // Empty target means "here" (the exe's current folder); otherwise the chosen folder,
    // which must exist. Either way a `.moonpool\` is stamped inside it.
    let root = if target_dir.trim().is_empty() {
        exe_dir().ok_or("cannot locate exe folder")?
    } else {
        let t = PathBuf::from(&target_dir);
        if !t.is_dir() {
            return Err("target folder does not exist".into());
        }
        t
    };

    let exe = stamp_portable(&root)?;
    crate::install::relaunch_and_exit(&app, &exe);
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
pub fn export_portable(_app: AppHandle, target_dir: String, clone: bool) -> Result<String, String> {
    let target = PathBuf::from(&target_dir);
    if !target.is_dir() {
        return Err("target folder does not exist".into());
    }
    let app_dir = app_dir_in(&target);

    let src_exe = std::env::current_exe().map_err(|e| format!("current_exe: {e}"))?;
    let exe_name = src_exe.file_name().ok_or("exe has no file name")?;
    let exe_target = app_dir.join(exe_name);

    // Don't export onto the running exe's own folder.
    if src_exe.canonicalize().ok() == exe_target.canonicalize().ok() {
        return Err("pick a different folder - that's this exe's own folder".into());
    }

    // Don't silently clobber an unrelated file: only overwrite a dest exe that's already
    // part of a portable Moonpool (flag file beside it in the .moonpool folder). A
    // same-named exe with no flag is something else - refuse rather than replace it.
    if exe_target.exists() && !app_dir.join(FLAG_FILE).exists() {
        return Err("that folder already has a .moonpool with an exe of this name that isn't a portable Moonpool - pick an empty folder".into());
    }

    let exe_target = stamp_portable(&target)?;
    let cfg = app_dir.join(DATA_SUBDIR);

    if clone {
        if let Some(src_cfg) = data_dir() {
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
/// user can zip it). Goes through the Tauri opener so it works on every platform,
/// not just Windows' explorer.exe.
#[tauri::command]
pub fn reveal_path(app: AppHandle, path: String) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    app.opener()
        .open_path(path, None::<&str>)
        .map_err(|e| format!("open folder: {e}"))
}
