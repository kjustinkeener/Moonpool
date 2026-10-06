---
title: "Tieni Moonpool nell'area di notifica: comportamento di chiusura, riduzione a icona ed Esci"
description: "Controlla cosa fanno l'icona nell'area di notifica, il pulsante di chiusura, la riduzione a icona ed Esci, e tieni la finestra in primo piano senza perderla."
---

## Icona nell'area di notifica

| Azione | Risultato |
| --- | --- |
| Clic sinistro | Mostra la finestra hub (la ripristina se è ridotta a icona o nascosta). |
| Clic destro | Menu con solo **Mostra Moonpool** ed **Esci** (nella tua lingua). |

Con più copie di Moonpool in esecuzione, ognuna ha la propria icona nell'area di notifica. Il
suggerimento indica di quale copia si tratta. Vedi
[Modalità portatile](/it/data/portable-mode/#più-copie-contemporaneamente).

## Esci

**Esci** chiude Moonpool e, su Windows, arresta ogni app avviata da Moonpool, inclusi i loro
processi figli. Le app che erano già in esecuzione prima che Moonpool le vedesse (mostrate come in
esecuzione senza «managed by Moonpool») vengono lasciate stare. Su Linux e macOS, uscire non
arresta in modo affidabile le app avviate.

## Chiusura e riduzione a icona

Per impostazione predefinita il pulsante di chiusura fa uscire Moonpool (`closeToTray` è
`false`). Attiva **Chiudi nella barra delle applicazioni** nelle Impostazioni e chiudendo la
finestra questa viene invece nascosta nell'area di notifica. Moonpool continua a funzionare, e
l'icona nell'area di notifica o **Mostra Moonpool** la riporta indietro.

**Riduci a icona nella barra delle applicazioni** (`minimizeToTray`, attivo per impostazione
predefinita) nasconde la finestra nell'area di notifica quando viene ridotta a icona, ed esce
dalla barra delle applicazioni. Disattivalo per ridurre a icona nella barra delle applicazioni
come al solito.

![Impostazioni: Chiudi nella barra delle applicazioni e Riduci a icona nella barra delle applicazioni (1), e il cursore Trasparenza dello sfondo (2)](../../../../assets/screenshots/settings-tray-and-transparency.png)

1. **Chiudi nella barra delle applicazioni** e **Riduci a icona nella barra delle applicazioni**.
2. **Trasparenza dello sfondo**. Vedi [Temi, lingua e trasparenza](/it/using/themes-and-language/#trasparenza).

## Blocco di area di notifica e barra delle applicazioni

**Mostra nell'area di notifica** e **Mostra nella barra delle applicazioni** controllano se
l'icona nell'area di notifica e il pulsante nella barra delle applicazioni sono visibili. Almeno
uno deve restare attivo, altrimenti una finestra nascosta non avrebbe modo di tornare. Quando ne
è attivo uno solo, la sua casella è disabilitata finché non riattivi l'altro.

## Sempre in primo piano

**Sempre in primo piano** nelle Impostazioni mantiene ogni finestra di Moonpool (l'hub,
Impostazioni, Informazioni, l'editor delle app, il browser dei temi, l'installer e Aiuto) sopra
le altre finestre. È disattivato per impostazione predefinita.

## Vedi anche

- [Eseguire un server di sviluppo npm in background su Windows](/it/guides/run-npm-dev-server-in-background-windows/)
- [Avviare uno script o un server di sviluppo automaticamente all'accesso a Windows](/it/guides/start-app-at-windows-login/)
- [Finestra delle impostazioni](/it/using/settings/)
- [La finestra hub](/it/using/hub-window/)
