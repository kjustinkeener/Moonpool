---
title: "KI-Agenten über MCP mit Moonpool verbinden"
description: "moonpool.exe mcp als stdio-MCP-Server bei Ihrem Host registrieren, installiert oder portabel, und wie Moonpool den MCP-Helfer einer App erkennt."
---

Die ausführbare Datei von Moonpool ist zugleich ihr eigener MCP-Server. Registrieren Sie sie
beim Host als stdio-Server, der `moonpool.exe` mit dem einzigen Argument `mcp` ausführt.

## Den Server registrieren

Installiert ist das Programm `%USERPROFILE%\.moonpool\moonpool.exe`. Portabel ist es die `moonpool.exe` in Ihrem `.moonpool\`-Ordner. Verwenden Sie diesen vollständigen Pfad als
`command`. Für einen Host, der eine `.mcp.json` liest:

```json title=".mcp.json" {5}
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

In einer JSON-Datei müssen die Backslashes verdoppelt werden, wie oben. Ein Host mit
Registrierung über die Befehlszeile, etwa Claude Code, kann den Server in einem Schritt
hinzufügen:

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

Der Server meldet sich als
`moonpool`, spricht die MCP-Protokollrevision `2025-06-18` und stellt nur Tools bereit (er
listet weder Ressourcen noch Prompts auf). Die Tools erscheinen für den Agenten als `moonpool_*`; siehe
[MCP-Tools](/de/automation/mcp-tools/).

## Mehr als ein Moonpool

Das installierte Moonpool und jede portable Kopie sind getrennte Starter mit jeweils eigenen Apps,
und alle können gleichzeitig laufen. Das `moonpool.exe mcp` einer Kopie steuert immer genau diese Kopie. Damit
ein Agent mehrere nutzen kann, registrieren Sie jede unter einem eigenen Namen und zeigen auf die EXE dieser Kopie:

```json title=".mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    },
    "moonpool-work": {
      "type": "stdio",
      "command": "D:\\Work\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

```powershell frame="terminal"
claude mcp add moonpool-work -- "D:\Work\.moonpool\moonpool.exe" mcp
```

Werden zwei Kopien unter demselben Namen registriert, ersetzt in den meisten Hosts die eine die andere. Die
Tool-Namen sind für jede Kopie gleich, der Host unterscheidet sie also anhand des Namens, den Sie
registrieren. Eine portable Kopie meldet sich außerdem als `moonpool (<folder>)`, und ihre Server-Anweisungen
nennen den Ordner, sodass der Agent erkennt, mit welcher Kopie er spricht.

## Hinweise

- `moonpool.exe mcp` öffnet nie ein Fenster und startet nie den Installer. Es beendet sich, wenn der
  Host seine Eingabe schließt.
- Es verwendet den Konfigurationsordner und den Steuerkanal der EXE, von der es gestartet wurde. Eine
  portable EXE liest also die Daten des portablen Ordners und steuert diese portable Kopie. Eine EXE
  gilt nur dann als portabel, solange `moonpool.portable` daneben liegt. Jede andere
  `moonpool.exe`, wo auch immer sie liegt, verwendet den Ordner des installierten Moonpool
  (`%USERPROFILE%\.moonpool\moonpool-config\`) und steuert das installierte Moonpool.
- Die meisten Tools brauchen ein laufendes Moonpool. Läuft es nicht, kann der Agent zuerst
  `moonpool_bootup_launcher` aufrufen.
- `moonpool_launcher_paths` zeigt die Ordner, die der Hub verwendet, neben denen, die der MCP-Prozess
  auflöst. Ein Unterschied bedeutet, dass der Agent eine andere `apps.json` sieht als der Hub.

## Hosts in einer Sandbox

Manche Hosts führen ihre Tools in einer gepackten Sandbox (Store/MSIX) aus, die AppData auf eine
private Kopie pro Paket umleitet. Moonpool erkennt das, wenn sein Konfigurationsordner oder seine EXE unter
einem Pfad wie `...\Packages\<package>\LocalCache\...` aufgelöst wird.

Es erkennt es auch, wenn der Steuerkanal antwortet, `state.json` aber nicht gelesen werden kann. Die
Tools, die Dateien lesen oder schreiben (`moonpool_app_output`, `moonpool_read_config`,
`moonpool_write_config`, `moonpool_restore_config`), liefern dann einen Fehler, der die
Ursache nennt, statt leerer oder veralteter Daten. Tools, die nur den Steuerkanal nutzen, etwa
`moonpool_list_apps`, werden nicht blockiert, solange der Kanal erreichbar ist. Verbirgt die Sandbox auch den
Kanal, melden die Tools die Sandbox statt „Moonpool is not running“.
Nutzen Sie stattdessen die [Befehlszeile](/de/automation/command-line/) in einer Shell außerhalb der Sandbox.

## Apps mit eigenem MCP-Server

Viele Apps in Moonpool werden von einem MCP-Host selbst über einen `<exe> mcp`-Helferprozess erreicht. Moonpool sucht nach einem Prozess, dessen Name zum `processName` der App passt und dessen
erstes Argument `mcp` ist, etwa `notes-app.exe mcp`. Läuft der Server unter einem anderen Namen, etwa als umbenannte Kopie, setzen Sie den Platzhalter `mcpProcessName` der App (siehe [Felder](/de/apps/fields/#mcpprocessname)); ein dazu passender Prozess zählt auch ohne das Argument `mcp`.

- Solange einer angebunden ist, zeigt die Seitenleiste der App eine MCP-Unterzeile als laufend an, und
  `moonpool_list_apps` hängt `[mcp: running]` an die Zeile der App an. Der Helfer zählt nicht
  als Lauf der App selbst.
- Sobald ein Helfer gesehen wurde, merkt sich Moonpool ihn (in `mcp_seen.json` im Konfigurationsordner),
  sodass die MCP-Unterzeile nach dem Ende des Helfers als gestoppt sichtbar bleibt und
  `moonpool_list_apps` `[mcp: stopped]` zeigt.
- Die MCP-Unterzeile wird durch die Einstellung `showMcpProcesses` gesteuert
  ([Einstellungsfenster](/de/using/settings/)).
- `moonpool_stop_mcp_server` beendet den Helfer und lässt die App in Ruhe. Es gibt kein Gegenstück zum
  Starten: Der Host, dem der Helfer gehört, startet ihn beim nächsten Tool-Aufruf erneut.

## Wenn die Tools nicht funktionieren

- **Der Host zeigt keine `moonpool_*`-Tools.** Prüfen Sie, ob `command` der vollständige Pfad zu
  `moonpool.exe` und `args` gleich `["mcp"]` ist, und starten Sie dann den Host neu.
- **Jedes Tool meldet, dass Moonpool nicht läuft.** Starten Sie Moonpool oder rufen Sie
  `moonpool_bootup_launcher` auf. Stellen Sie sicher, dass die registrierte EXE die Kopie ist, die Sie ausführen.
- **Eine Änderung wird nicht angezeigt.** Rufen Sie `moonpool_launcher_paths` auf und vergleichen Sie die Ordner des Hubs
  mit denen des MCP-Prozesses. Siehe [Hosts in einer Sandbox](#hosts-in-einer-sandbox).

Mehr unter [Fehlerbehebung](/de/support/troubleshooting/#mcp--und-skriptfehler).

## Siehe auch

- [Einem KI-Agenten (Claude Code, Codex, Cursor) einen MCP-Server zum Starten und Stoppen lokaler Apps geben](/de/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/)
- [MCP-Tools](/de/automation/mcp-tools/)
- [KI-Agenten: Schnellstart](/de/automation/quick-start/)
