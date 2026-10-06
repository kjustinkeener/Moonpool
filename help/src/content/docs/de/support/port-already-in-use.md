---
title: "Error: listen EADDRINUSE: address already in use :::3000 und Vite Port 5173 is in use beheben"
description: "Node-EADDRINUSE und Vite-Meldung Port 5173 is in use beheben: den Port-Besitzer finden, den Port freigeben und mit port und killMode in Moonpool Wiederholungen vermeiden."
---

```text
Error: listen EADDRINUSE: address already in use :::3000
```

Dieser Node.js-Fehler bedeutet, dass bereits ein anderer Prozess auf Port 3000 lauscht (`:::`
ist die IPv6-Form für „alle Adressen“; möglicherweise sehen Sie auch `127.0.0.1:3000`). Oft
ist es eine Kopie desselben Servers, die Sie früher gestartet und nie gestoppt haben.

Vite geht mit derselben Situation anders um. Standardmäßig gibt es Folgendes aus:

```text
Port 5173 is in use, trying another one...
```

und startet auf dem nächsten freien Port, sodass der Server zwar läuft, aber nicht dort, wo
Sie ihn erwarten. Mit `--strictPort` (oder `server.strictPort: true`) beendet sich Vite
stattdessen mit `Error: Port 5173 is already in use`.

## Selbst beheben

1. Finden Sie den Prozess, der den Port belegt, und beenden Sie ihn. Unter Windows:

   ```text frame="terminal"
   netstat -ano | findstr :3000
   taskkill /PID 12345 /F
   ```

   Schritt für Schritt, auch mit der PowerShell-Variante, unter
   [Den Prozess finden und beenden, der einen Port belegt](/de/guides/find-and-kill-process-using-port-windows/).
2. Oder starten Sie Ihren Server auf einem anderen Port, zum Beispiel `PORT=3001` bei vielen
   Node-Servern oder `--port 5174` bei Vite.

## So hilft Moonpool

Wenn Sie den Server über Moonpool ausführen, setzen Sie `port` bei seinem Eintrag. Moonpool
dann:

- zeigt die App als „läuft“, solange auf diesem Port etwas antwortet, sodass ein übrig
  gebliebener Server, der ihn hält, als laufend, aber nicht als „managed by Moonpool“
  erscheint;
- beendet bei **Stoppen** und **Neu starten** alles, was noch auf `port` lauscht, wenn
  `killMode` `port` ist (der Standard bei `web`-Apps), sodass der nächste Start den Port
  frei vorfindet;
- markiert zwei Apps, die mit demselben `port` konfiguriert sind.

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "node server.js",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

Moonpool prüft den Port nicht, bevor es startet. Ist der Port noch belegt, gibt der Befehl
den obigen Fehler im Terminal-Tab der App aus. Drücken Sie **Stoppen** (das den Port
freigibt) und dann erneut **Starten**.

Übergeben Sie bei Vite `--strictPort` und halten Sie `port` gleich dem angeforderten Port:

```text title="command"
npm run dev -- --port 5173 --strictPort
```

Ohne diese Option wechselt Vite möglicherweise auf 5174, während Moonpool weiter 5173
beobachtet, und der Statuspunkt wird nie ausgefüllt.

`killMode` `port` beendet jeden Prozess auf dem Port, verwenden Sie es daher nur für Ports,
die sonst nichts braucht. Für Docker-Apps unter Windows verwenden Sie es niemals. Siehe
[Stoppen und Neustarten](/de/apps/stop-and-restart/#docker-apps-unter-windows).

## Siehe auch

- [App-Felder](/de/apps/fields/): `port`, `killMode`.
- [Stoppen und Neustarten](/de/apps/stop-and-restart/)
- [Fehlerbehebung](/de/support/troubleshooting/#zwei-apps-verwenden-denselben-port)
- [Einen npm-Entwicklungsserver unter Windows im Hintergrund ausführen](/de/guides/run-npm-dev-server-in-background-windows/)
