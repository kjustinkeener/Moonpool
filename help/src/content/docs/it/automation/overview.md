---
title: "Automatizzare Moonpool con script e agenti IA"
description: "I tre modi per pilotare un Moonpool in esecuzione da script e agenti IA (MCP, riga di comando e verbi di controllo), come si relazionano e cosa modificano."
---

Moonpool può essere pilotato senza toccare la sua finestra. Ci sono tre superfici, tutte servite
dallo stesso Moonpool residente (l'istanza nell'area di notifica, qui chiamata hub).

Ogni copia di Moonpool è un hub a sé: quella installata e ogni copia portatile funzionano in
modo indipendente, ciascuna con il proprio canale di controllo. Una superficie raggiunge sempre
la copia di cui usa il `moonpool.exe`. Vedi [Modalità portatile](/it/data/portable-mode/#più-copie-contemporaneamente).

| Superficie | Cos'è | Riferimento |
| --- | --- | --- |
| Server MCP | `moonpool.exe mcp`, un server [MCP](https://modelcontextprotocol.io) stdio che viene avviato da un host IA. | [Configurazione MCP](/it/automation/mcp-setup/), [Strumenti MCP](/it/automation/mcp-tools/) |
| Riga di comando | `moonpool.exe <verb> [args]`. Una seconda esecuzione della stessa copia passa il verbo al suo hub tramite il canale di controllo e termina. | [Riga di comando](/it/automation/command-line/) |
| Canale di controllo | Una named pipe, `\\.\pipe\moonpool` (`\\.\pipe\moonpool-<id>` per una copia portatile), su Windows e un socket Unix su Linux, che accetta una richiesta JSON per riga. | [Verbi di controllo](/it/automation/control-verbs/) |

## Come si relazionano

- L'hub possiede tutto: l'avvio delle app, i registri di sessione, `apps.json`.
- Il server MCP è un client dell'hub, non una sua seconda copia. La maggior parte delle chiamate
  agli strumenti viene inoltrata all'hub tramite il canale di controllo, e la risposta torna come
  risultato dello strumento. Le eccezioni: `moonpool_bootup_launcher` avvia direttamente
  `moonpool.exe`; `moonpool_app_output` e gli strumenti di configurazione chiedono all'hub di
  scrivere un file e poi lo leggono; `moonpool_launcher_paths` aggiunge ai percorsi dell'hub
  quelli propri del processo MCP.
- Se un hub è in esecuzione lo si stabilisce interrogando quel canale, non cercando un processo.
  Un hub che risponde è in esecuzione; una pipe o un socket mancanti significano che non lo è.
- Ogni superficie esegue gli stessi gestori della finestra, quindi un verbo fa ciò che fa il
  clic corrispondente.
- Se nessun hub è in esecuzione, gli strumenti che agiscono su di esso, incluso
  `moonpool_list_apps`, rifiutano con "Moonpool is not running". Non esiste un elenco obsoleto.
  `moonpool_bootup_launcher` lo avvia. Se qualcosa occupa il canale ma non risponde entro pochi
  secondi, l'errore dice che un processo Moonpool potrebbe essersi bloccato.
- Il server MCP non ricade più sul pilotare una build dell'hub precedente al canale di
  controllo. Aggiorna quella copia, oppure chiudila e avviala di nuovo.

## Cosa può modificare le cose

| Può modificare | Superfici |
| --- | --- |
| Avviare, arrestare o riavviare un'app | MCP, riga di comando, pipe |
| Riscrivere `apps.json` | MCP (`moonpool_write_config`, `moonpool_restore_config`), riga di comando, pipe |
| Chiudere Moonpool | MCP (`moonpool_shutdown_launcher`), riga di comando (`quit`), pipe |
| Terminare il processo helper MCP di un'app | MCP (`moonpool_stop_mcp_server`), pipe (`stop-mcp`) |
| Ricaricare `apps.json`, recuperare di nuovo le icone, mostrare la finestra | MCP (`moonpool_reload_config`, `moonpool_refresh_app_icons`, `moonpool_raise_launcher`), riga di comando (`reload`, `refresh-icons`, `show`), pipe |
| Aprire una finestra o una scheda del terminale | pipe (`open-window`) |
| Cancellare gli avvistamenti memorizzati degli helper MCP | MCP (`moonpool_reset_mcp_seen`), pipe (`reset-mcp-seen`) |

Strumenti di sola lettura: `moonpool_list_apps`, `moonpool_app_output`, `moonpool_read_config`,
`moonpool_launcher_paths`, `moonpool_window_state`, `moonpool_screenshot`.

## Proprietà di sicurezza

- **Le scritture della configurazione sono protette.** Una scrittura deve portare il token di
  versione dell'ultima lettura, un token obsoleto viene rifiutato e il nuovo `apps.json` viene
  convalidato prima di scrivere qualsiasi cosa. Una scrittura rifiutata lascia `apps.json`
  intatto. Vedi [Strumenti MCP](/it/automation/mcp-tools/#configurazione).
- **Gli id delle app sono limitati.** Il server MCP accetta solo lettere, cifre, `.`, `_` e `-`,
  e mai un `-` iniziale, quindi un id non può essere interpretato come un'opzione della riga di
  comando.
- **Le schermate riguardano solo Moonpool.** `moonpool_screenshot` cattura una delle sei finestre
  di Moonpool (`main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`), mai lo
  schermo o un'altra app. Il PNG viene creato in memoria e restituito inline; Moonpool non lo
  salva in un file.
- **Nessuna autenticazione sul canale.** Moonpool non aggiunge alcun login o token alla pipe o al
  socket di controllo. Qualsiasi processo che possa aprirli può inviare verbi. Su Linux
  il file socket viene creato con modalità `0600`, quindi solo il tuo utente può farlo.
- **Gli host in sandbox vengono rilevati.** Se il server MCP rileva di essere eseguito in una
  sandbox in pacchetto (Store/MSIX), dove vedrebbe una copia privata dei file di Moonpool, gli
  strumenti che leggono o scrivono file (`moonpool_app_output`, `moonpool_read_config`,
  `moonpool_write_config`, `moonpool_restore_config`) restituiscono un errore che ne spiega il
  motivo anziché dati obsoleti. Gli strumenti che usano solo il canale di controllo non vengono
  bloccati. Vedi [Configurazione MCP](/it/automation/mcp-setup/#host-in-sandbox).

## Piattaforma

Il canale di controllo esiste su ogni piattaforma: una named pipe su Windows, un socket Unix su
Linux (posizione in [Verbi di controllo](/it/automation/control-verbs/#dove-è-in-ascolto)).
Solo `screenshot` (e quindi `moonpool_screenshot`) è esclusivo di Windows; su Linux
restituisce "not supported on this platform". I verbi della riga di comando funzionano su ogni
piattaforma.

## Vedi anche

- [Agenti IA: avvio rapido](/it/automation/quick-start/)
- [Configurazione MCP](/it/automation/mcp-setup/)
