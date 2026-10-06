---
title: "Moonpool mit Skripten und KI-Agenten automatisieren"
description: "Die drei Wege, ein laufendes Moonpool von Skripten und KI-Agenten aus zu steuern (MCP, Befehlszeile und Steuerverben), wie sie zusammenhängen und was jeder ändern kann."
---

Moonpool lässt sich steuern, ohne sein Fenster anzufassen. Es gibt drei Zugänge, die alle vom
selben residenten Moonpool bedient werden (der Tray-Instanz, hier Hub genannt).

Jede Moonpool-Kopie ist ein eigener Hub: Das installierte und jede portable Kopie laufen
unabhängig voneinander, jede mit eigenem Steuerkanal. Ein Zugang erreicht immer die Kopie, deren
`moonpool.exe` er verwendet. Siehe [Portabler Modus](/de/data/portable-mode/#mehrere-kopien-gleichzeitig).

| Zugang | Was es ist | Referenz |
| --- | --- | --- |
| MCP-Server | `moonpool.exe mcp`, ein stdio-[MCP](https://modelcontextprotocol.io)-Server, den ein KI-Host startet. | [MCP-Einrichtung](/de/automation/mcp-setup/), [MCP-Tools](/de/automation/mcp-tools/) |
| Befehlszeile | `moonpool.exe <verb> [args]`. Ein zweiter Start derselben Kopie übergibt das Verb über den Steuerkanal an ihren Hub und beendet sich. | [Befehlszeile](/de/automation/command-line/) |
| Steuerkanal | Eine Named Pipe, `\\.\pipe\moonpool` (`\\.\pipe\moonpool-<id>` für eine portable Kopie), unter Windows und ein Unix-Socket unter Linux und macOS, mit einer JSON-Anfrage pro Zeile. | [Steuerverben](/de/automation/control-verbs/) |

## Wie sie zusammenhängen

- Der Hub besitzt alles: das Starten von Apps, die Sitzungsprotokolle, `apps.json`.
- Der MCP-Server ist ein Client des Hubs, keine zweite Kopie davon. Die meisten Tool-Aufrufe werden
  über den Steuerkanal an den Hub weitergeleitet, und die Antwort kommt als Tool-Ergebnis zurück.
  Die Ausnahmen: `moonpool_bootup_launcher` startet `moonpool.exe` selbst;
  `moonpool_app_output` und die Konfigurations-Tools bitten den Hub, eine Datei zu schreiben, und lesen sie dann;
  `moonpool_launcher_paths` ergänzt die Pfade des MCP-Prozesses zu denen des Hubs.
- Ob ein Hub läuft, wird durch einen Ping an diesen Kanal entschieden, nicht durch die Suche nach einem
  Prozess. Ein Hub, der antwortet, läuft; eine fehlende Pipe oder ein fehlender Socket bedeutet, dass er nicht läuft.
- Jeder Zugang führt dieselben Handler aus wie das Fenster, ein Verb tut also dasselbe wie der
  entsprechende Klick.
- Läuft kein Hub, verweigern die Tools, die auf ihn wirken, einschließlich `moonpool_list_apps`, mit
  „Moonpool is not running“. Es gibt keine veraltete Liste. `moonpool_bootup_launcher` startet ihn.
  Hält etwas den Kanal, antwortet aber nicht innerhalb weniger Sekunden, sagt der Fehler, dass ein
  Moonpool-Prozess möglicherweise hängt.
- Der MCP-Server greift nicht mehr auf die Steuerung eines Hub-Builds zurück, der älter ist als der Steuerkanal.
  Aktualisieren Sie diese Kopie oder beenden Sie sie und starten Sie sie neu.

## Was etwas ändern kann

| Kann ändern | Zugänge |
| --- | --- |
| Eine App starten, stoppen oder neu starten | MCP, Befehlszeile, Pipe |
| `apps.json` neu schreiben | MCP (`moonpool_write_config`, `moonpool_restore_config`), Befehlszeile, Pipe |
| Moonpool beenden | MCP (`moonpool_shutdown_launcher`), Befehlszeile (`quit`), Pipe |
| Den MCP-Hilfsprozess einer App beenden | MCP (`moonpool_stop_mcp_server`), Pipe (`stop-mcp`) |
| `apps.json` neu laden, Symbole neu abrufen, das Fenster anzeigen | MCP (`moonpool_reload_config`, `moonpool_refresh_app_icons`, `moonpool_raise_launcher`), Befehlszeile (`reload`, `refresh-icons`, `show`), Pipe |
| Ein Fenster oder einen Terminal-Tab öffnen | Pipe (`open-window`) |
| Gemerkte Sichtungen von MCP-Hilfsprozessen löschen | MCP (`moonpool_reset_mcp_seen`), Pipe (`reset-mcp-seen`) |

Nur lesende Tools: `moonpool_list_apps`, `moonpool_app_output`, `moonpool_read_config`,
`moonpool_launcher_paths`, `moonpool_window_state`, `moonpool_screenshot`.

## Sicherheitseigenschaften

- **Konfigurationsänderungen sind abgesichert.** Ein Schreibvorgang muss das Versions-Token des letzten Lesens tragen, ein
  veraltetes Token wird abgelehnt, und die neue `apps.json` wird geprüft, bevor etwas geschrieben wird. Ein
  abgelehnter Schreibvorgang lässt `apps.json` unberührt. Siehe [MCP-Tools](/de/automation/mcp-tools/#konfiguration).
- **App-IDs sind eingeschränkt.** Der MCP-Server akzeptiert nur Buchstaben, Ziffern, `.`, `_` und `-`,
  und nie ein führendes `-`, damit eine ID nicht als Befehlszeilen-Flag gelesen werden kann.
- **Screenshots gelten nur für Moonpool.** `moonpool_screenshot` erfasst eines von Moonpools eigenen sechs
  Fenstern (`main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`), nie den Bildschirm oder
  eine andere App. Das PNG wird im Arbeitsspeicher erzeugt und inline zurückgegeben; Moonpool speichert es nicht in einer
  Datei.
- **Keine Authentifizierung auf dem Kanal.** Moonpool fügt der Control-Pipe oder dem Socket keine Anmeldung und kein Token hinzu.
  Jeder Prozess, der sie öffnen kann, kann Verben senden. Unter Linux und macOS wird die Socket-Datei
  mit Modus `0600` angelegt, sodass nur Ihr eigener Benutzer sie öffnen kann.
- **Sandbox-Hosts werden erkannt.** Stellt der MCP-Server fest, dass er in einer verpackten
  (Store/MSIX-)Sandbox läuft, in der er eine private Kopie der Dateien von Moonpool sehen würde, geben die Tools, die
  Dateien lesen oder schreiben (`moonpool_app_output`, `moonpool_read_config`,
  `moonpool_write_config`, `moonpool_restore_config`), einen Fehler zurück, der den Grund erklärt, statt
  veraltete Daten zu liefern. Tools, die nur den Steuerkanal nutzen, werden nicht blockiert. Siehe
  [MCP-Einrichtung](/de/automation/mcp-setup/#hosts-in-einer-sandbox).

## Plattform

Den Steuerkanal gibt es auf jeder Plattform: eine Named Pipe unter Windows, einen Unix-Socket unter Linux
und macOS (Speicherort unter [Steuerverben](/de/automation/control-verbs/#wo-er-lauscht)). Nur
`screenshot` (und damit `moonpool_screenshot`) gibt es ausschließlich unter Windows; unter Linux und macOS liefert es
„not supported on this platform“. Die Verben der Befehlszeile funktionieren auf jeder Plattform.

## Siehe auch

- [KI-Agenten: Schnellstart](/de/automation/quick-start/)
- [MCP-Einrichtung](/de/automation/mcp-setup/)
