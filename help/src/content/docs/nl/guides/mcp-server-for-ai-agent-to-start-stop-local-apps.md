---
title: "Een AI-agent (Claude Code, Codex, Cursor) een MCP-server geven om lokale apps te starten en te stoppen"
description: "Registreer Moonpool als MCP-server zodat Claude Code, Codex of Cursor je dev-servers kan starten, stoppen, herstarten en de uitvoer ervan kan lezen, zonder extra kopieën."
---

Een AI-codeeragent voert je dev-server meestal uit door `npm run dev` in de eigen shell te
typen. Dat kan de agent blokkeren, een verweesd proces achterlaten dat de poort bezet houdt,
of een tweede kopie starten van iets wat je al draaiend hebt. Met een MCP-server kan de agent
tools aanroepen om de app te starten en te stoppen die je al hebt ingesteld, in plaats van
de commandoregel opnieuw te reconstrueren.

## De Moonpool-manier

Het uitvoerbare bestand van Moonpool is zijn eigen MCP-server: registreer `moonpool.exe` met
het ene argument `mcp` als stdio-server. Zodra de app in `apps.json` staat, start de agent
hem op id.

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

Registreer de server. In Claude Code kan dat met één commando (geïnstalleerde Moonpool;
gebruik het volledige pad van je eigen exe):

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

Hosts die een JSON-bestand met MCP-servers lezen, zoals `mcp.json` van Cursor, nemen
dezelfde vorm aan (backslashes verdubbeld):

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

Voeg voor Codex een server toe met hetzelfde commando en argument `mcp` in de
configuratie (`~/.codex/config.toml`):

```toml title="config.toml"
[mcp_servers.moonpool]
command = 'C:\Users\you\.moonpool\moonpool.exe'
args = ["mcp"]
```

De exacte bestands- en sleutelnamen zijn per host verschillend, dus raadpleeg de
MCP-documentatie van je host als je versie afwijkt. Moonpool heeft alleen het volledige pad
naar `moonpool.exe` en `mcp` als argument nodig. Start de host daarna opnieuw.

## Wat de agent kan doen

De tools verschijnen als `moonpool_*`. Die voor het dagelijkse werk:

| Tool | Gebruik |
| --- | --- |
| `moonpool_list_apps` | De id van een app vinden en zien of de app actief is. |
| `moonpool_start_app` | Een app op id starten en het terminaltabblad openen. |
| `moonpool_stop_app` | De app stoppen, inclusief de onderliggende processen. |
| `moonpool_restart_app` | Stoppen, wachten tot de poort vrij is, starten. Gebruik dit na een codewijziging. |
| `moonpool_app_output` | Lezen wat de app heeft uitgevoerd, met `tail_lines` om de omvang te beperken. |
| `moonpool_bootup_launcher` | Moonpool zelf starten als het niet draait. |

Een typische cyclus is `moonpool_restart_app`, gevolgd door `moonpool_app_output`. De
overige tools (`apps.json` lezen en schrijven, schermafbeeldingen) staan in
[MCP-tools](/nl/automation/mcp-tools/).

## Als het niet werkt

Als elke tool `Moonpool is not running - call moonpool_bootup_launcher first` meldt, is
Moonpool nog niet gestart. Een wijziging die niet verschijnt, betekent meestal dat de agent
naar een andere `apps.json` kijkt: roep `moonpool_launcher_paths` aan. Zie
[Als de tools niet werken](/nl/automation/mcp-setup/#als-de-tools-niet-werken).

## Zie ook

- [MCP-installatie](/nl/automation/mcp-setup/)
- [MCP-tools](/nl/automation/mcp-tools/)
- [AI-agents: snelstart](/nl/automation/quick-start/)
- [Een npm dev-server op de achtergrond uitvoeren op Windows](/nl/guides/run-npm-dev-server-in-background-windows/)
