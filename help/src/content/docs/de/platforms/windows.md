---
title: "Moonpool unter Windows verwenden"
description: "Windows ist die Hauptplattform von Moonpool: so installieren Sie es, plus eine Tabelle der Unterschiede zwischen Windows und Linux, damit Sie wissen, was Sie erwartet."
---

Windows ist die Hauptplattform von Moonpool. Installieren Sie es wie unter
[Installation](/de/getting-started/install/) beschrieben.

## Vor dem Start

- **SmartScreen.** `moonpool.exe` ist nicht codesigniert, daher zeigt Windows beim ersten Mal
  womöglich „Der Computer wurde durch Windows geschützt“ an. Wählen Sie **Weitere Informationen**
  und dann **Trotzdem ausführen**.
- **Virenschutz.** Eine neue, unsignierte EXE, die sich selbst kopiert und bei einem Update selbst
  ersetzt, kann einen Virenschutz auslösen. Wenn Ihrer `moonpool.exe` blockiert oder in Quarantäne
  verschiebt, erlauben Sie sie für den Ordner `.moonpool`.
- **WebView2.** Die Fenster von Moonpool nutzen Microsoft Edge WebView2, das mit Windows 11 und dem
  aktuellen Windows 10 ausgeliefert wird. Bleibt das Fenster leer oder öffnet sich nie, installieren
  Sie die Evergreen WebView2 Runtime von Microsoft.

Mehr dazu: [Windows hat Ihren PC geschützt](/de/support/windows-protected-your-pc/),
[WebView2-Runtime fehlt](/de/support/webview2-runtime-missing/) und
[Ein Skript oder einen Entwicklungsserver automatisch bei der Windows-Anmeldung starten](/de/guides/start-app-at-windows-login/).

## Infobereich

Unter Windows 11 landet ein neues Symbol im Infobereich oft im Bereich der ausgeblendeten Symbole.
Klicken Sie rechts in der Taskleiste auf den Pfeil **^**, um es zu finden, und ziehen Sie es auf
die Taskleiste, damit es sichtbar bleibt.

## Befehle

- Befehle laufen über `cmd /c`. Vermeiden Sie verschachtelte doppelte Anführungszeichen in
  `command`; `cmd /c` verstümmelt sie. Für ein Skript, das eine Shell offen lassen soll, nutzen Sie
  `pwsh -NoLogo -NoProfile -NoExit -Command <script and args>` ohne Anführungszeichen um den
  Skriptteil.
- `processName` passt mit oder ohne `.exe` und ohne Beachtung der Groß-/Kleinschreibung.
- Stoppen beendet den gesamten von Moonpool gestarteten Prozessbaum, einschließlich Prozessen, die
  sich davon gelöst haben.
- Docker-Desktop-Apps brauchen `killMode` `none` oder `command`, nie `port`. Siehe
  [Docker-Apps unter Windows](/de/apps/stop-and-restart/#docker-apps-unter-windows).

## Unterschiede je Plattform

| | Windows | Linux |
| --- | --- | --- |
| Installation | Selbstinstallierende `moonpool.exe` oder portabel | AppImage, `.deb` oder RPM; keine Installationskarte |
| Selbstaktualisierung | Ja, installiert und portabel | Nur AppImage |
| Konfigurationsordner | `%USERPROFILE%\.moonpool\moonpool-config\` | `~/.config/Moonpool/` |
| Shell für Befehle | `cmd /c` | `$SHELL -c` |
| `processName` | Beliebige Länge, `.exe` optional, Groß-/Kleinschreibung egal | 15 Zeichen oder weniger, exakte Schreibung |
| Stoppen über `processName` | Beendet den Prozess und seine Kindprozesse | Beendet nur Prozesse mit genau diesem Namen |
| Steuerkanal | Named Pipe | Unix-Socket |
| Fenster-Screenshots (zum Testen) | Ja | Nein |
| Symbole aus einer Programmdatei | Ja | Nein |
| Infobereich | Funktioniert sofort | Braucht AppIndicator; reines GNOME braucht eine Erweiterung |

Einzelheiten zu Linux stehen auf der Seite [Linux](/de/platforms/linux/).
