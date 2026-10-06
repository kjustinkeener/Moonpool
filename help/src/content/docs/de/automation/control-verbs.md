---
title: "Referenz zu Steuerkanal und Verben von Moonpool"
description: "Wie der Moonpool-Steuerkanal (Named Pipe oder Unix-Socket) arbeitet, sein Protokoll und alle Verben, die die laufende App beantwortet, mit Argumenten."
---

## Wo er lauscht

Jede Moonpool-Kopie hat ihren eigenen Kanal, sodass das installierte Moonpool und alle portablen Kopien
nebeneinander laufen können, ohne füreinander zu antworten. Unter Windows lauscht das installierte Moonpool
auf der Named Pipe `\\.\pipe\moonpool`. Eine portable Kopie hängt eine aus ihrem
Ordner gebildete ID an: `\\.\pipe\moonpool-<id>`.

`<id>` besteht aus 8 Hexadezimalziffern, die aus dem Pfad des `moonpool-config`-Ordners der Kopie abgeleitet werden. Sie bleibt
für diesen Ordner über Neustarts und Updates hinweg gleich und ändert sich, wenn Sie den Ordner verschieben. Die
`moonpool.exe` einer Kopie, einschließlich `moonpool.exe mcp`, findet immer den Kanal ihrer eigenen Kopie.

Unter Linux und macOS lauscht es stattdessen auf einem Unix-Domain-Socket mit dem Modus `0600`:

| Fall | Socket-Pfad |
| --- | --- |
| Normal | `$XDG_RUNTIME_DIR/moonpool.sock`, wenn diese Variable gesetzt ist, sonst `moonpool.sock` im Konfigurationsordner von Moonpool |
| Portabler Modus | `moonpool.sock` im Konfigurationsordner der portablen Kopie, sodass eine portable Kopie nie mit einer installierten kollidiert |
| Pfad zu lang für einen Socket (etwa 100 Zeichen) | `/tmp/moonpool-<uid>/moonpool.sock`, in einem Verzeichnis, das nur Sie öffnen können (`moonpool-<id>.sock` bei einer portablen Kopie) |

Eine nach einem Absturz zurückgebliebene Socket-Datei wird beim nächsten Start erkannt und ersetzt. Ein Socket,
auf dem noch etwas antwortet, wird nie übernommen. Die Datei wird entfernt, wenn Moonpool normal
beendet wird.

Über den Kanal weiß auch der [MCP-Server](/de/automation/mcp-setup/), ob Moonpool
läuft: Wird ein `ping` beantwortet, läuft es, fehlt die Pipe oder der Socket, läuft es nicht. Dieselben
Verben sind auch über die [Befehlszeile](/de/automation/command-line/) erreichbar, außer den
Diagnoseverben weiter unten.

## Protokoll

Ein JSON-Objekt pro Zeile hinein, eine JSON-Zeile heraus, der Reihe nach. Eine Verbindung kann viele
Anfragen tragen.

```json title="request"
{"cmd": "restart", "args": ["my-app"]}
```

```jsonl title="replies"
{"ok": true, "result": "..."}
{"ok": false, "error": "unknown app id: ..."}
```

Eine Anfrage und ihre Antwort aus PowerShell:

Für eine portable Kopie verwenden Sie deren Pipe-Namen (`moonpool-<id>`, vom Verb `paths` angezeigt) anstelle
von `moonpool`.

```powershell frame="terminal"
$p = New-Object System.IO.Pipes.NamedPipeClientStream('.', 'moonpool', 'InOut')
$p.Connect(2000)
$w = New-Object System.IO.StreamWriter($p); $w.AutoFlush = $true
$r = New-Object System.IO.StreamReader($p)
$w.WriteLine('{"cmd":"ping"}')
$r.ReadLine()
```

```json title="reply"
{"ok":true,"result":"pong"}
```

- `args` ist eine Liste von Zeichenketten und darf fehlen. Andere Felder werden ignoriert.
- `result` ist eine Zeichenkette oder null. Verben, die strukturierte Daten liefern, geben sie als JSON-Zeichenkette
  zurück.
- Eine Zeile, die kein gültiges JSON ist, erhält `{"ok": false, "error": "bad request: ..."}`.
- Ein unbekanntes `cmd` erhält `unknown cmd: <name>`.
- Ein Verb, das über das Fenster läuft (`launch`, `stop`, `restart`, `reload`,
  `refresh-icons`, `help`, `open-window`), wird beantwortet, wenn die Aktion abgeschlossen ist, oder mit einem Zeitlimit-Fehler
  nach 45 s. Ist die Oberfläche des Hub-Fensters nicht geladen, schlägt es sofort mit `frontend not
  loaded` fehl.
- Ein Moonpool, das startet, während ein vorheriges noch beendet wird, versucht etwa 8 Sekunden lang, den Kanal zu binden.
  Gelingt das weiterhin nicht, protokolliert es das und läuft ohne Kanal weiter.

## Verben

