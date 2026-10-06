//! `moonpool.exe mcp` - an MCP (Model Context Protocol) server over stdio, so an
//! agent can drive Moonpool with tools instead of shelling out and polling files.
//!
//! Why it lives in this exe rather than a companion Node package: it is the same
//! binary the user already installed, so there is nothing extra to build, install,
//! or keep in sync with the commands it exposes.
//!
//! What it is NOT: the hub. This process is a *client* of the resident tray
//! instance, driving it over the control channel (`control.rs`: a named pipe on
//! Windows, a Unix socket on Linux/macOS), which is also how it knows whether a hub
//! is up at all: a `ping` answered or not. Every tool call returns the real outcome
//! synchronously. The older channel documented in AI-README.md (spawn ourselves
//! with `<action> <id> --ticket <key>`, then poll `state.json`) survives only as a
//! fallback for a hub build that predates the channel.
//!
//! The protocol is hand-rolled: MCP over stdio is newline-delimited JSON-RPC 2.0,
//! serde_json is already a dependency, and a full async SDK (plus tokio) would
//! outweigh the small tools below.
//!
//! Note the entry point must run BEFORE `tauri::Builder`: the single-instance
//! plugin would otherwise forward our argv to the resident window and exit.

use serde_json::{json, Value};
use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// MCP revision we implement. Clients send their own in `initialize`; we answer
/// with ours and they downgrade if needed.
const PROTOCOL_VERSION: &str = "2025-06-18";

/// How long to wait for a ticket to leave `pending`. Launch/restart are the slow
/// ones (a managed restart waits for the port to free), hence the generous cap.
const TICKET_TIMEOUT: Duration = Duration::from_secs(45);

/// Default number of trailing lines returned by the dump tool. A full 512 KB log
/// would swamp an agent's context; the caller can ask for more.
const DEFAULT_TAIL: usize = 200;

// ---------------------------------------------------------------------------
// Paths and the resident instance
// ---------------------------------------------------------------------------

fn state_path() -> Option<PathBuf> {
    crate::portable::data_dir().map(|d| d.join("state.json"))
}

fn read_json_with_retry<F>(mut read: F, attempts: usize, delay: Duration) -> Option<Value>
where
    F: FnMut() -> Option<String>,
{
    for attempt in 0..attempts.max(1) {
        if let Some(value) = read().and_then(|text| serde_json::from_str(&text).ok()) {
            return Some(value);
        }
        if attempt + 1 < attempts {
            std::thread::sleep(delay);
        }
    }
    None
}

fn read_state() -> Option<Value> {
    let path = state_path()?;
    read_json_with_retry(
        || std::fs::read_to_string(&path).ok(),
        4,
        Duration::from_millis(15),
    )
}

/// Subcommand tokens that mark a moonpool process as NOT the resident tray hub -
/// an MCP server (`mcp`), a control-action spawn (`launch`/`--ticket`/...), or the
/// installer/updater helpers. The hub is a moonpool process carrying none of these.
///
/// This scan is only a SAFETY NET now (an improved error message, and the old-hub argv
/// fallback in `control`): liveness is decided by pinging the control channel. Every verb either
/// dispatcher answers must be listed (a test enforces it), since a helper process missing here
/// reads as a hub.
const NON_HUB_TOKENS: &[&str] = &[
    "mcp",
    "launch",
    "stop",
    "restart",
    "dump",
    "reload",
    "refresh-icons",
    "help",
    "show",
    "quit",
    "paths",
    "read-config",
    "write-config",
    "restore-config",
    "ping",
    "list",
    "open-window",
    "screenshot",
    "window-state",
    "stop-mcp",
    "reset-mcp-seen",
    "--ticket",
    "--uninstall",
    "--wait-pid",
];

/// Whether something that LOOKS like a Moonpool tray hub is in the process table. Unreliable by
/// nature (a hub started by the argv fallback carries a helper token in its own cmdline and is
/// invisible; a second moonpool-named exe counts), so it never decides liveness - `hub_alive` /
/// `ping_state` do. It is used only to improve an error message when the channel is unreachable,
/// and to gate the old-hub argv fallback in `control`.
///
/// It must EXCLUDE the other short-lived moonpool processes: the idle `moonpool.exe mcp` stdio
/// servers (one per agent session) and transient control-action spawns. We identify the hub as
/// the one moonpool process whose argv carries no subcommand token.
fn hub_running() -> bool {
    use sysinfo::{ProcessRefreshKind, RefreshKind, System};
    let me = std::process::id();
    let sys =
        System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::new()));
    sys.processes().iter().any(|(pid, p)| {
        if pid.as_u32() == me || !p.name().to_ascii_lowercase().starts_with("moonpool") {
            return false;
        }
        // args[0] is the exe path; a subcommand token in args[1..] means this is
        // an MCP server or a control spawn, not the hub.
        !p.cmd()
            .iter()
            .skip(1)
            .any(|a| NON_HUB_TOKENS.contains(&a.as_str()))
    })
}

// ---------------------------------------------------------------------------
// Driving the control channel
// ---------------------------------------------------------------------------

/// What a probe of the control channel found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipeState {
    /// Nothing is listening: no hub (pipe missing / socket absent or refusing).
    Down,
    /// A hub answered.
    Up,
    /// Something owns the channel but did not answer in time (hung hub, or busy past retries).
    Unreachable,
}

/// Why a channel call failed.
#[derive(Debug, PartialEq, Eq)]
enum CallError {
    /// Could not connect because nothing is listening.
    Down,
    /// Connected or tried to, but no usable answer within the budget.
    Unreachable(String),
    /// Connected and sent the request, then the stream failed (closed, bad frame). For `quit`
    /// this is expected: the hub exits before it can reply.
    Io(String),
}

/// Quick verbs (liveness probes, `list`): a hard cap so a hung hub cannot hang the tool.
const QUICK_TIMEOUT: Duration = Duration::from_secs(3);
/// Everything that is answered directly by the hub without the UI (file writes, `paths`...).
const DIRECT_TIMEOUT: Duration = Duration::from_secs(15);
/// UI-owned actions can legitimately take as long as the hub's own `ACTION_TIMEOUT`.
const UI_ACTION_TIMEOUT: Duration = Duration::from_secs(55);

/// Cap on waiting for the hub to come up (cold boot: window + tray + first status tick).
const START_TIMEOUT: Duration = Duration::from_secs(30);
/// Cap on waiting for the hub to go away after `quit`.
const STOP_TIMEOUT: Duration = Duration::from_secs(30);

fn timeout_for(action: &str) -> Duration {
    if crate::control::UI_OWNED_ACTIONS.contains(&action) {
        UI_ACTION_TIMEOUT
    } else if matches!(action, "ping" | "list" | "show" | "quit") {
        QUICK_TIMEOUT
    } else {
        DIRECT_TIMEOUT
    }
}

/// Sort a connect/IO error into a channel state. `NotFound` is the missing pipe
/// (ERROR_FILE_NOT_FOUND = 2 on Windows) or absent socket file (ENOENT); `ConnectionRefused`
/// is a stale socket file with no listener. Anything else (busy pipe, timeout, access denied)
/// means something is there that we cannot talk to.
fn classify_io(e: &std::io::Error) -> PipeState {
    use std::io::ErrorKind;
    if matches!(e.kind(), ErrorKind::NotFound | ErrorKind::ConnectionRefused)
        || (cfg!(windows) && e.raw_os_error() == Some(2))
    {
        PipeState::Down
    } else {
        PipeState::Unreachable
    }
}

