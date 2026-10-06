---
title: "Tous les champs de apps.json : type, valeur par défaut et rôle"
description: "Consultez chaque clé d'une entrée apps.json avec son type, sa valeur par défaut et les types d'app qui l'utilisent, sous les noms de la boîte Modifier l'app."
---

La boîte de dialogue Modifier l'app affiche les mêmes champs sous les mêmes noms. Les champs qui ne s'appliquent pas au
type sélectionné sont grisés dans la boîte de dialogue mais tout de même enregistrés, à une exception près :
`stopCommand` n'est enregistré que lorsque `killMode` vaut `command`.

![La boîte de dialogue Modifier l'app, de name jusqu'à stopCommand, avec la liste killMode entourée ; les champs inutilisés comme processName et stopCommand sont grisés](../../../../assets/screenshots/edit-app-dialog.png)

1. La liste `killMode`. Les champs qu'elle n'utilise pas restent grisés.

| Champ | Type | Obligatoire | Utilisé par | Rôle |
| --- | --- | --- | --- | --- |
| `id` | chaîne | oui | tous | Clé unique. Lettres, chiffres, `.`, `_`, `-`, ne commençant pas par `-`. Voir [Vue d'ensemble](/fr/apps/apps-json/#lid). |
| `name` | chaîne | oui | tous | Libellé dans la barre latérale. Non vide. |
| `group` | chaîne | oui | tous | Titre de la barre latérale sous lequel l'app est listée. Non vide dans une modification manuelle ; la boîte de dialogue enregistre un groupe vide sous le nom `Apps`. Texte libre ; un nouveau nom crée un nouveau groupe. |
| `type` | chaîne | oui | tous | `web`, `desktop`, `static` ou `cli`. Voir [Types d'app](/fr/apps/types/). |
| `command` | chaîne | tous sauf `static` | tous | Exécutée dans un terminal pour démarrer l'app, via `cmd /c` sous Windows et `$SHELL -c` ailleurs (`/bin/sh` si `SHELL` n'est pas défini). Facultative pour `static`. |
| `cwd` | chaîne | non | tous avec une `command` | Dossier dans lequel la commande s'exécute. Par défaut, le dossier de travail de Moonpool lui-même. Accepte les jetons et `./`. Voir [Chemins et environnement](/fr/apps/paths-and-environment/). |
| `port` | entier, de 1 à 65535 | non | tous | En cours tant que quelque chose répond sur ce port en local (IPv4 ou IPv6). Lu par `killMode` `port`. |
| `processName` | chaîne | non | tous, surtout `desktop` | En cours tant qu'un processus portant ce nom existe. Insensible à la casse, avec ou sans `.exe` : `my-app` correspond donc à `my-app.exe`. Sous Linux, 15 caractères ou moins. Lu par `killMode` `processName`. |
| `mcpProcessName` | chaîne | non | tous avec un `processName` | Motif à caractères génériques pour le nom de processus du serveur MCP de cette app. `*` correspond à n'importe quelle suite de caractères, `?` à un seul caractère. Insensible à la casse, comparé au nom entier, et `.exe` est facultatif. Un processus correspondant est considéré comme le serveur MCP de l'app (la sous-ligne MCP de la barre latérale) et n'a pas besoin de `mcp` comme premier argument. Voir [mcpProcessName](#mcpprocessname). |
| `url` | chaîne | `static` uniquement | `web`, `static` | Page à ouvrir. Seules les URL `http://`, `https://`, `mailto:` et `file://` sont ouvertes. |
| `openBrowser` | booléen, `false` par défaut | non | tout type avec une `url` (la boîte de dialogue le grise pour `desktop` et `cli`) | Ouvre `url` automatiquement dès que Moonpool détecte que l'app est démarrée (voir ci-dessous). |
| `killMode` | chaîne | non | tous | Nettoyage supplémentaire à l'arrêt et au redémarrage : `processName`, `port`, `command` ou `none`. Voir [Arrêter et redémarrer](/fr/apps/stop-and-restart/). |
| `stopCommand` | chaîne | non | `killMode` `command` | Commande exécutée à l'arrêt. Ignorée dans tous les autres modes. |
| `env` | objet de chaînes | non | tous | Variables d'environnement supplémentaires. La boîte de dialogue les modifie à raison d'un `KEY=VALUE` par ligne. |
| `icon` | chaîne | non | tous | Image de la barre latérale : un chemin de fichier, une URL `http(s)` ou un URI `data:`. Définissez-la depuis **Choisir une icône...** dans le menu contextuel de l'app ou à la main. |
| `note` | chaîne | non | tous | Info-bulle affichée quand vous survolez l'app dans la barre latérale. |

Une entrée utilisant `env` et `killMode` :

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000", "NODE_ENV": "development" },
  "killMode": "port"
}
```

Voir [Trouver et arrêter le processus qui utilise un port](/fr/guides/find-and-kill-process-using-port-windows/)
pour comprendre comment `port` et `killMode` fonctionnent ensemble.

## mcpProcessName

Par défaut, Moonpool considère un processus comme le serveur MCP de l'app lorsque son nom correspond à
`processName` et que son premier argument est `mcp`, comme `notes-app.exe mcp`. Définissez
`mcpProcessName` lorsque le serveur s'exécute sous un autre nom : une app qui surveille un exe
alors que son serveur MCP en est un autre (`mog.exe mcp`), ou une copie renommée du serveur.

La valeur est un motif à caractères génériques. `*` correspond à n'importe quelle suite de caractères (y compris vide) et `?`
à exactement un caractère. Il est comparé sans tenir compte de la casse au nom de processus entier, et un
motif sans `.exe` correspond aussi au nom avec `.exe`. Une valeur vide est considérée comme non définie.

```json
{
  "id": "destiny",
  "name": "Destiny",
  "group": "Desktop apps",
  "type": "desktop",
  "processName": "destiny",
  "mcpProcessName": "destiny-mcp-*"
}
```

Cela correspond à une copie renommée telle que `destiny-mcp-2706210170.exe`. Un processus qui correspond à
`mcpProcessName` est le serveur, qu'il ait été démarré avec `mcp` ou non, et il n'est jamais compté
comme l'app elle-même en cours d'exécution. Si le motif correspond aussi à `processName` lui-même (par exemple
`destiny*`), Moonpool exige toujours l'argument `mcp`, afin que la vraie app ne soit jamais confondue
avec son serveur MCP. Voir [Configuration de MCP](/fr/automation/mcp-setup/#apps-qui-ont-leur-propre-serveur-mcp).

## openBrowser

Moonpool ouvre `url` une seule fois, lorsqu'une app lancée par Moonpool apparaît pour la première fois comme en cours. Cela
nécessite un `port` ou un `processName` pour la détecter. Sans l'un ni l'autre, « en cours » signifie seulement que le
processus du terminal est vivant, et le navigateur n'est pas ouvert automatiquement. Désactivez `openBrowser`
si votre commande ouvre elle-même un navigateur. Une entrée `static` sans commande ouvre `url`
chaque fois que vous appuyez sur Lancer, quelle que soit la valeur de `openBrowser`.

Deux apps configurées avec le même `port` sont signalées dans la barre latérale.

## Icônes

L'icône d'une app est la première de cette liste qui existe :

1. Le champ `icon`.
2. `icons\<id>.<ext>` dans le dossier de configuration, par exemple `icons\site.png`.
3. Un fichier d'icône dans le dossier de l'app elle-même (son `cwd`, ou le dossier d'une `url` `file:///`).
4. Pour `desktop`, l'icône de son `.exe` compilé ou en cours d'exécution.
5. Pour `web` et `static`, le `/favicon.ico` du site, une fois le serveur démarré.
6. Un glyphe correspondant au type.

La plupart des apps n'ont pas besoin de réglage d'icône.
