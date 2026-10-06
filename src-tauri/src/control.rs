//! Agent control surface: a local request/reply channel that exposes the same actions
//! `dispatch_control` (the argv path, see `lib.rs`) already answers, so the
//! MCP shim (`mcp.rs`) can reach the resident hub without spawning a throwaway process and
//! polling `state.json`. Ported from FasterDB's `control.rs` (`C:\claude-local\FasterDBApp`),
//! the first shipped implementation of the App-Patterns "Agent control surface" pattern.
//!
//! Transports (one shared handler, see `serve_conn`):
//!
//! - Windows: the named pipe `\\.\pipe\moonpool` for the installed copy, and
//!   `\\.\pipe\moonpool-<id>` for a portable copy (`instance::copy_id`), so every copy has its
//!   own channel and they never answer for each other.
//! - Linux/macOS: a Unix domain socket, `$XDG_RUNTIME_DIR/moonpool.sock` (see
//!   `socket_path_for` for the portable-mode and `sun_path`-length rules), mode 0600.
//!
//! The channel is also the hub's liveness probe: `ping` answering means a hub is up, nothing
//! answering means it is not. That is what `mcp.rs` trusts instead of scanning processes.
//!
//! Two dispatch shapes, matching what `dispatch_control` already does for the same actions:
//!
//! - `ping`, `list`, `show`, `quit`, `dump`, `paths`, `screenshot`, `stop-mcp`, `window-state`,
//!   `reset-mcp-seen`, `read-config`, `write-config`, `restore-config`:
//!   answered directly here, calling the SAME functions `dispatch_control` calls - one code
//!   path, no drift from the argv path or from what a click does. (`ping`, `list`, `screenshot`,
//!   `stop-mcp`, `window-state` and `reset-mcp-seen` exist only on this channel; `screenshot`
//!   is Windows-only.)
//! - `launch`, `stop`, `restart`, `reload`, `refresh-icons`, `help`: these are UI-owned today
//!   (terminal tab creation, `waitForRunning` status polling, icon loading all happen in
//!   Svelte). Rather than fork a Rust-only implementation that could drift from what the UI
//!   does, the handler emits the SAME `control://command` event `dispatch_control` emits, then
//!   waits on a waiter registered in `HubState::pipe_waiters` - so `report_outcome` (already
//!   called by the frontend when it finishes) wakes the reply directly, instead of a 150ms
//!   `state.json` poll loop.
//!
//! Protocol: newline-delimited JSON, one request per line, one reply per line. Request
//! `{"cmd": "...", "args": ["..."]}`, reply `{"ok": true, "result": "<string>"}` or
//! `{"ok": false, "error": "..."}`. Streaming (`watch`-style live `app_output`) is a follow-on
//! phase, not implemented here yet - see `private\PLAN-pipe-control-migration.md`.
//!
//! The argv+`state.json` control channel (`moonpool.exe launch <id> --ticket <key>`) still
//! works: a second launch of the same copy takes no lock (see `instance`), so it sends its
//! argv here as the `argv` verb and the hub runs it through `dispatch_control`, exactly as the
//! old single-instance plugin callback did.

use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

#[cfg(windows)]
use base64::Engine as _;
use serde::Deserialize;
use serde_json::{json, Value};
use tauri::{AppHandle, Emitter, Manager};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::sync::oneshot;

#[cfg(windows)]
use windows::Win32::Foundation::RECT;
#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{GetWindowRect, IsIconic, IsWindowVisible, IsZoomed};

use crate::{
    dispatch_control, dump_term_log, hub_paths_report, kill_mcp_shim, lock, read_config_cmd,
    reset_mcp_seen, restore_config_cmd, show_main, write_config_cmd, ControlCommand, HubState,
    ALL_WINDOWS,
};

/// How long the handler waits for `report_outcome` to resolve a UI-owned action. Mirrors
/// the old argv shim's `TICKET_TIMEOUT`: launch/restart are the slow ones (a managed restart
/// waits for the port to free), hence the generous cap.
pub(crate) const ACTION_TIMEOUT: Duration = Duration::from_secs(45);

/// How long a starting hub keeps retrying to bind the channel before giving up. The only way to
/// lose the bind on a healthy start is a previous hub of this copy still tearing down (the
/// per-copy lock is released a moment before the pipe/socket is), so a few seconds covers it.
const BIND_RETRY_FOR: Duration = Duration::from_secs(8);
const BIND_RETRY_EVERY: Duration = Duration::from_millis(250);

pub(crate) const UI_OWNED_ACTIONS: &[&str] = &[
    "launch",
    "stop",
    "restart",
    "reload",
    "refresh-icons",
    "help",
    "open-window",
];

