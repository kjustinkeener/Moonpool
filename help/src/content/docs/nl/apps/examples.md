---
title: "Kopieerbare apps.json-voorbeelden voor veelvoorkomende app-opzetten"
description: "Complete, geldige apps.json-items voor een dev-server, een desktop-app, een statische pagina, een CLI-tool, Docker Compose en een draagbare app om te kopiëren en aan te passen."
---

Elk fragment is één item. Zet ze in de array op het hoogste niveau van `apps.json`, gescheiden door
komma's. Pas de id's, namen en paden aan op je eigen situatie.

## Web-dev-server

Actief zolang poort 5173 antwoordt. De browser opent zodra dat zo is. Stoppen maakt ook de poort vrij,
wat de standaard is voor `web`.

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

## Web-app die zijn poort uit de omgeving leest

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

## Desktop-app

Actief zolang er een proces met de naam `notes-app` bestaat. Stoppen beëindigt dat proces op naam.

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

## Statische pagina die al wordt gehost

Geen terminal. Starten opent de pagina.

```json title="apps.json"
{
  "id": "team-board",
  "name": "Team board",
  "group": "Docs",
  "type": "static",
  "url": "https://example.com/board"
}
```

## Statische map die door een commando wordt geserveerd

Moonpool voert de server uit in een terminal, volgt hem via de poort en opent de pagina zodra die
antwoordt.

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

## CLI-tool

Draait in een terminaltabblad. Met `-NoExit` blijft de shell open nadat het script klaar is.

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

`command` maakt de container al opnieuw aan en eindigt, dus Actief komt van de poort. Stoppen
voert `stopCommand` uit in plaats van de eigenaar van de poort te beëindigen, wat onder Windows Docker
Desktop zou zijn. Zie [Stoppen en herstarten](/nl/apps/stop-and-restart/#docker-apps-onder-windows).

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

Als je de container liever laat draaien wanneer je de app stopt, gebruik je
`"killMode": "none"` en laat je `stopCommand` weg.

## Draagbare app

Paden verankeren aan de draagbare map, dus het item werkt nog nadat de map is verplaatst.

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

## Zie ook

- [Voorbeelddashboards](/nl/getting-started/example-dashboards/): de dashboards die met Moonpool worden meegeleverd.
- [App-velden](/nl/apps/fields/)
- [Een npm-dev-server op de achtergrond draaien onder Windows](/nl/guides/run-npm-dev-server-in-background-windows/)
- [Een Python-script op de achtergrond laten draaien onder Windows](/nl/guides/keep-python-script-running-background-windows/)
