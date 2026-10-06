---
title: "Moonpool-releasenotities en recente wijzigingen"
description: "Bekijk wat er is gewijzigd in recente Moonpool-releases, de vereisten om het te gebruiken en waar je de volledige releasenotities op GitHub vindt."
---

De volledige notities van elke release staan op de
[Releases-pagina](https://github.com/kjustinkeener/Moonpool/releases) van het project. Deze
help wordt met Moonpool meegeleverd en beschrijft dus altijd de versie die je gebruikt.
Moonpool werkt zichzelf bij; zie [Bijwerken](/nl/data/updating/).

## 0.3.16

- **Meerdere Moonpools tegelijk.** De geïnstalleerde Moonpool en een willekeurig aantal
  draagbare kopieën kunnen naast elkaar draaien, één per map, elk met eigen apps,
  systeemvakpictogram en besturingskanaal. Zie
  [Draagbare modus](/nl/data/portable-mode/#meerdere-kopieën-tegelijk).
- **Themabrowser.** 68 thema's, elk bekeken in de eigen kleuren. Zie
  [Thema's, taal en transparantie](/nl/using/themes-and-language/).
- **Uitvoerbare voorbeelden.** Een nieuwe `apps.json` bevat voorbeeld-apps die allemaal
  werken zoals ze zijn. De voorbeelddashboards staan nu in een map `dashboards/examples`
  die van de app is en met Moonpool wordt bijgewerkt. Zie
  [Voorbeelddashboards](/nl/getting-started/example-dashboards/).
- **Fouten in apps.json worden getoond.** Een banner boven de zijbalk toont de fout en een
  mislukte Opnieuw laden behoudt de laatst geladen lijst. Zie
  [Wanneer apps.json een fout bevat](/nl/using/hub-window/#wanneer-appsjson-een-fout-bevat).
- **Besturingskanaal op Linux en macOS**, via een Unix-socket, plus het werkwoord `list`. Zie
  [Besturingswerkwoorden](/nl/automation/control-verbs/).
- Over en de app-editor volgen wijzigingen van thema en taal live. Het menu-item
  **Moonpool installeren…** wordt buiten Windows verborgen.

## 0.3.15

- Een herstarte app behoudt de eerdere uitvoer, met een gedateerde scheidingslijn
  "restarted". Zie [Terminaltabbladen](/nl/using/terminal-tabs/#herstarten).
- Elke app heeft een eigen map `cli-output`, zodat het opschonen van logs nooit de logs van
  een andere app raakt.
- `killMode` en `stopCommand` staan in de app-editor. Zie
  [Stoppen en herstarten](/nl/apps/stop-and-restart/).
- Gestarte apps erven het eigen WebView2-profiel van Moonpool niet meer.

## 0.3.14

- Sessielogs kunnen tussen sessies worden bewaard, met een maximale grootte per app. Zie
  [Logs](/nl/data/logs/).
- Knoppen om de logmappen in Instellingen te openen en het pad te kopiëren.
- Verbeteringen aan de titelbalk van het Help-venster.

## Vereisten

- Windows 10 of 11 met WebView2 (zie [Windows](/nl/platforms/windows/)).
- Linux met WebKitGTK 4.1 en een AppIndicator-bibliotheek (zie [Linux](/nl/platforms/linux/)).
- macOS: zelf bouwen vanuit de broncode; nog niet gedistribueerd of getest.
