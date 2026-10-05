---
title: App fields
description: Every key of an apps.json entry with its type, default, and which app types use it.
---

The Edit app dialog shows the same fields under the same names. Fields that do not apply to
the selected type are dimmed in the dialog but still saved, with one exception:
`stopCommand` is only saved while `killMode` is `command`.

![The Edit app dialog from name down to stopCommand, with the killMode select outlined; unused fields such as processName and stopCommand are dimmed](../../../assets/screenshots/edit-app-dialog.png)

1. The `killMode` select. Fields it does not use stay dimmed.

| Field | Type | Required | Used by | What it does |
| --- | --- | --- | --- | --- |
| `id` | string | yes | all | Unique key. Letters, digits, `.`, `_`, `-`, not starting with `-`. See [Overview](/configuration/overview/#the-id). |
| `name` | string | yes | all | Label in the sidebar. Not blank. |
| `group` | string | yes | all | Sidebar heading the app is listed under. Not blank. Any text; a new name creates a new group. |
| `type` | string | yes | all | `web`, `desktop`, `static` or `cli`. See [App types](/configuration/app-types/). |
| `command` | string | all but `static` | all | Run in a terminal to start the app, through `cmd /c` on Windows and `sh -c` elsewhere. Optional for `static`. |
| `cwd` | string | no | all with a `command` | Folder the command runs in. Defaults to Moonpool's own working folder. Supports tokens and `./`. See [Paths and environment](/configuration/paths-and-environment/). |
| `port` | integer, 1 to 65535 | no | any | Running while something answers on this port on localhost (IPv4 or IPv6). Read by `killMode` `port`. |
| `processName` | string | no | any, mainly `desktop` | Running while a process with this name exists. Case-insensitive, with or without `.exe`, so `my-app` matches `my-app.exe`. On Linux, 15 characters or fewer. Read by `killMode` `processName`. |
| `url` | string | `static` only | `web`, `static` | Page to open. Only `http://`, `https://`, `mailto:` and `file://` URLs are opened. |
| `openBrowser` | boolean, default `false` | no | `web`, `static` | Open `url` automatically once Moonpool detects the app is up (see below). |
| `killMode` | string | no | all | Extra cleanup on Stop and Restart: `processName`, `port`, `command` or `none`. See [Stop and restart](/configuration/stop-and-restart/). |
| `stopCommand` | string | no | `killMode` `command` | Command run on Stop. Ignored in every other mode. |
| `env` | object of strings | no | all | Extra environment variables. The dialog edits it as one `KEY=VALUE` per line. |
| `icon` | string | no | all | Sidebar image: a file path, an `http(s)` URL, or a `data:` URI. Set it from **Set icon...** in the app's context menu or by hand. |
| `note` | string | no | all | Tooltip when you hover the app in the sidebar. |

An entry using `env` and `killMode`:

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000", "NODE_ENV": "development" },
  "killMode": "port"
}
```

## openBrowser

Moonpool opens `url` once, when an app that Moonpool launched first reads as Running. That
needs a `port` or `processName` to detect it. Without either, Running only means the
terminal process is alive, and the browser is not opened automatically. Turn `openBrowser`
off if your command opens a browser itself. A `static` entry with no command opens `url`
whenever you press Launch, regardless of `openBrowser`.

Two apps configured with the same `port` are flagged in the sidebar.

## Icons

An app's icon is the first of these that exists:

1. The `icon` field.
2. `icons\<id>.<ext>` in the config folder, for example `icons\site.png`.
3. An icon file in the app's own folder (its `cwd`, or the folder of a `file:///` `url`).
4. For `desktop`, the icon of its built or running `.exe`.
5. For `web` and `static`, the site's `/favicon.ico`, once the server is up.
6. A glyph for the type.

Most apps need no icon setting.
