---
title: "Risolvere Error: listen EADDRINUSE: address already in use :::3000 e Vite Port 5173 is in use"
description: "Risolvi EADDRINUSE di Node e Port 5173 is in use di Vite: trova chi occupa la porta, liberala e usa i campi port e killMode di Moonpool per evitarlo."
---

```text
Error: listen EADDRINUSE: address already in use :::3000
```

Questo errore di Node.js significa che un altro processo è già in ascolto sulla porta 3000 (i `:::`
sono la forma IPv6 di «tutti gli indirizzi»; potresti vedere anche `127.0.0.1:3000`). Spesso è
una copia dello stesso server che avevi avviato prima e non hai mai arrestato.

Vite gestisce la stessa situazione in modo diverso. Per impostazione predefinita stampa:

```text
Port 5173 is in use, trying another one...
```

(Porta 5173 in uso, ne provo un'altra...) e si avvia sulla successiva porta libera, quindi il
server è attivo ma non dove te lo aspetti. Con `--strictPort` (o `server.strictPort: true`) Vite
invece termina, con `Error: Port 5173 is already in use`.

## Risolvilo da solo

1. Trova il processo che possiede la porta e terminalo. Su Windows:

   ```text frame="terminal"
   netstat -ano | findstr :3000
   taskkill /PID 12345 /F
   ```

   Passo per passo, con la versione PowerShell, in
   [Trovare e terminare il processo che usa una porta](/it/guides/find-and-kill-process-using-port-windows/).
2. Oppure avvia il tuo server su un'altra porta, ad esempio `PORT=3001` per molti server Node o
   `--port 5174` per Vite.

## Come aiuta Moonpool

Se esegui il server tramite Moonpool, imposta `port` sulla sua voce. Moonpool allora:

- mostra l'app come In esecuzione finché qualcosa risponde su quella porta, quindi un server
  rimasto attivo che la occupa compare come in esecuzione ma non «managed by Moonpool»;
- con **Arresta** e **Riavvia**, termina qualsiasi processo ancora in ascolto su `port` quando
  `killMode` è `port`, che è il valore predefinito per le app `web`, così il successivo Avvia
  trova la porta libera;
- segnala due app configurate con la stessa `port`.

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

Moonpool non controlla la porta prima dell'avvio. Se la porta è ancora occupata, il comando
stampa l'errore visto sopra nella scheda del terminale dell'app. Premi **Arresta** (che libera
la porta) e **Avvia** di nuovo.

Per Vite, passa `--strictPort` e mantieni `port` uguale alla porta che richiedi:

```text title="command"
npm run dev -- --port 5173 --strictPort
```

Senza di esso, Vite può spostarsi su 5174 mentre Moonpool continua a controllare 5173, e il
pallino di stato non diventa mai pieno.

`killMode` `port` termina qualsiasi processo sulla porta, quindi usalo solo per porte di cui
nient'altro ha bisogno. Per le app Docker su Windows, non usarlo mai. Vedi
[Arresto e riavvio](/it/apps/stop-and-restart/#app-docker-su-windows).

## Vedi anche

- [Campi delle app](/it/apps/fields/): `port`, `killMode`.
- [Arresto e riavvio](/it/apps/stop-and-restart/)
- [Risoluzione dei problemi](/it/support/troubleshooting/#due-app-usano-la-stessa-porta)
- [Eseguire un server di sviluppo npm in background su Windows](/it/guides/run-npm-dev-server-in-background-windows/)
