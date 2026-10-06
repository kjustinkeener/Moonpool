---
title: "Trouver et arrêter le processus qui utilise un port sous Windows (3000, 5173, 8080)"
description: "Trouvez quel processus occupe le port 3000 ou 5173 sous Windows avec netstat ou PowerShell, terminez-le avec taskkill, ou laissez Moonpool libérer le port."
---

Quand un serveur de développement échoue parce qu'un port est déjà utilisé, quelque chose
d'autre écoute sur ce port. Dans l'Invite de commandes, listez les processus à l'écoute avec
l'ID du processus propriétaire, puis arrêtez-le :

```text frame="terminal"
netstat -ano | findstr :3000
taskkill /PID 12345 /F
```

La dernière colonne de la ligne `LISTENING` est le PID (`findstr :3000` correspond aussi à
`:30001`, lisez donc l'adresse locale). `tasklist /FI "PID eq 12345"` indique de quel programme
il s'agit. Dans PowerShell, la même recherche s'écrit :

```powershell frame="terminal"
Get-NetTCPConnection -LocalPort 3000 -State Listen | Select-Object LocalPort, OwningProcess
Get-Process -Id 12345
Stop-Process -Id 12345 -Force
```

Ajoutez `/T` à `taskkill` pour terminer aussi les processus enfants. Les processus qui
appartiennent à un autre utilisateur ou au système peuvent nécessiter une fenêtre avec
élévation (administrateur).

## La méthode Moonpool

Pour une app que vous exécutez via Moonpool, inutile de chercher le PID. Donnez à l'app un
`port`, et Arrêter le libère. Pour une app `web`, c'est le `killMode` par défaut, écrit ici
explicitement :

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

Arrêter met d'abord fin au terminal lancé par Moonpool, puis termine de force tout ce qui
écoute encore sur `port`. Sous Windows, c'est la même recherche que ci-dessus
(`Get-NetTCPConnection -LocalPort <port> -State Listen`), suivie de
`taskkill /PID <pid> /T /F` pour chaque propriétaire.

- Si quelque chose que vous n'avez pas démarré occupe le port, Moonpool affiche l'app comme
  en cours d'exécution mais non « managed by Moonpool ». Appuyez sur **Arrêter** : l'étape
  `port` s'exécute quand même.
- Moonpool refuse de terminer par port une liste fixe de processus Windows partagés, comme le
  backend de Docker Desktop, `svchost` et l'hôte WSL. Pour une app Docker, utilisez
  `killMode` `command` ou `none`, jamais `port`. Voir
  [Apps Docker sous Windows](/fr/apps/stop-and-restart/#apps-docker-sous-windows).
- Cela ne fonctionne que pour les ports des apps listées dans `apps.json`. Pour tout autre
  port, utilisez les commandes du début.
- Le mode `port` termine tout ce qui écoute, y compris une copie que vous avez lancée à la
  main : ne l'utilisez donc que pour des ports dont rien d'autre sur la machine n'a besoin.

## Voir aussi

- [Corriger EADDRINUSE et « Port 5173 is in use »](/fr/support/port-already-in-use/)
- [Arrêt et redémarrage](/fr/apps/stop-and-restart/)
- [Champs d'une app](/fr/apps/fields/) : `port` et `killMode`.
- [Deux apps utilisent le même port](/fr/support/troubleshooting/#deux-apps-utilisent-le-même-port)
