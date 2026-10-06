---
title: "Exécuter Moonpool depuis une clé USB ou un dossier synchronisé"
description: "Gardez Moonpool et toutes ses données dans un seul dossier déplaçable pour l'emporter sur une clé USB ou le synchroniser, et exécutez plusieurs copies côte à côte."
---

Le mode portable garde Moonpool et tout ce qu'il écrit dans un seul dossier `.moonpool\`, afin que
vous puissiez l'emporter sur une clé USB ou le déposer dans un dossier synchronisé et l'exécuter sur n'importe quel PC.

## Fonctionnement

Quand vous installez en mode portable, Moonpool crée un dossier `.moonpool\` à l'emplacement que vous
choisissez. Ce dossier contient le programme, votre configuration et son contenu d'aide. Rien
n'est écrit dans AppData de Windows : déplacer ou copier le dossier déplace donc toute votre configuration
avec lui.

```text
<chosen location>\.moonpool\
```

## Différences avec la version installée

| | Installé | Portable |
| --- | --- | --- |
| Programme | `%USERPROFILE%\.moonpool\moonpool.exe` | `<chosen location>\.moonpool\moonpool.exe` |
| Dossier de configuration | `%USERPROFILE%\.moonpool\moonpool-config\` | `<chosen location>\.moonpool\moonpool-config\` |
| Profil de navigateur de la fenêtre, taille et position de la fenêtre | Dans le dossier de configuration | Dans le dossier de configuration, donc ils voyagent aussi |
| Menu Démarrer, raccourci sur le bureau, entrée Ajout/Suppression de programmes | Oui | Aucun |
| Mises à jour | Remplace son propre exe | Idem, à l'intérieur du dossier `.moonpool\`. Voir [Mises à jour](/fr/data/updating/#copies-portables). |
| Suppression | Ajout/Suppression de programmes ou `--uninstall` | Supprimer le dossier |

Aucun des deux modes n'écrit dans AppData de Windows.

### Dossiers synchronisés

Vous pouvez garder une copie portable dans un dossier synchronisé (OneDrive, Dropbox et similaires), mais ne l'exécutez
que sur un PC à la fois. Moonpool écrit `state.json` toutes les quelques secondes et consigne des journaux pendant l'exécution des apps :
deux PC qui exécutent le même dossier se disputent donc les mêmes fichiers, et un conflit de synchronisation peut
laisser un `apps.json` endommagé. Quittez-le sur un PC avant de le démarrer sur un autre.

## Plusieurs copies à la fois

Un seul Moonpool s'exécute par dossier. Le Moonpool installé et un nombre quelconque de copies portables, chacune
dans son propre dossier, peuvent s'exécuter en même temps, et chacune est totalement indépendante : ses propres apps, son icône
de zone de notification, sa fenêtre, ses paramètres, ses journaux et son [canal de contrôle](/fr/automation/control-verbs/).

- L'info-bulle de la zone de notification et le nom dans la barre des tâches indiquent quelle copie est laquelle : `Moonpool` pour la
  copie installée, `Moonpool (<folder>)` pour une copie portable, où `<folder>` est le dossier que vous avez
  choisi (celui qui contient `.moonpool\`).
- Démarrer une deuxième fois la même copie ramène sa fenêtre au premier plan au lieu d'en ouvrir une
  autre. Démarrer une copie différente ouvre cette copie.
- Pour donner à un agent IA l'accès à plusieurs copies, enregistrez chacune sous son propre nom ; voir
  [Configuration de MCP](/fr/automation/mcp-setup/#plusieurs-moonpool).
- Déplacer ou renommer un dossier portable lui donne une nouvelle identité (un nouveau nom de canal de contrôle).
  Quittez-le avant de le déplacer.
- Les copies ne connaissent pas les apps des autres. Deux copies qui démarrent le même serveur sur
  le même port entreront toujours en conflit, et un arrêt qui fonctionne par nom de processus ou par port peut terminer
  quelque chose qu'une autre copie a démarré ; voir
  [Arrêter et redémarrer](/fr/apps/stop-and-restart/#plusieurs-moonpool-ou-vos-propres-processus).

## Faire voyager aussi vos apps

Utilisez le jeton `{MP_HOME}` dans le chemin d'une app pour qu'il pointe à l'intérieur du dossier portable plutôt que
vers un emplacement fixe sur une seule machine. Dans une copie portable, `{MP_HOME}` est le dossier qui
contient `moonpool.exe`, c'est-à-dire le dossier `.moonpool\` lui-même, et non le dossier que vous avez choisi :

```json title="apps.json"
{ "cwd": "{MP_HOME}/my-app" }
```

Ici `{MP_HOME}/my-app` est `<chosen location>\.moonpool\my-app`. Un chemin qui commence par
`./` est ancré de la même façon. Les jetons et les chemins `./` fonctionnent aussi dans un Moonpool installé.
Voir [Chemins et environnement](/fr/apps/paths-and-environment/) pour la façon dont les chemins se résolvent.

## Choisir le mode portable depuis l'installeur

Le mode portable se configure depuis la carte de l'installeur, qui propose **Installer en portable**
à côté de **Installer Moonpool**.

![La carte d'installation : le lien Installer en portable se trouve sous le bouton principal Installer Moonpool](../../../../assets/screenshots/installer-window.png)

Choisissez un dossier et Moonpool y crée le dossier `.moonpool\`, s'y copie lui-même et
démarre la nouvelle copie avec une configuration vierge.

La carte est aussi dans le menu « ... » sous le nom **Installer Moonpool…**, en mode installé comme en mode
portable. Utiliser **Installer en portable** depuis ce menu fait quitter le Moonpool en cours d'exécution et
démarrer la nouvelle copie portable à sa place. Le Moonpool depuis lequel vous avez démarré reste là où il
était : vous pouvez donc le redémarrer ensuite.

Une copie portable démarre vierge et ne copie pas vos apps existantes. Pour les transférer,
quittez la copie portable et copiez `apps.json` à la main :

| | Chemin |
| --- | --- |
| Source (installé) | `%USERPROFILE%\.moonpool\moonpool-config\apps.json` |
| Destination (portable) | `<chosen location>\.moonpool\moonpool-config\apps.json` |

Les entrées avec des chemins absolus fonctionnent toujours sur le même PC, mais elles ne voyagent pas. La boîte de dialogue
Modifier l'app les marque « non portable ».

## Comment Moonpool sait qu'il est portable

Une copie est portable tant qu'un fichier nommé `moonpool.portable` se trouve à côté de son `moonpool.exe`.
Rien d'autre ne la signale, et rien n'est enregistré auprès de Windows.

Pour supprimer une copie portable, quittez-la et supprimez son dossier `.moonpool\`. `--uninstall` ne
supprime que le Moonpool installé, jamais une copie portable.
