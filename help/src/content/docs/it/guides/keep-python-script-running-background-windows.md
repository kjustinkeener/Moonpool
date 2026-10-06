---
title: "Mantieni in esecuzione in background uno script Python su Windows"
description: "Esegui in background su Windows uno script Python di lunga durata o una piccola app web, vedine l'output e arrestala in modo pulito, con pythonw e con Moonpool."
---

Uno script Python eseguito da una finestra della console si ferma quando chiudi quella finestra.
Le soluzioni abituali su Windows sono `pythonw.exe` (lo stesso interprete senza finestra della
console, quindi l'output non va da nessuna parte), `Start-Process pythonw -ArgumentList worker.py`
per avviarlo staccato, oppure un'attività pianificata per ciò che deve partire all'accesso o a
intervalli. Con ciascuna, quando vuoi fermarlo devi cercare il processo in Gestione attività.

## Il metodo Moonpool

Moonpool esegue il comando in una propria scheda del terminale, quindi hai l'output e un pulsante
Arresta senza una finestra della console tua. Per uno script che gira finché non lo arresti, usa
un'app `cli`. `-u` fa sì che Python scriva subito l'output, così la scheda lo mostra in tempo
reale:

```json title="apps.json"
{
  "id": "worker",
  "name": "Queue worker",
  "group": "Scripts",
  "type": "cli",
  "cwd": "C:\\code\\worker",
  "command": ".venv\\Scripts\\python.exe -u worker.py"
}
```

Avviala e fai clic sul nome dell'app per vederne l'output. Un'app `cli` risulta in esecuzione
finché il suo comando è attivo e diventa grigia quando lo script termina, con `[process exited]`
lasciato nella scheda. **Arresta** termina lo script e tutto ciò che ha avviato. Usando il
`python.exe` dell'ambiente virtuale tramite il suo percorso non serve alcuna attivazione.

Se lo script serve HTTP (Flask, FastAPI, `python -m http.server`), rendilo un'app `web` così che
lo stato In esecuzione segua la sua porta:

```json title="apps.json"
{
  "id": "docs-api",
  "name": "Docs API",
  "group": "Scripts",
  "type": "web",
  "cwd": "C:\\code\\docs-api",
  "command": ".venv\\Scripts\\python.exe -u app.py",
  "port": 8091,
  "url": "http://127.0.0.1:8091",
  "env": { "PORT": "8091" }
}
```

## Limiti

- Lascia Moonpool in esecuzione. Per impostazione predefinita, chiudendo la sua finestra si
  esce, e su Windows uscire arresta ogni app che ha avviato. Attiva **Chiudi nella barra delle
  applicazioni** per nascondere invece la finestra; vedi
  [Area di notifica, chiusura e riduzione a icona](/it/using/tray-and-closing/).
- Moonpool non riavvia uno script che va in crash e non lo avvia da solo all'accesso a Windows.
  Vedi [Avviare uno script o un server di sviluppo automaticamente all'accesso a Windows](/it/guides/start-app-at-windows-login/).
- Evita le virgolette doppie annidate in `command`: il wrapper `cmd /c` le altera.

## Vedi anche

- [Tipi di app](/it/apps/types/#cli): come vengono monitorate le app `cli` e `web`.
- [Arresto e riavvio](/it/apps/stop-and-restart/)
- [Esempi](/it/apps/examples/)
- [Registri](/it/data/logs/): dove viene conservato l'output delle sessioni.
