# Moonpool - AI configuration guide

You are configuring **Moonpool**, a system-tray launcher hub for local apps. It lists apps,
shows live status, and launches each inside an embedded terminal. Your job: discover the user's
launchable apps and register them in Moonpool's manifest.

## The manifest

Moonpool reads a single JSON file (an array of app objects):

```
%APPDATA%\Moonpool\apps.json      (Windows: C:\Users\<user>\AppData\Roaming\Moonpool\apps.json)
```

Edit this file directly, then tell the user to click **Reload** in Moonpool's top bar (or they
restart it). Changes are picked up from disk - no rebuild.

**Always address `apps.json` (and `state.json` below) by its full literal absolute path** -
e.g. `C:\Users\<user>\AppData\Roaming\Moonpool\apps.json`, not a `%APPDATA%` / `$env:APPDATA`
shortcut. Some sandboxed agents get silently redirected to a private copy when they use the
variable form, and then edit a file the real Moonpool never sees.

## App object schema

```jsonc
{
  "id": "unique-slug",           // required, unique, kebab-case
  "name": "Display Name",        // required
  "group": "Web apps",           // required; any label. Groups render in first-seen order.
                                 //   Conventional groups: "Desktop apps", "Web apps", "Docs", "CLI tools"
  "type": "web",                 // required: desktop | web | static | cli
  "cwd": "C:\\path\\to\\app",    // working directory the command runs in
  "command": "npm run dev",      // launch command (run via `cmd /c`)
  "port": 5173,                  // web: used for status + browser-open
  "url": "http://localhost:5173",// web: opened when the port goes live; static: opened directly
  "openBrowser": true,           // web/static: open the browser
  "env": { "PORT": "5173" },     // optional env vars injected into the command
  "processName": "app",          // desktop: status by process name (its .exe, without extension)
  "note": "shown as a tooltip"   // optional
}
```

## type semantics

- **desktop** - a native app. Status is detected by `processName`. No browser.
- **web** - a local server. Status is a TCP health-check on `port`; the browser opens (to `url`)
  when it first answers. Set `openBrowser:false` if the command opens a browser itself.
- **static** - a static page/dashboard. With only a `url` it just opens in the browser (no
  terminal). With a `command` it runs that (e.g. a doc generator) then exits.
- **cli** - a tool. Opens an interactive shell in `cwd`. To run something first and keep the
  shell open, use `pwsh -NoLogo -NoProfile -NoExit -Command <tokens...>`.

## Command rules (important)

- Every `command` runs through `cmd /c` in `cwd`, inheriting the environment plus `env`.
- **Prefer a FOREGROUND command that streams logs** (`python app.py`, `node server.js`, an app's
  `dev-run.ps1`) over a detached/windowless launcher (`pythonw`, `.vbs`, `start ...`), so output
  shows in the terminal.
- **Do NOT use nested double-quotes** in `command` - they get mangled through the `cmd /c`
  wrapper and error out. For a one-shot-then-shell, use the UNQUOTED form:
  `pwsh -NoExit -Command python run.py --flag`  (NOT `-Command "python run.py --flag"`).
- Give each `web`/`static` app a UNIQUE `port` (Moonpool warns on collisions). If two apps
  hard-code the same port, move one via `env` (e.g. `"env": {"PORT": "8091"}`) if it reads `PORT`.

## Discovering apps

Look under the user's project roots for launchable things:
- **web/dashboards**: a `package.json` with a `dev`/`start` script (Vite/Next/etc.), or a Python
  file that serves HTTP (`http.server`, Flask/FastAPI/uvicorn). Grep for the port it binds.
- **static dashboards**: a standalone `.html` file meant to be opened directly -> `type: static`,
  `url: file:///...`.
- **desktop apps**: a Tauri/Electron project (has `src-tauri/` or an Electron main); use its dev
  launch script and set `processName` to the built exe's base name.
- **CLI/scripts**: a script the user runs by hand -> `type: cli`.

Confirm each app's real launch command and port before adding it. Preserve any existing entries
in `apps.json`; append new ones.

## Controlling Moonpool while it's running

Once Moonpool is running (it lives in the system tray), you can drive it from the command
line: running the Moonpool program again with a command word hands that command to the
already-open Moonpool instead of opening a second window. Use this to launch, stop, reload,
or refresh on the user's behalf.

First find the running program's own path - don't assume a fixed install location (a
per-user install lives under `%LOCALAPPDATA%\MoonPool\moonpool.exe`, not `Program Files`):

```powershell
$mp = (Get-Process moonpool -ErrorAction SilentlyContinue | Select-Object -First 1).Path
```

Then run it with a command word:

