---
title: "Choisir un type d'app : web, desktop, static ou cli"
description: "Découvrez comment les apps web, desktop, static et cli se lancent dans Moonpool, comment l'état en cours est détecté et ce que fait Arrêter par défaut."
---

`type` détermine quels champs comptent et ce que fait Arrêter par défaut.

| | `web` | `desktop` | `static` | `cli` |
| --- | --- | --- | --- | --- |
| Nécessite | `command` | `command` | `url` | `command` |
| Généralement aussi | `port`, `url` | `processName` | `command` et `port`, si elle se sert elle-même | `cwd` |
| Lancer | Exécute `command` dans un onglet de terminal | Exécute `command` dans un onglet de terminal | Sans `command` : ouvre `url` dans le navigateur. Avec une : l'exécute dans un onglet de terminal | Exécute `command` dans un onglet de terminal |
| `killMode` par défaut | `port` | `processName` | `none` | `none` |

![La boîte de dialogue Modifier l'app pour une app web : type réglé sur web avec une description d'une ligne, et un champ port renseigné](../../../../assets/screenshots/edit-app-type-and-port.png)

1. La liste `type`. Sa ligne d'indication décrit ce que fait ce type.
2. Le champ `port`. Pour une app `web`, l'état « en cours » dépend de la réponse de ce port.

## Comment l'état en cours est déterminé

Moonpool vérifie toutes les quelques secondes. Une app est en cours si l'une de ces conditions est remplie, quel que soit
son type :

- `processName` est défini et un processus portant ce nom existe. Les processus assistants `<exe> mcp` de
  Moonpool lui-même ne sont pas comptés.
- `port` est défini et répond en local.
- Moonpool l'a lancée, elle n'a ni `port` ni `processName`, et le processus du terminal
  est toujours vivant.

Une app `cli` est donc en cours tant que sa commande s'exécute, et une app `web` sans `port` se comporte de la
même façon. Une entrée `static` avec seulement une `url` n'a rien à suivre et n'affiche jamais « en cours ».

## web

Un serveur local. Définissez `port` pour que l'état en cours reflète la réponse du serveur, et `url`
ainsi que `openBrowser` pour l'ouvrir dès qu'il démarre.

## desktop

Une app native. Définissez `processName` au nom de l'exécutable pour que l'état en cours survive au détachement de la
fenêtre de la commande qui l'a démarrée. L'arrêt par défaut termine tous les processus portant ce
nom.

## static

Une page. Avec seulement une `url`, Lancer et Redémarrer l'ouvrent dans votre navigateur et Arrêter ne fait rien.
Les URL `http://`, `https://`, `mailto:` et `file://` sont ouvertes, ce qui permet d'ouvrir une page locale :

```json title="apps.json"
{ "id": "csv", "name": "CSV dashboard", "group": "Docs", "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html" }
```

Les pages qui ont besoin d'un serveur (PHP, ou tout ce qui
charge des fichiers locaux) nécessitent une `command` qui en démarre un et un `port` pour le suivre. Voir les
[exemples](/fr/apps/examples/).

## cli

Un outil. `command` s'exécute dans un onglet de terminal dans `cwd`, et l'app cesse d'être en cours quand la
commande se termine. Pour un shell qui reste ouvert, faites de la commande un shell, par exemple
cette `command` :

```text title="command"
pwsh -NoLogo -NoProfile -NoExit -Command python run.py --flag
```

Évitez les guillemets doubles imbriqués dans `command` : ils sont altérés par l'enveloppe `cmd /c`.

![L'onglet de terminal d'une app cli affichant la sortie d'une commande PowerShell et une invite ouverte en dessous](../../../../assets/screenshots/terminal-cli-output.png)

## Ce que fait un clic

Un clic sur le nom d'une app ouvre seulement son onglet de terminal. Utilisez les contrôles Lancer, Arrêter et Redémarrer
pour l'exécuter. Voir [États d'une app](/fr/support/glossary/#états-dune-app).
