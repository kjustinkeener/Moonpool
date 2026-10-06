---
title: "Moonpool von einem USB-Stick oder synchronisierten Ordner ausführen"
description: "Halten Sie Moonpool und alle seine Daten in einem verschiebbaren Ordner, um es auf einem USB-Stick mitzunehmen oder zu synchronisieren, und starten Sie mehrere Kopien parallel."
---

Der portable Modus hält Moonpool und alles, was es schreibt, in einem einzigen Ordner `.moonpool\`. So
können Sie ihn auf einem USB-Stick mitnehmen oder in einem synchronisierten Ordner ablegen und auf jedem PC ausführen.

## So funktioniert es

Wenn Sie portabel installieren, legt Moonpool im gewählten Speicherort einen Ordner `.moonpool\` an. Dieser Ordner enthält
das Programm, Ihre Konfiguration und seine Hilfeinhalte. Nichts wird in das Windows-AppData
geschrieben, daher nimmt das Verschieben oder Kopieren des Ordners Ihr gesamtes Setup mit.

```text
<chosen location>\.moonpool\
```

## Was sich gegenüber der Installation unterscheidet

| | Installiert | Portabel |
| --- | --- | --- |
| Programm | `%USERPROFILE%\.moonpool\moonpool.exe` | `<chosen location>\.moonpool\moonpool.exe` |
| Konfigurationsordner | `%USERPROFILE%\.moonpool\moonpool-config\` | `<chosen location>\.moonpool\moonpool-config\` |
| Browserprofil, Größe und Position des Fensters | Im Konfigurationsordner | Im Konfigurationsordner, wandern also mit |
| Startmenü, Desktop-Verknüpfung, Add/Remove-Eintrag | Ja | Keine |
| Updates | Ersetzt die eigene EXE | Genauso, innerhalb des Ordners `.moonpool\`. Siehe [Aktualisieren](/de/data/updating/#portable-kopien). |
| Entfernen | Add/Remove Programs oder `--uninstall` | Ordner löschen |

Keiner der beiden Modi schreibt in das Windows-AppData.

### Synchronisierte Ordner

Sie können eine portable Kopie in einem synchronisierten Ordner (OneDrive, Dropbox und dergleichen) halten, sollten sie aber
immer nur auf einem PC gleichzeitig ausführen. Moonpool schreibt alle paar Sekunden `state.json` und protokolliert, während Apps
laufen; zwei PCs, die denselben Ordner ausführen, kollidieren also bei denselben Dateien, und ein Synchronisierungskonflikt kann
eine defekte `apps.json` hinterlassen. Beenden Sie Moonpool auf dem einen PC, bevor Sie es auf einem anderen starten.

## Mehrere Kopien gleichzeitig

Pro Ordner läuft ein Moonpool. Das installierte Moonpool und beliebig viele portable Kopien, jede
in ihrem eigenen Ordner, können gleichzeitig laufen, und jede ist vollständig getrennt: mit eigenen Apps, eigenem Tray-Symbol,
Fenster, eigenen Einstellungen, Protokollen und eigenem [Steuerkanal](/de/automation/control-verbs/).

- Der Tray-Tooltip und der Name in der Taskleiste zeigen, welche Kopie welche ist: `Moonpool` für das
  installierte, `Moonpool (<folder>)` für ein portables, wobei `<folder>` der von Ihnen
  gewählte Ordner ist (der, der `.moonpool\` enthält).
- Wird dieselbe Kopie ein zweites Mal gestartet, kommt ihr Fenster nach vorn, statt ein weiteres
  zu öffnen. Wird eine andere Kopie gestartet, öffnet sich diese Kopie.
- Wenn Sie einem KI-Agenten mehr als eine Kopie geben möchten, registrieren Sie jede unter einem eigenen Namen; siehe
  [MCP-Einrichtung](/de/automation/mcp-setup/#mehr-als-ein-moonpool).
- Das Verschieben oder Umbenennen eines portablen Ordners gibt ihm eine neue Identität (einen neuen Namen des Steuerkanals).
  Beenden Sie ihn, bevor Sie ihn verschieben.
- Kopien wissen nichts von den Apps der anderen. Zwei Kopien, die denselben Server auf
  demselben Port starten, kollidieren weiterhin, und ein Stoppen über Prozessname oder Port kann etwas beenden,
  das eine andere Kopie gestartet hat; siehe
  [Stoppen und Neustarten](/de/apps/stop-and-restart/#mehrere-moonpools-oder-ihre-eigenen-prozesse).

## Ihre Apps ebenfalls mitnehmen

Verwenden Sie das Token `{MP_HOME}` im Pfad einer App, damit er in den portablen Ordner zeigt statt
auf einen festen Ort auf einem Rechner. In einer portablen Kopie ist `{MP_HOME}` der Ordner,
der `moonpool.exe` enthält, also der Ordner `.moonpool\` selbst, nicht der Ordner, den Sie gewählt haben:

```json title="apps.json"
{ "cwd": "{MP_HOME}/my-app" }
```

Hier ist `{MP_HOME}/my-app` gleich `<chosen location>\.moonpool\my-app`. Ein Pfad, der mit
`./` beginnt, wird auf dieselbe Weise verankert. Token und `./`-Pfade funktionieren auch in einem installierten Moonpool.
Wie Pfade aufgelöst werden, steht unter [Pfade und Umgebung](/de/apps/paths-and-environment/).

## Portabel im Installer wählen

Der portable Modus wird auf der Installer-Karte eingerichtet, die **Portabel installieren**
neben **Moonpool installieren** anbietet.

![Die Installer-Karte: Der Link „Portabel installieren“ steht unter der Hauptschaltfläche „Moonpool installieren“](../../../../assets/screenshots/installer-window.png)

Wählen Sie einen Ordner, und Moonpool legt dort den Ordner `.moonpool\` an, kopiert sich hinein und
startet die neue Kopie mit einer frischen Konfiguration.

Die Karte finden Sie auch im Menü „...“ als **Moonpool installieren…**, sowohl im installierten als auch im
portablen Modus. Wenn Sie dort **Portabel installieren** wählen, beendet sich das laufende Moonpool, und an
seiner Stelle startet die neue portable Kopie. Das Moonpool, von dem Sie ausgegangen sind, bleibt dort, wo es
war, sodass Sie es danach wieder starten können.

Eine portable Kopie beginnt frisch und übernimmt Ihre vorhandenen Apps nicht. Um sie zu übertragen,
beenden Sie die portable Kopie und kopieren `apps.json` von Hand:

| | Pfad |
| --- | --- |
| Von (installiert) | `%USERPROFILE%\.moonpool\moonpool-config\apps.json` |
| Nach (portabel) | `<chosen location>\.moonpool\moonpool-config\apps.json` |

Einträge mit absoluten Pfaden funktionieren auf demselben PC weiter, wandern aber nicht mit. Der Dialog „App bearbeiten“
kennzeichnet sie mit „nicht portabel“.

## Woran Moonpool erkennt, dass es portabel ist

Eine Kopie ist portabel, solange eine Datei namens `moonpool.portable` neben ihrer `moonpool.exe` liegt.
Nichts anderes kennzeichnet sie, und nichts wird bei Windows registriert.

Um eine portable Kopie zu entfernen, beenden Sie sie und löschen ihren Ordner `.moonpool\`. `--uninstall` entfernt
nur das installierte Moonpool, nie eine portable Kopie.
