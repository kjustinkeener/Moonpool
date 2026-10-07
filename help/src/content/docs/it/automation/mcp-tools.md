---
title: "Strumenti MCP di Moonpool: riferimento di parametri e risultati"
description: "Ogni strumento esposto dal server MCP di Moonpool agli agenti, con i suoi parametri, ciò che restituisce e i casi di errore che puoi incontrare."
---

Tutti gli strumenti restituiscono testo, tranne `moonpool_screenshot`, che restituisce
un'immagine PNG. Un errore torna come risultato dello strumento contrassegnato come errore, con
il motivo in forma di testo. Per la configurazione vedi
[Configurazione MCP](/it/automation/mcp-setup/).

Gli strumenti che accettano `app_id` richiedono l'`id` dell'app in `apps.json`. Deve usare solo
lettere, cifre, `.`, `_` e `-`, e non iniziare con `-`, altrimenti la chiamata fallisce con
"invalid app_id".

La maggior parte degli strumenti che agiscono sull'hub fallisce con questo messaggio quando
l'hub non è in esecuzione. `moonpool_bootup_launcher`, `moonpool_shutdown_launcher`,
`moonpool_raise_launcher` e `moonpool_launcher_paths` gestiscono da soli quel caso (vedi le
loro righe). Per una copia portatile il messaggio nomina la copia, ad esempio
`Moonpool (<folder>)`.

```text
Moonpool is not running - call moonpool_bootup_launcher first
```

Le chiamate che attendono un esito scadono dopo 45 secondi.

## Launcher e app

Esempio di risultato di `moonpool_list_apps`:

```text
site  [running] (managed by Moonpool)  Site
notes-app  [stopped]  [mcp: stopped]  Notes App
```

| Strumento | Parametri | Comportamento |
| --- | --- | --- |
| `moonpool_list_apps` | nessuno | Una riga per app: `id  [running]` oppure `[stopped]`, `(managed by Moonpool)` quando applicabile, `[mcp: running]` o `[mcp: stopped]` quando è stato visto un helper MCP, poi il nome. Richiesto all'hub in esecuzione tramite il canale di controllo (verbo `list`), quindi è aggiornato. Se Moonpool non è in esecuzione fallisce con "Moonpool is not running" anziché mostrare un elenco obsoleto. Subito dopo l'avvio di Moonpool, prima del suo primo controllo dello stato, le app mostrano `[status pending]`. Mentre `apps.json` contiene un errore, il risultato inizia con `apps.json has an error: <message>. This list is the last one that loaded; fix the file and call moonpool_reload_config.` Se il file era già danneggiato all'avvio di Moonpool, dice che nessuna app è caricata e suggerisce anche `moonpool_restore_config`. |
| `moonpool_bootup_launcher` | nessuno | Avvia Moonpool stesso e attende fino a 30 s che il suo canale di controllo risponda. Restituisce "Moonpool started", oppure "Moonpool is already running". Se il nuovo processo termina subito (ha passato il controllo a un Moonpool che stava ancora chiudendosi), ne avvia un altro. Se qualcosa occupa il canale senza rispondere, segnala che un processo Moonpool potrebbe essersi bloccato. |
| `moonpool_shutdown_launcher` | nessuno | Come Esci nel menu dell'area di notifica. Attende fino a 30 s che il canale di controllo scompaia. Restituisce "Moonpool shut down", oppure "Moonpool is not running". |
| `moonpool_raise_launcher` | nessuno | Porta la finestra di Moonpool in primo piano. Restituisce "window shown". Se Moonpool non è in esecuzione, lo avvia e restituisce "Moonpool was not running; started it". |
| `moonpool_start_app` | `app_id` (obbligatorio) | Avvia l'app e apre la sua scheda del terminale. Restituisce "launched" quando è in esecuzione, oppure il motivo per cui non lo è (`unknown app id: <id>`, `did not reach running in time` dopo 25 s). Per una voce `static` con il solo `url`, apre la pagina e restituisce anch'essa "launched". |
| `moonpool_stop_app` | `app_id` (obbligatorio) | Arresta l'app. Restituisce "stopped", oppure un errore come `still running after stop` (dopo 15 s). |
| `moonpool_restart_app` | `app_id` (obbligatorio) | Arresta, attende che porta e processo si liberino, avvia. Restituisce "restarted". |
| `moonpool_app_output` | `app_id` (obbligatorio), `tail_lines` (intero, predefinito 200, minimo 1) | L'output del terminale dell'app per la sessione corrente di Moonpool, con i codici ANSI rimossi. Quando il registro è più lungo di `tail_lines`, il testo inizia con una riga che indica il percorso del registro completo. Fallisce con `no console output recorded for '<id>' (not launched this session)` se l'app non è stata eseguita. Se il registro esiste ma è vuoto, restituisce `(no output recorded for '<id>')`. |
| `moonpool_stop_mcp_server` | `app_id` (obbligatorio) | Termina il processo helper MCP collegato all'app e lascia l'app in esecuzione. Restituisce "stopped". Non fa nulla se l'app non ha né `processName` né `mcpProcessName`. |
| `moonpool_refresh_app_icons` | nessuno | Recupera di nuovo ogni icona delle app. Restituisce "icons refreshed". |

