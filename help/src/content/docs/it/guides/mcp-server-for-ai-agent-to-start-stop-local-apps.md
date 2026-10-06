---
title: "Dai a un agente IA (Claude Code, Codex, Cursor) un server MCP per avviare e arrestare le app locali"
description: "Registra Moonpool come server MCP perché Claude Code, Codex o Cursor possano avviare, arrestare, riavviare e leggere l'output dei tuoi server di sviluppo."
---

Un agente IA di programmazione di solito esegue il tuo server di sviluppo digitando `npm run dev`
nella propria shell. Questo può bloccare l'agente, lasciare un processo orfano che occupa la
porta o avviare una seconda copia di qualcosa che hai già in esecuzione. Un server MCP permette
all'agente di chiamare strumenti per avviare e arrestare l'app che hai già configurato, invece di
ricostruirne la riga di comando.

## Il metodo Moonpool

L'eseguibile di Moonpool è esso stesso un server MCP: registra `moonpool.exe` con l'unico
argomento `mcp` come server stdio. Una volta che l'app è in `apps.json`, l'agente la avvia
tramite il suo id.

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

Registra il server. In Claude Code, con un solo comando (Moonpool installato; usa il percorso
completo del tuo exe):

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

Gli host che leggono un file JSON di server MCP, come il `mcp.json` di Cursor, accettano la
stessa struttura (con le barre rovesciate raddoppiate):

```json title="mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

Per Codex, aggiungi un server con lo stesso comando e l'argomento `mcp` nella sua configurazione
(`~/.codex/config.toml`):

```toml title="config.toml"
[mcp_servers.moonpool]
command = 'C:\Users\you\.moonpool\moonpool.exe'
args = ["mcp"]
```

I nomi esatti del file e delle chiavi dipendono da ciascun host, quindi consulta la sua
documentazione MCP se la tua versione è diversa. Moonpool richiede soltanto il percorso completo
di `moonpool.exe` e `mcp` come argomento. Riavvia poi l'host.

## Cosa può fare l'agente

Gli strumenti compaiono come `moonpool_*`. Quelli per il lavoro quotidiano:

| Strumento | Uso |
| --- | --- |
| `moonpool_list_apps` | Trovare l'id di un'app e vedere se è in esecuzione. |
| `moonpool_start_app` | Avviare un'app tramite id e aprire la sua scheda del terminale. |
| `moonpool_stop_app` | Arrestarla, inclusi i suoi processi figli. |
| `moonpool_restart_app` | Arrestare, attendere che la porta si liberi, avviare. Da usare dopo una modifica al codice. |
| `moonpool_app_output` | Leggere ciò che l'app ha stampato, con `tail_lines` per limitarlo. |
| `moonpool_bootup_launcher` | Avviare Moonpool stesso se non è in esecuzione. |

Un ciclo tipico è `moonpool_restart_app`, poi `moonpool_app_output`. Gli altri strumenti
(lettura e scrittura di `apps.json`, schermate) sono in [Strumenti MCP](/it/automation/mcp-tools/).

## Se non funziona

Se ogni strumento risponde `Moonpool is not running - call moonpool_bootup_launcher first`,
Moonpool non è ancora avviato. Una modifica che non compare di solito significa che l'agente sta
guardando un `apps.json` diverso: chiama `moonpool_launcher_paths`. Vedi
[Se gli strumenti non funzionano](/it/automation/mcp-setup/#se-gli-strumenti-non-funzionano).

## Vedi anche

- [Configurazione di MCP](/it/automation/mcp-setup/)
- [Strumenti MCP](/it/automation/mcp-tools/)
- [Agenti IA: guida rapida](/it/automation/quick-start/)
- [Eseguire un server di sviluppo npm in background su Windows](/it/guides/run-npm-dev-server-in-background-windows/)
