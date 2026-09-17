//! Agent control surface: a named-pipe server that exposes the same actions
//! `dispatch_control` (the argv/single-instance path, see `lib.rs`) already answers, so the
//! MCP shim (`mcp.rs`) can reach the resident hub without spawning a throwaway process and
//! polling `state.json`. Ported from FasterDB's `control.rs` (`C:\claude-local\FasterDBApp`),
//! the first shipped implementation of the App-Patterns "Agent control surface" pattern.
//!
//! Two dispatch shapes, matching what `dispatch_control` already does for the same actions:
//!
//! - `ping`, `show`, `quit`, `dump`, `paths`, `screenshot`, `stop-mcp`, `window-state`,
//!   `reset-mcp-seen`, `read-config`, `write-config`, `restore-config`:
//!   answered directly here, calling the SAME functions `dispatch_control` calls - one code
//!   path, no drift from the argv path or from what a click does.
//! - `launch`, `stop`, `restart`, `reload`, `refresh-icons`: these are UI-owned today (terminal
//!   tab creation, `waitForRunning` status polling, icon loading all happen in Svelte). Rather
//!   than fork a Rust-only implementation that could drift from what the UI does, the pipe
//!   handler emits the SAME `control://command` event `dispatch_control` emits, then waits on a
//!   waiter registered in `HubState::pipe_waiters` - so `report_outcome` (already called by the
//!   frontend when it finishes) wakes the pipe reply directly, instead of a 150ms `state.json`
//!   poll loop.
//!
//! Protocol: newline-delimited JSON, one request per line, one reply per line. Streaming
//! (`watch`-style live `app_output`) is a follow-on phase, not implemented here yet - see
//! `private\PLAN-pipe-control-migration.md`.
//!
//! The argv+`state.json` control channel (`dispatch_control`, `tauri-plugin-single-instance`)
//! is untouched: this pipe is an ADDITIVE second channel, not a replacement, until the shim is
//! proven live on it (belt and suspenders, per the migration plan).

use std::time::Duration;

use base64::Engine as _;
use serde::Deserialize;
use serde_json::{json, Value};
use tauri::{AppHandle, Emitter, Manager};
use tokio::io::{AsyncBufReadExt, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::windows::named_pipe::{NamedPipeServer, ServerOptions};
use tokio::sync::oneshot;

use windows::Win32::Foundation::RECT;
use windows::Win32::UI::WindowsAndMessaging::{GetWindowRect, IsIconic, IsWindowVisible, IsZoomed};

use crate::{
    dump_term_log, hub_paths_report, kill_mcp_shim, lock, read_config_cmd, reset_mcp_seen,
    restore_config_cmd, show_main, write_config_cmd, ControlCommand, HubState, ALL_WINDOWS,
};

/// Fixed pipe name. A pipe is a kernel NAMESPACE object, not a file under AppData, so the MSIX
/// file/registry virtualization does not shadow it: reachable regardless of where the MCP
/// client process's view of the filesystem is redirected to.
pub const PIPE_NAME: &str = r"\\.\pipe\moonpool";

/// How long the pipe handler waits for `report_outcome` to resolve a UI-owned action. Mirrors
/// the old argv shim's `TICKET_TIMEOUT`: launch/restart are the slow ones (a managed restart
/// waits for the port to free), hence the generous cap.
const ACTION_TIMEOUT: Duration = Duration::from_secs(45);

const UI_OWNED_ACTIONS: &[&str] = &["launch", "stop", "restart", "reload", "refresh-icons"];

#[derive(Deserialize)]
struct Request {
    cmd: String,
    #[serde(default)]
    args: Vec<String>,
}

/// Start the pipe server on Tauri's async runtime. Called from `.setup()` with the app handle.
/// Failure to bind is logged and non-fatal: the argv+state.json channel keeps working without
/// the pipe.
pub fn spawn(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        if let Err(e) = serve(app.clone()).await {
            crate::log_line(&app, &format!("control-pipe: server stopped: {e}"));
        }
    });
}

async fn serve(app: AppHandle) -> std::io::Result<()> {
    // `first_pipe_instance` guards against a second app instance also binding the name; the app
    // is already single-instance for the tray, so exactly one process owns the pipe.
    let mut server = ServerOptions::new()
        .first_pipe_instance(true)
        .create(PIPE_NAME)?;
    crate::log_line(&app, &format!("control-pipe: listening on {PIPE_NAME}"));
    loop {
        // Wait for a client, then immediately stand up the next instance so the listener never
        // has a window where a connect would be refused.
        server.connect().await?;
        let connected = server;
        server = ServerOptions::new().create(PIPE_NAME)?;

        let app = app.clone();
        tauri::async_runtime::spawn(async move {
            let _ = handle_conn(connected, app).await;
        });
    }
}

