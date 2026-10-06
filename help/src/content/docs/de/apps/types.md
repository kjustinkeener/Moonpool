---
title: "App-Typ wählen: web, desktop, static oder cli"
description: "So starten web-, desktop-, static- und cli-Apps in Moonpool, wie „läuft“ für jeden Typ erkannt wird und was die Schaltfläche Stoppen standardmäßig tut."
---

`type` bestimmt, welche Felder wichtig sind und was Stoppen standardmäßig tut.

| | `web` | `desktop` | `static` | `cli` |
| --- | --- | --- | --- | --- |
| Braucht | `command` | `command` | `url` | `command` |
| Meist zusätzlich | `port`, `url` | `processName` | `command` und `port`, wenn sie sich selbst ausliefert | `cwd` |
| Starten | Führt `command` in einem Terminal-Tab aus | Führt `command` in einem Terminal-Tab aus | Ohne `command`: öffnet `url` im Browser. Mit einem: führt ihn in einem Terminal-Tab aus | Führt `command` in einem Terminal-Tab aus |
| Standard-`killMode` | `port` | `processName` | `none` | `none` |

![Der Dialog „App bearbeiten“ für eine Web-App: type auf web gesetzt mit einer einzeiligen Beschreibung und ausgefülltem Feld port](../../../../assets/screenshots/edit-app-type-and-port.png)

1. Die Auswahl `type`. Ihre Hinweiszeile beschreibt, was dieser Typ tut.
2. Das Feld `port`. Bei einer `web`-App richtet sich „läuft“ danach, ob dieser Port antwortet.

## Wie „läuft“ festgestellt wird

Moonpool prüft alle paar Sekunden. Eine App läuft, wenn eine dieser Bedingungen zutrifft, unabhängig
von ihrem Typ:

- `processName` ist gesetzt und ein Prozess mit diesem Namen existiert. Die eigenen `<exe> mcp`-
  Helferprozesse von Moonpool zählen nicht.
- `port` ist gesetzt und antwortet auf localhost.
- Moonpool hat sie gestartet, sie hat weder `port` noch `processName`, und der Prozess des Terminals
  lebt noch.

Eine `cli`-App läuft also, solange ihr Befehl läuft, und eine `web`-App ohne `port` verhält sich
genauso. Ein `static`-Eintrag mit nur einer `url` hat nichts zu verfolgen und zeigt nie „läuft“.

## web

Ein lokaler Server. Setzen Sie `port`, damit „läuft“ widerspiegelt, ob der Server antwortet, und `url`
plus `openBrowser`, um ihn zu öffnen, sobald er hochkommt.

## desktop

Eine native App. Setzen Sie `processName` auf den Namen der ausführbaren Datei, damit „läuft“ erhalten bleibt, wenn sich das Fenster
von dem Befehl löst, der es gestartet hat. Das Standard-Stoppen beendet jeden Prozess mit diesem
Namen.

## static

Eine Seite. Mit nur einer `url` öffnen Starten und Neu starten sie in Ihrem Browser, und Stoppen tut nichts.
URLs mit `http://`, `https://`, `mailto:` und `file://` werden geöffnet, sodass auch eine lokale Seite funktioniert:

```json title="apps.json"
{ "id": "csv", "name": "CSV dashboard", "group": "Docs", "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html" }
```

Seiten, die einen Server brauchen (PHP oder alles, was
lokale Dateien abruft), brauchen einen `command`, der einen startet, und einen `port`, um ihn zu verfolgen. Siehe die
[Beispiele](/de/apps/examples/).

## cli

Ein Tool. `command` läuft in einem Terminal-Tab in `cwd`, und die App läuft nicht mehr, sobald der
Befehl endet. Damit eine Shell offen bleibt, machen Sie den Befehl zu einer Shell, zum Beispiel
mit diesem `command`:

```text title="command"
pwsh -NoLogo -NoProfile -NoExit -Command python run.py --flag
```

Vermeiden Sie verschachtelte doppelte Anführungszeichen in `command`: Der `cmd /c`-Wrapper verstümmelt sie.

![Der Terminal-Tab einer cli-App mit der Ausgabe eines PowerShell-Befehls und einer offenen Eingabeaufforderung darunter](../../../../assets/screenshots/terminal-cli-output.png)

## Was ein Klick bewirkt

Ein Klick auf den Namen einer App öffnet nur ihren Terminal-Tab. Mit den Bedienelementen Starten, Stoppen und Neu starten
führen Sie sie aus. Siehe [App-Status](/de/support/glossary/#app-zustände).
