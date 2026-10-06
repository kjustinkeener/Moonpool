---
title: "Esempi di apps.json da copiare per le configurazioni più comuni"
description: "Voci apps.json complete e valide per un server di sviluppo, un'app desktop, una pagina statica, uno strumento CLI, Docker Compose e un'app portatile."
---

Ogni frammento è una voce. Inseriscili nell'array di primo livello di `apps.json`, separati da
virgole. Cambia id, nomi e percorsi in base alla tua configurazione.

## Server di sviluppo web

È in esecuzione finché la porta 5173 risponde. Il browser si apre non appena risponde. Arresta
libera anche la porta, che è il comportamento predefinito per `web`.

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

## App web che legge la porta dall'ambiente

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

## App desktop

È in esecuzione finché esiste un processo chiamato `notes-app`. Arresta termina quel processo
in base al nome.

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

## Pagina statica già ospitata

Nessun terminale. Avvia apre la pagina.

```json title="apps.json"
{
  "id": "team-board",
  "name": "Team board",
  "group": "Docs",
  "type": "static",
  "url": "https://example.com/board"
}
```

## Cartella statica servita da un comando

Moonpool esegue il server in un terminale, lo monitora tramite la porta e apre la pagina quando
risponde.

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

## Strumento CLI

Viene eseguito in una scheda del terminale. `-NoExit` mantiene aperta la shell al termine dello
script.

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

`command` ricrea già il container e termina, quindi «in esecuzione» deriva dalla porta. Arresta
esegue `stopCommand` invece di terminare il proprietario della porta, che su Windows sarebbe
Docker Desktop. Vedi [Arresto e riavvio](/it/apps/stop-and-restart/#app-docker-su-windows).

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

Se preferisci lasciare il container in esecuzione quando arresti l'app, usa
`"killMode": "none"` ed elimina `stopCommand`.

## App portatile

I percorsi sono ancorati alla cartella portatile, quindi la voce continua a funzionare anche
dopo aver spostato la cartella.

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

## Vedi anche

- [Dashboard di esempio](/it/getting-started/example-dashboards/): le dashboard fornite con Moonpool.
- [Campi delle app](/it/apps/fields/)
- [Eseguire un server di sviluppo npm in background su Windows](/it/guides/run-npm-dev-server-in-background-windows/)
- [Mantenere in esecuzione in background uno script Python su Windows](/it/guides/keep-python-script-running-background-windows/)
