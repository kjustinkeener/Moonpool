---
title: "Moonpool-foutmeldingen uitgelegd: already running, requires a command en meer"
description: "Zoek de exacte tekst van Moonpool-foutmeldingen op, zoals already running, requires a command, stale token en Update failed, met de betekenis en de oplossing van elk."
---

Plak de melding die je ziet in de zoekfunctie van de pagina, of loop de tabellen door. Meldingen
worden geciteerd zoals Moonpool ze toont. Tekst tussen `<punthaken>` wordt vervangen door een
waarde (een app-id, een pad of een fout van het systeem). Symptomen die geen foutmelding zijn,
staan op [Probleemoplossing en veelgestelde vragen](/nl/support/troubleshooting/).

## Een app starten en stoppen

| Melding | Betekenis en oplossing |
| --- | --- |
| `already running` | Moonpool heeft al een terminal voor deze app. Stop de app eerst, of gebruik Herstarten. |
| `stopped during launch` | Er is op Stoppen gedrukt terwijl de start nog bezig was. Start opnieuw. |
| `app has no launch command` | Het item heeft geen `command`. Voeg er een toe in de app-editor of in `apps.json`. Alleen een `static`-item met een `url` kan zonder. |
| `unknown app: <id>` | Er is geen app met die `id` geladen. Controleer de id en gebruik daarna Opnieuw laden als je `apps.json` met de hand hebt bewerkt. |
| `unknown app id: <id>` | Hetzelfde probleem, gemeld aan een script of agent. Toon de apps met `moonpool_list_apps`. |
| `did not reach running in time` | Vanuit een script of agent: de app werd binnen 25 seconden niet als Actief gezien. Controleer `port` of `processName` en lees de uitvoer. Zie [De statusstip klopt niet](/nl/support/troubleshooting/#de-statusstip-klopt-niet). |
| `still running after stop` | Na 15 seconden staat de app nog steeds als Actief. Stel `killMode` in. Zie [Stoppen en herstarten](/nl/apps/stop-and-restart/). |
| `refusing to open non-web url: <url>` | De `url` is niet `http://`, `https://`, `mailto:` of `file://`. Corrigeer de `url`. |
| `[process exited]` | Geen fout: het commando van de app is geëindigd. Getoond in het terminaltabblad (in de app als `[proces beëindigd]`). |

## Validatie van apps.json

Moonpool weigert een `apps.json` die een regel schendt en behoudt de laatst geladen lijst.
`<n>` is de positie van het item in het bestand, geteld vanaf 1.

| Melding | Oplossing |
| --- | --- |
| `apps.json entry <n> has invalid id "<id>"; use letters, digits, '.', '_', and '-' without a leading '-'` | Wijzig de `id`. |
| `duplicate app id "<id>"` | Twee items delen een `id`. Maak elke id uniek. |
| `apps.json entry <n> (<id>) has an empty name` | Vul `name` in. |
| `apps.json entry <n> (<id>) has an empty group` | Vul `group` in. |
| `apps.json entry <n> (<id>) has unknown type "<type>"` | `type` moet `web`, `desktop`, `static` of `cli` zijn. |
| `apps.json entry <n> (<id>) has invalid port 0` | `port` moet 1 tot 65535 zijn. |
| `apps.json entry <n> (<id>) requires a url` | Een `static`-item heeft een `url` nodig. |
| `apps.json entry <n> (<id>) requires a command` | Elk ander type heeft een `command` nodig. |

Als je in de app-editor opslaat zonder naam, verschijnt "naam is verplicht." (in het Engels
`name is required.`). De banner-tekst, "apps.json bevat een fout; de laatst geladen lijst wordt
getoond." of "apps.json bevat een fout, dus er zijn geen apps geladen.", en hoe je herstelt, staan
onder [apps.json bevat een fout](/nl/support/troubleshooting/#appsjson-bevat-een-fout). Als de
banner meldt dat opslaan is gepauzeerd, eindigt de melding op `Repair apps.json and reload it
before saving from Moonpool`. De volledige lijst met regels staat in
[Validatie](/nl/apps/apps-json/#validatie).

## Instellingen, updates en het installatieprogramma

| Melding | Betekenis en oplossing |
| --- | --- |
| `... Repair settings.json and restart Moonpool before changing settings` | `settings.json` is beschadigd. Herstel of verwijder het en start opnieuw. Zie [settings.json](/nl/data/settings-json/#lezen-en-repareren). |
| `Update failed: <error>` | Het downloaden of installeren van een update is mislukt (in de app: "Update mislukt: <error>"). Zie [Wanneer een update mislukt](/nl/data/updating/#wanneer-een-update-mislukt). |
| `Update check failed: <error>` | De controle op updates in Over is mislukt (in de app: "Controle op updates mislukt: <error>"). De tekst na de dubbele punt geeft de reden. Probeer het later opnieuw. |
| `Install failed: <error>` | Het installatieprogramma is gestopt bij de stap die na de dubbele punt wordt genoemd, bijvoorbeeld `copy exe: ...`. Sluit elke Moonpool die vanuit `%USERPROFILE%\.moonpool` draait af en probeer het opnieuw. |
| `target folder does not exist` | De map die voor een draagbare kopie is gekozen, bestaat niet meer. Kies een bestaande map. |

## MCP en scripts

| Melding | Betekenis en oplossing |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Start Moonpool, of laat de agent die tool aanroepen. Bij een draagbare kopie noemt de melding de kopie. |
| `frontend not loaded` | Het hubvenster is nog niet klaar met laden. Wacht en probeer opnieuw. |
| `invalid app_id: use only letters, digits, '.', '_', '-' (no leading '-')` | De agent heeft een id doorgegeven die de MCP-server niet accepteert. Gebruik de id uit `moonpool_list_apps`. |
| `stale token: apps.json changed since it was read ...` | Lees `apps.json` opnieuw, pas de wijziging opnieuw toe en schrijf dan. |
| `rejected invalid manifest: ...` | De nieuwe `apps.json` voldeed niet aan de validatie (zie hierboven). Het bestand is niet gewijzigd. |
| `no console output recorded for '<id>' (not launched this session)` | `moonpool_app_output` is gevraagd voor een app die sinds het starten van Moonpool niet heeft gedraaid. |

Meer in [MCP-tools](/nl/automation/mcp-tools/) en
[MCP-installatie](/nl/automation/mcp-setup/#als-de-tools-niet-werken).

## Fouten van andere programma's

- [`Error: listen EADDRINUSE: address already in use :::3000` en `Port 5173 is in use`](/nl/support/port-already-in-use/)
- [`Windows protected your PC`](/nl/support/windows-protected-your-pc/)
- [WebView2-runtime ontbreekt](/nl/support/webview2-runtime-missing/)