```powershell
& $mp launch <app-id>      # open an app and start it
& $mp stop <app-id>        # stop a running app
& $mp restart <app-id>     # stop, wait for the port/process to free, relaunch
& $mp reload               # re-read apps.json
& $mp refresh-icons        # re-pull every icon
& $mp show                 # bring the window to the front
& $mp dump <app-id>        # write that app's console output to a file
```

`<app-id>` is the `id` field from `apps.json`. `restart` is the managed stop-then-relaunch
(it waits for the port/process to free before relaunching - prefer it over a manual stop +
launch). These only work while Moonpool is running; if it isn't, start it first (or a bare run
just opens it).

`dump` writes the app's recent terminal output (ANSI stripped, last ~512 KB, the current
run only) to `%APPDATA%\Moonpool\dumps\<app-id>.log`, or to a path you pass as a third
argument: `& $mp dump my-app C:	mp\out.log`. Read that file to see what an app printed
without opening the window. Tag it with `--ticket` and the ticket's `detail` is the file
path it wrote (or why it couldn't - e.g. the app hasn't been launched this session).

**This channel needs Moonpool v0.1.4 or newer.** If running a command opens a NEW window
instead of handing off to the open one, the running build is older - update Moonpool first.
A quick check: if `state.json` (below) is MISSING, the running build predates this channel.

### Driving Moonpool over MCP (recommended for agents)

Everything below can be done with tool calls instead of shelling out: Moonpool's own exe
is an MCP server over stdio.

```json
{
  "mcpServers": {
    "moonpool": { "type": "stdio", "command": "C:\path\to\moonpool.exe", "args": ["mcp"] }
  }
}
```

Tools: `moonpool_list`, `moonpool_launch`, `moonpool_stop`, `moonpool_restart`,
`moonpool_dump` (returns the terminal output as text, `tail_lines` to bound it),
`moonpool_reload`, `moonpool_refresh_icons`, `moonpool_show`.

The server is a *client* of the resident tray instance, using the same channel described
below - it fires the command with a ticket and waits for the outcome, so each tool call
returns success or the actual error instead of leaving you to poll a file. Moonpool must
already be running; the server does not start it.

### Checking what's running

Moonpool continuously writes a status snapshot to:

```
%APPDATA%\Moonpool\state.json   (C:\Users\<user>\AppData\Roaming\Moonpool\state.json)
```

It contains `apps` (the registered app list), `statuses` (one entry per app with
`running` and `managed` flags), and `tickets` (see below). Read this file to see which
apps are up before deciding what to launch or stop.

### Confirming a command worked (tickets)

The commands don't print back to you (a windowed app has no terminal to reply into), so
to learn whether an action actually took effect, tag it with a ticket and read the result
out of `state.json`. Pick any unique key yourself:

```powershell
& $mp launch my-app --ticket t1
```

Moonpool records the outcome under that key in the `tickets` array of `state.json`. Poll
that file until your ticket's `status` is no longer `pending`:

- `pending` - received, the UI is still acting on it.
- `ok` - the app reached the desired state (running for launch/restart, stopped for stop).
- `error` - it didn't; `detail` says why (e.g. `unknown app id`, `did not reach running in time`).

Each ticket entry: `{ ticket, action, arg, status, detail, ts }` (`ts` = Unix millis). Only
the last ~50 are kept. Give up after a sensible timeout if `status` stays `pending` (Moonpool
may have been closed mid-command). `reload` / `refresh-icons` accept a ticket too and just
report `ok` when done. Requires v0.1.5+ (older builds ignore `--ticket` and write no `tickets`).

### Remember this for later

So you can run, check, and stop the user's apps in future sessions without re-reading this
guide, save the above to your own memory: the control commands, how to find the running
`moonpool.exe` path (`Get-Process moonpool`), and the `state.json` location and shape.

## Icons (optional)

Moonpool resolves each app's sidebar icon in this order:
1. The manifest `"icon"` value: a file path (`C:\\path\\logo.png`), an `http(s)` URL, or a data URI.
2. `%APPDATA%\Moonpool\icons\<id>.<ext>` (png/ico/svg/jpg/webp), where `<id>` is the app's id.
3. Auto-discovered from the app's own project folder (its `cwd`): Tauri `src-tauri/icons/`,
   Electron `build/icon.*`, web `public/favicon.*` / `public/logo.png`, or a root
   `icon.png`/`logo.png`/`favicon.*`. Works with no config and without the app running.
4. A desktop app's exe icon (running process or its built exe), then a web app's live
   `/favicon.ico`.
5. Otherwise a type glyph.

So you usually don't need to set icons at all (web favicons and desktop exe icons come for free).
To force one, set `"icon"` to a real image path/URL, or drop `<id>.png` in the icons folder.
