---
title: "Add an app or dev server to Moonpool"
description: "Register a local app or dev server with a launch command, working folder and environment so Moonpool can start, stop and watch it for you."
---

Each app in Moonpool is one entry with a launch command, a working folder, and an optional
environment. Moonpool runs the command in its own managed terminal.

## Add an app

1. Open the **...** menu at the top of the sidebar and choose **Add app**.
2. Enter a **name** and pick a **group**.
3. Pick the **type**: `web` (server on a port), `desktop` (native app), `static` (a page) or `cli` (a command).
4. Set the **command** and the **cwd** it runs in.
5. Fill in what the type needs: **port** and **url** for web, **processName** for desktop,
   **url** for static. A `static` app with only a `url` needs no **command** or **cwd**.
6. Save. The app appears in the sidebar. Use its **Launch** control to start it.

![The type select (1) and the port field (2) in the app editor, with cwd and command between them](../../../assets/screenshots/edit-app-type-and-port.png)

1. The **type** select; its hint says how that type runs.
2. The **port** field, used by `web` apps.

The result is one entry in `apps.json`, for example:

```json title="apps.json"
{
  "id": "my-api",
  "name": "My API",
  "group": "Dev",
  "type": "web",
  "command": "npm run dev",
  "cwd": "C:\\code\\my-api",
  "port": 3000,
  "url": "http://localhost:3000"
}
```

Clicking an app's name only opens its terminal tab; see [App states](/support/glossary/#app-states).

## The app editor

- **Group.** Pick a group from the list, or choose **+ New group...** and type a name.
  **back to list** returns to the list. A blank group is saved as `Apps`.
- **Dimmed fields** are not used by the selected type. They are still saved.
- **Save without a name** shows `name is required.`
- **Esc** or closing the editor with unsaved changes asks "Discard your changes?".
- To change an app later, use the pencil on its row, or right-click it and choose **Edit**.

## Edit by hand

Choose **Edit apps.json** in the same menu, save the file, then choose **Reload**. The
format, validation rules and recovery options are in the
[Configuration overview](/apps/apps-json/).

## Where to go next

- [App fields](/apps/fields/): every key and what it does.
- [App types](/apps/types/): how each type launches and shows Running.
- [Stop and restart](/apps/stop-and-restart/): what to set when Stop leaves something running, and why Docker apps need care.
- [Paths and environment](/apps/paths-and-environment/): `{MP_HOME}`, `./` paths and `env`.
- [Examples](/apps/examples/): complete entries to copy.
- [How-to guides](/guides/run-npm-dev-server-in-background-windows/): dev servers in the background, Python scripts, ports.
- [Portable mode](/data/portable-mode/)
- [Updating](/data/updating/)
