---
title: "Give an AI agent (Claude Code, Codex, Cursor) an MCP server to start and stop local apps"
description: "Register Moonpool as an MCP server so Claude Code, Codex or Cursor can start, stop, restart and read the output of your dev servers without extra copies."
---

An AI coding agent usually runs your dev server by typing `npm run dev` into its own shell.
That can block the agent, leave an orphaned process holding the port, or start a second copy
of something you already have running. An MCP server lets the agent call tools to start and
stop the app you have already configured instead of reconstructing its command line.

## The Moonpool way

Moonpool's executable is its own MCP server: register `moonpool.exe` with the single
argument `mcp` as a stdio server. Once the app is in `apps.json`, the agent starts it by id.

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

Register the server. In Claude Code, one command (installed Moonpool; use the full path of
your own exe):

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

Hosts that read a JSON file of MCP servers, such as Cursor's `mcp.json`, take the same shape
(backslashes doubled):

```json title="mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

For Codex, add a server with the same command and `mcp` argument in its config
(`~/.codex/config.toml`):

```toml title="config.toml"
[mcp_servers.moonpool]
command = 'C:\Users\you\.moonpool\moonpool.exe'
args = ["mcp"]
```

The exact file and key names belong to each host, so check its MCP documentation if your
version differs. What Moonpool needs is only the full path to `moonpool.exe` and `mcp` as
the argument. Restart the host afterwards.

## What the agent can do

The tools appear as `moonpool_*`. The ones for day-to-day work:

| Tool | Use |
| --- | --- |
| `moonpool_list_apps` | Find an app's id and see whether it is running. |
| `moonpool_start_app` | Start an app by id and open its terminal tab. |
| `moonpool_stop_app` | Stop it, including its child processes. |
| `moonpool_restart_app` | Stop, wait for the port to free, start. Use after a code change. |
| `moonpool_app_output` | Read what the app printed, with `tail_lines` to limit it. |
| `moonpool_bootup_launcher` | Start Moonpool itself if it is not running. |

A typical loop is `moonpool_restart_app`, then `moonpool_app_output`. The rest of the tools
(reading and writing `apps.json`, screenshots) are in [MCP tools](/automation/mcp-tools/).

## If it does not work

Every tool saying `Moonpool is not running - call moonpool_bootup_launcher first` means
Moonpool is not started yet. An edit that does not show up usually means the agent is
looking at a different `apps.json`: call `moonpool_launcher_paths`. See
[If the tools do not work](/automation/mcp-setup/#if-the-tools-do-not-work).

## See also

- [MCP setup](/automation/mcp-setup/)
- [MCP tools](/automation/mcp-tools/)
- [AI agents: quick start](/automation/quick-start/)
- [Run an npm dev server in the background on Windows](/guides/run-npm-dev-server-in-background-windows/)
