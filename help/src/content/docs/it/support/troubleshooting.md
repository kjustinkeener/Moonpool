---
title: "Risolvi i problemi di Moonpool: area di notifica, app che non partono, aggiornamenti"
description: "Risolvi i problemi comuni di Moonpool in base a ciò che vedi: icona mancante, app che non partono, pallini di stato errati, aggiornamenti falliti ed errori MCP."
---

Trova il sintomo, poi segui la soluzione. Il testo tra virgolette è ciò che mostra Moonpool. Per
cercare un messaggio esatto, vedi [Messaggi di errore spiegati](/it/support/error-messages/).

## Non vedo l'icona nell'area di notifica

- **Windows.** L'icona potrebbe trovarsi nell'area delle icone nascoste. Fai clic sulla freccia
  **^** a destra della barra delle applicazioni. Trascina l'icona sulla barra per tenerla
  visibile.
- **Linux con GNOME standard.** GNOME non mostra le icone dell'area di notifica senza
  l'estensione AppIndicator. Vedi [Linux](/it/platforms/linux/#area-di-notifica-su-gnome).
- **Impostazioni.** **Mostra nell'area di notifica** potrebbe essere disattivato. Apri l'hub
  dalla barra delle applicazioni o dal menu Start e riattivalo nelle
  [Impostazioni](/it/using/settings/).

## L'installer mostra un errore

