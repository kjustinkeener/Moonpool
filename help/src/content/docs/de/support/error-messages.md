---
title: "Moonpool-Fehlermeldungen erklärt: already running, requires a command und mehr"
description: "Den genauen Wortlaut von Moonpool-Fehlermeldungen wie already running, requires a command, stale token und Update failed nachschlagen, mit Bedeutung und Lösung."
---

Fügen Sie die angezeigte Meldung in die Seitensuche ein oder überfliegen Sie die Tabellen.
Meldungen sind so zitiert, wie Moonpool sie anzeigt. Text in `<spitzen Klammern>` wird durch
einen Wert ersetzt (eine App-ID, einen Pfad oder einen Fehler des Systems). Symptome, die
keine Fehlermeldung sind, stehen unter
[Fehlerbehebung und FAQ](/de/support/troubleshooting/).

## Eine App starten und stoppen

| Meldung | Bedeutung und Lösung |
| --- | --- |
| `already running` | Moonpool hält bereits ein Terminal für diese App. Stoppen Sie sie zuerst oder nutzen Sie „Neu starten“. |
| `stopped during launch` | „Stoppen“ wurde gedrückt, während der Start noch lief. Starten Sie erneut. |
| `app has no launch command` | Der Eintrag hat keinen `command`. Fügen Sie einen im App-Editor oder in `apps.json` hinzu. Nur ein `static`-Eintrag mit einer `url` kommt ohne aus. |
| `unknown app: <id>` | Es ist keine App mit dieser `id` geladen. Prüfen Sie die ID und wählen Sie „Neu laden“, wenn Sie `apps.json` von Hand bearbeitet haben. |
| `unknown app id: <id>` | Dasselbe Problem, gemeldet an ein Skript oder einen Agenten. Listen Sie die Apps mit `moonpool_list_apps` auf. |
| `did not reach running in time` | Aus einem Skript oder von einem Agenten: Die App wurde innerhalb von 25 Sekunden nicht als „läuft“ erkannt. Prüfen Sie `port` oder `processName` und lesen Sie die Ausgabe. Siehe [Der Statuspunkt ist falsch](/de/support/troubleshooting/#der-statuspunkt-ist-falsch). |
| `still running after stop` | Nach 15 Sekunden gilt die App weiterhin als „läuft“. Setzen Sie `killMode`. Siehe [Stoppen und Neustarten](/de/apps/stop-and-restart/). |
| `refusing to open non-web url: <url>` | Die `url` beginnt nicht mit `http://`, `https://`, `mailto:` oder `file://`. Korrigieren Sie die `url`. |
| `[process exited]` | Kein Fehler: Der Befehl der App ist beendet. Wird im Terminal-Tab angezeigt (in der App-Oberfläche als „[Prozess beendet]“). |

## Validierung von apps.json

Moonpool lehnt eine `apps.json` ab, die eine Regel verletzt, und behält die zuletzt geladene
Liste. `<n>` ist die Position des Eintrags in der Datei, ab 1 gezählt.

| Meldung | Lösung |
| --- | --- |
| `apps.json entry <n> has invalid id "<id>"; use letters, digits, '.', '_', and '-' without a leading '-'` | Benennen Sie die `id` um. |
| `duplicate app id "<id>"` | Zwei Einträge teilen sich eine `id`. Machen Sie jede eindeutig. |
| `apps.json entry <n> (<id>) has an empty name` | Füllen Sie `name` aus. |
| `apps.json entry <n> (<id>) has an empty group` | Füllen Sie `group` aus. |
| `apps.json entry <n> (<id>) has unknown type "<type>"` | `type` muss `web`, `desktop`, `static` oder `cli` sein. |
| `apps.json entry <n> (<id>) has invalid port 0` | `port` muss zwischen 1 und 65535 liegen. |
| `apps.json entry <n> (<id>) requires a url` | Ein `static`-Eintrag braucht eine `url`. |
| `apps.json entry <n> (<id>) requires a command` | Jeder andere Typ braucht einen `command`. |

Wenn Sie im App-Editor ohne Namen speichern, erscheint „Name ist erforderlich.“ (englisch
`name is required.`). Der Banner-Text „apps.json enthält einen Fehler, angezeigt wird die
zuletzt geladene Liste.“ oder „apps.json enthält einen Fehler, daher sind keine Apps
geladen.“ und die Wiederherstellung stehen unter
[apps.json enthält einen Fehler](/de/support/troubleshooting/#appsjson-enthält-einen-fehler). Wenn
der Banner sagt, das Speichern sei pausiert, endet die Meldung mit `Repair apps.json and
reload it before saving from Moonpool`. Die vollständige Regelliste steht unter
[Validierung](/de/apps/apps-json/#prüfung).

## Einstellungen, Updates und Installationsprogramm

| Meldung | Bedeutung und Lösung |
| --- | --- |
| `... Repair settings.json and restart Moonpool before changing settings` | `settings.json` ist fehlerhaft. Korrigieren oder löschen Sie sie und starten Sie neu. Siehe [settings.json](/de/data/settings-json/#lesen-und-reparieren). |
| `Update failed: <error>` | Der Download oder die Installation eines Updates ist fehlgeschlagen (Update fehlgeschlagen: `<error>`). Siehe [Wenn ein Update fehlschlägt](/de/data/updating/#wenn-ein-update-fehlschlägt). |
| `Update check failed: <error>` | Die Update-Suche unter „Über“ ist fehlgeschlagen (Update-Suche fehlgeschlagen: `<error>`). Der Text nach dem Doppelpunkt nennt den Grund. Versuchen Sie es später erneut. |
| `Install failed: <error>` | Das Installationsprogramm ist bei dem nach dem Doppelpunkt genannten Schritt stehen geblieben, zum Beispiel `copy exe: ...`. Beenden Sie jedes Moonpool, das aus `%USERPROFILE%\.moonpool` läuft, und versuchen Sie es erneut. |
| `target folder does not exist` | Der für eine portable Kopie gewählte Ordner existiert nicht mehr. Wählen Sie einen vorhandenen. |

## MCP und Skripte

| Meldung | Bedeutung und Lösung |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Starten Sie Moonpool, oder lassen Sie den Agenten dieses Tool aufrufen. Bei einer portablen Kopie nennt die Meldung die Kopie. |
| `frontend not loaded` | Das Hub-Fenster ist noch nicht fertig geladen. Warten Sie und versuchen Sie es erneut. |
| `invalid app_id: use only letters, digits, '.', '_', '-' (no leading '-')` | Der Agent hat eine ID übergeben, die der MCP-Server nicht akzeptiert. Verwenden Sie die ID aus `moonpool_list_apps`. |
| `stale token: apps.json changed since it was read ...` | Lesen Sie `apps.json` erneut, wenden Sie die Änderung noch einmal an und schreiben Sie dann. |
| `rejected invalid manifest: ...` | Die neue `apps.json` hat die Validierung nicht bestanden (siehe oben). Die Datei wurde nicht geändert. |
| `no console output recorded for '<id>' (not launched this session)` | Für `moonpool_app_output` wurde eine App angefragt, die seit dem Start von Moonpool nicht gelaufen ist. |

Mehr unter [MCP-Tools](/de/automation/mcp-tools/) und
[MCP-Einrichtung](/de/automation/mcp-setup/#wenn-die-tools-nicht-funktionieren).

## Fehler anderer Programme

- [`Error: listen EADDRINUSE: address already in use :::3000` und `Port 5173 is in use`](/de/support/port-already-in-use/)
- [`Windows protected your PC`](/de/support/windows-protected-your-pc/)
- [WebView2-Runtime fehlt](/de/support/webview2-runtime-missing/)
