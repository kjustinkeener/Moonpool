---
title: "Start a script or dev server automatically at Windows login"
description: "Start Moonpool at Windows login with a Startup folder shortcut, then launch a dev server or script in it with a small PowerShell script. No setting does it."
---

Windows has two usual ways to start something at login: a shortcut in your Startup folder
(press Win+R, type `shell:startup`, press Enter), or a Task Scheduler task with an "At log
on" trigger. Either one runs a program or script, which could be your dev server's command
directly, but then nothing tracks it, shows its output or stops it for you.

## What Moonpool offers

Moonpool has no start-at-login setting, and an entry in `apps.json` has no field that
launches it when Moonpool starts (the full list is in [App fields](/apps/fields/) and
[settings.json](/data/settings-json/)). What you can do is start Moonpool at login yourself,
then have a script launch the apps you want, using the same verb the
[command line](/automation/command-line/) offers.

First register the app as usual:

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

Then save this as `start-moonpool-apps.ps1`. Installed, the program is
`%USERPROFILE%\.moonpool\moonpool.exe`; for a portable copy use the path of that copy's exe.

```powershell title="start-moonpool-apps.ps1"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
Start-Process $mp
Start-Sleep -Seconds 15
& $mp launch site
```

Moonpool must already be running for `launch` to be handed to it; run with nothing resident,
the same command starts a new Moonpool and the verb is not carried out. The delay gives it
time to start, so raise it on a slow machine. Add one `& $mp launch <id>` line per app.

Finally put a shortcut to the script in the Startup folder, with this target:

```text title="Shortcut target"
powershell.exe -NoProfile -WindowStyle Hidden -File "C:\Users\you\start-moonpool-apps.ps1"
```

To check what happened, add `--ticket t1` to a verb and read the result from `state.json`
([Reading the outcome](/automation/command-line/#reading-the-outcome)).

## Caveats

- A dev server started this way is "managed" by Moonpool like any other, so Stop and Quit
  work on it. If the same app is already running (started by hand, say), Moonpool shows it
  as running but not managed.
- Moonpool does not relaunch an app that exits, and does not remember which apps were
  running when you last quit.

## See also

- [Command line](/automation/command-line/)
- [Tray, close and minimize](/using/tray-and-closing/)
- [Run an npm dev server in the background on Windows](/guides/run-npm-dev-server-in-background-windows/)
