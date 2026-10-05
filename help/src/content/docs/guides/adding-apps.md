---
title: Adding Apps
description: Register an app or dev server so Moonpool can launch and manage it.
---

Each app in Moonpool is one entry with a launch command, a working folder, and an optional
environment. Moonpool runs the command in its own managed terminal.

## Add an app

1. Open the **...** menu at the top of the sidebar and choose **Add app**.
2. Enter a **name** and pick a **group**.
3. Pick the **type**: `web` (server on a port), `desktop` (native app), `static` (a page) or `cli` (a command).
4. Set the **command** and the **cwd** it runs in.
5. Fill in what the type needs: **port** and **url** for web, **processName** for desktop.
6. Save. The app appears in the sidebar. Use its **Launch** control to start it.

![The type select (1) and the port field (2) in the app editor, with cwd and command between them](../../../assets/screenshots/edit-app-type-and-port.png)

1. The **type** select; its hint says how that type shows Running.
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

Clicking an app's name only opens its terminal tab. It does not start the app.

## Edit by hand

Choose **Edit apps.json** in the same menu, save the file, then choose **Reload**. The
format, validation rules and recovery options are in the
[Configuration overview](/configuration/overview/).

## Where to go next

- [App fields](/configuration/fields/): every key and what it does.
- [App types](/configuration/app-types/): how each type launches and shows Running.
- [Stop and restart](/configuration/stop-and-restart/): what to set when Stop leaves something running, and why Docker apps need care.
- [Paths and environment](/configuration/paths-and-environment/): `{MP_HOME}`, `./` paths and `env`.
- [Examples](/configuration/examples/): complete entries to copy.
- [Portable mode](/guides/portable-mode/)
- [Updating](/guides/updating/)
