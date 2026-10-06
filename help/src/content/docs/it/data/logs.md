---
title: "Trovare e gestire i registri di sessione di Moonpool e il registro di debug"
description: "Individua il registro di sessione di ogni app, il registro di debug di Moonpool, i dump e la cronologia di scorrimento, con durata e modo di aprirli o copiarli."
---

Moonpool conserva quattro tipi di output:

| Tipo | Dove | Conservato |
| --- | --- | --- |
| Registro di sessione | `cli-output\<id>\<session-start-ms>.log` nella cartella di configurazione | Quello di questa sessione sempre; quelli delle sessioni precedenti secondo le regole di conservazione qui sotto |
| `moonpool.log` | La cartella di configurazione | Scritto solo mentre **Registra le informazioni di debug su file** è attivo |
| Dump | Dove lo chiedi, oppure il percorso del registro di sessione stesso | Finché non lo elimini |
| Cronologia di scorrimento | Nella scheda del terminale | 10.000 righe, finché Moonpool non viene chiuso |

La cartella di configurazione è indicata in [Dove si trova la configurazione](/it/apps/apps-json/#dove-si-trova-la-configurazione).

## Registri di sessione

Tutto ciò che un'app stampa nel suo terminale viene scritto anche in un file di registro:

```text
<config folder>\cli-output\<id>\<session-start-ms>.log
```

- Un file per app per sessione di Moonpool. Il numero indica quando è stato avviato quel
  processo Moonpool.
- Arrestare e riavviare un'app continua ad aggiungere dati allo stesso file. Una riga
  divisoria attenuata segna dove inizia ogni nuova esecuzione, e lo stesso indicatore compare
  nella scheda del terminale:

  ```text title="1767225600000.log"
  Local:   http://localhost:5173/
  ---------- restarted 2026-10-05 09:14:02 ----------
  Local:   http://localhost:5173/
  ```

- I caratteri di un `id` diversi da lettere, cifre, `-` e `_` diventano `_` nel nome della
  cartella. Quindi `.` diventa `_`. Le lettere non inglesi vengono mantenute.
- Il file contiene l'output grezzo del terminale, inclusi i codici colore. Usa un dump per
  ottenere testo semplice.

Riaprire la scheda di un'app ripropone il registro di questa sessione, così vedi il suo output
precedente.

## Conservazione

La conservazione riguarda solo i registri delle sessioni precedenti di Moonpool. Viene eseguita
quando avvii un'app, solo per la cartella di quell'app, a partire dai più vecchi.

| **Conserva i registri di output delle app tra le sessioni** (`cliLogging`) | Cosa succede ai registri delle sessioni precedenti |
| --- | --- |
| disattivato (predefinito) | Eliminati al successivo avvio dell'app. |
| attivato | Conservati finché la dimensione totale della cartella supera **Conservazione dei registri per app** (`logRetentionMb`, predefinito 10 MB), poi i più vecchi vengono eliminati. |

Il file della sessione corrente rientra in quel totale, ma non viene mai eliminato né troncato.
Quindi un solo registro corrente molto grande può far uscire tutti i più vecchi.

![La sezione Registrazione delle Impostazioni: la casella per conservare i registri, la dimensione di conservazione per app in MB e la casella del registro di debug, ciascuna con una riga del percorso della cartella](../../../../assets/screenshots/settings-logging-section.png)

1. **Conserva i registri di output delle app tra le sessioni** corrisponde a `cliLogging`. **Conservazione dei registri per app**, sotto di essa, corrisponde a `logRetentionMb`.

## moonpool.log

Con **Registra le informazioni di debug su file** (`debugLogging`) attivo, Moonpool aggiunge
righe con marca temporale a `moonpool.log` nella cartella di configurazione: caricamenti di
`apps.json`, avvii (con il comando e la cartella), comandi di controllo ed errori. Attivalo
prima di riprodurre un problema.

## Aprire e copiare

In [Impostazioni](/it/using/settings/#colonna-destra-registri), sotto ogni gruppo di registri:

- **Apri la cartella dei registri CLI** e **Apri il registro** aprono la cartella nel tuo file
  manager.
- **Copia il percorso della cartella dei registri CLI** e **Copia il percorso del file di registro**
  mettono il percorso negli appunti.

In una scheda del terminale, **Copia tutto** copia come testo l'intera cronologia di scorrimento.

## Dump

Il verbo `dump` ti fornisce un registro di sessione da uno script:

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" dump my-app C:\temp\my-app.log
```

Con un percorso di output scrive una copia in testo semplice, senza i codici colore. Senza,
riporta il percorso del registro di sessione stesso. Un agente ottiene lo stesso testo, già
ripulito, da `moonpool_app_output`. Vedi [Riga di comando](/it/automation/command-line/).