/// The two ends we actually use of a connected stream, boxed so the Windows pipe `File` and a
/// Unix socket share the request/reply code.
trait Conn: std::io::Read + std::io::Write {}
impl<T: std::io::Read + std::io::Write> Conn for T {}

#[cfg(windows)]
fn connect(endpoint: &std::path::Path, _timeout: Duration) -> Result<Box<dyn Conn>, CallError> {
    const ERROR_PIPE_BUSY: i32 = 231;
    let mut last = String::new();
    for _ in 0..10 {
        match std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(endpoint)
        {
            Ok(f) => return Ok(Box::new(f)),
            Err(e) if e.raw_os_error() == Some(ERROR_PIPE_BUSY) => {
                last = e.to_string();
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                return Err(match classify_io(&e) {
                    PipeState::Down => CallError::Down,
                    _ => CallError::Unreachable(e.to_string()),
                })
            }
        }
    }
    Err(CallError::Unreachable(format!(
        "control pipe busy ({last})"
    )))
}

#[cfg(unix)]
fn connect(endpoint: &std::path::Path, timeout: Duration) -> Result<Box<dyn Conn>, CallError> {
    match std::os::unix::net::UnixStream::connect(endpoint) {
        Ok(s) => {
            // Lets the worker thread give up on a hung hub instead of leaking forever.
            let _ = s.set_read_timeout(Some(timeout));
            let _ = s.set_write_timeout(Some(timeout));
            Ok(Box::new(s))
        }
        Err(e) => Err(match classify_io(&e) {
            PipeState::Down => CallError::Down,
            _ => CallError::Unreachable(e.to_string()),
        }),
    }
}

/// Blocking request/reply on one connection. Runs on a worker thread (see `call_at`).
fn exchange(
    endpoint: &std::path::Path,
    line: &[u8],
    timeout: Duration,
) -> Result<Value, CallError> {
    let mut conn = connect(endpoint, timeout)?;
    conn.write_all(line)
        .and_then(|_| conn.flush())
        .map_err(|e| CallError::Io(format!("write: {e}")))?;
    let mut reader = std::io::BufReader::new(conn);
    let mut buf = String::new();
    reader
        .read_line(&mut buf)
        .map_err(|e| CallError::Io(format!("read: {e}")))?;
    if buf.trim().is_empty() {
        return Err(CallError::Io("connection closed before a reply".into()));
    }
    serde_json::from_str(buf.trim()).map_err(|e| CallError::Io(format!("bad reply: {e}")))
}

/// Send one request to `endpoint` and parse its single reply line, giving up after `timeout`.
/// The call runs on a worker thread because a Windows named pipe opened as a `File` has no read
/// timeout; a thread left blocked on a hung hub is abandoned (it holds nothing we need).
fn call_at(
    endpoint: &std::path::Path,
    action: &str,
    args: &[&str],
    timeout: Duration,
) -> Result<Value, CallError> {
    let mut line = serde_json::to_vec(&json!({ "cmd": action, "args": args }))
        .map_err(|e| CallError::Io(e.to_string()))?;
    line.push(b'\n');
    let (tx, rx) = std::sync::mpsc::channel();
    let ep = endpoint.to_path_buf();
    std::thread::spawn(move || {
        let _ = tx.send(exchange(&ep, &line, timeout));
    });
    match rx.recv_timeout(timeout) {
        Ok(r) => r,
        Err(_) => Err(CallError::Unreachable(format!(
            "no reply within {}s",
            timeout.as_secs()
        ))),
    }
}

/// `call_at` against the real channel with an explicit time budget.
fn control_call_timeout(
    action: &str,
    args: &[&str],
    timeout: Duration,
) -> Result<Value, CallError> {
    call_at(&crate::control::default_endpoint(), action, args, timeout)
}

/// `call_at` against the real channel with the verb's usual budget.
fn control_call(action: &str, args: &[&str]) -> Result<Value, CallError> {
    control_call_timeout(action, args, timeout_for(action))
}

/// Probe the channel with `ping`. A connect that succeeds but gets no answer is `Unreachable`,
/// not `Up`: a hub that cannot answer a ping is not usable.
fn ping_at(endpoint: &std::path::Path) -> PipeState {
    match call_at(endpoint, "ping", &[], QUICK_TIMEOUT) {
        Ok(_) => PipeState::Up,
        Err(CallError::Down) => PipeState::Down,
        Err(CallError::Unreachable(_)) | Err(CallError::Io(_)) => PipeState::Unreachable,
    }
}

fn ping_state() -> PipeState {
    ping_at(&crate::control::default_endpoint())
}

/// Whether a hub is up and answering. The single source of truth for "is Moonpool running".
fn hub_alive() -> bool {
    ping_state() == PipeState::Up
}

/// The error for "no hub is listening". Names the sandbox when our file view is a packaged
/// overlay, since a sandboxed process may be unable to reach the channel at all and "not
/// running" would then be a misleading answer.
fn not_running_error() -> String {
    match sandbox_overlay_reason() {
        Some(reason) => sandbox_error(&reason),
        None => "Moonpool is not running - call moonpool_bootup_launcher first".into(),
    }
}

/// The error for "the channel is owned but silent". `hub_running()` only sharpens the wording.
fn unreachable_error(detail: &str) -> String {
    format!(
        "Moonpool's control channel did not answer ({detail}). A Moonpool process may be hung{} - \
         close it from the tray or end the process, then call moonpool_bootup_launcher.",
        if hub_running() {
            " (one is in the process list)"
        } else {
            ""
        }
    )
}

/// A ticket key unique enough for concurrent agents: pid + a monotonic counter.
fn new_ticket() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static N: AtomicU64 = AtomicU64::new(0);
    format!(
        "mcp-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    )
}

/// Fire one control command at the resident instance and block until it resolves. Returns
/// `Ok(detail)` on success (detail is whatever the handler reported - for `dump`, the file path
/// it wrote) or `Err(message)`.
///
/// Goes over the control channel (`control.rs`: named pipe on Windows, Unix socket elsewhere) -
/// one request, one reply, no spawn and no poll. When the channel reports `Down` nothing can
/// answer, so the answer is "Moonpool is not running". The one exception is an OLDER hub build
/// that predates the channel: it has no listener but is a live hub reachable through the argv +
/// `state.json` path. We only take that path when the process scan also sees a hub, accepting
/// that a stray moonpool-named process can occasionally make this try (and time out) rather
/// than fail fast.
fn control(action: &str, args: &[&str]) -> Result<String, String> {
    // Defense in depth: `action` is always a hard-coded literal and every `app_id`
    // reaching here has passed `is_valid_app_id`, so no forwarded token should ever
    // look like a flag. Refuse if that invariant is ever violated rather than spawn
    // a child whose argv could be reinterpreted.
    if action.starts_with('-') || args.iter().any(|a| a.starts_with('-')) {
        return Err("refusing to forward a flag-like control argument".into());
    }
    match control_call(action, args) {
        Ok(reply) => reply_to_result(action, reply),
        Err(CallError::Down) => {
            if hub_running() {
                control_via_argv(action, args)
            } else {
                Err(not_running_error())
            }
        }
        Err(CallError::Unreachable(m)) => Err(unreachable_error(&m)),
        Err(CallError::Io(m)) => Err(format!("{action}: control channel failed mid-call: {m}")),
    }
}

