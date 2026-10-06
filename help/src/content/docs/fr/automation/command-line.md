---
title: "Piloter Moonpool depuis la ligne de commande"
description: "Pilotez un Moonpool en cours avec les verbes de moonpool.exe depuis un terminal ou un script, étiquetez une commande d'un ticket et lisez le résultat dans state.json."
---

Lancer à nouveau un `moonpool.exe` alors que ce même Moonpool est déjà en cours d'exécution n'ouvre pas une
seconde fenêtre. Le second processus transmet ses arguments au premier via son
[canal de contrôle](/fr/automation/control-verbs/) puis se termine. Moonpool doit déjà être en cours d'exécution :
si aucune instance n'est résidente, la même commande démarre un nouveau Moonpool et le verbe n'est pas exécuté.

« Le même Moonpool » signifie le même dossier. Le Moonpool installé et chaque copie portable
s'exécutent indépendamment : une commande atteint donc la copie dont vous avez lancé le `moonpool.exe`, jamais une autre.
Voir [Mode portable](/fr/data/portable-mode/#plusieurs-copies-à-la-fois).

Utilisez le chemin de la copie voulue. Pour la copie installée :

```powershell frame="terminal"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
```

Lorsque plusieurs copies s'exécutent, `Get-Process moonpool` les liste toutes : choisissez donc selon `Path`
plutôt que de prendre la première. Il liste aussi les assistants `moonpool.exe mcp` inactifs démarrés par des
hôtes MCP : un processus `moonpool` ne prouve donc pas qu'un hub est en cours d'exécution. Interrogez plutôt le canal de contrôle
avec `ping` ([Verbes de contrôle](/fr/automation/control-verbs/)).

## Verbes

Le verbe n'est pas sensible à la casse. `<id>` est l'`id` d'une app dans `apps.json`.

| Commande | Effet |
| --- | --- |
| `moonpool.exe` | Sans verbe : amène la fenêtre au premier plan. |
| `moonpool.exe show` | Amène la fenêtre au premier plan. |
| `moonpool.exe launch <id>` | Démarre l'app et ouvre son onglet de terminal. |
| `moonpool.exe stop <id>` | Arrête l'app. |
| `moonpool.exe restart <id>` | Arrête, attend la libération du port et du processus, démarre. |
| `moonpool.exe reload` | Relit `apps.json`. |
| `moonpool.exe refresh-icons` | Récupère à nouveau chaque icône. |
| `moonpool.exe help` | Ouvre la fenêtre d'aide. |
| `moonpool.exe quit` | Quitte Moonpool, comme le menu de la zone de notification. |
| `moonpool.exe dump <id> [out-path]` | Sans `out-path`, indique le chemin du journal de l'app pour cette session. Avec, y copie le journal en texte brut, codes ANSI supprimés. |
| `moonpool.exe paths` | Indique le dossier de configuration, `apps.json`, `state.json`, le journal, le dossier dumps, le dossier icons, l'indicateur portable et le chemin de l'exe utilisés par le Moonpool en cours d'exécution. |
| `moonpool.exe read-config` | Écrit `dumps\read-config.json` dans le dossier de configuration, contenant `token`, `valid`, `error`, `path` et `manifest_text` (le contenu exact de `apps.json`). |
| `moonpool.exe write-config <file> [token]` | Remplace `apps.json` par le manifeste de `<file>`, si le manifeste est valide et, quand `token` est fourni, si `apps.json` y correspond toujours. |
| `moonpool.exe restore-config [index or filename]` | Sans argument, écrit la liste des instantanés dans `dumps\restore-config.json`. Avec un argument, restaure cet instantané s'il est valide. |

```powershell frame="terminal"
& $mp restart my-app
```

```powershell frame="terminal"
& $mp dump my-app C:\temp\my-app.log
```

Un verbe inconnu est ignoré. Le programme possède aussi ses propres arguments de démarrage :
`moonpool.exe mcp` ([Configuration de MCP](/fr/automation/mcp-setup/)), `--uninstall` (utilisé par Ajout/Suppression de
programmes) et `--wait-pid <pid>` (utilisé quand Moonpool se relance lui-même). Ils ne sont pris en compte
que comme premier argument : un id d'app tel que `--uninstall` ne peut donc pas les déclencher.

## Lire le résultat

La ligne de commande n'affiche rien : étiquetez donc une commande avec `--ticket <key>` (n'importe quelle clé unique, à
n'importe quelle position) et lisez le résultat dans `state.json` dans le dossier de configuration. Il s'agit de
`%USERPROFILE%\.moonpool\moonpool-config\` pour la copie installée, de `<your .moonpool folder>\moonpool-config\`
pour une copie portable, et de `~/.config/Moonpool/` sous Linux (voir
[Vue d'ensemble de la configuration](/fr/apps/apps-json/#où-se-trouve-la-configuration)). `show` et `quit`
n'écrivent aucun ticket.

```powershell frame="terminal"
& $mp launch my-app --ticket t1
```

`state.json` contient `apps`, `statuses` (`id`, `running`, `managed`, `mcpRunning`, `mcpSeen` par
app) et `tickets`. Le Moonpool en cours d'exécution le réécrit toutes les quelques secondes et après chaque
commande, et ne le supprime pas quand il se ferme : un fichier restant ne signifie donc pas que Moonpool
est en cours d'exécution. Pour le savoir, ou pour obtenir la liste des apps en direct, utilisez les verbes `ping`
et `list` du canal de contrôle ([Verbes de contrôle](/fr/automation/control-verbs/)) ou les outils MCP. Interrogez
votre ticket jusqu'à ce que `status` ne soit plus `pending` :

| `status` | Signification |
| --- | --- |
| `pending` | Reçu ; Moonpool est encore en train de le traiter. |
| `ok` | Terminé. Pour `dump`, `read-config`, `write-config`, `restore-config` et `paths`, `detail` contient le chemin, le jeton ou le rapport. |
| `error` | Échec ; `detail` en donne la raison, par exemple `unknown app id: x`, `did not reach running in time`, `unknown command`. |

Chaque ticket a la forme `{ ticket, action, arg, status, detail, ts }` avec `ts` en millisecondes Unix :

```json title="state.json (tickets entry)"
{
  "ticket": "t1",
  "action": "launch",
  "arg": "my-app",
  "status": "error",
  "detail": "did not reach running in time",
  "ts": 1767225600000
}
```

Les tickets terminés sont supprimés au bout de 24 heures, et la liste est réduite vers 50 entrées dès que
les tickets terminés ont au moins 5 minutes.

Un agent compatible MCP peut se passer de l'interrogation : voir [Configuration de MCP](/fr/automation/mcp-setup/).

## Voir aussi

- [Agents IA : démarrage rapide](/fr/automation/quick-start/)
- [Verbes de contrôle](/fr/automation/control-verbs/)
