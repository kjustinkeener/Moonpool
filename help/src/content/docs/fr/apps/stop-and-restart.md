---
title: "Arrêter un serveur de développement et tout ce qu'il a démarré"
description: "Faites en sorte qu'Arrêter et Redémarrer terminent proprement une app et ses processus enfants avec killMode et stopCommand, y compris Docker sous Windows."
---

Arrêter commence toujours par ceci : Moonpool termine le terminal qu'il a démarré pour l'app, y compris
tout ce que ce terminal a lancé. Pour beaucoup d'apps, cela suffit.

Certaines apps survivent à ce terminal (une fenêtre de bureau se détache du serveur de développement qui l'a
lancée, ou un sous-processus du serveur continue de retenir son port). **`killMode`** choisit une
étape supplémentaire qui s'exécute ensuite.

| `killMode` | Étape supplémentaire à l'arrêt | Lit | Par défaut pour |
| --- | --- | --- | --- |
| `processName` | Force l'arrêt de tous les processus portant ce nom. Sous Windows, leurs enfants aussi (`taskkill /IM <name>.exe /T /F`). Ailleurs `pkill -KILL -x <name>` : correspondance exacte du nom, sensible à la casse, sans les enfants. | `processName` | `desktop` |
| `port` | Force l'arrêt du processus qui écoute sur `port`. | `port` | `web` |
| `command` | Exécute `stopCommand` dans `cwd` et attend sa fin. | `stopCommand`, `cwd`, `env` | aucun |
| `none` | Rien. | rien | `static`, `cli` |

Omettez `killMode` pour obtenir la valeur par défaut du type de l'app, et ne le définissez que lorsque Arrêter laisse
quelque chose en cours d'exécution.

![La liste killMode dans la boîte de dialogue Modifier l'app, réglée sur « par défaut (selon le type) », avec sa ligne d'indication listant le comportement par défaut de chaque type](../../../../assets/screenshots/edit-app-killmode.png)

1. La liste `killMode`. « par défaut (selon le type) » équivaut à omettre la clé.

- Si le champ dont le mode a besoin est vide (le mode `port` sans `port`, par exemple), l'étape
  supplémentaire est ignorée. Ce n'est pas une erreur.
- `killMode` est indépendant de `type` : `port` fonctionne sur une app `cli`, `processName` sur une
  app `web`.
- Une chaîne vide ou une valeur non reconnue ne fait rien de plus. Elle ne revient pas à la valeur par défaut
  du type.

Pour une app de bureau, le mode `processName` exécute l'équivalent de :

```powershell frame="terminal"
taskkill /IM notes-app.exe /T /F
```

## Plusieurs Moonpool, ou vos propres processus

`processName` et `port` ne savent pas qui a démarré un processus. `processName` arrête tous les
processus portant ce nom, et `port` arrête tout ce qui écoute sur le port, y compris un processus
démarré par une autre copie de Moonpool (la copie installée et les copies portables s'exécutent indépendamment ; voir
[Mode portable](/fr/data/portable-mode/#plusieurs-copies-à-la-fois)) et un processus que vous avez démarré vous-même.
N'utilisez ces modes que pour des apps qui n'entreront pas en conflit de cette façon : un nom ou un port qu'aucun autre élément de
la machine n'utilise. Si deux copies enregistrent la même app, ou si vous l'exécutez aussi à la main, donnez-lui
`killMode` `none` ou une `command` qui n'arrête que sa propre instance.

## stopCommand

Utilisée uniquement lorsque `killMode` vaut `command`. Elle s'exécute via `cmd /c` sous Windows et `$SHELL -c`
ailleurs, dans `cwd`, avec votre `env` ajouté. `{MP_HOME}` et `{MP_DATA}` y fonctionnent. Moonpool
attend sa fin avant de faire quoi que ce soit d'autre : un redémarrage ne relance donc jamais l'app tant qu'elle
est encore en cours d'exécution. Son code de sortie est ignoré. Si elle tourne encore après 60 secondes,
Moonpool l'arrête avec ses enfants et continue.

## Redémarrer

Redémarrer, c'est Arrêter suivi de Lancer avec la même `command`. Moonpool attend jusqu'à 4 secondes que
l'ancienne instance apparaisse comme arrêtée (afin que son port soit libre) avant de relancer. Une entrée `static`
avec seulement une `url` n'a rien à arrêter : Redémarrer rouvre simplement la page.

## Apps Docker sous Windows

Utilisez `none`, ou `command` avec une vraie commande d'arrêt telle que `docker compose stop app`. N'utilisez pas
`port`.

Docker Desktop publie le port de chaque conteneur via un seul processus d'arrière-plan partagé. Sous
Windows, « ce qui écoute sur le port » est ce processus partagé : le mode `port` forcerait donc l'arrêt de Docker Desktop
et ferait tomber tous les conteneurs, pas seulement cette app. Par précaution,
Moonpool refuse d'arrêter par port une liste fixe de processus Windows partagés : les processus backend, proxy et service
de Docker Desktop, `dockerd`, `vpnkit`, les processus hôtes WSL et les processus système essentiels
comme `svchost`. Cela ne remplace pas le choix du bon mode.

Si votre `command` recrée déjà le conteneur (`docker compose up -d --build`), `none`
est le bon choix : Redémarrer l'exécute simplement à nouveau.

Voir aussi [Trouver et arrêter le processus qui utilise un port](/fr/guides/find-and-kill-process-using-port-windows/)
et [Corriger EADDRINUSE et « Port 5173 is in use »](/fr/support/port-already-in-use/).

## Exemples

Un serveur de développement qui laisse parfois un processus node retenir son port (c'est la valeur par défaut pour
`web`, indiquée ici explicitement) :

```json title="apps.json"
{ "id": "site", "name": "Site", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
  "killMode": "port" }
```

Une app Docker Compose :

```json title="apps.json"
{ "id": "api", "name": "API", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\api", "command": "docker compose up -d --build", "port": 8080,
  "killMode": "command", "stopCommand": "docker compose stop app" }
```