/// Turn a `control.rs`-shaped reply (`{"ok":true,"result":...}` / `{"ok":false,"error":...}`)
/// into the same `Result<String, String>` shape `control_via_argv` returns, so callers cannot
/// tell which transport served the request.
fn reply_to_result(action: &str, reply: Value) -> Result<String, String> {
    if reply.get("ok").and_then(Value::as_bool) == Some(true) {
        Ok(reply
            .get("result")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string())
    } else {
        let msg = reply.get("error").and_then(Value::as_str).unwrap_or("");
        Err(if msg.is_empty() {
            format!("{action} failed")
        } else {
            msg.to_string()
        })
    }
}

/// The original argv-spawn + `state.json`-poll implementation of `control()`. Kept for hubs
/// that predate the control channel; see `control`.
fn control_via_argv(action: &str, args: &[&str]) -> Result<String, String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let ticket = new_ticket();

    let mut cmd = std::process::Command::new(exe);
    cmd.arg(action);
    for a in args {
        cmd.arg(a);
    }
    cmd.arg("--ticket").arg(&ticket);
    // The forwarding process exits immediately; wait for it so a spawn failure
    // surfaces here rather than as a mysterious timeout.
    let status = cmd
        .status()
        .map_err(|e| format!("cannot run Moonpool: {e}"))?;
    if !status.success() {
        return Err(format!("Moonpool exited with {status}"));
    }

    let deadline = Instant::now() + TICKET_TIMEOUT;
    loop {
        if let Some(rec) = read_state()
            .and_then(|s| s.get("tickets").and_then(|t| t.as_array().cloned()))
            .and_then(|ts| {
                ts.into_iter()
                    .find(|t| t.get("ticket").and_then(Value::as_str) == Some(ticket.as_str()))
            })
        {
            let status = rec.get("status").and_then(Value::as_str).unwrap_or("");
            let detail = rec
                .get("detail")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string();
            match status {
                "ok" => return Ok(detail),
                "error" => {
                    return Err(if detail.is_empty() {
                        format!("{action} failed")
                    } else {
                        detail
                    })
                }
                _ => {} // still pending
            }
        }
        if Instant::now() >= deadline {
            return Err(format!(
                "timed out after {}s waiting for '{action}' to report back",
                TICKET_TIMEOUT.as_secs()
            ));
        }
        std::thread::sleep(Duration::from_millis(150));
    }
}

/// Boot the resident tray hub, for the cold-start case where no hub is running and
/// every other tool would (correctly) refuse. Spawns the installed exe with NO
/// subcommand so it comes up as the normal tray window, then waits until its control
/// channel answers so the caller can immediately follow with a launch/restart.
///
/// A no-arg spawn is the one invocation that boots the hub rather than the
/// installer (the exe is installed under %USERPROFILE%\.moonpool, so `needs_setup()`
/// is false) and rather than dropping a control action on the floor.
///
/// If the spawned process exits early it was forwarded to a hub that is itself exiting
/// (single-instance), so it is respawned once.
fn start_hub() -> Result<String, String> {
    match ping_state() {
        PipeState::Up => return Ok("Moonpool is already running".into()),
        PipeState::Unreachable => {
            return Err(unreachable_error(
                "a hub owns the channel but is not answering",
            ))
        }
        PipeState::Down => {}
    }
    // Starting a copy from inside a packaged sandbox would not reach the real profile.
    if let Some(reason) = sandbox_overlay_reason() {
        return Err(sandbox_error(&reason));
    }
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let mut last_exit = None;
    for attempt in 0..2 {
        let mut child = std::process::Command::new(&exe)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .map_err(|e| format!("cannot start Moonpool: {e}"))?;
        let deadline = Instant::now() + START_TIMEOUT;
        loop {
            if ping_state() == PipeState::Up {
                return Ok("Moonpool started".into());
            }
            if let Ok(Some(status)) = child.try_wait() {
                // Gone already: it handed off to a hub that was exiting. One last look in
                // case a hub did come up meanwhile, else go round again.
                if ping_state() == PipeState::Up {
                    return Ok("Moonpool started".into());
                }
                last_exit = Some(status);
                break;
            }
            if Instant::now() >= deadline {
                return Err(format!(
                    "started Moonpool but its control channel did not answer within {}s",
                    START_TIMEOUT.as_secs()
                ));
            }
            std::thread::sleep(Duration::from_millis(200));
        }
        if attempt == 0 {
            std::thread::sleep(Duration::from_secs(1));
        }
    }
    Err(format!(
        "Moonpool exited immediately after starting, twice ({}); the previous instance may \
         still be shutting down - try again in a few seconds",
        last_exit.map(|s| s.to_string()).unwrap_or_default()
    ))
}

