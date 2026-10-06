---
title: "Moonpool error messages explained: already running, requires a command and more"
description: "Look up the exact text of Moonpool error messages, such as already running, requires a command, stale token and Update failed, with what each means and the fix."
---

Paste the message you see into the page search, or scan the tables. Messages are quoted as
Moonpool shows them. Text in `<angle brackets>` is replaced by a value (an app id, a path or
an error from the system). Symptoms that are not an error message are on
[Troubleshooting and FAQ](/support/troubleshooting/).

## Starting and stopping an app

| Message | Meaning and fix |
| --- | --- |
| `already running` | Moonpool already holds a terminal for this app. Stop it first, or use Restart. |
| `stopped during launch` | Stop was pressed while the launch was still starting. Launch again. |
| `app has no launch command` | The entry has no `command`. Add one in the app editor or in `apps.json`. Only a `static` entry with a `url` can go without. |
| `unknown app: <id>` | No app with that `id` is loaded. Check the id, then Reload if you edited `apps.json` by hand. |
| `unknown app id: <id>` | The same problem, reported to a script or agent. List the apps with `moonpool_list_apps`. |
| `did not reach running in time` | From a script or agent: the app did not read as Running within 25 seconds. Check `port` or `processName`, and read the output. See [The status dot is wrong](/support/troubleshooting/#the-status-dot-is-wrong). |
| `still running after stop` | After 15 seconds the app still reads as Running. Set `killMode`. See [Stop and restart](/apps/stop-and-restart/). |
| `refusing to open non-web url: <url>` | The `url` is not `http://`, `https://`, `mailto:` or `file://`. Fix the `url`. |
| `[process exited]` | Not an error: the app's command ended. Shown in the terminal tab. |

## apps.json validation

Moonpool rejects an `apps.json` that breaks a rule and keeps the last list that loaded.
`<n>` is the entry's position in the file, counting from 1.

| Message | Fix |
| --- | --- |
| `apps.json entry <n> has invalid id "<id>"; use letters, digits, '.', '_', and '-' without a leading '-'` | Rename the `id`. |
| `duplicate app id "<id>"` | Two entries share an `id`. Make each unique. |
| `apps.json entry <n> (<id>) has an empty name` | Fill in `name`. |
| `apps.json entry <n> (<id>) has an empty group` | Fill in `group`. |
| `apps.json entry <n> (<id>) has unknown type "<type>"` | `type` must be `web`, `desktop`, `static` or `cli`. |
| `apps.json entry <n> (<id>) has invalid port 0` | `port` must be 1 to 65535. |
| `apps.json entry <n> (<id>) requires a url` | A `static` entry needs a `url`. |
| `apps.json entry <n> (<id>) requires a command` | Every other type needs a `command`. |

In the app editor, saving without a name shows `name is required.`.
The banner text, "apps.json has an error, showing the last list that loaded." or "apps.json
has an error, so no apps are loaded.", and how to recover are under
[apps.json has an error](/support/troubleshooting/#appsjson-has-an-error). If the banner says
saving is paused, the message ends `Repair apps.json and reload it before saving from
Moonpool`. The full rule list is in [Validation](/apps/apps-json/#validation).

## Settings, updates and the installer

| Message | Meaning and fix |
| --- | --- |
| `... Repair settings.json and restart Moonpool before changing settings` | `settings.json` is malformed. Fix or delete it and restart. See [settings.json](/data/settings-json/#reading-and-repair). |
| `Update failed: <error>` | The download or install of an update failed. See [When an update fails](/data/updating/#when-an-update-fails). |
| `Update check failed: <error>` | The update check in About failed. The text after the colon says why. Try again later. |
| `Install failed: <error>` | The installer stopped at the step named after the colon, for example `copy exe: ...`. Quit any Moonpool running from `%USERPROFILE%\.moonpool` and retry. |
| `target folder does not exist` | The folder picked for a portable copy is gone. Pick an existing one. |

## MCP and scripts

| Message | Meaning and fix |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Start Moonpool, or let the agent call that tool. For a portable copy the message names the copy. |
| `frontend not loaded` | The hub window has not finished loading. Wait and retry. |
| `invalid app_id: use only letters, digits, '.', '_', '-' (no leading '-')` | The agent passed an id the MCP server will not accept. Use the id from `moonpool_list_apps`. |
| `stale token: apps.json changed since it was read ...` | Read `apps.json` again, reapply the edit, then write. |
| `rejected invalid manifest: ...` | The new `apps.json` failed validation (see above). The file was not changed. |
| `no console output recorded for '<id>' (not launched this session)` | `moonpool_app_output` was asked for an app that has not run since Moonpool started. |

More in [MCP tools](/automation/mcp-tools/) and
[MCP setup](/automation/mcp-setup/#if-the-tools-do-not-work).

## Errors from other programs

- [`Error: listen EADDRINUSE: address already in use :::3000` and `Port 5173 is in use`](/support/port-already-in-use/)
- [`Windows protected your PC`](/support/windows-protected-your-pc/)
- [WebView2 runtime missing](/support/webview2-runtime-missing/)
