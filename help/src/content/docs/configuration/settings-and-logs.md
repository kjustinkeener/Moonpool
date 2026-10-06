---
title: settings.json
description: The shape of settings.json, the keys Moonpool writes for you, and what happens when the file is broken.
---

App-wide settings live in `settings.json` in the config folder (see
[Where the config lives](/configuration/overview/#where-the-config-lives)). Change them in the
[Settings window](/using/settings-window/), which lists every setting with its JSON key and
default. Logs and their retention are on the [Logs](/configuration/logs/) page.

## Shape

One JSON object. Keys you leave out take their defaults:

```json title="settings.json"
{
  "closeToTray": true,
  "transparency": 20,
  "debugLogging": true,
  "cliLogging": true,
  "logRetentionMb": 25
}
```

| Key | Default |
| --- | --- |
| `closeToTray` | `false` |
| `minimizeToTray` | `true` |
| `showInTray` | `true` |
| `showInTaskbar` | `true` |
| `alwaysOnTop` | `false` |
| `transparency` | `0` (0 to 90) |
| `showStatusbar` | `true` |
| `showMcpProcesses` | `true` |
| `checkOnStartup` | `true` |
| `locale` | `"auto"` |
| `debugLogging` | `false` |
| `cliLogging` | `false` |
| `logRetentionMb` | `10` (minimum 1) |

## Keys written for you

Moonpool also stores the UI zoom (`uiScale`, 0.5 to 3.0) and the resolved language
(`localeResolved`) in this file. You do not need to set either. The theme is not here: it is
kept in the webview's storage (see [Themes, language and transparency](/using/appearance/)).

## Reading and repair

Moonpool reads the file at startup. Edits made while it runs are not picked up; quit first.

If the file is malformed, Moonpool starts with the defaults and refuses to change settings.
The error ends with `Repair settings.json and restart Moonpool before changing settings`.
Fix the file, or delete it to reset every setting, then start Moonpool again.
