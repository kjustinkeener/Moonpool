---
title: Command line
description: Drive a running Moonpool with moonpool.exe verbs, tag a command with a ticket, and read the outcome from state.json.
---

Running a `moonpool.exe` again while that same Moonpool is already running does not open a
second window. The second process passes its arguments to the running one over its
[control channel](/automation/control-verbs/) and exits. Moonpool must already be running:
with nothing resident, the same command starts a new Moonpool and the verb is not run.

"Same Moonpool" means the same folder. The installed Moonpool and every portable copy each
run on their own, so a command reaches the copy whose `moonpool.exe` you ran, never another
one. See [Portable mode](/guides/portable-mode/#several-copies-at-once).

Use the path of the copy you mean. For the installed one:

```powershell frame="terminal"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
```

With several copies running, `Get-Process moonpool` lists all of them, so pick by `Path`
rather than taking the first. It also lists idle `moonpool.exe mcp` helpers that MCP hosts
started, so a `moonpool` process does not prove a hub is running. Ask the control channel
with `ping` instead ([Control verbs](/automation/control-verbs/)).

## Verbs

The verb is not case sensitive. `<id>` is an app's `id` from `apps.json`.

| Command | Effect |
| --- | --- |
| `moonpool.exe` | No verb: brings the window to the front. |
| `moonpool.exe show` | Brings the window to the front. |
| `moonpool.exe launch <id>` | Starts the app and opens its terminal tab. |
| `moonpool.exe stop <id>` | Stops the app. |
| `moonpool.exe restart <id>` | Stop, wait for the port and process to free, start. |
| `moonpool.exe reload` | Re-reads `apps.json`. |
| `moonpool.exe refresh-icons` | Re-fetches every icon. |
| `moonpool.exe help` | Opens the Help window. |
| `moonpool.exe quit` | Quits Moonpool, same as the tray menu. |
| `moonpool.exe dump <id> [out-path]` | Without `out-path`, reports the path of the app's log for this session. With it, copies the log there as plain text with ANSI codes removed. |
| `moonpool.exe paths` | Reports the config folder, `apps.json`, `state.json`, log, dumps folder, icons folder, portable flag and exe path the running Moonpool uses. |
| `moonpool.exe read-config` | Writes `dumps\read-config.json` in the config folder, holding `token`, `valid`, `error`, `path` and `manifest_text` (the exact contents of `apps.json`). |
| `moonpool.exe write-config <file> [token]` | Replaces `apps.json` with the manifest in `<file>`, if the manifest is valid and, when `token` is given, `apps.json` still matches it. |
| `moonpool.exe restore-config [index or filename]` | With no argument, writes the snapshot list to `dumps\restore-config.json`. With one, restores that snapshot if it is valid. |

```powershell frame="terminal"
& $mp restart my-app
```

```powershell frame="terminal"
& $mp dump my-app C:\temp\my-app.log
```

An unknown verb is ignored. The program also has startup arguments of its own:
`moonpool.exe mcp` ([MCP setup](/automation/mcp-setup/)), `--uninstall` (used by Add/Remove
Programs) and `--wait-pid <pid>` (used when Moonpool relaunches itself). These are honored
only as the first argument, so an app id such as `--uninstall` cannot trigger them.

## Reading the outcome

The command line prints nothing, so tag a command with `--ticket <key>` (any unique key, in
any position) and read the result from `state.json` in the config folder. That is
`%USERPROFILE%\.moonpool\moonpool-config\` installed, `<your .moonpool folder>\moonpool-config\`
for a portable copy, and `~/.config/Moonpool/` on Linux (see
[Configuration overview](/configuration/overview/#where-the-config-lives)). `show` and `quit`
write no ticket.

```powershell frame="terminal"
& $mp launch my-app --ticket t1
```

`state.json` has `apps`, `statuses` (`id`, `running`, `managed`, `mcpRunning`, `mcpSeen` per
app) and `tickets`. The running Moonpool rewrites it every couple of seconds and after each
command, and does not delete it when it quits, so a leftover file does not mean Moonpool is
running. To ask whether it is, or to get the live app list, use the control channel's `ping`
and `list` verbs ([Control verbs](/automation/control-verbs/)) or the MCP tools. Poll your
ticket until `status` is not `pending`:

| `status` | Meaning |
| --- | --- |
| `pending` | Received; Moonpool is still acting on it. |
| `ok` | Done. For `dump`, `read-config`, `write-config`, `restore-config` and `paths`, `detail` holds the path, token or report. |
| `error` | Failed; `detail` says why, for example `unknown app id: x`, `did not reach running in time`, `unknown command`. |

Each ticket is `{ ticket, action, arg, status, detail, ts }` with `ts` in Unix milliseconds:

```json title="state.json (tickets entry)"
{
  "ticket": "t1",
  "action": "launch",
  "arg": "my-app",
  "status": "error",
  "detail": "did not reach running in time",
  "ts": 1767225600000
}
```

Finished tickets are dropped after 24 hours, and the list is trimmed toward 50 entries once
finished tickets are at least 5 minutes old.

An agent that supports MCP can skip the polling: see [MCP setup](/automation/mcp-setup/).
