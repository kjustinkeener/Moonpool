---
title: "Automatiser Moonpool avec des scripts et des agents IA"
description: "Les trois façons de piloter un Moonpool en cours d'exécution depuis des scripts et des agents IA (MCP, ligne de commande, verbes de contrôle) et ce qu'elles modifient."
---

Moonpool peut être piloté sans toucher à sa fenêtre. Il existe trois interfaces, toutes servies par
le même Moonpool résident (l'instance de la zone de notification, appelée ici le hub).

Chaque copie de Moonpool est son propre hub : la copie installée et chaque copie portable s'exécutent
indépendamment, chacune avec son propre canal de contrôle. Une interface atteint toujours la copie dont
elle utilise le `moonpool.exe`. Voir [Mode portable](/fr/data/portable-mode/#plusieurs-copies-à-la-fois).

| Interface | De quoi il s'agit | Référence |
| --- | --- | --- |
| Serveur MCP | `moonpool.exe mcp`, un serveur [MCP](https://modelcontextprotocol.io) stdio démarré par un hôte IA. | [Configuration de MCP](/fr/automation/mcp-setup/), [Outils MCP](/fr/automation/mcp-tools/) |
| Ligne de commande | `moonpool.exe <verb> [args]`. Une seconde exécution de la même copie transmet le verbe à son hub via le canal de contrôle puis se termine. | [Ligne de commande](/fr/automation/command-line/) |
| Canal de contrôle | Un canal nommé, `\\.\pipe\moonpool` (`\\.\pipe\moonpool-<id>` pour une copie portable), sous Windows et un socket Unix sous Linux, avec une requête JSON par ligne. | [Verbes de contrôle](/fr/automation/control-verbs/) |

## Comment elles se relient

- Le hub possède tout : le lancement des apps, les journaux de session, `apps.json`.
- Le serveur MCP est un client du hub, pas une seconde copie de celui-ci. La plupart des appels d'outils sont
  transmis au hub via le canal de contrôle, et la réponse revient comme résultat
  de l'outil. Les exceptions : `moonpool_bootup_launcher` démarre `moonpool.exe` lui-même ;
  `moonpool_app_output` et les outils de configuration demandent au hub d'écrire un fichier puis le lisent ;
  `moonpool_launcher_paths` ajoute les chemins propres du processus MCP à ceux du hub.
- Savoir si un hub est en cours d'exécution se décide en interrogeant ce canal, pas en cherchant un
  processus. Un hub qui répond est en cours d'exécution ; un canal ou un socket absent signifie qu'il ne l'est pas.
- Chaque interface exécute les mêmes gestionnaires que la fenêtre : un verbe fait donc ce que fait le clic
  correspondant.
- Si aucun hub n'est en cours d'exécution, les outils qui agissent sur lui, y compris `moonpool_list_apps`, refusent avec
  « Moonpool is not running ». Il n'y a pas de liste obsolète. `moonpool_bootup_launcher` le démarre.
  Si quelque chose retient le canal mais ne répond pas en quelques secondes, l'erreur indique qu'un
  processus Moonpool est peut-être bloqué.
- Le serveur MCP ne se rabat plus sur le pilotage d'un hub antérieur au canal de contrôle.
  Mettez à jour cette copie, ou fermez-la et redémarrez-la.

## Ce qui peut modifier des choses

| Peut modifier | Interfaces |
| --- | --- |
| Démarrer, arrêter ou redémarrer une app | MCP, ligne de commande, canal |
| Réécrire `apps.json` | MCP (`moonpool_write_config`, `moonpool_restore_config`), ligne de commande, canal |
| Quitter Moonpool | MCP (`moonpool_shutdown_launcher`), ligne de commande (`quit`), canal |
| Arrêter le processus assistant MCP d'une app | MCP (`moonpool_stop_mcp_server`), canal (`stop-mcp`) |
| Recharger `apps.json`, récupérer à nouveau les icônes, afficher la fenêtre | MCP (`moonpool_reload_config`, `moonpool_refresh_app_icons`, `moonpool_raise_launcher`), ligne de commande (`reload`, `refresh-icons`, `show`), canal |
| Ouvrir une fenêtre ou un onglet de terminal | canal (`open-window`) |
| Effacer les observations mémorisées d'assistants MCP | MCP (`moonpool_reset_mcp_seen`), canal (`reset-mcp-seen`) |

Outils en lecture seule : `moonpool_list_apps`, `moonpool_app_output`, `moonpool_read_config`,
`moonpool_launcher_paths`, `moonpool_window_state`, `moonpool_screenshot`.

## Propriétés de sécurité

- **Les écritures de configuration sont protégées.** Une écriture doit porter le jeton de version de la dernière lecture, un
  jeton obsolète est rejeté, et le nouveau `apps.json` est validé avant toute écriture. Une
  écriture rejetée laisse `apps.json` intact. Voir [Outils MCP](/fr/automation/mcp-tools/#configuration).
- **Les ids d'app sont restreints.** Le serveur MCP n'accepte que des lettres, des chiffres, `.`, `_` et `-`,
  et jamais de `-` en tête : un id ne peut donc pas être lu comme une option de ligne de commande.
- **Les captures d'écran ne concernent que Moonpool.** `moonpool_screenshot` capture l'une des six fenêtres propres de Moonpool
  (`main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`), jamais l'écran ni
  une autre app. Le PNG est construit en mémoire et renvoyé en ligne ; Moonpool ne l'enregistre pas dans un
  fichier.
- **Aucune authentification sur le canal.** Moonpool n'ajoute ni connexion ni jeton au canal de contrôle ou au
  socket. Tout processus qui peut l'ouvrir peut envoyer des verbes. Sous Linux, le fichier socket est
  créé avec le mode `0600` : seul votre propre utilisateur le peut.
- **Les hôtes isolés dans un bac à sable sont détectés.** Si le serveur MCP constate qu'il s'exécute dans un bac à sable
  empaqueté (Store/MSIX), où il verrait une copie privée des fichiers de Moonpool, les outils qui
  lisent ou écrivent des fichiers (`moonpool_app_output`, `moonpool_read_config`,
  `moonpool_write_config`, `moonpool_restore_config`) renvoient une erreur qui l'explique au lieu de
  données obsolètes. Les outils qui n'utilisent que le canal de contrôle ne sont pas bloqués. Voir
  [Configuration de MCP](/fr/automation/mcp-setup/#hôtes-isolés-dans-un-bac-à-sable).

## Plateforme

Le canal de contrôle existe sur toutes les plateformes : un canal nommé sous Windows, un socket Unix sous Linux (emplacement dans [Verbes de contrôle](/fr/automation/control-verbs/#où-il-écoute)). Seul
`screenshot` (et donc `moonpool_screenshot`) est réservé à Windows ; sous Linux, il renvoie
« not supported on this platform ». Les verbes de la ligne de commande fonctionnent sur toutes les plateformes.

## Voir aussi

- [Agents IA : démarrage rapide](/fr/automation/quick-start/)
- [Configuration de MCP](/fr/automation/mcp-setup/)