| Verb | Argumente | Ergebnis |
| --- | --- | --- |
| `ping` | keine | `pong`. Nur Kanal. |
| `list` | keine | JSON-Zeichenkette `{"apps": [...], "statuses": [...]}`, aus dem Speicher des laufenden Hubs gelesen, mit derselben Struktur von `apps` und `statuses` wie `state.json`. Fügt `"statusNotReady": true` hinzu, wenn Apps registriert sind, die erste Statusprüfung aber noch nicht lief. Solange `apps.json` nicht geladen werden kann, kommt `"manifestError": "<message>"` hinzu (die Apps sind dann die zuletzt geladene Liste) und, wenn seit dem Start noch keine Liste geladen wurde, `"manifestLoaded": false`. Nur Kanal. |
| `show` | keine | null. Bringt das Fenster in den Vordergrund. |
| `quit` | keine | null. Beendet Moonpool. |
| `launch` | `<id>` | null bei Erfolg, oder `opened` bei einem `static`-Eintrag mit nur einer `url`. Fehler: `unknown app id: <id>`, `did not reach running in time`. |
| `stop` | `<id>` | null bei Erfolg, oder `stopped` bei einem `static`-Eintrag mit nur einer `url`. Fehler: `still running after stop`. |
| `restart` | `<id>` | Dieselben Ergebnisse und Fehler wie `launch`. |
| `reload` | keine | null bei Erfolg. |
| `refresh-icons` | keine | null bei Erfolg. |
| `help` | keine | null. Öffnet das Hilfefenster. |
| `dump` | `<id>` [`out-path`] | Pfad des Sitzungsprotokolls der App oder der Klartextkopie unter `out-path`. |
| `paths` | keine | Mehrzeiliger Bericht über die Ordner und die EXE, die der Hub verwendet. |
| `read-config` | keine | Pfad von `dumps\read-config.json`, die `token`, `valid`, `error`, `path`, `manifest_text` enthält. |
| `write-config` | `<source-file>` [`token`] | Der neue Versions-Token. Fehler: `stale token: ...`, `rejected invalid manifest: ...`, `cannot read source ...`. |
| `restore-config` | [`index` oder `filename`] | Ohne Argument: Pfad von `dumps\restore-config.json` (`count`, `snapshots`). Mit einem: `restored <file> (<n> apps); new version token <token>`. |
| `argv` | die Befehlszeilenargumente | null, sofort. Führt sie genau so aus, wie ein zweites `moonpool.exe <args>` dieser Kopie es täte, einschließlich `--ticket`. So übergibt dieser zweite Start seine Argumente, bevor er sich beendet. |

Beispielabläufe:

```jsonl title="request, reply"
{"cmd": "launch", "args": ["nope"]}
{"ok": false, "error": "unknown app id: nope"}

{"cmd": "restore-config", "args": ["1"]}
{"ok": true, "result": "restored 1767225600000.json (6 apps); new version token <token>"}
```

`write-config` und `restore-config` laden das neue Manifest sofort, legen einen Snapshot in
`apps.json.history\` ab und aktualisieren das Fenster.

## Diagnoseverben (Tests)

Nur Kanal: Die Befehlszeile akzeptiert diese nicht. Alle funktionieren unter Windows, Linux und macOS,
außer `screenshot`, das nur unter Windows läuft und sonst `screenshot is not supported on this
platform (Windows only)` antwortet.

| Verb | Argumente | Ergebnis |
| --- | --- | --- |
| `screenshot` | [`window`] [`max_dim`] | Nur Windows. Base64 eines PNG dieses Moonpool-Fensters (Standard `main`). Optional begrenzt `max_dim` die längere Seite in Pixeln (auf 320-2400 beschränkt, Standard 320; das MCP-Tool nutzt immer den Standard). Ein nicht ganzzahliges `max_dim` ist ein Fehler. Erlaubte Fenster: `main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`. Fehler: `unknown window '<name>'`, `window '<name>' is not open`. Wird nicht auf den Datenträger geschrieben. |
| `open-window` | `<kind>` [`<id>`] | null. Öffnet ein Fenster so, wie es sein Menüeintrag tut. `kind`: `settings`, `about`, `installer`, `help`, `themes`, `editor` (mit optionaler `<id>` öffnet es den Dialog „App bearbeiten“ dieser App, ohne den Dialog „App hinzufügen“), `terminal` (`<id>` erforderlich: wählt den Terminal-Tab dieser App und verbreitert den Hub, sodass der CLI-Bereich erscheint; startet die App nicht), `cli` (verbreitert nur den Hub). Fehler: `unknown window kind '<kind>'`, `terminal needs an app id`, `unknown app id: <id>`. Wird wie `launch` über das Hub-Fenster beantwortet. |
| `window-state` | [`window`] | JSON-Zeichenkette: `{"open":false}`, oder `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. |
| `stop-mcp` | `<id>` | `stopped`. Beendet den `<processName> mcp`-Helfer der App, nicht die App. Fehler: `missing app id`, `unknown app id: <id>`. |
| `reset-mcp-seen` | [`<id>`] | `<id>: cleared` oder `<id>: was not marked seen`; ohne ID `cleared <n> entries`. Löscht die gemerkten Sichtungen von MCP-Helfern. |

Das `--ticket` der Befehlszeile und die Ergebnisdatensätze in `state.json` gehören zum anderen Kanal;
siehe [Befehlszeile](/de/automation/command-line/#das-ergebnis-lesen). Anfragen über den Kanal erhalten ihre
Antwort in der Antwortzeile.

## Siehe auch

- [Befehlszeile](/de/automation/command-line/)
- [KI-Agenten: Schnellstart](/de/automation/quick-start/#dieselbe-aktion-auf-drei-wegen)
