---
title: "De sessielogs en het debuglog van Moonpool vinden en beheren"
description: "Zoek het sessielog van elke app, het eigen debuglog van Moonpool, dumps en scrollback, en zie hoe lang elk wordt bewaard en hoe je het opent of kopieert."
---

Moonpool bewaart vier soorten uitvoer:

| Soort | Waar | Bewaard |
| --- | --- | --- |
| Sessielog | `cli-output\<id>\<session-start-ms>.log` in de configuratiemap | Deze sessie altijd; oudere sessies volgens de bewaarregels hieronder |
| `moonpool.log` | De configuratiemap | Alleen geschreven zolang **Debug-informatie naar een bestand loggen** aan staat |
| Dump | Waar je erom vraagt, of het eigen pad van het sessielog | Tot je het verwijdert |
| Scrollback | In het terminaltabblad | 10.000 regels, tot Moonpool afsluit |

De configuratiemap staat vermeld in [Waar de configuratie staat](/nl/apps/apps-json/#waar-de-configuratie-staat).

## Sessielogs

Alles wat een app in zijn terminal afdrukt, wordt ook naar een logbestand geschreven:

```text
<config folder>\cli-output\<id>\<session-start-ms>.log
```

- Eén bestand per app per Moonpool-sessie. Het getal is het moment waarop dat Moonpool-proces startte.
- Een app stoppen en opnieuw starten blijft aan hetzelfde bestand toevoegen. Een gedimde scheidingsregel markeert
  waar elke nieuwe run begint, en dezelfde markering verschijnt in het terminaltabblad:

  ```text title="1767225600000.log"
  Local:   http://localhost:5173/
  ---------- restarted 2026-10-05 09:14:02 ----------
  Local:   http://localhost:5173/
  ```

- Tekens in een `id` anders dan letters, cijfers, `-` en `_` worden `_` in de mapnaam.
  Dus `.` wordt `_`. Letters buiten het Engels blijven behouden.
- Het bestand bevat de ruwe terminaluitvoer, inclusief kleurcodes. Gebruik een dump voor platte tekst.

Het opnieuw openen van het tabblad van een app speelt het log van deze sessie opnieuw af, zodat je de eerdere uitvoer ziet.

## Bewaring

Bewaring betreft alleen logs van eerdere Moonpool-sessies. Het wordt uitgevoerd wanneer je een app start,
alleen voor de map van die app, oudste eerst.

| **Uitvoerlogs van apps tussen sessies bewaren** (`cliLogging`) | Wat er met de logs van eerdere sessies gebeurt |
| --- | --- |
| uit (standaard) | Verwijderd bij de volgende start van de app. |
| aan | Bewaard tot de totale grootte van de map **Logbewaring per app** (`logRetentionMb`, standaard 10 MB) overschrijdt, daarna worden de oudste verwijderd. |

Het bestand van de huidige sessie telt mee voor dat totaal, maar wordt nooit verwijderd of ingekort.
Eén heel groot huidig log kan dus alle oudere verdringen.

![Het gedeelte Logging van Instellingen: het selectievakje voor het bewaren van logs, de bewaargrootte per app in MB en het selectievakje voor het debuglog, elk met een rij met een mappad](../../../../assets/screenshots/settings-logging-section.png)

1. **Uitvoerlogs van apps tussen sessies bewaren** is `cliLogging`. **Logbewaring per app** eronder is `logRetentionMb`.

## moonpool.log

Als **Debug-informatie naar een bestand loggen** (`debugLogging`) aan staat, voegt Moonpool regels met tijdstempel toe aan
`moonpool.log` in de configuratiemap: het laden van `apps.json`, starts (met het commando en de map),
besturingscommando's en fouten. Zet het aan voordat je een probleem reproduceert.

## Openen en kopiëren

In [Instellingen](/nl/using/settings/#rechterkolom-logs), onder elke loggroep:

- **CLI-logmap openen** en **Logbestand openen** openen de map in je bestandsbeheer.
- **Pad van CLI-logmap kopiëren** en **Pad van logbestand kopiëren** zetten het pad op het klembord.

In een terminaltabblad kopieert **Alles kopiëren** de hele scrollback als tekst.

## Dumps

De verb `dump` geef je een sessielog vanuit een script:

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" dump my-app C:\temp\my-app.log
```

Met een uitvoerpad schrijft het een kopie in platte tekst, zonder kleurcodes. Zonder pad meldt het
het eigen pad van het sessielog. Een agent krijgt dezelfde tekst, al opgeschoond, van
`moonpool_app_output`. Zie [Opdrachtregel](/nl/automation/command-line/).
