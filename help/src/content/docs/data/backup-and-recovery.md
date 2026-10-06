---
title: "Back up Moonpool, roll back apps.json and recover a setup"
description: "Know what to back up, roll back a bad apps.json, reset to the example apps, move an installed setup to a portable copy, and see what uninstall removes."
---

Everything Moonpool keeps is in two places: the config folder and the dashboards folder.
Paths for each mode are in [Where the config lives](/apps/apps-json/#where-the-config-lives).

## The config folder

```text
moonpool-config\
  apps.json            your apps                              back up
  apps.json.history\   the last 10 good apps.json files       back up (optional)
  settings.json        app settings                           back up
  icons\               icon overrides, <id>.png and so on     back up
  cli-output\<id>\     session logs                           disposable
  moonpool.log         debug log                              disposable
  state.json           live status snapshot                   disposable
  dumps\               files written by dump and read-config  disposable
  mcp_seen.json        which apps had an MCP helper           disposable
  window-state.json    hub window size and position           disposable
  AI-README.md         rewritten at every launch              disposable
  webview\             the window's browser profile (Windows) disposable
```

The dashboards folder is `{MP_HOME}\dashboards`: `%USERPROFILE%\.moonpool\dashboards`
installed, `<your .moonpool folder>\dashboards` portable, and `dashboards/` inside the config
folder on Linux. Back up anything of your own in it. Its `examples` folder belongs to Moonpool
and is rewritten on update.

The theme is kept in the window's browser storage, not in a file you can copy. It does not
travel with a backup; pick it again after a restore.

## Back up

1. Quit Moonpool, so no file is half written.
2. Copy `apps.json`, `settings.json` and `icons\` from the config folder, and your own files
   from `dashboards\`.

```powershell frame="terminal"
$cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
Copy-Item "$cfg\apps.json", "$cfg\settings.json" D:\backup\
Copy-Item "$cfg\icons" D:\backup\ -Recurse
```

To restore, quit Moonpool, copy the files back, and start it.

## Roll back apps.json

Every successful save, agent write and restore, and every Reload that finds changed content,
copies the validated `apps.json` into `apps.json.history\`, keeping the newest 10. Each file
is named by the time it was taken, for example `1767225600000.json`. There is no
`apps.json.bak`.

- **By hand.** Copy a snapshot over `apps.json`, then choose **Reload**.

  ```powershell frame="terminal"
  $cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
  Copy-Item "$cfg\apps.json.history\<snapshot>" "$cfg\apps.json"
  ```

- **From a script.** `moonpool.exe restore-config` lists the snapshots;
  `moonpool.exe restore-config 1` restores the newest. See
  [Command line](/automation/command-line/).
- **From an agent.** `moonpool_restore_config`. See [MCP tools](/automation/mcp-tools/#configuration).

Nothing is restored automatically.

## A broken file

- **apps.json.** Moonpool never overwrites a broken file. See
  [If the file is bad](/apps/apps-json/#if-the-file-is-bad).
- **settings.json.** Fix it, or delete it to reset every setting, then restart Moonpool. See
  [settings.json](/data/settings-json/#reading-and-repair).

## Reset to the examples

Moonpool writes its example apps only when there is no `apps.json`. To start over, quit
Moonpool (or keep it running), rename or delete `apps.json`, then start Moonpool or choose
**Reload**. A fresh `apps.json` with the examples is written.

## Installed to portable

A new portable copy starts with the example apps. To bring your own across, see
[Portable mode](/data/portable-mode/#choosing-portable-from-the-installer). Copy `icons\` and
`settings.json` the same way if you want them.

## Uninstalling

Uninstalling the installed Moonpool deletes the whole `%USERPROFILE%\.moonpool` folder,
including the config folder and dashboards. Back up first. See
[Uninstalling](/getting-started/install/#uninstalling). A portable copy is removed by
deleting its `.moonpool\` folder.
