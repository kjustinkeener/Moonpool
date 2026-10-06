---
title: Examples
description: Complete, valid apps.json entries for a dev server, a desktop app, a static page, a CLI tool, Docker Compose, and a portable app.
---

Each snippet is one entry. Put them inside the top-level array of `apps.json`, separated by
commas. Change the ids, names and paths to match your own setup.

## Web dev server

Running while port 5173 answers. The browser opens once it does. Stop also frees the port,
which is the default for `web`.

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173",
  "openBrowser": true
}
```

## Web app that reads its port from the environment

```json title="apps.json"
{
  "id": "habit-tracker",
  "name": "Habit Tracker",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\habits",
  "command": "python app.py",
  "port": 8091,
  "url": "http://127.0.0.1:8091",
  "openBrowser": true,
  "env": { "PORT": "8091" },
  "note": "Moved off 8000 to avoid a clash"
}
```

## Desktop app

Running while a process named `notes-app` exists. Stop kills that process by name.

```json title="apps.json"
{
  "id": "notes-app",
  "name": "Notes App",
  "group": "Desktop apps",
  "type": "desktop",
  "cwd": "C:\\code\\notes-app",
  "command": "npm run tauri dev",
  "processName": "notes-app"
}
```

## Static page that is already hosted

No terminal. Launch opens the page.

```json title="apps.json"
{
  "id": "team-board",
  "name": "Team board",
  "group": "Docs",
  "type": "static",
  "url": "https://example.com/board"
}
```

## Static folder served by a command

Moonpool runs the server in a terminal, tracks it by port, and opens the page when it
answers.

```json title="apps.json"
{
  "id": "docs-site",
  "name": "Docs",
  "group": "Docs",
  "type": "static",
  "cwd": "C:\\code\\docs\\public",
  "command": "python -m http.server 8090",
  "port": 8090,
  "url": "http://localhost:8090",
  "openBrowser": true,
  "killMode": "port"
}
```

## CLI tool

Runs in a terminal tab. The `-NoExit` keeps the shell open after the script finishes.

```json title="apps.json"
{
  "id": "backup",
  "name": "Backup script",
  "group": "CLI tools",
  "type": "cli",
  "cwd": "C:\\code\\scripts",
  "command": "pwsh -NoLogo -NoProfile -NoExit -Command .\\backup.ps1 -Verbose"
}
```

## Docker Compose

`command` already recreates the container and exits, so Running comes from the port. Stop
runs `stopCommand` instead of killing the port's owner, which on Windows would be Docker
Desktop. See [Stop and restart](/configuration/stop-and-restart/#docker-apps-on-windows).

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "docker compose up -d --build",
  "port": 8080,
  "url": "http://localhost:8080",
  "killMode": "command",
  "stopCommand": "docker compose stop app"
}
```

If you would rather leave the container running when you stop the app, use
`"killMode": "none"` and drop `stopCommand`.

## Portable app

Paths anchor to the portable folder, so the entry still works after the folder moves.

```json title="apps.json"
{
  "id": "notes",
  "name": "Notes",
  "group": "Desktop apps",
  "type": "desktop",
  "cwd": "./apps/notes",
  "command": "{MP_HOME}\\apps\\notes\\notes.exe",
  "processName": "notes",
  "icon": "{MP_HOME}\\icons\\notes.png"
}
```

## See also

- [Example dashboards](/using/example-dashboards/): the dashboards that ship with Moonpool.
- [App fields](/configuration/fields/)
