---
title: "Ein Python-Skript unter Windows im Hintergrund weiterlaufen lassen"
description: "Ein langlebiges Python-Skript oder eine kleine Web-App unter Windows im Hintergrund ausführen, die Ausgabe sehen und sauber stoppen, mit pythonw und mit Moonpool."
---

Ein Python-Skript, das aus einem Konsolenfenster gestartet wurde, endet, wenn Sie das Fenster
schließen. Die üblichen Windows-Lösungen sind `pythonw.exe` (derselbe Interpreter ohne
Konsolenfenster, sodass die Ausgabe ins Leere geht), `Start-Process pythonw -ArgumentList worker.py`
zum losgelösten Starten oder eine geplante Aufgabe für etwas, das beim Login oder nach einem
Zeitplan laufen soll. Bei jeder müssen Sie den Prozess im Task-Manager suchen, wenn Sie ihn
loswerden wollen.

## Der Moonpool-Weg

Moonpool führt den Befehl in einem eigenen Terminal-Tab aus, sodass Sie die Ausgabe und eine
Stopp-Schaltfläche behalten, ohne ein eigenes Konsolenfenster. Für ein Skript, das läuft, bis
Sie es stoppen, verwenden Sie eine `cli`-App. `-u` lässt Python die Ausgabe sofort leeren,
sodass der Tab sie live zeigt:

```json title="apps.json"
{
  "id": "worker",
  "name": "Queue worker",
  "group": "Scripts",
  "type": "cli",
  "cwd": "C:\\code\\worker",
  "command": ".venv\\Scripts\\python.exe -u worker.py"
}
```

Starten Sie sie und klicken Sie auf den Namen der App, um ihre Ausgabe zu verfolgen. Eine
`cli`-App gilt als „läuft“, solange ihr Befehl läuft, und wird grau, wenn das Skript endet,
wobei `[process exited]` im Tab stehen bleibt. **Stoppen** beendet das Skript und alles, was
es gestartet hat. Wenn Sie die `python.exe` der virtuellen Umgebung per Pfad verwenden,
braucht es keinen Aktivierungsschritt.

Wenn das Skript HTTP bedient (Flask, FastAPI, `python -m http.server`), machen Sie es zu
einer `web`-App, damit „läuft“ seinem Port folgt:

```json title="apps.json"
{
  "id": "docs-api",
  "name": "Docs API",
  "group": "Scripts",
  "type": "web",
  "cwd": "C:\\code\\docs-api",
  "command": ".venv\\Scripts\\python.exe -u app.py",
  "port": 8091,
  "url": "http://127.0.0.1:8091",
  "env": { "PORT": "8091" }
}
```

## Grenzen

- Lassen Sie Moonpool laufen. Das Schließen seines Fensters beendet es standardmäßig, und
  unter Windows stoppt das Beenden jede App, die es gestartet hat. Schalten Sie **Beim
  Schließen in den Infobereich** ein, um das Fenster stattdessen auszublenden; siehe
  [Tray, Schließen und Minimieren](/de/using/tray-and-closing/).
- Moonpool startet ein abgestürztes Skript nicht neu und startet es auch nicht von selbst
  beim Windows-Login. Siehe
  [Ein Skript oder einen Entwicklungsserver automatisch beim Windows-Login starten](/de/guides/start-app-at-windows-login/).
- Vermeiden Sie verschachtelte doppelte Anführungszeichen in `command`: Der Wrapper `cmd /c`
  verstümmelt sie.

## Siehe auch

- [App-Typen](/de/apps/types/#cli): wie `cli`- und `web`-Apps verfolgt werden.
- [Stoppen und Neustarten](/de/apps/stop-and-restart/)
- [Beispiele](/de/apps/examples/)
- [Protokolle](/de/data/logs/): wo die Sitzungsausgabe aufbewahrt wird.
