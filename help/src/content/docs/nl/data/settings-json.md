---
title: "settings.json begrijpen en een kapotte repareren"
description: "Zie de vorm van het settings.json van Moonpool, welke sleutels Moonpool voor jou schrijft en hoe je het bestand leest en repareert als het kapot is."
---

App-brede instellingen staan in `settings.json` in de configuratiemap (zie
[Waar de configuratie staat](/nl/apps/apps-json/#waar-de-configuratie-staat)). Wijzig ze in het
[venster Instellingen](/nl/using/settings/), dat elke instelling toont met zijn JSON-sleutel en
standaardwaarde. Logs en hun bewaring staan op de pagina [Logs](/nl/data/logs/).

## Vorm

Eén JSON-object. Sleutels die je weglaat nemen hun standaardwaarden aan:

```json title="settings.json"
{
  "closeToTray": true,
  "transparency": 20,
  "debugLogging": true,
  "cliLogging": true,
  "logRetentionMb": 25
}
```

| Sleutel | Standaard |
| --- | --- |
| `closeToTray` | `false` |
| `minimizeToTray` | `true` |
| `showInTray` | `true` |
| `showInTaskbar` | `true` |
| `alwaysOnTop` | `false` |
| `transparency` | `0` (0 tot 90) |
| `showStatusbar` | `true` |
| `showMcpProcesses` | `true` |
| `checkOnStartup` | `true` |
| `locale` | `"auto"` |
| `debugLogging` | `false` |
| `cliLogging` | `false` |
| `logRetentionMb` | `10` (minimaal 1) |

## Voor jou geschreven sleutels

Moonpool bewaart in dit bestand ook de UI-zoom (`uiScale`, 0,5 tot 3,0) en de bepaalde taal
(`localeResolved`). Je hoeft geen van beide in te stellen. Het thema staat hier niet: het wordt
bewaard in de opslag van de webview (zie [Thema's, taal en transparantie](/nl/using/themes-and-language/)).

## Lezen en repareren

Moonpool leest het bestand bij het opstarten. Wijzigingen terwijl het draait worden niet opgepikt; sluit eerst af.

Als het bestand misvormd is, start Moonpool met de standaardwaarden en weigert het instellingen te wijzigen.
De fout eindigt met `Repair settings.json and restart Moonpool before changing settings`.
Herstel het bestand, of verwijder het om alle instellingen terug te zetten, en start Moonpool dan opnieuw.
