// Moonpool - a tray launcher hub for local apps.
//
// The backend does four things:
//   1. Serves the app manifest (apps.json) to the frontend.
//   2. Launches an app's existing launch script inside a real PTY (ConPTY on
//      Windows) and streams its bytes to the frontend as `term://output` events,
//      so the embedded xterm.js terminal behaves like a real console.
//   3. Feeds keystrokes back into the PTY (`term_input`) and resizes it
//      (`term_resize`), so the terminal is interactive, not read-only.
//   4. Polls status (TCP port health-check for web apps, process-name check for
//      desktop apps) and emits `status://update`.

use std::collections::{HashMap, HashSet};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use base64::Engine as _;
use portable_pty::{native_pty_system, MasterPty, PtySize};
use serde::{Deserialize, Serialize};
use sysinfo::{Process, System};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_opener::OpenerExt;

mod dashboards;
mod i18n;
mod install;
mod mcp;
mod platform;
mod portable;
mod update;
mod winstate;

// ---------------------------------------------------------------------------
// Manifest
// ---------------------------------------------------------------------------

#[derive(Clone, Serialize, Deserialize)]
struct AppEntry {
    id: String,
    name: String,
    group: String,
    #[serde(rename = "type")]
    app_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cwd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    command: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    port: Option<u16>,
    #[serde(
        default,
        rename = "processName",
        skip_serializing_if = "Option::is_none"
    )]
    process_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(default, rename = "openBrowser")]
    open_browser: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    /// Extra environment variables injected into the launch command (e.g. PORT).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    env: Option<HashMap<String, String>>,
}

#[derive(Clone, Serialize)]
struct AppStatus {
    id: String,
    running: bool,
    managed: bool,
}

/// Outcome record for one control command the caller tagged with a ticket.
/// Published in state.json so a script/agent that fired `launch <id> --ticket K`
/// can read back whether it worked without any stdout reply channel.
#[derive(Clone, Serialize)]
struct Ticket {
    ticket: String,
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    arg: Option<String>,
    /// "pending" while the UI is acting, then "ok" or "error".
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    /// Unix millis when this record last changed.
    ts: u64,
}

#[derive(Clone, Serialize, Deserialize)]
struct Settings {
    #[serde(default, rename = "debugLogging")]
    debug_logging: bool,
    /// Hide to the tray (leave the taskbar) when the window is closed.
    #[serde(default, rename = "closeToTray")]
    close_to_tray: bool,
    /// Hide to the tray (leave the taskbar) when the window is minimized.
    #[serde(default = "default_true", rename = "minimizeToTray")]
    minimize_to_tray: bool,
    /// Check GitHub Releases for a newer version once, on app startup.
    #[serde(default = "default_true", rename = "checkOnStartup")]
    check_on_startup: bool,
    /// Background transparency, 0 (opaque) to 90 (percent see-through).
    #[serde(default = "default_transparency", rename = "transparency")]
    transparency: u8,
    /// Keep the hub (and its detached windows) above other windows.
    #[serde(default, rename = "alwaysOnTop")]
    always_on_top: bool,
    /// UI zoom factor set by Ctrl+wheel, 0.5 to 3.0. View state rather than a
    /// settings field: there is no control for it, it just has to survive a
    /// restart alongside the rest of the config.
    #[serde(default = "default_ui_scale", rename = "uiScale")]
    ui_scale: f64,
    /// UI language. The user's *choice*: "auto" follows the OS, otherwise a tag
    /// from `LOCALES` in `src/lib/i18n.svelte.ts` ("en", "pt-BR", ...).
    #[serde(default = "default_locale", rename = "locale")]
    locale: String,
    /// What "auto" last resolved to. The webview resolves the OS language and
    /// writes it back; Rust needs a concrete tag to label the tray at startup,
    /// before any window exists to ask. See `i18n::tray_strings`.
    #[serde(default = "default_locale_resolved", rename = "localeResolved")]
    locale_resolved: String,
}

fn default_true() -> bool {
    true
}
fn default_transparency() -> u8 {
    0
}
fn default_ui_scale() -> f64 {
    1.0
}
fn default_locale() -> String {
    "auto".into()
}
fn default_locale_resolved() -> String {
    "en".into()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            debug_logging: false,
            close_to_tray: false,
            minimize_to_tray: true,
            check_on_startup: true,
            transparency: default_transparency(),
            always_on_top: false,
            ui_scale: default_ui_scale(),
            locale: default_locale(),
            locale_resolved: default_locale_resolved(),
        }
    }
}

#[derive(Clone, Serialize)]
struct TermOutput {
    id: String,
    /// PTY bytes, base64-encoded (far cheaper over IPC than a JSON number array).
    data: String,
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

struct RunningApp {
    writer: Box<dyn Write + Send>,
    master: Box<dyn MasterPty + Send>,
    // The spawned PTY child: killed on stop_app, reaped by the poller's try_wait,
    // and waited on by the reader thread.
    child: Box<dyn portable_pty::Child + Send + Sync>,
    /// Signals the reader thread to exit (best-effort; honored after its next read).
    stop: Arc<AtomicBool>,
}

struct HubState {
    apps: Mutex<HashMap<String, RunningApp>>,
    manifest: Mutex<Vec<AppEntry>>,
    settings: Mutex<Settings>,
    /// Recent control-command outcomes, newest last; capped to avoid growth.
    tickets: Mutex<Vec<Ticket>>,
    /// Last status snapshot the poller computed, so an out-of-band state.json
    /// flush (after a ticket update) still writes accurate running/managed flags.
    last_statuses: Mutex<Vec<AppStatus>>,
    /// Recent PTY output per app id, capped at MAX_TERM_LOG bytes each. Kept in
    /// Rust (not just the WebView) so the `dump` control command can write an
    /// app's console output to a file, including after the app has exited.
    term_logs: Mutex<HashMap<String, Vec<u8>>>,
    /// Last non-degenerate physical window size (w, h). Used to re-assert a sane
    /// size when a Win+D restore of the borderless window brings it back as the
    /// ~215x26 minimized-placeholder sliver.
    last_good_size: Mutex<Option<(u32, u32)>>,
}

/// Cap on retained ticket records (oldest dropped first).
const MAX_TICKETS: usize = 50;

/// Per-app cap on the retained PTY output ring (bytes). Enough for a long build
/// log without letting a chatty app grow memory without bound.
const MAX_TERM_LOG: usize = 512 * 1024;

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// Generic example manifest (installed mode): the example dashboards plus the
/// teaching placeholder entries. Seeded into the config dir on first run.
const EXAMPLE_MANIFEST: &str = include_str!("../resources/apps.example.json");
/// Portable-mode example manifest: only the launch-immediately example dashboards,
/// all referenced by `{MP_HOME}` so every tile works from a moved bundle.
const EXAMPLE_MANIFEST_PORTABLE: &str = include_str!("../resources/apps.example.portable.json");

/// The example manifest to seed for the current mode (portable = dashboards only).
fn example_manifest() -> &'static str {
    if portable::is_portable() {
        EXAMPLE_MANIFEST_PORTABLE
    } else {
        EXAMPLE_MANIFEST
    }
}
/// AI configuration guide, seeded next to the manifest so agents can read it.
const AI_README: &str = include_str!("../../AI-README.md");

/// Moonpool's config directory. Installed: `%APPDATA%\Moonpool`. Portable:
/// `{exe dir}\moonpool-config` (see `portable`), so all data travels with the folder.
fn moonpool_dir(app: &AppHandle) -> Option<PathBuf> {
    let _ = app;
    portable::data_dir()
}