/// Every verb the channel answers. `mcp.rs` checks each of these against its hub-process scan's
/// `NON_HUB_TOKENS` (a test), and `dispatch` refuses anything not listed, so the list cannot
/// silently drift from the match below.
pub(crate) const CONTROL_VERBS: &[&str] = &[
    "ping",
    "list",
    "show",
    "quit",
    "dump",
    "paths",
    "screenshot",
    "stop-mcp",
    "window-state",
    "reset-mcp-seen",
    "read-config",
    "write-config",
    "restore-config",
    "launch",
    "stop",
    "restart",
    "reload",
    "refresh-icons",
    "help",
    "open-window",
    "argv",
];

#[derive(Deserialize)]
pub(crate) struct Request {
    pub(crate) cmd: String,
    #[serde(default)]
    pub(crate) args: Vec<String>,
}

/// A boxed reply future, so one handler type serves the real hub and a test fake.
pub(crate) type ReplyFuture = Pin<Box<dyn Future<Output = Value> + Send>>;
/// Transport-independent request handler: one decoded request in, one reply value out. The
/// named-pipe and Unix-socket servers both drive connections through this, and tests inject a
/// fake so the transports can be exercised without a Tauri `AppHandle`.
pub(crate) type Handler = Arc<dyn Fn(Request) -> ReplyFuture + Send + Sync>;
pub(crate) type LogFn = Arc<dyn Fn(&str) + Send + Sync>;

/// Where THIS copy's channel lives: its named pipe on Windows, its Unix socket elsewhere. The
/// hub, the `moonpool.exe mcp` client and a forwarding second launch all call this, so they
/// agree for the same folder. A pipe is a kernel NAMESPACE object, not a file under AppData, so
/// the MSIX file/registry virtualization does not shadow it: reachable regardless of where the
/// MCP client process's view of the filesystem is redirected to.
pub(crate) fn default_endpoint() -> PathBuf {
    #[cfg(windows)]
    {
        PathBuf::from(crate::instance::pipe_name_for(crate::instance::copy_id()))
    }
    #[cfg(not(windows))]
    {
        let uid = unsafe { libc::geteuid() };
        socket_path_for(
            crate::portable::is_portable(),
            std::env::var_os("XDG_RUNTIME_DIR")
                .map(PathBuf::from)
                .filter(|p| p.is_absolute())
                .as_deref(),
            crate::portable::data_dir().as_deref(),
            uid,
            &crate::instance::suffix(),
        )
    }
}

/// Longest socket path we will bind. `sun_path` is 108 bytes on Linux and 104 on macOS
/// (including the NUL); stay under both.
#[cfg_attr(windows, allow(dead_code))]
const MAX_SOCKET_PATH: usize = 100;

#[cfg_attr(windows, allow(dead_code))]
fn tmp_fallback_dir(uid: u32) -> PathBuf {
    PathBuf::from(format!("/tmp/moonpool-{uid}"))
}

/// Pick the Unix socket path. A portable bundle keeps its own socket in its own data dir so it
/// never collides with an installed hub; otherwise `$XDG_RUNTIME_DIR/moonpool.sock` (already
/// per-user and 0700), else the data dir. A path over the `sun_path` limit falls back to a short
/// per-user directory under /tmp (created 0700 by the server); that directory is shared by every
/// copy, so the file name there carries the copy's `suffix` (`""` installed, `-<id>` portable).
#[cfg_attr(windows, allow(dead_code))]
fn socket_path_for(
    portable: bool,
    xdg: Option<&Path>,
    data: Option<&Path>,
    uid: u32,
    suffix: &str,
) -> PathBuf {
    let base = if portable { data } else { xdg.or(data) };
    if let Some(base) = base {
        let candidate = base.join("moonpool.sock");
        if candidate.as_os_str().len() <= MAX_SOCKET_PATH {
            return candidate;
        }
    }
    tmp_fallback_dir(uid).join(format!("moonpool{suffix}.sock"))
}

/// Start the control server on Tauri's async runtime. Called from `.setup()` with the app
/// handle. Failure to bind is logged and non-fatal: the hub keeps running without the channel
/// (an old-style argv client still works).
pub fn spawn(app: AppHandle) {
    let handler = app_handler(app.clone());
    let log: LogFn = {
        let app = app.clone();
        Arc::new(move |m: &str| crate::log_line(&app, m))
    };
    tauri::async_runtime::spawn(async move {
        let endpoint = default_endpoint();
        if let Err(e) = serve_at(&endpoint, handler, log.clone()).await {
            log(&format!("control: server stopped: {e}"));
        }
    });
}

/// Best-effort removal of the Unix socket file on a clean exit. A pipe vanishes with the
/// process, so this is a no-op on Windows.
pub fn cleanup() {
    #[cfg(unix)]
    if let Some(path) = BOUND_SOCKET.get() {
        let _ = std::fs::remove_file(path);
    }
}

