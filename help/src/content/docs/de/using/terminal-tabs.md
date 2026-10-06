---
title: "Moonpool-Terminal-Tabs nutzen: öffnen, schließen, kopieren, neu starten"
description: "Arbeiten mit den Terminal-Tabs pro App im Hub: Tabs öffnen und schließen, den Bereich einklappen, kopieren und einfügen, eine Sitzung neu starten und Sitzungsprotokolle finden."
---

Jede App läuft in einem eigenen Terminal-Tab im CLI-Bereich.

## Tabs

![Tab-Leiste mit aktivem Tab Metrics Dashboard (umrandet) und dessen Live-Protokoll darunter; jeder Tab hat einen Punkt und ein x](../../../../assets/screenshots/hub-terminal-tab.png)

- Das Starten einer App oder ein Klick auf ihren Namen in der Seitenleiste öffnet ihren Tab. Ein Klick auf einen Namen startet nichts; siehe [App-Zustände](/de/support/glossary/#app-zustände).
- Ein Punkt auf dem Tab leuchtet, solange die App läuft.
- Das **x** auf einem Tab schließt den Tab. Es stoppt die App nicht. Klicken Sie erneut auf den Namen, um den Tab wieder zu öffnen; er zeigt das Protokoll dieser Sitzung.

## Den Bereich einklappen

Das **x** ganz rechts in der Tab-Leiste („CLI-Bereich ausblenden“) klappt den CLI-Bereich ein und
verkleinert das Fenster auf die Seitenleiste. Die Terminals laufen weiter und behalten ihren Scrollback.

Neben dem Filterfeld erscheint ein Chevron, der den Bereich in seiner vorherigen Breite zurückbringt. Der
Chevron pulsiert, wenn ein Update bereitsteht, weil das Update-Banner in diesem Bereich liegt.

## Kopieren und einfügen

| Aktion | Ergebnis |
| --- | --- |
| Text mit der Maus markieren | Wird beim Loslassen in die Zwischenablage kopiert, danach wird die Markierung aufgehoben. |
| Mittelklick | Fügt die Zwischenablage in das Terminal ein. |
| Schaltfläche **Alles kopieren** (oben rechts, erscheint beim Darüberfahren) | Kopiert den gesamten Scrollback als Text. |

## Scrollback

Jedes Terminal behält 10.000 Zeilen.

## Wenn ein Prozess endet

Wenn der Prozess beendet wird, gibt das Terminal aus:

```text
[Prozess beendet]
```

Der Tab bleibt mit unveränderter Ausgabe geöffnet. Die Zeile `[Prozess beendet]` wird in Ihrer
Sprache angezeigt.

## Neu starten

**Neu starten** (oder Starten bei einer gestoppten App) beginnt einen neuen Lauf im selben Tab. Der Tab wird
neu aufgebaut, und die frühere Ausgabe dieser Sitzung wird aus dem Sitzungsprotokoll wieder eingespielt.

Lief die App in dieser Sitzung schon einmal, schreibt Moonpool zuerst eine abgedunkelte Trennlinie in das
Sitzungsprotokoll, sodass sie zwischen der alten Ausgabe und dem neuen Lauf erscheint:

```text
---------- restarted 2026-10-05 09:14:02 ----------
```

Beginnt der neue Lauf damit, den Bildschirm zu löschen, wird die frühere Ausgabe in den Scrollback geschoben,
statt gelöscht zu werden.

## Sitzungsprotokolle

Alles, was eine App ausgibt, wird zusätzlich in eine Protokolldatei unter `cli-output\` geschrieben, eine Datei pro App und Moonpool-Sitzung. Speicherort, Aufbewahrung und die Einstellung **App-Ausgabeprotokolle zwischen Sitzungen behalten** finden Sie unter [Protokolle](/de/data/logs/).
