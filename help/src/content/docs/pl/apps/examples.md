---
title: "Gotowe przykłady apps.json dla typowych konfiguracji aplikacji"
description: "Kompletne, poprawne wpisy apps.json dla serwera deweloperskiego, aplikacji desktopowej, strony statycznej, narzędzia CLI, Docker Compose i aplikacji przenośnej."
---

Każdy fragment to jeden wpis. Umieść je wewnątrz tablicy najwyższego poziomu w `apps.json`, oddzielone
przecinkami. Zmień identyfikatory, nazwy i ścieżki zgodnie z własną konfiguracją.

## Serwer deweloperski web

Działa, dopóki port 5173 odpowiada. Gdy to nastąpi, otwiera się przeglądarka. Zatrzymanie zwalnia też port,
co jest ustawieniem domyślnym dla `web`.

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

## Aplikacja web odczytująca port ze środowiska

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

## Aplikacja desktopowa

Działa, dopóki istnieje proces o nazwie `notes-app`. Zatrzymanie kończy ten proces według nazwy.

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

## Strona statyczna, która jest już hostowana

Bez terminala. Uruchomienie otwiera stronę.

```json title="apps.json"
{
  "id": "team-board",
  "name": "Team board",
  "group": "Docs",
  "type": "static",
  "url": "https://example.com/board"
}
```

## Folder statyczny serwowany przez polecenie

Moonpool uruchamia serwer w terminalu, śledzi go po porcie i otwiera stronę, gdy serwer
odpowie.

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

## Narzędzie CLI

Działa w karcie terminala. Opcja `-NoExit` pozostawia powłokę otwartą po zakończeniu skryptu.

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

`command` już odtwarza kontener i kończy działanie, więc stan „działa” wynika z portu. Zatrzymanie
uruchamia `stopCommand` zamiast kończyć właściciela portu, którym w systemie Windows byłby Docker
Desktop. Zob. [Zatrzymywanie i ponowne uruchamianie](/pl/apps/stop-and-restart/#aplikacje-docker-w-systemie-windows).

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

Jeśli kontener ma pozostać uruchomiony po zatrzymaniu aplikacji, użyj
`"killMode": "none"` i pomiń `stopCommand`.

## Aplikacja przenośna

Ścieżki są zakotwiczone w folderze przenośnym, więc wpis działa także po przeniesieniu folderu.

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

## Zobacz też

- [Przykładowe pulpity](/pl/getting-started/example-dashboards/): pulpity dostarczane z Moonpool.
- [Pola aplikacji](/pl/apps/fields/)
- [Uruchamianie serwera deweloperskiego npm w tle w systemie Windows](/pl/guides/run-npm-dev-server-in-background-windows/)
- [Utrzymywanie skryptu Pythona działającego w tle w systemie Windows](/pl/guides/keep-python-script-running-background-windows/)
