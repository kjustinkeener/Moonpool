---
title: "Готовые примеры apps.json для типичных настроек приложений"
description: "Полные корректные записи apps.json для сервера разработки, обычного приложения, статической страницы, CLI-инструмента, Docker Compose и портативного приложения: копируйте и адаптируйте."
---

Каждый фрагмент - это одна запись. Поместите их в массив верхнего уровня `apps.json`, разделяя
запятыми. Измените идентификаторы, названия и пути под свою настройку.

## Веб-сервер разработки

Работает, пока порт 5173 отвечает. Как только это происходит, открывается браузер. Остановка также освобождает порт,
это значение по умолчанию для `web`.

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

## Веб-приложение, которое читает порт из окружения

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

## Обычное приложение

Работает, пока существует процесс с именем `notes-app`. Остановка завершает этот процесс по имени.

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

## Статическая страница, которая уже размещена

Без терминала. Запуск открывает страницу.

```json title="apps.json"
{
  "id": "team-board",
  "name": "Team board",
  "group": "Docs",
  "type": "static",
  "url": "https://example.com/board"
}
```

## Статическая папка, которую раздаёт команда

Moonpool запускает сервер в терминале, отслеживает его по порту и открывает страницу, когда он
отвечает.

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

## CLI-инструмент

Выполняется на вкладке терминала. Параметр `-NoExit` оставляет оболочку открытой после завершения скрипта.

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

`command` уже пересоздаёт контейнер и завершается, поэтому состояние «работает» определяется по порту. Остановка
выполняет `stopCommand` вместо завершения владельца порта, которым в Windows был бы Docker
Desktop. См. [Остановка и перезапуск](/ru/apps/stop-and-restart/#приложения-docker-в-windows).

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

Если при остановке приложения вы предпочитаете оставить контейнер работающим, используйте
`"killMode": "none"` и уберите `stopCommand`.

## Портативное приложение

Пути привязаны к портативной папке, поэтому запись продолжает работать и после перемещения папки.

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

## См. также

- [Примеры панелей](/ru/getting-started/example-dashboards/): панели, которые поставляются с Moonpool.
- [Поля приложения](/ru/apps/fields/)
- [Запуск сервера разработки npm в фоне в Windows](/ru/guides/run-npm-dev-server-in-background-windows/)
- [Постоянная работа скрипта Python в фоне в Windows](/ru/guides/keep-python-script-running-background-windows/)
