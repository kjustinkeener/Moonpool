---
title: "Controllare Moonpool dalla riga di comando"
description: "Pilota un Moonpool in esecuzione con i verbi di moonpool.exe da terminale o script, contrassegna un comando con un ticket e leggi l'esito in state.json."
---

Eseguire di nuovo `moonpool.exe` mentre lo stesso Moonpool è già in esecuzione non apre una
seconda finestra. Il secondo processo passa i suoi argomenti a quello in esecuzione tramite il
[canale di controllo](/it/automation/control-verbs/) e termina. Moonpool deve essere già in
esecuzione: se non c'è nulla di residente, lo stesso comando avvia un nuovo Moonpool e il verbo
non viene eseguito.

«Lo stesso Moonpool» significa la stessa cartella. Il Moonpool installato e ogni copia portatile
funzionano ciascuno per conto proprio, quindi un comando raggiunge la copia di cui hai eseguito
il `moonpool.exe`, mai un'altra. Vedi [Modalità portatile](/it/data/portable-mode/#più-copie-contemporaneamente).

Usa il percorso della copia che intendi. Per quella installata:

```powershell frame="terminal"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
```

Con più copie in esecuzione, `Get-Process moonpool` le elenca tutte, quindi scegli in base a
`Path` anziché prendere la prima. Elenca anche gli helper `moonpool.exe mcp` inattivi avviati
dagli host MCP, quindi un processo `moonpool` non dimostra che un hub sia in esecuzione.
Interroga invece il canale di controllo con `ping` ([Verbi di controllo](/it/automation/control-verbs/)).

## Verbi

Il verbo non distingue tra maiuscole e minuscole. `<id>` è l'`id` di un'app in `apps.json`.

| Comando | Effetto |
| --- | --- |
| `moonpool.exe` | Senza verbo: porta la finestra in primo piano. |
| `moonpool.exe show` | Porta la finestra in primo piano. |
| `moonpool.exe launch <id>` | Avvia l'app e apre la sua scheda del terminale. |
| `moonpool.exe stop <id>` | Arresta l'app. |
| `moonpool.exe restart <id>` | Arresta, attende che porta e processo si liberino, avvia. |
| `moonpool.exe reload` | Rilegge `apps.json`. |
| `moonpool.exe refresh-icons` | Recupera di nuovo ogni icona. |
| `moonpool.exe help` | Apre la finestra Aiuto. |
| `moonpool.exe quit` | Chiude Moonpool, come la voce «Esci» del menu nell'area di notifica. |
| `moonpool.exe dump <id> [out-path]` | Senza `out-path`, riporta il percorso del registro dell'app per questa sessione. Con `out-path`, copia lì il registro come testo semplice con i codici ANSI rimossi. |
| `moonpool.exe paths` | Riporta la cartella di configurazione, `apps.json`, `state.json`, il registro, la cartella dei dump, la cartella delle icone, il flag portatile e il percorso dell'exe usati dal Moonpool in esecuzione. |
| `moonpool.exe read-config` | Scrive `dumps\read-config.json` nella cartella di configurazione, con `token`, `valid`, `error`, `path` e `manifest_text` (l'esatto contenuto di `apps.json`). |
| `moonpool.exe write-config <file> [token]` | Sostituisce `apps.json` con il manifest in `<file>`, se il manifest è valido e, quando viene indicato `token`, `apps.json` corrisponde ancora ad esso. |
| `moonpool.exe restore-config [index or filename]` | Senza argomento, scrive l'elenco delle istantanee in `dumps\restore-config.json`. Con un argomento, ripristina quell'istantanea se è valida. |

```powershell frame="terminal"
& $mp restart my-app
```

```powershell frame="terminal"
& $mp dump my-app C:\temp\my-app.log
```

Un verbo sconosciuto viene ignorato. Il programma ha anche argomenti di avvio propri:
`moonpool.exe mcp` ([Configurazione MCP](/it/automation/mcp-setup/)), `--uninstall` (usato da
Add/Remove Programs) e `--wait-pid <pid>` (usato quando Moonpool si rilancia da solo). Vengono
accettati solo come primo argomento, quindi un id di app come `--uninstall` non può attivarli.

## Leggere l'esito

La riga di comando non stampa nulla, quindi contrassegna un comando con `--ticket <key>` (una
chiave univoca qualsiasi, in qualsiasi posizione) e leggi il risultato da `state.json` nella
cartella di configurazione. Si tratta di `%USERPROFILE%\.moonpool\moonpool-config\` per la copia
installata, `<your .moonpool folder>\moonpool-config\` per una copia portatile e
`~/.config/Moonpool/` su Linux (vedi [Panoramica della configurazione](/it/apps/apps-json/#dove-si-trova-la-configurazione)).
`show` e `quit` non scrivono alcun ticket.

```powershell frame="terminal"
& $mp launch my-app --ticket t1
```

`state.json` contiene `apps`, `statuses` (`id`, `running`, `managed`, `mcpRunning`, `mcpSeen`
per app) e `tickets`. Il Moonpool in esecuzione lo riscrive ogni paio di secondi e dopo ogni
comando, e non lo elimina quando termina, quindi un file rimasto non significa che Moonpool sia
in esecuzione. Per sapere se lo è, o per ottenere l'elenco aggiornato delle app, usa i verbi
`ping` e `list` del canale di controllo ([Verbi di controllo](/it/automation/control-verbs/)) o
gli strumenti MCP. Interroga il tuo ticket finché `status` non è più `pending`:

| `status` | Significato |
| --- | --- |
| `pending` | Ricevuto; Moonpool sta ancora eseguendo l'azione. |
| `ok` | Fatto. Per `dump`, `read-config`, `write-config`, `restore-config` e `paths`, `detail` contiene il percorso, il token o il resoconto. |
| `error` | Non riuscito; `detail` ne indica il motivo, ad esempio `unknown app id: x`, `did not reach running in time`, `unknown command`. |

Ogni ticket è `{ ticket, action, arg, status, detail, ts }` con `ts` in millisecondi Unix:

```json title="state.json (tickets entry)"
{
  "ticket": "t1",
  "action": "launch",
  "arg": "my-app",
  "status": "error",
  "detail": "did not reach running in time",
  "ts": 1767225600000
}
```

I ticket completati vengono eliminati dopo 24 ore, e l'elenco viene ridotto verso le 50 voci
quando i ticket completati hanno almeno 5 minuti.

Un agente che supporta MCP può evitare il polling: vedi [Configurazione MCP](/it/automation/mcp-setup/).

## Vedi anche

- [Agenti IA: avvio rapido](/it/automation/quick-start/)
- [Verbi di controllo](/it/automation/control-verbs/)
