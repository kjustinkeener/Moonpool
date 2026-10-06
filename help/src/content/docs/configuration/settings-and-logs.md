---
title: Settings and logs
description: settings.json keys and defaults, per-app terminal logs, the restart divider, and log retention.
---

## settings.json

App-wide settings live in `settings.json` in the config folder (see
[Overview](/configuration/overview/#where-the-config-lives)). Change them in the Settings
window. Moonpool reads the file at startup. If it is malformed, Moonpool refuses to change
settings until you repair it and restart.

| Key | Default | Meaning |
| --- | --- | --- |
| `closeToTray` | `false` | Hide to the tray when the window is closed. |
| `minimizeToTray` | `true` | Hide to the tray when the window is minimized. |
| `showInTray` | `true` | Show the tray icon. |
| `showInTaskbar` | `true` | Show the main window in the taskbar. |
| `alwaysOnTop` | `false` | Keep Moonpool's windows above others. |
| `transparency` | `0` | Background see-through percent, 0 to 90. |
| `showStatusbar` | `true` | Show the CPU and memory bar. |
| `showMcpProcesses` | `true` | Show an app's MCP helper process as a sub-item in the sidebar. |
| `checkOnStartup` | `true` | Check for a newer version once at startup. |
| `locale` | `"auto"` | UI language: `auto` follows the OS, otherwise a language tag. |
| `debugLogging` | `false` | Write manifest loads, launches and errors to `moonpool.log`. |
| `cliLogging` | `false` | Keep terminal logs from previous Moonpool sessions. |
| `logRetentionMb` | `10` | Per-app cap on kept logs from previous sessions. Minimum 1. |

```json title="settings.json"
{
  "closeToTray": true,
  "transparency": 20,
  "debugLogging": true,
  "cliLogging": true,
  "logRetentionMb": 25
}
```

Moonpool also stores the UI zoom (`uiScale`, 0.5 to 3.0) and the resolved language
(`localeResolved`) in this file. Both are written for you.

## Terminal logs

Everything an app prints in its terminal is also written to a log file:

```text
<config folder>\cli-output\<id>\<session-start-ms>.log
```

- One file per app per Moonpool session. The number is when that Moonpool process started.
- Stopping and relaunching an app keeps appending to the same file. A dim divider line
  marks where each new run begins, and the same marker shows in the terminal tab's
  scrollback:

  ```text title="1767225600000.log"
  Local:   http://localhost:5173/
  ---------- restarted 2026-10-05 09:14:02 ----------
  Local:   http://localhost:5173/
  ```
- Characters in an `id` other than letters, digits, `-` and `_` become `_` in the folder
  name. So `.` becomes `_`. Letters outside English are kept.
- The current session's file is always written and never truncated or deleted by retention.
- In Settings, **Reveal CLI log folder** opens `cli-output\`.

## Retention

Retention only concerns logs from earlier Moonpool sessions. It runs when you launch an app,
for that app's folder only, oldest first.

| `cliLogging` | What happens to earlier sessions' logs |
| --- | --- |
| `false` | Deleted at the app's next launch. |
| `true` | Kept until their combined size passes `logRetentionMb`, then the oldest are deleted. |

![The Logging section of Settings: the keep-logs checkbox, the per-app retention size in MB, and the debug-log checkbox, each with a folder path row](../../../assets/screenshots/settings-logging-section.png)

1. **Keep app output logs between sessions** is `cliLogging`. **Log retention per app** below it is `logRetentionMb`.

## moonpool.log

With `debugLogging` on, Moonpool appends timestamped lines to `moonpool.log` in the config
folder: manifest loads, launches (with the command and folder), and errors. It is separate
from the per-app terminal logs.
