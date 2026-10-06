---
title: "Esegui un server di sviluppo npm in background su Windows senza una finestra di terminale"
description: "Mantieni in esecuzione npm run dev, Vite o un altro server di sviluppo su Windows senza una console da sorvegliare, e gestiscilo dall'area di notifica."
---

Un server di sviluppo avviato con `npm run dev` gira nel terminale che lo ha lanciato, quindi
chiudendo quella finestra si ferma. Il modo semplice di Windows per tenerlo attivo è un processo
nascosto, ad esempio `Start-Process npm.cmd -ArgumentList "run","dev" -WindowStyle Hidden` in
PowerShell, ma poi non hai output da leggere e per arrestarlo devi cercare il `node.exe` giusto
(vedi [Trovare e terminare il processo che usa una porta](/it/guides/find-and-kill-process-using-port-windows/)).

## Il metodo Moonpool

Moonpool esegue il comando in una propria scheda del terminale integrata nella finestra hub,
quindi non c'è alcuna finestra di console separata da tenere aperta. Nascondi l'hub
nell'area di notifica e il server continua a girare. Aggiungi l'app una volta sola:

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

Fai clic sul controllo **Avvia** dell'app. Il pallino di stato diventa pieno quando `port`
risponde, e il browser si apre su `url` grazie a `openBrowser`. Fai clic sul nome dell'app per
leggerne l'output nella sua scheda. **Arresta** termina il terminale e tutto ciò che ha avviato,
e libera la porta (`killMode` `port` è il valore predefinito per `web`).

## Mantienilo in esecuzione quando chiudi la finestra

Per impostazione predefinita il pulsante di chiusura fa uscire Moonpool, e su Windows uscire
arresta ogni app che ha avviato. Attiva **Chiudi nella barra delle applicazioni** nelle
[Impostazioni](/it/using/settings/) e chiudendo la finestra questa viene solo nascosta.
L'icona nell'area di notifica (o **Mostra Moonpool**) la riporta indietro. I dettagli sono in
[Area di notifica, chiusura e riduzione a icona](/it/using/tray-and-closing/).

## Mantieni prevedibile la porta

Moonpool stabilisce lo stato In esecuzione da `port`. Vite passa alla successiva porta libera
quando la sua è occupata, e questo lascerebbe Moonpool a controllare la porta sbagliata. Passa
`--strictPort` così Vite termina invece di spostarsi, e imposta `port` di conseguenza:

```text title="command"
npm run dev -- --port 5173 --strictPort
```

Se la porta è già occupata, vedi
[Risolvere EADDRINUSE e «Port 5173 is in use»](/it/support/port-already-in-use/).

## Limiti

- Moonpool non riavvia un server che va in crash. Mostra l'app come arrestata e la scheda
  stampa `[process exited]`.
- Moonpool non si avvia da solo all'accesso a Windows. Vedi
  [Avviare uno script o un server di sviluppo automaticamente all'accesso a Windows](/it/guides/start-app-at-windows-login/).

## Vedi anche

- [Campi delle app](/it/apps/fields/): `port`, `openBrowser`, `killMode`.
- [Tipi di app](/it/apps/types/): come viene stabilito lo stato In esecuzione per `web`.
- [Arresto e riavvio](/it/apps/stop-and-restart/)
- [Esempi](/it/apps/examples/#server-di-sviluppo-web)
