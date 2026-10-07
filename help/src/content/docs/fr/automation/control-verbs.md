---
title: "Canal de contrôle de Moonpool et référence des verbes"
description: "Comment fonctionne le canal de contrôle de Moonpool (canal nommé ou socket Unix), son protocole et chaque verbe auquel l'app répond, avec arguments et réponses."
---

## Où il écoute

Chaque copie de Moonpool a son propre canal : le Moonpool installé et les copies portables
peuvent donc s'exécuter côte à côte sans répondre l'un pour l'autre. Sous Windows, le Moonpool installé
écoute sur le canal nommé `\\.\pipe\moonpool`. Une copie portable y ajoute un id construit à partir de son
dossier : `\\.\pipe\moonpool-<id>`.

`<id>` est composé de 8 chiffres hexadécimaux dérivés du chemin du dossier `moonpool-config` de la copie : il reste donc
le même pour ce dossier d'un redémarrage ou d'une mise à jour à l'autre, et change si vous déplacez le dossier. Le
`moonpool.exe` d'une copie, y compris `moonpool.exe mcp`, trouve toujours le canal de sa propre copie.

Sous Linux, il écoute à la place sur un socket de domaine Unix, avec le mode `0600` :

| Cas | Chemin du socket |
| --- | --- |
| Normal | `$XDG_RUNTIME_DIR/moonpool.sock` quand cette variable est définie, sinon `moonpool.sock` dans le dossier de configuration de Moonpool |
| Mode portable | `moonpool.sock` dans le dossier de configuration de la copie portable, afin qu'une copie portable n'entre jamais en collision avec une copie installée |
| Chemin trop long pour un socket (environ 100 caractères) | `/tmp/moonpool-<uid>/moonpool.sock`, dans un répertoire que vous seul pouvez ouvrir (`moonpool-<id>.sock` pour une copie portable) |

Un fichier socket laissé par un plantage est détecté et remplacé au démarrage suivant. Un socket sur lequel
quelque chose répond encore n'est jamais repris. Le fichier est supprimé quand Moonpool se ferme
normalement.

Le canal est aussi le moyen par lequel le [serveur MCP](/fr/automation/mcp-setup/) sait si Moonpool
est en cours d'exécution : si un `ping` obtient une réponse, il l'est, et si le canal ou le socket est absent, il ne l'est pas. Les
mêmes verbes sont aussi accessibles depuis la [ligne de commande](/fr/automation/command-line/), sauf les
verbes de diagnostic ci-dessous.

## Protocole

Un objet JSON par ligne en entrée, une ligne JSON en sortie, dans l'ordre. Une connexion peut porter de nombreuses
requêtes.

```json title="request"
{"cmd": "restart", "args": ["my-app"]}
```

```jsonl title="replies"
{"ok": true, "result": "..."}
{"ok": false, "error": "unknown app id: ..."}
```

Une requête et sa réponse depuis PowerShell :

Pour une copie portable, utilisez le nom de son canal (`moonpool-<id>`, indiqué par le verbe `paths`) à la
place de `moonpool`.

```powershell frame="terminal"
$p = New-Object System.IO.Pipes.NamedPipeClientStream('.', 'moonpool', 'InOut')
$p.Connect(2000)
$w = New-Object System.IO.StreamWriter($p); $w.AutoFlush = $true
$r = New-Object System.IO.StreamReader($p)
$w.WriteLine('{"cmd":"ping"}')
$r.ReadLine()
```

```json title="reply"
{"ok":true,"result":"pong"}
```

- `args` est une liste de chaînes et peut être omis. Les autres champs sont ignorés.
- `result` est une chaîne ou null. Les verbes qui renvoient des données structurées les renvoient sous forme de chaîne
  JSON.
- Une ligne qui n'est pas du JSON valide reçoit `{"ok": false, "error": "bad request: ..."}`.
- Un `cmd` inconnu reçoit `unknown cmd: <name>`.
- Un verbe qui passe par la fenêtre (`launch`, `stop`, `restart`, `reload`,
  `refresh-icons`, `help`, `open-window`) reçoit sa réponse quand l'action se termine, ou une erreur de délai
  dépassé après 45 s. Si l'interface de la fenêtre du hub n'est pas chargée, il échoue immédiatement avec `frontend not
  loaded`.
- Un Moonpool qui démarre alors qu'un précédent est encore en train de se fermer retente de lier le canal
  pendant environ 8 secondes. S'il n'y parvient toujours pas, il le consigne dans le journal et continue de s'exécuter sans canal.

## Verbes

