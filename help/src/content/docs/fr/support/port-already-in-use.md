---
title: "Corriger Error: listen EADDRINUSE: address already in use :::3000 et Vite Port 5173 is in use"
description: "Corrigez EADDRINUSE de Node et Port 5173 is in use de Vite : trouvez ce qui occupe le port, libérez-le et utilisez port et killMode de Moonpool pour l'éviter."
---

```text
Error: listen EADDRINUSE: address already in use :::3000
```

Cette erreur de Node.js signifie qu'un autre processus écoute déjà sur le port 3000 (le `:::` est
la forme IPv6 de « toutes les adresses » ; vous pouvez aussi voir `127.0.0.1:3000`). Il s'agit
souvent d'une copie du même serveur que vous aviez démarrée plus tôt sans jamais l'arrêter.

Vite gère la même situation différemment. Par défaut, il affiche :

```text
Port 5173 is in use, trying another one...
```

et démarre sur le port libre suivant : le serveur est donc actif, mais pas là où vous l'attendez.
Avec `--strictPort` (ou `server.strictPort: true`), Vite s'arrête à la place, avec
`Error: Port 5173 is already in use`.

## Corriger vous-même

1. Trouvez le processus qui détient le port et terminez-le. Sous Windows :

   ```text frame="terminal"
   netstat -ano | findstr :3000
   taskkill /PID 12345 /F
   ```

   Pas à pas, avec la version PowerShell, dans
   [Trouver et terminer le processus qui utilise un port](/fr/guides/find-and-kill-process-using-port-windows/).
2. Ou démarrez votre serveur sur un autre port, par exemple `PORT=3001` pour de nombreux serveurs
   Node ou `--port 5174` pour Vite.

## Comment Moonpool vous aide

Si vous exécutez le serveur via Moonpool, définissez `port` dans son entrée. Moonpool alors :

- affiche l'app comme en cours d'exécution tant que quelque chose répond sur ce port, de sorte
  qu'un serveur résiduel qui l'occupe apparaît comme en cours d'exécution mais pas « managed by
  Moonpool » (géré par Moonpool) ;
- lors de **Arrêter** et de **Redémarrer**, termine tout ce qui écoute encore sur `port` quand
  `killMode` vaut `port`, ce qui est la valeur par défaut des apps `web`, de sorte que le
  prochain Lancer trouve le port libre ;
- signale deux apps configurées avec le même `port`.

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "node server.js",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

Moonpool ne vérifie pas le port avant de lancer. Si le port est encore pris, la commande affiche
l'erreur ci-dessus dans l'onglet de terminal de l'app. Cliquez sur **Arrêter** (ce qui libère le
port) puis de nouveau sur **Lancer**.

Pour Vite, passez `--strictPort` et gardez `port` égal au port demandé :

```text title="command"
npm run dev -- --port 5173 --strictPort
```

Sans cela, Vite peut passer au 5174 pendant que Moonpool continue de surveiller le 5173, et le
voyant d'état ne devient jamais plein.

`killMode` `port` termine tout processus sur le port : ne l'utilisez donc que pour des ports dont
rien d'autre n'a besoin. Pour les apps Docker sous Windows, ne l'utilisez jamais. Voir
[Apps Docker sous Windows](/fr/apps/stop-and-restart/#apps-docker-sous-windows).

## Voir aussi

- [Champs d'une app](/fr/apps/fields/) : `port`, `killMode`.
- [Arrêt et redémarrage](/fr/apps/stop-and-restart/)
- [Dépannage](/fr/support/troubleshooting/#deux-apps-utilisent-le-même-port)
- [Exécuter un serveur de développement npm en arrière-plan sous Windows](/fr/guides/run-npm-dev-server-in-background-windows/)