/// Serve one connected client: read newline-delimited request frames, dispatch, write one reply
/// per request.
async fn handle_conn(pipe: NamedPipeServer, app: AppHandle) -> std::io::Result<()> {
    let (read_half, mut write_half) = tokio::io::split(pipe);
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
        let reply = dispatch(&app, req).await;
        write_frame(&mut write_half, &reply).await?;
    }
    Ok(())
}

/// Route one request. `positional` mirrors the shape `dispatch_control` builds from argv:
/// `[action, arg1, arg2, ...]`, so the same handler functions can be called unmodified.
async fn dispatch(app: &AppHandle, req: Request) -> Value {
    let positional: Vec<&str> = std::iter::once(req.cmd.as_str())
        .chain(req.args.iter().map(String::as_str))
        .collect();
    let arg = req.args.first().cloned();

    match req.cmd.as_str() {
        "ping" => json!({ "ok": true, "result": "pong" }),
        "show" => {
            show_main(app);
            json!({ "ok": true, "result": Value::Null })
        }
        "quit" => {
            crate::log_line(app, "control-pipe: quit");
            app.exit(0);
            json!({ "ok": true, "result": Value::Null })
        }
        "dump" => {
            let (status, detail) = dump_term_log(app, arg.as_deref(), &positional);
            reply(status, detail)
        }
        "paths" => json!({ "ok": true, "result": hub_paths_report(app) }),
        "screenshot" => screenshot(app, arg.as_deref()),
        "stop-mcp" => stop_mcp(app, arg.as_deref()),
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

/// Capture one of Moonpool's OWN windows (never an arbitrary HWND/PID from the wire) and return
/// it as base64-encoded BMP bytes. `label` defaults to `"main"`; any value outside `ALL_WINDOWS`
/// is rejected before a window lookup even happens, so this can never be pointed at another
/// process's window. See `screenshot.rs` for why `PrintWindow` is safe to use this way.
fn screenshot(app: &AppHandle, label: Option<&str>) -> Value {
    let label = label.unwrap_or("main");
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
    // `result` must stay a plain string: `pipe_reply_to_result` (mcp.rs) reads it via
    // `Value::as_str`, the same shape every other verb here returns (dump's file path, etc).
    // BMP bytes, base64-encoded; callers decode knowing the format is always BMP.
    match crate::screenshot::capture_hwnd(hwnd) {
        Ok(bmp) => json!({
            "ok": true,
            "result": base64::engine::general_purpose::STANDARD.encode(bmp)
        }),
        Err(e) => json!({ "ok": false, "error": e }),
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
    let hwnd = match window.hwnd() {
        Ok(h) => h,
        Err(e) => return json!({ "ok": false, "error": format!("hwnd: {e}") }),
    };
    let mut rect = RECT::default();
    if let Err(e) = unsafe { GetWindowRect(hwnd, &mut rect) } {
        return json!({ "ok": false, "error": format!("GetWindowRect: {e}") });
    }
    let state = json!({
        "open": true,
        "visible": unsafe { IsWindowVisible(hwnd) }.as_bool(),
        "minimized": unsafe { IsIconic(hwnd) }.as_bool(),
        "maximized": unsafe { IsZoomed(hwnd) }.as_bool(),
        "x": rect.left,
        "y": rect.top,
        "width": rect.right - rect.left,
        "height": rect.bottom - rect.top,
    });
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
    if !state.frontend_ready.load(std::sync::atomic::Ordering::Relaxed) {
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
    use super::{reply, Request, UI_OWNED_ACTIONS};

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
            serde_json::from_str(r#"{"cmd":"restart","args":["fasterdb"],"ticket":"x"}"#)
                .unwrap();
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
        // the pipe and argv transports would disagree on which actions go through the
        // UI at all - a silent behavior split between the two control channels.
        for action in ["launch", "stop", "restart", "reload", "refresh-icons"] {
            assert!(
                UI_OWNED_ACTIONS.contains(&action),
                "{action} must be UI-owned"
            );
        }
        assert!(!UI_OWNED_ACTIONS.contains(&"ping"));
    }
}
