---
title: "Naslag van de MCP-tools van Moonpool: parameters en resultaten"
description: "Elke tool die de Moonpool-MCP-server aan agents biedt, met parameters, wat hij teruggeeft en de foutgevallen die je kunt tegenkomen."
---

Alle tools geven tekst terug, behalve `moonpool_screenshot`, dat een PNG-afbeelding teruggeeft. Een fout
komt terug als een toolresultaat dat als fout is gemarkeerd, met de reden als tekst. Voor de installatie zie
[MCP-installatie](/nl/automation/mcp-setup/).

Tools die `app_id` nemen hebben de `id` van de app uit `apps.json` nodig. Die mag alleen letters,
cijfers, `.`, `_` en `-` bevatten en niet met `-` beginnen, anders mislukt de aanroep met "invalid
app_id".

De meeste tools die op de hub inwerken mislukken met deze melding wanneer die niet draait.
`moonpool_bootup_launcher`, `moonpool_shutdown_launcher`, `moonpool_raise_launcher` en
`moonpool_launcher_paths` handelen dat geval zelf af (zie hun rijen). Bij een draagbare kopie
noemt de melding de kopie, bijvoorbeeld `Moonpool (<folder>)`.

```text
Moonpool is not running - call moonpool_bootup_launcher first
```

Aanroepen die op een uitkomst wachten krijgen na 45 seconden een time-out.

## Starter en apps

Voorbeeldresultaat van `moonpool_list_apps`:

```text
site  [running] (managed by Moonpool)  Site
notes-app  [stopped]  [mcp: stopped]  Notes App
```

| Tool | Parameters | Gedrag |
| --- | --- | --- |
| `moonpool_list_apps` | geen | Eén regel per app: `id  [running]` of `[stopped]`, `(managed by Moonpool)` waar van toepassing, `[mcp: running]` of `[mcp: stopped]` wanneer een MCP-helper is waargenomen, en dan de naam. Opgevraagd bij de draaiende hub via het besturingskanaal (verb `list`), dus live. Als Moonpool niet draait, mislukt het met "Moonpool is not running" in plaats van een verouderde lijst te tonen. Vlak na het starten van Moonpool, vóór de eerste statuscontrole, tonen apps `[status pending]`. Zolang `apps.json` een fout bevat, begint het resultaat met `apps.json has an error: <message>. This list is the last one that loaded; fix the file and call moonpool_reload_config.` Als het bestand al kapot was toen Moonpool startte, staat er dat er geen apps zijn geladen en wordt ook `moonpool_restore_config` voorgesteld. |
| `moonpool_bootup_launcher` | geen | Start Moonpool zelf en wacht maximaal 30 s tot het besturingskanaal antwoordt. Geeft "Moonpool started" terug, of "Moonpool is already running". Als het nieuwe proces meteen afsluit (het droeg over aan een Moonpool die nog aan het afsluiten was), start het er nog een. Als iets het kanaal bezet houdt zonder te antwoorden, meldt het dat een Moonpool-proces mogelijk vastgelopen is. |
| `moonpool_shutdown_launcher` | geen | Hetzelfde als Afsluiten in het systeemvakmenu. Wacht maximaal 30 s tot het besturingskanaal verdwijnt. Geeft "Moonpool shut down" terug, of "Moonpool is not running". |
| `moonpool_raise_launcher` | geen | Brengt het Moonpool-venster naar voren. Geeft "window shown" terug. Als Moonpool niet draait, start het dat en geeft "Moonpool was not running; started it" terug. |
| `moonpool_start_app` | `app_id` (verplicht) | Start de app en opent het terminaltabblad ervan. Geeft "launched" terug zodra hij draait, of de reden waarom niet (`unknown app id: <id>`, `did not reach running in time` na 25 s). Voor een `static`-item met alleen een `url` opent het de pagina en geeft ook "launched" terug. |
| `moonpool_stop_app` | `app_id` (verplicht) | Stopt de app. Geeft "stopped" terug, of een fout zoals `still running after stop` (na 15 s). |
| `moonpool_restart_app` | `app_id` (verplicht) | Stoppen, wachten tot de poort en het proces vrij zijn, starten. Geeft "restarted" terug. |
| `moonpool_app_output` | `app_id` (verplicht), `tail_lines` (geheel getal, standaard 200, minimaal 1) | De terminaluitvoer van de app voor de huidige Moonpool-sessie, ANSI-codes verwijderd. Als het log langer is dan `tail_lines`, begint de tekst met een regel met het pad van het volledige log. Mislukt met `no console output recorded for '<id>' (not launched this session)` als de app niet heeft gedraaid. Als het log bestaat maar leeg is, geeft het `(no output recorded for '<id>')` terug. |
| `moonpool_stop_mcp_server` | `app_id` (verplicht) | Beëindigt het gekoppelde MCP-helperproces van de app en laat de app draaien. Geeft "stopped" terug. Doet niets als de app noch `processName` noch `mcpProcessName` heeft. |
| `moonpool_refresh_app_icons` | geen | Haalt elk app-pictogram opnieuw op. Geeft "icons refreshed" terug. |

