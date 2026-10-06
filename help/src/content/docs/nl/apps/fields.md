---
title: "Elk apps.json-veld: type, standaardwaarde en wat het doet"
description: "Zoek elke sleutel van een apps.json-item op met type, standaardwaarde en welke app-typen hem gebruiken, overeenkomend met de namen in het dialoogvenster App bewerken."
---

Het dialoogvenster App bewerken toont dezelfde velden onder dezelfde namen. Velden die niet gelden voor
het gekozen type zijn in het dialoogvenster gedimd maar worden wel opgeslagen, met één uitzondering:
`stopCommand` wordt alleen opgeslagen zolang `killMode` `command` is.

![Het dialoogvenster App bewerken van name tot en met stopCommand, met de keuzelijst killMode omlijnd; ongebruikte velden zoals processName en stopCommand zijn gedimd](../../../../assets/screenshots/edit-app-dialog.png)

1. De keuzelijst `killMode`. Velden die hij niet gebruikt blijven gedimd.

| Veld | Type | Verplicht | Gebruikt door | Wat het doet |
| --- | --- | --- | --- | --- |
| `id` | string | ja | alle | Unieke sleutel. Letters, cijfers, `.`, `_`, `-`, niet beginnend met `-`. Zie [Overzicht](/nl/apps/apps-json/#de-id). |
| `name` | string | ja | alle | Label in de zijbalk. Niet leeg. |
| `group` | string | ja | alle | Kop in de zijbalk waaronder de app wordt getoond. Niet leeg bij handmatig bewerken; het dialoogvenster slaat een lege groep op als `Apps`. Elke tekst; een nieuwe naam maakt een nieuwe groep. |
| `type` | string | ja | alle | `web`, `desktop`, `static` of `cli`. Zie [App-typen](/nl/apps/types/). |
| `command` | string | alle behalve `static` | alle | Wordt in een terminal uitgevoerd om de app te starten, via `cmd /c` onder Windows en `$SHELL -c` elders (`/bin/sh` als `SHELL` niet is ingesteld). Optioneel voor `static`. |
| `cwd` | string | nee | alle met een `command` | Map waarin het commando draait. Standaard de eigen werkmap van Moonpool. Ondersteunt tokens en `./`. Zie [Paden en omgeving](/nl/apps/paths-and-environment/). |
| `port` | geheel getal, 1 tot 65535 | nee | alle | Actief zolang er iets antwoordt op deze poort op localhost (IPv4 of IPv6). Wordt gelezen door `killMode` `port`. |
| `processName` | string | nee | alle, vooral `desktop` | Actief zolang er een proces met deze naam bestaat. Hoofdletterongevoelig, met of zonder `.exe`, dus `my-app` komt overeen met `my-app.exe`. Onder Linux maximaal 15 tekens. Wordt gelezen door `killMode` `processName`. |
| `mcpProcessName` | string | nee | alle met een `processName` | Jokerteken-patroon voor de procesnaam van de MCP-server van deze app. `*` komt overeen met een willekeurige reeks tekens, `?` met één teken. Hoofdletterongevoelig, vergeleken met de hele naam, en `.exe` is optioneel. Een overeenkomend proces telt als de MCP-server van de app (de MCP-subrij in de zijbalk) en heeft `mcp` niet nodig als eerste argument. Zie [mcpProcessName](#mcpprocessname). |
| `url` | string | alleen `static` | `web`, `static` | Pagina om te openen. Alleen URL's met `http://`, `https://`, `mailto:` en `file://` worden geopend. |
| `openBrowser` | boolean, standaard `false` | nee | elk type met een `url` (het dialoogvenster dimt het voor `desktop` en `cli`) | Opent `url` automatisch zodra Moonpool detecteert dat de app draait (zie hieronder). |
| `killMode` | string | nee | alle | Extra opruimstap bij Stoppen en Herstarten: `processName`, `port`, `command` of `none`. Zie [Stoppen en herstarten](/nl/apps/stop-and-restart/). |
| `stopCommand` | string | nee | `killMode` `command` | Commando dat bij Stoppen wordt uitgevoerd. Genegeerd in elke andere modus. |
| `env` | object van strings | nee | alle | Extra omgevingsvariabelen. Het dialoogvenster bewerkt ze als één `KEY=VALUE` per regel. |
| `icon` | string | nee | alle | Afbeelding in de zijbalk: een bestandspad, een `http(s)`-URL of een `data:`-URI. Stel in via **Pictogram kiezen...** in het contextmenu van de app of handmatig. |
| `note` | string | nee | alle | Tooltip wanneer je in de zijbalk over de app zweeft. |

Een item met `env` en `killMode`:

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000", "NODE_ENV": "development" },
  "killMode": "port"
}
```

Zie [Het proces vinden en beëindigen dat een poort gebruikt](/nl/guides/find-and-kill-process-using-port-windows/)
voor hoe `port` en `killMode` samenwerken.

## mcpProcessName

Standaard beschouwt Moonpool een proces als de MCP-server van de app wanneer de naam overeenkomt met
`processName` en het eerste argument `mcp` is, zoals `notes-app.exe mcp`. Stel
`mcpProcessName` in wanneer de server onder een andere naam draait: een app die één exe bewaakt
terwijl de MCP-server een andere is (`mog.exe mcp`), of een hernoemde kopie van de server.

De waarde is een jokerteken-patroon. `*` komt overeen met een willekeurige reeks tekens (ook geen) en `?`
met precies één teken. Het wordt hoofdletterongevoelig vergeleken met de hele procesnaam, en een
patroon zonder `.exe` komt ook overeen met de naam met `.exe`. Een lege waarde geldt als niet ingesteld.

```json
{
  "id": "destiny",
  "name": "Destiny",
  "group": "Desktop apps",
  "type": "desktop",
  "processName": "destiny",
  "mcpProcessName": "destiny-mcp-*"
}
```

Dit komt overeen met een hernoemde kopie zoals `destiny-mcp-2706210170.exe`. Een proces dat overeenkomt met
`mcpProcessName` is de server, ongeacht of het met `mcp` is gestart, en telt nooit
als de app zelf die draait. Komt het patroon ook overeen met `processName` zelf (bijvoorbeeld
`destiny*`), dan vereist Moonpool nog steeds het argument `mcp`, zodat de echte app nooit wordt aangezien
voor zijn MCP-server. Zie [MCP-installatie](/nl/automation/mcp-setup/#apps-die-een-eigen-mcp-server-hebben).

## openBrowser

Moonpool opent `url` één keer, wanneer een app die Moonpool heeft gestart voor het eerst als Actief wordt gelezen. Daarvoor
is een `port` of `processName` nodig om dat te detecteren. Zonder beide betekent Actief alleen dat het
terminalproces leeft, en wordt de browser niet automatisch geopend. Zet `openBrowser`
uit als je commando zelf een browser opent. Een `static`-item zonder commando opent `url`
telkens wanneer je op Starten drukt, ongeacht `openBrowser`.

Twee apps die met dezelfde `port` zijn geconfigureerd, worden in de zijbalk gemarkeerd.

## Pictogrammen

Het pictogram van een app is de eerste van deze die bestaat:

1. Het veld `icon`.
2. `icons\<id>.<ext>` in de configuratiemap, bijvoorbeeld `icons\site.png`.
3. Een pictogrambestand in de eigen map van de app (zijn `cwd`, of de map van een `file:///`-`url`).
4. Voor `desktop` het pictogram van de gebouwde of draaiende `.exe`.
5. Voor `web` en `static` de `/favicon.ico` van de site, zodra de server draait.
6. Een glyph voor het type.

De meeste apps hebben geen pictograminstelling nodig.