/// Location of the user-editable manifest: <config>/Moonpool/apps.json.
fn manifest_path(app: &AppHandle) -> Option<PathBuf> {
    moonpool_dir(app).map(|d| d.join("apps.json"))
}

/// User icon directory: <config>/Moonpool/icons.
fn icons_dir(app: &AppHandle) -> Option<PathBuf> {
    moonpool_dir(app).map(|d| d.join("icons"))
}

/// Read an image file and encode it as a data: URI (mime guessed from extension).
fn file_to_data_uri(path: &Path) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    let mime = match path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_lowercase)
        .as_deref()
    {
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("webp") => "image/webp",
        _ => "image/png",
    };
    Some(format!(
        "data:{};base64,{}",
        mime,
        base64::engine::general_purpose::STANDARD.encode(bytes)
    ))
}

/// Whether process `p`'s name matches `name`, with or without a `.exe` suffix
/// (case-insensitive). Shared by the exe lookup and the status poller.
fn matches_process_name(p: &Process, name: &str) -> bool {
    let target = name.to_lowercase();
    let n = p.name().to_lowercase();
    n == target || n == format!("{target}.exe")
}

/// Path of the running process whose name matches `pn` (with or without .exe).
fn running_exe_path(pn: &str) -> Option<PathBuf> {
    let mut sys = System::new();
    sys.refresh_processes();
    sys.processes().values().find_map(|p| {
        if matches_process_name(p, pn) {
            match p.exe() {
                Some(path) if !path.as_os_str().is_empty() => Some(path.to_path_buf()),
                _ => None,
            }
        } else {
            None
        }
    })
}

/// Common icon locations inside an app's project folder, most-preferred first.
const PROJECT_ICON_CANDIDATES: &[&str] = &[
    "src-tauri/icons/128x128@2x.png", // Tauri
    "src-tauri/icons/128x128.png",
    "src-tauri/icons/icon.png",
    "build/icon.png", // Electron
    "build/icon.ico",
    "public/logo.png", // web
    "public/icon.png",
    "public/favicon.png",
    "public/favicon.svg",
    "public/favicon.ico",
    "static/favicon.png",
    "static/favicon.ico",
    "assets/icon.png",
    "resources/icon.png",
    "icon.png",
    "logo.png",
    "favicon.png",
    "favicon.svg",
    "favicon.ico",
    "src-tauri/icons/icon.ico",
];

/// Directories to scan for an app's own icon: its cwd, plus a file:// url's folder.
fn app_dirs(entry: &AppEntry) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(cwd) = entry.cwd.as_deref().filter(|c| !c.is_empty()) {
        dirs.push(PathBuf::from(cwd));
    }
    if let Some(url) = &entry.url {
        if let Some(rest) = url.strip_prefix("file:///") {
            let decoded = rest.replace("%20", " ");
            if let Some(parent) = PathBuf::from(decoded).parent() {
                dirs.push(parent.to_path_buf());
            }
        }
    }
    dirs
}

/// Last-modified time of a file, if available.
fn file_mtime(p: &Path) -> Option<SystemTime> {
    std::fs::metadata(p).ok()?.modified().ok()
}

/// Append a unique query param so the WebView refetches a remote icon URL instead
/// of serving a cached (stale) copy. No-op unless `refresh` is set.
fn cache_bust(url: &str, refresh: bool) -> String {
    if !refresh {
        return url.to_string();
    }
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let sep = if url.contains('?') { '&' } else { '?' };
    format!("{url}{sep}mp_icon={nonce}")
}

/// The first existing candidate icon file inside `dir`.
fn find_project_icon(dir: &Path) -> Option<PathBuf> {
    PROJECT_ICON_CANDIDATES
        .iter()
        .map(|rel| dir.join(rel))
        .find(|p| p.is_file())
}

/// Locate a desktop app's built exe (even when it isn't running) for icon extraction.
fn located_exe(entry: &AppEntry) -> Option<PathBuf> {
    let cwd = entry.cwd.as_deref()?;
    let pn = entry.process_name.as_deref()?;
    let base = PathBuf::from(cwd);
    let ext = platform::exe_suffix();
    [
        format!("src-tauri/target/release/{pn}{ext}"),
        format!("src-tauri/target/debug/{pn}{ext}"),
        format!("target/release/{pn}{ext}"),
        format!("target/debug/{pn}{ext}"),
        format!("{pn}{ext}"),
    ]
    .iter()
    .map(|rel| base.join(rel))
    .find(|p| p.is_file())
}

/// The port to health-check for a LOCAL http(s) url (explicit, or the scheme default).
/// Returns None for non-local hosts (we can't meaningfully TCP-probe those here).
fn local_port(url: &str) -> Option<u16> {
    let (scheme, rest) = if let Some(r) = url.strip_prefix("http://") {
        ("http", r)
    } else {
        let r = url.strip_prefix("https://")?;
        ("https", r)
    };
    let authority = rest.split('/').next().unwrap_or(rest);
    let (host, port) = if authority.starts_with('[') {
        // Bracketed IPv6 authority (e.g. [::1] or [::1]:8080): the host is the
        // bracketed part, and any :port follows the closing ]. Splitting on the
        // last ':' naively would break inside the address.
        match authority.find(']') {
            Some(end) => {
                let host = &authority[..=end];
                let port = authority[end + 1..]
                    .strip_prefix(':')
                    .and_then(|p| p.parse::<u16>().ok());
                (host, port)
            }
            None => (authority, None),
        }
    } else {
        match authority.rsplit_once(':') {
            Some((h, p)) => (h, p.parse::<u16>().ok()),
            None => (authority, None),
        }
    };
    if matches!(host, "localhost" | "127.0.0.1" | "[::1]") {
        Some(port.unwrap_or(if scheme == "https" { 443 } else { 80 }))
    } else {
        None
    }
}

/// scheme://host[:port] for an http(s) URL; None for file:// etc.
fn http_origin(url: &str) -> Option<String> {
    let (scheme, rest) = if let Some(r) = url.strip_prefix("http://") {
        ("http", r)
    } else {
        let r = url.strip_prefix("https://")?;
        ("https", r)
    };
    let host = rest.split('/').next().unwrap_or(rest);
    if host.is_empty() {
        None
    } else {
        Some(format!("{scheme}://{host}"))
    }
}

/// Write the AI guide next to the manifest (refreshed each launch to track the app).
fn seed_ai_readme(app: &AppHandle) {
    if let Some(dir) = moonpool_dir(app) {
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::write(dir.join("AI-README.md"), AI_README);
    }
}

fn settings_path(app: &AppHandle) -> Option<PathBuf> {
    moonpool_dir(app).map(|d| d.join("settings.json"))
}

fn log_path(app: &AppHandle) -> Option<PathBuf> {
    moonpool_dir(app).map(|d| d.join("moonpool.log"))
}

fn load_settings(app: &AppHandle) -> Settings {
    settings_path(app)
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_default()
}

fn save_settings(app: &AppHandle, s: &Settings) -> Result<(), String> {
    let path = settings_path(app).ok_or("no config directory")?;
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let json = serde_json::to_string_pretty(s).map_err(|e| e.to_string())?;
    std::fs::write(path, json).map_err(|e| e.to_string())
}

