---
title: "Laisser un agent IA configurer et piloter Moonpool : démarrage rapide"
description: "Trois façons de laisser un agent IA ou un script configurer et piloter Moonpool, laquelle choisir selon votre agent, et la même action montrée avec chacune."
---

Il y a trois points d'entrée. Choisissez selon ce que votre agent sait faire.

| Vous voulez | Utilisez | Commencez ici |
| --- | --- | --- |
| Qu'un agent trouve vos apps et les ajoute, une seule fois | **Copier le prompt** sur l'écran vide du hub | Ci-dessous |
| Qu'un agent démarre, arrête et lise des apps par appels d'outils | Le serveur MCP, `moonpool.exe mcp` | [Configuration de MCP](/fr/automation/mcp-setup/) |
| Un script, ou un agent sans MCP | Les verbes de la ligne de commande | [Ligne de commande](/fr/automation/command-line/) |

## Copier le prompt

Quand aucun onglet n'est ouvert, le volet CLI affiche un prompt prêt à l'emploi (« Vous débutez ? Donnez ceci à un agent IA pour qu'il configure vos apps : »). **Copier le prompt** le place dans le presse-papiers. Collez-le dans votre
agent. Il oriente l'agent vers `AI-README.md` et `apps.json` dans votre dossier de configuration et lui demande
de trouver vos apps et de les enregistrer. Une fois terminé, choisissez **Recharger**.

Moonpool réécrit `AI-README.md` à côté de `apps.json` à chaque lancement : il correspond donc toujours
à la version que vous exécutez. N'y conservez pas vos propres modifications.

## La même action de trois façons

| Action | Ligne de commande | Verbe du canal de contrôle | Outil MCP |
| --- | --- | --- | --- |
| Démarrer une app | `moonpool.exe launch <id>` | `launch` | `moonpool_start_app` |
| Arrêter une app | `moonpool.exe stop <id>` | `stop` | `moonpool_stop_app` |
| Redémarrer une app | `moonpool.exe restart <id>` | `restart` | `moonpool_restart_app` |
| Lire la sortie d'une app | `moonpool.exe dump <id> [out-path]` | `dump` | `moonpool_app_output` |
| Lister les apps et leur état | lire `state.json` | `list` | `moonpool_list_apps` |
| Relire `apps.json` | `moonpool.exe reload` | `reload` | `moonpool_reload_config` |
| Lire `apps.json` | `moonpool.exe read-config` | `read-config` | `moonpool_read_config` |
| Remplacer `apps.json` | `moonpool.exe write-config <file> [token]` | `write-config` | `moonpool_write_config` |
| Revenir en arrière sur `apps.json` | `moonpool.exe restore-config [n]` | `restore-config` | `moonpool_restore_config` |
| Afficher la fenêtre | `moonpool.exe show` | `show` | `moonpool_raise_launcher` |
| Démarrer Moonpool | `moonpool.exe` | aucun | `moonpool_bootup_launcher` |
| Quitter Moonpool | `moonpool.exe quit` | `quit` | `moonpool_shutdown_launcher` |
| Afficher les dossiers utilisés | `moonpool.exe paths` | `paths` | `moonpool_launcher_paths` |

La ligne de commande n'affiche rien ; lisez le résultat avec un `--ticket` (voir
[Lire le résultat](/fr/automation/command-line/#lire-le-résultat)). Le canal et MCP
répondent directement.

## Quand les outils d'un agent échouent

- `Moonpool is not running - call moonpool_bootup_launcher first` : démarrez Moonpool, ou laissez
  l'agent appeler cet outil.
- Une modification « n'a pas pris » : demandez à l'agent `moonpool_launcher_paths`. Si les dossiers du hub et
  de MCP diffèrent, l'agent lit un autre `apps.json`. Voir
  [Hôtes isolés dans un bac à sable](/fr/automation/mcp-setup/#hôtes-isolés-dans-un-bac-à-sable).
- Plusieurs copies de Moonpool : enregistrez chacune sous son propre nom. Voir
  [Plusieurs Moonpool](/fr/automation/mcp-setup/#plusieurs-moonpool).

Un exemple détaillé pour Claude Code, Codex et Cursor se trouve dans
[Donner à un agent IA un serveur MCP pour démarrer et arrêter des apps locales](/fr/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/).

Plus de symptômes dans [Dépannage](/fr/support/troubleshooting/#erreurs-mcp-et-de-script).
