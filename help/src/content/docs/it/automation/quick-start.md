---
title: "Lascia che un agente IA configuri e pilota Moonpool: avvio rapido"
description: "Tre modi per lasciare che un agente IA o uno script configuri e pilota Moonpool, quale scegliere per il tuo agente e la stessa azione mostrata in ciascuno."
---

Ci sono tre modi per cominciare. Scegli in base a ciò che può fare il tuo agente.

| Vuoi | Usa | Parti da qui |
| --- | --- | --- |
| Un agente che trovi le tue app e le aggiunga, una volta sola | **Copia il prompt** nella schermata vuota dell'hub | Sotto |
| Un agente che avvii, arresti e legga le app tramite chiamate a strumenti | Il server MCP, `moonpool.exe mcp` | [Configurazione MCP](/it/automation/mcp-setup/) |
| Uno script, o un agente senza MCP | I verbi della riga di comando | [Riga di comando](/it/automation/command-line/) |

## Copia il prompt

Quando non c'è alcuna scheda aperta, il pannello CLI mostra un prompt già pronto («È la prima
volta? Passa questo testo a un agente IA perché configuri le tue app:»). **Copia il prompt** lo
mette negli appunti. Incollalo nel tuo agente. Indica all'agente `AI-README.md` e `apps.json`
nella tua cartella di configurazione e gli chiede di trovare le tue app e di registrarle.
Quando ha finito, scegli **Ricarica**.

Moonpool riscrive `AI-README.md` accanto a `apps.json` a ogni avvio, così corrisponde sempre
alla versione che usi. Non conservarvi modifiche tue.

## La stessa azione in tre modi

| Azione | Riga di comando | Verbo del canale di controllo | Strumento MCP |
| --- | --- | --- | --- |
| Avviare un'app | `moonpool.exe launch <id>` | `launch` | `moonpool_start_app` |
| Arrestare un'app | `moonpool.exe stop <id>` | `stop` | `moonpool_stop_app` |
| Riavviare un'app | `moonpool.exe restart <id>` | `restart` | `moonpool_restart_app` |
| Leggere l'output di un'app | `moonpool.exe dump <id> [out-path]` | `dump` | `moonpool_app_output` |
| Elencare le app e lo stato | leggere `state.json` | `list` | `moonpool_list_apps` |
| Rileggere `apps.json` | `moonpool.exe reload` | `reload` | `moonpool_reload_config` |
| Leggere `apps.json` | `moonpool.exe read-config` | `read-config` | `moonpool_read_config` |
| Sostituire `apps.json` | `moonpool.exe write-config <file> [token]` | `write-config` | `moonpool_write_config` |
| Ripristinare `apps.json` a una versione precedente | `moonpool.exe restore-config [n]` | `restore-config` | `moonpool_restore_config` |
| Mostrare la finestra | `moonpool.exe show` | `show` | `moonpool_raise_launcher` |
| Avviare Moonpool | `moonpool.exe` | nessuno | `moonpool_bootup_launcher` |
| Chiudere Moonpool | `moonpool.exe quit` | `quit` | `moonpool_shutdown_launcher` |
| Mostrare le cartelle in uso | `moonpool.exe paths` | `paths` | `moonpool_launcher_paths` |

La riga di comando non stampa nulla; leggi l'esito con un `--ticket` (vedi
[Leggere l'esito](/it/automation/command-line/#leggere-lesito)). Il canale e MCP
rispondono direttamente.

## Quando gli strumenti di un agente falliscono

- `Moonpool is not running - call moonpool_bootup_launcher first`: avvia Moonpool, oppure lascia
  che l'agente chiami quello strumento.
- Una modifica «non ha avuto effetto»: chiedi all'agente `moonpool_launcher_paths`. Se le cartelle
  dell'hub e di MCP differiscono, l'agente sta leggendo un `apps.json` diverso. Vedi
  [Host in sandbox](/it/automation/mcp-setup/#host-in-sandbox).
- Più copie di Moonpool: registra ciascuna con il proprio nome. Vedi
  [Più di un Moonpool](/it/automation/mcp-setup/#più-di-un-moonpool).

Un esempio pratico per Claude Code, Codex e Cursor si trova in
[Dare a un agente IA un server MCP per avviare e arrestare app locali](/it/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/).

Altri sintomi in [Risoluzione dei problemi](/it/support/troubleshooting/#errori-di-mcp-e-degli-script).
