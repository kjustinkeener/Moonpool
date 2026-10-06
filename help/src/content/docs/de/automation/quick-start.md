---
title: "KI-Agenten Moonpool einrichten und steuern lassen: Schnellstart"
description: "Drei Wege, wie ein KI-Agent oder ein Skript Moonpool einrichten und steuern kann, welcher für Ihren Agenten passt und dieselbe Aktion in jedem Weg gezeigt."
---

Es gibt drei Wege. Wählen Sie danach, was Ihr Agent kann.

| Sie möchten | Verwenden | Hier beginnen |
| --- | --- | --- |
| Dass ein Agent einmalig Ihre Apps findet und hinzufügt | **Prompt kopieren** auf dem leeren Bildschirm des Hubs | Unten |
| Dass ein Agent Apps als Tool-Aufrufe startet, stoppt und ausliest | Der MCP-Server, `moonpool.exe mcp` | [MCP-Einrichtung](/de/automation/mcp-setup/) |
| Ein Skript oder ein Agent ohne MCP | Verben der Befehlszeile | [Befehlszeile](/de/automation/command-line/) |

## Prompt kopieren

Wenn kein Tab geöffnet ist, zeigt der CLI-Bereich einen vorgefertigten Prompt („Neu hier? Geben Sie das einem
KI-Agenten, damit er Ihre Apps einrichtet“). **Prompt kopieren** legt ihn in die Zwischenablage. Fügen Sie ihn in Ihren
Agenten ein. Er verweist den Agenten auf `AI-README.md` und `apps.json` in Ihrem Konfigurationsordner und bittet ihn,
Ihre Apps zu finden und zu registrieren. Wenn er fertig ist, wählen Sie **Neu laden**.

Moonpool schreibt `AI-README.md` bei jedem Start neben `apps.json` neu, sodass sie immer zur
Version passt, die Sie ausführen. Halten Sie darin keine eigenen Änderungen.

## Dieselbe Aktion auf drei Wegen

| Aktion | Befehlszeile | Verb des Steuerkanals | MCP-Tool |
| --- | --- | --- | --- |
| Eine App starten | `moonpool.exe launch <id>` | `launch` | `moonpool_start_app` |
| Eine App stoppen | `moonpool.exe stop <id>` | `stop` | `moonpool_stop_app` |
| Eine App neu starten | `moonpool.exe restart <id>` | `restart` | `moonpool_restart_app` |
| Die Ausgabe einer App lesen | `moonpool.exe dump <id> [out-path]` | `dump` | `moonpool_app_output` |
| Apps und Status auflisten | `state.json` lesen | `list` | `moonpool_list_apps` |
| `apps.json` neu einlesen | `moonpool.exe reload` | `reload` | `moonpool_reload_config` |
| `apps.json` lesen | `moonpool.exe read-config` | `read-config` | `moonpool_read_config` |
| `apps.json` ersetzen | `moonpool.exe write-config <file> [token]` | `write-config` | `moonpool_write_config` |
| `apps.json` zurückrollen | `moonpool.exe restore-config [n]` | `restore-config` | `moonpool_restore_config` |
| Das Fenster anzeigen | `moonpool.exe show` | `show` | `moonpool_raise_launcher` |
| Moonpool starten | `moonpool.exe` | keines | `moonpool_bootup_launcher` |
| Moonpool beenden | `moonpool.exe quit` | `quit` | `moonpool_shutdown_launcher` |
| Die verwendeten Ordner anzeigen | `moonpool.exe paths` | `paths` | `moonpool_launcher_paths` |

Die Befehlszeile gibt nichts aus; lesen Sie das Ergebnis mit einem `--ticket` (siehe
[Das Ergebnis lesen](/de/automation/command-line/#das-ergebnis-lesen)). Der Kanal und MCP
antworten direkt.

## Wenn die Tools eines Agenten fehlschlagen

- `Moonpool is not running - call moonpool_bootup_launcher first`: Starten Sie Moonpool oder lassen Sie den
  Agenten dieses Tool aufrufen.
- Eine Änderung „hat nicht gegriffen“: Bitten Sie den Agenten um `moonpool_launcher_paths`. Unterscheiden sich die Ordner von Hub und MCP,
  liest der Agent eine andere `apps.json`. Siehe
  [Sandbox-Hosts](/de/automation/mcp-setup/#hosts-in-einer-sandbox).
- Mehrere Moonpool-Kopien: Registrieren Sie jede unter einem eigenen Namen. Siehe
  [Mehr als ein Moonpool](/de/automation/mcp-setup/#mehr-als-ein-moonpool).

Ein durchgespieltes Beispiel für Claude Code, Codex und Cursor finden Sie unter
[Einem KI-Agenten einen MCP-Server zum Starten und Stoppen lokaler Apps geben](/de/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/).

Weitere Symptome unter [Fehlerbehebung](/de/support/troubleshooting/#mcp--und-skriptfehler).
