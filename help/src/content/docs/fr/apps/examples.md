---
title: "Exemples apps.json à copier pour les configurations courantes"
description: "Des entrées apps.json complètes et valides pour un serveur de développement, une app de bureau, une page statique, un outil CLI, Docker Compose et une app portable."
---

Chaque extrait est une entrée. Placez-les dans le tableau de premier niveau de `apps.json`, séparées par
des virgules. Modifiez les ids, les noms et les chemins pour les adapter à votre configuration.

## Serveur de développement web

En cours tant que le port 5173 répond. Le navigateur s'ouvre dès que c'est le cas. Arrêter libère aussi le port,
ce qui est le comportement par défaut pour `web`.

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

## App web qui lit son port dans l'environnement

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

## App de bureau

En cours tant qu'un processus nommé `notes-app` existe. Arrêter termine ce processus par son nom.

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

## Page statique déjà hébergée

Aucun terminal. Lancer ouvre la page.

```json title="apps.json"
{
  "id": "team-board",
  "name": "Team board",
  "group": "Docs",
  "type": "static",
  "url": "https://example.com/board"
}
```

## Dossier statique servi par une commande

Moonpool exécute le serveur dans un terminal, le suit par son port et ouvre la page dès qu'il
répond.

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

## Outil CLI

S'exécute dans un onglet de terminal. L'option `-NoExit` garde le shell ouvert une fois le script terminé.

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

`command` recrée déjà le conteneur puis se termine : l'état « en cours » vient donc du port. Arrêter
exécute `stopCommand` au lieu de terminer le processus propriétaire du port, qui sous Windows serait Docker
Desktop. Voir [Arrêter et redémarrer](/fr/apps/stop-and-restart/#apps-docker-sous-windows).

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

Si vous préférez laisser le conteneur en cours d'exécution quand vous arrêtez l'app, utilisez
`"killMode": "none"` et supprimez `stopCommand`.

## App portable

Les chemins sont ancrés au dossier portable : l'entrée continue donc de fonctionner après le déplacement du dossier.

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

## Voir aussi

- [Exemples de tableaux de bord](/fr/getting-started/example-dashboards/) : les tableaux de bord livrés avec Moonpool.
- [Champs d'une app](/fr/apps/fields/)
- [Exécuter un serveur de développement npm en arrière-plan sous Windows](/fr/guides/run-npm-dev-server-in-background-windows/)
- [Garder un script Python en cours d'exécution en arrière-plan sous Windows](/fr/guides/keep-python-script-running-background-windows/)
