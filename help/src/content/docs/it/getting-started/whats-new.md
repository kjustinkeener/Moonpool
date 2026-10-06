---
title: "Note di rilascio di Moonpool e ultime novità"
description: "Scopri cosa è cambiato nelle ultime versioni di Moonpool, quali sono i requisiti per eseguirlo e dove trovare le note di rilascio complete su GitHub."
---

Le note complete di ogni versione sono nella
[pagina Releases](https://github.com/kjustinkeener/Moonpool/releases) del progetto. Questa guida
è inclusa in Moonpool, quindi descrive sempre la versione che esegui. Moonpool si aggiorna da
solo; vedi [Aggiornamenti](/it/data/updating/).

## 0.3.16

- **Più Moonpool contemporaneamente.** Il Moonpool installato e un numero qualsiasi di copie
  portatili possono essere eseguiti fianco a fianco, uno per cartella, ciascuno con le proprie
  app, la propria icona nell'area di notifica e il proprio canale di controllo. Vedi
  [Modalità portatile](/it/data/portable-mode/#più-copie-contemporaneamente).
- **Browser dei temi.** 68 temi, ciascuno con un'anteprima nei propri colori. Vedi
  [Temi, lingua e trasparenza](/it/using/themes-and-language/).
- **Esempi eseguibili.** Un nuovo `apps.json` contiene app di esempio che funzionano tutte così
  come sono. Le dashboard di esempio ora si trovano in una cartella `dashboards/examples` di
  proprietà dell'app, che si aggiorna con Moonpool. Vedi
  [Dashboard di esempio](/it/getting-started/example-dashboards/).
- **Gli errori di apps.json vengono mostrati.** Un banner sopra la barra laterale mostra
  l'errore, e un Ricarica non riuscito mantiene l'ultimo elenco caricato. Vedi
  [Quando apps.json contiene un errore](/it/using/hub-window/#quando-appsjson-contiene-un-errore).
- **Canale di controllo su Linux e macOS**, tramite un socket Unix, più il verbo `list`. Vedi
  [Verbi di controllo](/it/automation/control-verbs/).
- La finestra Informazioni e l'editor delle app seguono in tempo reale i cambi di tema e di
  lingua. La voce di menu **Installa Moonpool...** è nascosta fuori da Windows.

## 0.3.15

- Un'app riavviata conserva l'output precedente, con un separatore «restarted» datato. Vedi
  [Schede del terminale](/it/using/terminal-tabs/#riavvia).
- Ogni app ha una propria cartella `cli-output`, quindi la pulizia dei registri non tocca mai i
  registri di un'altra app.
- `killMode` e `stopCommand` sono nell'editor delle app. Vedi
  [Arresto e riavvio](/it/apps/stop-and-restart/).
- Le app avviate non ereditano più il profilo WebView2 di Moonpool.

## 0.3.14

- I registri di sessione possono essere conservati tra una sessione e l'altra, con un limite di
  dimensione per app. Vedi [Registri](/it/data/logs/).
- Pulsanti per aprire e copiare le cartelle dei registri nelle Impostazioni.
- Correzioni alla barra del titolo della finestra della guida.

## Requisiti

- Windows 10 o 11 con WebView2 (vedi [Windows](/it/platforms/windows/)).
- Linux con WebKitGTK 4.1 e una libreria AppIndicator (vedi [Linux](/it/platforms/linux/)).
- macOS: compila dal sorgente; non ancora distribuito né testato.
