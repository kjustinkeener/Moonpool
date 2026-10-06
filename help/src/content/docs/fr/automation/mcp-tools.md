---
title: "Référence des outils MCP de Moonpool : paramètres et résultats"
description: "Chaque outil exposé par le serveur MCP de Moonpool aux agents, avec ses paramètres, ce qu'il renvoie et les cas d'erreur que vous pouvez rencontrer."
---

Tous les outils renvoient du texte, sauf `moonpool_screenshot`, qui renvoie une image PNG. Un échec
revient sous la forme d'un résultat d'outil signalé comme erreur, avec la raison en texte. Pour la configuration, voir
[Configuration de MCP](/fr/automation/mcp-setup/).

Les outils qui prennent `app_id` ont besoin de l'`id` de l'app dans `apps.json`. Il ne doit utiliser que des lettres,
des chiffres, `.`, `_` et `-`, et ne pas commencer par `-`, faute de quoi l'appel échoue avec « invalid
app_id ».

La plupart des outils qui agissent sur le hub échouent avec ce message quand il n'est pas en cours d'exécution.
`moonpool_bootup_launcher`, `moonpool_shutdown_launcher`, `moonpool_raise_launcher` et
`moonpool_launcher_paths` gèrent eux-mêmes ce cas (voir leurs lignes). Pour une copie portable,
le message nomme la copie, par exemple `Moonpool (<folder>)`.

```text
Moonpool is not running - call moonpool_bootup_launcher first
```

Les appels qui attendent un résultat expirent après 45 secondes.

## Lanceur et apps

Exemple de résultat de `moonpool_list_apps` :

```text
site  [running] (managed by Moonpool)  Site
notes-app  [stopped]  [mcp: stopped]  Notes App
```

| Outil | Paramètres | Comportement |
| --- | --- | --- |
| `moonpool_list_apps` | aucun | Une ligne par app : `id  [running]` ou `[stopped]`, `(managed by Moonpool)` le cas échéant, `[mcp: running]` ou `[mcp: stopped]` quand un assistant MCP a été vu, puis le nom. Demandé au hub en cours d'exécution via le canal de contrôle (verbe `list`), donc en direct. Si Moonpool n'est pas en cours d'exécution, il échoue avec « Moonpool is not running » plutôt que d'afficher une liste obsolète. Juste après le démarrage de Moonpool, avant sa première vérification d'état, les apps affichent `[status pending]`. Tant que `apps.json` contient une erreur, le résultat commence par `apps.json has an error: <message>. This list is the last one that loaded; fix the file and call moonpool_reload_config.` Si le fichier était déjà endommagé au démarrage de Moonpool, il indique qu'aucune app n'est chargée et suggère aussi `moonpool_restore_config`. |
| `moonpool_bootup_launcher` | aucun | Démarre Moonpool lui-même et attend jusqu'à 30 s que son canal de contrôle réponde. Renvoie « Moonpool started », ou « Moonpool is already running ». Si le nouveau processus se termine aussitôt (il a passé la main à un Moonpool qui s'arrêtait encore), il en démarre un de plus. Si quelque chose retient le canal sans répondre, il signale qu'un processus Moonpool est peut-être bloqué. |
| `moonpool_shutdown_launcher` | aucun | Identique à Quitter dans le menu de la zone de notification. Attend jusqu'à 30 s que le canal de contrôle disparaisse. Renvoie « Moonpool shut down », ou « Moonpool is not running ». |
| `moonpool_raise_launcher` | aucun | Amène la fenêtre de Moonpool au premier plan. Renvoie « window shown ». Si Moonpool n'est pas en cours d'exécution, le démarre et renvoie « Moonpool was not running; started it ». |
| `moonpool_start_app` | `app_id` (obligatoire) | Démarre l'app et ouvre son onglet de terminal. Renvoie « launched » une fois qu'elle est en cours, ou la raison pour laquelle elle ne l'est pas (`unknown app id: <id>`, `did not reach running in time` après 25 s). Pour une entrée `static` avec seulement une `url`, ouvre la page et renvoie aussi « launched ». |
| `moonpool_stop_app` | `app_id` (obligatoire) | Arrête l'app. Renvoie « stopped », ou une erreur telle que `still running after stop` (après 15 s). |
| `moonpool_restart_app` | `app_id` (obligatoire) | Arrête, attend la libération du port et du processus, démarre. Renvoie « restarted ». |
| `moonpool_app_output` | `app_id` (obligatoire), `tail_lines` (entier, 200 par défaut, minimum 1) | La sortie du terminal de l'app pour la session Moonpool en cours, codes ANSI supprimés. Quand le journal est plus long que `tail_lines`, le texte commence par une ligne indiquant le chemin du journal complet. Échoue avec `no console output recorded for '<id>' (not launched this session)` si l'app n'a pas été exécutée. Si le journal existe mais est vide, renvoie `(no output recorded for '<id>')`. |
| `moonpool_stop_mcp_server` | `app_id` (obligatoire) | Arrête le processus assistant MCP connecté de l'app et laisse l'app en cours d'exécution. Renvoie « stopped ». Ne fait rien si l'app n'a ni `processName` ni `mcpProcessName`. |
| `moonpool_refresh_app_icons` | aucun | Récupère à nouveau chaque icône d'app. Renvoie « icons refreshed ». |