/// Shut the resident hub down (same as the tray Quit). Sends `quit` over the control channel,
/// then waits until the channel goes away, so the caller gets a definite "stopped" rather than a
/// fire-and-forget.
fn stop_hub() -> Result<String, String> {
    match ping_state() {
        PipeState::Down => return Ok("Moonpool is not running".into()),
        PipeState::Unreachable => {
            return Err(unreachable_error(
                "a hub owns the channel but is not answering",
            ))
        }
        PipeState::Up => {}
    }
    match control_call("quit", &[]) {
        // The hub usually exits before replying, so a stream error after the write is success.
        Ok(_) | Err(CallError::Io(_)) | Err(CallError::Down) => {}
        Err(CallError::Unreachable(m)) => return Err(unreachable_error(&m)),
    }
    let deadline = Instant::now() + STOP_TIMEOUT;
    let mut last = PipeState::Up;
    while Instant::now() < deadline {
        last = ping_state();
        if last == PipeState::Down {
            return Ok("Moonpool shut down".into());
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    Err(match last {
        PipeState::Unreachable => format!(
            "sent quit but Moonpool's control channel was still held without answering after \
             {}s; the process may be hung",
            STOP_TIMEOUT.as_secs()
        ),
        _ => format!(
            "sent quit but Moonpool was still answering after {}s",
            STOP_TIMEOUT.as_secs()
        ),
    })
}

/// Bring the window to the front. `show` is answered by the window itself and writes no
/// ticket, so there is nothing to wait for. If no hub is running, starting one shows it.
fn raise_launcher() -> Result<String, String> {
    match control_call("show", &[]) {
        Ok(reply) => reply_to_result("show", reply).map(|_| "window shown".into()),
        Err(CallError::Down) => start_hub().map(|_| "Moonpool was not running; started it".into()),
        Err(CallError::Unreachable(m)) => Err(unreachable_error(&m)),
        Err(CallError::Io(m)) => Err(format!("show: control channel failed mid-call: {m}")),
    }
}

/// Registered apps and their live status, asked of the hub itself over the control channel (its
/// in-memory state, never a leftover `state.json`). If the hub is not up there is no list.
fn list_apps() -> Result<String, String> {
    match control_call("list", &[]) {
        Ok(reply) => match reply_to_result("list", reply) {
            Ok(text) => {
                let snap: Value =
                    serde_json::from_str(&text).map_err(|e| format!("bad list reply: {e}"))?;
                Ok(format_apps(&snap))
            }
            // A hub that predates the `list` verb is up (it answered) but cannot be asked;
            // fall back to its state.json, flagged as possibly stale.
            Err(e) if e.starts_with("unknown cmd") => {
                let state = read_state().ok_or_else(|| {
                    "this Moonpool build predates the `list` verb and state.json is unreadable"
                        .to_string()
                })?;
                Ok(format!(
                    "{}\n(from state.json: this Moonpool build predates the `list` verb, so the \
                     data may be stale)",
                    format_apps(&state)
                ))
            }
            Err(e) => Err(e),
        },
        Err(CallError::Down) => Err(not_running_error()),
        Err(CallError::Unreachable(m)) => Err(unreachable_error(&m)),
        Err(CallError::Io(m)) => Err(format!("list: control channel failed mid-call: {m}")),
    }
}

/// Flatten a `{apps, statuses, statusNotReady?}` snapshot (the `list` reply, or `state.json`)
/// into one line per app so an agent reads it without walking two parallel arrays.
fn format_apps(snapshot: &Value) -> String {
    let empty = vec![];
    let apps = snapshot
        .get("apps")
        .and_then(Value::as_array)
        .unwrap_or(&empty);
    let statuses = snapshot
        .get("statuses")
        .and_then(Value::as_array)
        .unwrap_or(&empty);
    if apps.is_empty() {
        return "No apps registered in apps.json.".into();
    }
    let not_ready = snapshot
        .get("statusNotReady")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let mut out = String::new();
    for a in apps {
        let id = a.get("id").and_then(Value::as_str).unwrap_or("?");
        let name = a.get("name").and_then(Value::as_str).unwrap_or(id);
        let st = statuses
            .iter()
            .find(|s| s.get("id").and_then(Value::as_str) == Some(id));
        let flag = |key: &str| {
            st.and_then(|s| s.get(key))
                .and_then(Value::as_bool)
                .unwrap_or(false)
        };
        let managed = flag("managed");
        let mcp_running = flag("mcpRunning");
        let mcp_seen = flag("mcpSeen");
        let run_state = if not_ready {
            "status pending"
        } else if flag("running") {
            "running"
        } else {
            "stopped"
        };
        out.push_str(&format!(
            "{id}  [{run_state}]{}  {name}{}\n",
            if managed {
                " (managed by Moonpool)"
            } else {
                ""
            },
            if mcp_running {
                "  [mcp: running]"
            } else if mcp_seen {
                "  [mcp: stopped]"
            } else {
                ""
            },
        ));
    }
    if not_ready {
        out.push_str("\n(Moonpool has only just started and has not reported app status yet; ask again in a couple of seconds.)");
    } else {
        out.pop(); // trailing newline
    }
    out
}

/// Run the `dump` command, then read back the file it wrote and return its tail.
/// Returning the text (rather than a path) is the whole point of doing this over
/// MCP: the agent gets the console output without a second file-read round trip.
fn dump(id: &str, tail: usize) -> Result<String, String> {
    let path = control("dump", &[id])?;
    let bytes = std::fs::read(&path)
        .map_err(|e| format!("dump pointed at {path} but it could not be read: {e}"))?;
    // The persistent log is raw PTY bytes (ANSI included); strip it here for display.
    let text = crate::strip_ansi(&bytes);
    if text.trim().is_empty() {
        return Ok(format!("(no output recorded for '{id}')"));
    }
    let lines: Vec<&str> = text.lines().collect();
    let start = lines.len().saturating_sub(tail);
    let mut out = String::new();
    if start > 0 {
        out.push_str(&format!(
            "(showing last {tail} of {} lines; full log at {path})\n",
            lines.len()
        ));
    }
    out.push_str(&lines[start..].join("\n"));
    Ok(out)
}

/// Read the current manifest plus a version token through the hub (the hub writes a
/// JSON file; we read it back and return its contents). Going through the hub is the
/// point: the agent gets the file the resident hub actually uses, never a sandbox
/// shadow copy, and a token to feed the compare-and-swap in the write tool.
fn read_config() -> Result<String, String> {
    let path = control("read-config", &[])?;
    std::fs::read_to_string(&path)
        .map_err(|e| format!("read-config wrote {path} but it could not be read: {e}"))
}

/// Replace the manifest through the hub. Stages the new JSON to a temp file the hub
/// reads and validates before committing, passing the token the caller read so a
/// concurrent edit is rejected rather than clobbered. (The temp file is safe here: the
/// sandbox gate in `call_tool` means this only runs when our filesystem view is real.)
fn write_config(manifest: &str, expected_token: &str) -> Result<String, String> {
    let tmp = std::env::temp_dir().join(format!("moonpool-write-{}.json", std::process::id()));
    std::fs::write(&tmp, manifest).map_err(|e| format!("cannot stage manifest: {e}"))?;
    let tmp_str = tmp.to_string_lossy().to_string();
    let result = control("write-config", &[&tmp_str, expected_token]);
    let _ = std::fs::remove_file(&tmp);
    result.map(|token| format!("apps.json updated; new version token {token}"))
}

/// Roll apps.json back to a known-good snapshot through the hub. With no selector the
/// hub writes the ring listing to a file we read back; with one (an index or filename)
/// the hub validates that snapshot and commits it, returning a human summary. Going
/// through the hub keeps this sandbox-proof, exactly like read/write-config.
fn restore_config(selector: Option<&str>) -> Result<String, String> {
    match selector {
        None => {
            let path = control("restore-config", &[])?;
            std::fs::read_to_string(&path)
                .map_err(|e| format!("restore-config wrote {path} but it could not be read: {e}"))
        }
        Some(sel) => control("restore-config", &[sel]),
    }
}

/// Full paths the MCP process itself resolves. Paired with the hub's own report
/// (via `control("paths")`) so a divergence - the MCP reading one apps.json while
/// the resident hub launches from another - is visible at a glance.
fn mcp_paths_report() -> String {
    let dir = crate::portable::data_dir();
    let show = |p: Option<PathBuf>| {
        p.map(|d| d.display().to_string())
            .unwrap_or_else(|| "<unresolved>".into())
    };
    let sub = |name: &str| show(dir.clone().map(|d| d.join(name)));
    format!(
        "mcp config dir: {}\n\
         mcp apps.json:  {}\n\
         mcp state.json: {}\n\
         mcp dumps dir:  {}\n\
         mcp portable:   {}\n\
         mcp exe:        {}",
        show(dir.clone()),
        sub("apps.json"),
        sub("state.json"),
        sub("dumps"),
        crate::portable::is_portable(),
        std::env::current_exe()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| "<unknown>".into()),
    )
}

/// Report every path both the hub and this MCP process use. The hub's list is the
/// authoritative one (it owns launching); the MCP's is shown for comparison.
fn paths_report() -> Result<String, String> {
    let mcp = mcp_paths_report();
    let hub = match control("paths", &[]) {
        Ok(d) => d,
        Err(e) => format!("hub paths unavailable: {e}"),
    };
    Ok(format!("{hub}\n\n{mcp}"))
}

// ---------------------------------------------------------------------------
// Sandbox self-detection
// ---------------------------------------------------------------------------

/// True if a canonicalized path lands inside a Windows Store/MSIX package's
/// per-package overlay (`...\Packages\<pkg>\LocalCache\...`). Kept separate from the
/// path resolution so it can be unit-tested without a real sandbox.
fn is_container_overlay_path(canonical: &str) -> bool {
    let low = canonical.to_ascii_lowercase();
    low.contains("\\packages\\") && low.contains("\\localcache\\")
}

/// Signal 1 of sandbox detection (reliable): THIS `moonpool.exe mcp` process is running inside
/// a packaged (Store/MSIX) sandbox - notably the Claude desktop app's - where AppData is
/// silently redirected to a per-package overlay. The canonicalized data dir or exe path sits in
/// a Store-container overlay. canonicalize() is essential - the raw path looks normal
/// (`%APPDATA%\Moonpool`); only the resolved form reveals the redirect into
/// `...\Packages\<pkg>\LocalCache\...`. Needs no hub and no channel.
fn sandbox_overlay_reason() -> Option<String> {
    let overlay = |p: Option<PathBuf>| -> Option<String> {
        let canonical = std::fs::canonicalize(p?).ok()?;
        let s = canonical.to_string_lossy().to_string();
        is_container_overlay_path(&s).then_some(s)
    };
    if let Some(p) = overlay(crate::portable::data_dir()) {
        return Some(format!(
            "config dir canonicalizes into a Store-container overlay ({p})"
        ));
    }
    if let Some(p) = overlay(std::env::current_exe().ok()) {
        return Some(format!(
            "exe canonicalizes into a Store-container overlay ({p})"
        ));
    }
    None
}

/// Whether the files this process reads diverge from the live hub's (see
/// `sandbox_overlay_reason`). When it is, every file this process reads (state.json,
/// apps.json, dump output) resolves to a private copy the real resident hub never writes, so
/// the file-reading tools would otherwise return stale/empty data with no hint why. Returns a
/// human reason string when sandboxed, so the caller can name what tripped detection.
fn sandbox_reason() -> Option<String> {
    if let Some(reason) = sandbox_overlay_reason() {
        return Some(reason);
    }
    // Signal 2 (divergence fallback): the control channel answers - a hub is up - yet this
    // process cannot read state.json at all. A running hub always writes it to the real data
    // dir, so an unreadable state.json from our view means our filesystem view diverges from
    // the live hub. Narrowed to "unreadable" (not merely empty apps) to avoid a false
    // positive against a genuinely empty manifest with the hub running.
    if hub_alive() && read_state().is_none() {
        return Some(
            "a hub is running but this process cannot read state.json (its file view \
             diverges from the live hub)"
                .to_string(),
        );
    }
    None
}

/// The loud, actionable error the file-reading tools return when the sandbox is detected, so
/// an agent is told exactly why they are blind and what to do instead, rather than acting on
/// wrong/empty results. Tools that only talk over the control channel (list, start/stop,
/// reload...) are not blocked: a pipe/socket is not redirected by the file overlay. See
/// [[claude-msix-appdata-virtualization]].
fn sandbox_error(reason: &str) -> String {
    format!(
        "MoonPool's MCP server is running inside a packaged (Store/MSIX) sandbox - \
         typically the Claude desktop app - so its view of MoonPool's files is a \
         private, stale copy the real resident hub never reads or writes. Tools that \
         read or write files (app output, reading/writing apps.json) therefore CANNOT \
         see or change MoonPool's real state, and if the sandbox also blocks the \
         hub's control channel, no tool can reach the hub.\n\
         Detected via: {reason}.\n\
         Drive MoonPool with `moonpool.exe <verb>` (launch <id> / stop <id> / \
         restart <id> / reload / dump <id> / paths / read-config / show / quit) from a \
         shell OUTSIDE the sandbox instead - that process reaches the real hub and its \
         real files. To see the app list from outside, read `state.json` in the config \
         dir that `moonpool.exe paths` reports."
    )
}

/// Tools that read or write files on this process's own filesystem view (a dump file, the
/// manifest, a staged temp file). Only these are blocked by the sandbox gate.
fn tool_uses_local_files(name: &str) -> bool {
    matches!(
        name,
        "moonpool_app_output"
            | "moonpool_read_config"
            | "moonpool_write_config"
            | "moonpool_restore_config"
    )
}

// ---------------------------------------------------------------------------
// Tool surface
// ---------------------------------------------------------------------------

/// One required app-id argument, the shape most tools take.
fn app_id_schema(verb: &str) -> Value {
    json!({
        "type": "object",
        "properties": {
            "app_id": { "type": "string", "description": format!("The app's `id` from apps.json. Call moonpool_list_apps first if unsure. This is the app to {verb}.") }
        },
        "required": ["app_id"]
    })
}

fn no_args_schema() -> Value {
    json!({ "type": "object", "properties": {} })
}

fn tool_list() -> Value {
    json!([
        {
            "name": "moonpool_list_apps",
            "description": "List every app registered with Moonpool and whether it is currently running. Start here: the other tools need an app id from this list.",
            "inputSchema": no_args_schema()
        },
        {
            "name": "moonpool_bootup_launcher",
            "description": "Start Moonpool itself (the tray launcher). Use this when another tool reported that Moonpool is not running: it boots the launcher so the app tools work. No-op if it is already running.",
            "inputSchema": no_args_schema()
        },
        {
            "name": "moonpool_shutdown_launcher",
            "description": "Shut Moonpool itself down (the tray launcher), same as choosing Quit from its tray menu. No-op if it is not running.",
            "inputSchema": no_args_schema()
        },
        {
            "name": "moonpool_start_app",
            "description": "Start an app and open its terminal tab. Returns once it is actually running, or with the reason it did not start. If Moonpool itself is not running, call moonpool_bootup_launcher first.",
            "inputSchema": app_id_schema("start")
        },
        {
            "name": "moonpool_stop_app",
            "description": "Stop a running app (kills the process tree Moonpool owns). Returns once it is actually stopped.",
            "inputSchema": app_id_schema("stop")
        },
        {
            "name": "moonpool_stop_mcp_server",
            "description": "Kill the app's attached MCP shim process (a `<processName> mcp` subprocess an MCP host spawned to reach this app's own tools), leaving the app itself untouched. There is no matching 'start' - the shim isn't something Moonpool launches; the MCP host that owns it respawns it on its own next tool call.",
            "inputSchema": app_id_schema("stop the MCP shim for")
        },
        {
            "name": "moonpool_restart_app",
            "description": "Stop an app, wait for its port and process to free, then start it again. Prefer this over a stop followed by a start - it does the waiting for you. Use it after changing the app's code or config.",
            "inputSchema": app_id_schema("restart")
        },
        {
            "name": "moonpool_app_output",
            "description": "Read an app's terminal output (the current run, ANSI stripped). Use this to see why an app failed to start, or what it logged, without opening the Moonpool window.",
            "inputSchema": json!({
                "type": "object",
                "properties": {
                    "app_id": { "type": "string", "description": "The app's `id` from apps.json." },
                    "tail_lines": { "type": "integer", "description": format!("How many trailing lines to return (default {DEFAULT_TAIL}). Raise it only if the tail is not enough - the full log can be ~512 KB.") }
                },
                "required": ["app_id"]
            })
        },
        {
            "name": "moonpool_reload_config",
            "description": "Re-read apps.json. Call this after editing the manifest so Moonpool picks up added or changed apps.",
            "inputSchema": no_args_schema()
        },
        {
            "name": "moonpool_read_config",
            "description": "Read the apps.json manifest the launcher actually uses, plus a version token. Returns JSON: manifest_text (the file's exact contents), token (pass back to moonpool_write_config), valid (whether it parses) and error (the validation error if not). Always read through this tool before editing - never read apps.json off disk yourself, as the launcher's copy may differ from what you can see.",
            "inputSchema": no_args_schema()
        },
        {
            "name": "moonpool_write_config",
            "description": "Replace apps.json with new contents, through the launcher. The launcher VALIDATES the new manifest first and rejects it (file left untouched) with the exact error if invalid, so you cannot brick it. Pass expected_token from moonpool_read_config: if the file changed since you read it, the write is rejected as stale - re-read and reapply. On success the launcher reloads the new manifest.",
            "inputSchema": json!({
                "type": "object",
                "properties": {
                    "manifest": { "type": "string", "description": "The full new apps.json content (a JSON array of app entries) as text." },
                    "expected_token": { "type": "string", "description": "The token from your most recent moonpool_read_config. The write commits only if apps.json still matches it." }
                },
                "required": ["manifest", "expected_token"]
            })
        },
        {
            "name": "moonpool_restore_config",
            "description": "Roll apps.json back to a previous known-good version from the launcher's history ring (the launcher snapshots every validated manifest change). Call with NO argument to list the saved snapshots (newest first, each with an index, filename, timestamp and app count); call again with `snapshot` set to an index (1 = newest) or a filename to restore that one. The chosen snapshot is validated before it is written, so a corrupt one is refused and apps.json is left untouched. On success the launcher reloads the restored manifest.",
            "inputSchema": json!({
                "type": "object",
                "properties": {
                    "snapshot": { "type": "string", "description": "Which snapshot to restore: an index (1 = newest) or a filename, both from the no-argument listing. Omit to list the available snapshots instead of restoring." }
                }
            })
        },
        {
            "name": "moonpool_refresh_app_icons",
            "description": "Re-fetch every app icon.",
            "inputSchema": no_args_schema()
        },
        {
            "name": "moonpool_raise_launcher",
            "description": "Bring the Moonpool window to the front (it lives in the tray).",
            "inputSchema": no_args_schema()
        },
        {
            "name": "moonpool_screenshot",
            "description": "Capture Moonpool's own window content (not the screen) as an inline PNG image, e.g. to check that a UI change actually rendered. Scoped to Moonpool's own windows only - it cannot capture any other app.",
            "inputSchema": json!({
                "type": "object",
                "properties": {
                    "window": { "type": "string", "enum": crate::ALL_WINDOWS, "description": "Which Moonpool window to capture. Defaults to \"main\" (the hub) if omitted." }
                }
            })
        },
        {
            "name": "moonpool_window_state",
            "description": "Report one of Moonpool's own windows' geometry and visibility (open, visible, minimized, maximized, x/y/width/height), as JSON text. For asserting window state in tests without eyeballing a screenshot - e.g. confirming a window did not collapse to a sliver, or that it actually opened.",
            "inputSchema": json!({
                "type": "object",
                "properties": {
                    "window": { "type": "string", "enum": crate::ALL_WINDOWS, "description": "Which Moonpool window to inspect. Defaults to \"main\" (the hub) if omitted." }
                }
            })
        },
        {
            "name": "moonpool_reset_mcp_seen",
            "description": "Clear the sticky 'this app's MCP shim has been seen' record, so the sidebar's MCP row goes back to not showing for that app until its shim is observed again. Test-only: normal operation never clears this. Omit app_id to clear every app's record.",
            "inputSchema": json!({
                "type": "object",
                "properties": {
                    "app_id": { "type": "string", "description": "The app's `id` from apps.json. Omit to clear every app's seen record." }
                }
            })
        },
        {
            "name": "moonpool_launcher_paths",
            "description": "Report the full filesystem paths Moonpool is using - config dir, apps.json, state.json, log, dumps - for BOTH the resident launcher (the authoritative one that launches apps) and this MCP process. Use it when an edit to apps.json is not taking effect, to confirm which file the launcher actually reads.",
            "inputSchema": no_args_schema()
        }
    ])
}

/// Server-level guidance, surfaced by MCP clients so an agent knows when to reach
/// for these tools at all rather than hunting for an app's raw launch command.
const INSTRUCTIONS: &str = "\
Moonpool is the user's tray launcher: it holds the launch command, working directory, and \
environment for each of their local apps and dev servers, and runs each one in its own terminal.

When a task involves starting, stopping, restarting, or checking one of the user's local apps \
or dev servers, drive it through these tools instead of reconstructing its command line - \
Moonpool already knows how to run it, and running it any other way risks a second copy on the \
same port. The usual loop: moonpool_list_apps to find the id, moonpool_restart_app after a \
code change, then moonpool_app_output to read what it printed.";

/// An `app_id` supplied by an MCP client is forwarded to a fresh `moonpool.exe`
/// process as a positional command-line token. Restrict it to the character set
/// real manifest ids use so it can never be read as a launcher flag (an id of
/// `--uninstall` would otherwise appear in the child's argv) and can never carry
/// shell/whitespace surprises across the process boundary. This is the primary
/// guard; `startup_mode` in lib.rs (which only honours flags in argv[1]) is the
/// second, independent layer.
fn is_valid_app_id(id: &str) -> bool {
    !id.is_empty()
        && !id.starts_with('-')
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
}

/// Dispatch one tool call. Returns the text the agent sees.
fn call_tool(name: &str, args: &Value) -> Result<String, String> {
    // Fail loudly if we are sandboxed, but only for the tools that read or write files: those
    // would act on a private overlay of MoonPool's files and return misleading "results". The
    // rest go over the control channel (a pipe/socket, not shadowed by the file overlay); if
    // the sandbox does block it they report that via `not_running_error`.
    if tool_uses_local_files(name) {
        if let Some(reason) = sandbox_reason() {
            return Err(sandbox_error(&reason));
        }
    }
    let id = || -> Result<String, String> {
        let raw = args
            .get("app_id")
            .and_then(Value::as_str)
            .ok_or_else(|| "app_id is required".to_string())?;
        if !is_valid_app_id(raw) {
            return Err(
                "invalid app_id: use only letters, digits, '.', '_', '-' (no leading '-')".into(),
            );
        }
        Ok(raw.to_string())
    };
    match name {
        "moonpool_list_apps" => list_apps(),
        "moonpool_bootup_launcher" => start_hub(),
        "moonpool_shutdown_launcher" => stop_hub(),
        "moonpool_start_app" => control("launch", &[&id()?]).map(|_| "launched".into()),
        "moonpool_stop_app" => control("stop", &[&id()?]).map(|_| "stopped".into()),
        "moonpool_restart_app" => control("restart", &[&id()?]).map(|_| "restarted".into()),
        "moonpool_stop_mcp_server" => control("stop-mcp", &[&id()?]).map(|_| "stopped".into()),
        "moonpool_app_output" => {
            let tail = args
                .get("tail_lines")
                .and_then(Value::as_u64)
                .map(|n| n.max(1) as usize)
                .unwrap_or(DEFAULT_TAIL);
            dump(&id()?, tail)
        }
        "moonpool_reload_config" => control("reload", &[]).map(|_| "apps.json reloaded".into()),
        "moonpool_read_config" => read_config(),
        "moonpool_write_config" => {
            let manifest = args
                .get("manifest")
                .and_then(Value::as_str)
                .ok_or_else(|| "manifest (the full apps.json text) is required".to_string())?;
            let token = args.get("expected_token").and_then(Value::as_str).ok_or_else(|| {
                "expected_token is required - call moonpool_read_config first and pass back its token".to_string()
            })?;
            if token.is_empty() {
                return Err(
                    "expected_token must not be empty; get it from moonpool_read_config".into(),
                );
            }
            write_config(manifest, token)
        }
        "moonpool_restore_config" => {
            let sel = args
                .get("snapshot")
                .and_then(Value::as_str)
                .filter(|s| !s.is_empty());
            restore_config(sel)
        }
        "moonpool_screenshot" => {
            let window = args.get("window").and_then(Value::as_str).unwrap_or("main");
            if !crate::ALL_WINDOWS.contains(&window) {
                return Err(format!(
                    "unknown window '{window}' - expected one of {:?}",
                    crate::ALL_WINDOWS
                ));
            }
            control("screenshot", &[window])
        }
        "moonpool_window_state" => {
            let window = args.get("window").and_then(Value::as_str).unwrap_or("main");
            if !crate::ALL_WINDOWS.contains(&window) {
                return Err(format!(
                    "unknown window '{window}' - expected one of {:?}",
                    crate::ALL_WINDOWS
                ));
            }
            control("window-state", &[window])
        }
        "moonpool_reset_mcp_seen" => {
            let app_id = args
                .get("app_id")
                .and_then(Value::as_str)
                .filter(|s| !s.is_empty());
            match app_id {
                Some(id) => control("reset-mcp-seen", &[id]),
                None => control("reset-mcp-seen", &[]),
            }
        }
        "moonpool_launcher_paths" => paths_report(),
        "moonpool_refresh_app_icons" => {
            control("refresh-icons", &[]).map(|_| "icons refreshed".into())
        }
        "moonpool_raise_launcher" => raise_launcher(),
        other => Err(format!("unknown tool '{other}'")),
    }
}

// ---------------------------------------------------------------------------
// JSON-RPC plumbing
// ---------------------------------------------------------------------------

fn result(id: Value, result: Value) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "result": result })
}

