---
title: "Garder un script Python en cours d'exécution en arrière-plan sous Windows"
description: "Exécutez en arrière-plan sous Windows un script Python de longue durée ou une petite app web, voyez sa sortie et arrêtez-le proprement, avec pythonw et Moonpool."
---

Un script Python lancé depuis une fenêtre de console s'arrête quand vous fermez cette
fenêtre. Les solutions habituelles sous Windows sont `pythonw.exe` (le même interpréteur sans
fenêtre de console, donc la sortie ne va nulle part), `Start-Process pythonw -ArgumentList
worker.py` pour le lancer détaché, ou une tâche planifiée pour ce qui doit s'exécuter à
l'ouverture de session ou selon une minuterie. Chacune vous laisse chercher le processus dans
le Gestionnaire des tâches quand vous voulez le supprimer.

## La méthode Moonpool

Moonpool exécute la commande dans son propre onglet de terminal : vous gardez la sortie et un
bouton Arrêter sans fenêtre de console à vous. Pour un script qui tourne jusqu'à ce que vous
l'arrêtiez, utilisez une app `cli`. `-u` force Python à vider sa sortie immédiatement, pour que
l'onglet l'affiche en direct :

```json title="apps.json"
{
  "id": "worker",
  "name": "Queue worker",
  "group": "Scripts",
  "type": "cli",
  "cwd": "C:\\code\\worker",
  "command": ".venv\\Scripts\\python.exe -u worker.py"
}
```

Lancez-la et cliquez sur le nom de l'app pour suivre sa sortie. Une app `cli` compte comme en
cours d'exécution tant que sa commande tourne, et devient grise quand le script se termine,
avec `[process exited]` laissé dans l'onglet. **Arrêter** met fin au script et à tout ce
qu'il a démarré. Utiliser le `python.exe` de l'environnement virtuel par son chemin évite
toute étape d'activation.

Si le script sert du HTTP (Flask, FastAPI, `python -m http.server`), faites-en une app `web`
pour que l'état « en cours » suive son port :

```json title="apps.json"
{
  "id": "docs-api",
  "name": "Docs API",
  "group": "Scripts",
  "type": "web",
  "cwd": "C:\\code\\docs-api",
  "command": ".venv\\Scripts\\python.exe -u app.py",
  "port": 8091,
  "url": "http://127.0.0.1:8091",
  "env": { "PORT": "8091" }
}
```

## Limites

- Gardez Moonpool en cours d'exécution. Par défaut, fermer sa fenêtre le quitte, et sous
  Windows quitter arrête toutes les apps qu'il a lancées. Activez **Fermer vers la zone de
  notification** pour masquer la fenêtre à la place ; voir
  [Zone de notification, fermeture et réduction](/fr/using/tray-and-closing/).
- Moonpool ne relance pas un script qui plante, et ne le démarre pas de lui-même à
  l'ouverture de session Windows. Voir
  [Démarrer un script ou un serveur de développement automatiquement à l'ouverture de session Windows](/fr/guides/start-app-at-windows-login/).
- Évitez les guillemets doubles imbriqués dans `command` : l'enveloppe `cmd /c` les déforme.

## Voir aussi

- [Types d'app](/fr/apps/types/#cli) : comment les apps `cli` et `web` sont suivies.
- [Arrêt et redémarrage](/fr/apps/stop-and-restart/)
- [Exemples](/fr/apps/examples/)
- [Journaux](/fr/data/logs/) : où la sortie des sessions est conservée.
