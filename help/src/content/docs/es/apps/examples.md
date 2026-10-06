---
title: "Ejemplos de apps.json para copiar y pegar en configuraciones habituales"
description: "Entradas de apps.json completas y válidas para un servidor de desarrollo, una app de escritorio, una página estática, una herramienta CLI, Docker Compose y una app portable."
---

Cada fragmento es una entrada. Colócalos dentro de la matriz de nivel superior de `apps.json`, separados
por comas. Cambia los ids, los nombres y las rutas para que coincidan con tu propia configuración.

## Servidor de desarrollo web

En ejecución mientras el puerto 5173 responde. El navegador se abre cuando lo hace. Detener también
libera el puerto, que es el comportamiento predeterminado de `web`.

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

## App web que lee su puerto del entorno

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

## App de escritorio

En ejecución mientras existe un proceso llamado `notes-app`. Detener termina ese proceso por su nombre.

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

## Página estática que ya está alojada

Sin terminal. Iniciar abre la página.

```json title="apps.json"
{
  "id": "team-board",
  "name": "Team board",
  "group": "Docs",
  "type": "static",
  "url": "https://example.com/board"
}
```

## Carpeta estática servida por un comando

Moonpool ejecuta el servidor en una terminal, lo vigila por puerto y abre la página cuando responde.

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

## Herramienta CLI

Se ejecuta en una pestaña de terminal. `-NoExit` mantiene abierto el shell cuando el script termina.

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

`command` ya recrea el contenedor y termina, así que "en ejecución" depende del puerto. Detener ejecuta
`stopCommand` en lugar de terminar al propietario del puerto, que en Windows sería Docker Desktop.
Consulta [Detener y reiniciar](/es/apps/stop-and-restart/#apps-de-docker-en-windows).

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

Si prefieres dejar el contenedor en ejecución al detener la app, usa
`"killMode": "none"` y elimina `stopCommand`.

## App portable

Las rutas se anclan a la carpeta portable, así que la entrada sigue funcionando después de mover la carpeta.

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

## Véase también

- [Paneles de ejemplo](/es/getting-started/example-dashboards/): los paneles que incluye Moonpool.
- [Campos de las apps](/es/apps/fields/)
- [Ejecutar un servidor de desarrollo de npm en segundo plano en Windows](/es/guides/run-npm-dev-server-in-background-windows/)
- [Mantener un script de Python en ejecución en segundo plano en Windows](/es/guides/keep-python-script-running-background-windows/)
