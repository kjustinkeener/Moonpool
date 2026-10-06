---
title: "Moonpool glossary: apps, states, files and settings"
description: "Plain definitions of the words Moonpool help uses for its parts, app states, files and settings, so you can follow the rest of the docs."
---

## Apps

| Term | Meaning |
| --- | --- |
| app | One thing Moonpool manages: a dev server, a desktop app, a page or a command. |
| entry | An app's record in `apps.json`. Used only when talking about the JSON. |
| app row | An app's line in the sidebar, with its status dot and controls. |
| group | The sidebar heading an app is listed under, from its `group` field. |
| type | `web`, `desktop`, `static` or `cli`. Decides which fields matter. See [App types](/apps/types/). |
| id | An app's permanent key, used in file names, commands and agent tools. See [The id](/apps/apps-json/#the-id). |

## App states

| State | Meaning |
| --- | --- |
| starting | Moonpool launched the app but has not seen it up yet. Pulsing dot. |
| Running | Its `port` answers, its `processName` exists, or, with neither set, the terminal Moonpool started is still alive. Solid dot. See [How Running is decided](/apps/types/#how-running-is-decided). |
| stopped | None of the above. Grey dot. |
| managed | Moonpool started it in this session. A running app that is not managed was started some other way, and Quit leaves it alone. |

A terminal tab and a running app are separate things. Clicking an app's name only opens its
terminal tab; it never starts the app. Closing a tab never stops the app.

## Windows and parts

| Term | Meaning |
| --- | --- |
| hub | The resident Moonpool process and its main window. Tool names call it the "launcher". |
| hub window | The main window: sidebar on the left, CLI pane on the right. |
| tray | The system tray icon and its menu (**Show Moonpool**, **Quit**). |
| sidebar | The left side of the hub window: filter box, **...** menu and app rows. |
| CLI pane | The right side of the hub window, holding the terminal tabs. |
| terminal tab | One app's terminal in the CLI pane. |
| MCP sub-row | A dimmed row under an app showing its own `<exe> mcp` helper process. |
| app editor | The Add app and Edit app dialog. |

## Files and folders

| Term | Meaning |
| --- | --- |
| config folder | The folder holding `apps.json` and Moonpool's other files. The `{MP_DATA}` token. See [Where the config lives](/apps/apps-json/#where-the-config-lives). |
| `{MP_HOME}` | The Moonpool folder: `%USERPROFILE%\.moonpool` installed, the `.moonpool\` folder of a portable copy, the config folder on Linux. |
| session | One run of the hub, from start to Quit. |
| session log | The file holding everything an app printed during one session, under `cli-output\`. See [Logs](/data/logs/). |
| `moonpool.log` | Moonpool's own debug log, written only with **Log debug info to a file** on. |
| dump | A plain-text copy of a session log made by the `dump` verb. |
| snapshot | A copy of a good `apps.json` in `apps.json.history\`. See [Backup and recovery](/data/backup-and-recovery/). |

## Modes

| Term | Meaning |
| --- | --- |
| installed | A Moonpool in `%USERPROFILE%\.moonpool`, with Start Menu shortcut and Add/Remove entry. Windows only. |
| portable | A Moonpool in a `.moonpool\` folder you chose, marked by a `moonpool.portable` file. See [Portable mode](/data/portable-mode/). |
| copy | One Moonpool folder, installed or portable. Each copy runs on its own. |

## Stop and automation

| Term | Meaning |
| --- | --- |
| `killMode` | The extra step Stop takes after ending the app's terminal. See [Stop and restart](/apps/stop-and-restart/). |
| `stopCommand` | The command Stop runs when `killMode` is `command`. |
| `processName` | The process name Moonpool watches for, and kills in `processName` mode. |
| control channel | The named pipe (Windows) or Unix socket (Linux, macOS) the hub answers on. See [Control verbs](/automation/control-verbs/). |
| verb | A command word such as `launch` or `reload`, given on the command line or the control channel. |
| ticket | A key you attach with `--ticket` to read a command's outcome from `state.json`. |
| token | The version stamp of `apps.json` that a config write must carry. |
| MCP helper (shim) | An `<exe> mcp` process an AI host starts to reach an app's own tools. |
