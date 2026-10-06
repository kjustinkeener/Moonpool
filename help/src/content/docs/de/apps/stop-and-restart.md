---
title: "Einen Entwicklungsserver und alles, was er gestartet hat, stoppen"
description: "Lassen Sie Stoppen und Neu starten eine App und ihre Kindprozesse mit killMode und stopCommand sauber beenden, samt Standards je Typ und Docker unter Windows."
---

Stoppen tut immer zuerst Folgendes: Moonpool beendet das Terminal, das es für die App gestartet hat, samt
allem, was dieses Terminal gestartet hat. Für viele Apps genügt das.

Manche Apps überleben dieses Terminal (ein Desktop-Fenster löst sich von dem Entwicklungsserver, der es
gestartet hat, oder ein Server-Unterprozess hält weiter seinen Port). **`killMode`** wählt
einen zusätzlichen Schritt, der danach läuft.

| `killMode` | Zusätzlicher Schritt beim Stoppen | Liest | Standard für |
| --- | --- | --- | --- |
| `processName` | Beendet zwangsweise jeden Prozess mit diesem Namen. Unter Windows auch dessen Kindprozesse (`taskkill /IM <name>.exe /T /F`). Sonst `pkill -KILL -x <name>`: ein exakter Namensabgleich mit Beachtung der Groß-/Kleinschreibung, Kindprozesse nicht eingeschlossen. | `processName` | `desktop` |
| `port` | Beendet zwangsweise den Prozess, der auf `port` lauscht. | `port` | `web` |
| `command` | Führt `stopCommand` in `cwd` aus und wartet, bis er fertig ist. | `stopCommand`, `cwd`, `env` | nichts |
| `none` | Nichts. | nichts | `static`, `cli` |

Lassen Sie `killMode` weg, um den Standard für den Typ der App zu erhalten, und setzen Sie es nur, wenn Stoppen
etwas weiterlaufen lässt.

![Die Auswahl killMode im Dialog „App bearbeiten“, auf „default (nach Typ)“ gesetzt, mit ihrer Hinweiszeile, die den Standard je Typ auflistet](../../../../assets/screenshots/edit-app-killmode.png)

1. Die Auswahl `killMode`. „default (nach Typ)“ entspricht dem Weglassen des Schlüssels.

- Ist das Feld, das der Modus braucht, leer (zum Beispiel Modus `port` ohne `port`), wird der zusätzliche
  Schritt übersprungen. Das ist kein Fehler.
- `killMode` ist unabhängig von `type`: `port` funktioniert bei einer `cli`-App, `processName` bei einer
  `web`-App.
- Eine leere Zeichenfolge oder ein unbekannter Wert tut nichts zusätzlich. Es wird nicht auf den
  Standard des Typs zurückgegriffen.

Bei einer Desktop-App führt der Modus `processName` das Äquivalent von Folgendem aus:

```powershell frame="terminal"
taskkill /IM notes-app.exe /T /F
```

## Mehrere Moonpools oder Ihre eigenen Prozesse

`processName` und `port` wissen nicht, wer einen Prozess gestartet hat. `processName` beendet jeden
Prozess mit diesem Namen, und `port` beendet alles, was auf dem Port lauscht, auch einen,
den eine andere Moonpool-Kopie gestartet hat (die installierte und portable Kopien laufen unabhängig; siehe
[Portabler Modus](/de/data/portable-mode/#mehrere-kopien-gleichzeitig)), und einen, den Sie selbst gestartet haben.
Nutzen Sie diese Modi nur für Apps, die sich so nicht in die Quere kommen: ein Name oder Port, den sonst nichts auf
dem Rechner verwendet. Wenn zwei Kopien dieselbe App registrieren oder Sie sie auch von Hand ausführen, geben Sie ihr
`killMode` `none` oder einen `command`, der nur die eigene Instanz stoppt.

## stopCommand

Wird nur verwendet, wenn `killMode` `command` ist. Er läuft unter Windows über `cmd /c` und sonst über `$SHELL -c`,
in `cwd`, mit Ihrem `env` dazu. `{MP_HOME}` und `{MP_DATA}` funktionieren darin. Moonpool
wartet, bis er fertig ist, bevor es etwas anderes tut, sodass ein Neustart nie startet, solange er
noch läuft. Sein Exit-Code wird ignoriert. Läuft er nach 60 Sekunden noch,
beendet Moonpool ihn samt Kindprozessen und macht weiter.

## Neu starten

Neu starten ist Stoppen gefolgt von Starten desselben `command`. Moonpool wartet bis zu 4 Sekunden, bis
die alte Instanz als gestoppt gilt (damit ihr Port frei ist), bevor es neu startet. Ein `static`-
Eintrag mit nur einer `url` hat nichts zu stoppen: Neu starten öffnet die Seite einfach erneut.

## Docker-Apps unter Windows

Verwenden Sie `none` oder `command` mit einem echten Stopp-Befehl wie `docker compose stop app`. Verwenden Sie
nicht `port`.

Docker Desktop veröffentlicht den Port jedes Containers über einen gemeinsamen Hintergrundprozess. Unter
Windows ist „was auch immer auf dem Port lauscht“ dieser gemeinsame Prozess, daher würde der Modus `port`
Docker Desktop zwangsweise beenden und jeden Container herunterreißen, nicht nur diese App. Als Absicherung
verweigert Moonpool, eine feste Liste gemeinsamer Windows-Prozesse über den Port zu beenden: das Backend,
den Proxy und die Dienstprozesse von Docker Desktop, `dockerd`, `vpnkit`, die WSL-Hostprozesse und zentrale
Systemprozesse wie `svchost`. Das ersetzt nicht die Wahl des richtigen Modus.

Wenn Ihr `command` den Container bereits neu erstellt (`docker compose up -d --build`), ist `none`
richtig: Neu starten führt ihn einfach erneut aus.

Siehe auch [Den Prozess finden und beenden, der einen Port belegt](/de/guides/find-and-kill-process-using-port-windows/)
und [EADDRINUSE und „Port 5173 is in use“ beheben](/de/support/port-already-in-use/).

## Beispiele

Ein Entwicklungsserver, der manchmal einen node-Prozess zurücklässt, der seinen Port hält (das ist der Standard für
`web`, hier ausdrücklich gezeigt):

```json title="apps.json"
{ "id": "site", "name": "Site", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
  "killMode": "port" }
```

Eine Docker-Compose-App:

```json title="apps.json"
{ "id": "api", "name": "API", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\api", "command": "docker compose up -d --build", "port": 8080,
  "killMode": "command", "stopCommand": "docker compose stop app" }
```