fn app_handler(app: AppHandle) -> Handler {
    Arc::new(move |req| {
        let app = app.clone();
        Box::pin(async move { dispatch(&app, req).await })
    })
}

/// Retry `attempt` for `BIND_RETRY_FOR`, logging once when it first fails and once on giving up.
async fn with_bind_retry<T, F>(what: &str, log: &LogFn, mut attempt: F) -> std::io::Result<T>
where
    F: FnMut() -> std::io::Result<T>,
{
    let deadline = tokio::time::Instant::now() + BIND_RETRY_FOR;
    let mut warned = false;
    loop {
        match attempt() {
            Ok(v) => return Ok(v),
            Err(e) => {
                if tokio::time::Instant::now() >= deadline {
                    log(&format!(
                        "control: could not bind {what} after {}s ({e}); another hub may own \
                         it, running without the control channel",
                        BIND_RETRY_FOR.as_secs()
                    ));
                    return Err(e);
                }
                if !warned {
                    log(&format!(
                        "control: {what} is busy ({e}); retrying for {}s in case a previous \
                         hub is still exiting",
                        BIND_RETRY_FOR.as_secs()
                    ));
                    warned = true;
                }
                tokio::time::sleep(BIND_RETRY_EVERY).await;
            }
        }
    }
}

#[cfg(windows)]
pub(crate) async fn serve_at(endpoint: &Path, handler: Handler, log: LogFn) -> std::io::Result<()> {
    use tokio::net::windows::named_pipe::ServerOptions;
    let mut server =
        with_bind_retry("the control pipe", &log, || bind_first_pipe(endpoint)).await?;
    log(&format!(
        "control-pipe: listening on {}",
        endpoint.display()
    ));
    loop {
        // Wait for a client, then immediately stand up the next instance so the listener never
        // has a window where a connect would be refused.
        server.connect().await?;
        let connected = server;
        server = ServerOptions::new().create(endpoint)?;

        let handler = handler.clone();
        tokio::spawn(async move {
            let _ = serve_conn(connected, handler).await;
        });
    }
}

/// Create the first instance of the pipe `endpoint`, failing if any process already has one.
/// `first_pipe_instance` is what keeps a second hub (of this copy, since each copy has its own
/// name) from also listening on it; the per-copy lock (`instance`) normally stops that earlier.
#[cfg(windows)]
pub(crate) fn bind_first_pipe(
    endpoint: &Path,
) -> std::io::Result<tokio::net::windows::named_pipe::NamedPipeServer> {
    tokio::net::windows::named_pipe::ServerOptions::new()
        .first_pipe_instance(true)
        .create(endpoint)
}

#[cfg(unix)]
static BOUND_SOCKET: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();

#[cfg(unix)]
pub(crate) async fn serve_at(endpoint: &Path, handler: Handler, log: LogFn) -> std::io::Result<()> {
    let listener = with_bind_retry("the control socket", &log, || bind_unix(endpoint)).await?;
    let _ = BOUND_SOCKET.set(endpoint.to_path_buf());
    log(&format!("control: listening on {}", endpoint.display()));
    loop {
        let (stream, _) = listener.accept().await?;
        let handler = handler.clone();
        tokio::spawn(async move {
            let _ = serve_conn(stream, handler).await;
        });
    }
}

/// Make sure `dir` exists and is a private directory we own (used for the /tmp fallback, where
/// another user could otherwise pre-create it).
#[cfg(unix)]
fn ensure_private_dir(dir: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::{DirBuilderExt, MetadataExt, PermissionsExt};
    match std::fs::DirBuilder::new().mode(0o700).create(dir) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            let md = std::fs::symlink_metadata(dir)?;
            if !md.is_dir() || md.uid() != unsafe { libc::geteuid() } {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    format!("{} exists and is not a directory we own", dir.display()),
                ));
            }
            std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o700))
        }
        Err(e) => Err(e),
    }
}