## Configurazione

Questi strumenti leggono e modificano `apps.json` tramite l'hub, mai il file su disco. Una
scrittura deve portare il token dell'ultima lettura, un token obsoleto viene rifiutato e il nuovo
file viene convalidato prima di scrivere qualsiasi cosa. Passare dall'hub è importante perché a
un agente in un host con sandbox può essere mostrata una copia privata della cartella di
configurazione anziché quella reale.

| Strumento | Parametri | Comportamento |
| --- | --- | --- |
| `moonpool_read_config` | nessuno | Testo JSON con `manifest_text` (l'esatto contenuto del file), `token`, `valid`, `error` (null quando valido) e `path`. `token` è `none` quando il file manca o è vuoto. |
| `moonpool_write_config` | `manifest` (obbligatorio, il testo completo del nuovo `apps.json`), `expected_token` (obbligatorio, dall'ultima lettura) | Convalida il manifest e sostituisce `apps.json`, poi lo carica. Restituisce `apps.json updated; new version token <token>`. Un token obsoleto fallisce con `stale token: apps.json changed since it was read ...`. Un manifest non valido fallisce con `rejected invalid manifest: ...`. In entrambi i casi il file resta intatto. Un `expected_token` vuoto viene rifiutato. |
| `moonpool_restore_config` | `snapshot` (facoltativo) | Senza valore, testo JSON che elenca le istantanee salvate dalla più recente (`index`, `filename`, `millis`, `app_count`, `valid`). Con un indice (1 = la più recente) o un nome di file, convalida quell'istantanea e la ripristina. Restituisce `restored <file> (<n> apps); new version token <token>`. Non serve alcun token: un ripristino sovrascrive volutamente il file corrente. |
| `moonpool_reload_config` | nessuno | Rilegge `apps.json`. Restituisce "apps.json reloaded". Se il file non viene analizzato o non supera la convalida, fallisce con `apps.json has an error: ...` e Moonpool mantiene l'ultimo elenco caricato. |
| `moonpool_launcher_paths` | nessuno | Elenca la cartella di configurazione dell'hub, `apps.json`, `state.json`, il registro, la cartella dei dump, la cartella delle icone, il flag portatile e il percorso dell'exe, poi la cartella di configurazione del processo MCP, `apps.json`, `state.json`, la cartella dei dump, il flag portatile e il percorso dell'exe (senza registro né icone). Se l'hub non è in esecuzione, la sua metà riporta `hub paths unavailable: ...` e la metà MCP viene comunque mostrata. Usalo quando una modifica non ha effetto. |

## Avanzato: strumenti di test

`moonpool_screenshot` è solo per Windows; su Linux fallisce con "screenshot is not
supported on this platform". `moonpool_window_state` e `moonpool_reset_mcp_seen` funzionano su
ogni piattaforma.

`window` è uno tra `main`, `settings`, `about`, `installer`, `editor`, `help` o `themes`, e il
valore predefinito è `main`. Un nome sconosciuto fallisce con `unknown window '<name>'`.

| Strumento | Parametri | Comportamento |
| --- | --- | --- |
| `moonpool_screenshot` | `window` (facoltativo) | Cattura il contenuto di quella finestra di Moonpool come PNG inline, di al massimo 320 pixel sul lato più lungo. La dimensione non può essere aumentata da MCP. Fallisce con `window '<name>' is not open` se non è visibile. Non può catturare nessun'altra app. |
| `moonpool_window_state` | `window` (facoltativo) | Testo JSON: `{"open":false}` quando la finestra non è aperta, altrimenti `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. Pensato per i test. |
| `moonpool_reset_mcp_seen` | `app_id` (facoltativo) | Solo per test. Cancella il record memorizzato «è stato visto un helper MCP» per un'app, o per ogni app se omesso, così la voce secondaria MCP della barra laterale si nasconde di nuovo finché non viene visto un helper. |

## Vedi anche

- [Configurazione MCP](/it/automation/mcp-setup/)
- [Riga di comando](/it/automation/command-line/)
