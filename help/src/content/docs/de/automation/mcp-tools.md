---
title: "Referenz der Moonpool-MCP-Tools: Parameter und Ergebnisse"
description: "Alle Tools, die der Moonpool-MCP-Server für Agenten bereitstellt, mit Parametern, Rückgabewerten und den Fehlerfällen, die auftreten können."
---

Alle Tools liefern Text, mit Ausnahme von `moonpool_screenshot`, das ein PNG-Bild zurückgibt. Ein Fehler
kommt als Tool-Ergebnis, das als Fehler markiert ist, mit dem Grund als Text. Zur Einrichtung siehe
[MCP-Einrichtung](/de/automation/mcp-setup/).

Tools mit `app_id` brauchen die `id` der App aus `apps.json`. Sie darf nur Buchstaben,
Ziffern, `.`, `_` und `-` enthalten und nicht mit `-` beginnen, sonst schlägt der Aufruf mit „invalid
app_id“ fehl.

Die meisten Tools, die auf den Hub wirken, schlagen mit dieser Meldung fehl, wenn er nicht läuft.
`moonpool_bootup_launcher`, `moonpool_shutdown_launcher`, `moonpool_raise_launcher` und
`moonpool_launcher_paths` behandeln diesen Fall selbst (siehe ihre Zeilen). Bei einer portablen Kopie
nennt die Meldung die Kopie, zum Beispiel `Moonpool (<folder>)`.

```text
Moonpool is not running - call moonpool_bootup_launcher first
```

Aufrufe, die auf ein Ergebnis warten, laufen nach 45 Sekunden in ein Zeitlimit.

## Launcher und Apps

Beispielergebnis von `moonpool_list_apps`:

```text
site  [running] (managed by Moonpool)  Site
notes-app  [stopped]  [mcp: stopped]  Notes App
```

| Tool | Parameter | Verhalten |
| --- | --- | --- |
| `moonpool_list_apps` | keine | Eine Zeile pro App: `id  [running]` oder `[stopped]`, `(managed by Moonpool)`, wo zutreffend, `[mcp: running]` oder `[mcp: stopped]`, wenn ein MCP-Helfer gesehen wurde, dann der Name. Wird über den Steuerkanal (Verb `list`) beim laufenden Hub abgefragt und ist daher live. Läuft Moonpool nicht, schlägt es mit „Moonpool is not running“ fehl, statt eine veraltete Liste zu zeigen. Direkt nach dem Start von Moonpool, vor der ersten Statusprüfung, zeigen Apps `[status pending]`. Solange `apps.json` einen Fehler hat, beginnt das Ergebnis mit `apps.json has an error: <message>. This list is the last one that loaded; fix the file and call moonpool_reload_config.` War die Datei schon beim Start von Moonpool defekt, steht dort, dass keine Apps geladen sind, und zusätzlich der Hinweis auf `moonpool_restore_config`. |
| `moonpool_bootup_launcher` | keine | Startet Moonpool selbst und wartet bis zu 30 s, bis sein Steuerkanal antwortet. Liefert „Moonpool started“ oder „Moonpool is already running“. Beendet sich der neue Prozess sofort (er hat an ein Moonpool übergeben, das noch herunterfuhr), startet es noch einen. Hält etwas den Kanal, ohne zu antworten, meldet es, dass ein Moonpool-Prozess möglicherweise hängt. |
| `moonpool_shutdown_launcher` | keine | Dasselbe wie **Beenden** im Tray-Menü. Wartet bis zu 30 s, bis der Steuerkanal verschwindet. Liefert „Moonpool shut down“ oder „Moonpool is not running“. |
| `moonpool_raise_launcher` | keine | Bringt das Moonpool-Fenster in den Vordergrund. Liefert „window shown“. Läuft Moonpool nicht, wird es gestartet und „Moonpool was not running; started it“ geliefert. |
| `moonpool_start_app` | `app_id` (erforderlich) | Startet die App und öffnet ihren Terminal-Tab. Liefert „launched“, sobald sie läuft, oder den Grund, warum nicht (`unknown app id: <id>`, `did not reach running in time` nach 25 s). Bei einem `static`-Eintrag mit nur einer `url` öffnet es die Seite und liefert ebenfalls „launched“. |
| `moonpool_stop_app` | `app_id` (erforderlich) | Stoppt die App. Liefert „stopped“ oder einen Fehler wie `still running after stop` (nach 15 s). |
| `moonpool_restart_app` | `app_id` (erforderlich) | Stoppen, warten, bis Port und Prozess frei sind, starten. Liefert „restarted“. |
| `moonpool_app_output` | `app_id` (erforderlich), `tail_lines` (Ganzzahl, Standard 200, Minimum 1) | Die Terminalausgabe der App für die aktuelle Moonpool-Sitzung, ANSI-Codes entfernt. Ist das Protokoll länger als `tail_lines`, beginnt der Text mit einer Zeile, die den Pfad des vollständigen Protokolls nennt. Schlägt mit `no console output recorded for '<id>' (not launched this session)` fehl, wenn die App nicht lief. Existiert das Protokoll, ist aber leer, liefert es `(no output recorded for '<id>')`. |
| `moonpool_stop_mcp_server` | `app_id` (erforderlich) | Beendet den angebundenen MCP-Helferprozess der App und lässt die App laufen. Liefert „stopped“. Tut nichts, wenn die App weder `processName` noch `mcpProcessName` hat. |
| `moonpool_refresh_app_icons` | keine | Lädt jedes App-Symbol neu. Liefert „icons refreshed“. |

