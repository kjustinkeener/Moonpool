---
title: "Connecter un agent IA à Moonpool via MCP"
description: "Enregistrez moonpool.exe mcp comme serveur MCP stdio auprès de votre hôte, installé ou portable, et voyez comment Moonpool suit l'assistant MCP propre à une app."
---

L'exécutable de Moonpool est son propre serveur MCP. Enregistrez-le auprès de l'hôte comme serveur stdio
qui exécute `moonpool.exe` avec le seul argument `mcp`.

## Enregistrer le serveur

Pour la copie installée, le programme est `%USERPROFILE%\.moonpool\moonpool.exe`. Pour une copie portable, c'est le `moonpool.exe` situé dans votre dossier `.moonpool\`. Utilisez ce chemin complet comme
`command`. Pour un hôte qui lit un `.mcp.json` :

```json title=".mcp.json" {5}
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

Dans un fichier JSON, les barres obliques inverses doivent être doublées, comme ci-dessus. Un hôte disposant d'un enregistrement en ligne de commande,
tel que Claude Code, peut l'ajouter en une seule étape :

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

Le serveur se présente sous le nom
`moonpool`, parle la révision `2025-06-18` du protocole MCP et expose uniquement des outils (il ne liste ni
ressources ni prompts). Les outils apparaissent à l'agent sous la forme `moonpool_*` ; voir
[Outils MCP](/fr/automation/mcp-tools/).

## Plusieurs Moonpool

Le Moonpool installé et chaque copie portable sont des lanceurs distincts, chacun avec ses propres apps,
et ils peuvent tous s'exécuter en même temps. Le `moonpool.exe mcp` d'une copie pilote toujours cette copie. Pour permettre à un
agent d'en utiliser plusieurs, enregistrez chacune sous un nom distinct, pointant vers l'exe de cette copie :

```json title=".mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    },
    "moonpool-work": {
      "type": "stdio",
      "command": "D:\\Work\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

```powershell frame="terminal"
claude mcp add moonpool-work -- "D:\Work\.moonpool\moonpool.exe" mcp
```

Enregistrer deux copies sous le même nom fait que l'une remplace l'autre dans la plupart des hôtes. Les
noms d'outils sont les mêmes pour toutes les copies : l'hôte les distingue donc par le nom que vous
enregistrez. Une copie portable se présente aussi sous le nom `moonpool (<folder>)` et ses instructions de
serveur nomment le dossier, afin que l'agent voie avec quelle copie il communique.

## Remarques

- `moonpool.exe mcp` n'ouvre jamais de fenêtre et ne démarre jamais l'installeur. Il se termine quand
  l'hôte ferme son entrée.
- Il utilise le dossier de configuration et le canal de contrôle de l'exe depuis lequel il a été démarré : un
  exe portable lit donc les données du dossier portable et pilote cette copie portable. Un exe
  n'est considéré comme portable que tant que `moonpool.portable` se trouve à côté de lui. Tout autre
  `moonpool.exe`, où qu'il soit, utilise le dossier du Moonpool installé
  (`%USERPROFILE%\.moonpool\moonpool-config\`) et pilote le Moonpool installé.
- La plupart des outils nécessitent un Moonpool en cours d'exécution. S'il ne l'est pas, l'agent peut appeler
  d'abord `moonpool_bootup_launcher`.
- `moonpool_launcher_paths` affiche les dossiers utilisés par le hub à côté de ceux que le processus MCP
  résout. Une différence signifie que l'agent regarde un autre `apps.json` que le hub.

## Hôtes isolés dans un bac à sable

Certains hôtes exécutent leurs outils dans un bac à sable empaqueté (Store/MSIX) qui redirige AppData vers une
copie privée propre à chaque paquet. Moonpool le détecte lorsque son dossier de configuration ou son exe se résout sous
un chemin du type `...\Packages\<package>\LocalCache\...`.

Il le détecte aussi lorsque le canal de contrôle répond mais que `state.json` est illisible. Les
outils qui lisent ou écrivent des fichiers (`moonpool_app_output`, `moonpool_read_config`,
`moonpool_write_config`, `moonpool_restore_config`) renvoient alors une erreur qui en nomme la
cause, au lieu de données vides ou obsolètes. Les outils qui n'utilisent que le canal de contrôle, tels que
`moonpool_list_apps`, ne sont pas bloqués tant que le canal est accessible. Si le bac à sable masque aussi le
canal, les outils signalent le bac à sable plutôt que « Moonpool is not running ».
Utilisez plutôt la [ligne de commande](/fr/automation/command-line/) depuis un shell extérieur au bac à sable.

## Apps qui ont leur propre serveur MCP

De nombreuses apps de Moonpool sont elles-mêmes jointes par un hôte MCP via un processus assistant `<exe> mcp`.
Moonpool cherche un processus dont le nom correspond au `processName` de l'app et dont le
premier argument est `mcp`, comme `notes-app.exe mcp`. Si le serveur s'exécute sous un autre nom, comme une copie renommée, définissez le caractère générique `mcpProcessName` de l'app (voir [Champs](/fr/apps/fields/#mcpprocessname)) ; un processus qui y correspond est compté sans l'argument `mcp`.

- Tant qu'un assistant est connecté, la barre latérale de l'app affiche une sous-ligne MCP comme en cours, et
  `moonpool_list_apps` ajoute `[mcp: running]` à la ligne de l'app. L'assistant n'est pas compté
  comme l'app elle-même en cours d'exécution.
- Une fois qu'un assistant a été vu, Moonpool s'en souvient (dans `mcp_seen.json` dans le dossier
  de configuration) : la sous-ligne MCP reste donc visible comme arrêtée, et `moonpool_list_apps` affiche
  `[mcp: stopped]`, après la fin de l'assistant.
- La sous-ligne MCP est contrôlée par le paramètre `showMcpProcesses`
  ([Fenêtre Paramètres](/fr/using/settings/)).
- `moonpool_stop_mcp_server` arrête l'assistant et laisse l'app tranquille. Il n'existe pas d'équivalent
  de démarrage : l'hôte qui possède l'assistant le redémarre à son prochain appel d'outil.

## Si les outils ne fonctionnent pas

- **L'hôte n'affiche aucun outil `moonpool_*`.** Vérifiez que `command` est le chemin complet de
  `moonpool.exe` et que `args` vaut `["mcp"]`, puis redémarrez l'hôte.
- **Chaque outil indique que Moonpool n'est pas en cours d'exécution.** Démarrez Moonpool, ou appelez
  `moonpool_bootup_launcher`. Vérifiez que l'exe enregistré est la copie que vous exécutez.
- **Une modification n'apparaît pas.** Appelez `moonpool_launcher_paths` et comparez les dossiers du hub
  avec ceux du processus MCP. Voir [Hôtes isolés dans un bac à sable](#hôtes-isolés-dans-un-bac-à-sable).

Plus de détails dans [Dépannage](/fr/support/troubleshooting/#erreurs-mcp-et-de-script).

## Voir aussi

- [Donner à un agent IA (Claude Code, Codex, Cursor) un serveur MCP pour démarrer et arrêter des apps locales](/fr/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/)
- [Outils MCP](/fr/automation/mcp-tools/)
- [Agents IA : démarrage rapide](/fr/automation/quick-start/)
