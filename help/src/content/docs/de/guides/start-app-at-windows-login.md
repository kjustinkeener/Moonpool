---
title: "Ein Skript oder einen Entwicklungsserver automatisch beim Windows-Login starten"
description: "Moonpool beim Windows-Login per Verknüpfung im Autostart-Ordner starten und darin mit einem kleinen PowerShell-Skript einen Entwicklungsserver oder ein Skript starten."
---

Windows kennt zwei übliche Wege, etwas beim Login zu starten: eine Verknüpfung in Ihrem
Autostart-Ordner (drücken Sie Win+R, geben Sie `shell:startup` ein, drücken Sie Enter) oder
eine Aufgabe der Aufgabenplanung mit dem Trigger „At log on“ (Bei Anmeldung). Beide führen
ein Programm oder Skript aus, das auch direkt der Befehl Ihres Entwicklungsservers sein
könnte, aber dann verfolgt nichts seinen Zustand, zeigt seine Ausgabe an oder stoppt ihn für
Sie.

## Was Moonpool bietet

Moonpool hat keine Einstellung zum Start beim Login, und ein Eintrag in `apps.json` hat kein
Feld, das ihn beim Start von Moonpool startet (die vollständige Liste steht unter
[App-Felder](/de/apps/fields/) und [settings.json](/de/data/settings-json/)). Sie können
Moonpool aber selbst beim Login starten und dann ein Skript die gewünschten Apps starten
lassen, mit demselben Verb, das die [Befehlszeile](/de/automation/command-line/) bietet.

Registrieren Sie die App zuerst wie gewohnt:

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

Speichern Sie dann Folgendes als `start-moonpool-apps.ps1`. Bei einer Installation ist das
Programm `%USERPROFILE%\.moonpool\moonpool.exe`; bei einer portablen Kopie verwenden Sie den
Pfad der EXE dieser Kopie.

```powershell title="start-moonpool-apps.ps1"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
Start-Process $mp
Start-Sleep -Seconds 15
& $mp launch site
```

Moonpool muss bereits laufen, damit `launch` an es übergeben werden kann; läuft nichts, startet
derselbe Befehl ein neues Moonpool, und das Verb wird nicht ausgeführt. Die Verzögerung gibt
ihm Zeit zum Starten, erhöhen Sie sie daher auf einem langsamen Rechner. Fügen Sie pro App
eine Zeile `& $mp launch <id>` hinzu.

Legen Sie schließlich eine Verknüpfung zum Skript im Autostart-Ordner an, mit diesem Ziel:

```text title="Shortcut target"
powershell.exe -NoProfile -WindowStyle Hidden -File "C:\Users\you\start-moonpool-apps.ps1"
```

Um zu prüfen, was passiert ist, hängen Sie `--ticket t1` an ein Verb an und lesen Sie das
Ergebnis aus `state.json`
([Das Ergebnis lesen](/de/automation/command-line/#das-ergebnis-lesen)).

## Hinweise

- Ein so gestarteter Entwicklungsserver wird von Moonpool wie jeder andere „verwaltet“, sodass
  „Stoppen“ und Beenden bei ihm funktionieren. Läuft dieselbe App bereits (zum Beispiel von
  Hand gestartet), zeigt Moonpool sie als laufend, aber nicht verwaltet.
- Moonpool startet eine App, die sich beendet, nicht neu und merkt sich nicht, welche Apps
  liefen, als Sie zuletzt beendet haben.

## Siehe auch

- [Befehlszeile](/de/automation/command-line/)
- [Tray, Schließen und Minimieren](/de/using/tray-and-closing/)
- [Einen npm-Entwicklungsserver unter Windows im Hintergrund ausführen](/de/guides/run-npm-dev-server-in-background-windows/)
