---
title: "Donner à un agent IA (Claude Code, Codex, Cursor) un serveur MCP pour démarrer et arrêter des apps locales"
description: "Enregistrez Moonpool comme serveur MCP pour que Claude Code, Codex ou Cursor puisse lancer, arrêter, redémarrer vos serveurs de développement et lire leur sortie."
---

Un agent IA de codage exécute généralement votre serveur de développement en tapant
`npm run dev` dans son propre shell. Cela peut bloquer l'agent, laisser un processus orphelin
qui occupe le port, ou démarrer une seconde copie de quelque chose que vous exécutez déjà. Un
serveur MCP permet à l'agent d'appeler des outils pour démarrer et arrêter l'app que vous avez
déjà configurée, au lieu de reconstituer sa ligne de commande.

## La méthode Moonpool

L'exécutable de Moonpool est son propre serveur MCP : enregistrez `moonpool.exe` avec
l'unique argument `mcp` comme serveur stdio. Une fois l'app dans `apps.json`, l'agent la
démarre par son id.

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

Enregistrez le serveur. Dans Claude Code, une seule commande (Moonpool installé ; utilisez le
chemin complet de votre propre exe) :

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

Les hôtes qui lisent un fichier JSON de serveurs MCP, comme le `mcp.json` de Cursor,
acceptent la même forme (antislashs doublés) :

```json title="mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

Pour Codex, ajoutez un serveur avec la même commande et l'argument `mcp` dans sa
configuration (`~/.codex/config.toml`) :

```toml title="config.toml"
[mcp_servers.moonpool]
command = 'C:\Users\you\.moonpool\moonpool.exe'
args = ["mcp"]
```

Le fichier exact et les noms de clés dépendent de chaque hôte : consultez sa documentation MCP
si votre version diffère. Moonpool n'a besoin que du chemin complet de `moonpool.exe` et de
`mcp` comme argument. Redémarrez ensuite l'hôte.

## Ce que l'agent peut faire

Les outils apparaissent sous la forme `moonpool_*`. Ceux du travail quotidien :

| Outil | Usage |
| --- | --- |
| `moonpool_list_apps` | Trouver l'id d'une app et voir si elle est en cours d'exécution. |
| `moonpool_start_app` | Démarrer une app par id et ouvrir son onglet de terminal. |
| `moonpool_stop_app` | L'arrêter, y compris ses processus enfants. |
| `moonpool_restart_app` | Arrêter, attendre que le port se libère, démarrer. À utiliser après une modification du code. |
| `moonpool_app_output` | Lire ce que l'app a affiché, avec `tail_lines` pour limiter la taille. |
| `moonpool_bootup_launcher` | Démarrer Moonpool lui-même s'il n'est pas en cours d'exécution. |

Une boucle typique est `moonpool_restart_app`, puis `moonpool_app_output`. Les autres outils
(lecture et écriture de `apps.json`, captures d'écran) sont décrits dans
[Outils MCP](/fr/automation/mcp-tools/).

## Si cela ne fonctionne pas

Si chaque outil répond `Moonpool is not running - call moonpool_bootup_launcher first`,
Moonpool n'est pas encore démarré. Une modification qui n'apparaît pas signifie généralement
que l'agent regarde un autre `apps.json` : appelez `moonpool_launcher_paths`. Voir
[Si les outils ne fonctionnent pas](/fr/automation/mcp-setup/#si-les-outils-ne-fonctionnent-pas).

## Voir aussi

- [Configuration MCP](/fr/automation/mcp-setup/)
- [Outils MCP](/fr/automation/mcp-tools/)
- [Agents IA : démarrage rapide](/fr/automation/quick-start/)
- [Exécuter un serveur de développement npm en arrière-plan sous Windows](/fr/guides/run-npm-dev-server-in-background-windows/)
