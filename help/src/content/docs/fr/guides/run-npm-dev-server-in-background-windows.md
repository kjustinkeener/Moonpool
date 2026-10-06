---
title: "Exécuter un serveur de développement npm en arrière-plan sous Windows sans fenêtre de terminal"
description: "Gardez npm run dev, Vite ou un autre serveur actif sous Windows sans console à surveiller, puis lancez-le et arrêtez-le depuis la zone de notification."
---

Un serveur de développement lancé avec `npm run dev` s'exécute dans le terminal qui l'a
démarré : fermer cette fenêtre y met fin. La méthode Windows classique pour le garder en vie
est un processus caché, par exemple `Start-Process npm.cmd -ArgumentList "run","dev"
-WindowStyle Hidden` dans PowerShell, mais vous n'avez alors aucune sortie à lire, et
l'arrêter implique de chercher le bon `node.exe` (voir
[Trouver et arrêter le processus qui utilise un port](/fr/guides/find-and-kill-process-using-port-windows/)).

## La méthode Moonpool

Moonpool exécute la commande dans son propre onglet de terminal intégré, à l'intérieur de la
fenêtre du hub : il n'y a donc pas de fenêtre de console séparée à garder ouverte. Masquez le
hub dans la zone de notification et le serveur continue de tourner. Ajoutez l'app une fois :

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173",
  "openBrowser": true
}
```

Cliquez sur la commande **Lancer** de l'app. Le point d'état devient plein dès que `port`
répond, et le navigateur s'ouvre sur `url` grâce à `openBrowser`. Cliquez sur le nom de l'app
pour lire sa sortie dans son propre onglet. **Arrêter** met fin au terminal et à tout ce qu'il
a démarré, et libère le port (`killMode` `port` est la valeur par défaut pour `web`).

## Le garder en cours d'exécution quand vous fermez la fenêtre

Par défaut, le bouton de fermeture quitte Moonpool, et sous Windows quitter arrête toutes les
apps qu'il a lancées. Activez **Fermer vers la zone de notification** dans les
[Paramètres](/fr/using/settings/) : fermer la fenêtre ne fait alors que la masquer. L'icône de
la zone de notification (ou **Afficher Moonpool**) la ramène. Les détails sont dans
[Zone de notification, fermeture et réduction](/fr/using/tray-and-closing/).

## Garder le port prévisible

Moonpool détermine l'état « en cours » à partir de `port`. Vite passe au port libre suivant
quand son port est pris, ce qui laisserait Moonpool surveiller le mauvais. Passez
`--strictPort` pour que Vite s'arrête à la place, et définissez `port` en conséquence :

```text title="command"
npm run dev -- --port 5173 --strictPort
```

Si le port est déjà pris, voir
[Corriger EADDRINUSE et « Port 5173 is in use »](/fr/support/port-already-in-use/).

## Limites

- Moonpool ne relance pas un serveur qui plante. Il affiche l'app comme arrêtée et l'onglet
  affiche `[process exited]`.
- Moonpool ne démarre pas de lui-même à l'ouverture de session Windows. Voir
  [Démarrer un script ou un serveur de développement automatiquement à l'ouverture de session Windows](/fr/guides/start-app-at-windows-login/).

## Voir aussi

- [Champs d'une app](/fr/apps/fields/) : `port`, `openBrowser`, `killMode`.
- [Types d'app](/fr/apps/types/) : comment l'état « en cours » est déterminé pour `web`.
- [Arrêt et redémarrage](/fr/apps/stop-and-restart/)
- [Exemples](/fr/apps/examples/#serveur-de-développement-web)