fn error(id: Value, code: i64, message: &str) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
}

/// A tool result. Failures come back as `isError` content rather than a JSON-RPC
/// error, per MCP: the model is meant to read and act on them, not just see the
/// call fail.
fn tool_result(id: Value, text: String, is_error: bool) -> Value {
    result(
        id,
        json!({
            "content": [{ "type": "text", "text": text }],
            "isError": is_error
        }),
    )
}

/// Like `tool_result`, but the success payload is an inline base64 PNG image content block
/// rather than text - lets the MCP client render it directly instead of a giant text blob it
/// would have to decode by hand. `moonpool_screenshot` is the only tool that returns this shape.
fn image_tool_result(id: Value, png_base64: String) -> Value {
    result(
        id,
        json!({
            "content": [{ "type": "image", "data": png_base64, "mimeType": "image/png" }],
            "isError": false
        }),
    )
}

fn handle(req: &Value) -> Option<Value> {
    let method = req.get("method").and_then(Value::as_str).unwrap_or("");
    // A request has an id; a notification does not, and must not be answered.
    let id = match req.get("id") {
        Some(v) if !v.is_null() => v.clone(),
        _ => return None,
    };
    match method {
        "initialize" => Some(result(
            id,
            json!({
                "protocolVersion": PROTOCOL_VERSION,
                "capabilities": { "tools": {} },
                "serverInfo": { "name": "moonpool", "version": env!("CARGO_PKG_VERSION") },
                "instructions": INSTRUCTIONS
            }),
        )),
        "ping" => Some(result(id, json!({}))),
        "tools/list" => Some(result(id, json!({ "tools": tool_list() }))),
        "tools/call" => {
            let params = req.get("params").cloned().unwrap_or_else(|| json!({}));
            let name = params.get("name").and_then(Value::as_str).unwrap_or("");
            let args = params
                .get("arguments")
                .cloned()
                .unwrap_or_else(|| json!({}));
            Some(match call_tool(name, &args) {
                Ok(text) if name == "moonpool_screenshot" => image_tool_result(id, text),
                Ok(text) => tool_result(id, text, false),
                Err(e) => tool_result(id, e, true),
            })
        }
        // Empty lists rather than "method not found": clients probe for these even
        // when the capability was not advertised.
        "resources/list" => Some(result(id, json!({ "resources": [] }))),
        "prompts/list" => Some(result(id, json!({ "prompts": [] }))),
        _ => Some(error(id, -32601, "method not found")),
    }
}

