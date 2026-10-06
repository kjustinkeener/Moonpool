---
title: "Die Seitenleiste lesen: Statuspunkte, Gruppen, Filter, Zeilenmenü"
description: "Was jede Zeile der Seitenleiste zeigt, wie Statuspunkte und Gruppen funktionieren, wie Sie Apps filtern, das Kontextmenü nutzen und die Seitenleiste in der Größe ändern."
---

Die Seitenleiste listet jede App aus `apps.json`, gruppiert nach dem Feld `group` der jeweiligen App. Siehe [App-Felder](/de/apps/fields/).

## Zeilen

Jede Zeile zeigt einen Statuspunkt, das App-Symbol (oder ein Typ-Symbol, wenn kein Symbol vorhanden ist), den Namen, den Port, falls gesetzt (`:3000`), und Bedienelemente.

| Punkt | Bedeutung |
| --- | --- |
| Gefüllt | läuft |
| Pulsierend | startet: Moonpool hat die App gestartet, sie wird aber noch nicht als aktiv erkannt |
| Grau | gestoppt |

Fahren Sie mit der Maus über den Punkt, um das Wort zu sehen.

![Die Seitenleiste mit zwei hervorgehobenen laufenden Web-Apps: leuchtende Punkte und Stoppen-Schaltflächen](../../../../assets/screenshots/sidebar-running-narrow.png)

1. Zwei laufende Apps. Ihre Punkte leuchten, und Stoppen (das Quadrat) ersetzt Starten.

| Bedienelement | Funktion |
| --- | --- |
| Stift | Die App bearbeiten. |
| Neu starten | Stoppen und erneut starten. Bei einer gestoppten App startet es sie einfach. |
| Starten (Play) | Startet die App und öffnet ihren Terminal-Tab. Wird angezeigt, wenn die App gestoppt ist. |
| Stoppen (Quadrat) | Stoppt die App. Wird angezeigt, während sie läuft oder startet. |

![Eine laufende Zeile, vergrößert: Statuspunkt, Typ-Symbol, Name und Port, dann die Schaltflächen Bearbeiten, Neu starten und Stoppen](../../../../assets/screenshots/sidebar-row-controls.png)

1. Statuspunkt (leuchtet, solange die App läuft).
2. Typ-Symbol.
3. Bearbeiten (Stift).
4. Neu starten.
5. Stoppen (wird statt Starten angezeigt, solange die App läuft).

Während ein Start oder Stopp läuft, werden die Bedienelemente durch einen Spinner ersetzt (`Arbeitet...`).

Ein Klick auf den **Namen** einer App öffnet oder fokussiert ihren Terminal-Tab und startet nie etwas. Der Tab einer gestoppten App zeigt das Protokoll dieser Sitzung. Verwenden Sie Starten oder Neu starten, um sie zu starten. Eine `static`-App mit nur einer `url` und ohne `command` hat kein Terminal: Starten öffnet die URL in Ihrem Browser.

### Tooltip

Fährt man über den Namen, erscheint die `note` der App, falls vorhanden, sonst ihr Name. Legen Sie `note` im Editor oder in `apps.json` fest.

### MCP-Unterzeile

Hat ein KI-Client die eigenen MCP-Tools einer App verwendet, erscheint unter der App eine abgedunkelte Unterzeile `MCP-Server`. Ihr Punkt leuchtet und der Tooltip lautet „MCP-Client verbunden“, solange der Client
verbunden ist. Eine Stoppen-Schaltfläche beendet diesen Prozess.

Die Zeile findet den Prozess anhand von `processName` plus dem Argument `mcp`, oder anhand des
Musters `mcpProcessName` der App, falls eines gesetzt ist. Siehe [Felder](/de/apps/fields/#mcpprocessname).

Blenden Sie diese Zeilen mit **MCP-Prozesse anzeigen** in den Einstellungen aus. Siehe
[MCP-Einrichtung](/de/automation/mcp-setup/#apps-mit-eigenem-mcp-server).

## Gruppen

![Untätige Seitenleiste mit den fünf hervorgehobenen Gruppenüberschriften, jeweils mit der Anzahl der Apps rechts](../../../../assets/screenshots/sidebar-groups-narrow.png)

- Klicken Sie auf eine Gruppenüberschrift, um sie ein- oder auszuklappen. Die Zahl daneben ist die Anzahl der angezeigten Apps. Eingeklappte Gruppen werden gemerkt.
- Innerhalb einer Gruppe steht die zuletzt gestartete App oben. Apps, die nie gestartet wurden, behalten die Reihenfolge aus `apps.json`. Eine frisch gestartete App leuchtet und rückt nach oben.

## Filterfeld

Geben Sie in **Apps filtern...** Text ein, um die Liste einzugrenzen. Verglichen werden App-Name und Gruppenname, ohne Beachtung der Groß- und Kleinschreibung. Passt nichts, zeigt die Liste:

```text
Keine App passt zu „<Text>“.
```

Die App setzt den Text in typografische Anführungszeichen, hier und in der Löschabfrage weiter unten.

## Rechtsklickmenü

Klicken Sie mit der rechten Maustaste auf eine Zeile für:

| Eintrag | Funktion |
| --- | --- |
| Bearbeiten | Öffnet den App-Editor. |
| Umbenennen | Verwandelt den Namen in ein Eingabefeld. **Enter** oder ein Klick daneben speichert, **Esc** bricht ab. Ein leerer oder unveränderter Name wird ignoriert. |
| Symbol wählen... | Wählen Sie eine Bilddatei (png, jpg, jpeg, gif, svg, webp, ico) als Symbol. |
| Löschen | Fragt `„<Name>“ löschen?` und entfernt den Eintrag aus `apps.json`. Führt Moonpool die App gerade aus, wird sie zuerst gestoppt. |

**Esc** schließt das Menü, ohne etwas auszuführen.

## Größe ändern

Ziehen Sie die Trennlinie zwischen Seitenleiste und CLI-Bereich. Die Breite ist auf 180 bis 620 px begrenzt (Standard 280) und wird gemerkt. Die Trennlinie ist gesperrt, solange der CLI-Bereich eingeklappt ist. Siehe [Terminal-Tabs](/de/using/terminal-tabs/).