| Verbe | Args | Résultat |
| --- | --- | --- |
| `ping` | aucun | `pong`. Canal uniquement. |
| `list` | aucun | Chaîne JSON `{"apps": [...], "statuses": [...]}` lue dans la mémoire du hub en cours d'exécution, avec la même forme `apps` et `statuses` que `state.json`. Ajoute `"statusNotReady": true` quand des apps sont enregistrées mais que la première vérification d'état n'a pas encore eu lieu. Tant que `apps.json` ne se charge pas, ajoute `"manifestError": "<message>"` (les apps sont alors la dernière liste chargée) et, quand aucune liste n'a été chargée depuis le démarrage, `"manifestLoaded": false`. Canal uniquement. |
| `show` | aucun | null. Amène la fenêtre au premier plan. |
| `quit` | aucun | null. Quitte Moonpool. |
| `launch` | `<id>` | null en cas de succès, ou `opened` pour une entrée `static` avec seulement une `url`. Erreurs : `unknown app id: <id>`, `did not reach running in time`. |
| `stop` | `<id>` | null en cas de succès, ou `stopped` pour une entrée `static` avec seulement une `url`. Erreur : `still running after stop`. |
| `restart` | `<id>` | Mêmes résultats et erreurs que `launch`. |
| `reload` | aucun | null en cas de succès. |
| `refresh-icons` | aucun | null en cas de succès. |
| `help` | aucun | null. Ouvre la fenêtre d'aide. |
| `dump` | `<id>` [`out-path`] | Chemin du journal de session de l'app, ou de la copie en texte brut dans `out-path`. |
| `paths` | aucun | Rapport multiligne des dossiers et de l'exe utilisés par le hub. |
| `read-config` | aucun | Chemin de `dumps\read-config.json`, qui contient `token`, `valid`, `error`, `path`, `manifest_text`. |
| `write-config` | `<source-file>` [`token`] | Le nouveau jeton de version. Erreurs : `stale token: ...`, `rejected invalid manifest: ...`, `cannot read source ...`. |
| `restore-config` | [`index` ou `filename`] | Sans argument : chemin de `dumps\restore-config.json` (`count`, `snapshots`). Avec un argument : `restored <file> (<n> apps); new version token <token>`. |
| `argv` | les arguments de ligne de commande | null, immédiatement. Les exécute exactement comme le ferait un second `moonpool.exe <args>` de cette copie, y compris `--ticket`. C'est ainsi que ce second lancement transmet ses arguments avant de se terminer. |

Exemples d'échanges :

```jsonl title="request, reply"
{"cmd": "launch", "args": ["nope"]}
{"ok": false, "error": "unknown app id: nope"}

{"cmd": "restore-config", "args": ["1"]}
{"ok": true, "result": "restored 1767225600000.json (6 apps); new version token <token>"}
```

`write-config` et `restore-config` chargent immédiatement le nouveau manifeste, enregistrent un instantané dans
`apps.json.history\` et actualisent la fenêtre.

## Verbes de diagnostic (tests)

Canal uniquement : la ligne de commande ne les accepte pas. Tous fonctionnent sous Windows et Linux
sauf `screenshot`, qui est réservé à Windows et répond `screenshot is not supported on this
platform (Windows only)` ailleurs.

| Verbe | Args | Résultat |
| --- | --- | --- |
| `screenshot` | [`window`] [`max_dim`] | Windows uniquement. Base64 d'un PNG de cette fenêtre de Moonpool (`main` par défaut). `max_dim` facultatif plafonne le plus grand côté en pixels (borné entre 320 et 2400, 320 par défaut ; l'outil MCP utilise toujours la valeur par défaut). Un `max_dim` non entier est une erreur. Fenêtres autorisées : `main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`. Erreurs : `unknown window '<name>'`, `window '<name>' is not open`. Non écrit sur le disque. |
| `open-window` | `<kind>` [`<id>`] | null. Ouvre une fenêtre comme le ferait son élément de menu. `kind` : `settings`, `about`, `installer`, `help`, `themes`, `editor` (`<id>` facultatif ouvre la boîte de dialogue Modifier l'app de cette app, sans id elle ouvre Ajouter une app), `terminal` (`<id>` obligatoire : sélectionne l'onglet de terminal de cette app et élargit le hub pour afficher le volet CLI ; ne la lance pas), `cli` (élargit seulement le hub). Erreurs : `unknown window kind '<kind>'`, `terminal needs an app id`, `unknown app id: <id>`. Reçoit sa réponse via la fenêtre du hub comme `launch`. |
| `window-state` | [`window`] | Chaîne JSON : `{"open":false}`, ou `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. |
| `stop-mcp` | `<id>` | `stopped`. Arrête l'assistant `<processName> mcp` de l'app, pas l'app. Erreurs : `missing app id`, `unknown app id: <id>`. |
| `reset-mcp-seen` | [`<id>`] | `<id>: cleared` ou `<id>: was not marked seen` ; sans id, `cleared <n> entries`. Efface les observations mémorisées d'assistants MCP. |

L'option `--ticket` de la ligne de commande et les enregistrements de résultat de `state.json` appartiennent à l'autre canal ;
voir [Ligne de commande](/fr/automation/command-line/#lire-le-résultat). Les requêtes du canal reçoivent
leur réponse directement.

## Voir aussi

- [Ligne de commande](/fr/automation/command-line/)
- [Agents IA : démarrage rapide](/fr/automation/quick-start/#la-même-action-de-trois-façons)
