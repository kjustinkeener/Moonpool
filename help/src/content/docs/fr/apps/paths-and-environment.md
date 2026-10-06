---
title: "Utiliser des chemins, des jetons MP_HOME et des variables d'environnement"
description: "Utilisez les jetons {MP_HOME} et {MP_DATA} et les chemins ./ dans les entrées d'app, voyez quels champs les développent et réglez env et le dossier de travail."
---

## Jetons

| Jeton | Se développe en |
| --- | --- |
| `{MP_HOME}` | Portable : le dossier contenant `moonpool.exe` (le dossier `.moonpool\`). Installé sous Windows : `%USERPROFILE%\.moonpool`. Linux : `$XDG_CONFIG_HOME/Moonpool`, sinon `~/.config/Moonpool`, le même dossier que `{MP_DATA}`. |
| `{MP_DATA}` | Le dossier de configuration, celui qui contient `apps.json`. |

Un jeton qui ne peut pas être résolu est laissé tel quel.

## Champs qui développent les jetons

| Champ | Jetons | `./` ou `.\` en tête |
| --- | --- | --- |
| `cwd` | oui | oui, ancré à `{MP_HOME}` |
| `command` | oui | non |
| `stopCommand` | oui | non (elle s'exécute dans `cwd`, qui est ancré) |
| `url` | oui | non |
| `icon` | oui | oui, ancré à `{MP_HOME}` |
| valeurs de `env`, `processName`, `note` | non | non |

Un chemin relatif sans `./` (comme `apps\tool`) est laissé tel quel et se résout par rapport au
dossier de travail de Moonpool lui-même, ce qui est rarement ce que vous voulez. Préférez `./` ou un jeton.

```text
./apps/notes                       anchored to {MP_HOME}
{MP_HOME}\apps\notes\notes.exe     token
{MP_DATA}\dumps                    token
apps\tool                          left alone, resolves against Moonpool's working folder
```

```json title="apps.json"
{ "id": "notes", "name": "Notes", "group": "Desktop apps", "type": "desktop",
  "cwd": "./apps/notes",
  "command": "{MP_HOME}\\apps\\notes\\notes.exe",
  "processName": "notes" }
```

Les deux formes continuent de fonctionner lorsque vous déplacez le dossier portable. Un chemin fixe tel que
`C:\tools\notes` ne voyage pas. En mode portable, la boîte de dialogue Modifier l'app marque les valeurs absolues de `cwd`
et `url` avec un badge « non portable ». Voir [Mode portable](/fr/data/portable-mode/).

## Environnement

`env` est un objet de chaînes. La boîte de dialogue le modifie à raison d'un `KEY=VALUE` par ligne ; elle coupe
chaque ligne au premier `=`, supprime les espaces des deux côtés et ignore les lignes qui n'en ont pas.

Dans la boîte de dialogue :

```text
PORT=8091
NODE_ENV=development
```

Dans `apps.json`, comme clé `env` de l'entrée :

```json title="apps.json (one entry)"
{ "id": "habits", "name": "Habits", "group": "Web apps", "type": "web", "command": "python app.py",
  "env": { "PORT": "8091", "NODE_ENV": "development" } }
```

- La commande lancée hérite de l'environnement de Moonpool, plus `env`. Les entrées de `env` l'emportent.
- `env` est aussi appliqué à `stopCommand`.
- Les valeurs sont utilisées telles quelles : pas de développement de `{MP_HOME}` ni de `%VAR%` par Moonpool.
- Moonpool dirige son propre WebView2 vers un dossier de profil privé via
  `WEBVIEW2_USER_DATA_FOLDER`. Les apps lancées n'en héritent pas. Si vous aviez défini
  cette variable vous-même avant de démarrer Moonpool, elles reçoivent votre valeur ; sinon elle n'est pas définie.
  Une entrée `env` peut toujours la remplacer.

## Dossier de travail

La commande et `stopCommand` s'exécutent dans `cwd`. Quand `cwd` est omis, la commande s'exécute dans le
dossier de travail de Moonpool lui-même : définissez donc `cwd` pour tout ce qui utilise des chemins relatifs.