## Configuratie

Deze lezen en wijzigen `apps.json` via de hub, nooit het bestand op schijf. Een schrijfactie moet
het token van de laatste leesactie meenemen, een verouderd token wordt afgewezen en het nieuwe bestand wordt gevalideerd
voordat er iets wordt geschreven. Via de hub gaan is belangrijk omdat een agent in een host in een sandbox
een privékopie van de configuratiemap te zien kan krijgen in plaats van de echte.

| Tool | Parameters | Gedrag |
| --- | --- | --- |
| `moonpool_read_config` | geen | JSON-tekst met `manifest_text` (de exacte inhoud van het bestand), `token`, `valid`, `error` (null wanneer geldig) en `path`. `token` is `none` wanneer het bestand ontbreekt of leeg is. |
| `moonpool_write_config` | `manifest` (verplicht, de volledige nieuwe `apps.json`-tekst), `expected_token` (verplicht, van de laatste leesactie) | Valideert het manifest en vervangt `apps.json`, en laadt het dan. Geeft `apps.json updated; new version token <token>` terug. Een verouderd token mislukt met `stale token: apps.json changed since it was read ...`. Een ongeldig manifest mislukt met `rejected invalid manifest: ...`. In beide gevallen blijft het bestand onaangeroerd. Een lege `expected_token` wordt geweigerd. |
| `moonpool_restore_config` | `snapshot` (optioneel) | Zonder waarde: JSON-tekst met de opgeslagen momentopnamen, nieuwste eerst (`index`, `filename`, `millis`, `app_count`, `valid`). Met een index (1 = nieuwste) of een bestandsnaam valideert het die momentopname en herstelt die. Geeft `restored <file> (<n> apps); new version token <token>` terug. Er is geen token nodig: een herstelactie overschrijft het huidige bestand met opzet. |
| `moonpool_reload_config` | geen | Leest `apps.json` opnieuw. Geeft "apps.json reloaded" terug. Als het bestand niet kan worden geparseerd of gevalideerd, mislukt het met `apps.json has an error: ...` en houdt Moonpool de laatste lijst die is geladen. |
| `moonpool_launcher_paths` | geen | Toont de configuratiemap, `apps.json`, `state.json`, het log, de dumpmap, de pictogrammap, de draagbare vlag en het exe-pad van de hub, en daarna de configuratiemap, `apps.json`, `state.json`, dumpmap, draagbare vlag en het exe-pad van het MCP-proces (geen log of pictogrammen). Als de hub niet draait, luidt zijn helft `hub paths unavailable: ...` en wordt de MCP-helft nog steeds getoond. Gebruik het wanneer een wijziging niet doorwerkt. |

## Geavanceerd: testtools

`moonpool_screenshot` werkt alleen onder Windows; onder Linux en macOS mislukt het met "screenshot is not
supported on this platform". `moonpool_window_state` en `moonpool_reset_mcp_seen` werken op
elk platform.

`window` is een van `main`, `settings`, `about`, `installer`, `editor`, `help` of `themes`, en is standaard
`main`. Een onbekende naam mislukt met `unknown window '<name>'`.

| Tool | Parameters | Gedrag |
| --- | --- | --- |
| `moonpool_screenshot` | `window` (optioneel) | Legt de eigen inhoud van dat Moonpool-venster vast als inline PNG, maximaal 320 pixels aan de langste zijde. De grootte kan niet via MCP worden verhoogd. Mislukt met `window '<name>' is not open` als het niet zichtbaar is. Het kan geen enkele andere app vastleggen. |
| `moonpool_window_state` | `window` (optioneel) | JSON-tekst: `{"open":false}` wanneer het venster niet open is, anders `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. Bedoeld voor tests. |
| `moonpool_reset_mcp_seen` | `app_id` (optioneel) | Alleen voor tests. Wist het onthouden record "er is een MCP-helper waargenomen" voor één app, of voor elke app wanneer weggelaten, zodat de MCP-subrij van de zijbalk weer verborgen is tot er een helper wordt waargenomen. |

## Zie ook

- [MCP-installatie](/nl/automation/mcp-setup/)
- [Opdrachtregel](/nl/automation/command-line/)
