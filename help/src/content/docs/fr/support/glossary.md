---
title: "Glossaire de Moonpool : apps, états, fichiers et paramètres"
description: "Des définitions simples des mots que l'aide de Moonpool emploie pour ses éléments, les états des apps, les fichiers et les paramètres, pour suivre la documentation."
---

## Apps

| Terme | Signification |
| --- | --- |
| app | Un élément géré par Moonpool : un serveur de développement, une app de bureau, une page ou une commande. |
| entrée | L'enregistrement d'une app dans `apps.json`. Employé uniquement quand on parle du JSON. |
| ligne d'app | La ligne d'une app dans la barre latérale, avec son voyant d'état et ses commandes. |
| groupe | Le titre de la barre latérale sous lequel une app est listée, d'après son champ `group`. |
| type | `web`, `desktop`, `static` ou `cli`. Détermine quels champs comptent. Voir [Types d'app](/fr/apps/types/). |
| id | La clé permanente d'une app, utilisée dans les noms de fichiers, les commandes et les outils d'agent. Voir [L'id](/fr/apps/apps-json/#lid). |

## États d'une app

| État | Signification |
| --- | --- |
| démarrage... | Moonpool a lancé l'app mais ne l'a pas encore vue active. Voyant qui pulse. |
| en cours | Son `port` répond, son `processName` existe ou, si aucun des deux n'est défini, le terminal démarré par Moonpool est toujours actif. Voyant plein. Voir [Comment l'état « en cours » est déterminé](/fr/apps/types/#comment-létat-en-cours-est-déterminé). |
| arrêtée | Aucun des cas ci-dessus. Voyant gris. |
| managed (gérée) | Moonpool l'a démarrée dans cette session. Une app en cours d'exécution qui n'est pas gérée a été démarrée autrement, et Quitter n'y touche pas. |

Un onglet de terminal et une app en cours d'exécution sont deux choses distinctes. Cliquer sur le
nom d'une app ouvre seulement son onglet de terminal ; cela ne démarre jamais l'app. Fermer un
onglet n'arrête jamais l'app.

## Fenêtres et éléments

| Terme | Signification |
| --- | --- |
| hub | Le processus Moonpool résident et sa fenêtre principale. Les noms d'outils l'appellent le « launcher ». |
| fenêtre du hub | La fenêtre principale : barre latérale à gauche, panneau CLI à droite. |
| zone de notification | L'icône de la zone de notification du système et son menu (**Afficher Moonpool**, **Quitter**). |
| barre latérale | Le côté gauche de la fenêtre du hub : zone de filtre, menu **...** et lignes d'apps. |
| panneau CLI | Le côté droit de la fenêtre du hub, qui contient les onglets de terminal. |
| onglet de terminal | Le terminal d'une app dans le panneau CLI. |
| sous-ligne MCP | Une ligne atténuée sous une app, qui montre son propre processus auxiliaire `<exe> mcp`. |
| éditeur d'app | La boîte de dialogue Ajouter une app et Modifier l'app. |

## Fichiers et dossiers

| Terme | Signification |
| --- | --- |
| dossier de configuration | Le dossier qui contient `apps.json` et les autres fichiers de Moonpool. Le jeton `{MP_DATA}`. Voir [Où se trouve la configuration](/fr/apps/apps-json/#où-se-trouve-la-configuration). |
| `{MP_HOME}` | Le dossier de Moonpool : `%USERPROFILE%\.moonpool` en mode installé, le dossier `.moonpool\` d'une copie portable, le dossier de configuration sous Linux. |
| session | Une exécution du hub, du démarrage jusqu'à Quitter. |
| journal de session | Le fichier qui contient tout ce qu'une app a affiché pendant une session, sous `cli-output\`. Voir [Journaux](/fr/data/logs/). |
| `moonpool.log` | Le journal de débogage de Moonpool lui-même, écrit uniquement quand **Journaliser les infos de débogage dans un fichier** est activé. |
| dump | Une copie en texte brut d'un journal de session, créée par le verbe `dump`. |
| instantané | Une copie d'un bon `apps.json` dans `apps.json.history\`. Voir [Sauvegarde et récupération](/fr/data/backup-and-recovery/). |

## Modes

| Terme | Signification |
| --- | --- |
| installé | Un Moonpool dans `%USERPROFILE%\.moonpool`, avec raccourci du menu Démarrer et entrée dans Ajout/Suppression de programmes. Windows uniquement. |
| portable | Un Moonpool dans un dossier `.moonpool\` de votre choix, repéré par un fichier `moonpool.portable`. Voir [Mode portable](/fr/data/portable-mode/). |
| copie | Un dossier Moonpool, installé ou portable. Chaque copie s'exécute indépendamment. |

## Arrêt et automatisation

| Terme | Signification |
| --- | --- |
| `killMode` | L'étape supplémentaire qu'Arrêter exécute après avoir mis fin au terminal de l'app. Voir [Arrêt et redémarrage](/fr/apps/stop-and-restart/). |
| `stopCommand` | La commande qu'Arrêter exécute quand `killMode` vaut `command`. |
| `processName` | Le nom de processus que Moonpool surveille, et qu'il termine en mode `processName`. |
| canal de contrôle | Le canal nommé (named pipe, Windows) ou le socket Unix (Linux) sur lequel le hub répond. Voir [Verbes de contrôle](/fr/automation/control-verbs/). |
| verbe | Un mot de commande comme `launch` ou `reload`, donné en ligne de commande ou sur le canal de contrôle. |
| ticket | Une clé que vous joignez avec `--ticket` pour lire le résultat d'une commande dans `state.json`. |
| jeton | L'horodatage de version d'`apps.json` que toute écriture de configuration doit porter. |
| assistant MCP (shim) | Un processus `<exe> mcp` qu'un hôte IA démarre pour accéder aux outils propres d'une app. |
