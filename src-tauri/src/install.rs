//! Custom first-run installer. The portable `moonpool.exe` you download doubles as
//! its own installer: on first launch (running from outside the install dir) the
//! frontend shows a skinned install card that calls `perform_install`, which copies
//! the exe into place, makes shortcuts, and registers an uninstall entry.
//!
//! Install location is `%LOCALAPPDATA%\Moonpool` on purpose: it's per-user (no UAC)
//! and it's the same dir the updater's self-replace targets, so first-install and
//! later updates operate on the same files.

use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Serialize;
use tauri::AppHandle;

use crate::platform;

const APP_NAME: &str = "Moonpool";
const UNINSTALL_KEY: &str = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\Moonpool";

/// `%LOCALAPPDATA%\Moonpool`, the per-user install dir.
pub fn install_dir() -> Option<PathBuf> {
    std::env::var_os("LOCALAPPDATA").map(|p| PathBuf::from(p).join(APP_NAME))
}

/// The installed exe path, `<install_dir>\moonpool.exe`.
fn installed_exe() -> Option<PathBuf> {
    install_dir().map(|d| d.join("moonpool.exe"))
}

/// Are we running from inside the install dir (i.e. already installed)?
fn is_installed() -> bool {
    match (std::env::current_exe(), install_dir()) {
        (Ok(cur), Some(dir)) => {
            let cur = cur.canonicalize().unwrap_or(cur);
            let dir = dir.canonicalize().unwrap_or(dir);
            cur.starts_with(&dir)
        }
        _ => false,
    }
}

/// Should the app show the install card instead of booting the hub? True only in
/// release builds run from outside the install dir. Dev builds (`cargo tauri dev`)
/// always boot the hub so development isn't interrupted by the installer.
pub fn needs_setup() -> bool {
    // The custom installer (LOCALAPPDATA copy, shortcuts, registry) is Windows-only.
    // On other platforms there's nothing to install into, and returning true would
    // strand a release build on an install card whose `perform_install` fails with
    // "no LOCALAPPDATA" - so non-Windows always boots the hub directly.
    if !cfg!(windows) {
        return false;
    }
    // A portable exe (flag file beside it) boots the hub directly - it deliberately
    // runs from outside the install dir, so `is_installed()` is false there too.
    !cfg!(debug_assertions) && !is_installed() && !crate::portable::is_portable()
}

#[derive(Serialize)]
pub struct SetupState {
    /// Show the install card (release build, not yet installed).
    pub needs_setup: bool,
    /// Running from the install dir already.
    pub installed: bool,
    /// A prior install exists on disk even if this exe is the portable one.
    pub existing: bool,
    pub version: String,
    pub build_date: String,
    pub install_dir: String,
}

/// Tell the frontend whether to show the install card or boot straight to the hub.
#[tauri::command]
pub fn setup_state() -> SetupState {
    let dir = install_dir()
        .map(|d| d.display().to_string())
        .unwrap_or_default();
    let existing = installed_exe().map(|p| p.exists()).unwrap_or(false);
    SetupState {
        needs_setup: needs_setup(),
        installed: is_installed(),
        existing,
        version: env!("CARGO_PKG_VERSION").to_string(),
        build_date: env!("MOONPOOL_BUILD_DATE").to_string(),
        install_dir: dir,
    }
}

/// Copy this exe into the install dir, create shortcuts, and register uninstall.
/// Returns the installed exe path so the frontend can relaunch it.
#[tauri::command]
pub fn perform_install(desktop_shortcut: bool) -> Result<String, String> {
    let dir = install_dir().ok_or("no LOCALAPPDATA")?;
    let target = dir.join("moonpool.exe");
    let src = std::env::current_exe().map_err(|e| format!("current_exe: {e}"))?;

    std::fs::create_dir_all(&dir).map_err(|e| format!("create install dir: {e}"))?;

    // If we're somehow already the installed copy, don't copy onto ourselves.
    if src.canonicalize().ok() != target.canonicalize().ok() {
        std::fs::copy(&src, &target).map_err(|e| format!("copy exe: {e}"))?;
    }

    let start_menu = start_menu_dir().map(|d| d.join("Moonpool.lnk"));
    if let Some(lnk) = &start_menu {
        create_shortcut(lnk, &target)?;
    }
    if desktop_shortcut {
        if let Some(desk) = desktop_dir() {
            create_shortcut(&desk.join("Moonpool.lnk"), &target)?;
        }
    }

    register_uninstall(&dir, &target)?;
    Ok(target.display().to_string())
}