## Konfiguration

Diese Tools lesen und ändern `apps.json` über den Hub, nie die Datei auf dem Datenträger. Ein Schreibvorgang muss
den Token des letzten Lesens mitbringen, ein veralteter Token wird abgelehnt, und die neue Datei wird geprüft,
bevor etwas geschrieben wird. Der Weg über den Hub ist wichtig, weil einem Agenten in einem Sandbox-Host
eine private Kopie des Konfigurationsordners statt des echten angezeigt werden kann.

| Tool | Parameter | Verhalten |
| --- | --- | --- |
| `moonpool_read_config` | keine | JSON-Text mit `manifest_text` (der genaue Dateiinhalt), `token`, `valid`, `error` (null, wenn gültig) und `path`. `token` ist `none`, wenn die Datei fehlt oder leer ist. |
| `moonpool_write_config` | `manifest` (erforderlich, der vollständige neue `apps.json`-Text), `expected_token` (erforderlich, vom letzten Lesen) | Prüft das Manifest und ersetzt `apps.json`, dann wird sie geladen. Liefert `apps.json updated; new version token <token>`. Ein veralteter Token schlägt mit `stale token: apps.json changed since it was read ...` fehl. Ein ungültiges Manifest schlägt mit `rejected invalid manifest: ...` fehl. In beiden Fällen bleibt die Datei unberührt. Ein leerer `expected_token` wird abgelehnt. |
| `moonpool_restore_config` | `snapshot` (optional) | Ohne Wert ein JSON-Text, der die gespeicherten Snapshots mit dem neuesten zuerst auflistet (`index`, `filename`, `millis`, `app_count`, `valid`). Mit einem Index (1 = neuester) oder einem Dateinamen wird dieser Snapshot geprüft und wiederhergestellt. Liefert `restored <file> (<n> apps); new version token <token>`. Es ist kein Token nötig: Eine Wiederherstellung überschreibt die aktuelle Datei absichtlich. |
| `moonpool_reload_config` | keine | Liest `apps.json` neu ein. Liefert „apps.json reloaded“. Lässt sich die Datei nicht parsen oder prüfen, schlägt es mit `apps.json has an error: ...` fehl, und Moonpool behält die zuletzt geladene Liste. |
| `moonpool_launcher_paths` | keine | Listet Konfigurationsordner, `apps.json`, `state.json`, Protokoll, Dumps-Ordner, Symbolordner, Portabel-Kennzeichen und EXE-Pfad des Hubs auf, danach Konfigurationsordner, `apps.json`, `state.json`, Dumps-Ordner, Portabel-Kennzeichen und EXE-Pfad des MCP-Prozesses (ohne Protokoll und Symbole). Läuft der Hub nicht, lautet seine Hälfte `hub paths unavailable: ...`, und die MCP-Hälfte wird trotzdem gezeigt. Nutzen Sie es, wenn eine Änderung nicht wirkt. |

## Erweitert: Testwerkzeuge

`moonpool_screenshot` gibt es nur unter Windows; unter Linux und macOS schlägt es mit „screenshot is not
supported on this platform“ fehl. `moonpool_window_state` und `moonpool_reset_mcp_seen` funktionieren auf
allen Plattformen.

`window` ist eines von `main`, `settings`, `about`, `installer`, `editor`, `help` oder `themes` und ist standardmäßig
`main`. Ein unbekannter Name schlägt mit `unknown window '<name>'` fehl.

| Tool | Parameter | Verhalten |
| --- | --- | --- |
| `moonpool_screenshot` | `window` (optional) | Erfasst den eigenen Inhalt dieses Moonpool-Fensters als eingebettetes PNG, höchstens 320 Pixel auf der längeren Seite. Die Größe lässt sich über MCP nicht erhöhen. Schlägt mit `window '<name>' is not open` fehl, wenn es nicht angezeigt wird. Andere Apps kann es nicht erfassen. |
| `moonpool_window_state` | `window` (optional) | JSON-Text: `{"open":false}`, wenn das Fenster nicht geöffnet ist, sonst `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. Für Tests gedacht. |
| `moonpool_reset_mcp_seen` | `app_id` (optional) | Nur zum Testen. Löscht den gemerkten Eintrag „ein MCP-Helfer wurde gesehen“ für eine App, oder für alle Apps, wenn weggelassen, sodass die MCP-Unterzeile der Seitenleiste wieder verschwindet, bis ein Helfer gesehen wird. |

## Siehe auch

- [MCP-Einrichtung](/de/automation/mcp-setup/)
- [Befehlszeile](/de/automation/command-line/)
