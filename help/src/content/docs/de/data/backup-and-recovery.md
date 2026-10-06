---
title: "Moonpool sichern, apps.json zurücksetzen und ein Setup wiederherstellen"
description: "Was Sie sichern sollten, wie Sie eine fehlerhafte apps.json zurückrollen, auf die Beispiel-Apps zurücksetzen, ein Setup in eine portable Kopie übernehmen und was die Deinstallation entfernt."
---

Alles, was Moonpool speichert, liegt an zwei Orten: im Konfigurationsordner und im Ordner für Dashboards.
Die Pfade für jeden Modus finden Sie unter [Wo die Konfiguration liegt](/de/apps/apps-json/#wo-die-konfiguration-liegt).

## Der Konfigurationsordner

```text
moonpool-config\
  apps.json            your apps                              back up
  apps.json.history\   the last 10 good apps.json files       back up (optional)
  settings.json        app settings                           back up
  icons\               icon overrides, <id>.png and so on     back up
  cli-output\<id>\     session logs                           disposable
  moonpool.log         debug log                              disposable
  state.json           live status snapshot                   disposable
  dumps\               files written by dump and read-config  disposable
  mcp_seen.json        which apps had an MCP helper           disposable
  window-state.json    hub window size and position           disposable
  AI-README.md         rewritten at every launch              disposable
  webview\             the window's browser profile (Windows) disposable
```

Der Ordner für Dashboards ist `{MP_HOME}\dashboards`: `%USERPROFILE%\.moonpool\dashboards`
bei der Installation, `<your .moonpool folder>\dashboards` im portablen Modus und `dashboards/` im
Konfigurationsordner unter Linux. Sichern Sie darin alles, was von Ihnen stammt. Der Unterordner `examples`
gehört zu Moonpool und wird bei einem Update neu geschrieben.

Das Design wird im Browser-Speicher des Fensters abgelegt, nicht in einer Datei, die Sie kopieren können. Es
wandert nicht mit einer Sicherung mit; wählen Sie es nach einer Wiederherstellung erneut aus.

## Sichern

1. Beenden Sie Moonpool, damit keine Datei nur halb geschrieben ist.
2. Kopieren Sie `apps.json`, `settings.json` und `icons\` aus dem Konfigurationsordner sowie Ihre eigenen Dateien
   aus `dashboards\`.

```powershell frame="terminal"
$cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
Copy-Item "$cfg\apps.json", "$cfg\settings.json" D:\backup\
Copy-Item "$cfg\icons" D:\backup\ -Recurse
```

Zum Wiederherstellen beenden Sie Moonpool, kopieren die Dateien zurück und starten es.

## apps.json zurückrollen

Jedes erfolgreiche Speichern, jeder Schreibvorgang eines Agenten und jede Wiederherstellung sowie jedes
**Neu laden**, das geänderten Inhalt findet, kopiert die geprüfte `apps.json` nach `apps.json.history\`;
die neuesten 10 bleiben erhalten. Jede Datei ist nach dem Zeitpunkt ihrer Erstellung benannt, zum Beispiel
`1767225600000.json`. Eine `apps.json.bak` gibt es nicht.

- **Von Hand.** Kopieren Sie einen Snapshot über `apps.json` und wählen Sie dann **Neu laden**.

  ```powershell frame="terminal"
  $cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
  Copy-Item "$cfg\apps.json.history\<snapshot>" "$cfg\apps.json"
  ```

- **Aus einem Skript.** `moonpool.exe restore-config` listet die Snapshots auf;
  `moonpool.exe restore-config 1` stellt den neuesten wieder her. Siehe
  [Befehlszeile](/de/automation/command-line/).
- **Von einem Agenten.** `moonpool_restore_config`. Siehe [MCP-Tools](/de/automation/mcp-tools/#konfiguration).

Nichts wird automatisch wiederhergestellt.

## Eine fehlerhafte Datei

- **apps.json.** Moonpool überschreibt eine fehlerhafte Datei nie. Siehe
  [Wenn die Datei fehlerhaft ist](/de/apps/apps-json/#wenn-die-datei-fehlerhaft-ist).
- **settings.json.** Korrigieren Sie sie oder löschen Sie sie, um alle Einstellungen zurückzusetzen, und starten Sie
  Moonpool dann neu. Siehe [settings.json](/de/data/settings-json/#lesen-und-reparieren).

## Auf die Beispiele zurücksetzen

Moonpool schreibt seine Beispiel-Apps nur, wenn keine `apps.json` vorhanden ist. Um neu anzufangen, beenden Sie
Moonpool (oder lassen Sie es laufen), benennen Sie `apps.json` um oder löschen Sie sie und starten Sie Moonpool
dann oder wählen Sie **Neu laden**. Es wird eine neue `apps.json` mit den Beispielen geschrieben.

## Von installiert zu portabel

Eine neue portable Kopie beginnt mit den Beispiel-Apps. Wie Sie Ihre eigenen übernehmen, steht unter
[Portabler Modus](/de/data/portable-mode/#portabel-im-installer-wählen). Kopieren Sie `icons\` und
`settings.json` auf dieselbe Weise, wenn Sie sie ebenfalls möchten.

## Deinstallieren

Die Deinstallation des installierten Moonpool löscht den gesamten Ordner `%USERPROFILE%\.moonpool`,
einschließlich des Konfigurationsordners und der Dashboards. Sichern Sie vorher. Siehe
[Deinstallieren](/de/getting-started/install/#deinstallieren). Eine portable Kopie entfernen Sie, indem Sie ihren
Ordner `.moonpool\` löschen.
