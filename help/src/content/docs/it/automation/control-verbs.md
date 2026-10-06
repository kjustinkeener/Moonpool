---
title: "Canale di controllo e verbi di Moonpool: riferimento"
description: "Come funziona il canale di controllo di Moonpool (named pipe o socket Unix), il suo protocollo e ogni verbo a cui risponde l'app, con argomenti e risposte."
---

## Dove è in ascolto

Ogni copia di Moonpool ha il proprio canale, quindi il Moonpool installato e le eventuali copie
portatili possono funzionare affiancati senza rispondere l'uno per l'altro. Su Windows il
Moonpool installato è in ascolto sulla named pipe `\\.\pipe\moonpool`. Una copia portatile
aggiunge un id ricavato dalla sua cartella: `\\.\pipe\moonpool-<id>`.

`<id>` è composto da 8 cifre esadecimali derivate dal percorso della cartella `moonpool-config`
della copia, quindi resta lo stesso per quella cartella tra riavvii e aggiornamenti, e cambia
se sposti la cartella. Il `moonpool.exe` di una copia, incluso `moonpool.exe mcp`, trova sempre
il canale della propria copia.

Su Linux e macOS è invece in ascolto su un socket di dominio Unix, con modalità `0600`:

| Caso | Percorso del socket |
| --- | --- |
| Normale | `$XDG_RUNTIME_DIR/moonpool.sock` quando quella variabile è impostata, altrimenti `moonpool.sock` nella cartella di configurazione di Moonpool |
| Modalità portatile | `moonpool.sock` nella cartella di configurazione della copia portatile, così una copia portatile non entra mai in conflitto con una installata |
| Percorso troppo lungo per un socket (circa 100 caratteri) | `/tmp/moonpool-<uid>/moonpool.sock`, in una directory che solo tu puoi aprire (`moonpool-<id>.sock` per una copia portatile) |

Un file socket lasciato da un arresto anomalo viene rilevato e sostituito al successivo avvio.
Un socket su cui qualcosa risponde ancora non viene mai preso in carico. Il file viene rimosso
quando Moonpool termina normalmente.

Il canale è anche il modo in cui il [server MCP](/it/automation/mcp-setup/) sa se Moonpool è in
esecuzione: se un `ping` riceve risposta lo è, se la pipe o il socket mancano non lo è. Gli
stessi verbi sono raggiungibili anche dalla [riga di comando](/it/automation/command-line/),
tranne i verbi diagnostici descritti più avanti.

## Protocollo

Un oggetto JSON per riga in ingresso, una riga JSON in uscita, in ordine. Una connessione può
trasportare molte richieste.

```json title="request"
{"cmd": "restart", "args": ["my-app"]}
```

```jsonl title="replies"
{"ok": true, "result": "..."}
{"ok": false, "error": "unknown app id: ..."}
```

Una richiesta e la sua risposta da PowerShell:

Per una copia portatile, usa il nome della sua pipe (`moonpool-<id>`, mostrato dal verbo
`paths`) al posto di `moonpool`.

```powershell frame="terminal"
$p = New-Object System.IO.Pipes.NamedPipeClientStream('.', 'moonpool', 'InOut')
$p.Connect(2000)
$w = New-Object System.IO.StreamWriter($p); $w.AutoFlush = $true
$r = New-Object System.IO.StreamReader($p)
$w.WriteLine('{"cmd":"ping"}')
$r.ReadLine()
```

```json title="reply"
{"ok":true,"result":"pong"}
```

- `args` è un elenco di stringhe e può essere omesso. Gli altri campi vengono ignorati.
- `result` è una stringa o null. I verbi che restituiscono dati strutturati li restituiscono
  come stringa JSON.
- Una riga che non è JSON valido riceve `{"ok": false, "error": "bad request: ..."}`.
- Un `cmd` sconosciuto riceve `unknown cmd: <name>`.
- Un verbo che passa dalla finestra (`launch`, `stop`, `restart`, `reload`,
  `refresh-icons`, `help`, `open-window`) riceve risposta quando l'azione termina, oppure con un
  errore di timeout dopo 45 s. Se l'interfaccia della finestra hub non è stata caricata, fallisce
  subito con `frontend not
  loaded`.
- Un Moonpool che si avvia mentre uno precedente sta ancora terminando riprova ad associare il
  canale per circa 8 secondi. Se ancora non ci riesce, lo registra e continua a funzionare
  senza.

## Verbi

