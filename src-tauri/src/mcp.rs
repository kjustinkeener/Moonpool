//! `moonpool.exe mcp` - an MCP (Model Context Protocol) server over stdio, so an
//! agent can drive Moonpool with tools instead of shelling out and polling files.
//!
//! Why it lives in this exe rather than a companion Node package: it is the same
//! binary the user already installed, so there is nothing extra to build, install,
//! or keep in sync with the commands it exposes.
//!
//! What it is NOT: the hub. This process is a *client* of the resident tray
//! instance, driving it through the very control channel documented in
//! AI-README.md - spawn ourselves with `<action> <id> --ticket <key>`, then poll
//! `state.json` until that ticket resolves. The payoff over calling the exe
//! directly is that the poll happens here, so every tool call returns the real
//! outcome synchronously instead of leaving the caller to watch a file.
//!
//! The protocol is hand-rolled: MCP over stdio is newline-delimited JSON-RPC 2.0,
//! serde_json is already a dependency, and a full async SDK (plus tokio) would
//! outweigh the seven small tools below.
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

/// Moonpool's config directory, resolved WITHOUT an `AppHandle` (we never build a
/// Tauri app in this process). Mirrors `portable::data_dir`: beside the exe in a
/// portable bundle, `%APPDATA%\Moonpool` when installed.
fn data_dir() -> Option<PathBuf> {
    if crate::portable::is_portable() {
        return crate::portable::exe_dir().map(|d| d.join("moonpool-config"));
    }
    #[cfg(windows)]
    {
        std::env::var_os("APPDATA").map(|d| PathBuf::from(d).join("Moonpool"))
    }
    #[cfg(not(windows))]
    {
        std::env::var_os("HOME").map(|d| PathBuf::from(d).join(".config").join("Moonpool"))
    }
}

fn state_path() -> Option<PathBuf> {
    data_dir().map(|d| d.join("state.json"))
}

fn read_state() -> Option<Value> {
    let text = std::fs::read_to_string(state_path()?).ok()?;
    serde_json::from_str(&text).ok()
}

/// Whether a Moonpool window is resident to receive commands. Checked before every
/// action: without it, spawning the exe would start a NEW instance that opens a
/// window and ignores the argv (the single-instance callback only fires in the
/// *second* process), which looks like a silent no-op to the caller.
fn hub_running() -> bool {
    use sysinfo::{ProcessRefreshKind, RefreshKind, System};
    let me = std::process::id();
    let sys = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::new()),
    );
    sys.processes().iter().any(|(pid, p)| {
        pid.as_u32() != me
            && p.name()
                .to_ascii_lowercase()
                .starts_with("moonpool")
    })
}

// ---------------------------------------------------------------------------
// Driving the control channel
// ---------------------------------------------------------------------------

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

