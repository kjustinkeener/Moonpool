---
title: "Messages d'erreur de Moonpool expliqués : already running, requires a command et autres"
description: "Retrouvez le texte exact des messages d'erreur de Moonpool, comme already running, requires a command ou stale token, avec leur sens et la solution."
---

Collez le message que vous voyez dans la recherche de la page, ou parcourez les tableaux. Les
messages sont cités tels que Moonpool les affiche : les messages produits par le programme en
arrière-plan restent en anglais, ceux de l'interface suivent la langue choisie. Le texte entre
`<chevrons>` est remplacé par une valeur (un id d'app, un chemin ou une erreur du système). Les
symptômes qui ne sont pas un message d'erreur se trouvent dans
[Dépannage et FAQ](/fr/support/troubleshooting/).

## Démarrage et arrêt d'une app

| Message | Sens et solution |
| --- | --- |
| `already running` | Moonpool détient déjà un terminal pour cette app. Arrêtez-la d'abord, ou utilisez Redémarrer. |
| `stopped during launch` | Arrêter a été actionné alors que le lancement était encore en cours de démarrage. Lancez de nouveau. |
| `app has no launch command` | L'entrée n'a pas de `command`. Ajoutez-en une dans l'éditeur d'app ou dans `apps.json`. Seule une entrée `static` avec une `url` peut s'en passer. |
| `unknown app: <id>` | Aucune app avec cet `id` n'est chargée. Vérifiez l'id, puis Recharger si vous avez modifié `apps.json` à la main. |
| `unknown app id: <id>` | Le même problème, signalé à un script ou à un agent. Listez les apps avec `moonpool_list_apps`. |
| `did not reach running in time` | Depuis un script ou un agent : l'app n'a pas été vue en cours d'exécution dans les 25 secondes. Vérifiez `port` ou `processName`, et lisez la sortie. Voir [Le voyant d'état est incorrect](/fr/support/troubleshooting/#le-voyant-détat-est-incorrect). |
| `still running after stop` | Après 15 secondes, l'app est toujours vue en cours d'exécution. Définissez `killMode`. Voir [Arrêt et redémarrage](/fr/apps/stop-and-restart/). |
| `refusing to open non-web url: <url>` | L'`url` n'est pas `http://`, `https://`, `mailto:` ni `file://`. Corrigez l'`url`. |
| `[process exited]` | Ce n'est pas une erreur : la commande de l'app s'est terminée. Affiché dans l'onglet du terminal (en français : `[processus terminé]`). |

## Validation d'apps.json

Moonpool rejette un `apps.json` qui enfreint une règle et conserve la dernière liste chargée.
`<n>` est la position de l'entrée dans le fichier, en comptant à partir de 1.

| Message | Solution |
| --- | --- |
| `apps.json entry <n> has invalid id "<id>"; use letters, digits, '.', '_', and '-' without a leading '-'` | Renommez l'`id`. |
| `duplicate app id "<id>"` | Deux entrées partagent un `id`. Rendez chacun unique. |
| `apps.json entry <n> (<id>) has an empty name` | Renseignez `name`. |
| `apps.json entry <n> (<id>) has an empty group` | Renseignez `group`. |
| `apps.json entry <n> (<id>) has unknown type "<type>"` | `type` doit être `web`, `desktop`, `static` ou `cli`. |
| `apps.json entry <n> (<id>) has invalid port 0` | `port` doit être compris entre 1 et 65535. |
| `apps.json entry <n> (<id>) requires a url` | Une entrée `static` a besoin d'une `url`. |
| `apps.json entry <n> (<id>) requires a command` | Tous les autres types ont besoin d'une `command`. |

Dans l'éditeur d'app, enregistrer sans nom affiche `le nom est obligatoire.`.
Le texte du bandeau, « apps.json contient une erreur ; affichage de la dernière liste chargée. »
ou « apps.json contient une erreur ; aucune app n'est chargée. », et la façon de récupérer se
trouvent sous [apps.json contient une erreur](/fr/support/troubleshooting/#appsjson-contient-une-erreur).
Si le bandeau indique que l'enregistrement est suspendu, le message se termine par `Repair apps.json and reload it before saving from
Moonpool`. La liste complète des règles est dans [Validation](/fr/apps/apps-json/#validation).

## Paramètres, mises à jour et installateur

| Message | Sens et solution |
| --- | --- |
| `... Repair settings.json and restart Moonpool before changing settings` | `settings.json` est mal formé. Corrigez-le ou supprimez-le, puis redémarrez. Voir [settings.json](/fr/data/settings-json/#lecture-et-réparation). |
| `Échec de la mise à jour : <error>` | Le téléchargement ou l'installation d'une mise à jour a échoué. Voir [Quand une mise à jour échoue](/fr/data/updating/#quand-une-mise-à-jour-échoue). |
| `Échec de la recherche de mises à jour : <error>` | La recherche de mises à jour dans À propos a échoué. Le texte après les deux-points en donne la raison. Réessayez plus tard. |
| `Échec de l'installation : <error>` | L'installateur s'est arrêté à l'étape nommée après les deux-points, par exemple `copy exe: ...`. Quittez tout Moonpool exécuté depuis `%USERPROFILE%\.moonpool` et réessayez. |
| `target folder does not exist` | Le dossier choisi pour une copie portable n'existe plus. Choisissez-en un qui existe. |

## MCP et scripts

| Message | Sens et solution |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Démarrez Moonpool, ou laissez l'agent appeler cet outil. Pour une copie portable, le message nomme la copie. |
| `frontend not loaded` | La fenêtre du hub n'a pas fini de se charger. Attendez et réessayez. |
| `invalid app_id: use only letters, digits, '.', '_', '-' (no leading '-')` | L'agent a transmis un id que le serveur MCP n'accepte pas. Utilisez l'id fourni par `moonpool_list_apps`. |
| `stale token: apps.json changed since it was read ...` | Relisez `apps.json`, réappliquez la modification, puis écrivez. |
| `rejected invalid manifest: ...` | Le nouvel `apps.json` a échoué à la validation (voir ci-dessus). Le fichier n'a pas été modifié. |
| `no console output recorded for '<id>' (not launched this session)` | `moonpool_app_output` a été appelé pour une app qui ne s'est pas exécutée depuis le démarrage de Moonpool. |

Plus de détails dans [Outils MCP](/fr/automation/mcp-tools/) et
[Configuration MCP](/fr/automation/mcp-setup/#si-les-outils-ne-fonctionnent-pas).

## Erreurs d'autres programmes

- [`Error: listen EADDRINUSE: address already in use :::3000` et `Port 5173 is in use`](/fr/support/port-already-in-use/)
- [`Windows protected your PC`](/fr/support/windows-protected-your-pc/)
- [Runtime WebView2 manquant](/fr/support/webview2-runtime-missing/)
