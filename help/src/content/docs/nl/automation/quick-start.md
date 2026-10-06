---
title: "Een AI-agent Moonpool laten instellen en aansturen: snelstart"
description: "Drie manieren om een AI-agent of een script Moonpool te laten instellen en aansturen, welke je voor je agent kiest en dezelfde actie getoond in elk."
---

Er zijn drie manieren om binnen te komen. Kies op basis van wat je agent kan.

| Je wilt | Gebruik | Begin hier |
| --- | --- | --- |
| Een agent die je apps vindt en toevoegt, eenmalig | **Prompt kopiëren** op het lege scherm van de hub | Hieronder |
| Een agent die apps start, stopt en uitleest als toolaanroepen | De MCP-server, `moonpool.exe mcp` | [MCP-installatie](/nl/automation/mcp-setup/) |
| Een script, of een agent zonder MCP | Verbs van de opdrachtregel | [Opdrachtregel](/nl/automation/command-line/) |

## Prompt kopiëren

Zonder open tabblad toont het CLI-paneel een kant-en-klare prompt ("Nieuw hier? Geef dit aan een AI-agent om je apps in te stellen:"). **Prompt kopiëren** zet die op het klembord. Plak hem in je
agent. Hij wijst de agent naar `AI-README.md` en `apps.json` in je configuratiemap en vraagt
je apps te vinden en te registreren. Als hij klaar is, kies je **Opnieuw laden**.

Moonpool herschrijft `AI-README.md` naast `apps.json` bij elke start, zodat het altijd overeenkomt met
de versie die je draait. Bewaar er geen eigen wijzigingen in.

## Dezelfde actie op drie manieren

| Actie | Opdrachtregel | Verb van het besturingskanaal | MCP-tool |
| --- | --- | --- | --- |
| Een app starten | `moonpool.exe launch <id>` | `launch` | `moonpool_start_app` |
| Een app stoppen | `moonpool.exe stop <id>` | `stop` | `moonpool_stop_app` |
| Een app herstarten | `moonpool.exe restart <id>` | `restart` | `moonpool_restart_app` |
| De uitvoer van een app lezen | `moonpool.exe dump <id> [out-path]` | `dump` | `moonpool_app_output` |
| Apps en status tonen | `state.json` lezen | `list` | `moonpool_list_apps` |
| `apps.json` opnieuw lezen | `moonpool.exe reload` | `reload` | `moonpool_reload_config` |
| `apps.json` lezen | `moonpool.exe read-config` | `read-config` | `moonpool_read_config` |
| `apps.json` vervangen | `moonpool.exe write-config <file> [token]` | `write-config` | `moonpool_write_config` |
| `apps.json` terugdraaien | `moonpool.exe restore-config [n]` | `restore-config` | `moonpool_restore_config` |
| Het venster tonen | `moonpool.exe show` | `show` | `moonpool_raise_launcher` |
| Moonpool starten | `moonpool.exe` | geen | `moonpool_bootup_launcher` |
| Moonpool afsluiten | `moonpool.exe quit` | `quit` | `moonpool_shutdown_launcher` |
| De gebruikte mappen tonen | `moonpool.exe paths` | `paths` | `moonpool_launcher_paths` |

De opdrachtregel drukt niets af; lees de uitkomst met een `--ticket` (zie
[De uitkomst lezen](/nl/automation/command-line/#de-uitkomst-lezen)). Het kanaal en MCP
antwoorden direct.

## Wanneer de tools van een agent falen

- `Moonpool is not running - call moonpool_bootup_launcher first`: start Moonpool, of laat de
  agent die tool aanroepen.
- Een wijziging "kwam niet door": vraag de agent om `moonpool_launcher_paths`. Als de mappen van de hub en MCP
  verschillen, leest de agent een andere `apps.json`. Zie
  [Hosts in een sandbox](/nl/automation/mcp-setup/#hosts-in-een-sandbox).
- Meerdere Moonpool-kopieën: registreer elke onder een eigen naam. Zie
  [Meer dan één Moonpool](/nl/automation/mcp-setup/#meer-dan-één-moonpool).

Een uitgewerkt voorbeeld voor Claude Code, Codex en Cursor staat in
[Een AI-agent een MCP-server geven om lokale apps te starten en te stoppen](/nl/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/).

Meer symptomen in [Problemen oplossen](/nl/support/troubleshooting/#mcp--en-scriptfouten).