/// Blocking stdio loop. Runs until the client closes stdin.
///
/// Nothing may be printed to stdout except JSON-RPC - a stray line breaks the
/// framing and the client drops the connection.
pub fn serve() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        let response = match serde_json::from_str::<Value>(&line) {
            Ok(req) => handle(&req),
            Err(_) => Some(error(Value::Null, -32700, "parse error")),
        };
        if let Some(resp) = response {
            if writeln!(stdout, "{resp}").is_err() {
                break;
            }
            let _ = stdout.flush();
        }
    }
}

#[cfg(test)]
mod app_id_tests {
    use super::{is_container_overlay_path, is_valid_app_id, read_json_with_retry};
    use std::time::Duration;

    #[test]
    fn flags_store_container_overlay_paths() {
        // The canonicalized redirect that bit us on 2026-09-11.
        assert!(is_container_overlay_path(
            r"\\?\C:\Users\user\AppData\Local\Packages\Claude_pzs8sxrjxfjjc\LocalCache\Roaming\Moonpool\state.json"
        ));
        // Case-insensitive.
        assert!(is_container_overlay_path(
            r"C:\...\PACKAGES\Some_Pkg\LOCALCACHE\Roaming\Moonpool"
        ));
        // A normal install path must not trip it.
        assert!(!is_container_overlay_path(
            r"\\?\C:\Users\user\AppData\Roaming\Moonpool\state.json"
        ));
        // LocalCache without a Packages segment (or vice versa) is not the overlay.
        assert!(!is_container_overlay_path(r"C:\foo\LocalCache\bar"));
        assert!(!is_container_overlay_path(r"C:\foo\Packages\bar"));
    }