/// Append a line to moonpool.log when debug logging is enabled.
pub(crate) fn log_line(app: &AppHandle, msg: &str) {
    let enabled = app
        .try_state::<HubState>()
        .map(|s| s.settings.lock().unwrap().debug_logging)
        .unwrap_or(false);
    if !enabled {
        return;
    }
    if let Some(path) = log_path(app) {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        if let Ok(mut f) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
        {
            let _ = writeln!(f, "[{ts}] {msg}");
        }
    }
}

fn parse_example() -> Vec<AppEntry> {
    serde_json::from_str(example_manifest()).unwrap_or_default()
}

/// Read the manifest from disk, seeding the example on first run and falling back
/// to it if the file is missing or malformed.
fn load_manifest(app: &AppHandle) -> Vec<AppEntry> {
    let Some(path) = manifest_path(app) else {
        return parse_example();
    };
    if !path.exists() {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(&path, example_manifest());
        log_line(
            app,
            &format!("seeded example manifest at {}", path.display()),
        );
    }
    // Read with retry: a dev launch can momentarily lock the file, and we must NOT
    // fall back to the example just because a transient read failed.
    let mut text = None;
    let mut last_err = String::new();
    for _ in 0..10 {
        match std::fs::read_to_string(&path) {
            Ok(t) => {
                text = Some(t);
                break;
            }
            Err(e) => {
                last_err = e.to_string();
                std::thread::sleep(Duration::from_millis(120));
            }
        }
    }
    match text {
        Some(t) => match serde_json::from_str::<Vec<AppEntry>>(&t) {
            Ok(v) => {
                log_line(
                    app,
                    &format!("loaded {} apps from {}", v.len(), path.display()),
                );
                v
            }
            Err(e) => {
                // Route through log_line (eprintln is invisible under
                // windows_subsystem = "windows").
                log_line(app, &format!("apps.json parse error ({e}); using example"));
                parse_example()
            }
        },
        None => {
            log_line(app, &format!("read failed ({last_err}); using example"));
            parse_example()
        }
    }
}

#[tauri::command]
fn get_apps(state: State<HubState>) -> Vec<AppEntry> {
    state.manifest.lock().unwrap().clone()
}

/// Re-read the manifest from disk into state and return it.
#[tauri::command]
fn reload_manifest(app: AppHandle, state: State<HubState>) -> Vec<AppEntry> {
    let m = load_manifest(&app);
    *state.manifest.lock().unwrap() = m.clone();
    m
}

