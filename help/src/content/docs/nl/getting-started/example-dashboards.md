---
title: "Probeer de voorbeelddashboards die met Moonpool worden meegeleverd"
description: "Open de meegeleverde offline voorbeelddashboards, zie waar ze staan en hoe voorbeeld-apps ernaar verwijzen, en voeg ze toe aan een bestaande configuratie."
---

Moonpool levert een aantal op zichzelf staande dashboards mee in het programma. Ze draaien
volledig offline, zonder server en zonder CDN.

| Dashboard | Wat het is |
| --- | --- |
| CSV explorer | Sleep een CSV- of TSV-bestand erheen; het profileert de kolommen en toont de gegevens. |
| JSON explorer | Sleep JSON erheen (arrays, geneste objecten of maps). |
| Excel explorer | Sleep een `.xlsx`- of `.xls`-bestand erheen, offline verwerkt. |
| Moonpool Docs | Een offline browser voor Markdown-documentatie. |

## Waar ze staan

Bij het opstarten schrijft Moonpool de dashboards naar `{MP_HOME}\dashboards\examples`:

| Modus | Map |
| --- | --- |
| Geïnstalleerd (Windows) | `%USERPROFILE%\.moonpool\dashboards\examples` |
| Draagbaar | `<your .moonpool folder, the one holding moonpool.exe>\dashboards\examples` |
| Linux | `~/.config/Moonpool/dashboards/examples` (of `$XDG_CONFIG_HOME/Moonpool/dashboards/examples`) |

De map `examples` is van Moonpool: ze wordt vervangen bij elke update van Moonpool, dus
wijzigingen daar gaan verloren. Wil je een dashboard aanpassen, kopieer dan de map ervan en
de gedeelde map `_lib` een niveau omhoog naar `dashboards` en laat je app naar de kopie
wijzen. Moonpool wijzigt nooit iets anders in `dashboards`.

Versies vóór 0.3.16 schreven de voorbeelden rechtstreeks naar `dashboards`. Die kopieën
blijven waar ze zijn en krijgen geen updates meer; apps die ernaar wijzen blijven werken.
Om de bijgewerkte versies te krijgen, wijzig je hun `url` naar het onderstaande pad
`dashboards/examples/...`.

## Hoe de apps ernaar verwijzen

Elk is een `static`-app waarvan de `url` een `file:///`-URL is, verankerd op `{MP_HOME}`:

```text
file:///{MP_HOME}/dashboards/examples/csv/index.html
```

`{MP_HOME}` verwijst naar de installatiemap of, in draagbare modus, naar de bundelmap, zodat
het item ook na het verplaatsen van de bundel blijft werken. `file://`-URL's zijn toegestaan.
Zie [Paden en omgeving](/nl/apps/paths-and-environment/).

## Voorbeeld-apps verschijnen alleen bij de eerste keer

De voorbeelditems worden alleen naar `apps.json` geschreven als er nog geen
configuratiebestand bestaat. Heb je al een `apps.json`, voeg de dashboarditems dan zelf toe
(**apps.json bewerken** in het menu "...", daarna **Opnieuw laden**). Voeg deze vier toe
binnen de array op het hoogste niveau, door komma's gescheiden van je andere items:

```jsonc title="apps.json (excerpt)"
{
  "id": "csv-explorer",
  "name": "Sample CSV Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html",
  "openBrowser": true
},
{
  "id": "json-explorer",
  "name": "Sample JSON Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/json/index.html",
  "openBrowser": true
},
{
  "id": "xlsx-explorer",
  "name": "Sample Excel Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/xlsx/index.html",
  "openBrowser": true
},
{
  "id": "docs-browser",
  "name": "Moonpool Docs",
  "group": "Docs",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/docs/index.html",
  "openBrowser": true
}
```

De betekenis van de velden staat in [App-velden](/nl/apps/fields/).

## Zie ook

- [Voorbeelden](/nl/apps/examples/): completere items om te kopiëren.
- [App-typen](/nl/apps/types/#static)
