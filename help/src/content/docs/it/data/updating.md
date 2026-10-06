---
title: "Aggiornare Moonpool e risolvere un aggiornamento non riuscito"
description: "Scopri come Moonpool controlla, scarica e applica gli aggiornamenti, cosa fa l'avviso, come si aggiornano le copie portatili e Linux, e cosa fare se fallisce."
---

Moonpool si aggiorna da solo. Non c'è un programma di installazione separato da scaricare né una
procedura guidata da completare.

## Come arrivano gli aggiornamenti

Moonpool scarica `update.json` (`linux-update.json` su Linux) dalle Release GitHub del
progetto, confronta le versioni e propone solo una versione strettamente più recente. Esegue il
controllo:

- all'avvio, a meno che **Controlla aggiornamenti all'avvio** sia disattivato in
  [Impostazioni](/it/using/settings/);
- ogni volta che premi **Controlla aggiornamenti** nella finestra Informazioni. Quel pulsante
  installa subito una versione più recente e riavvia Moonpool. Altrimenti dice che hai già
  l'ultima versione, oppure mostra l'errore.

La finestra Informazioni mostra la versione in uso, sotto il nome:

![La parte superiore della finestra Informazioni: il logo, il nome (1) e la riga della versione sotto di esso](../../../../assets/screenshots/about-header.png)

1. Il nome. La riga sotto di esso è la versione e la data della build.

Ogni download viene verificato con la chiave di firma minisign di Moonpool prima di essere
applicato, quindi un download manomesso o danneggiato viene rifiutato. Moonpool non installa mai
una versione più vecchia.

## L'avviso di aggiornamento

All'avvio, un aggiornamento trovato compare come avviso nella schermata vuota dell'hub:

```text
Moonpool {version} is available (you have {current}).
```

(Nell'app italiana il testo è «Moonpool {version} è disponibile (hai la {current}).»)

L'avviso compare solo quando nessuna scheda di app è aperta e il pannello CLI è espanso. Con il
pannello compresso, pulsa invece la freccia accanto alla casella del filtro. Con una scheda
aperta non c'è alcun segnale. Per vedere l'avviso, chiudi tutte le schede (ed espandi il
pannello), oppure usa **Controlla aggiornamenti** nella finestra Informazioni.

Fai clic su **Scarica e installa** e Moonpool sostituisce se stesso e si riavvia, oppure ignora
l'avviso con la x.

## Copie portatili

Una copia portatile aggiorna il `moonpool.exe` nella propria cartella `.moonpool\`, allo stesso
modo. Ogni copia controlla e si aggiorna per conto proprio. La cartella deve essere scrivibile,
quindi una copia su una chiavetta o una condivisione di sola lettura non può aggiornarsi da
sola; copia a mano un `moonpool.exe` più recente sopra quello esistente.

## Linux

Solo l'AppImage si aggiorna da sola. Sostituisce il file AppImage sul posto, quindi tienilo in
una cartella in cui puoi scrivere. Un'installazione `.deb` o RPM viene aggiornata dal tuo gestore
di pacchetti: l'installazione da Moonpool fallisce con

```text
automatic updates are available for the AppImage only; update the .deb or RPM with your package manager
```

Vedi [Linux](/it/platforms/linux/#aggiornamenti).

## Quando un aggiornamento non riesce

L'avviso mostra il motivo e il pulsante torna disponibile così puoi riprovare:

```text
Update failed: <error>
```

(Nell'app italiana: «Aggiornamento non riuscito: <error>».)

| L'errore contiene | Causa probabile | Cosa fare |
| --- | --- | --- |
| `download failed` | Nessuna connessione, un proxy, o GitHub che limita le richieste | Attendi e riprova, oppure aggiorna a mano. |
| `signature verification FAILED - refusing to install` | Il download è danneggiato o è stato modificato | Riprova. Se continua a fallire, aggiorna a mano dalla pagina delle Release. |
| `rename self aside` o `write new exe` | La cartella è di sola lettura, oppure un antivirus tiene occupato il file | Rendi scrivibile la cartella, oppure consenti `moonpool.exe` nel tuo antivirus, poi riprova. |
| `refusing to install ... not newer than current` | La versione proposta non è più recente | Non c'è nulla da fare. |

### Aggiornare a mano

Chiudi Moonpool, scarica `moonpool.exe` dalla
[pagina delle Release](https://github.com/kjustinkeener/Moonpool/releases) del progetto e
copialo sopra quello vecchio: `%USERPROFILE%\.moonpool\moonpool.exe` nella copia installata,
oppure quello nella tua cartella `.moonpool\` per una copia portatile. La tua cartella di
configurazione non viene toccata. Su Linux, sostituisci l'AppImage, oppure usa il gestore di
pacchetti.

## Anche la guida si aggiorna

Questa guida è inclusa in Moonpool, quindi ogni aggiornamento del programma porta con sé la
guida corrispondente. La copia offline corrisponde sempre alla versione in uso.

## Vedi anche

- [Novità](/it/getting-started/whats-new/)
- [Finestra Impostazioni](/it/using/settings/)
