---
title: MCP setup
description: Register moonpool.exe mcp with an MCP host, installed or portable, and how Moonpool tracks an app's own MCP helper.
---

Moonpool's executable is its own MCP server. Register it with the host as a stdio server
that runs `moonpool.exe` with the single argument `mcp`.

## Register the server

Installed, the program is:

```text
%USERPROFILE%\.moonpool\moonpool.exe
```

Portable, it is the `moonpool.exe` inside your `.moonpool\` folder. Use that full path as
`command`. For a host that reads a `.mcp.json`:

```json title=".mcp.json" {5}
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

In a JSON file the backslashes must be doubled, as above. A host with a command-line
registration, such as Claude Code, can add it in one step:

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

The server announces itself as
`moonpool`, speaks MCP protocol revision `2025-06-18`, and exposes tools only (it lists no
resources or prompts). Tools appear to the agent as `moonpool_*`; see
[MCP tools](/automation/mcp-tools/).

## Notes

- `moonpool.exe mcp` never opens a window and never starts the installer. It exits when the
  host closes its input.
- It uses the config folder of the exe it was started from, so a portable exe reads the
  portable folder's data.
- Most tools need a running Moonpool. If it is not running, the agent can call
  `moonpool_bootup_launcher` first.
- `moonpool_launcher_paths` shows the folders the hub uses next to the ones the MCP process
  resolves. A difference means the agent is looking at a different `apps.json` than the hub.

## Sandboxed hosts

Some hosts run their tools inside a packaged (Store/MSIX) sandbox that redirects AppData to a
private per-package copy. Moonpool detects this when its config folder or exe resolves under
a path like this:

```text
...\Packages\<package>\LocalCache\...
```

It also detects it when the control channel answers but `state.json` cannot be read. The
tools that read or write files (`moonpool_app_output`, `moonpool_read_config`,
`moonpool_write_config`, `moonpool_restore_config`) then return an error that names the
cause, rather than empty or stale data. Tools that only use the control channel, such as
`moonpool_list_apps`, are not blocked while the channel is reachable. If the sandbox also
hides the channel, the tools report the sandbox instead of "Moonpool is not running".
Use the [command line](/automation/command-line/) from a shell outside the sandbox instead.

## Apps that have their own MCP server

Many apps in Moonpool are themselves reached by an MCP host through an `<exe> mcp` helper
process. Moonpool looks for a process whose name matches the app's `processName` and whose
first argument is `mcp`, such as:

```text
notes-app.exe mcp
```

- While one is attached, the app's sidebar shows an **MCP** sub-item as running, and
  `moonpool_list_apps` appends `[mcp: running]` to the app's line. The helper does not count
  as the app itself running.
- Once a helper has been seen, Moonpool remembers it (in `mcp_seen.json` in the config
  folder), so the sub-item stays visible as stopped, and `moonpool_list_apps` shows
  `[mcp: stopped]`, after the helper exits.
- The sub-item is controlled by the `showMcpProcesses` setting
  ([Settings and logs](/configuration/settings-and-logs/)).
- `moonpool_stop_mcp_server` kills the helper and leaves the app alone. There is no start
  counterpart: the host that owns the helper starts it again on its next tool call.
