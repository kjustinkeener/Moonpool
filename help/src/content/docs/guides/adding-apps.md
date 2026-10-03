---
title: Adding Apps
description: Register an app or dev server so Moonpool can launch and manage it.
---

Each app in Moonpool is one entry with a launch command, a working directory, and an
optional environment. Moonpool runs the command in its own managed terminal.

## Add an entry

1. Open the editor from the hub.
2. Give the app a **name** and pick a **group** (groups organize the tiles).
3. Pick the **type**: `web` (dev server on a port), `desktop` (native app), `static` (a page) or `cli` (a terminal).
4. Set the **command** to run and the **working directory** to run it in.
5. Fill in what the type needs: **port** and **url** for web, **processName** for desktop.
6. Optionally set **killMode** if Stop leaves something running (see below).
7. Save. The new tile appears in the hub; click it to launch.

Every field is explained in the [App fields reference](/reference/apps-json/).

## Stop and restart

Stop ends the terminal Moonpool started for the app and everything it launched. If the app
outlives that, pick a **killMode**: `processName` kills by executable name, `port` kills
whatever listens on the port, `command` runs your own **stopCommand**, and `none` does
nothing more. Leave it empty for the default for the type (`desktop` uses `processName`,
`web` uses `port`, others use `none`). Docker apps should use `none` or `command`, never
`port`. Details: [Stop and Restart](/reference/apps-json/#stop-and-restart).

## Portable-friendly paths

If you run Moonpool portably, use the `{MP_HOME}` token in a path instead of a fixed
drive letter (for example `{MP_HOME}\tools\myapp.exe`). `{MP_HOME}` resolves to Moonpool's
own folder, so your apps travel with the bundle. Absolute paths still work but will not
move with the folder.

## Next

- [App fields reference](/reference/apps-json/)
- [Portable mode](/guides/portable-mode/)
- [Updating](/guides/updating/)
