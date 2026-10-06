---
title: "settings.json verstehen und eine fehlerhafte Datei reparieren"
description: "Aufbau der settings.json von Moonpool, welche Schlüssel Moonpool für Sie schreibt und wie Sie die Datei lesen und reparieren, wenn sie fehlerhaft ist."
---

Die app-weiten Einstellungen liegen in `settings.json` im Konfigurationsordner (siehe
[Wo die Konfiguration liegt](/de/apps/apps-json/#wo-die-konfiguration-liegt)). Ändern Sie sie im
[Einstellungsfenster](/de/using/settings/), das jede Einstellung mit ihrem JSON-Schlüssel und
Standardwert auflistet. Protokolle und ihre Aufbewahrung finden Sie auf der Seite [Protokolle](/de/data/logs/).

## Aufbau

Ein JSON-Objekt. Schlüssel, die Sie weglassen, übernehmen ihren Standardwert:

```json title="settings.json"
{
  "closeToTray": true,
  "transparency": 20,
  "debugLogging": true,
  "cliLogging": true,
  "logRetentionMb": 25
}
```

| Schlüssel | Standard |
| --- | --- |
| `closeToTray` | `false` |
| `minimizeToTray` | `true` |
| `showInTray` | `true` |
| `showInTaskbar` | `true` |
| `alwaysOnTop` | `false` |
| `transparency` | `0` (0 bis 90) |
| `showStatusbar` | `true` |
| `showMcpProcesses` | `true` |
| `checkOnStartup` | `true` |
| `locale` | `"auto"` |
| `debugLogging` | `false` |
| `cliLogging` | `false` |
| `logRetentionMb` | `10` (mindestens 1) |

## Für Sie geschriebene Schlüssel

Moonpool speichert in dieser Datei außerdem den UI-Zoom (`uiScale`, 0,5 bis 3,0) und die aufgelöste Sprache
(`localeResolved`). Sie müssen keinen der beiden setzen. Das Design steht nicht hier: Es wird im Speicher der
WebView abgelegt (siehe [Designs, Sprache und Transparenz](/de/using/themes-and-language/)).

## Lesen und Reparieren

Moonpool liest die Datei beim Start. Änderungen, die Sie während des Betriebs vornehmen, werden nicht übernommen; beenden Sie Moonpool vorher.

Ist die Datei fehlerhaft aufgebaut, startet Moonpool mit den Standardwerten und lehnt Änderungen an den Einstellungen ab.
Der Fehler endet mit `Repair settings.json and restart Moonpool before changing settings`.
Korrigieren Sie die Datei oder löschen Sie sie, um alle Einstellungen zurückzusetzen, und starten Sie Moonpool dann erneut.