    #[test]
    fn rejects_flags_and_shell_surprises() {
        for bad in [
            "",
            "--uninstall",
            "--wait-pid",
            "-x",
            "--ticket",
            "a b",
            "a/b",
            "a;b",
            "a\"b",
            "a$b",
            "app\n",
        ] {
            assert!(!is_valid_app_id(bad), "{bad:?} must be rejected");
        }
    }

    #[test]
    fn accepts_real_manifest_ids() {
        for ok in ["web", "my-app", "app_2", "docs.v3", "A1", "a-b_c.d"] {
            assert!(is_valid_app_id(ok), "{ok:?} must be accepted");
        }
    }

    #[test]
    fn retries_a_transient_snapshot_parse_failure() {
        let mut reads = 0;
        let value = read_json_with_retry(
            || {
                reads += 1;
                Some(if reads < 3 {
                    "{incomplete".into()
                } else {
                    r#"{"tickets":[]}"#.into()
                })
            },
            4,
            Duration::ZERO,
        )
        .unwrap();

        assert_eq!(reads, 3);
        assert_eq!(value["tickets"].as_array().unwrap().len(), 0);
    }
}

#[cfg(test)]
mod pipe_reply_tests {
    use super::reply_to_result;
    use serde_json::json;

    #[test]
    fn ok_reply_with_string_result_passes_through() {
        let r = reply_to_result("dump", json!({ "ok": true, "result": "wrote file.log" }));
        assert_eq!(r, Ok("wrote file.log".to_string()));
    }

    #[test]
    fn ok_reply_with_null_result_becomes_empty_string() {
        // launch/stop/restart/reload/refresh-icons report no detail on success.
        let r = reply_to_result("reload", json!({ "ok": true, "result": null }));
        assert_eq!(r, Ok(String::new()));
    }

    #[test]
    fn error_reply_surfaces_the_message() {
        let r = reply_to_result(
            "restart",
            json!({ "ok": false, "error": "app not found: bogus" }),
        );
        assert_eq!(r, Err("app not found: bogus".to_string()));
    }

    #[test]
    fn error_reply_with_no_message_falls_back_to_a_generic_one() {
        let r = reply_to_result("restart", json!({ "ok": false }));
        assert_eq!(r, Err("restart failed".to_string()));
    }

    #[test]
    fn malformed_reply_missing_ok_field_is_treated_as_failure() {
        // Guards against a future control.rs change that forgets `ok` - silently
        // treating it as success would surface a wrong result to an MCP caller.
        let r = reply_to_result("ping", json!({ "result": "pong" }));
        assert_eq!(r, Err("ping failed".to_string()));
    }
}

