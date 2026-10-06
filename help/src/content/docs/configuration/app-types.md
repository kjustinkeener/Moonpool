---
title: App types
description: How web, desktop, static and cli apps launch, how Running is detected, and what Stop does by default.
---

`type` decides which fields matter and what Stop does by default.

| | `web` | `desktop` | `static` | `cli` |
| --- | --- | --- | --- | --- |
| Needs | `command` | `command` | `url` | `command` |
| Usually also | `port`, `url` | `processName` | `command` and `port`, if it serves itself | `cwd` |
| Launch | Runs `command` in a terminal tab | Runs `command` in a terminal tab | No `command`: opens `url` in the browser. With one: runs it in a terminal tab | Runs `command` in a terminal tab |
| Default `killMode` | `port` | `processName` | `none` | `none` |

![The Edit app dialog for a web app: type set to web with a one-line description, and a port field filled in](../../../assets/screenshots/edit-app-type-and-port.png)

1. The `type` select. Its hint line describes what that type does.
2. The `port` field. For a `web` app, Running follows whether this port answers.

## How Running is decided

Moonpool checks every couple of seconds. An app is Running if any of these holds, whatever
its type:

- `processName` is set and a process with that name exists. Moonpool's own `<exe> mcp`
  helper processes are not counted.
- `port` is set and answers on localhost.
- Moonpool launched it, it has neither `port` nor `processName`, and the terminal's process
  is still alive.

So a `cli` app is Running while its command runs, and a `web` app with no `port` behaves the
same way. A `static` entry with only a `url` has nothing to track and never shows Running.

## web

A local server. Set `port` so Running reflects whether the server is answering, and `url`
plus `openBrowser` to open it when it comes up.

## desktop

A native app. Set `processName` to the executable name so Running survives the window
detaching from the command that started it. The default Stop kills every process with that
name.

## static

A page. With only a `url`, Launch and Restart open it in your browser and Stop does nothing.
`http://`, `https://`, `mailto:` and `file://` URLs are opened, so a local page works:

```json title="apps.json"
{ "id": "csv", "name": "CSV dashboard", "group": "Docs", "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html" }
```

Pages that need a server (PHP, or anything
fetching local files) need a `command` that starts one and a `port` to track it. See the
[examples](/configuration/examples/).

## cli

A tool. `command` runs in a terminal tab in `cwd`, and the app stops being Running when the
command exits. For a shell that stays open, make the command a shell, for example
this `command`:

```text title="command"
pwsh -NoLogo -NoProfile -NoExit -Command python run.py --flag
```

Avoid nested double quotes in `command`: they are mangled by the `cmd /c` wrapper.

![A cli app's terminal tab showing a PowerShell command's output and an open prompt below it](../../../assets/screenshots/terminal-cli-output.png)

## What clicking does

Clicking an app's name opens or focuses its terminal tab and shows this session's output.
It does not launch anything. Use the Launch, Stop and Restart controls for that.
