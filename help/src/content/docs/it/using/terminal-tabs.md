---
title: "Usa le schede del terminale di Moonpool: apri, chiudi, copia, riavvia"
description: "Lavora con le schede del terminale di ogni app nell'hub: apri e chiudi le schede, comprimi il pannello, copia e incolla, riavvia e trova i registri."
---

Ogni app viene eseguita in una propria scheda del terminale nel pannello CLI.

## Schede

![Barra delle schede con Metrics Dashboard attiva (evidenziata) e il suo registro in tempo reale sotto; ogni scheda ha un pallino e una x](../../../../assets/screenshots/hub-terminal-tab.png)

- Avviare un'app, o fare clic sul suo nome nella barra laterale, apre la sua scheda. Fare clic su un nome non avvia nulla; vedi [Stati delle app](/it/support/glossary/#stati-delle-app).
- Un pallino sulla scheda è acceso mentre l'app è in esecuzione.
- La **x** su una scheda chiude la scheda. Non arresta l'app. Fai di nuovo clic sul nome per riaprire la scheda; mostra il registro di questa sessione.

## Comprimere il pannello

La **x** all'estrema destra della barra delle schede («Nascondi il pannello CLI») comprime il
pannello CLI e riduce la finestra alla sola barra laterale. I terminali continuano a girare e
mantengono la loro cronologia.

Accanto alla casella del filtro compare una freccia per riportare il pannello alla larghezza
precedente. La freccia pulsa quando c'è un aggiornamento in attesa, perché il banner di
aggiornamento si trova nel pannello.

## Copia e incolla

| Azione | Risultato |
| --- | --- |
| Seleziona il testo con il mouse | Copiato negli appunti al rilascio, poi la selezione viene annullata. |
| Clic centrale | Incolla gli appunti nel terminale. |
| Pulsante **Copia tutto** (in alto a destra, compare al passaggio del cursore) | Copia l'intera cronologia come testo. |

## Cronologia

Ogni terminale conserva 10.000 righe.

## Quando un processo termina

Quando il processo termina, il terminale stampa:

```text
[process exited]
```

La scheda resta aperta con l'output intatto. La riga `[process exited]` viene mostrata nella tua
lingua (in italiano: `[processo terminato]`).

## Riavvia

**Riavvia** (o Avvia su un'app arrestata) avvia una nuova esecuzione nella stessa scheda. La
scheda viene ricostruita e l'output precedente di questa sessione viene riprodotto al suo interno
dal registro di sessione.

Se l'app è già stata eseguita in precedenza in questa sessione, Moonpool scrive prima un
separatore attenuato nel registro di sessione, così compare tra il vecchio output e la nuova
esecuzione:

```text
---------- restarted 2026-10-05 09:14:02 ----------
```

Se la nuova esecuzione inizia cancellando lo schermo, l'output precedente viene spinto nella
cronologia invece di essere cancellato.

## Registri di sessione

Tutto ciò che un'app stampa viene scritto anche in un file di registro in `cli-output\`, un file per app per sessione di Moonpool. La posizione, la conservazione e l'impostazione **Conserva i registri di output delle app tra le sessioni** sono in [Registri](/it/data/logs/).
