---
title: "Collegare un agente IA a Moonpool tramite MCP"
description: "Registra moonpool.exe mcp come server MCP stdio nel tuo host, installato o portatile, e scopri come Moonpool monitora l'helper MCP di un'app."
---

L'eseguibile di Moonpool è esso stesso un server MCP. Registralo nell'host come server stdio
che esegue `moonpool.exe` con il solo argomento `mcp`.

## Registrare il server

Se installato, il programma è `%USERPROFILE%\.moonpool\moonpool.exe`. Se portatile, è il
`moonpool.exe` dentro la tua cartella `.moonpool\`. Usa quel percorso completo come
`command`. Per un host che legge un `.mcp.json`:

```json title=".mcp.json" {5}
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

In un file JSON le barre rovesciate devono essere raddoppiate, come sopra. Un host con
registrazione da riga di comando, come Claude Code, può aggiungerlo in un solo passaggio:

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

Il server si presenta come
`moonpool`, parla la revisione `2025-06-18` del protocollo MCP ed espone solo strumenti (non
elenca risorse né prompt). Gli strumenti compaiono all'agente come `moonpool_*`; vedi
[Strumenti MCP](/it/automation/mcp-tools/).

## Più di un Moonpool

Il Moonpool installato e ogni copia portatile sono launcher separati, ciascuno con le proprie
app, e possono essere tutti in esecuzione contemporaneamente. Il `moonpool.exe mcp` di una copia
pilota sempre quella copia. Per permettere a un agente di usarne più di una, registra ciascuna
con un nome distinto, puntando all'exe di quella copia:

```json title=".mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    },
    "moonpool-work": {
      "type": "stdio",
      "command": "D:\\Work\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

```powershell frame="terminal"
claude mcp add moonpool-work -- "D:\Work\.moonpool\moonpool.exe" mcp
```

Registrare due copie con lo stesso nome fa sì che nella maggior parte degli host una sostituisca
l'altra. I nomi degli strumenti sono gli stessi per ogni copia, quindi l'host li distingue in
base al nome con cui li registri. Una copia portatile si presenta anche come
`moonpool (<folder>)` e le istruzioni del suo server indicano la cartella, così l'agente può
vedere con quale copia sta parlando.

## Note

- `moonpool.exe mcp` non apre mai una finestra e non avvia mai il programma di installazione.
  Termina quando l'host chiude il suo input.
- Usa la cartella di configurazione e il canale di controllo dell'exe da cui è stato avviato,
  quindi un exe portatile legge i dati della cartella portatile e pilota quella copia portatile.
  Un exe è considerato portatile solo finché `moonpool.portable` si trova accanto ad esso.
  Qualsiasi altro `moonpool.exe`, ovunque si trovi, usa la cartella del Moonpool installato
  (`%USERPROFILE%\.moonpool\moonpool-config\`) e pilota il Moonpool installato.
- La maggior parte degli strumenti richiede un Moonpool in esecuzione. Se non lo è, l'agente può
  chiamare prima `moonpool_bootup_launcher`.
- `moonpool_launcher_paths` mostra le cartelle usate dall'hub accanto a quelle risolte dal
  processo MCP. Una differenza significa che l'agente sta guardando un `apps.json` diverso da
  quello dell'hub.

## Host in sandbox

Alcuni host eseguono i propri strumenti all'interno di una sandbox in pacchetto (Store/MSIX) che
reindirizza AppData a una copia privata per pacchetto. Moonpool lo rileva quando la sua cartella
di configurazione o il suo exe si risolvono in un percorso come
`...\Packages\<package>\LocalCache\...`.

Lo rileva anche quando il canale di controllo risponde ma `state.json` non può essere letto. Gli
strumenti che leggono o scrivono file (`moonpool_app_output`, `moonpool_read_config`,
`moonpool_write_config`, `moonpool_restore_config`) restituiscono allora un errore che ne indica
la causa, anziché dati vuoti o obsoleti. Gli strumenti che usano solo il canale di controllo,
come `moonpool_list_apps`, non vengono bloccati finché il canale è raggiungibile. Se la sandbox
nasconde anche il canale, gli strumenti segnalano la sandbox anziché «Moonpool is not running».
Usa invece la [riga di comando](/it/automation/command-line/) da una shell fuori dalla sandbox.

## App che hanno un proprio server MCP

Molte app in Moonpool sono a loro volta raggiunte da un host MCP tramite un processo helper
`<exe> mcp`. Moonpool cerca un processo il cui nome corrisponde al `processName` dell'app e il
cui primo argomento è `mcp`, come `notes-app.exe mcp`. Se il server viene eseguito con un altro
nome, ad esempio una copia rinominata, imposta il modello con caratteri jolly `mcpProcessName`
dell'app (vedi [Campi](/it/apps/fields/#mcpprocessname)); un processo che vi corrisponde conta
anche senza l'argomento `mcp`.

- Mentre ne è collegato uno, la barra laterale dell'app mostra una voce secondaria MCP come in
  esecuzione, e `moonpool_list_apps` aggiunge `[mcp: running]` alla riga dell'app. L'helper non
  conta come l'app stessa in esecuzione.
- Una volta visto un helper, Moonpool lo ricorda (in `mcp_seen.json` nella cartella di
  configurazione), quindi la voce secondaria MCP resta visibile come arrestata, e
  `moonpool_list_apps` mostra `[mcp: stopped]`, dopo che l'helper è terminato.
- La voce secondaria MCP è controllata dall'impostazione `showMcpProcesses`
  ([finestra Impostazioni](/it/using/settings/)).
- `moonpool_stop_mcp_server` termina l'helper e lascia stare l'app. Non esiste un
  equivalente per l'avvio: l'host che possiede l'helper lo avvia di nuovo alla successiva
  chiamata a uno strumento.

## Se gli strumenti non funzionano

- **L'host non mostra alcuno strumento `moonpool_*`.** Verifica che `command` sia il percorso
  completo di `moonpool.exe` e che `args` sia `["mcp"]`, poi riavvia l'host.
- **Ogni strumento dice che Moonpool non è in esecuzione.** Avvia Moonpool, oppure chiama
  `moonpool_bootup_launcher`. Assicurati che l'exe registrato sia la copia che stai eseguendo.
- **Una modifica non compare.** Chiama `moonpool_launcher_paths` e confronta le cartelle
  dell'hub con quelle del processo MCP. Vedi [Host in sandbox](#host-in-sandbox).

Altro in [Risoluzione dei problemi](/it/support/troubleshooting/#errori-di-mcp-e-degli-script).

## Vedi anche

- [Dare a un agente IA (Claude Code, Codex, Cursor) un server MCP per avviare e arrestare app locali](/it/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/)
- [Strumenti MCP](/it/automation/mcp-tools/)
- [Agenti IA: avvio rapido](/it/automation/quick-start/)
