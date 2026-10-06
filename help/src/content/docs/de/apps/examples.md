---
title: "apps.json-Beispiele zum Kopieren für gängige App-Setups"
description: "Vollständige, gültige apps.json-Einträge für Entwicklungsserver, Desktop-App, statische Seite, CLI-Tool, Docker Compose und portable App zum Kopieren."
---

Jedes Beispiel ist ein Eintrag. Setzen Sie sie in das Array der obersten Ebene von `apps.json`, durch
Kommas getrennt. Ändern Sie IDs, Namen und Pfade passend zu Ihrem eigenen Setup.

## Web-Entwicklungsserver

Läuft, solange Port 5173 antwortet. Der Browser öffnet sich, sobald das der Fall ist. Stoppen gibt außerdem den Port frei,
was der Standard für `web` ist.

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\code\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173",
  "openBrowser": true
}
```

## Web-App, die ihren Port aus der Umgebung liest

```json title="apps.json"
{
  "id": "habit-tracker",
  "name": "Habit Tracker",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\code\habits",
  "command": "python app.py",
  "port": 8091,
  "url": "http://127.0.0.1:8091",
  "openBrowser": true,
  "env": { "PORT": "8091" },
  "note": "Moved off 8000 to avoid a clash"
}
```

## Desktop-App

Läuft, solange ein Prozess namens `notes-app` existiert. Stoppen beendet diesen Prozess über seinen Namen.

```json title="apps.json"
{
  "id": "notes-app",
  "name": "Notes App",
  "group": "Desktop apps",
  "type": "desktop",
  "cwd": "C:\code\notes-app",
  "command": "npm run tauri dev",
  "processName": "notes-app"
}
```

## Statische Seite, die bereits gehostet wird

Kein Terminal. Starten öffnet die Seite.

```json title="apps.json"
{
  "id": "team-board",
  "name": "Team board",
  "group": "Docs",
  "type": "static",
  "url": "https://example.com/board"
}
```

## Statischer Ordner, der von einem Befehl bereitgestellt wird

Moonpool führt den Server in einem Terminal aus, verfolgt ihn über den Port und öffnet die Seite, sobald er
antwortet.

```json title="apps.json"
{
  "id": "docs-site",
  "name": "Docs",
  "group": "Docs",
  "type": "static",
  "cwd": "C:\code\docs\public",
  "command": "python -m http.server 8090",
  "port": 8090,
  "url": "http://localhost:8090",
  "openBrowser": true,
  "killMode": "port"
}
```

## CLI-Tool

Läuft in einem Terminal-Tab. Das `-NoExit` hält die Shell offen, nachdem das Skript beendet ist.

```json title="apps.json"
{
  "id": "backup",
  "name": "Backup script",
  "group": "CLI tools",
  "type": "cli",
  "cwd": "C:\code\scripts",
  "command": "pwsh -NoLogo -NoProfile -NoExit -Command .\backup.ps1 -Verbose"
}
```

## Docker Compose

`command` erstellt den Container bereits neu und endet, daher stammt „läuft“ vom Port. Stoppen
führt `stopCommand` aus, statt den Besitzer des Ports zu beenden, was unter Windows Docker
Desktop wäre. Siehe [Stoppen und neu starten](/de/apps/stop-and-restart/#docker-apps-unter-windows).

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\code\api",
  "command": "docker compose up -d --build",
  "port": 8080,
  "url": "http://localhost:8080",
  "killMode": "command",
  "stopCommand": "docker compose stop app"
}
```

Wenn der Container beim Stoppen der App lieber weiterlaufen soll, verwenden Sie
`"killMode": "none"` und lassen `stopCommand` weg.

## Portable App

Pfade sind am portablen Ordner verankert, daher funktioniert der Eintrag auch nach dem Verschieben des Ordners.

```json title="apps.json"
{
  "id": "notes",
  "name": "Notes",
  "group": "Desktop apps",
  "type": "desktop",
  "cwd": "./apps/notes",
  "command": "{MP_HOME}\apps\notes\notes.exe",
  "processName": "notes",
  "icon": "{MP_HOME}\icons\notes.png"
}
```

## Siehe auch

- [Beispiel-Dashboards](/de/getting-started/example-dashboards/): die Dashboards, die mit Moonpool ausgeliefert werden.
- [App-Felder](/de/apps/fields/)
- [Einen npm-Entwicklungsserver unter Windows im Hintergrund ausführen](/de/guides/run-npm-dev-server-in-background-windows/)
- [Ein Python-Skript unter Windows dauerhaft im Hintergrund laufen lassen](/de/guides/keep-python-script-running-background-windows/)