| Verbo | Argomenti | Risultato |
| --- | --- | --- |
| `ping` | nessuno | `pong`. Solo canale. |
| `list` | nessuno | Stringa JSON `{"apps": [...], "statuses": [...]}` letta dalla memoria dell'hub in esecuzione, con la stessa struttura `apps` e `statuses` di `state.json`. Aggiunge `"statusNotReady": true` quando le app sono registrate ma il primo controllo dello stato non è ancora stato eseguito. Mentre `apps.json` non si carica, aggiunge `"manifestError": "<message>"` (le app sono allora l'ultimo elenco caricato) e, quando dall'avvio non è stato caricato alcun elenco, `"manifestLoaded": false`. Solo canale. |
| `show` | nessuno | null. Porta la finestra in primo piano. |
| `quit` | nessuno | null. Chiude Moonpool. |
| `launch` | `<id>` | null in caso di successo, oppure `opened` per una voce `static` con il solo `url`. Errori: `unknown app id: <id>`, `did not reach running in time`. |
| `stop` | `<id>` | null in caso di successo, oppure `stopped` per una voce `static` con il solo `url`. Errore: `still running after stop`. |
| `restart` | `<id>` | Stessi risultati ed errori di `launch`. |
| `reload` | nessuno | null in caso di successo. |
| `refresh-icons` | nessuno | null in caso di successo. |
| `help` | nessuno | null. Apre la finestra Aiuto. |
| `dump` | `<id>` [`out-path`] | Percorso del registro di sessione dell'app, o della copia in testo semplice in `out-path`. |
| `paths` | nessuno | Resoconto su più righe delle cartelle e dell'exe usati dall'hub. |
| `read-config` | nessuno | Percorso di `dumps\read-config.json`, che contiene `token`, `valid`, `error`, `path`, `manifest_text`. |
| `write-config` | `<source-file>` [`token`] | Il nuovo token di versione. Errori: `stale token: ...`, `rejected invalid manifest: ...`, `cannot read source ...`. |
| `restore-config` | [`index` o `filename`] | Nessun argomento: percorso di `dumps\restore-config.json` (`count`, `snapshots`). Con un argomento: `restored <file> (<n> apps); new version token <token>`. |
| `argv` | gli argomenti della riga di comando | null, subito. Li esegue esattamente come farebbe un secondo `moonpool.exe <args>` di questa copia, incluso `--ticket`. È così che quel secondo avvio passa i suoi argomenti prima di terminare. |

Esempi di scambi:

```jsonl title="request, reply"
{"cmd": "launch", "args": ["nope"]}
{"ok": false, "error": "unknown app id: nope"}

{"cmd": "restore-config", "args": ["1"]}
{"ok": true, "result": "restored 1767225600000.json (6 apps); new version token <token>"}
```

`write-config` e `restore-config` caricano subito il nuovo manifest, registrano un'istantanea in
`apps.json.history\` e aggiornano la finestra.

## Verbi diagnostici (test)

Solo canale: la riga di comando non li accetta. Funzionano tutti su Windows, Linux e macOS
tranne `screenshot`, che è solo per Windows e altrove risponde `screenshot is not supported on this
platform (Windows only)`.

| Verbo | Argomenti | Risultato |
| --- | --- | --- |
| `screenshot` | [`window`] [`max_dim`] | Solo Windows. Base64 di un PNG di quella finestra di Moonpool (predefinita `main`). `max_dim` facoltativo limita il lato più lungo in pixel (compreso tra 320 e 2400, predefinito 320; lo strumento MCP usa sempre il valore predefinito). Un `max_dim` non intero è un errore. Finestre consentite: `main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`. Errori: `unknown window '<name>'`, `window '<name>' is not open`. Non viene scritto su disco. |
| `open-window` | `<kind>` [`<id>`] | null. Apre una finestra come farebbe la sua voce di menu. `kind`: `settings`, `about`, `installer`, `help`, `themes`, `editor` (un `<id>` facoltativo apre la finestra Modifica app di quell'app, senza apre Aggiungi app), `terminal` (`<id>` obbligatorio: seleziona la scheda del terminale di quell'app e allarga l'hub in modo che il pannello CLI sia visibile; non la avvia), `cli` (allarga soltanto l'hub). Errori: `unknown window kind '<kind>'`, `terminal needs an app id`, `unknown app id: <id>`. Riceve risposta tramite la finestra hub come `launch`. |
| `window-state` | [`window`] | Stringa JSON: `{"open":false}`, oppure `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. |
| `stop-mcp` | `<id>` | `stopped`. Termina l'helper `<processName> mcp` dell'app, non l'app. Errori: `missing app id`, `unknown app id: <id>`. |
| `reset-mcp-seen` | [`<id>`] | `<id>: cleared` oppure `<id>: was not marked seen`; senza id, `cleared <n> entries`. Cancella gli avvistamenti memorizzati degli helper MCP. |

L'opzione `--ticket` della riga di comando e i record di esito in `state.json` appartengono
all'altro canale; vedi [Riga di comando](/it/automation/command-line/#leggere-lesito). Le
richieste del canale ricevono la risposta nella risposta stessa.

## Vedi anche

- [Riga di comando](/it/automation/command-line/)
- [Agenti IA: avvio rapido](/it/automation/quick-start/#la-stessa-azione-in-tre-modi)
