---
title: "Een AI-agent via MCP met Moonpool verbinden"
description: "Registreer moonpool.exe mcp als stdio-MCP-server bij je host, geïnstalleerd of draagbaar, en leer hoe Moonpool de eigen MCP-helper van een app volgt."
---

Het uitvoerbare bestand van Moonpool is zijn eigen MCP-server. Registreer het bij de host als stdio-server
die `moonpool.exe` uitvoert met het ene argument `mcp`.

## De server registreren

Geïnstalleerd is het programma `%USERPROFILE%\.moonpool\moonpool.exe`. Draagbaar is het de `moonpool.exe` in je map `.moonpool\`. Gebruik dat volledige pad als
`command`. Voor een host die een `.mcp.json` leest:

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

In een JSON-bestand moeten de backslashes worden verdubbeld, zoals hierboven. Een host met registratie via de opdrachtregel,
zoals Claude Code, kan het in één stap toevoegen:

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

De server meldt zich aan als
`moonpool`, spreekt MCP-protocolrevisie `2025-06-18` en biedt alleen tools aan (hij toont geen
resources of prompts). Tools verschijnen voor de agent als `moonpool_*`; zie
[MCP-tools](/nl/automation/mcp-tools/).

## Meer dan één Moonpool

De geïnstalleerde Moonpool en elke draagbare kopie zijn aparte starters, elk met eigen apps,
en ze kunnen allemaal tegelijk draaien. De `moonpool.exe mcp` van een kopie stuurt altijd die kopie aan. Om een
agent er meerdere te laten gebruiken, registreer je elke onder een eigen naam, wijzend naar de exe van die kopie:

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

Twee kopieën onder dezelfde naam registreren zorgt er in de meeste hosts voor dat de ene de andere vervangt. De
toolnamen zijn voor elke kopie hetzelfde, dus de host onderscheidt ze aan de naam die je
registreert. Een draagbare kopie meldt zich ook aan als `moonpool (<folder>)` en de
serverinstructies noemen de map, zodat de agent kan zien met welke kopie hij praat.

## Opmerkingen

- `moonpool.exe mcp` opent nooit een venster en start nooit het installatieprogramma. Het sluit af wanneer de
  host zijn invoer sluit.
- Het gebruikt de configuratiemap en het besturingskanaal van de exe waarmee het is gestart, dus een
  draagbare exe leest de gegevens van de draagbare map en stuurt die draagbare kopie aan. Een exe
  geldt alleen als draagbaar zolang `moonpool.portable` ernaast staat. Elke andere
  `moonpool.exe`, waar die ook staat, gebruikt de map van de geïnstalleerde Moonpool
  (`%USERPROFILE%\.moonpool\moonpool-config\`) en stuurt de geïnstalleerde Moonpool aan.
- De meeste tools hebben een draaiende Moonpool nodig. Als die niet draait, kan de agent eerst
  `moonpool_bootup_launcher` aanroepen.
- `moonpool_launcher_paths` toont de mappen die de hub gebruikt naast die welke het MCP-proces
  bepaalt. Een verschil betekent dat de agent naar een andere `apps.json` kijkt dan de hub.

## Hosts in een sandbox

Sommige hosts draaien hun tools in een verpakte (Store/MSIX) sandbox die AppData omleidt naar een
privékopie per pakket. Moonpool detecteert dit wanneer zijn configuratiemap of exe wordt omgezet naar
een pad zoals `...\Packages\<package>\LocalCache\...`.

Het detecteert het ook wanneer het besturingskanaal antwoordt maar `state.json` niet kan worden gelezen. De
tools die bestanden lezen of schrijven (`moonpool_app_output`, `moonpool_read_config`,
`moonpool_write_config`, `moonpool_restore_config`) geven dan een fout die de
oorzaak noemt, in plaats van lege of verouderde gegevens. Tools die alleen het besturingskanaal gebruiken, zoals
`moonpool_list_apps`, worden niet geblokkeerd zolang het kanaal bereikbaar is. Als de sandbox ook
het kanaal verbergt, melden de tools de sandbox in plaats van "Moonpool is not running".
Gebruik in dat geval de [opdrachtregel](/nl/automation/command-line/) vanuit een shell buiten de sandbox.

## Apps die een eigen MCP-server hebben

Veel apps in Moonpool worden zelf door een MCP-host bereikt via een helperproces `<exe> mcp`.
Moonpool zoekt een proces waarvan de naam overeenkomt met de `processName` van de app en waarvan het
eerste argument `mcp` is, zoals `notes-app.exe mcp`. Als de server onder een andere naam draait, zoals een hernoemde kopie, stel dan het jokerteken `mcpProcessName` van de app in (zie [Velden](/nl/apps/fields/#mcpprocessname)); een proces dat daarmee overeenkomt telt zonder het argument `mcp`.

- Zolang er een is gekoppeld, toont de zijbalk van de app een MCP-subrij als actief, en
  `moonpool_list_apps` voegt `[mcp: running]` toe aan de regel van de app. De helper telt niet
  als de app zelf die draait.
- Zodra een helper is waargenomen, onthoudt Moonpool hem (in `mcp_seen.json` in de configuratie-
  map), dus de MCP-subrij blijft zichtbaar als gestopt, en `moonpool_list_apps` toont
  `[mcp: stopped]`, nadat de helper is afgesloten.
- De MCP-subrij wordt bestuurd door de instelling `showMcpProcesses`
  ([venster Instellingen](/nl/using/settings/)).
- `moonpool_stop_mcp_server` beëindigt de helper en laat de app met rust. Er is geen start-
  tegenhanger: de host die de helper bezit start hem opnieuw bij zijn volgende toolaanroep.

## Als de tools niet werken

- **De host toont geen `moonpool_*`-tools.** Controleer of `command` het volledige pad naar
  `moonpool.exe` is en `args` `["mcp"]`, en start dan de host opnieuw.
- **Elke tool zegt dat Moonpool niet draait.** Start Moonpool, of roep
  `moonpool_bootup_launcher` aan. Zorg dat de geregistreerde exe de kopie is die je draait.
- **Een wijziging verschijnt niet.** Roep `moonpool_launcher_paths` aan en vergelijk de mappen van de hub
  met die van het MCP-proces. Zie [Hosts in een sandbox](#hosts-in-een-sandbox).

Meer in [Problemen oplossen](/nl/support/troubleshooting/#mcp--en-scriptfouten).

## Zie ook

- [Een AI-agent (Claude Code, Codex, Cursor) een MCP-server geven om lokale apps te starten en te stoppen](/nl/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/)
- [MCP-tools](/nl/automation/mcp-tools/)
- [AI-agents: snelstart](/nl/automation/quick-start/)