/// Fire one control command at the resident instance and block until its ticket
/// resolves. Returns `Ok(detail)` on success (detail is whatever the handler
/// reported - for `dump`, the file path it wrote) or `Err(message)`.
fn control(action: &str, args: &[&str]) -> Result<String, String> {
    if !hub_running() {
        return Err("Moonpool is not running - start it first (its tray icon must be live)".into());
    }
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
    let status = cmd.status().map_err(|e| format!("cannot run Moonpool: {e}"))?;
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

/// Registered apps and their live status, flattened into one line per app so an
/// agent reads it without walking two parallel arrays.
fn list_apps() -> Result<String, String> {
    let state = read_state().ok_or_else(|| {
        "no state.json - Moonpool has not run on this machine (or is too old)".to_string()
    })?;
    let empty = vec![];
    let apps = state.get("apps").and_then(Value::as_array).unwrap_or(&empty);
    let statuses = state
        .get("statuses")
        .and_then(Value::as_array)
        .unwrap_or(&empty);
    if apps.is_empty() {
        return Ok("No apps registered in apps.json.".into());
    }
    let mut out = String::new();
    for a in apps {
        let id = a.get("id").and_then(Value::as_str).unwrap_or("?");
        let name = a.get("name").and_then(Value::as_str).unwrap_or(id);
        let st = statuses
            .iter()
            .find(|s| s.get("id").and_then(Value::as_str) == Some(id));
        let running = st
            .and_then(|s| s.get("running"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let managed = st
            .and_then(|s| s.get("managed"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        out.push_str(&format!(
            "{id}  [{}]{}  {name}\n",
            if running { "running" } else { "stopped" },
            if managed { " (managed by Moonpool)" } else { "" },
        ));
    }
    out.push_str(&format!(
        "\nHub: {}",
        if hub_running() {
            "running"
        } else {
            "NOT running - commands will fail until it is started"
        }
    ));
    Ok(out)
}

/// Run the `dump` command, then read back the file it wrote and return its tail.
/// Returning the text (rather than a path) is the whole point of doing this over
/// MCP: the agent gets the console output without a second file-read round trip.
fn dump(id: &str, tail: usize) -> Result<String, String> {
    let path = control("dump", &[id])?;
    let text = std::fs::read_to_string(&path)
        .map_err(|e| format!("dump wrote {path} but it could not be read: {e}"))?;
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

// ---------------------------------------------------------------------------
// Tool surface
// ---------------------------------------------------------------------------

/// One required app-id argument, the shape most tools take.
fn app_id_schema(verb: &str) -> Value {
    json!({
        "type": "object",
        "properties": {
            "app_id": { "type": "string", "description": format!("The app's `id` from apps.json. Call moonpool_list first if unsure. This is the app to {verb}.") }
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
            "name": "moonpool_list",
            "description": "List every app registered with Moonpool and whether it is currently running. Start here: the other tools need an app id from this list.",
            "inputSchema": no_args_schema()
        },
        {
            "name": "moonpool_launch",
            "description": "Start an app and open its terminal tab. Returns once it is actually running, or with the reason it did not start.",
            "inputSchema": app_id_schema("start")
        },
        {
            "name": "moonpool_stop",
            "description": "Stop a running app (kills the process tree Moonpool owns). Returns once it is actually stopped.",
            "inputSchema": app_id_schema("stop")
        },
        {
            "name": "moonpool_restart",
            "description": "Stop an app, wait for its port and process to free, then start it again. Prefer this over a stop followed by a launch - it does the waiting for you. Use it after changing the app's code or config.",
            "inputSchema": app_id_schema("restart")
        },
        {
            "name": "moonpool_dump",
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
            "name": "moonpool_reload",
            "description": "Re-read apps.json. Call this after editing the manifest so Moonpool picks up added or changed apps.",
            "inputSchema": no_args_schema()
        },
        {
            "name": "moonpool_refresh_icons",
            "description": "Re-fetch every app icon.",
            "inputSchema": no_args_schema()
        },
        {
            "name": "moonpool_show",
            "description": "Bring the Moonpool window to the front (it lives in the tray).",
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
same port. The usual loop: moonpool_list to find the id, moonpool_restart after a code change, \
then moonpool_dump to read what it printed.";

/// Dispatch one tool call. Returns the text the agent sees.
fn call_tool(name: &str, args: &Value) -> Result<String, String> {
    let id = || -> Result<String, String> {
        args.get("app_id")
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .ok_or_else(|| "app_id is required".to_string())
    };
    match name {
        "moonpool_list" => list_apps(),
        "moonpool_launch" => control("launch", &[&id()?]).map(|_| "launched".into()),
        "moonpool_stop" => control("stop", &[&id()?]).map(|_| "stopped".into()),
        "moonpool_restart" => control("restart", &[&id()?]).map(|_| "restarted".into()),
        "moonpool_dump" => {
            let tail = args
                .get("tail_lines")
                .and_then(Value::as_u64)
                .map(|n| n.max(1) as usize)
                .unwrap_or(DEFAULT_TAIL);
            dump(&id()?, tail)
        }
        "moonpool_reload" => control("reload", &[]).map(|_| "apps.json reloaded".into()),
        "moonpool_refresh_icons" => control("refresh-icons", &[]).map(|_| "icons refreshed".into()),
        // `show` is answered by the window itself and writes no ticket, so there is
        // nothing to wait for.
        "moonpool_show" => {
            if !hub_running() {
                return Err("Moonpool is not running".into());
            }
            let exe = std::env::current_exe().map_err(|e| e.to_string())?;
            std::process::Command::new(exe)
                .arg("show")
                .status()
                .map_err(|e| e.to_string())?;
            Ok("window shown".into())
        }
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
