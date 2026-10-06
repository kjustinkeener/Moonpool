---
title: "Jedes apps.json-Feld: Typ, Standardwert und Wirkung"
description: "Schlagen Sie jeden Schlüssel eines apps.json-Eintrags mit Typ, Standardwert und nutzenden App-Typen nach, passend zu den Namen im Dialog „App bearbeiten“."
---

Der Dialog „App bearbeiten“ zeigt dieselben Felder unter denselben Namen. Felder, die für
den gewählten Typ nicht gelten, sind im Dialog abgedunkelt, werden aber trotzdem gespeichert, mit einer Ausnahme:
`stopCommand` wird nur gespeichert, solange `killMode` auf `command` steht.

![Der Dialog „App bearbeiten“ von name bis stopCommand, mit hervorgehobener Auswahl killMode; ungenutzte Felder wie processName und stopCommand sind abgedunkelt](../../../../assets/screenshots/edit-app-dialog.png)

1. Die Auswahl `killMode`. Felder, die sie nicht nutzt, bleiben abgedunkelt.

| Feld | Typ | Pflicht | Verwendet von | Wirkung |
| --- | --- | --- | --- | --- |
| `id` | string | ja | alle | Eindeutiger Schlüssel. Buchstaben, Ziffern, `.`, `_`, `-`, darf nicht mit `-` beginnen. Siehe [Übersicht](/de/apps/apps-json/#die-id). |
| `name` | string | ja | alle | Bezeichnung in der Seitenleiste. Nicht leer. |
| `group` | string | ja | alle | Überschrift in der Seitenleiste, unter der die App steht. Bei einer Handänderung nicht leer; der Dialog speichert eine leere Gruppe als `Apps`. Beliebiger Text; ein neuer Name legt eine neue Gruppe an. |
| `type` | string | ja | alle | `web`, `desktop`, `static` oder `cli`. Siehe [App-Typen](/de/apps/types/). |
| `command` | string | alle außer `static` | alle | Wird in einem Terminal ausgeführt, um die App zu starten, über `cmd /c` unter Windows und `$SHELL -c` sonst (`/bin/sh`, wenn `SHELL` nicht gesetzt ist). Für `static` optional. |
| `cwd` | string | nein | alle mit einem `command` | Ordner, in dem der Befehl läuft. Standard ist der eigene Arbeitsordner von Moonpool. Unterstützt Token und `./`. Siehe [Pfade und Umgebung](/de/apps/paths-and-environment/). |
| `port` | integer, 1 bis 65535 | nein | beliebig | Läuft, solange etwas auf diesem Port auf localhost antwortet (IPv4 oder IPv6). Wird von `killMode` `port` gelesen. |
| `processName` | string | nein | beliebig, vor allem `desktop` | Läuft, solange ein Prozess mit diesem Namen existiert. Ohne Beachtung der Groß-/Kleinschreibung, mit oder ohne `.exe`, sodass `my-app` auf `my-app.exe` passt. Unter Linux höchstens 15 Zeichen. Wird von `killMode` `processName` gelesen. |
| `mcpProcessName` | string | nein | beliebig mit einem `processName` | Platzhaltermuster für den Prozessnamen des MCP-Servers dieser App. `*` steht für eine beliebige Zeichenfolge, `?` für genau ein Zeichen. Ohne Beachtung der Groß-/Kleinschreibung, verglichen mit dem ganzen Namen, und `.exe` ist optional. Ein passender Prozess zählt als MCP-Server der App (die MCP-Unterzeile in der Seitenleiste) und braucht `mcp` nicht als erstes Argument. Siehe [mcpProcessName](#mcpprocessname). |
| `url` | string | nur `static` | `web`, `static` | Seite, die geöffnet wird. Nur URLs mit `http://`, `https://`, `mailto:` und `file://` werden geöffnet. |
| `openBrowser` | boolean, Standard `false` | nein | jeder Typ mit einer `url` (der Dialog dunkelt es für `desktop` und `cli` ab) | Öffnet `url` automatisch, sobald Moonpool erkennt, dass die App oben ist (siehe unten). |
| `killMode` | string | nein | alle | Zusätzliche Aufräumschritte bei Stoppen und Neu starten: `processName`, `port`, `command` oder `none`. Siehe [Stoppen und neu starten](/de/apps/stop-and-restart/). |
| `stopCommand` | string | nein | `killMode` `command` | Befehl, der beim Stoppen ausgeführt wird. In jedem anderen Modus ignoriert. |
| `env` | object of strings | nein | alle | Zusätzliche Umgebungsvariablen. Der Dialog bearbeitet sie als eine `KEY=VALUE` pro Zeile. |
| `icon` | string | nein | alle | Bild in der Seitenleiste: ein Dateipfad, eine `http(s)`-URL oder ein `data:`-URI. Setzen Sie es über **Symbol wählen...** im Kontextmenü der App oder von Hand. |
| `note` | string | nein | alle | Tooltip, wenn Sie in der Seitenleiste über die App fahren. |

Ein Eintrag mit `env` und `killMode`:

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000", "NODE_ENV": "development" },
  "killMode": "port"
}
```

Wie `port` und `killMode` zusammenarbeiten, steht unter
[Den Prozess finden und beenden, der einen Port belegt](/de/guides/find-and-kill-process-using-port-windows/).

## mcpProcessName

Standardmäßig behandelt Moonpool einen Prozess als MCP-Server der App, wenn sein Name zu
`processName` passt und sein erstes Argument `mcp` ist, etwa `notes-app.exe mcp`. Setzen Sie
`mcpProcessName`, wenn der Server unter einem anderen Namen läuft: eine App, die eine exe überwacht,
während ihr MCP-Server eine andere ist (`mog.exe mcp`), oder eine umbenannte Kopie des Servers.

Der Wert ist ein Platzhaltermuster. `*` steht für eine beliebige Zeichenfolge (auch keine) und `?`
für genau ein Zeichen. Es wird ohne Beachtung der Groß-/Kleinschreibung mit dem ganzen Prozessnamen verglichen, und ein
Muster ohne `.exe` passt auch auf den Namen mit `.exe`. Ein leerer Wert gilt als nicht gesetzt.

```json
{
  "id": "destiny",
  "name": "Destiny",
  "group": "Desktop apps",
  "type": "desktop",
  "processName": "destiny",
  "mcpProcessName": "destiny-mcp-*"
}
```

Das passt auf eine umbenannte Kopie wie `destiny-mcp-2706210170.exe`. Ein Prozess, der auf
`mcpProcessName` passt, ist der Server, egal ob er mit `mcp` gestartet wurde, und er zählt nie
als Lauf der App selbst. Passt das Muster auch auf `processName` selbst (zum Beispiel
`destiny*`), verlangt Moonpool weiterhin das Argument `mcp`, sodass die eigentliche App nie mit
ihrem MCP-Server verwechselt wird. Siehe [MCP-Einrichtung](/de/automation/mcp-setup/#apps-mit-eigenem-mcp-server).

## openBrowser

Moonpool öffnet `url` einmal, wenn eine App, die Moonpool gestartet hat, erstmals als „läuft“ erkannt wird. Dafür
braucht es einen `port` oder `processName` zur Erkennung. Ohne beides bedeutet „läuft“ nur, dass der
Terminalprozess lebt, und der Browser wird nicht automatisch geöffnet. Schalten Sie `openBrowser`
aus, wenn Ihr Befehl selbst einen Browser öffnet. Ein `static`-Eintrag ohne Befehl öffnet `url`
bei jedem Druck auf Starten, unabhängig von `openBrowser`.

Zwei Apps mit demselben `port` werden in der Seitenleiste markiert.

## Symbole

Das Symbol einer App ist das erste der folgenden, das existiert:

1. Das Feld `icon`.
2. `icons\<id>.<ext>` im Konfigurationsordner, zum Beispiel `icons\site.png`.
3. Eine Symboldatei im eigenen Ordner der App (ihr `cwd` oder der Ordner einer `file:///`-`url`).
4. Bei `desktop` das Symbol der gebauten oder laufenden `.exe`.
5. Bei `web` und `static` die `/favicon.ico` der Seite, sobald der Server oben ist.
6. Ein Glyph für den Typ.

Die meisten Apps brauchen keine Symboleinstellung.