/// Bind the Unix socket at `path` with mode 0600. A leftover socket file from a crashed hub is
/// removed, but only after confirming nothing answers on it: a live listener means another hub
/// owns the channel, which we must not steal (reported as `AddrInUse`).
#[cfg(unix)]
fn bind_unix(path: &Path) -> std::io::Result<tokio::net::UnixListener> {
    use std::os::unix::fs::{FileTypeExt, PermissionsExt};
    use tokio::net::UnixListener;

    if let Some(dir) = path.parent() {
        let uid = unsafe { libc::geteuid() };
        if dir == tmp_fallback_dir(uid) {
            ensure_private_dir(dir)?;
        } else {
            std::fs::create_dir_all(dir)?;
        }
    }
    // The 0177 umask makes the socket file 0600 from the instant it exists; the explicit
    // set_permissions below is the belt to its braces. umask is process-global but only held
    // across this one bind call.
    let bind_once = || {
        let old = unsafe { libc::umask(0o177) };
        let r = UnixListener::bind(path);
        unsafe { libc::umask(old) };
        r
    };
    let listener = match bind_once() {
        Ok(l) => l,
        Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
            let is_socket = std::fs::symlink_metadata(path)
                .map(|m| m.file_type().is_socket())
                .unwrap_or(false);
            match std::os::unix::net::UnixStream::connect(path) {
                // Something is listening: a live hub owns it.
                Ok(_) => return Err(e),
                Err(c)
                    if is_socket
                        && matches!(
                            c.kind(),
                            std::io::ErrorKind::ConnectionRefused | std::io::ErrorKind::NotFound
                        ) =>
                {
                    std::fs::remove_file(path)?;
                    bind_once()?
                }
                Err(_) => return Err(e),
            }
        }
        Err(e) => return Err(e),
    };
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
    Ok(listener)
}

/// Serve one connected client: read newline-delimited request frames, dispatch, write one reply
/// per request. Shared by every transport.
pub(crate) async fn serve_conn<S>(stream: S, handler: Handler) -> std::io::Result<()>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    let (read_half, mut write_half) = tokio::io::split(stream);
    let mut lines = BufReader::new(read_half).lines();
    while let Some(line) = lines.next_line().await? {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let req: Request = match serde_json::from_str(line) {
            Ok(r) => r,
            Err(e) => {
                write_frame(
                    &mut write_half,
                    &json!({ "ok": false, "error": format!("bad request: {e}") }),
                )
                .await?;
                continue;
            }
        };
        let reply = handler(req).await;
        write_frame(&mut write_half, &reply).await?;
    }
    Ok(())
}

/// Route one request. `positional` mirrors the shape `dispatch_control` builds from argv:
/// `[action, arg1, arg2, ...]`, so the same handler functions can be called unmodified.
async fn dispatch(app: &AppHandle, req: Request) -> Value {
    if !CONTROL_VERBS.contains(&req.cmd.as_str()) {
        return json!({ "ok": false, "error": format!("unknown cmd: {}", req.cmd) });
    }
    let positional: Vec<&str> = std::iter::once(req.cmd.as_str())
        .chain(req.args.iter().map(String::as_str))
        .collect();
    let arg = req.args.first().cloned();

    match req.cmd.as_str() {
        "ping" => json!({ "ok": true, "result": "pong" }),
        // A second launch of this copy handing over its argv (`instance::forward`). Run on
        // the same path the old single-instance plugin callback used.
        "argv" => {
            let argv: Vec<String> = std::iter::once(String::from("moonpool"))
                .chain(req.args.iter().cloned())
                .collect();
            dispatch_control(app, &argv);
            json!({ "ok": true, "result": Value::Null })
        }
        "list" => list_verb(app),
        "show" => {
            show_main(app);
            json!({ "ok": true, "result": Value::Null })
        }
        "quit" => {
            crate::log_line(app, "control: quit");
            app.exit(0);
            json!({ "ok": true, "result": Value::Null })
        }
        "dump" => {
            let (status, detail) = dump_term_log(app, arg.as_deref(), &positional);
            reply(status, detail)
        }
        "paths" => json!({ "ok": true, "result": hub_paths_report(app) }),
        "screenshot" => screenshot(app, arg.as_deref(), req.args.get(1).map(String::as_str)),
        "stop-mcp" => stop_mcp(app, arg.as_deref()),
        "open-window" => match open_window_arg(&req.args) {
            Ok(a) => run_ui_action(app, "open-window", Some(a)).await,
            Err(e) => json!({ "ok": false, "error": e }),
        },
        "window-state" => window_state(app, arg.as_deref()),
        "reset-mcp-seen" => reset_mcp_seen_cmd(app, arg.as_deref()),
        "read-config" => {
            let (status, detail) = read_config_cmd(app);
            reply(status, detail)
        }
        "write-config" => {
            let (status, detail) = write_config_cmd(app, &positional);
            reply(status, detail)
        }
        "restore-config" => {
            let (status, detail) = restore_config_cmd(app, &positional);
            reply(status, detail)
        }
        action if UI_OWNED_ACTIONS.contains(&action) => run_ui_action(app, action, arg).await,
        other => json!({ "ok": false, "error": format!("unknown cmd: {other}") }),
    }
}

fn reply(status: &str, detail: String) -> Value {
    if status == "ok" {
        json!({ "ok": true, "result": detail })
    } else {
        json!({ "ok": false, "error": detail })
    }
}

