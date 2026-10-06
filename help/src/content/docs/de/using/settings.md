---
title: "Moonpool-Einstellungen ändern: jede Option in Einstellungen und Über"
description: "Die vollständige Liste der Bedienelemente in den Moonpool-Fenstern Einstellungen und Über, der jeweils geschriebene Schlüssel in settings.json und das Zurücksetzen einer Einstellung."
---

Öffnen Sie **Einstellungen** über das Menü „...“ des Hubs. Änderungen werden sofort gespeichert. Escape schließt
das Fenster. Diese Seite ist die vollständige Liste der Einstellungen. Jede wird in `settings.json`
unter dem angegebenen Schlüssel gespeichert; die Datei selbst wird unter
[settings.json](/de/data/settings-json/) beschrieben.

![Einstellungsfenster: Schalter und Regler in der linken Spalte, Protokolloptionen in der rechten](../../../../assets/screenshots/settings-window.png)

## Ein Bedienelement zurücksetzen

Klicken Sie mit der rechten Maustaste auf ein beliebiges Kontrollkästchen, einen Regler oder ein Zahlenfeld, um nur diese Einstellung auf
den Standard zurückzusetzen. Der Tooltip an jedem Bedienelement weist darauf hin. Die Auswahl für Sprache und Design haben
kein Zurücksetzen.

## Linke Spalte

| Bedienelement | Schlüssel | Standard | Funktion |
| --- | --- | --- | --- |
| Sprache | `locale` | Automatisch (System) | Sprache der eigenen Texte von Moonpool. Wirkt sofort. Siehe [Designs, Sprache und Transparenz](/de/using/themes-and-language/). |
| Design | keiner (Browser-Speicher) | Automatisch (System) | Farbdesign. Die Schaltfläche öffnet einen Design-Browser mit Vorschau jedes Designs; ein Klick wendet es sofort an. Siehe [Designs, Sprache und Transparenz](/de/using/themes-and-language/). |
| Beim Schließen in den Infobereich | `closeToTray` | aus | An: Das Schließen des Fensters blendet Moonpool in den Tray aus. Aus: Schließen beendet die App. |
| Beim Minimieren in den Infobereich | `minimizeToTray` | an | An: Das Minimieren blendet Moonpool in den Tray aus, und es verschwindet aus der Taskleiste. Aus: minimiert in die Taskleiste. |
| Immer im Vordergrund | `alwaysOnTop` | aus | Hält jedes Moonpool-Fenster über anderen Fenstern. |
| Im Infobereich anzeigen | `showInTray` | an | Lässt das Tray-Symbol sichtbar. |
| In der Taskleiste anzeigen | `showInTaskbar` | an | Lässt die Taskleisten-Schaltfläche sichtbar. |
| CPU-/Speicher-Statusleiste anzeigen | `showStatusbar` | an | Live-Leiste für CPU und Arbeitsspeicher am unteren Rand des Hubs. |
| MCP-Prozesse anzeigen | `showMcpProcesses` | an | Zeigt den MCP-Prozess einer App als MCP-Unterzeile in der Seitenleiste, solange ihre MCP-Tools genutzt werden. |
| Hintergrundtransparenz | `transparency` | 0 % | Regler von 0 bis 90 in Schritten von 5. Siehe [Designs, Sprache und Transparenz](/de/using/themes-and-language/#transparenz). |
| Beim Start nach Updates suchen | `checkOnStartup` | an | Prüft beim Start auf GitHub, ob es eine neuere Version gibt, und zeigt bei Erfolg ein Banner. Siehe [Aktualisieren](/de/data/updating/). |

### Sperre von Tray und Taskleiste

Mindestens eines von **Im Infobereich anzeigen** und **In der Taskleiste anzeigen** muss aktiv bleiben, sonst gäbe es für ein ausgeblendetes
Fenster keinen Weg zurück. Ist nur eines aktiv, ist sein Kontrollkästchen deaktiviert, bis Sie
das andere wieder einschalten.

## Rechte Spalte: Protokolle

| Bedienelement | Schlüssel | Standard | Funktion |
| --- | --- | --- | --- |
| App-Ausgabeprotokolle zwischen Sitzungen behalten | `cliLogging` | aus | Die Terminal-Ausgabe der laufenden Sitzung bleibt immer für ihre eigenen Tabs erhalten. An: Protokolle älterer Sitzungen bleiben unter `cli-output\` auf dem Datenträger, begrenzt durch die Aufbewahrungseinstellung. Aus: Sie werden beim nächsten Start dieser App gelöscht. |
| Protokollaufbewahrung pro App | `logRetentionMb` | 10 MB | Obergrenze für die gesamten Protokolle jeder App. Minimum 1. Deaktiviert, solange der Schalter darüber aus ist. Das Protokoll der aktuellen Sitzung zählt zur Obergrenze, wird davon aber nie gekürzt oder gelöscht. |
| Debug-Infos in eine Datei schreiben | `debugLogging` | aus | Protokolliert Ladevorgänge von `apps.json`, Starts und Fehler in `moonpool.log`. |

Unter jeder Protokollgruppe zeigt ein Pfadfeld den Speicherort, mit zwei Schaltflächen:

- **Öffnen** öffnet den Ordner im Dateimanager (**CLI-Protokollordner öffnen** für `cli-output\`, **Protokoll öffnen** für `moonpool.log`).
- **Kopieren** legt den Pfad in die Zwischenablage (**Pfad des CLI-Protokollordners kopieren**, **Pfad der Protokolldatei kopieren**).

Formate der Protokolldateien, die Neustart-Trennlinie und die Aufbewahrungsregeln finden Sie unter
[Protokolle](/de/data/logs/).

Lässt sich ein Kontrollkästchen nicht speichern, weist eine rote Meldung oben im Fenster darauf hin, und das
Kontrollkästchen springt zurück.

## Fenster „Über“

Öffnen Sie **Über** über das Menü „...“.

![Fenster Über mit Versionszeile, Links sowie den Schaltflächen Nach Updates suchen und Schließen](../../../../assets/screenshots/about-window.png)

Es zeigt:

- Die Version und das Build-Datum.
- Links zur Projektseite, zum GitHub-Repository und zur Kontaktadresse.
- **Nach Updates suchen**. Gibt es eine neuere Version, wird sie heruntergeladen, geprüft und installiert, danach startet Moonpool neu. Andernfalls meldet es, dass Sie die neueste Version haben, oder den Fehler, falls die Suche fehlgeschlagen ist.
- Danksagungen für die Bibliotheken, mit denen Moonpool gebaut ist, und den Autor.

Escape schließt das Fenster. Über übernimmt Design, Transparenz und Sprache live aus den Einstellungen.
