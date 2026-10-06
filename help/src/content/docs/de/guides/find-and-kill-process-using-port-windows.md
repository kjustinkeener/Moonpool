---
title: "Den Prozess finden und beenden, der unter Windows einen Port belegt (3000, 5173, 8080)"
description: "Unter Windows mit netstat oder PowerShell herausfinden, welcher Prozess Port 3000 oder 5173 belegt, ihn mit taskkill beenden und Moonpool den Port beim Stoppen freigeben lassen."
---

Wenn ein Entwicklungsserver mit der Meldung scheitert, dass der Port bereits belegt ist,
lauscht etwas anderes auf diesem Port. Listen Sie in der Eingabeaufforderung die lauschenden
Prozesse mit der Prozess-ID des Besitzers auf und beenden Sie ihn dann:

```text frame="terminal"
netstat -ano | findstr :3000
taskkill /PID 12345 /F
```

Die letzte Spalte der Zeile `LISTENING` ist die PID (`findstr :3000` findet auch `:30001`,
lesen Sie also die lokale Adresse). `tasklist /FI "PID eq 12345"` zeigt, um welches Programm
es sich handelt. In PowerShell lautet dieselbe Abfrage:

```powershell frame="terminal"
Get-NetTCPConnection -LocalPort 3000 -State Listen | Select-Object LocalPort, OwningProcess
Get-Process -Id 12345
Stop-Process -Id 12345 -Force
```

Fügen Sie `/T` zu `taskkill` hinzu, um auch die untergeordneten Prozesse zu beenden. Prozesse,
die einem anderen Benutzer oder dem System gehören, erfordern möglicherweise ein Fenster mit
erhöhten Rechten (Administrator).

## Der Moonpool-Weg

Bei einer App, die Sie über Moonpool ausführen, müssen Sie die PID nicht nachschlagen. Geben
Sie der App einen `port`, und „Stoppen“ gibt ihn frei. Bei einer `web`-App ist das der
Standard-`killMode`, hier ausgeschrieben:

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

„Stoppen“ beendet zuerst das von Moonpool gestartete Terminal und beendet dann erzwungen
alles, was noch auf `port` lauscht. Unter Windows ist das dieselbe Abfrage wie oben
(`Get-NetTCPConnection -LocalPort <port> -State Listen`), gefolgt von
`taskkill /PID <pid> /T /F` für jeden Besitzer.

- Wenn etwas, das Sie nicht gestartet haben, den Port hält, zeigt Moonpool die App als
  laufend, aber nicht als „managed by Moonpool“. Drücken Sie bei ihr **Stoppen**: Der Schritt
  für `port` wird trotzdem ausgeführt.
- Moonpool weigert sich, eine feste Liste gemeinsam genutzter Windows-Prozesse über den Port
  zu beenden, etwa das Backend von Docker Desktop, `svchost` und den WSL-Host. Verwenden Sie
  für eine Docker-App `killMode` `command` oder `none`, niemals `port`. Siehe
  [Docker-Apps unter Windows](/de/apps/stop-and-restart/#docker-apps-unter-windows).
- Das funktioniert nur für Ports von Apps, die in `apps.json` aufgeführt sind. Für jeden
  anderen Port verwenden Sie die Befehle am Anfang.
- Der Modus `port` beendet alles, was lauscht, auch eine Kopie, die Sie von Hand gestartet
  haben, verwenden Sie ihn also nur für Ports, die nichts anderes auf dem Rechner braucht.

## Siehe auch

- [EADDRINUSE und „Port 5173 is in use“ beheben](/de/support/port-already-in-use/)
- [Stoppen und Neustarten](/de/apps/stop-and-restart/)
- [App-Felder](/de/apps/fields/): `port` und `killMode`.
- [Zwei Apps verwenden denselben Port](/de/support/troubleshooting/#zwei-apps-verwenden-denselben-port)
