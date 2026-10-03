---
title: App fields reference
description: Every field of an app entry, which app types use it, and exactly what Stop and Restart do.
---

Each app is one entry in `apps.json`. The Edit App dialog shows the same fields, using the
same names. Fields that don't apply to the selected **type** are dimmed in the dialog but
still saved.

## Fields

| Field | Used by | What it does |
| --- | --- | --- |
| `name` | all | Label shown on the tile. Required. |
| `group` | all | Heading the tile is listed under. Any text; new groups are created on the fly. |
| `type` | all | `web`, `desktop`, `static` or `cli`. Decides how the app is launched and tracked, and the default **Stop** behavior. See below. |
| `cwd` | web, desktop, cli, static with a command | Folder the command runs in. Supports `{MP_HOME}` and `./` for portable setups. |
| `command` | web, desktop, cli, static | What Moonpool runs in the app's terminal to start it. |
| `port` | web | The TCP port the app listens on. Moonpool shows **Running** when it answers. Also used by Stop when `killMode` is `port`. |
| `url` | web, static | Page to open. For web apps it opens once the port answers. |
| `openBrowser` | web, static | Open `url` automatically. Turn it off if your command opens a browser itself. |
| `processName` | desktop | Executable name, without `.exe` (15 characters or fewer on Linux). Moonpool shows **Running** while a process with this name exists. Also used by Stop when `killMode` is `processName`. |
| `killMode` | all | Extra cleanup on Stop and Restart. See [Stop and Restart](#stop-and-restart). |
| `stopCommand` | `killMode` = `command` | Command to run on Stop. Ignored in every other mode. |
| `env` | all | Extra environment variables for the command, one `KEY=VALUE` per line in the dialog. |
| `note` | all | Tooltip text on the tile. |

## Types

- **web**: a local server. Running means the `port` answers. Moonpool opens `url` once it does.
- **desktop**: a native app. Running means a process named `processName` exists.
- **static**: a page. With only a `url` it just opens in the browser. With a `command` it runs the command, then exits.
- **cli**: a tool. Opens an interactive terminal in `cwd`.

## Stop and Restart

Stop always does this first: Moonpool ends the terminal it started for the app, including
everything that terminal launched. For many apps that is all that's needed.

Some apps outlive that terminal (a desktop window detaches from the dev server that
launched it, or a server subprocess keeps holding its port). **`killMode`** chooses one
extra step that runs afterward:

| `killMode` | Extra step on Stop | Reads these fields | Default for |
| --- | --- | --- | --- |
| `processName` | Force-kills every process with the name `processName`. | `processName` | `desktop` |
| `port` | Force-kills whatever process is listening on `port`. | `port` | `web` |
| `command` | Runs `stopCommand` in `cwd` and waits for it to finish. | `stopCommand`, `cwd`, `env` | nothing |
| `none` | Nothing. | none | `static`, `cli` |

Leave `killMode` empty to get the default for the app's type. Existing apps keep working
unchanged. If the field `killMode` needs is empty (for example `port` with no `port` set),
the extra step is skipped.

Restart is Stop followed by launching `command` again. A `stopCommand` is awaited first, so the relaunch
never races it.

### Docker apps on Windows

Use `none`, or `command` with a real stop command such as `docker compose stop app`. Do not
use `port`.

Docker Desktop publishes every container's port through one shared background process. On
Windows, "whatever is listening on the port" is that shared process, so `port` mode would
force-kill Docker Desktop and take down every container, not just this app. Moonpool refuses
to kill a short list of well-known shared processes as a backstop, but that is not a
substitute for choosing the right mode.

If your `command` already recreates the container (`docker compose up -d --build`), `none`
is correct: Restart just runs it again.

## Examples

A dev server that sometimes leaves a node process holding its port (this is the default for
`web`, shown here explicitly):

```json
{ "id": "site", "name": "Site", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
  "killMode": "port" }
```

A Docker Compose app:

```json
{ "id": "api", "name": "API", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\api", "command": "docker compose up -d --build", "port": 8080,
  "killMode": "command", "stopCommand": "docker compose stop app" }
```
