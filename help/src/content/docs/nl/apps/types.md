---
title: "Een app-type kiezen: web, desktop, static of cli"
description: "Leer hoe web-, desktop-, static- en cli-apps starten in Moonpool, hoe Actief per type wordt gedetecteerd en wat de knop Stoppen standaard doet."
---

`type` bepaalt welke velden ertoe doen en wat Stoppen standaard doet.

| | `web` | `desktop` | `static` | `cli` |
| --- | --- | --- | --- | --- |
| Nodig | `command` | `command` | `url` | `command` |
| Meestal ook | `port`, `url` | `processName` | `command` en `port`, als het zichzelf serveert | `cwd` |
| Starten | Voert `command` uit in een terminaltabblad | Voert `command` uit in een terminaltabblad | Geen `command`: opent `url` in de browser. Met een `command`: voert het uit in een terminaltabblad | Voert `command` uit in een terminaltabblad |
| Standaard `killMode` | `port` | `processName` | `none` | `none` |

![Het dialoogvenster App bewerken voor een web-app: type ingesteld op web met een beschrijving van één regel, en een ingevuld veld port](../../../../assets/screenshots/edit-app-type-and-port.png)

1. De keuzelijst `type`. De hintregel beschrijft wat dat type doet.
2. Het veld `port`. Bij een `web`-app volgt Actief of deze poort antwoordt.

## Hoe Actief wordt bepaald

Moonpool controleert dit om de paar seconden. Een app is Actief als een van deze voorwaarden geldt, ongeacht
het type:

- `processName` is ingesteld en er bestaat een proces met die naam. De eigen `<exe> mcp`-helperprocessen
  van Moonpool tellen niet mee.
- `port` is ingesteld en antwoordt op localhost.
- Moonpool heeft hem gestart, hij heeft noch `port` noch `processName`, en het proces van de terminal
  leeft nog.

Een `cli`-app is dus Actief zolang zijn commando draait, en een `web`-app zonder `port` gedraagt zich
op dezelfde manier. Een `static`-item met alleen een `url` heeft niets om te volgen en toont nooit Actief.

## web

Een lokale server. Stel `port` in zodat Actief weergeeft of de server antwoordt, en `url`
plus `openBrowser` om hem te openen zodra hij draait.

## desktop

Een native app. Stel `processName` in op de naam van het uitvoerbare bestand, zodat Actief behouden blijft wanneer het venster
zich losmaakt van het commando dat het startte. Het standaard Stoppen beëindigt elk proces met die
naam.

## static

Een pagina. Met alleen een `url` openen Starten en Herstarten die in je browser en doet Stoppen niets.
URL's met `http://`, `https://`, `mailto:` en `file://` worden geopend, dus een lokale pagina werkt:

```json title="apps.json"
{ "id": "csv", "name": "CSV dashboard", "group": "Docs", "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html" }
```

Pagina's die een server nodig hebben (PHP, of alles wat
lokale bestanden ophaalt) hebben een `command` nodig dat er een start en een `port` om die te volgen. Zie de
[voorbeelden](/nl/apps/examples/).

## cli

Een tool. `command` draait in een terminaltabblad in `cwd`, en de app is niet meer Actief zodra het
commando eindigt. Voor een shell die open blijft, maak je van het commando een shell, bijvoorbeeld
dit `command`:

```text title="command"
pwsh -NoLogo -NoProfile -NoExit -Command python run.py --flag
```

Vermijd geneste dubbele aanhalingstekens in `command`: de wrapper `cmd /c` maakt ze stuk.

![Het terminaltabblad van een cli-app met de uitvoer van een PowerShell-commando en een open prompt eronder](../../../../assets/screenshots/terminal-cli-output.png)

## Wat klikken doet

Klikken op de naam van een app opent alleen het terminaltabblad ervan. Gebruik de knoppen Starten, Stoppen en Herstarten
om hem uit te voeren. Zie [App-statussen](/nl/support/glossary/#app-statussen).