/// Build the `list` payload from the manifest and the last status tick. Same `apps`/`statuses`
/// shape `write_state` puts in `state.json`. Before the poller's first tick `statuses` is empty
/// while `apps` is not; flag that so a caller does not read "no status" as "everything stopped".
pub(crate) fn list_snapshot(apps: Value, statuses: Value) -> Value {
    let not_ready = statuses.as_array().is_some_and(Vec::is_empty)
        && apps.as_array().is_some_and(|a| !a.is_empty());
    let mut snap = json!({ "apps": apps, "statuses": statuses });
    if not_ready {
        snap["statusNotReady"] = Value::Bool(true);
    }
    snap
}

/// `list`: the live registered apps and their statuses straight from memory, so the answer is
/// never a stale `state.json` left behind by a hub that has since exited. `result` is the
/// snapshot serialized as a string (every verb's `result` is a plain string).
fn list_verb(app: &AppHandle) -> Value {
    let Some(state) = app.try_state::<HubState>() else {
        return json!({ "ok": false, "error": "hub state unavailable" });
    };
    let apps = serde_json::to_value(&*lock(&state.manifest)).unwrap_or(Value::Null);
    let statuses = serde_json::to_value(&*lock(&state.last_statuses)).unwrap_or(Value::Null);
    json!({ "ok": true, "result": list_snapshot(apps, statuses).to_string() })
}

/// Capture one of Moonpool's OWN windows (never an arbitrary HWND/PID from the wire) and return
/// it as base64-encoded PNG bytes. `label` defaults to `"main"`; any value outside `ALL_WINDOWS`
/// is rejected before a window lookup even happens, so this can never be pointed at another
/// process's window. See `screenshot.rs` for why `PrintWindow` is safe to use this way, and why
/// PNG (not raw BMP) keeps this small enough to hand back inline.
#[cfg(windows)]
fn screenshot(app: &AppHandle, label: Option<&str>, max_dim: Option<&str>) -> Value {
    let label = label.unwrap_or("main");
    // Optional second arg: longer-side cap, clamped to 320..=2400; absent keeps the default.
    let max_dim = match max_dim {
        None => crate::screenshot::MAX_DIMENSION,
        Some(v) => match v.parse::<u32>() {
            Ok(n) => crate::screenshot::clamp_max_dim(n),
            Err(_) => {
                return json!({ "ok": false, "error": format!("bad max_dim '{v}' - expected an integer") })
            }
        },
    };
    if !ALL_WINDOWS.contains(&label) {
        return json!({
            "ok": false,
            "error": format!("unknown window '{label}' - expected one of {ALL_WINDOWS:?}")
        });
    }
    let Some(window) = app.get_webview_window(label) else {
        return json!({ "ok": false, "error": format!("window '{label}' is not open") });
    };
    let hwnd = match window.hwnd() {
        Ok(h) => h,
        Err(e) => return json!({ "ok": false, "error": format!("hwnd: {e}") }),
    };
    // `result` must stay a plain string: `reply_to_result` (mcp.rs) reads it via
    // `Value::as_str`, the same shape every other verb here returns (dump's file path, etc).
    match crate::screenshot::capture_hwnd_to_png(hwnd, max_dim) {
        Ok(png) => json!({
            "ok": true,
            "result": base64::engine::general_purpose::STANDARD.encode(png)
        }),
        Err(e) => json!({ "ok": false, "error": e }),
    }
}

/// Window capture is built on Win32 `PrintWindow`; there is no equivalent here yet.
#[cfg(not(windows))]
fn screenshot(_app: &AppHandle, _label: Option<&str>, _max_dim: Option<&str>) -> Value {
    json!({ "ok": false, "error": "screenshot is not supported on this platform (Windows only)" })
}

/// Window kinds `open-window` accepts. `terminal` selects an app's terminal tab and widens the
/// hub so the CLI pane is showing; `cli` only widens the hub. The rest open the same windows the
/// menu items open (the frontend calls the same `open*Window` helpers).
const OPEN_WINDOW_KINDS: &[&str] = &[
    "settings",
    "about",
    "editor",
    "installer",
    "help",
    "terminal",
    "cli",
];

/// Validate `open-window <kind> [app-id]` and fold it into the single `arg` string the
/// `control://command` event carries: `kind` or `kind:app-id`.
fn open_window_arg(args: &[String]) -> Result<String, String> {
    let Some(kind) = args.first() else {
        return Err(format!(
            "missing window kind - expected one of {OPEN_WINDOW_KINDS:?}"
        ));
    };
    if !OPEN_WINDOW_KINDS.contains(&kind.as_str()) {
        return Err(format!(
            "unknown window kind '{kind}' - expected one of {OPEN_WINDOW_KINDS:?}"
        ));
    }
    match args.get(1) {
        Some(id) if !id.is_empty() => Ok(format!("{kind}:{id}")),
        _ if kind == "terminal" => Err("terminal needs an app id".to_string()),
        _ => Ok(kind.clone()),
    }
}

