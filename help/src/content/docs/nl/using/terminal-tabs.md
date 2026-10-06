---
title: "Moonpool-terminaltabbladen gebruiken: openen, sluiten, kopiëren, herstarten"
description: "Werk met de terminaltabbladen per app in de hub: tabbladen openen en sluiten, het paneel samenklappen, kopiëren en plakken, een sessie herstarten en sessielogs vinden."
---

Elke app draait in een eigen terminaltabblad in het CLI-paneel.

## Tabbladen

![Tabbladenbalk met Metrics Dashboard actief (omlijnd) en het live log eronder; elk tabblad heeft een stip en een x](../../../../assets/screenshots/hub-terminal-tab.png)

- Een app starten, of op de naam ervan in de zijbalk klikken, opent het tabblad. Klikken op een naam start niets; zie [App-statussen](/nl/support/glossary/#app-statussen).
- Een stip op het tabblad is verlicht terwijl de app actief is.
- De **x** op een tabblad sluit het tabblad. Het stopt de app niet. Klik opnieuw op de naam om het tabblad te heropenen; het toont het log van deze sessie.

## Het paneel samenklappen

De **x** uiterst rechts van de tabbladenbalk ("CLI-paneel verbergen") klapt het CLI-paneel samen en
verkleint het venster tot alleen de zijbalk. Terminals blijven draaien en behouden hun scrollback.

Naast het filtervak verschijnt een pijltje om het paneel terug te brengen op de vorige breedte. Het
pijltje pulseert als er een update wacht, omdat de updatebanner in het paneel staat.

## Kopiëren en plakken

| Actie | Resultaat |
| --- | --- |
| Tekst selecteren met de muis | Bij het loslaten gekopieerd naar het klembord, daarna wordt de selectie gewist. |
| Middelste klik | Plakt het klembord in de terminal. |
| Knop **Alles kopiëren** (rechtsboven, verschijnt bij aanwijzen) | Kopieert de hele scrollback als tekst. |

## Scrollback

Elke terminal bewaart 10.000 regels.

## Wanneer een proces eindigt

Wanneer het proces eindigt, toont de terminal:

```text
[proces beëindigd]
```

Het tabblad blijft open met de uitvoer intact. De regel `[proces beëindigd]` wordt in je taal getoond.

## Herstarten

**Herstarten** (of Starten bij een gestopte app) begint een nieuwe run in hetzelfde tabblad. Het
tabblad wordt opnieuw opgebouwd en de eerdere uitvoer van deze sessie wordt eruit afgespeeld
vanuit het sessielog.

Als de app eerder in deze sessie al draaide, schrijft Moonpool eerst een gedimde scheidingslijn
naar het sessielog, zodat die tussen de oude uitvoer en de nieuwe run staat:

```text
---------- restarted 2026-10-05 09:14:02 ----------
```

Als de nieuwe run begint met het wissen van het scherm, wordt de eerdere uitvoer naar de scrollback
geduwd in plaats van gewist.

## Sessielogs

Alles wat een app uitvoert, wordt ook weggeschreven naar een logbestand onder `cli-output\`, één bestand per app per Moonpool-sessie. Locatie, bewaring en de instelling **Uitvoerlogs van apps tussen sessies bewaren** staan in [Logs](/nl/data/logs/).
