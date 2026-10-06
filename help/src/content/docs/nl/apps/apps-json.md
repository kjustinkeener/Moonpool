---
title: "apps.json bewerken: waar het staat, hoe je het opnieuw laadt en herstelt"
description: "Zoek het apps.json-bestand dat Moonpool voor elke beheerde app leest, bewerk het in de app-editor of handmatig, laad het opnieuw en herstel na een foute wijziging."
---

Elke app die Moonpool beheert is één item in `apps.json`. Je kunt het bewerken in de app-editor
(het dialoogvenster App toevoegen en App bewerken) of handmatig. Beide schrijven naar hetzelfde bestand. Sommige
tool-resultaten en meldingen noemen dit bestand het manifest.

## Waar de configuratie staat

| Modus | Configuratiemap |
| --- | --- |
| Geïnstalleerd (Windows) | `%USERPROFILE%\.moonpool\moonpool-config\` |
| Draagbaar | `moonpool-config\` naast `moonpool.exe` (in de map `.moonpool\`) |
| Linux | `$XDG_CONFIG_HOME/Moonpool/`, anders `~/.config/Moonpool/` |

`apps.json` staat in die map, naast deze items:

| Item | Doel |
| --- | --- |
| `apps.json.history\` | Terugdraairing met de laatste 10 geldige `apps.json`-bestanden. |
| `settings.json` | App-instellingen. Zie [settings.json](/nl/data/settings-json/). |
| `cli-output\<id>\` | Sessielogs per app. Zie [Logs](/nl/data/logs/). |
| `moonpool.log` | Debuglog, zolang **Debug-informatie naar een bestand loggen** aan staat. |
| `icons\` | Optionele pictogramoverschrijvingen `<id>.png` (ook `.ico`, `.svg`, `.jpg`, `.jpeg`, `.webp`). |
| `state.json` | Live statusmomentopname, om de paar seconden vernieuwd. |
| `dumps\` | Bestanden geschreven door de verbs `dump`, `read-config` en `restore-config`. |
| `mcp_seen.json` | Welke apps een MCP-helper hebben gehad. |
| `window-state.json` | De grootte en positie van het hubvenster. |
| `AI-README.md` | De gids voor AI-agents, bij elke start opnieuw geschreven. |

Welke hiervan je moet back-uppen staat in [Back-up en herstel](/nl/data/backup-and-recovery/#de-configuratiemap).

Bij de eerste start vult Moonpool `apps.json` met voorbeelditems. Een bestaand bestand wordt
nooit overschreven.

## Bewerken

- **Dialoogvenster.** Gebruik **App toevoegen** in het menu **...** bovenaan de zijbalk. Om een
  app te wijzigen gebruik je het potlood op de rij of klik je er met de rechtermuisknop op en kies je **Bewerken**. Het dialoogvenster
  valideert en slaat direct op.
- **Handmatig.** **apps.json bewerken** in hetzelfde menu opent het bestand in je standaardeditor.
  Sla het op en kies dan **Opnieuw laden** in het menu (of druk op F5 of Ctrl+R).

Handmatige wijzigingen worden pas opgepikt als je opnieuw laadt. Opnieuw laden leest het bestand
alleen; het schrijft het niet terug.

Opslaan vanuit het dialoogvenster herschrijft het hele bestand in een genormaliseerde, ingesprongen vorm. Sleutels die Moonpool
niet kent worden weggelaten, en JSON kent geen opmerkingen, dus bewaar notities in het veld `note`.

## Vorm

Het bestand is een JSON-array van objecten. Vier sleutels zijn verplicht bij elk item: `id`, `name`,
`group`, `type`. Al het andere is optioneel. Zie [App-velden](/nl/apps/fields/).

```json title="apps.json"
[
  { "id": "site", "name": "Site", "group": "Web apps", "type": "web",
    "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
    "url": "http://localhost:5173", "openBrowser": true }
]
```

Groepen verschijnen in de zijbalk in de volgorde waarin ze voor het eerst in het bestand voorkomen.

## Wat opnieuw laden doet

Opnieuw laden vervangt de lijst in het geheugen van Moonpool door de inhoud van het bestand. Starten, Stoppen en Herstarten
lezen het item op het moment van klikken, dus een gewijzigd `command`, `cwd`, `env` of beëindigingsinstelling
geldt de volgende keer dat je die app start of herstart. Opnieuw laden herstart nooit iets: een
app die al draait blijft draaien met de instellingen waarmee hij is gestart.

## Validatie

Moonpool valideert het hele bestand bij het laden, bij elke opslag en bij elke schrijfactie van een agent.
Eén fout item wijst het hele bestand af.

| Regel | De fout bevat |
| --- | --- |
| Geen geldige JSON, een verplichte sleutel ontbreekt of een waarde heeft het verkeerde type | de melding van de JSON-parser |
| `id` is leeg, begint met `-` of bevat andere tekens dan letters, cijfers, `.`, `_`, `-` | `invalid id` |
| Twee items hebben dezelfde `id` | `duplicate app id` |
| `name` is leeg | `has an empty name` |
| `group` is leeg | `has an empty group` |
| `type` is niet `desktop`, `web`, `static` of `cli` | `unknown type` |
| `port` is `0` (een `port` boven 65535 kan niet worden verwerkt) | `invalid port 0` |
| `static`-item zonder `url` | `requires a url` |
| Elk ander type zonder `command` | `requires a command` |

Fouten noemen het item op positie, bijvoorbeeld:

```text
apps.json entry 2 (site) requires a command
```

### De id

De `id` is de blijvende sleutel van het item. Hij benoemt de logmap en het pictogrambestand, en is wat
je meegeeft aan `moonpool.exe launch <id>` en aan agents. Het dialoogvenster leidt hem af van de naam
wanneer je een app toevoegt. Het zet de naam om naar kleine letters, verandert elke reeks
tekens anders dan `a` tot `z` en `0` tot `9` in één `-`, en verwijdert `-` aan beide
uiteinden. Een leeg resultaat wordt `app`. Is de id al in gebruik, dan komt er `-2`, `-3` enzovoort achter. Hij
wordt daarna nooit meer gewijzigd, dus een app hernoemen behoudt de id. De naam `Habit Tracker` krijgt de id
`habit-tracker`.

## Als het bestand fout is

- **Bij Opnieuw laden** blijft een bestand dat de validatie niet doorstaat onaangeroerd en houdt Moonpool de laatste
  lijst die is geladen. Een banner boven de zijbalk toont de fout, met een knop om het
  bestand te openen; de lijst blijft bruikbaar maar gedimd. Zie
  [Wanneer apps.json een fout bevat](/nl/using/hub-window/#wanneer-appsjson-een-fout-bevat).
- **Bij het opstarten** betekent een kapot bestand dat er geen lijst is om te behouden, dus Moonpool start zonder
  apps en de banner zegt dat. Herstel het bestand en kies **Opnieuw laden**, of zet een momentopname terug
  (hieronder, of met de tool `moonpool_restore_config`).
- In beide gevallen worden opslagacties vanuit het dialoogvenster (en hernoemen, verwijderen, pictogram kiezen) geweigerd totdat het
  bestand weer laadt, zodat het kapotte bestand nooit wordt overschreven. Herstel het bestand en kies
  **Opnieuw laden**.
- **Vanuit het dialoogvenster, een agent of een herstelactie** wordt een ongeldige wijziging afgewezen en blijft het bestand
  op schijf zoals het was.

Moonpool bewaart de laatste 10 goede versies van `apps.json` in `apps.json.history\`. Hoe je
terugdraait staat in [Back-up en herstel](/nl/data/backup-and-recovery/#appsjson-terugdraaien).
Symptomen en oplossingen staan in [Problemen oplossen](/nl/support/troubleshooting/#appsjson-bevat-een-fout).

## Agents

Een AI-agent hoort `apps.json` te wijzigen via de MCP-tools van Moonpool in plaats van via het bestand, zodat een
verouderde of ongeldige schrijfactie wordt afgewezen en een agent in een sandbox nooit een privékopie bewerkt. Zie
[MCP-tools](/nl/automation/mcp-tools/#configuratie).
