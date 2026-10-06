---
title: "Démarrer un script ou un serveur de développement automatiquement à l'ouverture de session Windows"
description: "Démarrez Moonpool à l'ouverture de session Windows avec un raccourci du dossier Démarrage, puis lancez-y un serveur ou un script avec PowerShell."
---

Windows offre deux façons habituelles de démarrer quelque chose à l'ouverture de session : un
raccourci dans votre dossier Démarrage (appuyez sur Win+R, tapez `shell:startup`, appuyez sur
Entrée), ou une tâche du Planificateur de tâches avec un déclencheur « À l'ouverture de
session ». L'une comme l'autre exécute un programme ou un script, qui pourrait être directement
la commande de votre serveur de développement, mais rien ne le suit alors, n'affiche sa sortie
ni ne l'arrête pour vous.

## Ce que Moonpool propose

Moonpool n'a pas de réglage de démarrage à l'ouverture de session, et une entrée de
`apps.json` n'a aucun champ qui la lance au démarrage de Moonpool (la liste complète est dans
[Champs d'une app](/fr/apps/fields/) et [settings.json](/fr/data/settings-json/)). Ce que vous
pouvez faire, c'est démarrer vous-même Moonpool à l'ouverture de session, puis laisser un
script lancer les apps de votre choix, avec le même verbe que celui qu'offre la
[ligne de commande](/fr/automation/command-line/).

Enregistrez d'abord l'app comme d'habitude :

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

Enregistrez ensuite ceci sous `start-moonpool-apps.ps1`. Une fois installé, le programme est
`%USERPROFILE%\.moonpool\moonpool.exe` ; pour une copie portable, utilisez le chemin de
l'exe de cette copie.

```powershell title="start-moonpool-apps.ps1"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
Start-Process $mp
Start-Sleep -Seconds 15
& $mp launch site
```

Moonpool doit déjà être en cours d'exécution pour que `launch` lui soit transmis ; sans
instance résidente, la même commande démarre un nouveau Moonpool et le verbe n'est pas
exécuté. Le délai lui laisse le temps de démarrer : augmentez-le sur une machine lente.
Ajoutez une ligne `& $mp launch <id>` par app.

Placez enfin un raccourci vers le script dans le dossier Démarrage, avec cette cible :

```text title="Shortcut target"
powershell.exe -NoProfile -WindowStyle Hidden -File "C:\Users\you\start-moonpool-apps.ps1"
```

Pour vérifier ce qui s'est passé, ajoutez `--ticket t1` à un verbe et lisez le résultat dans
`state.json`
([Lire le résultat](/fr/automation/command-line/#lire-le-résultat)).

## Précautions

- Un serveur de développement démarré ainsi est « managed » par Moonpool comme n'importe quel
  autre : Arrêter et Quitter agissent donc sur lui. Si la même app est déjà en cours
  d'exécution (démarrée à la main, par exemple), Moonpool l'affiche comme en cours
  d'exécution mais non managed.
- Moonpool ne relance pas une app qui se termine, et ne retient pas quelles apps étaient en
  cours d'exécution la dernière fois que vous avez quitté.

## Voir aussi

- [Ligne de commande](/fr/automation/command-line/)
- [Zone de notification, fermeture et réduction](/fr/using/tray-and-closing/)
- [Exécuter un serveur de développement npm en arrière-plan sous Windows](/fr/guides/run-npm-dev-server-in-background-windows/)