| Messaggio | Cosa fare |
| --- | --- |
| `Install failed: <error>` (nell'interfaccia in italiano: `Installazione non riuscita: <error>`) | Il testo dopo i due punti indica il passaggio non riuscito, ad esempio `copy exe: ...`. Se un file è in uso, esci da qualsiasi Moonpool in esecuzione da `%USERPROFILE%\.moonpool` e riprova. |
| `target folder does not exist` | La cartella scelta per una copia portatile non esiste più. Scegli una cartella esistente. |
| `that folder already has a .moonpool with an exe of this name that isn't a portable Moonpool - pick an empty folder` | Scegli una cartella vuota, oppure rimuovi prima quella cartella `.moonpool`. |

## Windows protected your PC compare quando eseguo l'installer

È Windows SmartScreen, perché `moonpool.exe` non è firmato digitalmente. Fai clic su **More info**
(Ulteriori informazioni), poi su **Run anyway** (Esegui comunque). Vedi
[Windows protected your PC](/it/support/windows-protected-your-pc/).

## La finestra di Moonpool è vuota o non si apre su Windows

Potrebbe mancare il runtime Microsoft Edge WebView2. Vedi
[WebView2 runtime missing](/it/support/webview2-runtime-missing/).

## Un'app non si avvia

1. Fai clic sul nome dell'app per aprire la sua scheda del terminale e leggere l'output. Un
   agente può leggere lo stesso testo con `moonpool_app_output`.
2. Controlla `cwd`. Una cartella mancante, o un percorso relativo senza `./`, è la causa più
   comune. Vedi [Percorsi e ambiente](/it/apps/paths-and-environment/).
3. Controlla `command`. Eseguilo a mano in un terminale in `cwd`. Su Windows evita le virgolette
   doppie annidate; `cmd /c` le altera.
4. Attiva **Registra le informazioni di debug su file** nelle Impostazioni e avvia di nuovo.
   `moonpool.log` registra il comando e la cartella esatti. Vedi [Registri](/it/data/logs/).

| Messaggio | Significato |
| --- | --- |
| `already running` | Moonpool ha già un terminale per questa app. Arrestala prima, oppure usa Riavvia. |
| `stopped during launch` | È stato premuto Arresta mentre l'avvio era ancora in corso. |
| `did not reach running in time` | Da uno script o da un agente: l'app non è risultata In esecuzione entro 25 secondi. Controlla la sua `port` o il suo `processName` e il suo output. |

## Il pallino di stato è sbagliato

Moonpool stabilisce lo stato In esecuzione da `port`, poi da `processName`, poi dal fatto che il
proprio terminale sia ancora attivo. Vedi
[Come viene stabilito lo stato In esecuzione](/it/apps/types/#come-viene-stabilito-in-esecuzione).

- **Non diventa mai pieno.** La `port` di un'app `web` non risponde, oppure il `processName` di
  un'app `desktop` non corrisponde. Su Linux `processName` deve essere di 15 caratteri o meno.
- **Diventa grigio subito dopo l'avvio.** Un'app `cli` smette di risultare In esecuzione quando
  il suo comando termina. Usa una shell con `-NoExit` se vuoi che resti aperta.
- **Un'app `static` non risulta mai In esecuzione.** È normale per una voce con il solo `url`.
- **Risulta In esecuzione anche se non l'hai avviata tu.** Qualcos'altro sta usando quella porta
  o quel nome di processo. Moonpool la mostra come in esecuzione ma non «managed by Moonpool».

## Error: listen EADDRINUSE o "Port 5173 is in use"

Qualcos'altro è già in ascolto sulla porta che il tuo server vuole. Trovalo e terminalo, oppure
imposta `port` sull'app così che Arresta la liberi. Vedi
[Risolvere EADDRINUSE e «Port 5173 is in use»](/it/support/port-already-in-use/) e
[Trovare e terminare il processo che usa una porta](/it/guides/find-and-kill-process-using-port-windows/).

## Due app usano la stessa porta

In fondo al menu **...** compare una riga di avviso, ad esempio `port 3000: App A / App B`
(nell'interfaccia in italiano: `porta 3000: App A e App B`). Cambia la `port` di una delle due
app (e il suo `env`, se legge `PORT`). Vedi
[Avviso di conflitto di porte](/it/using/hub-window/#avviso-di-conflitto-di-porte).

## L'app continua a funzionare dopo Arresta

Da uno script o da un agente l'errore è `still running after stop` (dopo 15 secondi).

- L'app sopravvive al proprio terminale. Imposta `killMode` su `port` o `processName`. Vedi
  [Arresto e riavvio](/it/apps/stop-and-restart/).
- Un'app Docker su Windows: usa `killMode` `command` con uno `stopCommand` come
  `docker compose stop app`. Mai `port`.

## apps.json contiene un errore

La barra laterale mostra un banner, «apps.json contiene un errore: viene mostrato l'ultimo
elenco caricato» oppure, all'avvio, «apps.json contiene un errore, quindi nessuna app è
caricata.» Il salvataggio da Moonpool resta sospeso finché il file non si carica di nuovo.

Errori tipici:

```text
apps.json entry 2 (site) requires a command
apps.json entry 3 has invalid id "my app"; use letters, digits, '.', '_', and '-' without a leading '-'
duplicate app id "site"
apps.json entry 4 (api) has invalid port 0
```

1. Scegli **Modifica apps.json** nel banner, correggi la voce, salva, poi **Ricarica** (F5).
2. Oppure torna a una copia valida recente. Vedi
   [Backup e ripristino](/it/data/backup-and-recovery/#ripristinare-appsjson).

L'elenco completo delle regole è in [Validazione](/it/apps/apps-json/#convalida).

Se non puoi cambiare un'impostazione e il messaggio termina con `Repair settings.json and restart
Moonpool before changing settings`, correggi o elimina `settings.json` nella cartella di
configurazione e avvia di nuovo Moonpool. Eliminarlo riporta ogni impostazione al valore
predefinito.

## La mia modifica non ha effetto

- Le modifiche a mano richiedono **Ricarica** (o F5). Moonpool non controlla il file.
- Ricarica non riavvia le app in esecuzione. Riavvia l'app per usare un `command`, un `cwd` o un
  `env` modificato.
- Un agente potrebbe star modificando un altro `apps.json`. Chiedigli di chiamare
  `moonpool_launcher_paths` e di confrontare la cartella dell'hub con la propria. Con più copie
  di Moonpool, controlla quale copia stai modificando.

## Le app di esempio sono sparite

Gli esempi vengono scritti solo quando non esiste alcun `apps.json`. Per riaverli, vedi
[Tornare agli esempi](/it/data/backup-and-recovery/#tornare-agli-esempi), oppure copia le voci
da [Dashboard di esempio](/it/getting-started/example-dashboards/#le-app-di-esempio-compaiono-solo-al-primo-avvio).

## Un aggiornamento non è riuscito

Il banner mostra `Update failed: <error>` (nell'interfaccia in italiano: `Aggiornamento non
riuscito: <error>`). Vedi
[Quando un aggiornamento non riesce](/it/data/updating/#quando-un-aggiornamento-non-riesce).

## Un link web non si apre

`refusing to open non-web url: <url>` significa che l'`url` non è `http://`, `https://`,
`mailto:` né `file://`. Correggi l'`url`.

## Errori di MCP e degli script

| Messaggio | Cosa fare |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Avvia Moonpool, oppure lascia che l'agente chiami `moonpool_bootup_launcher`. |
| `frontend not loaded` | La finestra hub non ha ancora finito di caricarsi. Attendi un momento e riprova. |
| `stale token: ...` | `apps.json` è cambiato da quando l'agente lo ha letto. Rileggilo, poi scrivi. |
| `rejected invalid manifest: ...` | Il nuovo `apps.json` non ha superato la validazione. Il file non è stato modificato. |
| `... A Moonpool process may be hung ...` | Qualcosa tiene il canale di controllo senza rispondere. Esci da Moonpool dall'area di notifica, oppure termina il processo, e avvialo di nuovo. |

Altro in [Configurazione di MCP](/it/automation/mcp-setup/#note) e
[Strumenti MCP](/it/automation/mcp-tools/).

## Problemi con la finestra

- **Fuori dallo schermo.** Moonpool ignora una posizione salvata che non appartiene a nessun
  display collegato. Se la finestra è ancora introvabile, esci da Moonpool ed elimina
  `window-state.json` nella cartella di configurazione.
- **Zoom bloccato troppo grande o troppo piccolo.** Lo cambia Ctrl + rotellina sull'hub. Vedi
  [Scorciatoie e zoom](/it/using/keyboard-shortcuts/#zoom).
- **Impostazioni si apre dietro l'hub.** Disattiva, o attiva, **Sempre in primo piano** nelle
  Impostazioni. Si applica a ogni finestra di Moonpool, così restano sullo stesso livello.

## Dove sono i registri?

Vedi [Registri](/it/data/logs/).

## Backup, ripristino o disinstallazione

Vedi [Backup e ripristino](/it/data/backup-and-recovery/) e
[Disinstallazione](/it/getting-started/install/#disinstallazione).

## FAQ

**Chiudere la finestra arresta le mie app?**
Per impostazione predefinita chiudere fa uscire Moonpool, e su Windows uscire arresta le app che
ha avviato. Attiva **Chiudi nella barra delle applicazioni** per lasciare Moonpool in esecuzione
quando chiudi la finestra. Vedi
[Area di notifica, chiusura e riduzione a icona](/it/using/tray-and-closing/).

**Posso eseguire Moonpool due volte?**
Uno per cartella. Avviando di nuovo la stessa copia ne riporti in primo piano la finestra. La
copia installata e le copie portatili possono essere eseguite fianco a fianco. Vedi
[Modalità portatile](/it/data/portable-mode/#più-copie-contemporaneamente).

**Moonpool contatta server esterni?**
Solo per controllare gli aggiornamenti: scarica il file di rilascio (`update.json`) da GitHub
all'avvio (se **Controlla aggiornamenti all'avvio** è attivo) e quando premi **Controlla
aggiornamenti**. Ogni download viene verificato con la chiave di firma di Moonpool prima di
essere usato.

**Quale shell esegue i miei comandi?**
`cmd /c` su Windows, `$SHELL -c` su Linux.

**Dove metto i segreti?**
I valori di `env` sono salvati in chiaro in `apps.json`. Preferisci un file che la tua app legge
da sé, oppure una variabile già impostata nel tuo ambiente utente, che le app avviate ereditano.

**Il canale di controllo è protetto?**
Non ha login né token. Qualsiasi processo eseguito come te può inviargli comandi. Su Linux il socket è leggibile solo dal tuo utente. Vedi
[Proprietà di sicurezza](/it/automation/overview/#proprietà-di-sicurezza).