/// Spawn `exe` and quit this process, handing off cleanly. The child is told to wait
/// for THIS process to exit (`--wait-pid`) before it builds anything, so it doesn't
/// race the single-instance lock we still hold and get routed back into us (which, for
/// the installer, would just re-show the install card instead of booting the copy).
pub fn relaunch_and_exit(app: &AppHandle, exe: &Path) {
    let mut c = Command::new(exe);
    c.arg("--wait-pid").arg(std::process::id().to_string());
    platform::hidden(&mut c);
    let _ = c.spawn();
    app.exit(0);
}

/// Launch the installed exe and quit this (portable) process.
#[tauri::command]
pub fn launch_installed_and_exit(app: AppHandle, exe: String) {
    relaunch_and_exit(&app, Path::new(&exe));
}

/// Quit the app outright. Used by the installer's close button: the installer
/// window is frameless (no OS titlebar), so without this the only way out before
/// installing is Alt+F4 / Task Manager.
#[tauri::command]
pub fn quit_app(app: AppHandle) {
    app.exit(0);
}

// --- uninstall -------------------------------------------------------------

/// Handle a `--uninstall` invocation (called from run() before the app builds).
/// Removes shortcuts + registry, then schedules deletion of the install dir via a
/// detached cmd (we can't delete our own running exe directly).
pub fn run_uninstall() {
    remove_shortcuts();
    remove_uninstall_key();

    if let Some(dir) = install_dir() {
        // Detached: wait for THIS exe to exit (so its dir isn't locked), then remove
        // the whole install dir. Done in PowerShell, not `cmd /c "... & rmdir ..."`:
        // `cmd /c` strips the outer quotes it's handed and unbalances the quoted path
        // ("syntax is incorrect"), so the delete silently never ran. PowerShell takes
        // the path as a single-quoted literal and retries until the lock clears, and
        // -Recurse -Force clears the seeded dashboards tree too.
        // Stop any OTHER running Moonpool first (a hub left open holds moonpool.exe
        // locked, which is what made the delete fail and left the folder behind - and
        // a leftover folder is exactly what makes Windows' Program Compatibility
        // Assistant claim the uninstall failed). Then retry for ~30s.
        let dir_s = dir.display().to_string().replace('\'', "''"); // ' -> '' for PS
        let script = format!(
            "Get-Process moonpool -ErrorAction SilentlyContinue|Stop-Process -Force -ErrorAction SilentlyContinue;\
             Start-Sleep -Milliseconds 400;\
             for($i=0;$i -lt 60;$i++){{try{{Remove-Item -LiteralPath '{dir_s}' -Recurse -Force -ErrorAction Stop;break}}catch{{Start-Sleep -Milliseconds 500}}}}"
        );
        let mut c = Command::new("powershell");
        c.args(["-NoProfile", "-NonInteractive", "-Command", &script]);
        platform::hidden(&mut c);
        let _ = c.spawn();
    }
    std::process::exit(0);
}

fn remove_shortcuts() {
    if let Some(d) = start_menu_dir() {
        let _ = std::fs::remove_file(d.join("Moonpool.lnk"));
    }
    if let Some(d) = desktop_dir() {
        let _ = std::fs::remove_file(d.join("Moonpool.lnk"));
    }
}

// --- shortcut + registry helpers ------------------------------------------

fn start_menu_dir() -> Option<PathBuf> {
    std::env::var_os("APPDATA")
        .map(|p| PathBuf::from(p).join(r"Microsoft\Windows\Start Menu\Programs"))
}

fn desktop_dir() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE").map(|p| PathBuf::from(p).join("Desktop"))
}

