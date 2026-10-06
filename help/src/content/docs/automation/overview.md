---
title: Automation overview
description: The three ways to drive a running Moonpool from scripts and AI agents, how they relate, and what each one can change.
---

Moonpool can be driven without touching its window. There are three surfaces, all served by
the same resident Moonpool (the tray instance, called the hub here).

| Surface | What it is | Reference |
| --- | --- | --- |
| MCP server | `moonpool.exe mcp`, a stdio [MCP](https://modelcontextprotocol.io) server that an AI host starts. | [MCP setup](/automation/mcp-setup/), [MCP tools](/automation/mcp-tools/) |
| Command line | `moonpool.exe <verb> [args]`. A second run hands the verb to the hub and exits. | [Command line](/automation/command-line/) |
| Control pipe | A named pipe, `\\.\pipe\moonpool`, speaking one JSON request per line. Windows only. | [Control verbs](/automation/control-verbs/) |

## How they relate

- The hub owns everything: launching apps, the terminal logs, `apps.json`.
- The MCP server is a client of the hub, not a second copy of it. Each tool call is
  forwarded to the hub over the control pipe, and the reply comes back as the tool result.
  If the pipe cannot be reached, it falls back to running `moonpool.exe <verb>` and
  waiting for the outcome in `state.json`.
- Every surface runs the same handlers as the window, so a verb does what the matching click
  does.
- If no hub is running, the tools that act on it refuse with "Moonpool is not running".
  `moonpool_bootup_launcher` starts it. `moonpool_list_apps` still answers from the last
  `state.json` and says the hub is not running.

## What can change things

| Can change | Surfaces |
| --- | --- |
| Start, stop or restart an app | MCP, command line, pipe |
| Rewrite `apps.json` | MCP (`moonpool_write_config`, `moonpool_restore_config`), command line, pipe |
| Quit Moonpool | MCP (`moonpool_shutdown_launcher`), command line (`quit`), pipe |
| Kill an app's MCP helper process | MCP (`moonpool_stop_mcp_server`), pipe (`stop-mcp`) |

Read-only tools: `moonpool_list_apps`, `moonpool_app_output`, `moonpool_read_config`,
`moonpool_launcher_paths`, `moonpool_window_state`, `moonpool_screenshot`.

## Safety properties

- **Config writes are guarded.** A write must carry the version token from the last read, a
  stale token is rejected, and the new manifest is validated before anything is written. A
  rejected write leaves `apps.json` untouched. See [Configuration](/configuration/overview/#agents).
- **App ids are restricted.** The MCP server accepts only letters, digits, `.`, `_` and `-`,
  and never a leading `-`, so an id cannot be read as a command-line flag.
- **Screenshots are Moonpool only.** `moonpool_screenshot` captures one of Moonpool's own six
  windows (`main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`), never the screen or
  another app. The PNG is built in memory and returned inline; Moonpool does not save it to a
  file.
- **No authentication on the pipe.** Moonpool adds no login or token to the control pipe. Any
  process that can open it can send verbs.
- **Sandboxed hosts are detected.** If the MCP server finds it is running inside a packaged
  (Store/MSIX) sandbox, where it would see a private copy of Moonpool's files, every tool
  returns an error explaining why instead of stale data. See
  [MCP setup](/automation/mcp-setup/#sandboxed-hosts).

## Platform

The pipe, screenshots, window state and the MCP-helper verbs are Windows only. On other
platforms the command line verbs and the MCP server still work through the
`moonpool.exe <verb>` path.
