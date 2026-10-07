---
title: "Moonpool-Glossar: Apps, Zustände, Dateien und Einstellungen"
description: "Einfache Definitionen der Begriffe, die die Moonpool-Hilfe für Bestandteile, App-Zustände, Dateien und Einstellungen nutzt, damit Sie der Doku folgen können."
---

## Apps

| Begriff | Bedeutung |
| --- | --- |
| App | Etwas, das Moonpool verwaltet: ein Entwicklungsserver, eine Desktop-App, eine Seite oder ein Befehl. |
| Eintrag | Der Datensatz einer App in `apps.json`. Wird nur verwendet, wenn vom JSON die Rede ist. |
| App-Zeile | Die Zeile einer App in der Seitenleiste, mit Statuspunkt und Steuerelementen. |
| Gruppe | Die Überschrift in der Seitenleiste, unter der eine App steht, aus ihrem Feld `group`. |
| Typ | `web`, `desktop`, `static` oder `cli`. Bestimmt, welche Felder wichtig sind. Siehe [App-Typen](/de/apps/types/). |
| id | Der dauerhafte Schlüssel einer App, verwendet in Dateinamen, Befehlen und Agenten-Tools. Siehe [Die id](/de/apps/apps-json/#die-id). |

## App-Zustände

| Zustand | Bedeutung |
| --- | --- |
| startet | Moonpool hat die App gestartet, sie aber noch nicht laufen sehen. Pulsierender Punkt. |
| läuft | Ihr `port` antwortet, ihr `processName` existiert oder, wenn keines von beiden gesetzt ist, das von Moonpool gestartete Terminal lebt noch. Voller Punkt. Siehe [Wie „läuft“ bestimmt wird](/de/apps/types/#wie-läuft-festgestellt-wird). |
| gestoppt | Nichts davon trifft zu. Grauer Punkt. |
| verwaltet | Moonpool hat sie in dieser Sitzung gestartet. Eine laufende App, die nicht verwaltet ist, wurde anders gestartet, und Beenden lässt sie in Ruhe. |

Ein Terminal-Tab und eine laufende App sind getrennte Dinge. Ein Klick auf den Namen einer App öffnet nur ihren
Terminal-Tab; er startet die App nie. Das Schließen eines Tabs stoppt die App nie.

## Fenster und Bestandteile

| Begriff | Bedeutung |
| --- | --- |
| Hub | Der residente Moonpool-Prozess und sein Hauptfenster. Die Tool-Namen nennen ihn „Launcher“. |
| Hub-Fenster | Das Hauptfenster: links die Seitenleiste, rechts der CLI-Bereich. |
| Tray | Das Symbol im Infobereich und sein Menü (**Moonpool anzeigen**, **Beenden**). |
| Seitenleiste | Die linke Seite des Hub-Fensters: Filterfeld, Menü **...** und App-Zeilen. |
| CLI-Bereich | Die rechte Seite des Hub-Fensters, die die Terminal-Tabs enthält. |
| Terminal-Tab | Das Terminal einer App im CLI-Bereich. |
| MCP-Unterzeile | Eine abgedunkelte Zeile unter einer App, die deren eigenen `<exe> mcp`-Helferprozess zeigt. |
| App-Editor | Der Dialog „App hinzufügen“ und „App bearbeiten“. |

## Dateien und Ordner

| Begriff | Bedeutung |
| --- | --- |
| Konfigurationsordner | Der Ordner mit `apps.json` und den anderen Dateien von Moonpool. Das Token `{MP_DATA}`. Siehe [Wo die Konfiguration liegt](/de/apps/apps-json/#wo-die-konfiguration-liegt). |
| `{MP_HOME}` | Der Moonpool-Ordner: `%USERPROFILE%\.moonpool` bei der Installation, der `.moonpool\`-Ordner einer portablen Kopie, unter Linux der Konfigurationsordner. |
| Sitzung | Ein Lauf des Hubs, vom Start bis zum Beenden. |
| Sitzungsprotokoll | Die Datei mit allem, was eine App während einer Sitzung ausgegeben hat, unter `cli-output\`. Siehe [Protokolle](/de/data/logs/). |
| `moonpool.log` | Moonpools eigenes Debug-Protokoll, nur geschrieben, wenn **Debug-Infos in eine Datei schreiben** aktiv ist. |
| Dump | Eine Klartextkopie eines Sitzungsprotokolls, erstellt mit dem Verb `dump`. |
| Snapshot | Eine Kopie einer funktionierenden `apps.json` in `apps.json.history\`. Siehe [Sicherung und Wiederherstellung](/de/data/backup-and-recovery/). |

## Modi

| Begriff | Bedeutung |
| --- | --- |
| installiert | Ein Moonpool in `%USERPROFILE%\.moonpool`, mit Startmenü-Verknüpfung und Eintrag unter „Apps hinzufügen/entfernen“. Nur Windows. |
| portabel | Ein Moonpool in einem von Ihnen gewählten `.moonpool\`-Ordner, gekennzeichnet durch eine Datei `moonpool.portable`. Siehe [Portabler Modus](/de/data/portable-mode/). |
| Kopie | Ein Moonpool-Ordner, installiert oder portabel. Jede Kopie läuft für sich. |

## Stoppen und Automatisierung

| Begriff | Bedeutung |
| --- | --- |
| `killMode` | Der zusätzliche Schritt, den Stoppen nach dem Beenden des Terminals der App ausführt. Siehe [Stoppen und Neustarten](/de/apps/stop-and-restart/). |
| `stopCommand` | Der Befehl, den Stoppen ausführt, wenn `killMode` gleich `command` ist. |
| `processName` | Der Prozessname, auf den Moonpool achtet und den es im Modus `processName` beendet. |
| Steuerkanal | Die Named Pipe (Windows) oder der Unix-Socket (Linux), auf der der Hub antwortet. Siehe [Steuerverben](/de/automation/control-verbs/). |
| Verb | Ein Befehlswort wie `launch` oder `reload`, angegeben in der Befehlszeile oder über den Steuerkanal. |
| Ticket | Ein Schlüssel, den Sie mit `--ticket` anhängen, um das Ergebnis eines Befehls aus `state.json` zu lesen. |
| Token | Der Versionsstempel von `apps.json`, den ein Schreibvorgang auf die Konfiguration mitbringen muss. |
| MCP-Helfer (Shim) | Ein `<exe> mcp`-Prozess, den ein KI-Host startet, um die eigenen Tools einer App zu erreichen. |
