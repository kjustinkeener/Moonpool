---
title: "Moonpool automatiseren met scripts en AI-agents"
description: "De drie manieren om een draaiende Moonpool aan te sturen vanuit scripts en AI-agents (MCP, opdrachtregel en besturings-verbs), hoe ze zich tot elkaar verhouden en wat elk kan wijzigen."
---

Moonpool kan worden aangestuurd zonder zijn venster aan te raken. Er zijn drie oppervlakken, alle bediend door
dezelfde resident draaiende Moonpool (de instantie in het systeemvak, hier de hub genoemd).

Elke Moonpool-kopie is zijn eigen hub: de geïnstalleerde en elke draagbare kopie draaien
onafhankelijk, elk met een eigen besturingskanaal. Een oppervlak bereikt altijd de kopie waarvan de
`moonpool.exe` het gebruikt. Zie [Draagbare modus](/nl/data/portable-mode/#meerdere-kopieën-tegelijk).

| Oppervlak | Wat het is | Naslag |
| --- | --- | --- |
| MCP-server | `moonpool.exe mcp`, een stdio-[MCP](https://modelcontextprotocol.io)-server die een AI-host start. | [MCP-installatie](/nl/automation/mcp-setup/), [MCP-tools](/nl/automation/mcp-tools/) |
| Opdrachtregel | `moonpool.exe <verb> [args]`. Een tweede uitvoering van dezelfde kopie geeft de verb via het besturingskanaal door aan zijn hub en sluit af. | [Opdrachtregel](/nl/automation/command-line/) |
| Besturingskanaal | Een named pipe, `\\.\pipe\moonpool` (`\\.\pipe\moonpool-<id>` voor een draagbare kopie), onder Windows en een Unix-socket onder Linux, met één JSON-verzoek per regel. | [Besturings-verbs](/nl/automation/control-verbs/) |

## Hoe ze zich tot elkaar verhouden

- De hub is eigenaar van alles: apps starten, de sessielogs, `apps.json`.
- De MCP-server is een client van de hub, geen tweede kopie ervan. De meeste toolaanroepen worden
  via het besturingskanaal doorgestuurd naar de hub, en het antwoord komt terug als toolresultaat.
  De uitzonderingen: `moonpool_bootup_launcher` start `moonpool.exe` zelf;
  `moonpool_app_output` en de configuratietools vragen de hub een bestand te schrijven en lezen het dan;
  `moonpool_launcher_paths` voegt de eigen paden van het MCP-proces toe aan die van de hub.
- Of een hub draait wordt bepaald door dat kanaal te pingen, niet door naar een
  proces te zoeken. Een hub die antwoordt draait; een ontbrekende pipe of socket betekent dat hij niet draait.
- Elk oppervlak voert dezelfde handlers uit als het venster, dus een verb doet wat de overeenkomstige klik
  doet.
- Als er geen hub draait, weigeren de tools die erop inwerken, waaronder `moonpool_list_apps`, met
  "Moonpool is not running". Er is geen verouderde lijst. `moonpool_bootup_launcher` start hem.
  Als iets het kanaal bezet houdt maar niet binnen een paar seconden antwoordt, zegt de fout dat een
  Moonpool-proces mogelijk vastgelopen is.
- De MCP-server valt niet langer terug op het aansturen van een hub-build van vóór het besturingskanaal.
  Werk die kopie bij, of sluit hem af en start hem opnieuw.

## Wat dingen kan wijzigen

| Kan wijzigen | Oppervlakken |
| --- | --- |
| Een app starten, stoppen of herstarten | MCP, opdrachtregel, pipe |
| `apps.json` herschrijven | MCP (`moonpool_write_config`, `moonpool_restore_config`), opdrachtregel, pipe |
| Moonpool afsluiten | MCP (`moonpool_shutdown_launcher`), opdrachtregel (`quit`), pipe |
| Het MCP-helperproces van een app beëindigen | MCP (`moonpool_stop_mcp_server`), pipe (`stop-mcp`) |
| `apps.json` opnieuw laden, pictogrammen opnieuw ophalen, het venster tonen | MCP (`moonpool_reload_config`, `moonpool_refresh_app_icons`, `moonpool_raise_launcher`), opdrachtregel (`reload`, `refresh-icons`, `show`), pipe |
| Een venster of een terminaltabblad openen | pipe (`open-window`) |
| Onthouden waarnemingen van MCP-helpers wissen | MCP (`moonpool_reset_mcp_seen`), pipe (`reset-mcp-seen`) |

Alleen-lezen tools: `moonpool_list_apps`, `moonpool_app_output`, `moonpool_read_config`,
`moonpool_launcher_paths`, `moonpool_window_state`, `moonpool_screenshot`.

## Veiligheidseigenschappen

- **Configuratieschrijfacties zijn beveiligd.** Een schrijfactie moet het versietoken van de laatste leesactie meenemen, een
  verouderd token wordt afgewezen en de nieuwe `apps.json` wordt gevalideerd voordat er iets wordt geschreven. Een
  afgewezen schrijfactie laat `apps.json` onaangeroerd. Zie [MCP-tools](/nl/automation/mcp-tools/#configuratie).
- **App-id's zijn beperkt.** De MCP-server accepteert alleen letters, cijfers, `.`, `_` en `-`,
  en nooit een voorafgaande `-`, zodat een id niet als opdrachtregelvlag kan worden gelezen.
- **Schermafbeeldingen zijn alleen van Moonpool.** `moonpool_screenshot` legt een van de eigen zes vensters van Moonpool vast
  (`main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`), nooit het scherm of
  een andere app. De PNG wordt in het geheugen opgebouwd en inline teruggegeven; Moonpool slaat hem niet op in een
  bestand.
- **Geen authenticatie op het kanaal.** Moonpool voegt geen login of token toe aan de besturingspipe of
  -socket. Elk proces dat hem kan openen kan verbs sturen. Onder Linux wordt het socketbestand
  aangemaakt met modus `0600`, dus alleen je eigen gebruiker kan dat.
- **Hosts in een sandbox worden gedetecteerd.** Als de MCP-server merkt dat hij in een verpakte
  (Store/MSIX) sandbox draait, waar hij een privékopie van de bestanden van Moonpool zou zien, geven de tools die
  bestanden lezen of schrijven (`moonpool_app_output`, `moonpool_read_config`,
  `moonpool_write_config`, `moonpool_restore_config`) een fout met uitleg in plaats van
  verouderde gegevens. Tools die alleen het besturingskanaal gebruiken worden niet geblokkeerd. Zie
  [MCP-installatie](/nl/automation/mcp-setup/#hosts-in-een-sandbox).

## Platform

Het besturingskanaal bestaat op elk platform: een named pipe onder Windows, een Unix-socket onder Linux (locatie in [Besturings-verbs](/nl/automation/control-verbs/#waar-het-luistert)). Alleen
`screenshot` (en dus `moonpool_screenshot`) werkt alleen onder Windows; onder Linux geeft het
"not supported on this platform" terug. De verbs van de opdrachtregel werken op elk platform.

## Zie ook

- [AI-agents: snelstart](/nl/automation/quick-start/)
- [MCP-installatie](/nl/automation/mcp-setup/)
