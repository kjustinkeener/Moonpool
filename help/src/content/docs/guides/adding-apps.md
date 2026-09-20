---
title: Adding Apps
description: Register an app or dev server so Moonpool can launch and manage it.
---

Each app in Moonpool is one entry with a launch command, a working directory, and an
optional environment. Moonpool runs the command in its own managed terminal.

## Add an entry

1. Open the editor from the hub.
2. Give the app a **name** and pick a **group** (groups organize the tiles).
3. Set the **command** to run and the **working directory** to run it in.
4. Save. The new tile appears in the hub; click it to launch.

## Portable-friendly paths

If you run Moonpool portably, use the `{MP_HOME}` token in a path instead of a fixed
drive letter (for example `{MP_HOME}\tools\myapp.exe`). `{MP_HOME}` resolves to Moonpool's
own folder, so your apps travel with the bundle. Absolute paths still work but will not
move with the folder.

## Next

- [Portable mode](/guides/portable-mode/)
- [Updating](/guides/updating/)