/// Open the manifest file in the user's default editor.
#[tauri::command]
fn open_manifest(app: AppHandle) -> Result<(), String> {
    let path = manifest_path(&app).ok_or("no config directory")?;
    if !path.exists() {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(&path, example_manifest());
    }
    // On Linux the manifest's application/json MIME often has no handler and
    // xdg-open lands in a browser; open the text editor explicitly there.
    if platform::open_text_file(&path) {
        return Ok(());
    }
    app.opener()
        .open_path(path.to_string_lossy().to_string(), None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_settings(state: State<HubState>) -> Settings {
    state.settings.lock().unwrap().clone()
}

#[tauri::command]
fn set_debug_logging(enabled: bool, app: AppHandle, state: State<HubState>) -> Result<(), String> {
    let s = {
        let mut g = state.settings.lock().unwrap();
        g.debug_logging = enabled;
        g.clone()
    };
    save_settings(&app, &s)?;
    if enabled {
        log_line(&app, "--- debug logging enabled ---");
    }
    Ok(())
}

#[tauri::command]
fn set_close_to_tray(enabled: bool, app: AppHandle, state: State<HubState>) -> Result<(), String> {
    let s = {
        let mut g = state.settings.lock().unwrap();
        g.close_to_tray = enabled;
        g.clone()
    };
    save_settings(&app, &s)
}

#[tauri::command]
fn set_minimize_to_tray(
    enabled: bool,
    app: AppHandle,
    state: State<HubState>,
) -> Result<(), String> {
    let s = {
        let mut g = state.settings.lock().unwrap();
        g.minimize_to_tray = enabled;
        g.clone()
    };
    save_settings(&app, &s)
}

#[tauri::command]
fn set_check_on_startup(
    enabled: bool,
    app: AppHandle,
    state: State<HubState>,
) -> Result<(), String> {
    let s = {
        let mut g = state.settings.lock().unwrap();
        g.check_on_startup = enabled;
        g.clone()
    };
    save_settings(&app, &s)
}

/// Every window this app opens. Always-on-top is applied to all of them so the
/// detached Settings/About windows stay in the same z-band as the hub; otherwise
/// turning the setting on sinks the Settings window (the one you're using) behind
/// the hub, where it's hard to move or close.
const ALL_WINDOWS: [&str; 4] = ["main", "settings", "about", "installer"];

fn apply_always_on_top(app: &AppHandle, on: bool) {
    for label in ALL_WINDOWS {
        if let Some(w) = app.get_webview_window(label) {
            let _ = w.set_always_on_top(on);
        }
    }
}

#[tauri::command]
fn set_always_on_top(enabled: bool, app: AppHandle, state: State<HubState>) -> Result<(), String> {
    let s = {
        let mut g = state.settings.lock().unwrap();
        g.always_on_top = enabled;
        g.clone()
    };
    apply_always_on_top(&app, enabled);
    save_settings(&app, &s)
}

/// Change the UI language.
///
/// `locale` is the user's choice and may be "auto"; `resolved` is the concrete
/// tag the frontend picked for it. Both are persisted: the choice so the picker
/// still reads "Auto", the resolved tag so `setup()` can label the tray in the
/// right language on the next launch, before a webview exists to ask.
#[tauri::command]
fn set_locale(
    locale: String,
    resolved: String,
    app: AppHandle,
    state: State<HubState>,
) -> Result<(), String> {
    let s = {
        let mut g = state.settings.lock().unwrap();
        g.locale = locale;
        g.locale_resolved = resolved.clone();
        g.clone()
    };
    retranslate_tray(&app, &resolved);
    save_settings(&app, &s)
}

#[tauri::command]
fn set_transparency(value: u8, app: AppHandle, state: State<HubState>) -> Result<(), String> {
    let s = {
        let mut g = state.settings.lock().unwrap();
        g.transparency = value.min(90);
        g.clone()
    };
    save_settings(&app, &s)
}

/// Persist the Ctrl+wheel zoom factor. Clamped here as well as in the webview,
/// since the value is read back at launch and applied without further checking.
#[tauri::command]
fn set_ui_scale(scale: f64, app: AppHandle, state: State<HubState>) -> Result<(), String> {
    let s = {
        let mut g = state.settings.lock().unwrap();
        g.ui_scale = scale.clamp(0.5, 3.0);
        g.clone()
    };
    save_settings(&app, &s)
}

/// Open the log file in the default app.
#[tauri::command]
fn open_log(app: AppHandle) -> Result<(), String> {
    let path = log_path(&app).ok_or("no config directory")?;
    if !path.exists() {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let _ = std::fs::write(&path, "");
    }
    app.opener()
        .open_path(path.to_string_lossy().to_string(), None::<&str>)
        .map_err(|e| e.to_string())
}

/// Absolute path of Moonpool's config directory (holds apps.json + AI-README.md).
#[tauri::command]
fn manifest_dir(app: AppHandle) -> Result<String, String> {
    moonpool_dir(&app)
        .map(|d| d.to_string_lossy().to_string())
        .ok_or_else(|| "no config directory".to_string())
}

/// Resolve an app's icon to an <img> src: manifest icon -> icons/<id>.* ->
/// desktop exe icon -> web favicon -> None (frontend shows a glyph).
#[tauri::command]
fn app_icon(
    id: String,
    refresh: Option<bool>,
    app: AppHandle,
    state: State<HubState>,
) -> Option<String> {
    // `refresh` forces past the exe mtime-cache and the WebView's favicon cache,
    // so an icon changed in a new build of the target app shows up immediately.
    let refresh = refresh.unwrap_or(false);
    let mut entry = {
        let m = state.manifest.lock().unwrap();
        m.iter().find(|e| e.id == id).cloned()
    }?;
    // Resolve {MP_HOME}/{MP_DATA} tokens and ./-anchored paths so icon lookup (cwd,
    // file:// url, explicit icon path) works for portable bundles.
    entry.cwd = entry
        .cwd
        .as_deref()
        .map(|c| portable::resolve_path(c, &app));
    entry.url = entry
        .url
        .as_deref()
        .map(|u| portable::resolve_tokens(u, &app));
    entry.icon = entry
        .icon
        .as_deref()
        .map(|i| portable::resolve_path(i, &app));

    // `id` is used as a filename below (icons/<id>.<ext> and moonpool-icon-<id>.png).
    // A hand-edited apps.json could set id to something like `..\..\x`; refuse to
    // build a cache path from an id carrying a path separator or `..`.
    let id_is_safe_filename = !(id.contains('/') || id.contains('\\') || id.contains(".."));

    // 1. Explicit icon in the manifest (url / data URI passed through; path -> data URI).
    if let Some(icon) = entry
        .icon
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        if icon.starts_with("data:") {
            return Some(icon.to_string());
        }
        if icon.starts_with("http://") || icon.starts_with("https://") {
            return Some(cache_bust(icon, refresh));
        }
        let p = PathBuf::from(icon);
        if p.is_file() {
            if let Some(u) = file_to_data_uri(&p) {
                return Some(u);
            }
        }
    }

    // 2. icons/<id>.<ext> in the config dir.
    if id_is_safe_filename {
        if let Some(dir) = icons_dir(&app) {
            for ext in ["png", "ico", "svg", "jpg", "jpeg", "webp"] {
                let p = dir.join(format!("{id}.{ext}"));
                if p.is_file() {
                    if let Some(u) = file_to_data_uri(&p) {
                        return Some(u);
                    }
                }
            }
        }
    }

    // 3. The app's OWN icon in its project folder (Tauri/Electron/web) - proactive,
    // works without the app running and without any config.
    for dir in app_dirs(&entry) {
        if let Some(p) = find_project_icon(&dir) {
            if let Some(u) = file_to_data_uri(&p) {
                return Some(u);
            }
        }
    }

    // 4. Desktop: extract from the built exe (cheap on-disk lookup) or, failing that,
    // the running process (a full process scan, so tried last). Cached in temp keyed
    // by the exe's mtime so we don't re-spawn PowerShell on every fetch.
    if entry.app_type == "desktop" {
        let exe = located_exe(&entry)
            .or_else(|| entry.process_name.as_deref().and_then(running_exe_path));
        if let (true, Some(exe)) = (id_is_safe_filename, exe) {
            let out = std::env::temp_dir().join(format!("moonpool-icon-{id}.png"));
            let cached_fresh = !refresh
                && file_mtime(&out)
                    .zip(file_mtime(&exe))
                    .map(|(o, e)| o >= e)
                    .unwrap_or(false);
            if (cached_fresh || platform::extract_exe_icon(&exe, &out)) && out.is_file() {
                if let Some(u) = file_to_data_uri(&out) {
                    return Some(u);
                }
            }
        }
    }

    // 5. Web/static: the site's favicon, loaded live by the WebView, only when the
    // server is actually up (so it doesn't flash a broken image).
    if matches!(entry.app_type.as_str(), "web" | "static") {
        if let Some(url) = entry.url.as_deref() {
            if let Some(origin) = http_origin(url) {
                // Verify liveness for a local port (explicit or derived); let remote
                // hosts through (the WebView loads it, falling back to a glyph on 404).
                let up = match entry.port.or_else(|| local_port(url)) {
                    Some(p) => tcp_alive(p),
                    None => true,
                };
                if up {
                    return Some(cache_bust(&format!("{origin}/favicon.ico"), refresh));
                }
            }
        }
    }

    None
}

/// Write the full manifest to disk (used by the in-app editor).
#[tauri::command]
fn save_manifest(
    app: AppHandle,
    entries: Vec<AppEntry>,
    state: State<HubState>,
) -> Result<(), String> {
    let path = manifest_path(&app).ok_or("no config directory")?;
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let json = serde_json::to_string_pretty(&entries).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;
    *state.manifest.lock().unwrap() = entries;
    Ok(())
}

#[tauri::command]
fn launch_app(
    id: String,
    cols: u16,
    rows: u16,
    app: AppHandle,
    state: State<HubState>,
) -> Result<(), String> {
    let entry = {
        let m = state.manifest.lock().unwrap();
        m.iter().find(|e| e.id == id).cloned()
    }
    .ok_or_else(|| format!("unknown app: {id}"))?;

    {
        let apps = state.apps.lock().unwrap();
        if apps.contains_key(&id) {
            return Err("already running".into());
        }
    }

    // Expand {MP_HOME}/{MP_DATA} tokens and anchor ./ paths so a portable bundle's
    // entries resolve against the folder rather than the process cwd.
    let command = entry
        .command
        .as_deref()
        .map(|c| portable::resolve_tokens(c, &app))
        .ok_or_else(|| "app has no launch command".to_string())?;
    let cwd = entry
        .cwd
        .as_deref()
        .map(|c| portable::resolve_path(c, &app))
        .unwrap_or_else(|| ".".to_string());

    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| e.to_string())?;

    // Run every launch line through the platform shell (cmd /c on Windows, sh -c
    // on Unix) so any command works with a single code path.
    let mut cmd = platform::shell_command(&command);
    cmd.cwd(&cwd);
    if let Some(env) = &entry.env {
        for (k, v) in env {
            cmd.env(k, v);
        }
    }

    let mut child = pair.slave.spawn_command(cmd).map_err(|e| e.to_string())?;
    let mut reader = pair.master.try_clone_reader().map_err(|e| e.to_string())?;
    let writer = pair.master.take_writer().map_err(|e| e.to_string())?;
    let stop = Arc::new(AtomicBool::new(false));

    // Reserve the slot under the lock BEFORE spawning the reader thread. The
    // "already running" guard above dropped the lock, so a concurrent launch of
    // the same id could reach here too. Re-check with the Entry API: if the id is
    // already present, tear down the app we just spawned (stop flag + kill) and
    // return, instead of overwriting/leaking the existing RunningApp. Spawning the
    // reader thread only after we win the slot also keeps a losing launch's reader
    // from later removing the winner's entry on cleanup.
    {
        use std::collections::hash_map::Entry;
        let mut apps = state.apps.lock().unwrap();
        match apps.entry(id.clone()) {
            Entry::Occupied(_) => {
                drop(apps);
                stop.store(true, Ordering::Relaxed);
                let _ = child.kill();
                return Err("already running".into());
            }
            Entry::Vacant(slot) => {
                slot.insert(RunningApp {
                    writer,
                    master: pair.master,
                    child,
                    stop: stop.clone(),
                });
            }
        }
    }

    // A fresh run starts a fresh log so `dump` never mixes two runs' output.
    state.term_logs.lock().unwrap().insert(id.clone(), Vec::new());

    log_line(&app, &format!("launch {id}: {command} (cwd {cwd})"));

    // Reader thread: pump PTY bytes to the frontend until EOF/stop, then clean up.
    let app2 = app.clone();
    let id2 = id.clone();
    let stop2 = stop.clone();
    std::thread::spawn(move || {
        let mut buf = [0u8; 8192];
        loop {
            if stop2.load(Ordering::Relaxed) {
                break;
            }
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    if let Some(state) = app2.try_state::<HubState>() {
                        let mut logs = state.term_logs.lock().unwrap();
                        let buf_for_id = logs.entry(id2.clone()).or_default();
                        buf_for_id.extend_from_slice(&buf[..n]);
                        // Trim from the front so the ring keeps the newest output.
                        if buf_for_id.len() > MAX_TERM_LOG {
                            let cut = buf_for_id.len() - MAX_TERM_LOG;
                            buf_for_id.drain(0..cut);
                        }
                    }
                    let _ = app2.emit(
                        "term://output",
                        TermOutput {
                            id: id2.clone(),
                            data: base64::engine::general_purpose::STANDARD.encode(&buf[..n]),
                        },
                    );
                }
                Err(_) => break,
            }
        }
        // Drop the running entry so status reflects the exit.
        if let Some(state) = app2.try_state::<HubState>() {
            if let Some(mut a) = state.apps.lock().unwrap().remove(&id2) {
                let _ = a.child.kill();
            }
        }
        let _ = app2.emit("term://exit", id2.clone());
    });

    Ok(())
}