## Configuration

Ces outils lisent et modifient `apps.json` via le hub, jamais le fichier sur le disque. Une écriture doit
porter le jeton de la dernière lecture, un jeton obsolète est rejeté, et le nouveau fichier est validé
avant toute écriture. Passer par le hub est important, car un agent dans un hôte isolé dans un bac à sable
peut voir une copie privée du dossier de configuration au lieu du vrai.

| Outil | Paramètres | Comportement |
| --- | --- | --- |
| `moonpool_read_config` | aucun | Texte JSON avec `manifest_text` (le contenu exact du fichier), `token`, `valid`, `error` (null quand valide) et `path`. `token` vaut `none` quand le fichier est absent ou vide. |
| `moonpool_write_config` | `manifest` (obligatoire, le nouveau texte complet de `apps.json`), `expected_token` (obligatoire, issu de la dernière lecture) | Valide le manifeste et remplace `apps.json`, puis le charge. Renvoie `apps.json updated; new version token <token>`. Un jeton obsolète échoue avec `stale token: apps.json changed since it was read ...`. Un manifeste non valide échoue avec `rejected invalid manifest: ...`. Dans les deux cas, le fichier reste intact. Un `expected_token` vide est refusé. |
| `moonpool_restore_config` | `snapshot` (facultatif) | Sans valeur, texte JSON listant les instantanés enregistrés du plus récent au plus ancien (`index`, `filename`, `millis`, `app_count`, `valid`). Avec un index (1 = le plus récent) ou un nom de fichier, valide cet instantané et le restaure. Renvoie `restored <file> (<n> apps); new version token <token>`. Aucun jeton n'est nécessaire : une restauration écrase volontairement le fichier actuel. |
| `moonpool_reload_config` | aucun | Relit `apps.json`. Renvoie « apps.json reloaded ». Si le fichier ne s'analyse pas ou ne se valide pas, échoue avec `apps.json has an error: ...` et Moonpool conserve la dernière liste chargée. |
| `moonpool_launcher_paths` | aucun | Liste le dossier de configuration du hub, `apps.json`, `state.json`, le journal, le dossier dumps, le dossier icons, l'indicateur portable et le chemin de l'exe, puis le dossier de configuration du processus MCP, `apps.json`, `state.json`, le dossier dumps, l'indicateur portable et le chemin de l'exe (sans journal ni icônes). Si le hub n'est pas en cours d'exécution, sa moitié indique `hub paths unavailable: ...` et la moitié MCP est tout de même affichée. À utiliser quand une modification ne prend pas effet. |

## Avancé : outils de test

`moonpool_screenshot` est réservé à Windows ; sous Linux et macOS, il échoue avec « screenshot is not
supported on this platform ». `moonpool_window_state` et `moonpool_reset_mcp_seen` fonctionnent sur
toutes les plateformes.

`window` vaut `main`, `settings`, `about`, `installer`, `editor`, `help` ou `themes`, et `main` par
défaut. Un nom inconnu échoue avec `unknown window '<name>'`.

| Outil | Paramètres | Comportement |
| --- | --- | --- |
| `moonpool_screenshot` | `window` (facultatif) | Capture le contenu propre de cette fenêtre de Moonpool sous forme de PNG en ligne, de 320 pixels au plus sur son plus grand côté. La taille ne peut pas être augmentée depuis MCP. Échoue avec `window '<name>' is not open` si elle n'est pas affichée. Elle ne peut capturer aucune autre app. |
| `moonpool_window_state` | `window` (facultatif) | Texte JSON : `{"open":false}` quand la fenêtre n'est pas ouverte, sinon `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. Destiné aux tests. |
| `moonpool_reset_mcp_seen` | `app_id` (facultatif) | Test uniquement. Efface l'enregistrement mémorisé « un assistant MCP a été vu » pour une app, ou pour toutes les apps si omis, de sorte que la sous-ligne MCP de la barre latérale se masque à nouveau jusqu'à ce qu'un assistant soit vu. |

## Voir aussi

- [Configuration de MCP](/fr/automation/mcp-setup/)
- [Ligne de commande](/fr/automation/command-line/)
