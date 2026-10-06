---
title: "Einen npm-Entwicklungsserver unter Windows im Hintergrund ohne Terminalfenster ausführen"
description: "npm run dev, Vite oder einen anderen Entwicklungsserver unter Windows ohne Konsolenfenster laufen lassen und Ausgabe, Start und Stopp bequem über den Tray steuern."
---

Ein mit `npm run dev` gestarteter Entwicklungsserver läuft in dem Terminal, das ihn gestartet
hat, sodass das Schließen dieses Fensters ihn beendet. Der einfache Windows-Weg, ihn am
Laufen zu halten, ist ein versteckter Prozess, zum Beispiel
`Start-Process npm.cmd -ArgumentList "run","dev" -WindowStyle Hidden` in PowerShell. Dann
haben Sie aber keine Ausgabe zum Lesen, und das Stoppen heißt, das richtige `node.exe` zu
suchen (siehe
[Den Prozess finden und beenden, der einen Port belegt](/de/guides/find-and-kill-process-using-port-windows/)).

## Der Moonpool-Weg

Moonpool führt den Befehl in einem eigenen eingebetteten Terminal-Tab im Hub-Fenster aus,
sodass kein separates Konsolenfenster offen bleiben muss. Blenden Sie den Hub in den Tray
aus, und der Server läuft weiter. Fügen Sie die App einmal hinzu:

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173",
  "openBrowser": true
}
```

Klicken Sie auf das Steuerelement **Starten** der App. Der Statuspunkt ist ausgefüllt, sobald
`port` antwortet, und der Browser öffnet wegen `openBrowser` die `url`. Klicken Sie auf den
Namen der App, um ihre Ausgabe in ihrem eigenen Tab zu lesen. **Stoppen** beendet das
Terminal und alles, was es gestartet hat, und gibt den Port frei (`killMode` `port` ist der
Standard für `web`).

## Weiterlaufen lassen, wenn Sie das Fenster schließen

Standardmäßig beendet die Schaltfläche zum Schließen Moonpool, und unter Windows stoppt das
Beenden jede App, die es gestartet hat. Schalten Sie in den
[Einstellungen](/de/using/settings/) **Beim Schließen in den Infobereich** ein, dann blendet
das Schließen des Fensters es nur aus. Das Tray-Symbol (oder **Moonpool anzeigen**) holt es
zurück. Einzelheiten stehen unter
[Tray, Schließen und Minimieren](/de/using/tray-and-closing/).

## Den Port berechenbar halten

Moonpool entscheidet anhand von `port`, ob eine App läuft. Vite wechselt auf den nächsten
freien Port, wenn sein Port belegt ist, sodass Moonpool den falschen beobachten würde.
Übergeben Sie `--strictPort`, damit Vite sich stattdessen beendet, und setzen Sie `port`
passend:

```text title="command"
npm run dev -- --port 5173 --strictPort
```

Wenn der Port bereits belegt ist, siehe
[EADDRINUSE und „Port 5173 is in use“ beheben](/de/support/port-already-in-use/).

## Grenzen

- Moonpool startet einen abgestürzten Server nicht neu. Es zeigt die App als gestoppt an,
  und der Tab gibt `[process exited]` aus.
- Moonpool startet nicht von selbst beim Windows-Login. Siehe
  [Ein Skript oder einen Entwicklungsserver automatisch beim Windows-Login starten](/de/guides/start-app-at-windows-login/).

## Siehe auch

- [App-Felder](/de/apps/fields/): `port`, `openBrowser`, `killMode`.
- [App-Typen](/de/apps/types/): wie „läuft“ bei `web` bestimmt wird.
- [Stoppen und Neustarten](/de/apps/stop-and-restart/)
- [Beispiele](/de/apps/examples/#web-entwicklungsserver)
