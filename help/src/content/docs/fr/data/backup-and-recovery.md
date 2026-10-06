---
title: "Sauvegarder Moonpool, revenir en arrière sur apps.json et récupérer une configuration"
description: "Sachez quoi sauvegarder, annulez un apps.json défectueux, revenez aux apps d'exemple, passez à une copie portable et voyez ce que la désinstallation supprime."
---

Tout ce que Moonpool conserve se trouve à deux endroits : le dossier de configuration et le dossier des tableaux de bord.
Les chemins pour chaque mode sont indiqués dans [Où se trouve la configuration](/fr/apps/apps-json/#où-se-trouve-la-configuration).

## Le dossier de configuration

```text
moonpool-config\
  apps.json            your apps                              back up
  apps.json.history\   the last 10 good apps.json files       back up (optional)
  settings.json        app settings                           back up
  icons\               icon overrides, <id>.png and so on     back up
  cli-output\<id>\     session logs                           disposable
  moonpool.log         debug log                              disposable
  state.json           live status snapshot                   disposable
  dumps\               files written by dump and read-config  disposable
  mcp_seen.json        which apps had an MCP helper           disposable
  window-state.json    hub window size and position           disposable
  AI-README.md         rewritten at every launch              disposable
  webview\             the window's browser profile (Windows) disposable
```

Le dossier des tableaux de bord est `{MP_HOME}\dashboards` : `%USERPROFILE%\.moonpool\dashboards`
pour la copie installée, `<your .moonpool folder>\dashboards` pour une copie portable, et `dashboards/` dans le dossier de
configuration sous Linux. Sauvegardez-y tout ce qui vous appartient. Son dossier `examples` appartient à Moonpool
et est réécrit à chaque mise à jour.

Le thème est conservé dans le stockage du navigateur de la fenêtre, pas dans un fichier que vous pouvez copier. Il ne
suit pas une sauvegarde ; choisissez-le à nouveau après une restauration.

## Sauvegarder

1. Quittez Moonpool, afin qu'aucun fichier ne soit à moitié écrit.
2. Copiez `apps.json`, `settings.json` et `icons\` depuis le dossier de configuration, ainsi que vos propres fichiers
   de `dashboards\`.

```powershell frame="terminal"
$cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
Copy-Item "$cfg\apps.json", "$cfg\settings.json" D:\backup\
Copy-Item "$cfg\icons" D:\backup\ -Recurse
```

Pour restaurer, quittez Moonpool, recopiez les fichiers et démarrez-le.

## Revenir en arrière sur apps.json

Chaque enregistrement réussi, chaque écriture d'agent et chaque restauration, ainsi que chaque rechargement qui détecte un contenu modifié,
copie le `apps.json` validé dans `apps.json.history\`, en gardant les 10 plus récents. Chaque fichier
est nommé d'après l'heure à laquelle il a été pris, par exemple `1767225600000.json`. Il n'existe pas de
`apps.json.bak`.

- **À la main.** Copiez un instantané par-dessus `apps.json`, puis choisissez **Recharger**.

  ```powershell frame="terminal"
  $cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
  Copy-Item "$cfg\apps.json.history\<snapshot>" "$cfg\apps.json"
  ```

- **Depuis un script.** `moonpool.exe restore-config` liste les instantanés ;
  `moonpool.exe restore-config 1` restaure le plus récent. Voir
  [Ligne de commande](/fr/automation/command-line/).
- **Depuis un agent.** `moonpool_restore_config`. Voir [Outils MCP](/fr/automation/mcp-tools/#configuration).

Rien n'est restauré automatiquement.

## Un fichier endommagé

- **apps.json.** Moonpool n'écrase jamais un fichier endommagé. Voir
  [Si le fichier est incorrect](/fr/apps/apps-json/#si-le-fichier-est-incorrect).
- **settings.json.** Corrigez-le, ou supprimez-le pour réinitialiser tous les paramètres, puis redémarrez Moonpool. Voir
  [settings.json](/fr/data/settings-json/#lecture-et-réparation).

## Revenir aux exemples

Moonpool n'écrit ses apps d'exemple que lorsqu'il n'y a pas de `apps.json`. Pour repartir de zéro, quittez
Moonpool (ou laissez-le en cours d'exécution), renommez ou supprimez `apps.json`, puis démarrez Moonpool ou choisissez
**Recharger**. Un nouveau `apps.json` contenant les exemples est écrit.

## De l'installé au portable

Une nouvelle copie portable démarre avec les apps d'exemple. Pour y transférer les vôtres, voir
[Mode portable](/fr/data/portable-mode/#choisir-le-mode-portable-depuis-linstalleur). Copiez `icons\` et
`settings.json` de la même façon si vous le souhaitez.

## Désinstallation

Désinstaller le Moonpool installé supprime tout le dossier `%USERPROFILE%\.moonpool`,
y compris le dossier de configuration et les tableaux de bord. Sauvegardez d'abord. Voir
[Désinstallation](/fr/getting-started/install/#désinstallation). Une copie portable se supprime en
supprimant son dossier `.moonpool\`.
