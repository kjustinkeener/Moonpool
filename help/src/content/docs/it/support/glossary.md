---
title: "Glossario di Moonpool: app, stati, file e impostazioni"
description: "Definizioni semplici dei termini che la guida di Moonpool usa per le sue parti, gli stati delle app, i file e le impostazioni, per seguire la documentazione."
---

## App

| Termine | Significato |
| --- | --- |
| app | Una cosa gestita da Moonpool: un server di sviluppo, un'app desktop, una pagina o un comando. |
| voce (entry) | Il record di un'app in `apps.json`. Usato solo quando si parla del JSON. |
| riga dell'app | La riga di un'app nella barra laterale, con il suo pallino di stato e i suoi controlli. |
| gruppo | L'intestazione della barra laterale sotto cui è elencata un'app, dal suo campo `group`. |
| tipo (type) | `web`, `desktop`, `static` o `cli`. Decide quali campi contano. Vedi [Tipi di app](/it/apps/types/). |
| id | La chiave permanente di un'app, usata nei nomi dei file, nei comandi e negli strumenti degli agenti. Vedi [L'id](/it/apps/apps-json/#lid). |

## Stati delle app

| Stato | Significato |
| --- | --- |
| avvio in corso (starting) | Moonpool ha avviato l'app ma non l'ha ancora vista attiva. Pallino pulsante. |
| In esecuzione (Running) | La sua `port` risponde, esiste il suo `processName` oppure, se non è impostato nessuno dei due, il terminale avviato da Moonpool è ancora attivo. Pallino pieno. Vedi [Come viene stabilito lo stato In esecuzione](/it/apps/types/#come-viene-stabilito-in-esecuzione). |
| arrestata (stopped) | Nessuna delle condizioni precedenti. Pallino grigio. |
| gestita (managed) | Moonpool l'ha avviata in questa sessione. Un'app in esecuzione che non è gestita è stata avviata in un altro modo, ed Esci non la tocca. |

Una scheda del terminale e un'app in esecuzione sono due cose distinte. Facendo clic sul nome di
un'app si apre soltanto la sua scheda del terminale; non avvia mai l'app. Chiudere una scheda non
arresta mai l'app.

## Finestre e componenti

| Termine | Significato |
| --- | --- |
| hub | Il processo Moonpool residente e la sua finestra principale. I nomi degli strumenti lo chiamano «launcher». |
| finestra hub | La finestra principale: barra laterale a sinistra, pannello CLI a destra. |
| area di notifica (tray) | L'icona nell'area di notifica di sistema e il suo menu (**Mostra Moonpool**, **Esci**). |
| barra laterale | La parte sinistra della finestra hub: casella del filtro, menu **...** e righe delle app. |
| pannello CLI | La parte destra della finestra hub, che contiene le schede del terminale. |
| scheda del terminale | Il terminale di un'app nel pannello CLI. |
| sottoriga MCP | Una riga attenuata sotto un'app che mostra il suo processo ausiliario `<exe> mcp`. |
| editor delle app | La finestra di dialogo Aggiungi app e Modifica app. |

## File e cartelle

| Termine | Significato |
| --- | --- |
| cartella di configurazione | La cartella che contiene `apps.json` e gli altri file di Moonpool. Il token `{MP_DATA}`. Vedi [Dove si trova la configurazione](/it/apps/apps-json/#dove-si-trova-la-configurazione). |
| `{MP_HOME}` | La cartella di Moonpool: `%USERPROFILE%\.moonpool` se installato, la cartella `.moonpool\` di una copia portatile, la cartella di configurazione su Linux. |
| sessione | Un'esecuzione dell'hub, dall'avvio a Esci. |
| registro di sessione | Il file che contiene tutto ciò che un'app ha stampato durante una sessione, in `cli-output\`. Vedi [Registri](/it/data/logs/). |
| `moonpool.log` | Il registro di debug di Moonpool, scritto solo con **Registra le informazioni di debug su file** attivo. |
| dump | Una copia in testo semplice di un registro di sessione creata dal verbo `dump`. |
| snapshot | Una copia di un `apps.json` valido in `apps.json.history\`. Vedi [Backup e ripristino](/it/data/backup-and-recovery/). |

## Modalità

| Termine | Significato |
| --- | --- |
| installata | Un Moonpool in `%USERPROFILE%\.moonpool`, con collegamento nel menu Start e voce in Installazione applicazioni. Solo Windows. |
| portatile | Un Moonpool in una cartella `.moonpool\` da te scelta, contrassegnata da un file `moonpool.portable`. Vedi [Modalità portatile](/it/data/portable-mode/). |
| copia | Una cartella di Moonpool, installata o portatile. Ogni copia funziona per conto suo. |

## Arresto e automazione

| Termine | Significato |
| --- | --- |
| `killMode` | Il passaggio aggiuntivo che Arresta esegue dopo aver terminato il terminale dell'app. Vedi [Arresto e riavvio](/it/apps/stop-and-restart/). |
| `stopCommand` | Il comando che Arresta esegue quando `killMode` è `command`. |
| `processName` | Il nome del processo che Moonpool controlla e che termina in modalità `processName`. |
| canale di controllo | La named pipe (Windows) o il socket Unix (Linux, macOS) su cui risponde l'hub. Vedi [Verbi di controllo](/it/automation/control-verbs/). |
| verbo (verb) | Una parola di comando come `launch` o `reload`, data sulla riga di comando o sul canale di controllo. |
| ticket | Una chiave che alleghi con `--ticket` per leggere l'esito di un comando da `state.json`. |
| token | Il contrassegno di versione di `apps.json` che una scrittura di configurazione deve riportare. |
| helper MCP (shim) | Un processo `<exe> mcp` che un host IA avvia per raggiungere gli strumenti propri di un'app. |
