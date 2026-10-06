---
title: "Einem KI-Agenten (Claude Code, Codex, Cursor) einen MCP-Server zum Starten und Stoppen lokaler Apps geben"
description: "Moonpool als MCP-Server registrieren, damit Claude Code, Codex oder Cursor Entwicklungsserver starten, stoppen, neu starten und deren Ausgabe lesen können, ohne Duplikate."
---

Ein KI-Coding-Agent führt Ihren Entwicklungsserver meist aus, indem er `npm run dev` in
seine eigene Shell tippt. Das kann den Agenten blockieren, einen verwaisten Prozess
hinterlassen, der den Port belegt, oder eine zweite Kopie von etwas starten, das Sie schon
laufen haben. Ein MCP-Server lässt den Agenten Tools aufrufen, um die bereits konfigurierte
App zu starten und zu stoppen, statt ihre Befehlszeile selbst nachzubauen.

## Der Moonpool-Weg

Die ausführbare Datei von Moonpool ist selbst ein MCP-Server: Registrieren Sie `moonpool.exe`
mit dem einzigen Argument `mcp` als stdio-Server. Sobald die App in `apps.json` steht,
startet der Agent sie über ihre ID.

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

Registrieren Sie den Server. In Claude Code genügt ein Befehl (installiertes Moonpool;
verwenden Sie den vollständigen Pfad Ihrer eigenen EXE):

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

Hosts, die eine JSON-Datei mit MCP-Servern lesen, etwa die `mcp.json` von Cursor, verwenden
dieselbe Form (Backslashes verdoppelt):

```json title="mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

Für Codex fügen Sie in dessen Konfiguration (`~/.codex/config.toml`) einen Server mit
demselben Befehl und dem Argument `mcp` hinzu:

```toml title="config.toml"
[mcp_servers.moonpool]
command = 'C:\Users\you\.moonpool\moonpool.exe'
args = ["mcp"]
```

Datei- und Schlüsselnamen sind Sache des jeweiligen Hosts, schlagen Sie daher in dessen
MCP-Dokumentation nach, falls Ihre Version abweicht. Moonpool braucht nur den vollständigen
Pfad zu `moonpool.exe` und `mcp` als Argument. Starten Sie den Host anschließend neu.

## Was der Agent tun kann

Die Tools erscheinen als `moonpool_*`. Die für die tägliche Arbeit:

| Tool | Verwendung |
| --- | --- |
| `moonpool_list_apps` | Die ID einer App finden und sehen, ob sie läuft. |
| `moonpool_start_app` | Eine App über ihre ID starten und ihren Terminal-Tab öffnen. |
| `moonpool_stop_app` | Sie stoppen, einschließlich ihrer untergeordneten Prozesse. |
| `moonpool_restart_app` | Stoppen, warten bis der Port frei ist, starten. Nach einer Codeänderung verwenden. |
| `moonpool_app_output` | Lesen, was die App ausgegeben hat, mit `tail_lines` zur Begrenzung. |
| `moonpool_bootup_launcher` | Moonpool selbst starten, falls es nicht läuft. |

Ein typischer Ablauf ist `moonpool_restart_app`, dann `moonpool_app_output`. Die übrigen
Tools (Lesen und Schreiben von `apps.json`, Screenshots) stehen unter
[MCP-Tools](/de/automation/mcp-tools/).

## Wenn es nicht funktioniert

Wenn jedes Tool `Moonpool is not running - call moonpool_bootup_launcher first` meldet, ist
Moonpool noch nicht gestartet. Eine Änderung, die nicht erscheint, bedeutet meist, dass der
Agent eine andere `apps.json` ansieht: Rufen Sie `moonpool_launcher_paths` auf. Siehe
[Wenn die Tools nicht funktionieren](/de/automation/mcp-setup/#wenn-die-tools-nicht-funktionieren).

## Siehe auch

- [MCP-Einrichtung](/de/automation/mcp-setup/)
- [MCP-Tools](/de/automation/mcp-tools/)
- [KI-Agenten: Schnellstart](/de/automation/quick-start/)
- [Einen npm-Entwicklungsserver unter Windows im Hintergrund ausführen](/de/guides/run-npm-dev-server-in-background-windows/)
