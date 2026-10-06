---
title: "Sitzungsprotokolle und Debug-Protokoll von Moonpool finden und verwalten"
description: "Finden Sie das Sitzungsprotokoll jeder App, das Debug-Protokoll von Moonpool, Dumps und Scrollback und erfahren Sie, wie lange jedes aufbewahrt und wie es geöffnet oder kopiert wird."
---

Moonpool hält vier Arten von Ausgabe vor:

| Art | Wo | Aufbewahrung |
| --- | --- | --- |
| Sitzungsprotokoll | `cli-output\<id>\<session-start-ms>.log` im Konfigurationsordner | Die aktuelle Sitzung immer; ältere Sitzungen nach den Aufbewahrungsregeln weiter unten |
| `moonpool.log` | Der Konfigurationsordner | Wird nur geschrieben, solange **Debug-Infos in eine Datei schreiben** an ist |
| Dump | Wo Sie es angeben, oder der eigene Pfad des Sitzungsprotokolls | Bis Sie ihn löschen |
| Scrollback | Im Terminal-Tab | 10.000 Zeilen, bis Moonpool beendet wird |

Der Konfigurationsordner ist unter [Wo die Konfiguration liegt](/de/apps/apps-json/#wo-die-konfiguration-liegt) aufgeführt.

## Sitzungsprotokolle

Alles, was eine App in ihrem Terminal ausgibt, wird zusätzlich in eine Protokolldatei geschrieben:

```text
<config folder>\cli-output\<id>\<session-start-ms>.log
```

- Eine Datei pro App und Moonpool-Sitzung. Die Zahl ist der Zeitpunkt, zu dem dieser Moonpool-Prozess gestartet wurde.
- Stoppen und erneutes Starten einer App hängt weiter an dieselbe Datei an. Eine abgedunkelte Trennzeile markiert,
  wo jeder neue Lauf beginnt, und dieselbe Markierung erscheint im Terminal-Tab:

  ```text title="1767225600000.log"
  Local:   http://localhost:5173/
  ---------- restarted 2026-10-05 09:14:02 ----------
  Local:   http://localhost:5173/
  ```

- Zeichen in einer `id`, die keine Buchstaben, Ziffern, `-` oder `_` sind, werden im Ordnernamen zu `_`.
  So wird `.` zu `_`. Buchstaben außerhalb des Englischen bleiben erhalten.
- Die Datei enthält die rohe Terminal-Ausgabe, einschließlich Farbcodes. Für reinen Text verwenden Sie einen Dump.

Beim erneuten Öffnen des Tabs einer App wird das Protokoll dieser Sitzung wieder eingespielt, sodass Sie ihre frühere Ausgabe sehen.

## Aufbewahrung

Die Aufbewahrung betrifft nur Protokolle früherer Moonpool-Sitzungen. Sie läuft, wenn Sie eine App starten,
nur für den Ordner dieser App, die ältesten zuerst.

| **App-Ausgabeprotokolle zwischen Sitzungen behalten** (`cliLogging`) | Was mit den Protokollen früherer Sitzungen geschieht |
| --- | --- |
| aus (Standard) | Werden beim nächsten Start der App gelöscht. |
| an | Bleiben erhalten, bis die Gesamtgröße des Ordners **Protokollaufbewahrung pro App** (`logRetentionMb`, Standard 10 MB) übersteigt, dann werden die ältesten gelöscht. |

Die Datei der aktuellen Sitzung zählt zu dieser Summe, wird aber nie gelöscht oder gekürzt.
Ein einzelnes sehr großes aktuelles Protokoll kann also alle älteren verdrängen.

![Der Abschnitt Protokollierung der Einstellungen: das Kontrollkästchen zum Behalten der Protokolle, die Aufbewahrungsgröße pro App in MB und das Kontrollkästchen für das Debug-Protokoll, jeweils mit einer Pfadzeile](../../../../assets/screenshots/settings-logging-section.png)

1. **App-Ausgabeprotokolle zwischen Sitzungen behalten** ist `cliLogging`. **Protokollaufbewahrung pro App** darunter ist `logRetentionMb`.

## moonpool.log

Ist **Debug-Infos in eine Datei schreiben** (`debugLogging`) an, hängt Moonpool Zeilen mit Zeitstempel an
`moonpool.log` im Konfigurationsordner an: Ladevorgänge von `apps.json`, Starts (mit Befehl und Ordner),
Steuerbefehle und Fehler. Schalten Sie es ein, bevor Sie ein Problem nachstellen.

## Öffnen und Kopieren

In den [Einstellungen](/de/using/settings/#rechte-spalte-protokolle), unter jeder Protokollgruppe:

- **CLI-Protokollordner öffnen** und **Protokoll öffnen** öffnen den Ordner in Ihrem Dateimanager.
- **Pfad des CLI-Protokollordners kopieren** und **Pfad der Protokolldatei kopieren** legen den Pfad in die Zwischenablage.

In einem Terminal-Tab kopiert **Alles kopieren** den gesamten Scrollback als Text.

## Dumps

Das Verb `dump` liefert Ihnen ein Sitzungsprotokoll aus einem Skript:

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" dump my-app C:\temp\my-app.log
```

Mit einem Ausgabepfad schreibt es eine Klartextkopie ohne Farbcodes. Ohne Pfad meldet es
den eigenen Pfad des Sitzungsprotokolls. Ein Agent erhält denselben, bereits bereinigten Text von
`moonpool_app_output`. Siehe [Befehlszeile](/de/automation/command-line/).