/// Report one of Moonpool's OWN windows' geometry and visibility state (never an arbitrary
/// HWND/PID from the wire - same `ALL_WINDOWS` allowlist as `screenshot`). Built for tests: lets
/// a test assert window position/minimized/maximized state directly instead of eyeballing a
/// screenshot, e.g. to guard the minimize/restore collapse bug (see
/// moonpool-window-minimize memory) without a human watching.
fn window_state(app: &AppHandle, label: Option<&str>) -> Value {
    let label = label.unwrap_or("main");
    if !ALL_WINDOWS.contains(&label) {
        return json!({
            "ok": false,
            "error": format!("unknown window '{label}' - expected one of {ALL_WINDOWS:?}")
        });
    }
    let Some(window) = app.get_webview_window(label) else {
        // Not an error: "not open" is itself a useful, expected state for windows like
        // installer/editor/settings/about that only exist while shown.
        return json!({ "ok": true, "result": json!({ "open": false }).to_string() });
    };
    #[cfg(windows)]
    let state = {
        let hwnd = match window.hwnd() {
            Ok(h) => h,
            Err(e) => return json!({ "ok": false, "error": format!("hwnd: {e}") }),
        };
        let mut rect = RECT::default();
        if let Err(e) = unsafe { GetWindowRect(hwnd, &mut rect) } {
            return json!({ "ok": false, "error": format!("GetWindowRect: {e}") });
        }
        json!({
            "open": true,
            "visible": unsafe { IsWindowVisible(hwnd) }.as_bool(),
            "minimized": unsafe { IsIconic(hwnd) }.as_bool(),
            "maximized": unsafe { IsZoomed(hwnd) }.as_bool(),
            "x": rect.left,
            "y": rect.top,
            "width": rect.right - rect.left,
            "height": rect.bottom - rect.top,
        })
    };
    // Same fields from Tauri's portable window API (physical pixels, outer frame). Not run on
    // Linux/macOS yet; the Win32 branch above is the tested one.
    #[cfg(not(windows))]
    let state = {
        let pos = match window.outer_position() {
            Ok(p) => p,
            Err(e) => return json!({ "ok": false, "error": format!("outer_position: {e}") }),
        };
        let size = match window.outer_size() {
            Ok(s) => s,
            Err(e) => return json!({ "ok": false, "error": format!("outer_size: {e}") }),
        };
        json!({
            "open": true,
            "visible": window.is_visible().unwrap_or(false),
            "minimized": window.is_minimized().unwrap_or(false),
            "maximized": window.is_maximized().unwrap_or(false),
            "x": pos.x,
            "y": pos.y,
            "width": size.width,
            "height": size.height,
        })
    };
    // Same plain-string `result` requirement as every other verb here - nest the geometry as a
    // JSON string rather than a JSON object.
    json!({ "ok": true, "result": state.to_string() })
}

/// Clear the sticky "ever seen" record for app `id`'s MCP shim (or every app's, if no id is
/// given). Test-only knob: normal operation never clears `mcp_seen` (see `AppStatus.mcp_seen`),
/// so a test suite that wants to re-observe the "not yet seen" sidebar state needs an explicit
/// way back to it between runs.
fn reset_mcp_seen_cmd(app: &AppHandle, id: Option<&str>) -> Value {
    let Some(state) = app.try_state::<HubState>() else {
        return json!({ "ok": false, "error": "hub state unavailable" });
    };
    json!({ "ok": true, "result": reset_mcp_seen(app, &state, id) })
}

/// Kill app `id`'s attached MCP shim process (see `kill_mcp_shim`), leaving the app itself
/// untouched. Only kills the process by name+shim-check, never anything the MCP host would
/// need to respawn on its own - there is no "restart" verb because Moonpool never owned
/// spawning the shim in the first place (its MCP host does, on next tool call).
fn stop_mcp(app: &AppHandle, id: Option<&str>) -> Value {
    let Some(id) = id else {
        return json!({ "ok": false, "error": "missing app id" });
    };
    let Some(state) = app.try_state::<HubState>() else {
        return json!({ "ok": false, "error": "hub state unavailable" });
    };
    match kill_mcp_shim(&state, id) {
        Ok(()) => json!({ "ok": true, "result": "stopped" }),
        Err(e) => json!({ "ok": false, "error": e }),
    }
}

