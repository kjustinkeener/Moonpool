---
title: "Trova e termina il processo che usa una porta su Windows (3000, 5173, 8080)"
description: "Trova quale processo occupa la porta 3000 o 5173 su Windows con netstat o PowerShell, terminalo con taskkill e lascia che Moonpool liberi la porta all'arresto."
---

Quando un server di sviluppo non parte perché la porta è già in uso, qualcos'altro è in ascolto
su quella porta. Nel Prompt dei comandi, elenca i processi in ascolto con il relativo ID di
processo, poi terminalo:

```text frame="terminal"
netstat -ano | findstr :3000
taskkill /PID 12345 /F
```

L'ultima colonna della riga `LISTENING` è il PID (`findstr :3000` trova anche `:30001`, quindi
controlla l'indirizzo locale). `tasklist /FI "PID eq 12345"` mostra di quale programma si
tratta. In PowerShell la stessa ricerca è:

```powershell frame="terminal"
Get-NetTCPConnection -LocalPort 3000 -State Listen | Select-Object LocalPort, OwningProcess
Get-Process -Id 12345
Stop-Process -Id 12345 -Force
```

Aggiungi `/T` a `taskkill` per terminare anche i processi figli. I processi di un altro utente o
del sistema possono richiedere una finestra con privilegi elevati (amministratore).

## Il metodo Moonpool

Per un'app che esegui tramite Moonpool non devi cercare il PID. Assegna all'app una `port` e
Arresta la libera. Per un'app `web` è il `killMode` predefinito, qui scritto esplicitamente:

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

Arresta termina prima il terminale avviato da Moonpool, poi termina con la forza qualsiasi cosa
sia ancora in ascolto su `port`. Su Windows è la stessa ricerca vista sopra
(`Get-NetTCPConnection -LocalPort <port> -State Listen`), seguita da `taskkill /PID <pid> /T /F`
per ogni proprietario.

- Se qualcosa che non hai avviato tu occupa la porta, Moonpool mostra l'app come in esecuzione ma
  non «managed by Moonpool». Premi **Arresta**: il passaggio `port` viene comunque eseguito.
- Moonpool si rifiuta di terminare tramite la porta un elenco fisso di processi Windows
  condivisi, come il backend di Docker Desktop, `svchost` e l'host WSL. Per un'app Docker usa
  `killMode` `command` oppure `none`, mai `port`. Vedi
  [App Docker su Windows](/it/apps/stop-and-restart/#app-docker-su-windows).
- Funziona solo per le porte delle app elencate in `apps.json`. Per qualsiasi altra porta, usa i
  comandi all'inizio.
- La modalità `port` termina qualsiasi processo in ascolto, anche una copia che hai avviato a
  mano, quindi usala solo per porte di cui nient'altro sul computer ha bisogno.

## Vedi anche

- [Risolvere EADDRINUSE e «Port 5173 is in use»](/it/support/port-already-in-use/)
- [Arresto e riavvio](/it/apps/stop-and-restart/)
- [Campi delle app](/it/apps/fields/): `port` e `killMode`.
- [Due app usano la stessa porta](/it/support/troubleshooting/#due-app-usano-la-stessa-porta)
