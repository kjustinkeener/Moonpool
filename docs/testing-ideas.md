# Testing ideas via the agent control surface

Moonpool's pipe/MCP control surface (`src-tauri/src/control_pipe.rs`, `mcp.rs`) is
also a test-automation surface: everything an agent can do to the running app, a
test can do too, without driving the actual GUI. This doc tracks test ideas and
which verbs they need.

## Already fully testable with existing verbs

- **App lifecycle**: `moonpool_start_app` / `stop_app` / `restart_app`, polling
  `moonpool_list_apps` for `running`. Covers the golden path and stop/restart races.
- **Config round-trip**: `moonpool_read_config` / `write_config` / `restore_config`
  already enforce optimistic-concurrency (`expected_token`) and validate before
  writing - good coverage for manifest persistence and the "stale write rejected"
  path without any new verb.
- **Icon refresh**: `moonpool_refresh_app_icons` + `moonpool_screenshot`.
- **Output assertions**: `moonpool_app_output` for stdout/stderr text checks (e.g.
  "did the app log its startup banner").
- **External-kill detection**: kill an app's process from outside Moonpool, then
  poll `moonpool_list_apps` and confirm `running` flips to false within one status
  poll tick. No verb needed - this is exactly what the poller is for.
- **MCP shim lifecycle**: call any of that app's own MCP tools to spawn its shim,
  confirm `mcp: running` via `moonpool_list_apps`, call `moonpool_stop_mcp_server`,
  confirm it flips to `mcp: stopped` while staying visible (mcpSeen sticky). Shims
  are spawned LAZILY by the MCP host on first tool call per session/connection -
  there is deliberately no "start/restart shim" verb, since Moonpool never owns
  that lifecycle.

## New verbs added for testing (this pass)

- **`window-state <label>`** / `moonpool_window_state`: geometry + visibility
  (open/visible/minimized/maximized/x/y/width/height) for any window in
  `ALL_WINDOWS`. Lets a test assert window state directly - e.g. regression-guard
  the minimize/restore collapse bug (see `moonpool-window-minimize` memory) - instead
  of eyeballing a screenshot every time.
- **`reset-mcp-seen [id]`** / `moonpool_reset_mcp_seen`: clears the sticky
  `mcp_seen` record (one app or all). Normal operation never clears it, so without
  this a test run can only ever observe the "seen" sidebar state once per machine,
  never the "never seen" state again.

## Ideas not yet built

- **Generic setting passthrough** (`set-setting <key> <value>` or similar): the
  Settings window currently only writes settings through dedicated
  `#[tauri::command]`s (`set_locale`, `set_theme`, `set_always_on_top`, etc.), none
  exposed on the pipe. A locale/theme sweep test (screenshot all 14 locales x 18
  themes looking for Settings-window overflow, per `moonpool-localization` memory)
  would need either one verb per setting or a generic key/value verb. Not started -
  scope it against the settings list in `moonpool-settings-plumbing` memory before
  picking a shape.
- **Multi-window open trigger**: `moonpool_screenshot`/`moonpool_window_state`
  can inspect `installer`/`editor`/`about` once open, but nothing on the pipe
  currently *opens* them (they're triggered from tray/menu clicks in the UI). A
  visual smoke-test sweep across all `ALL_WINDOWS` needs a way to open each one
  first - candidate verb, not built.
