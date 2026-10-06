---
title: "Das Moonpool-Hub-Fenster im Überblick"
description: "Eine Tour durch das Moonpool-Hub: Seitenleiste, Terminal-Tabs, Statusleiste, das Menü, das Fehlerbanner für apps.json und das Merken von Größe und Position."
---

![Das Hub mit drei laufenden Apps: zwei Zeilen laufender Web-Apps (1), die Tab-Leiste (2), die Live-Ausgabe der aktiven App (3) und die Statusleiste (4)](../../../../assets/screenshots/hub-window.png)

1. Zwei der laufenden Apps: ein leuchtender Statuspunkt und eine Stopp-Schaltfläche statt Play.
2. Die Tab-Leiste, ein Tab pro geöffneter App, der aktive Tab ist hervorgehoben.
3. Live-Ausgabe der aktiven App.
4. Die Statusleiste für CPU und Arbeitsspeicher.

## Layout

| Bereich | Was er enthält |
| --- | --- |
| Titelleiste | Minimieren, Maximieren und Schließen. |
| Seitenleiste | Das Filterfeld, das Menü **...** und Ihre Apps, gruppiert nach `group`. Siehe [Seitenleiste und Menüs](/de/using/sidebar-and-menus/). |
| CLI-Bereich | Ein Terminal-Tab pro geöffneter App. Siehe [Terminal-Tabs](/de/using/terminal-tabs/). |
| Statusleiste | Live-Werte für CPU und Arbeitsspeicher, am unteren Rand. |

Ziehen Sie die Trennlinie zwischen Seitenleiste und CLI-Bereich, um die Seitenleiste in der Größe zu ändern.

## Statusleiste

![Die Statusleiste: CPU-Balken pro Kern links, der Speicherbalken rechts](../../../../assets/screenshots/status-bar.png)

Die Statusleiste zeigt einen schmalen Balken pro CPU-Kern (beim Darüberfahren erscheint „CPU-Auslastung pro Kern“), dann einen
Speicherbalken mit der Beschriftung `used/total GB`. Schalten Sie sie in den Einstellungen mit **CPU-/Speicher-Statusleiste anzeigen** aus (`showStatusbar`; siehe [Einstellungsfenster](/de/using/settings/)). Die Änderung wirkt sofort.

## Das Menü ...

Die Schaltfläche **...** links neben dem Filterfeld öffnet das Menü.

![Die Menüschaltfläche ... (1) und das Feld Apps filtern (2) oben in der Seitenleiste](../../../../assets/screenshots/sidebar-filter-and-menu.png)

1. Die Menüschaltfläche **...**.
2. Das Feld **Apps filtern...**.

| Eintrag | Funktion |
| --- | --- |
| App hinzufügen | Öffnet den App-Editor. Siehe [Apps hinzufügen](/de/apps/add-an-app/). |
| apps.json bearbeiten | Öffnet `apps.json` zum manuellen Bearbeiten in Ihrem Standardeditor. |
| Neu laden | Liest `apps.json` erneut von der Festplatte ein (auch F5, siehe [Tastenkürzel und Zoom](/de/using/keyboard-shortcuts/)). |
| Einstellungen | Öffnet das Einstellungsfenster. |
| Hilfe | Öffnet diese Hilfe. |
| Über | Öffnet das Fenster „Über“ mit der Version und der Update-Suche. |
| Moonpool installieren… | Nur unter Windows. Öffnet das Installationsfenster, um die App zu installieren oder eine portable Kopie anzulegen. Siehe [Installation](/de/getting-started/install/) und [Portabler Modus](/de/data/portable-mode/). |

### Warnung bei Port-Konflikt

Verwenden zwei Apps in `apps.json` denselben `port`, erscheint am unteren Rand des Menüs eine Warnzeile, zum Beispiel:

```text
port 3000: App A / App B
```

Fahren Sie mit der Maus darüber, um den vollständigen Satz zu lesen. Beheben Sie den Konflikt in `apps.json` oder im App-Editor; die Zeile verschwindet, sobald kein Port mehr doppelt belegt ist.

## Wenn apps.json einen Fehler enthält

Stellt Neu laden (oder F5) fest, dass sich `apps.json` nicht mehr parsen oder validieren lässt, behält Moonpool die
bereits geladene Liste. Ein Banner oben in der Seitenleiste meldet „apps.json enthält einen Fehler, angezeigt wird die zuletzt geladene Liste.“,
gefolgt vom Fehler (fahren Sie mit der Maus darüber, um den vollständigen Text zu sehen). Die
Liste darunter ist abgedunkelt, funktioniert aber weiterhin, sodass Sie Apps wie gewohnt starten und stoppen können. **apps.json bearbeiten** im Banner öffnet die Datei; korrigieren Sie sie und wählen Sie **Neu laden**, dann
verschwindet das Banner.

Solange die Datei nicht wieder geladen werden kann, speichert Moonpool keine Änderungen aus dem App-Editor, durch Umbenennen,
Löschen oder Symbol wählen, damit eine fehlerhafte Datei nie überschrieben wird.

Ist die Datei bereits beim Start von Moonpool fehlerhaft, gibt es keine frühere Liste, die erhalten bleiben könnte: Das
Banner meldet, dass keine Apps geladen sind, und die Seitenleiste ist leer. Korrigieren Sie die Datei und laden Sie neu, oder kehren Sie zu einer
aktuellen, funktionierenden Kopie zurück (siehe [Wenn die Datei fehlerhaft ist](/de/apps/apps-json/#wenn-die-datei-fehlerhaft-ist)).

## Leerer Bildschirm

Ist kein Tab geöffnet, zeigt der CLI-Bereich „Wählen Sie links eine App aus, um sie zu starten.“ Außerdem enthält er
zwei Elemente, die nur erscheinen, solange kein Tab geöffnet ist:

- **Das Update-Banner**, wenn beim Start eine neuere Version gefunden wurde. Siehe
  [Aktualisieren](/de/data/updating/).
- **Prompt kopieren**, ein fertiger Prompt, mit dem Sie das Einrichten Ihrer Apps einem KI-Agenten übergeben. Siehe
  [KI-Agenten: Schnellstart](/de/automation/quick-start/#prompt-kopieren).

Das Tray-Symbol, Schließen, Minimieren, Beenden und Immer im Vordergrund finden Sie unter
[Tray, Schließen und Minimieren](/de/using/tray-and-closing/).

## Größe, Position und maximierter Zustand

Moonpool merkt sich Größe, Position und maximierten Zustand des Hub-Fensters zwischen den Starts. Beim
ersten Start öffnet sich das Fenster mit 1200x780 an der Position, die Windows wählt.

Liegt die gespeicherte Position auf keinem angeschlossenen Bildschirm mehr (zum Beispiel wegen eines abgesteckten
Monitors), wird die Position ignoriert und die gespeicherte Größe an der Standardposition verwendet. Die
Datei heißt `window-state.json` und liegt im Konfigurationsordner (siehe
[Wo die Konfiguration liegt](/de/apps/apps-json/#wo-die-konfiguration-liegt)).

Auch die Breite der Seitenleiste und ob der CLI-Bereich eingeklappt ist, werden gespeichert.
