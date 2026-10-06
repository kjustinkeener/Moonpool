---
title: "Avvia uno script o un server di sviluppo automaticamente all'accesso a Windows"
description: "Avvia Moonpool all'accesso a Windows con un collegamento in Esecuzione automatica, poi lancia un server di sviluppo o uno script con uno script PowerShell."
---

Windows ha due modi abituali per avviare qualcosa all'accesso: un collegamento nella cartella
Esecuzione automatica (premi Win+R, digita `shell:startup`, premi Invio) oppure un'attività di
Utilità di pianificazione con un trigger «All'accesso». Entrambi eseguono un programma o uno
script, che potrebbe essere direttamente il comando del tuo server di sviluppo, ma allora nulla
lo tiene traccia, ne mostra l'output o lo arresta per te.

## Cosa offre Moonpool

Moonpool non ha un'impostazione per l'avvio all'accesso, e una voce di `apps.json` non ha alcun
campo che la avvii quando parte Moonpool (l'elenco completo è in [Campi delle app](/it/apps/fields/)
e in [settings.json](/it/data/settings-json/)). Quello che puoi fare è avviare Moonpool tu stesso
all'accesso, poi far lanciare da uno script le app che vuoi, usando lo stesso verbo offerto dalla
[riga di comando](/it/automation/command-line/).

Per prima cosa registra l'app come al solito:

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

Poi salva questo come `start-moonpool-apps.ps1`. Se è installato, il programma è
`%USERPROFILE%\.moonpool\moonpool.exe`; per una copia portatile usa il percorso dell'exe di
quella copia.

```powershell title="start-moonpool-apps.ps1"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
Start-Process $mp
Start-Sleep -Seconds 15
& $mp launch site
```

Moonpool deve essere già in esecuzione perché `launch` gli venga passato; se non c'è nulla di
residente, lo stesso comando avvia un nuovo Moonpool e il verbo non viene eseguito. L'attesa gli
dà il tempo di avviarsi, quindi aumentala su un computer lento. Aggiungi una riga
`& $mp launch <id>` per ogni app.

Infine metti un collegamento allo script nella cartella Esecuzione automatica, con questa
destinazione:

```text title="Shortcut target"
powershell.exe -NoProfile -WindowStyle Hidden -File "C:\Users\you\start-moonpool-apps.ps1"
```

Per verificare cosa è successo, aggiungi `--ticket t1` a un verbo e leggi il risultato da
`state.json` ([Leggere l'esito](/it/automation/command-line/#leggere-lesito)).

## Avvertenze

- Un server di sviluppo avviato in questo modo è «gestito» da Moonpool come qualsiasi altro,
  quindi Arresta ed Esci funzionano su di esso. Se la stessa app è già in esecuzione (avviata a
  mano, ad esempio), Moonpool la mostra come in esecuzione ma non gestita.
- Moonpool non rilancia un'app che termina e non ricorda quali app erano in esecuzione quando sei
  uscito l'ultima volta.

## Vedi anche

- [Riga di comando](/it/automation/command-line/)
- [Area di notifica, chiusura e riduzione a icona](/it/using/tray-and-closing/)
- [Eseguire un server di sviluppo npm in background su Windows](/it/guides/run-npm-dev-server-in-background-windows/)
