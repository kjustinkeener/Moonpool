---
title: "Moonpool MCP tools reference: parameters and results"
description: "Every tool the Moonpool MCP server exposes for agents, with its parameters, what it returns, and the error cases you may see."
---

All tools return text, except `moonpool_screenshot`, which returns a PNG image. A failure
comes back as a tool result flagged as an error, with the reason as text. For setup see
[MCP setup](/automation/mcp-setup/).

Tools that take `app_id` need the app's `id` from `apps.json`. It must use only letters,
digits, `.`, `_` and `-`, and not start with `-`, otherwise the call fails with "invalid
app_id".

Most tools that act on the hub fail with this message when it is not running.
`moonpool_bootup_launcher`, `moonpool_shutdown_launcher`, `moonpool_raise_launcher` and
`moonpool_launcher_paths` handle that case themselves (see their rows). For a portable copy
the message names the copy, for example `Moonpool (<folder>)`.

```text
Moonpool is not running - call moonpool_bootup_launcher first
```

Calls that wait for an outcome time out after 45 seconds.

## Launcher and apps

Example `moonpool_list_apps` result:

```text
site  [running] (managed by Moonpool)  Site
notes-app  [stopped]  [mcp: stopped]  Notes App
```

| Tool | Parameters | Behavior |
| --- | --- | --- |
| `moonpool_list_apps` | none | One line per app: `id  [running]` or `[stopped]`, `(managed by Moonpool)` when applicable, `[mcp: running]` or `[mcp: stopped]` when an MCP helper has been seen, then the name. Asked of the running hub over the control channel (`list` verb), so it is live. If Moonpool is not running it fails with "Moonpool is not running" rather than showing a stale list. Just after Moonpool starts, before its first status check, apps show `[status pending]`. While `apps.json` has an error, the result starts with `apps.json has an error: <message>. This list is the last one that loaded; fix the file and call moonpool_reload_config.` If the file was already broken when Moonpool started, it says no apps are loaded and suggests `moonpool_restore_config` as well. |
| `moonpool_bootup_launcher` | none | Starts Moonpool itself and waits up to 30 s for its control channel to answer. Returns "Moonpool started", or "Moonpool is already running". If the new process exits straight away (it handed off to a Moonpool that was still shutting down), it starts one more. If something holds the channel without answering, it reports that a Moonpool process may be hung. |
| `moonpool_shutdown_launcher` | none | Same as Quit in the tray menu. Waits up to 30 s for the control channel to go away. Returns "Moonpool shut down", or "Moonpool is not running". |
| `moonpool_raise_launcher` | none | Brings the Moonpool window to the front. Returns "window shown". If Moonpool is not running, it starts it and returns "Moonpool was not running; started it". |
| `moonpool_start_app` | `app_id` (required) | Starts the app and opens its terminal tab. Returns "launched" once it is running, or the reason it was not (`unknown app id: <id>`, `did not reach running in time` after 25 s). For a `static` entry with only a `url`, it opens the page and also returns "launched". |
| `moonpool_stop_app` | `app_id` (required) | Stops the app. Returns "stopped", or an error such as `still running after stop` (after 15 s). |
| `moonpool_restart_app` | `app_id` (required) | Stop, wait for the port and process to free, start. Returns "restarted". |
| `moonpool_app_output` | `app_id` (required), `tail_lines` (integer, default 200, minimum 1) | The app's terminal output for the current Moonpool session, ANSI codes removed. When the log is longer than `tail_lines`, the text starts with a line giving the full log's path. Fails with `no console output recorded for '<id>' (not launched this session)` if the app has not run. If the log exists but is empty, returns `(no output recorded for '<id>')`. |
| `moonpool_stop_mcp_server` | `app_id` (required) | Kills the app's attached MCP helper process and leaves the app running. Returns "stopped". Does nothing if the app has neither `processName` nor `mcpProcessName`. |
| `moonpool_refresh_app_icons` | none | Re-fetches every app icon. Returns "icons refreshed". |

## Configuration

These read and change `apps.json` through the hub, never the file on disk. A write must
carry the token from the last read, a stale token is rejected, and the new file is validated
before anything is written. Going through the hub matters because an agent in a sandboxed
host can be shown a private copy of the config folder instead of the real one.

| Tool | Parameters | Behavior |
| --- | --- | --- |
| `moonpool_read_config` | none | JSON text with `manifest_text` (the file's exact contents), `token`, `valid`, `error` (null when valid) and `path`. `token` is `none` when the file is missing or empty. |
| `moonpool_write_config` | `manifest` (required, the full new `apps.json` text), `expected_token` (required, from the last read) | Validates the manifest and replaces `apps.json`, then loads it. Returns `apps.json updated; new version token <token>`. A stale token fails with `stale token: apps.json changed since it was read ...`. An invalid manifest fails with `rejected invalid manifest: ...`. Either way the file is untouched. An empty `expected_token` is refused. |
| `moonpool_restore_config` | `snapshot` (optional) | With no value, JSON text listing the saved snapshots newest first (`index`, `filename`, `millis`, `app_count`, `valid`). With an index (1 = newest) or a filename, validates that snapshot and restores it. Returns `restored <file> (<n> apps); new version token <token>`. No token is needed: a restore overwrites the current file on purpose. |
| `moonpool_reload_config` | none | Re-reads `apps.json`. Returns "apps.json reloaded". If the file does not parse or validate, it fails with `apps.json has an error: ...` and Moonpool keeps the last list that loaded. |
| `moonpool_launcher_paths` | none | Lists the hub's config folder, `apps.json`, `state.json`, log, dumps folder, icons folder, portable flag and exe path, then the MCP process's config folder, `apps.json`, `state.json`, dumps folder, portable flag and exe path (no log or icons). If the hub is not running, its half reads `hub paths unavailable: ...` and the MCP half is still shown. Use it when an edit does not take effect. |

## Advanced: testing tools

`moonpool_screenshot` is Windows only; on Linux it fails with "screenshot is not
supported on this platform". `moonpool_window_state` and `moonpool_reset_mcp_seen` work on
every platform.

`window` is one of `main`, `settings`, `about`, `installer`, `editor`, `help` or `themes`, and defaults
to `main`. An unknown name fails with `unknown window '<name>'`.

| Tool | Parameters | Behavior |
| --- | --- | --- |
| `moonpool_screenshot` | `window` (optional) | Captures that Moonpool window's own content as an inline PNG, at most 320 pixels on its longer side. The size cannot be raised from MCP. Fails with `window '<name>' is not open` if it is not showing. It cannot capture any other app. |
| `moonpool_window_state` | `window` (optional) | JSON text: `{"open":false}` when the window is not open, otherwise `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. Intended for tests. |
| `moonpool_reset_mcp_seen` | `app_id` (optional) | Test only. Clears the remembered "an MCP helper was seen" record for one app, or for every app when omitted, so the sidebar's MCP sub-row hides again until a helper is seen. |

## See also

- [MCP setup](/automation/mcp-setup/)
- [Command line](/automation/command-line/)
