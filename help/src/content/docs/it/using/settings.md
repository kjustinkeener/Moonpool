---
title: "Cambia le impostazioni di Moonpool: ogni opzione di Impostazioni e Informazioni"
description: "L'elenco completo dei controlli nelle finestre Impostazioni e Informazioni di Moonpool, la chiave di settings.json di ciascuno e come ripristinarli."
---

Apri **Impostazioni** dal menu «...» dell'hub. Le modifiche vengono salvate man mano. Esc chiude
la finestra. Questa pagina è l'elenco completo delle impostazioni. Ognuna è salvata in
`settings.json` sotto la chiave indicata; il file stesso è descritto in
[settings.json](/it/data/settings-json/).

![Finestra Impostazioni: interruttori e cursori nella colonna sinistra, opzioni dei registri in quella destra](../../../../assets/screenshots/settings-window.png)

## Ripristinare un controllo

Fai clic destro su qualsiasi casella, cursore o campo numerico per riportare solo quella
impostazione al valore predefinito. Il suggerimento che compare passando il cursore su ogni
controllo lo dice. I selettori Lingua e Tema non hanno un ripristino.

## Colonna sinistra

| Controllo | Chiave | Predefinito | Cosa fa |
| --- | --- | --- | --- |
| Lingua | `locale` | Automatico (sistema) | Lingua del testo di Moonpool. Si applica subito. Vedi [Temi, lingua e trasparenza](/it/using/themes-and-language/). |
| Tema | nessuna (archivio del browser) | Automatico (sistema) | Tema di colori. Il pulsante apre un browser dei temi con l'anteprima di ogni tema; facendo clic su uno lo si applica subito. Vedi [Temi, lingua e trasparenza](/it/using/themes-and-language/). |
| Chiudi nella barra delle applicazioni | `closeToTray` | disattivato | Attivo: chiudendo la finestra Moonpool si nasconde nell'area di notifica. Disattivato: chiudendo si esce. |
| Riduci a icona nella barra delle applicazioni | `minimizeToTray` | attivato | Attivo: riducendo a icona Moonpool si nasconde nell'area di notifica ed esce dalla barra delle applicazioni. Disattivato: si riduce alla barra delle applicazioni. |
| Sempre in primo piano | `alwaysOnTop` | disattivato | Mantiene ogni finestra di Moonpool sopra le altre finestre. |
| Mostra nell'area di notifica | `showInTray` | attivato | Mantiene visibile l'icona nell'area di notifica. |
| Mostra nella barra delle applicazioni | `showInTaskbar` | attivato | Mantiene visibile il pulsante nella barra delle applicazioni. |
| Mostra la barra di stato CPU/memoria | `showStatusbar` | attivato | Barra in tempo reale di CPU e memoria in fondo all'hub. |
| Mostra i processi MCP | `showMcpProcesses` | attivato | Mostra il processo MCP di un'app come sottoriga MCP nella barra laterale mentre i suoi strumenti MCP sono in uso. |
| Trasparenza dello sfondo | `transparency` | 0% | Cursore da 0 a 90 a passi di 5. Vedi [Temi, lingua e trasparenza](/it/using/themes-and-language/#trasparenza). |
| Controlla aggiornamenti all'avvio | `checkOnStartup` | attivato | Controlla su GitHub la presenza di una versione più recente all'avvio e mostra un banner se ne trova una. Vedi [Aggiornamenti](/it/data/updating/). |

### Blocco di area di notifica e barra delle applicazioni

Almeno una tra **Mostra nell'area di notifica** e **Mostra nella barra delle applicazioni** deve
restare attiva, altrimenti una finestra nascosta non avrebbe modo di tornare. Quando ne è attiva
una sola, la sua casella è disabilitata finché non riattivi l'altra.

## Colonna destra: registri

| Controllo | Chiave | Predefinito | Cosa fa |
| --- | --- | --- | --- |
| Conserva i registri di output delle app tra le sessioni | `cliLogging` | disattivato | L'output del terminale della sessione in corso viene sempre conservato per le sue schede. Attivo: i registri delle sessioni precedenti restano su disco in `cli-output\`, entro il limite dell'impostazione di conservazione. Disattivato: vengono eliminati al successivo avvio di quell'app. |
| Conservazione dei registri per app | `logRetentionMb` | 10 MB | Limite ai registri complessivi di ogni app. Minimo 1. Disabilitato mentre l'interruttore sopra è disattivato. Il registro della sessione corrente conta ai fini del limite ma non viene mai troncato né eliminato da esso. |
| Registra le informazioni di debug su file | `debugLogging` | disattivato | Registra i caricamenti di `apps.json`, gli avvii e gli errori in `moonpool.log`. |

Sotto ogni gruppo di registri, un campo percorso mostra la posizione, con due pulsanti:

- Il pulsante di apertura apre la cartella nel gestore di file (**Apri la cartella dei registri CLI** per `cli-output\`, **Apri il registro** per `moonpool.log`).
- Il pulsante di copia mette il percorso negli appunti (**Copia il percorso della cartella dei registri CLI**, **Copia il percorso del file di registro**).

I formati dei file di registro, il separatore di riavvio e le regole di conservazione sono in
[Registri](/it/data/logs/).

Se una casella non può essere salvata, un messaggio rosso in cima alla finestra lo segnala e la
casella torna al valore precedente.

## Finestra Informazioni

Apri **Informazioni** dal menu «...».

![Finestra Informazioni con la riga della versione, i link e i pulsanti Controlla aggiornamenti e Chiudi](../../../../assets/screenshots/about-window.png)

Mostra:

- La versione e la data di compilazione.
- I link al sito del progetto, al repository GitHub e all'indirizzo di contatto.
- **Controlla aggiornamenti**. Se esiste una versione più recente, la scarica, la verifica e la installa, poi riavvia Moonpool. Altrimenti segnala che hai già l'ultima versione, oppure l'errore se il controllo non è riuscito.
- I riconoscimenti per le librerie con cui è realizzato Moonpool e l'autore.

Esc la chiude. Informazioni segue in tempo reale le impostazioni di tema, trasparenza e lingua.
