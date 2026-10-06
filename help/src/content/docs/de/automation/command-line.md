---
title: "Moonpool über die Befehlszeile steuern"
description: "Steuern Sie ein laufendes Moonpool mit moonpool.exe-Verben aus einem Terminal oder Skript, versehen Sie einen Befehl mit einem Ticket und lesen Sie das Ergebnis aus state.json."
---

Wird eine `moonpool.exe` erneut gestartet, während dasselbe Moonpool bereits läuft, öffnet sich kein
zweites Fenster. Der zweite Prozess übergibt seine Argumente über den
[Steuerkanal](/de/automation/control-verbs/) an das laufende und beendet sich. Moonpool muss bereits laufen:
Ist nichts resident, startet derselbe Befehl ein neues Moonpool, und das Verb wird nicht ausgeführt.

„Dasselbe Moonpool“ bedeutet denselben Ordner. Das installierte Moonpool und jede portable Kopie laufen
jeweils für sich, ein Befehl erreicht also die Kopie, deren `moonpool.exe` Sie gestartet haben, nie eine andere.
Siehe [Portabler Modus](/de/data/portable-mode/#mehrere-kopien-gleichzeitig).

Verwenden Sie den Pfad der Kopie, die Sie meinen. Für die installierte:

```powershell frame="terminal"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
```

Laufen mehrere Kopien, listet `Get-Process moonpool` alle auf; wählen Sie also nach `Path`
aus, statt die erste zu nehmen. Es listet auch untätige `moonpool.exe mcp`-Hilfsprozesse auf, die MCP-Hosts
gestartet haben, ein `moonpool`-Prozess beweist also nicht, dass ein Hub läuft. Fragen Sie stattdessen den Steuerkanal
mit `ping` ([Steuerverben](/de/automation/control-verbs/)).

## Verben

Beim Verb wird die Groß-/Kleinschreibung nicht beachtet. `<id>` ist die `id` einer App aus `apps.json`.

| Befehl | Wirkung |
| --- | --- |
| `moonpool.exe` | Kein Verb: bringt das Fenster nach vorn. |
| `moonpool.exe show` | Bringt das Fenster nach vorn. |
| `moonpool.exe launch <id>` | Startet die App und öffnet ihren Terminal-Tab. |
| `moonpool.exe stop <id>` | Stoppt die App. |
| `moonpool.exe restart <id>` | Stoppen, warten bis Port und Prozess frei sind, starten. |
| `moonpool.exe reload` | Liest `apps.json` neu ein. |
| `moonpool.exe refresh-icons` | Ruft jedes Symbol neu ab. |
| `moonpool.exe help` | Öffnet das Hilfefenster. |
| `moonpool.exe quit` | Beendet Moonpool, wie das Tray-Menü. |
| `moonpool.exe dump <id> [out-path]` | Ohne `out-path` meldet es den Pfad des Protokolls der App für diese Sitzung. Mit `out-path` kopiert es das Protokoll dorthin, als reiner Text ohne ANSI-Codes. |
| `moonpool.exe paths` | Meldet den Konfigurationsordner, `apps.json`, `state.json`, das Protokoll, den dumps-Ordner, den Symbolordner, das Portabel-Flag und den EXE-Pfad, die das laufende Moonpool verwendet. |
| `moonpool.exe read-config` | Schreibt `dumps\read-config.json` im Konfigurationsordner mit `token`, `valid`, `error`, `path` und `manifest_text` (dem exakten Inhalt von `apps.json`). |
| `moonpool.exe write-config <file> [token]` | Ersetzt `apps.json` durch das Manifest in `<file>`, wenn das Manifest gültig ist und, falls `token` angegeben ist, `apps.json` ihm noch entspricht. |
| `moonpool.exe restore-config [index or filename]` | Ohne Argument schreibt es die Snapshot-Liste nach `dumps\restore-config.json`. Mit einem Argument stellt es diesen Snapshot wieder her, wenn er gültig ist. |

```powershell frame="terminal"
& $mp restart my-app
```

```powershell frame="terminal"
& $mp dump my-app C:\temp\my-app.log
```

Ein unbekanntes Verb wird ignoriert. Das Programm hat außerdem eigene Startargumente:
`moonpool.exe mcp` ([MCP-Einrichtung](/de/automation/mcp-setup/)), `--uninstall` (von Add/Remove
Programs verwendet) und `--wait-pid <pid>` (verwendet, wenn Moonpool sich selbst neu startet). Diese werden nur
als erstes Argument beachtet, sodass eine App-ID wie `--uninstall` sie nicht auslösen kann.

## Das Ergebnis lesen

Die Befehlszeile gibt nichts aus. Versehen Sie einen Befehl daher mit `--ticket <key>` (ein beliebiger eindeutiger Schlüssel, an
beliebiger Position) und lesen Sie das Ergebnis aus `state.json` im Konfigurationsordner. Das ist
`%USERPROFILE%\.moonpool\moonpool-config\` bei der Installation, `<your .moonpool folder>\moonpool-config\`
bei einer portablen Kopie und `~/.config/Moonpool/` unter Linux (siehe
[Konfigurationsübersicht](/de/apps/apps-json/#wo-die-konfiguration-liegt)). `show` und `quit`
schreiben kein Ticket.

```powershell frame="terminal"
& $mp launch my-app --ticket t1
```

`state.json` enthält `apps`, `statuses` (`id`, `running`, `managed`, `mcpRunning`, `mcpSeen` pro
App) und `tickets`. Das laufende Moonpool schreibt sie alle paar Sekunden und nach jedem
Befehl neu und löscht sie beim Beenden nicht, eine übrig gebliebene Datei bedeutet also nicht, dass Moonpool
läuft. Um zu fragen, ob es läuft, oder um die aktuelle App-Liste zu erhalten, nutzen Sie die Verben `ping`
und `list` des Steuerkanals ([Steuerverben](/de/automation/control-verbs/)) oder die MCP-Tools. Fragen Sie
Ihr Ticket ab, bis `status` nicht mehr `pending` ist:

| `status` | Bedeutung |
| --- | --- |
| `pending` | Empfangen; Moonpool arbeitet noch daran. |
| `ok` | Erledigt. Bei `dump`, `read-config`, `write-config`, `restore-config` und `paths` enthält `detail` den Pfad, das Token oder den Bericht. |
| `error` | Fehlgeschlagen; `detail` nennt den Grund, zum Beispiel `unknown app id: x`, `did not reach running in time`, `unknown command`. |

Jedes Ticket ist `{ ticket, action, arg, status, detail, ts }`, mit `ts` in Unix-Millisekunden:

```json title="state.json (tickets entry)"
{
  "ticket": "t1",
  "action": "launch",
  "arg": "my-app",
  "status": "error",
  "detail": "did not reach running in time",
  "ts": 1767225600000
}
```

Abgeschlossene Tickets werden nach 24 Stunden verworfen, und die Liste wird auf etwa 50 Einträge gekürzt, sobald
abgeschlossene Tickets mindestens 5 Minuten alt sind.

Ein Agent, der MCP unterstützt, kann das Abfragen überspringen: siehe [MCP-Einrichtung](/de/automation/mcp-setup/).

## Siehe auch

- [KI-Agenten: Schnellstart](/de/automation/quick-start/)
- [Steuerverben](/de/automation/control-verbs/)
