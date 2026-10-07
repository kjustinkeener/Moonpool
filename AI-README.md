# Moonpool - AI configuration guide

You are configuring **Moonpool**, a system-tray launcher hub for local apps. It lists apps,
shows live status, and launches each inside an embedded terminal. Your job: discover the user's
launchable apps and register them in Moonpool's manifest.

## The manifest

Moonpool reads a single JSON file (an array of app objects):

```
%USERPROFILE%\.moonpool\moonpool-config\apps.json      (Windows: C:\Users\<user>\.moonpool\moonpool-config\apps.json)
<exe folder>\moonpool-config\apps.json                (portable mode)
~/.config/Moonpool/apps.json                           (Linux; $XDG_CONFIG_HOME/Moonpool/apps.json if set)
```

Edit this file directly, then tell the user to click **Reload** in the **...** menu at the top of Moonpool's sidebar (or they
restart it). Changes are picked up from disk - no rebuild.

**Always address `apps.json` (and `state.json` below) by its full literal absolute path** -
e.g. `C:\Users\<user>\.moonpool\moonpool-config\apps.json`, not a `%USERPROFILE%` / `$env:USERPROFILE`
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
  "command": "npm run dev",      // launch command (run via `cmd /c`; `$SHELL -c` on Linux)
  "port": 5173,                  // web: used for status + browser-open
  "url": "http://localhost:5173",// web: opened when the port goes live; static: opened directly
  "openBrowser": true,           // web/static: open the browser
  "env": { "PORT": "5173" },     // optional env vars injected into the command
  "processName": "app",          // desktop: status by process name (its .exe, without extension;
                                 //   on Linux 15 characters or fewer, longer names are truncated)
  "mcpProcessName": "app-mcp-*",   // optional wildcard (* any run, ? one char; case-insensitive, whole name,
                                 //   .exe optional) for this app's MCP server process, when it is not
                                 //   `processName` or is a renamed copy. A match needs no `mcp` argument
                                 //   (unless the pattern also matches `processName`). Shown as the MCP sub-row.
  "killMode": "port",            // how stop/restart finds & kills what this app left running,
                                  //   beyond the PTY tree Moonpool already tree-kills unconditionally.
                                  //   One of: "processName" | "port" | "command" | "none".
                                  //   Unset defaults to "processName" for desktop, "port" for web,
                                  //   "none" otherwise - i.e. omitting it preserves old behavior.
  "stopCommand": "docker compose stop app", // killMode "command" only: run this in `cwd`, awaited
                                  //   to completion before restart's relaunch
  "note": "shown as a tooltip"   // optional
}
```

## type semantics

- **desktop** - a native app. Status is detected by `processName`. No browser. On Linux the
  kernel truncates process names to 15 characters, so a longer `processName` never matches.
- **web** - a local server. Status is a TCP health-check on `port`; the browser opens (to `url`)
  when it first answers. Set `openBrowser:false` if the command opens a browser itself.
- **static** - a static page/dashboard. With only a `url` it just opens in the browser (no
  terminal). With a `command` it runs that in a terminal (e.g. a local server).
- **cli** - a tool. Runs `command` in a terminal in `cwd`; it shows as running until the
  command exits. To run something first and keep a shell open, use `pwsh -NoLogo -NoProfile -NoExit -Command <tokens...>`.

## killMode (stop/restart cleanup, beyond the PTY tree)

On stop/restart, Moonpool always tree-kills the PTY subtree it spawned for `command` first. For
some apps that isn't enough (a desktop window detaches from its dev server; a web server
subprocess can linger holding `port`), so it does one more thing afterward, per `killMode`:

- **"processName"** (desktop default) - force-kill every process matching `processName`.
- **"port"** (web default) - force-kill whatever process is listening on `port`.
- **"command"** - run `stopCommand` in `cwd` and wait for it to finish.
- **"none"** - nothing further. Use this when the app's real lifecycle isn't Moonpool's to
  manage beyond the launcher script itself.

**Set `killMode: "none"` (or `"command"` with a real stop command) for anything backed by Docker
on Windows.** Docker Desktop proxies every container's published port through one shared backend
process (`com.docker.backend.exe` / vpnkit) - there is no per-container host listener. So `"port"`
mode's "whatever owns the port" resolves to that one shared Docker Desktop process for EVERY
container, not the one this app's config points at: force-killing it takes down Docker Desktop
entirely, for every app that depends on it, not just this one. A Docker Compose app whose launch
`command` already does `docker compose up -d --build` (an idempotent recreate) needs no separate
kill step at all - `killMode: "none"` is correct, and restart re-running `command` does the
right thing on its own. Moonpool also refuses to kill a handful of well-known shared/system
process images even under `"port"` mode as a backstop (see `platform::NEVER_KILL_BY_PORT` in the
source), but don't rely on that list instead of setting the right `killMode` - it only covers
process image names, not every way an app-specific config could point at shared infrastructure.

`processName` and `port` don't know who started a process: they kill EVERY process with that name,
or whatever listens on that port, including one another Moonpool copy started (an installed
Moonpool and portable copies can run side by side) or one the user started by hand. Use them only
for apps that won't clash that way; otherwise use `"none"` or a `"command"` that stops just this
instance.

`killMode` is independent of `type`: `port` works on a `cli` app and `processName` on a `web` app if
you set it explicitly. If the field the mode reads is missing (no `port`, no `processName`, no
`stopCommand`), the extra step is silently skipped, not an error.

## Command rules (important)

- Every `command` runs through `cmd /c` in `cwd` on Windows (`$SHELL -c`, else `/bin/sh -c`,
  on Linux), inheriting the environment plus `env`. The quoting rules below are
  for `cmd /c`.
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

One Moonpool runs per folder: the installed copy and any portable copies (each in its own
folder) can run at the same time, each with its own apps.json and its own control channel.
A command goes to the copy whose `moonpool.exe` you run, never to another one. On Windows the
copy that owns THIS file is the `moonpool.exe` in the folder above this config folder.

Use that copy's own path - don't assume a fixed install location (a per-user install lives
under `%USERPROFILE%\.moonpool\moonpool.exe`, not `Program Files`), and with several copies
running don't just take the first `moonpool` process:

```powershell
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"   # installed; a portable copy: <folder>\.moonpool\moonpool.exe
```

On Linux use the AppImage file the user runs, or for a `.deb`/`.rpm` install the binary the
package put on the PATH (`ls /usr/bin | grep -i moonpool`). The command words are the same.

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

Each app writes ONE persistent, append-only log per hub session (never truncated) to
`<config folder>\cli-output\<app-id>\<hub-start-ms>.log` - stopping and relaunching
the app keeps appending to the same file; only restarting Moonpool itself starts a new
one. Older sessions' files are kept only while `Settings > Keep app output logs between
sessions` (`cliLogging`) is on; then `Log retention per app` (default 10 MB) prunes the
oldest once the app's combined size passes the cap. With it off (the default), older
sessions' files are deleted the next time that app launches. The current file is exempt
and always survives. `dump` hands you that file's path directly (no `out-path`),
or copies it out as ANSI-stripped plain text to a path you pass as a third argument:
`& $mp dump my-app C:\tmp\out.log`. Tag it with `--ticket` and the ticket's `detail` is
the path (or why it couldn't - e.g. the app hasn't produced output this session).

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

Tools: `moonpool_list_apps`, `moonpool_bootup_launcher`, `moonpool_shutdown_launcher`,
`moonpool_raise_launcher`, `moonpool_start_app`, `moonpool_stop_app`, `moonpool_restart_app`,
`moonpool_app_output` (returns the terminal output as text, `tail_lines` to bound it),
`moonpool_stop_mcp_server`, `moonpool_reload_config`, `moonpool_read_config`,
`moonpool_write_config`, `moonpool_restore_config`, `moonpool_refresh_app_icons`,
`moonpool_launcher_paths`, plus the test tools `moonpool_screenshot` (Windows only),
`moonpool_window_state` and `moonpool_reset_mcp_seen`. Full reference: the Automation section
of Moonpool's built-in help.

With more than one Moonpool copy, register each copy's exe under its own server name (e.g.
`moonpool` and `moonpool-work`); the tool names are identical across copies. A portable copy
announces itself as `moonpool (<folder>)` and names its folder in its server instructions.

The server is a *client* of the resident tray instance of ITS copy: it sends each command over
that copy's control channel (a named pipe on Windows, `\\.\pipe\moonpool` installed or
`\\.\pipe\moonpool-<id>` portable; a Unix socket on Linux) and returns the real outcome.
Whether Moonpool is running is decided by pinging that channel. Most tools need Moonpool
already running; call `moonpool_bootup_launcher` first if it is not. `moonpool_list_apps` is
answered live by the running hub and fails with "Moonpool is not running" rather than
returning a stale list. `state.json` (below) is still written for scripts, but it is not
deleted when Moonpool quits, so a leftover file does not mean Moonpool is running.

### Checking what's running

Moonpool continuously writes a status snapshot to:

```
<config folder>\state.json   (installed: C:\Users\<user>\.moonpool\moonpool-config\state.json)
~/.config/Moonpool/state.json (Linux; $XDG_CONFIG_HOME/Moonpool/state.json if set)
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
`moonpool.exe` path (the copy's own folder, not just the first `Get-Process moonpool`), and
the `state.json` location and shape.

## Icons (optional)

Moonpool resolves each app's sidebar icon in this order:
1. The manifest `"icon"` value: a file path (`C:\\path\\logo.png`), an `http(s)` URL, or a data URI.
2. `<config folder>\icons\<id>.<ext>` (png/ico/svg/jpg/webp), where `<id>` is the app's id.
3. Auto-discovered from the app's own project folder (its `cwd`): Tauri `src-tauri/icons/`,
   Electron `build/icon.*`, web `public/favicon.*` / `public/logo.png`, or a root
   `icon.png`/`logo.png`/`favicon.*`. Works with no config and without the app running.
4. A desktop app's exe icon (running process or its built exe), then a web app's live
   `/favicon.ico`.
5. Otherwise a type glyph.

So you usually don't need to set icons at all (web favicons and desktop exe icons come for free).
To force one, set `"icon"` to a real image path/URL, or drop `<id>.png` in the icons folder.
