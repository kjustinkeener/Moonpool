---
title: "Moonpool in het systeemvak houden: sluiten, minimaliseren en afsluiten"
description: "Bepaal wat het pictogram in het systeemvak, de sluitknop, minimaliseren en Afsluiten doen, houd het venster op de voorgrond en voorkom dat je systeemvak en taakbalk beide verbergt."
---

## Pictogram in het systeemvak

| Actie | Resultaat |
| --- | --- |
| Linksklik | Toont het hubvenster (herstelt het als het geminimaliseerd of verborgen is). |
| Rechtsklik | Menu met alleen **Moonpool tonen** en **Afsluiten** (in je taal). |

Als er meerdere Moonpool-kopieën draaien, heeft elke kopie een eigen pictogram in het systeemvak. De tooltip
vermeldt om welke kopie het gaat. Zie [Draagbare modus](/nl/data/portable-mode/#meerdere-kopieën-tegelijk).

## Afsluiten

**Afsluiten** sluit Moonpool af en stopt op Windows elke app die Moonpool heeft gestart, inclusief
hun onderliggende processen. Apps die al draaiden voordat Moonpool ze zag (getoond als actief
zonder "managed by Moonpool") blijven met rust gelaten. Op Linux en macOS stopt afsluiten gestarte
apps niet betrouwbaar.

## Sluiten en minimaliseren

De sluitknop sluit Moonpool standaard af (`closeToTray` is `false`). Zet **Sluiten naar systeemvak**
aan in Instellingen en sluiten verbergt het venster in plaats daarvan naar het systeemvak. Moonpool
blijft draaien en het pictogram in het systeemvak of **Moonpool tonen** brengt het terug.

**Minimaliseren naar systeemvak** (`minimizeToTray`, standaard aan) verbergt het venster naar het
systeemvak wanneer het wordt geminimaliseerd, en het verdwijnt uit de taakbalk. Zet het uit om
zoals gebruikelijk naar de taakbalk te minimaliseren.

![Instellingen: Sluiten naar systeemvak en Minimaliseren naar systeemvak (1), en de schuifregelaar Transparantie van de achtergrond (2)](../../../../assets/screenshots/settings-tray-and-transparency.png)

1. **Sluiten naar systeemvak** en **Minimaliseren naar systeemvak**.
2. **Transparantie van de achtergrond**. Zie [Thema's, taal en transparantie](/nl/using/themes-and-language/#transparantie).

## Vergrendeling van systeemvak en taakbalk

**Weergeven in systeemvak** en **Weergeven in taakbalk** bepalen of het pictogram in het systeemvak
en de knop in de taakbalk zichtbaar zijn. Minstens één moet aan blijven, anders zou een verborgen
venster geen weg terug hebben. Als er maar één aan staat, is het selectievakje ervan uitgeschakeld
totdat je de andere weer aanzet.

## Altijd op de voorgrond

**Altijd op voorgrond** in Instellingen houdt elk Moonpool-venster (de hub, Instellingen, Over, de
app-editor, de themabrowser, het installatieprogramma en Help) boven andere vensters. Standaard
staat het uit.

## Zie ook

- [Een npm dev-server op de achtergrond uitvoeren op Windows](/nl/guides/run-npm-dev-server-in-background-windows/)
- [Een script of dev-server automatisch starten bij het aanmelden bij Windows](/nl/guides/start-app-at-windows-login/)
- [Venster Instellingen](/nl/using/settings/)
- [Het hubvenster](/nl/using/hub-window/)
