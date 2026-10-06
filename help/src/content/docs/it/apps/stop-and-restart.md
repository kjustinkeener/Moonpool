---
title: "Arrestare un server di sviluppo e tutto ciò che ha avviato"
description: "Fai terminare con Arresta e Riavvia un'app e i suoi processi figli in modo pulito con killMode e stopCommand, inclusi i default per tipo e Docker su Windows."
---

Arresta fa sempre questo per prima cosa: Moonpool termina il terminale che ha avviato per l'app,
insieme a tutto ciò che quel terminale ha lanciato. Per molte app basta questo.

Alcune app sopravvivono a quel terminale (una finestra desktop si stacca dal server di sviluppo
che l'ha avviata, oppure un sottoprocesso del server continua a occupare la sua porta).
**`killMode`** sceglie un passaggio aggiuntivo che viene eseguito dopo.

| `killMode` | Passaggio aggiuntivo con Arresta | Legge | Predefinito per |
| --- | --- | --- | --- |
| `processName` | Termina forzatamente ogni processo con quel nome. Su Windows anche i suoi figli (`taskkill /IM <name>.exe /T /F`). Altrove `pkill -KILL -x <name>`: corrispondenza esatta del nome, con distinzione tra maiuscole e minuscole, figli esclusi. | `processName` | `desktop` |
| `port` | Termina forzatamente il processo in ascolto su `port`. | `port` | `web` |
| `command` | Esegue `stopCommand` in `cwd` e ne attende il termine. | `stopCommand`, `cwd`, `env` | nessuno |
| `none` | Nulla. | nulla | `static`, `cli` |

Ometti `killMode` per ottenere il comportamento predefinito del tipo di app, e impostalo solo
quando Arresta lascia qualcosa in esecuzione.

![La selezione killMode nella finestra Modifica app, impostata su «predefinito (in base al tipo)», con la riga di suggerimento che elenca cosa fa ogni tipo per impostazione predefinita](../../../../assets/screenshots/edit-app-killmode.png)

1. La selezione `killMode`. «predefinito (in base al tipo)» equivale a omettere la chiave.

- Se il campo richiesto dalla modalità è vuoto (ad esempio la modalità `port` senza `port`), il
  passaggio aggiuntivo viene saltato. Non è un errore.
- `killMode` è indipendente da `type`: `port` funziona su un'app `cli`, `processName` su
  un'app `web`.
- Una stringa vuota o un valore non riconosciuto non fa nulla di più. Non ricade sul
  valore predefinito del tipo.

Per un'app desktop, la modalità `processName` esegue l'equivalente di:

```powershell frame="terminal"
taskkill /IM notes-app.exe /T /F
```

## Più copie di Moonpool, o processi tuoi

`processName` e `port` non sanno chi ha avviato un processo. `processName` termina ogni
processo con quel nome, e `port` termina qualsiasi cosa sia in ascolto sulla porta, compreso un
processo avviato da un'altra copia di Moonpool (la copia installata e quelle portatili sono
indipendenti; vedi
[Modalità portatile](/it/data/portable-mode/#più-copie-contemporaneamente)) e uno che hai avviato
tu stesso. Usa queste modalità solo per app che non entrano in conflitto in questo modo: un
nome o una porta che nient'altro sul computer usa. Se due copie registrano la stessa app, o la
esegui anche a mano, assegnale `killMode` `none` oppure un `command` che arresti solo la sua
istanza.

## stopCommand

Usato solo quando `killMode` è `command`. Viene eseguito tramite `cmd /c` su Windows e
`$SHELL -c` altrove, in `cwd`, con il tuo `env` aggiunto. `{MP_HOME}` e `{MP_DATA}` funzionano
al suo interno. Moonpool ne attende il termine prima di fare qualsiasi altra cosa, quindi un
Riavvia non rilancia mai l'app mentre è ancora in esecuzione. Il suo codice di uscita viene
ignorato. Se è ancora in esecuzione dopo 60 secondi, Moonpool lo termina insieme ai suoi figli e
prosegue.

## Riavvio

Riavvia è Arresta seguito da Avvia dello stesso `command`. Moonpool attende fino a 4 secondi che
la vecchia istanza risulti arrestata (così la sua porta è libera) prima di rilanciarla. Una voce
`static` con il solo `url` non ha nulla da arrestare: Riavvia riapre semplicemente la pagina.

## App Docker su Windows

Usa `none`, oppure `command` con un vero comando di arresto come `docker compose stop app`. Non
usare `port`.

Docker Desktop pubblica la porta di ogni container tramite un unico processo condiviso in
background. Su Windows, «qualsiasi cosa sia in ascolto sulla porta» è quel processo condiviso,
quindi la modalità `port` terminerebbe forzatamente Docker Desktop e farebbe cadere ogni
container, non solo questa app. Come ulteriore protezione, Moonpool rifiuta di terminare per
porta un elenco fisso di processi Windows condivisi: i processi backend, proxy e di servizio di
Docker Desktop, `dockerd`, `vpnkit`, i processi host di WSL e i processi di sistema essenziali
come `svchost`. Questo non sostituisce la scelta della modalità giusta.

Se il tuo `command` ricrea già il container (`docker compose up -d --build`), `none` è la scelta
corretta: Riavvia lo esegue semplicemente di nuovo.

Vedi anche [Trovare e terminare il processo che usa una porta](/it/guides/find-and-kill-process-using-port-windows/)
e [Risolvere EADDRINUSE e «Port 5173 is in use»](/it/support/port-already-in-use/).

## Esempi

Un server di sviluppo che a volte lascia un processo node a occupare la sua porta (è il
comportamento predefinito per `web`, qui indicato esplicitamente):

```json title="apps.json"
{ "id": "site", "name": "Site", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
  "killMode": "port" }
```

Un'app Docker Compose:

```json title="apps.json"
{ "id": "api", "name": "API", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\api", "command": "docker compose up -d --build", "port": 8080,
  "killMode": "command", "stopCommand": "docker compose stop app" }
```