/// Create a .lnk via WScript.Shell (PowerShell), matching the codebase's existing
/// pattern of shelling out to PowerShell for Windows-only shell work.
fn create_shortcut(lnk: &Path, target: &Path) -> Result<(), String> {
    let lnk_s = lnk.to_string_lossy().replace('\'', "''");
    let target_s = target.to_string_lossy().replace('\'', "''");
    let workdir_s = target
        .parent()
        .map(|p| p.to_string_lossy().replace('\'', "''"))
        .unwrap_or_default();
    let script = format!(
        "$w = New-Object -ComObject WScript.Shell; \
         $s = $w.CreateShortcut('{lnk_s}'); \
         $s.TargetPath = '{target_s}'; \
         $s.WorkingDirectory = '{workdir_s}'; \
         $s.IconLocation = '{target_s},0'; \
         $s.Description = 'Moonpool'; \
         $s.Save()"
    );
    let mut c = Command::new("powershell");
    c.args(["-NoProfile", "-Command", &script]);
    platform::hidden(&mut c);
    c.status()
        .map(|s| s.success())
        .map_err(|e| format!("shortcut: {e}"))
        .and_then(|ok| {
            if ok {
                Ok(())
            } else {
                Err("shortcut failed".into())
            }
        })
}

/// Today's date as the `yyyyMMdd` string "Installed apps" expects for `InstallDate`.
/// Done by hand (civil-from-days) rather than pulling in a date crate for one value.
fn install_date_stamp() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // Howard Hinnant's civil_from_days, shifted to an era starting 0000-03-01.
    let z = (secs / 86_400) as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = era * 400 + yoe + i64::from(month <= 2);
    format!("{year:04}{month:02}{day:02}")
}

/// Total size of a directory tree in bytes (best effort; unreadable entries skipped).
fn dir_size(dir: &Path) -> u64 {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return 0;
    };
    entries
        .flatten()
        .map(|e| match e.file_type() {
            Ok(t) if t.is_dir() => dir_size(&e.path()),
            Ok(_) => e.metadata().map(|m| m.len()).unwrap_or(0),
            Err(_) => 0,
        })
        .sum()
}

fn register_uninstall(dir: &Path, exe: &Path) -> Result<(), String> {
    let version = env!("CARGO_PKG_VERSION");
    let exe_s = exe.display().to_string();
    let dir_s = dir.display().to_string();
    let uninstall_cmd = format!("\"{exe_s}\" --uninstall");

    // "Installed apps" shows size and date from these; without them the row lists
    // blank and sorts oddly under "Date installed". EstimatedSize is in KB.
    let size_kb = (dir_size(dir) / 1024).max(1).to_string();
    let install_date = install_date_stamp();

    let values: [(&str, &str, &str); 11] = [
        ("DisplayName", "REG_SZ", APP_NAME),
        ("DisplayVersion", "REG_SZ", version),
        ("Publisher", "REG_SZ", "Justin Keener"),
        ("DisplayIcon", "REG_SZ", &exe_s),
        ("InstallLocation", "REG_SZ", &dir_s),
        ("UninstallString", "REG_SZ", &uninstall_cmd),
        // Same command: the uninstall is already non-interactive.
        ("QuietUninstallString", "REG_SZ", &uninstall_cmd),
        ("EstimatedSize", "REG_DWORD", &size_kb),
        ("InstallDate", "REG_SZ", &install_date),
        ("NoModify", "REG_DWORD", "1"),
        ("NoRepair", "REG_DWORD", "1"),
    ];
    for (name, ty, data) in values {
        let mut c = Command::new("reg");
        c.args(["add", UNINSTALL_KEY, "/v", name, "/t", ty, "/d", data, "/f"]);
        platform::hidden(&mut c);
        let ok = c.status().map(|s| s.success()).unwrap_or(false);
        if !ok {
            return Err(format!("registry write failed for {name}"));
        }
    }
    Ok(())
}

/// Rewrite just the `DisplayVersion` value under the uninstall key, so "Installed
/// apps" tracks the version after a self-update. The updater swaps the exe bytes in
/// place but never re-runs `register_uninstall`, so without this the row stays stuck
/// at whatever version first installed. Best-effort and a no-op when not installed
/// (portable / dev / run-in-place), where the key does not exist and must not be
/// created. `version` is the plain semver, no leading `v`.
pub fn update_display_version(version: &str) {
    if !is_installed() {
        return;
    }
    let mut c = Command::new("reg");
    c.args([
        "add",
        UNINSTALL_KEY,
        "/v",
        "DisplayVersion",
        "/t",
        "REG_SZ",
        "/d",
        version,
        "/f",
    ]);
    platform::hidden(&mut c);
    let _ = c.status();
}

fn remove_uninstall_key() {
    let mut c = Command::new("reg");
    c.args(["delete", UNINSTALL_KEY, "/f"]);
    platform::hidden(&mut c);
    let _ = c.status();
}
