---
title: "Aggiungi ed esegui la tua prima app in Moonpool"
description: "Dal primo avvio a una tua app in esecuzione in pochi minuti: aggiungila, avviala, arrestala e ritrova più tardi l'hub e questa guida."
---

## 1. Avvia Moonpool

Su Windows, esegui `moonpool.exe` e fai clic su **Installa Moonpool** (vedi
[Installazione](/it/getting-started/install/)). Su Linux, avvia l'AppImage o il pacchetto
installato.

La prima volta che Moonpool viene eseguito, riempie la barra laterale con app di esempio
(Blocco note su Windows, una shell, un piccolo server web e le dashboard incluse). Funzionano
così come sono (il server web richiede Python), quindi puoi provarle e poi modificarle o
eliminarle. Inoltre inserisce un'icona nell'area di notifica. Su Windows, se non vedi l'icona,
fai clic sulla freccia **^** a destra della barra delle applicazioni.

## 2. Aggiungi la tua app

1. Apri il menu **...** in cima alla barra laterale e scegli **Aggiungi app**.
2. Inserisci un **name**. Il gruppo parte come `Web apps`; mantienilo oppure scegline un altro.
3. Lascia **type** su `web` per un server di sviluppo.
4. Imposta **cwd** sulla cartella del tuo progetto e **command** su ciò che digiti per avviarlo,
   ad esempio `npm run dev`.
5. Imposta **port** sulla porta su cui è in ascolto e **url** sulla pagina da aprire.
6. Salva.

I dettagli di ogni campo sono in [Aggiungere app](/it/apps/add-an-app/).

## 3. Avviala

Fai clic sul pulsante **Avvia** dell'app (l'icona di riproduzione sulla sua riga). Si apre la
sua scheda del terminale, che mostra l'output. Il pallino di stato pulsa mentre l'app si sta
avviando, poi diventa pieno quando la sua porta risponde. Se **openBrowser** è attivo, la pagina
si apre.

Facendo clic sul nome dell'app si apre soltanto la sua scheda del terminale. Non avvia mai
l'app.

## 4. Arrestala

Fai clic sul pulsante **Arresta** (il quadrato) sulla riga. Il pallino diventa grigio.

Se qualcosa resta in esecuzione dopo Arresta, vedi [Arresto e riavvio](/it/apps/stop-and-restart/).

## Lascia fare a un agente

Quando nessuna scheda è aperta, il pannello CLI mostra un pulsante **Copia il prompt**. Incolla
il prompt in un agente IA e lui trova le tue app e le aggiunge. Vedi
[Agenti IA: guida rapida](/it/automation/quick-start/).

## Ritrovare l'hub più tardi

- Fai clic sinistro sull'icona nell'area di notifica per mostrare l'hub. Con il clic destro
  si apre un menu con **Mostra Moonpool** ed **Esci**.
- Per impostazione predefinita, chiudendo la finestra Moonpool si chiude. Attiva
  **Chiudi nella barra delle applicazioni** nelle Impostazioni per nasconderlo nell'area di
  notifica e lasciarlo in esecuzione. Vedi
  [Area di notifica, chiusura e riduzione a icona](/it/using/tray-and-closing/).

## Ottenere aiuto

**Aiuto**, nel menu **...** in cima alla barra laterale, apre questa guida in una finestra
propria. Funziona offline e corrisponde sempre alla versione che esegui.

![Finestra della guida con la navigazione per sezioni evidenziata a sinistra e una pagina a destra](../../../../assets/screenshots/help-window.png)

## Passi successivi

- [Aggiungere app](/it/apps/add-an-app/)
- [Risoluzione dei problemi](/it/support/troubleshooting/)