/// Run a UI-owned action (`launch`/`stop`/`restart`/`reload`/`refresh-icons`) by emitting the
/// same `control://command` event the argv path emits, then waiting on a waiter woken by
/// `report_outcome` - the same completion signal the frontend already reports, just delivered
/// as a direct wake instead of a `state.json` poll.
async fn run_ui_action(app: &AppHandle, action: &str, arg: Option<String>) -> Value {
    let Some(state) = app.try_state::<HubState>() else {
        return json!({ "ok": false, "error": "hub state unavailable" });
    };
    if !state
        .frontend_ready
        .load(std::sync::atomic::Ordering::Relaxed)
    {
        return json!({
            "ok": false,
            "error": "frontend not loaded - the hub window has no UI to act on this command \
                      (check for a devUrl/bundling build mistake, see \
                      tauri-cargo-build-is-dev-frontend memory)"
        });
    }
    let ticket = new_ticket();
    let (tx, rx) = oneshot::channel();
    lock(&state.pipe_waiters).insert(ticket.clone(), tx);

    let _ = app.emit(
        "control://command",
        ControlCommand {
            action: action.to_string(),
            arg,
            ticket: Some(ticket.clone()),
        },
    );

    match tokio::time::timeout(ACTION_TIMEOUT, rx).await {
        Ok(Ok(())) => {
            let outcome = lock(&state.tickets)
                .iter()
                .find(|t| t.ticket == ticket)
                .map(|t| (t.status.clone(), t.detail.clone()));
            match outcome {
                Some((status, detail)) if status == "ok" => {
                    json!({ "ok": true, "result": detail })
                }
                Some((_, detail)) => json!({
                    "ok": false,
                    "error": detail.unwrap_or_else(|| format!("{action} failed"))
                }),
                None => json!({ "ok": false, "error": format!("{action}: no outcome recorded") }),
            }
        }
        // Sender dropped without sending - treat like a timeout rather than hang forever.
        Ok(Err(_)) | Err(_) => {
            lock(&state.pipe_waiters).remove(&ticket);
            json!({
                "ok": false,
                "error": format!(
                    "timed out after {}s waiting for '{action}' to report back",
                    ACTION_TIMEOUT.as_secs()
                )
            })
        }
    }
}

fn new_ticket() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static N: AtomicU64 = AtomicU64::new(0);
    format!(
        "pipe-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    )
}

