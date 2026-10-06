---
title: "apps.json bearbeiten: Speicherort, Neu laden und Wiederherstellung"
description: "Finden Sie die apps.json, die Moonpool für alle Apps liest, bearbeiten Sie sie im App-Editor oder von Hand, laden Sie sie neu und stellen Sie sie wieder her."
---

Jede App, die Moonpool verwaltet, ist ein Eintrag in `apps.json`. Sie können die Datei im App-Editor
(dem Dialog „App hinzufügen“ und „App bearbeiten“) oder von Hand bearbeiten. Beide schreiben dieselbe Datei.
In manchen Tool-Ergebnissen und Meldungen heißt diese Datei Manifest.

## Wo die Konfiguration liegt

| Modus | Konfigurationsordner |
| --- | --- |
| Installiert (Windows) | `%USERPROFILE%\.moonpool\moonpool-config\` |
| Portabel | `moonpool-config\` neben `moonpool.exe` (im Ordner `.moonpool\`) |
| Linux | `$XDG_CONFIG_HOME/Moonpool/`, sonst `~/.config/Moonpool/` |

`apps.json` liegt in diesem Ordner, neben diesen Elementen:

| Element | Zweck |
| --- | --- |
| `apps.json.history\` | Rollback-Ring der letzten 10 gültigen `apps.json`-Dateien. |
| `settings.json` | App-Einstellungen. Siehe [settings.json](/de/data/settings-json/). |
| `cli-output\<id>\` | Sitzungsprotokolle pro App. Siehe [Logs](/de/data/logs/). |
| `moonpool.log` | Debug-Protokoll, solange **Debug-Infos in eine Datei schreiben** aktiv ist. |
| `icons\` | Optionale Symbol-Überschreibungen `<id>.png` (auch `.ico`, `.svg`, `.jpg`, `.jpeg`, `.webp`). |
| `state.json` | Live-Statusmomentaufnahme, die alle paar Sekunden aktualisiert wird. |
| `dumps\` | Dateien, die von den Verben `dump`, `read-config` und `restore-config` geschrieben werden. |
| `mcp_seen.json` | Welche Apps schon einen MCP-Helfer hatten. |
| `window-state.json` | Größe und Position des Hub-Fensters. |
| `AI-README.md` | Die Anleitung für KI-Agenten, bei jedem Start neu geschrieben. |

Welche davon Sie sichern sollten, steht unter [Sicherung und Wiederherstellung](/de/data/backup-and-recovery/#der-konfigurationsordner).

Beim ersten Start legt Moonpool in `apps.json` Beispieleinträge an. Eine bereits vorhandene Datei
wird nie überschrieben.

## Bearbeiten

- **Dialog.** Wählen Sie **App hinzufügen** im Menü **...** oben in der Seitenleiste. Um eine
  App zu ändern, nutzen Sie den Stift in ihrer Zeile oder klicken Sie sie mit der rechten Maustaste an und wählen **Bearbeiten**. Der Dialog
  prüft und speichert sofort.
- **Von Hand.** **apps.json bearbeiten** im selben Menü öffnet die Datei in Ihrem Standardeditor.
  Speichern Sie sie und wählen Sie dann **Neu laden** im Menü (oder drücken Sie F5 oder Strg+R).

Handänderungen werden erst nach dem Neuladen übernommen. „Neu laden“ liest die Datei nur, es
schreibt sie nicht neu.

Das Speichern aus dem Dialog schreibt die ganze Datei in normalisierter, eingerückter Form neu. Schlüssel, die Moonpool
nicht kennt, gehen verloren, und JSON kennt keine Kommentare, also halten Sie Notizen im Feld `note` fest.

## Aufbau

Die Datei ist ein JSON-Array aus Objekten. Vier Schlüssel sind in jedem Eintrag Pflicht: `id`, `name`,
`group`, `type`. Alles andere ist optional. Siehe [App-Felder](/de/apps/fields/).

```json title="apps.json"
[
  { "id": "site", "name": "Site", "group": "Web apps", "type": "web",
    "cwd": "C:\code\site", "command": "npm run dev", "port": 5173,
    "url": "http://localhost:5173", "openBrowser": true }
]
```

Gruppen erscheinen in der Seitenleiste in der Reihenfolge, in der sie zuerst in der Datei vorkommen.

## Was „Neu laden“ bewirkt

„Neu laden“ ersetzt die Liste im Arbeitsspeicher von Moonpool durch den Inhalt der Datei. Starten, Stoppen und Neu starten
lesen den Eintrag beim Klick, daher gilt ein geänderter `command`, `cwd`, `env` oder eine Beenden-Einstellung
beim nächsten Start oder Neustart dieser App. „Neu laden“ startet nie etwas neu: Eine
App, die bereits läuft, läuft mit den Einstellungen weiter, mit denen sie gestartet wurde.

## Prüfung

Moonpool prüft die ganze Datei beim Laden, bei jedem Speichern und bei jedem Schreibzugriff eines Agenten.
Ein einziger fehlerhafter Eintrag lässt die ganze Datei durchfallen.

| Regel | Fehler enthält |
| --- | --- |
| Kein gültiges JSON, ein Pflichtschlüssel fehlt oder ein Wert hat den falschen Typ | die Meldung des JSON-Parsers |
| `id` ist leer, beginnt mit `-` oder enthält andere Zeichen als Buchstaben, Ziffern, `.`, `_`, `-` | `invalid id` |
| Zwei Einträge haben dieselbe `id` | `duplicate app id` |
| `name` ist leer | `has an empty name` |
| `group` ist leer | `has an empty group` |
| `type` ist nicht `desktop`, `web`, `static` oder `cli` | `unknown type` |
| `port` ist `0` (ein `port` über 65535 lässt sich nicht parsen) | `invalid port 0` |
| `static`-Eintrag ohne `url` | `requires a url` |
| Jeder andere Typ ohne `command` | `requires a command` |

Fehler benennen den Eintrag über seine Position, zum Beispiel:

```text
apps.json entry 2 (site) requires a command
```

### Die ID

Die `id` ist der dauerhafte Schlüssel des Eintrags. Sie benennt den Protokollordner und die Symboldatei, und sie ist es, die
Sie an `moonpool.exe launch <id>` und an Agenten übergeben. Der Dialog leitet sie aus dem Namen ab,
wenn Sie eine App hinzufügen. Er schreibt den Namen klein, macht aus jeder Folge
von Zeichen außer `a` bis `z` und `0` bis `9` ein einzelnes `-` und entfernt `-` an beiden
Enden. Ein leeres Ergebnis wird zu `app`. Ist die ID schon vergeben, hängt er `-2`, `-3` usw. an. Er
ändert die ID danach nie mehr, sodass eine umbenannte App ihre ID behält. Der Name `Habit Tracker` ergibt die ID
`habit-tracker`.

## Wenn die Datei fehlerhaft ist

- **Beim Neu laden** bleibt eine Datei, die die Prüfung nicht besteht, unangetastet, und Moonpool behält die zuletzt
  geladene Liste. Ein Banner über der Seitenleiste zeigt den Fehler mit einer Schaltfläche zum Öffnen der
  Datei; die Liste bleibt benutzbar, wird aber abgedunkelt. Siehe
  [Wenn apps.json einen Fehler enthält](/de/using/hub-window/#wenn-appsjson-einen-fehler-enthält).
- **Beim Start** gibt es bei einer defekten Datei keine Liste, die man behalten könnte, also startet Moonpool ohne
  Apps, und das Banner weist darauf hin. Korrigieren Sie die Datei und wählen Sie **Neu laden**, oder stellen Sie einen Snapshot wieder her
  (unten, oder über das Tool `moonpool_restore_config`).
- In beiden Fällen wird das Speichern aus dem Dialog (sowie Umbenennen, Löschen, Symbol wählen) abgelehnt, bis die
  Datei wieder geladen werden kann, sodass die defekte Datei nie überschrieben wird. Korrigieren Sie die Datei und wählen Sie
  **Neu laden**.
- **Bei einer Änderung über den Dialog, einen Agenten oder eine Wiederherstellung** wird eine ungültige Änderung abgelehnt, und die Datei
  auf dem Datenträger bleibt, wie sie war.

Moonpool bewahrt die letzten 10 guten Versionen von `apps.json` in `apps.json.history\` auf. Wie Sie
zurückrollen, steht unter [Sicherung und Wiederherstellung](/de/data/backup-and-recovery/#appsjson-zurückrollen).
Symptome und Lösungen finden Sie unter [Fehlerbehebung](/de/support/troubleshooting/#appsjson-enthält-einen-fehler).

## Agenten

Ein KI-Agent sollte `apps.json` über die MCP-Tools von Moonpool ändern statt über die Datei, damit ein
veralteter oder ungültiger Schreibzugriff abgelehnt wird und ein Agent in einer Sandbox nie eine private Kopie bearbeitet. Siehe
[MCP-Tools](/de/automation/mcp-tools/#konfiguration).
