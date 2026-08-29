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