async fn write_frame<W: AsyncWrite + Unpin>(w: &mut W, v: &Value) -> std::io::Result<()> {
    let mut buf = serde_json::to_vec(v)
        .unwrap_or_else(|_| br#"{"ok":false,"error":"reply encode failed"}"#.to_vec());
    buf.push(b'\n');
    w.write_all(&buf).await?;
    w.flush().await
}

#[cfg(test)]
mod tests {
    use super::{
        list_snapshot, open_window_arg, reply, socket_path_for, with_bind_retry, LogFn, Request,
        CONTROL_VERBS, UI_OWNED_ACTIONS,
    };
    use serde_json::json;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    fn v(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn open_window_arg_validates_kind_and_id() {
        assert_eq!(open_window_arg(&v(&["settings"])).unwrap(), "settings");
        assert_eq!(open_window_arg(&v(&["editor", "x"])).unwrap(), "editor:x");
        assert_eq!(
            open_window_arg(&v(&["terminal", "x"])).unwrap(),
            "terminal:x"
        );
        assert!(open_window_arg(&v(&["terminal"])).is_err());
        assert!(open_window_arg(&v(&["bogus"])).is_err());
        assert!(open_window_arg(&[]).is_err());
    }

    #[test]
    fn request_without_args_defaults_to_empty_vec() {
        let req: Request = serde_json::from_str(r#"{"cmd":"ping"}"#).unwrap();
        assert_eq!(req.cmd, "ping");
        assert!(req.args.is_empty());
    }

    #[test]
    fn request_ignores_unknown_fields() {
        // The MCP client and any future field additions shouldn't break parsing.
        let req: Request =
            serde_json::from_str(r#"{"cmd":"restart","args":["fasterdb"],"ticket":"x"}"#).unwrap();
        assert_eq!(req.cmd, "restart");
        assert_eq!(req.args, vec!["fasterdb".to_string()]);
    }

    #[test]
    fn reply_ok_wraps_detail_as_result() {
        assert_eq!(
            reply("ok", "hello".into()),
            serde_json::json!({ "ok": true, "result": "hello" })
        );
    }

    #[test]
    fn reply_error_wraps_detail_as_error() {
        assert_eq!(
            reply("error", "boom".into()),
            serde_json::json!({ "ok": false, "error": "boom" })
        );
    }

    #[test]
    fn ui_owned_actions_matches_the_verbs_dispatch_control_treats_as_ui_owned() {
        // If this list and lib.rs's argv-path handling of the same verbs ever diverge,
        // the two transports would disagree on which actions go through the
        // UI at all - a silent behavior split between the two control channels.
        for action in [
            "launch",
            "stop",
            "restart",
            "reload",
            "refresh-icons",
            "help",
        ] {
            assert!(
                UI_OWNED_ACTIONS.contains(&action),
                "{action} must be UI-owned"
            );
        }
        assert!(!UI_OWNED_ACTIONS.contains(&"ping"));
    }

    #[test]
    fn every_ui_owned_action_is_a_control_verb() {
        for a in UI_OWNED_ACTIONS {
            assert!(CONTROL_VERBS.contains(a), "{a} missing from CONTROL_VERBS");
        }
    }

    #[test]
    fn list_snapshot_flags_statuses_not_ready() {
        let apps = json!([{ "id": "web" }]);
        let snap = list_snapshot(apps.clone(), json!([]));
        assert_eq!(snap["statusNotReady"], json!(true));
        assert_eq!(snap["apps"], apps);

        // Statuses present: no flag. No apps at all: nothing to be "not ready" about.
        assert!(list_snapshot(apps, json!([{ "id": "web" }]))
            .get("statusNotReady")
            .is_none());
        assert!(list_snapshot(json!([]), json!([]))
            .get("statusNotReady")
            .is_none());
    }

    #[test]
    fn socket_path_prefers_xdg_runtime_dir() {
        let xdg = Path::new("/run/user/1000");
        let data = Path::new("/home/u/.config/Moonpool");
        assert_eq!(
            socket_path_for(false, Some(xdg), Some(data), 1000, ""),
            xdg.join("moonpool.sock")
        );
        assert_eq!(
            socket_path_for(false, None, Some(data), 1000, ""),
            data.join("moonpool.sock")
        );
    }

    #[test]
    fn socket_path_portable_stays_in_its_own_data_dir() {
        let xdg = Path::new("/run/user/1000");
        let data = Path::new("/media/usb/moonpool/.moonpool");
        assert_eq!(
            socket_path_for(true, Some(xdg), Some(data), 1000, "-0badf00d"),
            data.join("moonpool.sock")
        );
    }

    #[test]
    fn socket_path_too_long_falls_back_to_short_tmp_dir() {
        let long = PathBuf::from(format!("/home/u/{}", "deep/".repeat(30)));
        let p = socket_path_for(false, None, Some(&long), 1234, "");
        assert_eq!(p, PathBuf::from("/tmp/moonpool-1234").join("moonpool.sock"));
        // And with nothing to base it on at all.
        assert_eq!(
            socket_path_for(false, None, None, 7, ""),
            PathBuf::from("/tmp/moonpool-7").join("moonpool.sock")
        );
        // A portable copy's fallback socket carries its id, so it cannot collide with the
        // installed copy's (or another portable copy's) in the shared /tmp directory.
        assert_eq!(
            socket_path_for(true, None, Some(&long), 1234, "-0badf00d"),
            PathBuf::from("/tmp/moonpool-1234").join("moonpool-0badf00d.sock")
        );
    }

    /// Two hubs with different (per-copy) pipe names can both bind; a second bind of the SAME
    /// name fails. Unique test-only names: never the real `\\.\pipe\moonpool`.
    #[cfg(windows)]
    #[test]
    fn pipe_bind_is_exclusive_per_name() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0);
            let base = format!(
                r"\\.\pipe\moonpool-test-bind-{}-{nanos}",
                std::process::id()
            );
            let a = PathBuf::from(format!("{base}-a"));
            let b = PathBuf::from(format!("{base}-b"));
            let _sa = super::bind_first_pipe(&a).expect("copy A binds its name");
            let _sb = super::bind_first_pipe(&b).expect("copy B binds its own name too");
            assert!(
                super::bind_first_pipe(&a).is_err(),
                "a second hub of copy A must not bind A's name"
            );
        });
    }

    #[test]
    fn bind_retry_succeeds_after_transient_failures_and_gives_up_otherwise() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .unwrap();
        let logs = Arc::new(AtomicU32::new(0));
        let log: LogFn = {
            let logs = logs.clone();
            Arc::new(move |_| {
                logs.fetch_add(1, Ordering::Relaxed);
            })
        };
        let mut n = 0;
        let got = rt.block_on(with_bind_retry("test", &log, || {
            n += 1;
            if n < 3 {
                Err(std::io::Error::from(std::io::ErrorKind::PermissionDenied))
            } else {
                Ok(n)
            }
        }));
        assert_eq!(got.unwrap(), 3);
        assert_eq!(logs.load(Ordering::Relaxed), 1, "one 'retrying' line only");
    }
}