#[tauri::command]
fn term_input(id: String, data: String, state: State<HubState>) -> Result<(), String> {
    let mut apps = state.apps.lock().unwrap();
    if let Some(a) = apps.get_mut(&id) {
        a.writer
            .write_all(data.as_bytes())
            .map_err(|e| e.to_string())?;
        let _ = a.writer.flush();
    }
    Ok(())
}

#[tauri::command]
fn term_resize(id: String, cols: u16, rows: u16, state: State<HubState>) -> Result<(), String> {
    let apps = state.apps.lock().unwrap();
    if let Some(a) = apps.get(&id) {
        a.master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn stop_app(id: String, state: State<HubState>) -> Result<(), String> {
    let entry = {
        let m = state.manifest.lock().unwrap();
        m.iter().find(|e| e.id == id).cloned()
    };

    // Kill the PTY subtree we own. child.kill() ends cmd.exe, but the launch
    // scripts spawn deep trees (cmd -> pwsh -> npm -> node/cargo -> app), so
    // tree-kill the whole subtree by PID.
    if let Some(mut a) = state.apps.lock().unwrap().remove(&id) {
        a.stop.store(true, Ordering::Relaxed);
        let pid = a.child.process_id();
        let _ = a.child.kill();
        if let Some(pid) = pid {
            platform::kill_tree(pid);
        }
    }

    if let Some(e) = entry {
        match e.app_type.as_str() {
            // A desktop app may outlive the PTY tree (the window detaches from the
            // dev server), so also kill it by name.
            "desktop" => {
                if let Some(pn) = &e.process_name {
                    platform::kill_by_name(pn);
                }
            }
            // A web server subprocess can linger holding the port; free it.
            "web" => {
                if let Some(port) = e.port {
                    platform::free_port(port);
                }
            }
            _ => {}
        }
    }
    Ok(())
}

#[tauri::command]
fn open_url(app: AppHandle, url: String) -> Result<(), String> {
    let url = portable::resolve_tokens(&url, &app);
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn tcp_alive(port: u16) -> bool {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), port);
    TcpStream::connect_timeout(&addr, Duration::from_millis(250)).is_ok()
}

fn process_running(sys: &System, target: &str) -> bool {
    sys.processes()
        .values()
        .any(|p| matches_process_name(p, target))
}

/// Background poller: emits `status://update` every ~2s.
fn spawn_status_poller(app: AppHandle) {
    std::thread::spawn(move || {
        let mut sys = System::new();
        // Apps we've already auto-opened a browser for this run (once per launch).
        let mut opened: HashSet<String> = HashSet::new();
        loop {
            let entries = {
                match app.try_state::<HubState>() {
                    Some(s) => s.manifest.lock().unwrap().clone(),
                    None => break,
                }
            };
            sys.refresh_processes();

            // Reap exited PTY children. The reader thread clears the map on EOF, but a
            // lingering grandchild can hold the PTY open so EOF never fires - which would
            // leave the app stuck "managed" (permanent amber). try_wait() on the top
            // process is the reliable liveness signal.
            let managed: HashSet<String> = match app.try_state::<HubState>() {
                Some(state) => {
                    let mut apps = state.apps.lock().unwrap();
                    let dead: Vec<String> = apps
                        .iter_mut()
                        .filter_map(|(id, a)| match a.child.try_wait() {
                            Ok(Some(_)) => {
                                a.stop.store(true, Ordering::Relaxed);
                                Some(id.clone())
                            }
                            _ => None,
                        })
                        .collect();
                    for id in &dead {
                        apps.remove(id);
                        let _ = app.emit("term://exit", id.clone());
                    }
                    apps.keys().cloned().collect()
                }
                None => break,
            };

            let mut statuses = Vec::with_capacity(entries.len());
            for e in &entries {
                let is_managed = managed.contains(&e.id);
                let mut detected = false;
                if let Some(pn) = &e.process_name {
                    detected = process_running(&sys, pn);
                }
                if !detected {
                    if let Some(port) = e.port {
                        detected = tcp_alive(port);
                    }
                }
                // A CLI/shell tab has neither port nor process marker; treat the
                // owned PTY as its "running" signal.
                let running =
                    detected || (is_managed && e.port.is_none() && e.process_name.is_none());

                // Open the browser once, when a web app we launched first goes green.
                if detected && is_managed && e.open_browser && !opened.contains(&e.id) {
                    if let Some(url) = &e.url {
                        let url = portable::resolve_tokens(url, &app);
                        let _ = app.opener().open_url(url, None::<&str>);
                        opened.insert(e.id.clone());
                    }
                }
                if !detected {
                    opened.remove(&e.id);
                }

                statuses.push(AppStatus {
                    id: e.id.clone(),
                    running,
                    managed: is_managed,
                });
            }
            if let Some(state) = app.try_state::<HubState>() {
                *state.last_statuses.lock().unwrap() = statuses.clone();
            }
            write_state(&app);
            let _ = app.emit("status://update", statuses);
            std::thread::sleep(Duration::from_millis(2000));
        }
    });
}

// ---------------------------------------------------------------------------
// External control channel (single-instance argv forwarding)
// ---------------------------------------------------------------------------

/// A command forwarded to the UI, mirroring a user action so the terminal tab,
/// icons, and status all stay consistent. `arg` is the app id where relevant.
#[derive(Clone, serde::Serialize)]
struct ControlCommand {
    action: String,
    arg: Option<String>,
    /// Caller-supplied correlation key. When present, the UI reports the command's
    /// outcome back under this key in state.json (see `report_outcome`).
    ticket: Option<String>,
}

/// Handle a second-instance invocation. `argv[0]` is the exe path; the rest is
/// the command, e.g. `moonpool.exe launch my-app`.
/// Block until the process `pid` has exited (or a ~10s safety timeout). Used by the
/// `--wait-pid` relaunch handshake so a freshly-spawned copy doesn't collide with the
/// single-instance lock still held by the process that spawned it.
fn wait_for_pid_exit(pid: u32) {
    use sysinfo::{Pid, System};
    let target = Pid::from_u32(pid);
    let mut sys = System::new();
    for _ in 0..200 {
        if !sys.refresh_process(target) {
            return; // process is gone
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

fn dispatch_control(app: &AppHandle, argv: &[String]) {
    // Pull an optional `--ticket <key>` correlation flag out of the args; the rest
    // are positional (action + app id) exactly as before.
    let raw: Vec<&str> = argv.iter().skip(1).map(String::as_str).collect();
    let mut positional: Vec<&str> = Vec::new();
    let mut ticket: Option<String> = None;
    let mut i = 0;
    while i < raw.len() {
        if raw[i] == "--ticket" {
            ticket = raw.get(i + 1).map(|s| s.to_string());
            i += 2;
        } else {
            positional.push(raw[i]);
            i += 1;
        }
    }

    let action = match positional.first() {
        Some(a) => a.to_lowercase(),
        None => {
            // A bare second launch just means "come back" - surface the window.
            show_main(app);
            return;
        }
    };
    let arg = positional.get(1).map(|s| s.to_string());

    // `show` acts on the window directly; everything else is handled by the UI,
    // which owns launching/stopping/reloading/icon refresh.
    if action == "show" {
        show_main(app);
        return;
    }
    // `dump` is answered entirely here: the output ring lives in Rust, so there is
    // no need to round-trip through the UI. Writes the app's recent console output
    // to a file and reports the path back through the ticket detail.
    if action == "dump" {
        let (status, detail) = dump_term_log(app, arg.as_deref(), &positional);
        log_line(app, &format!("control: dump -> {status}: {detail}"));
        if let Some(t) = &ticket {
            record_ticket(app, t, &action, arg.clone(), status, Some(detail));
        }
        return;
    }
    // `paths` is answered entirely here, like `dump`: report every filesystem
    // location the *hub* process resolves (config dir, apps.json, state.json, log,
    // dumps, icons) plus whether it is portable and which exe is running. Lets an
    // agent confirm exactly which apps.json this resident instance reads, instead
    // of guessing when an edit "doesn't take".
    if action == "paths" {
        let detail = hub_paths_report(app);
        log_line(app, "control: paths");
        if let Some(t) = &ticket {
            record_ticket(app, t, &action, None, "ok", Some(detail));
        }
        return;
    }
    if !matches!(
        action.as_str(),
        "launch" | "stop" | "restart" | "reload" | "refresh-icons"
    ) {
        log_line(
            app,
            &format!("control: ignoring unknown command '{action}'"),
        );
        if let Some(t) = &ticket {
            record_ticket(
                app,
                t,
                &action,
                arg.clone(),
                "error",
                Some("unknown command".into()),
            );
        }
        return;
    }
    log_line(
        app,
        &format!(
            "control: {action}{}",
            arg.as_deref().map(|a| format!(" {a}")).unwrap_or_default()
        ),
    );
    // Record a pending outcome up front so a caller polling state.json sees the
    // command was received even before the UI finishes acting on it.
    if let Some(t) = &ticket {
        record_ticket(app, t, &action, arg.clone(), "pending", None);
    }
    let _ = app.emit(
        "control://command",
        ControlCommand {
            action,
            arg,
            ticket,
        },
    );
}

/// Strip ANSI/VT escape sequences so a dumped log reads as plain text. Handles
/// CSI (`ESC [ ... final`), OSC (`ESC ] ... BEL or ST`) and short two-byte
/// escapes; anything else is passed through.
fn strip_ansi(bytes: &[u8]) -> String {
    let text = String::from_utf8_lossy(bytes);
    let src: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(src.len());
    let mut i = 0;
    while i < src.len() {
        if src[i] != '\u{1b}' {
            // Drop carriage returns: the PTY pairs one with every newline, and
            // progress redraws would otherwise leave garbled overwritten lines.
            if src[i] != '\r' {
                out.push(src[i]);
            }
            i += 1;
            continue;
        }
        match src.get(i + 1) {
            Some('[') => {
                i += 2;
                while i < src.len() && !matches!(src[i], '@'..='~') {
                    i += 1;
                }
                i += 1;
            }
            Some(']') => {
                i += 2;
                while i < src.len() {
                    if src[i] == '\u{7}' {
                        i += 1;
                        break;
                    }
                    if src[i] == '\u{1b}' && src.get(i + 1) == Some(&'\\') {
                        i += 2;
                        break;
                    }
                    i += 1;
                }
            }
            Some(_) => i += 2,
            None => i += 1,
        }
    }
    out
}

/// Back the `dump <app-id> [out-path]` control command: write the retained PTY
/// output for `id` to a file and return (status, detail) for the ticket. Default
/// destination is `<moonpool_dir>/dumps/<id>.log`.
fn dump_term_log(app: &AppHandle, id: Option<&str>, positional: &[&str]) -> (&'static str, String) {
    let Some(id) = id else {
        return ("error", "dump needs an app id".into());
    };
    let Some(bytes) = app
        .try_state::<HubState>()
        .and_then(|s| s.term_logs.lock().unwrap().get(id).cloned())
    else {
        return (
            "error",
            format!("no console output recorded for '{id}' (not launched this session)"),
        );
    };
    let text = strip_ansi(&bytes);
    let path = match positional.get(2) {
        Some(p) => PathBuf::from(p),
        None => {
            let Some(dir) = moonpool_dir(app).map(|d| d.join("dumps")) else {
                return ("error", "no config directory".into());
            };
            if let Err(e) = std::fs::create_dir_all(&dir) {
                return ("error", format!("cannot create {}: {e}", dir.display()));
            }
            // Keep the file name filesystem-safe whatever the id looks like.
            let safe: String = id
                .chars()
                .map(|c| {
                    if c.is_alphanumeric() || c == '-' || c == '_' {
                        c
                    } else {
                        '_'
                    }
                })
                .collect();
            dir.join(format!("{safe}.log"))
        }
    };
    match std::fs::write(&path, &text) {
        Ok(()) => ("ok", path.display().to_string()),
        Err(e) => ("error", format!("cannot write {}: {e}", path.display())),
    }
}

/// Milliseconds since the Unix epoch (0 if the clock is somehow before it).
fn now_millis() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// Write a machine-readable snapshot (apps + live status + recent control-command
/// outcomes) to <config>/Moonpool/state.json. Refreshed every status tick and
/// again immediately whenever a ticket's outcome changes, so a script/agent can
/// read back what's registered, what's running, and whether its tagged command
/// succeeded - all without a stdout reply channel.
fn write_state(app: &AppHandle) {
    let Some(path) = moonpool_dir(app).map(|d| d.join("state.json")) else {
        return;
    };
    let Some(state) = app.try_state::<HubState>() else {
        return;
    };
    let entries = state.manifest.lock().unwrap().clone();
    let statuses = state.last_statuses.lock().unwrap().clone();
    let tickets = state.tickets.lock().unwrap().clone();
    let snapshot = serde_json::json!({
        "apps": entries,
        "statuses": statuses,
        "tickets": tickets,
    });
    if let Ok(text) = serde_json::to_string_pretty(&snapshot) {
        let _ = std::fs::write(path, text);
    }
}

/// Insert or update a ticket record, then flush state.json so a poller on the
/// caller's side sees the change within one read rather than waiting ~2s.
/// A newline-separated list of every filesystem path the resident (hub) process
/// resolves. Answered by the `paths` control command so an agent can confirm which
/// apps.json this instance actually reads/writes - the MCP `moonpool_paths` tool
/// pairs this with the paths the MCP process resolves so a mismatch is obvious.
fn hub_paths_report(app: &AppHandle) -> String {
    let dir = moonpool_dir(app);
    let show = |p: Option<PathBuf>| {
        p.map(|d| d.display().to_string())
            .unwrap_or_else(|| "<unresolved>".into())
    };
    let sub = |name: &str| show(dir.clone().map(|d| d.join(name)));
    format!(
        "hub config dir: {}\n\
         hub apps.json:  {}\n\
         hub state.json: {}\n\
         hub log:        {}\n\
         hub dumps dir:  {}\n\
         hub icons dir:  {}\n\
         hub portable:   {}\n\
         hub exe:        {}",
        show(dir.clone()),
        sub("apps.json"),
        sub("state.json"),
        sub("moonpool.log"),
        sub("dumps"),
        sub("icons"),
        portable::is_portable(),
        std::env::current_exe()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "<unknown>".into()),
    )
}

fn record_ticket(
    app: &AppHandle,
    ticket: &str,
    action: &str,
    arg: Option<String>,
    status: &str,
    detail: Option<String>,
) {
    if let Some(state) = app.try_state::<HubState>() {
        let mut tickets = state.tickets.lock().unwrap();
        let rec = Ticket {
            ticket: ticket.to_string(),
            action: action.to_string(),
            arg,
            status: status.to_string(),
            detail,
            ts: now_millis(),
        };
        match tickets.iter_mut().find(|t| t.ticket == ticket) {
            Some(existing) => *existing = rec,
            None => {
                tickets.push(rec);
                let overflow = tickets.len().saturating_sub(MAX_TICKETS);
                if overflow > 0 {
                    tickets.drain(0..overflow);
                }
            }
        }
    }
    write_state(app);
}

/// Called by the UI once it has acted on a ticketed control command, reporting
/// the final outcome ("ok" / "error") so it lands in state.json for the caller.
#[tauri::command]
fn report_outcome(
    ticket: String,
    action: String,
    arg: Option<String>,
    status: String,
    detail: Option<String>,
    app: AppHandle,
) {
    record_ticket(&app, &ticket, &action, arg, &status, detail);
}

// ---------------------------------------------------------------------------
// Tray
// ---------------------------------------------------------------------------

fn show_main(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.show();
        let _ = win.unminimize();
        // If the window was hidden while collapsed to the borderless minimize
        // sliver (~215x26), showing it would bring back that sliver. Re-assert the
        // last good size so Show always restores a usable window.
        if let Ok(sz) = win.inner_size() {
            if sz.width < 300 || sz.height < 300 {
                let last = app
                    .try_state::<HubState>()
                    .and_then(|s| *s.last_good_size.lock().unwrap());
                if let Some((w, h)) = last {
                    let _ = win.set_size(tauri::PhysicalSize::new(w, h));
                    // The collapsed sliver also sits at an off-screen minimize
                    // position, so a resize alone shows it off-screen (taskbar
                    // button, no visible window). Re-center to bring it on-screen.
                    let _ = win.center();
                }
            }
        }
        let _ = win.set_focus();
    }
}

/// Build the tray menu in the given locale. Split out from `build_tray` because
/// a language change has to rebuild the menu in place - the tray outlives every
/// window, so it cannot just re-render.
fn tray_menu(app: &AppHandle, locale: &str) -> tauri::Result<Menu<tauri::Wry>> {
    let s = i18n::tray_strings(locale);
    let show = MenuItem::with_id(app, "show", s.show, true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", s.quit, true, None::<&str>)?;
    Menu::with_items(app, &[&show, &quit])
}

/// Swap the tray menu to a new language. No-op before the tray exists.
fn retranslate_tray(app: &AppHandle, locale: &str) {
    if let (Some(tray), Ok(menu)) = (app.tray_by_id(TRAY_ID), tray_menu(app, locale)) {
        let _ = tray.set_menu(Some(menu));
    }
}

const TRAY_ID: &str = "moonpool-tray";

fn build_tray(app: &AppHandle, locale: &str) -> tauri::Result<()> {
    let menu = tray_menu(app, locale)?;

    let mut builder = TrayIconBuilder::with_id(TRAY_ID)
        .tooltip("Moonpool")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => show_main(app),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                show_main(tray.app_handle());
            }
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    builder.build(app)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // MCP server entry point (`moonpool.exe mcp`, run by an agent client over
    // stdio). Must come before the Tauri builder: the single-instance plugin would
    // otherwise forward this argv to the resident window and exit, leaving the
    // client talking to a process that is gone.
    if std::env::args().nth(1).as_deref() == Some("mcp") {
        mcp::serve();
        return;
    }

    // Uninstall entry point (Add/Remove Programs calls `moonpool.exe --uninstall`).
    // Handled before any window/tray so it's a clean, headless teardown.
    if std::env::args().any(|a| a == "--uninstall") {
        install::run_uninstall();
        return;
    }

    // A relaunch we spawned (install / portable) passes `--wait-pid <pid>`: wait for
    // that parent (the installer or previous instance) to fully exit BEFORE we build
    // anything. Otherwise the single-instance plugin below routes us straight back
    // into the still-alive parent - which, for a bare relaunch, just re-surfaces the
    // parent's window (the installer) instead of letting this fresh copy boot the hub.
    {
        let args: Vec<String> = std::env::args().collect();
        if let Some(i) = args.iter().position(|a| a == "--wait-pid") {
            if let Some(pid) = args.get(i + 1).and_then(|s| s.parse::<u32>().ok()) {
                wait_for_pid_exit(pid);
            }
        }
    }

    // Remove a leftover `moonpool.old` from a prior self-update.
    update::cleanup_old();

    // Portable: keep WebView2's browser profile inside the bundle too. WebView2 reads
    // WEBVIEW2_USER_DATA_FOLDER when it creates its environment, so this must be set
    // before any window is built. Otherwise it defaults to
    // %LOCALAPPDATA%\<identifier>\EBWebView - the last thing that would land outside a
    // portable folder.
    if let Some(dir) = portable::webview_data_dir() {
        let _ = std::fs::create_dir_all(&dir);
        std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &dir);
    }

    tauri::Builder::default()
        // Must be the FIRST plugin. A second run of the exe (Moonpool is already
        // resident in the tray) forwards its args here instead of starting anew;
        // this is the external control channel used by scripts/agents.
        .plugin(tauri_plugin_single_instance::init(|app, argv, _cwd| {
            dispatch_control(app, &argv);
        }))
        // Window size/position persistence is hand-rolled (see `winstate`): restored
        // in setup, saved eagerly on Resized/Moved below. We roll our own instead of
        // tauri-plugin-window-state so the state file lives in `moonpool_dir` (next to
        // apps.json) in both modes - nothing leaks outside a portable bundle, and there
        // is no stray %APPDATA%\<id> directory. Installer mode never touches it (the
        // window is the install-card size and setup/save are both skipped there).
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(HubState {
            apps: Mutex::new(HashMap::new()),
            manifest: Mutex::new(Vec::new()),
            settings: Mutex::new(Settings::default()),
            tickets: Mutex::new(Vec::new()),
            last_statuses: Mutex::new(Vec::new()),
            term_logs: Mutex::new(HashMap::new()),
            last_good_size: Mutex::new(None),
        })
        .setup(|app| {
            let handle = app.handle().clone();
            // First-run install mode: the portable exe is running from outside the
            // install dir. Show only the install card (the frontend routes to it via
            // setup_state) - no tray, no status poller, and shrink the window to the
            // card size. The hub proper boots on the next launch from the install dir.
            if install::needs_setup() {
                if let Some(win) = handle.get_webview_window("main") {
                    let _ = win.set_decorations(false);
                    let _ = win.set_resizable(false);
                    let _ = win.set_size(tauri::LogicalSize::new(452.0, 432.0));
                    let _ = win.center();
                    let _ = win.show();
                }
                return Ok(());
            }
            // Load settings first so logging (if enabled) captures the manifest load.
            let settings = load_settings(&handle);
            let always_on_top = settings.always_on_top;
            let locale = settings.locale_resolved.clone();
            *handle.state::<HubState>().settings.lock().unwrap() = settings;
            log_line(&handle, "=== Moonpool starting ===");
            // Load the user-editable manifest (seeded from the example on first run).
            let manifest = load_manifest(&handle);
            *handle.state::<HubState>().manifest.lock().unwrap() = manifest;
            seed_ai_readme(&handle);
            // Write the embedded example dashboards to {MP_HOME}/dashboards on first
            // run (skips files that already exist, so user edits are preserved).
            dashboards::seed(&handle);
            build_tray(&handle, &locale)?;
            // Belt-and-braces: strip the native title bar on the main window even if
            // the config value didn't take (the frontend draws its own title bar).
            if let Some(win) = handle.get_webview_window("main") {
                let _ = win.set_decorations(false);
                let _ = win.set_always_on_top(always_on_top);
                // Restore the saved geometry (no-op -> config default on a missing or
                // corrupt file). Do this before seeding last-good size below.
                winstate::restore(&win);
                // Seed the last-good size from the initial (restored) window size so
                // a Win+D before any user resize still has something to re-assert to
                // (otherwise a fresh-launch minimize leaves the collapsed sliver).
                if let Ok(sz) = win.inner_size() {
                    if sz.width >= 300 && sz.height >= 300 {
                        if let Some(s) = handle.try_state::<HubState>() {
                            *s.last_good_size.lock().unwrap() = Some((sz.width, sz.height));
                        }
                    }
                }
            }
            spawn_status_poller(handle);
            Ok(())
        })
        .on_window_event(|window, event| {
            // Installer mode has no tray: the minimize-to-tray / hide handlers below
            // would hide the window with no way to bring it back (Win+M makes it
            // vanish). Skip all of them and let the OS handle minimize normally.
            if install::needs_setup() {
                return;
            }
            let (close_to_tray, minimize_to_tray) = window
                .app_handle()
                .try_state::<HubState>()
                .map(|s| {
                    let g = s.settings.lock().unwrap();
                    (g.close_to_tray, g.minimize_to_tray)
                })
                .unwrap_or((true, true));
            // Trace window geometry events to diagnose the borderless minimize-all
            // collapse (window restoring to a title-bar-height sliver). No-op unless
            // Debug logging is on.
            if window.label() == "main" {
                if let tauri::WindowEvent::Resized(sz) = event {
                    let mini = window.is_minimized().unwrap_or(false);
                    let scale = window.scale_factor().unwrap_or(1.0);
                    log_line(
                        window.app_handle(),
                        &format!(
                            "win Resized: {}x{} phys (scale {:.2}), minimized={}",
                            sz.width, sz.height, scale, mini
                        ),
                    );
                }
            }
            match event {
                // With "close to tray" on, closing hides to the tray instead of
                // quitting (Quit in the tray menu is the only real exit, so
                // Moonpool stays resident). With it off, the close proceeds and
                // the app exits normally.
                tauri::WindowEvent::CloseRequested { api, .. }
                    if close_to_tray && window.label() == "main" =>
                {
                    let _ = window.hide();
                    api.prevent_close();
                }
                // With "minimize to tray" on, minimizing hides to the tray too,
                // so Moonpool leaves the taskbar. Tauri has no Minimized event, so
                // detect it on Resized. But a RESTORE animation also fires transient
                // minimized=true frames (verified via trace) - hiding on those made
                // the window animate back full and then vanish. So debounce: wait a
                // beat and only hide if it's STILL minimized (a real minimize stays
                // that way; a restore's stray frame is immediately followed by a
                // full-size frame). Also unminimize the hidden window so the next
                // Show restores cleanly without a native minimize animation.
                tauri::WindowEvent::Resized(_)
                    if minimize_to_tray
                        && window.label() == "main"
                        && window.is_minimized().unwrap_or(false) =>
                {
                    let w = window.clone();
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(200));
                        if w.is_minimized().unwrap_or(false) {
                            let _ = w.hide();
                            let _ = w.unminimize();
                        }
                    });
                }
                // Persist size/position eagerly: a tray Quit can kill the process
                // before any save-on-exit could fire. Skip while minimized so we
                // don't record the collapsed geometry.
                //
                // The is_minimized() guard alone isn't enough: on a Win+D restore of
                // a borderless window, Windows fires a Resized where is_minimized() is
                // already false but the size is still the ~215x26 minimized placeholder
                // (verified via trace). Saving that persisted a title-bar sliver that
                // came back on next launch. So also reject any geometry below a sane
                // floor - a real window is never this small.
                tauri::WindowEvent::Resized(_) | tauri::WindowEvent::Moved(_)
                    if window.label() == "main" && !window.is_minimized().unwrap_or(false) =>
                {
                    const MIN_SAVE_W: u32 = 300;
                    const MIN_SAVE_H: u32 = 300;
                    if let Ok(sz) = window.inner_size() {
                        if sz.width < MIN_SAVE_W || sz.height < MIN_SAVE_H {
                            log_line(
                                window.app_handle(),
                                &format!(
                                    "win skip degenerate save: {}x{} phys (below floor)",
                                    sz.width, sz.height
                                ),
                            );
                            // Re-assert the last good size so the live window doesn't
                            // stay collapsed as the sliver (covers minimize-to-tray on
                            // and off). Harmless if the window is currently hidden.
                            let last = window
                                .app_handle()
                                .try_state::<HubState>()
                                .and_then(|s| *s.last_good_size.lock().unwrap());
                            if let Some((w, h)) = last {
                                let _ = window.set_size(tauri::PhysicalSize::new(w, h));
                            }
                        } else {
                            // Remember this as the last good size so the tray "show"
                            // path can restore it if a Win+D brings the window back
                            // collapsed.
                            if let Some(s) = window.app_handle().try_state::<HubState>() {
                                *s.last_good_size.lock().unwrap() = Some((sz.width, sz.height));
                            }
                            log_line(
                                window.app_handle(),
                                &format!(
                                    "win eager-save geometry: {}x{} phys",
                                    sz.width, sz.height
                                ),
                            );
                            if let Some(wv) = window.app_handle().get_webview_window("main") {
                                winstate::save(&wv);
                            }
                        }
                    }
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_apps,
            reload_manifest,
            open_manifest,
            manifest_dir,
            save_manifest,
            app_icon,
            get_settings,
            set_debug_logging,
            set_close_to_tray,
            set_minimize_to_tray,
            set_check_on_startup,
            set_always_on_top,
            set_locale,
            set_transparency,
            set_ui_scale,
            open_log,
            launch_app,
            term_input,
            term_resize,
            stop_app,
            open_url,
            report_outcome,
            install::setup_state,
            install::perform_install,
            install::launch_installed_and_exit,
            install::quit_app,
            portable::portable_state,
            portable::establish_portable,
            portable::establish_portable_at,
            portable::export_portable,
            portable::reveal_path,
            update::update_check,
            update::update_apply
        ])
        .run(tauri::generate_context!())
        .expect("error while running Moonpool");
}