#[cfg(test)]
mod channel_tests {
    use super::{
        call_at, classify_io, format_apps, ping_at, tool_uses_local_files, CallError, PipeState,
        NON_HUB_TOKENS,
    };
    use crate::control::{serve_at, Handler, LogFn, CONTROL_VERBS};
    use serde_json::{json, Value};
    use std::io::{Error, ErrorKind};
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn classifies_missing_and_refused_as_down() {
        assert_eq!(
            classify_io(&Error::from(ErrorKind::NotFound)),
            PipeState::Down
        );
        assert_eq!(
            classify_io(&Error::from(ErrorKind::ConnectionRefused)),
            PipeState::Down
        );
        // ERROR_FILE_NOT_FOUND (Windows) / ENOENT (unix): both are raw OS error 2.
        assert_eq!(classify_io(&Error::from_raw_os_error(2)), PipeState::Down);
    }

    #[test]
    fn classifies_busy_timeout_and_denied_as_unreachable() {
        // ERROR_PIPE_BUSY (231) must never read as "no hub": something owns the pipe.
        #[cfg(windows)]
        assert_eq!(
            classify_io(&Error::from_raw_os_error(231)),
            PipeState::Unreachable
        );
        assert_eq!(
            classify_io(&Error::from(ErrorKind::TimedOut)),
            PipeState::Unreachable
        );
        assert_eq!(
            classify_io(&Error::from(ErrorKind::PermissionDenied)),
            PipeState::Unreachable
        );
    }

    #[test]
    fn every_verb_of_both_dispatchers_is_a_non_hub_token() {
        for v in CONTROL_VERBS.iter().chain(crate::ARGV_VERBS) {
            assert!(
                NON_HUB_TOKENS.contains(v),
                "{v} must be in NON_HUB_TOKENS or a helper running it reads as a hub"
            );
        }
        for startup in [
            "mcp",
            "--uninstall",
            "--wait-pid",
            "--ticket",
            "open-window",
        ] {
            assert!(NON_HUB_TOKENS.contains(&startup), "{startup} missing");
        }
    }

    #[test]
    fn only_file_reading_tools_are_sandbox_gated() {
        for t in [
            "moonpool_app_output",
            "moonpool_read_config",
            "moonpool_write_config",
            "moonpool_restore_config",
        ] {
            assert!(tool_uses_local_files(t), "{t}");
        }
        for t in [
            "moonpool_list_apps",
            "moonpool_start_app",
            "moonpool_raise_launcher",
            "moonpool_screenshot",
            "moonpool_launcher_paths",
        ] {
            assert!(!tool_uses_local_files(t), "{t}");
        }
    }

    fn fixture() -> Value {
        json!({
            "apps": [
                { "id": "web", "name": "Web UI" },
                { "id": "api" },
                { "id": "cold" }
            ],
            "statuses": [
                { "id": "web", "running": true, "managed": true, "mcpRunning": true },
                { "id": "api", "running": false, "mcpSeen": true }
            ]
        })
    }

    #[test]
    fn formats_one_line_per_app_without_a_hub_footer() {
        let out = format_apps(&fixture());
        assert_eq!(
            out,
            "web  [running] (managed by Moonpool)  Web UI  [mcp: running]\n\
             api  [stopped]  api  [mcp: stopped]\n\
             cold  [stopped]  cold"
        );
        assert!(!out.contains("Hub:"));
    }

    #[test]
    fn formats_empty_manifest() {
        assert_eq!(
            format_apps(&json!({ "apps": [], "statuses": [] })),
            "No apps registered in apps.json."
        );
    }

    #[test]
    fn formats_status_not_ready_as_pending_not_stopped() {
        let snap = json!({
            "apps": [{ "id": "web", "name": "Web" }],
            "statuses": [],
            "statusNotReady": true
        });
        let out = format_apps(&snap);
        assert!(out.starts_with("web  [status pending]  Web"), "{out}");
        assert!(!out.contains("[stopped]"));
        assert!(out.contains("not reported app status yet"));
    }

    /// A unique, test-only endpoint: never the real `\\.\pipe\moonpool` / hub socket.
    fn test_endpoint(tag: &str) -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let name = format!("moonpool-test-{tag}-{}-{nanos}", std::process::id());
        #[cfg(windows)]
        {
            PathBuf::from(format!(r"\\.\pipe\{name}"))
        }
        #[cfg(not(windows))]
        {
            std::env::temp_dir().join(format!("{name}.sock"))
        }
    }

    #[test]
    fn missing_endpoint_is_down() {
        assert_eq!(ping_at(&test_endpoint("absent")), PipeState::Down);
        assert_eq!(
            call_at(
                &test_endpoint("absent2"),
                "list",
                &[],
                Duration::from_secs(2)
            ),
            Err(CallError::Down)
        );
    }

    /// Ping / list / quit through the real transport (named pipe on Windows, Unix socket
    /// elsewhere) and the shared connection loop, with a fake handler standing in for the hub.
    #[test]
    fn transport_round_trips_ping_list_and_quit() {
        let endpoint = test_endpoint("rt");
        let quit_seen = Arc::new(AtomicBool::new(false));
        let handler: Handler = {
            let quit_seen = quit_seen.clone();
            Arc::new(move |req| {
                let quit_seen = quit_seen.clone();
                Box::pin(async move {
                    match req.cmd.as_str() {
                        "ping" => json!({ "ok": true, "result": "pong" }),
                        "list" => {
                            let snap = json!({ "apps": [], "statuses": [] });
                            json!({ "ok": true, "result": snap.to_string() })
                        }
                        "echo" => json!({ "ok": true, "result": req.args.join(",") }),
                        "quit" => {
                            quit_seen.store(true, Ordering::SeqCst);
                            json!({ "ok": true, "result": Value::Null })
                        }
                        other => json!({ "ok": false, "error": format!("unknown cmd: {other}") }),
                    }
                })
            })
        };
        let log: LogFn = Arc::new(|_| {});

        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .unwrap();
        {
            let endpoint = endpoint.clone();
            rt.spawn(async move {
                let _ = serve_at(&endpoint, handler, log).await;
            });
        }

        // Wait for the server to come up (the pipe/socket appears once it binds).
        let mut up = false;
        for _ in 0..40 {
            if ping_at(&endpoint) == PipeState::Up {
                up = true;
                break;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
        assert!(up, "test server never answered ping");

        let t = Duration::from_secs(3);
        let reply = call_at(&endpoint, "list", &[], t).unwrap();
        assert_eq!(reply["ok"], json!(true));
        let snap: Value = serde_json::from_str(reply["result"].as_str().unwrap()).unwrap();
        assert_eq!(snap["apps"], json!([]));

        let reply = call_at(&endpoint, "echo", &["a", "b"], t).unwrap();
        assert_eq!(reply["result"], json!("a,b"));

        let reply = call_at(&endpoint, "nope", &[], t).unwrap();
        assert_eq!(reply["ok"], json!(false));

        let reply = call_at(&endpoint, "quit", &[], t).unwrap();
        assert_eq!(reply["ok"], json!(true));
        assert!(quit_seen.load(Ordering::SeqCst));

        // Dropping the runtime tears the server down; the endpoint then reads as Down (unix
        // leaves a stale socket file, which refuses the connection).
        drop(rt);
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(ping_at(&endpoint), PipeState::Down);
        #[cfg(unix)]
        let _ = std::fs::remove_file(&endpoint);
    }
}
