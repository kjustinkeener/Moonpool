---
title: "Messaggi di errore di Moonpool spiegati: already running, requires a command e altri"
description: "Cerca il testo esatto dei messaggi di errore di Moonpool, come already running, requires a command, stale token e Update failed, con significato e soluzione."
---

Incolla il messaggio che vedi nella ricerca della pagina, oppure scorri le tabelle. I messaggi
sono citati come li mostra Moonpool. Il testo tra `<parentesi angolari>` viene sostituito da un
valore (un id di app, un percorso o un errore del sistema). I sintomi che non sono un messaggio
di errore sono in [Risoluzione dei problemi e FAQ](/it/support/troubleshooting/).

## Avvio e arresto di un'app

| Messaggio | Significato e soluzione |
| --- | --- |
| `already running` | Moonpool ha già un terminale per questa app. Arrestala prima, oppure usa Riavvia. |
| `stopped during launch` | È stato premuto Arresta mentre l'avvio era ancora in corso. Avvia di nuovo. |
| `app has no launch command` | La voce non ha `command`. Aggiungine uno nell'editor delle app o in `apps.json`. Solo una voce `static` con un `url` può farne a meno. |
| `unknown app: <id>` | Nessuna app con quell'`id` è caricata. Controlla l'id, poi usa Ricarica se hai modificato `apps.json` a mano. |
| `unknown app id: <id>` | Lo stesso problema, segnalato a uno script o a un agente. Elenca le app con `moonpool_list_apps`. |
| `did not reach running in time` | Da uno script o da un agente: l'app non è risultata In esecuzione entro 25 secondi. Controlla `port` o `processName` e leggi l'output. Vedi [Il pallino di stato è sbagliato](/it/support/troubleshooting/#il-pallino-di-stato-è-sbagliato). |
| `still running after stop` | Dopo 15 secondi l'app risulta ancora In esecuzione. Imposta `killMode`. Vedi [Arresto e riavvio](/it/apps/stop-and-restart/). |
| `refusing to open non-web url: <url>` | L'`url` non è `http://`, `https://`, `mailto:` né `file://`. Correggi l'`url`. |
| `[process exited]` | Non è un errore: il comando dell'app è terminato. Mostrato nella scheda del terminale (nell'interfaccia in italiano appare come `[processo terminato]`). |

## Validazione di apps.json

Moonpool rifiuta un `apps.json` che viola una regola e mantiene l'ultimo elenco caricato.
`<n>` è la posizione della voce nel file, a partire da 1.

| Messaggio | Soluzione |
| --- | --- |
| `apps.json entry <n> has invalid id "<id>"; use letters, digits, '.', '_', and '-' without a leading '-'` | Rinomina l'`id`. |
| `duplicate app id "<id>"` | Due voci hanno lo stesso `id`. Rendi ciascuno univoco. |
| `apps.json entry <n> (<id>) has an empty name` | Compila `name`. |
| `apps.json entry <n> (<id>) has an empty group` | Compila `group`. |
| `apps.json entry <n> (<id>) has unknown type "<type>"` | `type` deve essere `web`, `desktop`, `static` o `cli`. |
| `apps.json entry <n> (<id>) has invalid port 0` | `port` deve essere da 1 a 65535. |
| `apps.json entry <n> (<id>) requires a url` | Una voce `static` ha bisogno di un `url`. |
| `apps.json entry <n> (<id>) requires a command` | Ogni altro tipo ha bisogno di un `command`. |

Nell'editor delle app, salvando senza un nome compare `name is required.` (nell'interfaccia in
italiano: «il nome è obbligatorio.»). Il testo del banner, «apps.json contiene un errore: viene
mostrato l'ultimo elenco caricato.» oppure «apps.json contiene un errore, quindi nessuna app è
caricata.» (in inglese: "apps.json has an error, showing the last list that loaded." o "apps.json
has an error, so no apps are loaded."), e come ripristinare si trovano in
[apps.json contiene un errore](/it/support/troubleshooting/#appsjson-contiene-un-errore). Se il banner
dice che il salvataggio è sospeso, il messaggio termina con `Repair apps.json and reload it before saving from
Moonpool`. L'elenco completo delle regole è in [Validazione](/it/apps/apps-json/#convalida).

## Impostazioni, aggiornamenti e installer

| Messaggio | Significato e soluzione |
| --- | --- |
| `... Repair settings.json and restart Moonpool before changing settings` | `settings.json` è malformato. Correggilo o eliminalo e riavvia. Vedi [settings.json](/it/data/settings-json/#lettura-e-riparazione). |
| `Update failed: <error>` (nell'interfaccia in italiano: `Aggiornamento non riuscito: <error>`) | Il download o l'installazione di un aggiornamento non è riuscito. Vedi [Quando un aggiornamento non riesce](/it/data/updating/#quando-un-aggiornamento-non-riesce). |
| `Update check failed: <error>` (in italiano: `Controllo aggiornamenti non riuscito: <error>`) | Il controllo degli aggiornamenti in Informazioni non è riuscito. Il testo dopo i due punti ne indica il motivo. Riprova più tardi. |
| `Install failed: <error>` (in italiano: `Installazione non riuscita: <error>`) | L'installer si è fermato al passaggio indicato dopo i due punti, ad esempio `copy exe: ...`. Esci da qualsiasi Moonpool in esecuzione da `%USERPROFILE%\.moonpool` e riprova. |
| `target folder does not exist` | La cartella scelta per una copia portatile non esiste più. Scegline una esistente. |

## MCP e script

| Messaggio | Significato e soluzione |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Avvia Moonpool, oppure lascia che l'agente chiami quello strumento. Per una copia portatile il messaggio ne indica il nome. |
| `frontend not loaded` | La finestra hub non ha ancora finito di caricarsi. Attendi e riprova. |
| `invalid app_id: use only letters, digits, '.', '_', '-' (no leading '-')` | L'agente ha passato un id che il server MCP non accetta. Usa l'id restituito da `moonpool_list_apps`. |
| `stale token: apps.json changed since it was read ...` | Rileggi `apps.json`, applica di nuovo la modifica, poi scrivi. |
| `rejected invalid manifest: ...` | Il nuovo `apps.json` non ha superato la validazione (vedi sopra). Il file non è stato modificato. |
| `no console output recorded for '<id>' (not launched this session)` | È stato richiesto `moonpool_app_output` per un'app che non è stata eseguita da quando Moonpool è partito. |

Altro in [Strumenti MCP](/it/automation/mcp-tools/) e
[Configurazione di MCP](/it/automation/mcp-setup/#se-gli-strumenti-non-funzionano).

## Errori di altri programmi

- [`Error: listen EADDRINUSE: address already in use :::3000` e `Port 5173 is in use`](/it/support/port-already-in-use/)
- [`Windows protected your PC`](/it/support/windows-protected-your-pc/)
- [WebView2 runtime missing](/it/support/webview2-runtime-missing/)
