---
title: Automation overview
description: The three ways to drive a running Moonpool from scripts and AI agents, how they relate, and what each one can change.
---

Moonpool can be driven without touching its window. There are three surfaces, all served by
the same resident Moonpool (the tray instance, called the hub here).

Each Moonpool copy is its own hub: the installed one and every portable copy run
independently, each with its own control channel. A surface always reaches the copy whose
`moonpool.exe` it uses. See [Portable mode](/data/portable-mode/#several-copies-at-once).

| Surface | What it is | Reference |
| --- | --- | --- |
| MCP server | `moonpool.exe mcp`, a stdio [MCP](https://modelcontextprotocol.io) server that an AI host starts. | [MCP setup](/automation/mcp-setup/), [MCP tools](/automation/mcp-tools/) |
| Command line | `moonpool.exe <verb> [args]`. A second run of the same copy hands the verb to its hub over the control channel and exits. | [Command line](/automation/command-line/) |
| Control channel | A named pipe, `\\.\pipe\moonpool` (`\\.\pipe\moonpool-<id>` for a portable copy), on Windows and a Unix socket on Linux and macOS, speaking one JSON request per line. | [Control verbs](/automation/control-verbs/) |

## How they relate

- The hub owns everything: launching apps, the session logs, `apps.json`.
- The MCP server is a client of the hub, not a second copy of it. Most tool calls are
  forwarded to the hub over the control channel, and the reply comes back as the tool
  result. The exceptions: `moonpool_bootup_launcher` starts `moonpool.exe` itself;
  `moonpool_app_output` and the config tools ask the hub to write a file and then read it;
  `moonpool_launcher_paths` adds the MCP process's own paths to the hub's.
- Whether a hub is running is decided by pinging that channel, not by looking for a
  process. A hub that answers is running; a missing pipe or socket means it is not.
- Every surface runs the same handlers as the window, so a verb does what the matching click
  does.
- If no hub is running, the tools that act on it, including `moonpool_list_apps`, refuse with
  "Moonpool is not running". There is no stale list. `moonpool_bootup_launcher` starts it.
  If something holds the channel but does not answer within a few seconds, the error says a
  Moonpool process may be hung.
- The MCP server no longer falls back to driving a hub build that predates the control
  channel. Update that copy, or quit it and start it again.

## What can change things

| Can change | Surfaces |
| --- | --- |
| Start, stop or restart an app | MCP, command line, pipe |
| Rewrite `apps.json` | MCP (`moonpool_write_config`, `moonpool_restore_config`), command line, pipe |
| Quit Moonpool | MCP (`moonpool_shutdown_launcher`), command line (`quit`), pipe |
| Kill an app's MCP helper process | MCP (`moonpool_stop_mcp_server`), pipe (`stop-mcp`) |
| Reload `apps.json`, re-fetch icons, show the window | MCP (`moonpool_reload_config`, `moonpool_refresh_app_icons`, `moonpool_raise_launcher`), command line (`reload`, `refresh-icons`, `show`), pipe |
| Open a window or a terminal tab | pipe (`open-window`) |
| Clear remembered MCP helper sightings | MCP (`moonpool_reset_mcp_seen`), pipe (`reset-mcp-seen`) |

Read-only tools: `moonpool_list_apps`, `moonpool_app_output`, `moonpool_read_config`,
`moonpool_launcher_paths`, `moonpool_window_state`, `moonpool_screenshot`.

## Safety properties

- **Config writes are guarded.** A write must carry the version token from the last read, a
  stale token is rejected, and the new `apps.json` is validated before anything is written. A
  rejected write leaves `apps.json` untouched. See [MCP tools](/automation/mcp-tools/#configuration).
- **App ids are restricted.** The MCP server accepts only letters, digits, `.`, `_` and `-`,
  and never a leading `-`, so an id cannot be read as a command-line flag.
- **Screenshots are Moonpool only.** `moonpool_screenshot` captures one of Moonpool's own six
  windows (`main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`), never the screen or
  another app. The PNG is built in memory and returned inline; Moonpool does not save it to a
  file.
- **No authentication on the channel.** Moonpool adds no login or token to the control pipe or
  socket. Any process that can open it can send verbs. On Linux and macOS the socket file is
  created with mode `0600`, so only your own user can.
- **Sandboxed hosts are detected.** If the MCP server finds it is running inside a packaged
  (Store/MSIX) sandbox, where it would see a private copy of Moonpool's files, the tools that
  read or write files (`moonpool_app_output`, `moonpool_read_config`,
  `moonpool_write_config`, `moonpool_restore_config`) return an error explaining why instead
  of stale data. Tools that only use the control channel are not blocked. See
  [MCP setup](/automation/mcp-setup/#sandboxed-hosts).

## Platform

The control channel exists on every platform: a named pipe on Windows, a Unix socket on Linux
and macOS (location in [Control verbs](/automation/control-verbs/#where-it-listens)). Only
`screenshot` (and so `moonpool_screenshot`) is Windows only; on Linux and macOS it returns
"not supported on this platform". The command line verbs work on every platform.

## See also

- [AI agents: quick start](/automation/quick-start/)
- [MCP setup](/automation/mcp-setup/)
