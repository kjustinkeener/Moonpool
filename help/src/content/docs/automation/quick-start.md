---
title: "Let an AI agent set up and drive Moonpool: quick start"
description: "Three ways to let an AI agent or a script set up and drive Moonpool, which one to pick for your agent, and the same action shown in each."
---

There are three ways in. Pick by what your agent can do.

| You want | Use | Start here |
| --- | --- | --- |
| An agent to find your apps and add them, once | **Copy prompt** on the hub's empty screen | Below |
| An agent to start, stop and read apps as tool calls | The MCP server, `moonpool.exe mcp` | [MCP setup](/automation/mcp-setup/) |
| A script, or an agent without MCP | Command-line verbs | [Command line](/automation/command-line/) |

## Copy prompt

With no tab open, the CLI pane shows a ready-made prompt ("New here? Hand this to an AI
agent to set up your apps"). **Copy prompt** puts it on the clipboard. Paste it into your
agent. It points the agent at `AI-README.md` and `apps.json` in your config folder and asks
it to find your apps and register them. When it is done, choose **Reload**.

Moonpool rewrites `AI-README.md` next to `apps.json` at each launch, so it always matches
the version you run. Do not keep your own edits in it.

## The same action three ways

| Action | Command line | Control channel verb | MCP tool |
| --- | --- | --- | --- |
| Start an app | `moonpool.exe launch <id>` | `launch` | `moonpool_start_app` |
| Stop an app | `moonpool.exe stop <id>` | `stop` | `moonpool_stop_app` |
| Restart an app | `moonpool.exe restart <id>` | `restart` | `moonpool_restart_app` |
| Read an app's output | `moonpool.exe dump <id> [out-path]` | `dump` | `moonpool_app_output` |
| List apps and status | read `state.json` | `list` | `moonpool_list_apps` |
| Re-read `apps.json` | `moonpool.exe reload` | `reload` | `moonpool_reload_config` |
| Read `apps.json` | `moonpool.exe read-config` | `read-config` | `moonpool_read_config` |
| Replace `apps.json` | `moonpool.exe write-config <file> [token]` | `write-config` | `moonpool_write_config` |
| Roll back `apps.json` | `moonpool.exe restore-config [n]` | `restore-config` | `moonpool_restore_config` |
| Show the window | `moonpool.exe show` | `show` | `moonpool_raise_launcher` |
| Start Moonpool | `moonpool.exe` | none | `moonpool_bootup_launcher` |
| Quit Moonpool | `moonpool.exe quit` | `quit` | `moonpool_shutdown_launcher` |
| Show the folders in use | `moonpool.exe paths` | `paths` | `moonpool_launcher_paths` |

The command line prints nothing; read the outcome with a `--ticket` (see
[Reading the outcome](/automation/command-line/#reading-the-outcome)). The channel and MCP
answer directly.

## When an agent's tools fail

- `Moonpool is not running - call moonpool_bootup_launcher first`: start Moonpool, or let the
  agent call that tool.
- An edit "did not take": ask the agent for `moonpool_launcher_paths`. If the hub and MCP
  folders differ, the agent is reading a different `apps.json`. See
  [Sandboxed hosts](/automation/mcp-setup/#sandboxed-hosts).
- Several Moonpool copies: register each under its own name. See
  [More than one Moonpool](/automation/mcp-setup/#more-than-one-moonpool).

More symptoms in [Troubleshooting](/support/troubleshooting/#mcp-and-script-errors).
