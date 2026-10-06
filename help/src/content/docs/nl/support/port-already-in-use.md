---
title: "Oplossen: Error: listen EADDRINUSE: address already in use :::3000 en Vite Port 5173 is in use"
description: "Los Node's EADDRINUSE en Vite's Port 5173 is in use op: zoek wat de poort bezet houdt, geef hem vrij en voorkom herhaling met de velden port en killMode van Moonpool."
---

```text
Error: listen EADDRINUSE: address already in use :::3000
```

Deze Node.js-fout betekent dat een ander proces al op poort 3000 luistert (de `:::` is de
IPv6-vorm van "alle adressen"; je ziet misschien ook `127.0.0.1:3000`). Vaak is het een kopie
van dezelfde server die je eerder hebt gestart en nooit hebt gestopt.

Vite gaat anders met dezelfde situatie om. Standaard toont het:

```text
Port 5173 is in use, trying another one...
```

en start het op de volgende vrije poort, dus de server draait wel maar niet waar je het
verwacht. Met `--strictPort` (of `server.strictPort: true`) stopt Vite in plaats daarvan, met
`Error: Port 5173 is already in use`.

## Zelf oplossen

1. Zoek het proces dat de poort bezit en beëindig het. Op Windows:

   ```text frame="terminal"
   netstat -ano | findstr :3000
   taskkill /PID 12345 /F
   ```

   Stap voor stap, met de PowerShell-versie, in
   [Het proces vinden en beëindigen dat een poort gebruikt](/nl/guides/find-and-kill-process-using-port-windows/).
2. Of start je server op een andere poort, bijvoorbeeld `PORT=3001` voor veel Node-servers of
   `--port 5174` voor Vite.

## Hoe Moonpool helpt

Als je de server via Moonpool uitvoert, stel dan `port` in op het item. Moonpool:

- toont de app als Actief zolang er iets op die poort antwoordt, zodat een achtergebleven server
  die de poort bezet houdt als actief maar niet "managed by Moonpool" verschijnt;
- beëindigt bij **Stoppen** en **Herstarten** alles wat nog op `port` luistert als `killMode`
  gelijk is aan `port` (de standaard voor `web`-apps), zodat de volgende start de poort vrij vindt;
- markeert twee apps die met dezelfde `port` zijn geconfigureerd.

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

Moonpool controleert de poort niet voordat het start. Is de poort nog bezet, dan toont het
commando de bovenstaande fout in het terminaltabblad van de app. Druk op **Stoppen** (dat de poort
vrijgeeft) en daarna weer op **Starten**.

Geef voor Vite `--strictPort` mee en houd `port` gelijk aan de poort die je opvraagt:

```text title="command"
npm run dev -- --port 5173 --strictPort
```

Zonder dat kan Vite doorschuiven naar 5174 terwijl Moonpool 5173 blijft bewaken, en wordt de
statusstip nooit egaal.

`killMode` `port` beëindigt elk proces op de poort, dus gebruik het alleen voor poorten die niets
anders nodig heeft. Gebruik het nooit voor Docker-apps op Windows. Zie
[Stoppen en herstarten](/nl/apps/stop-and-restart/#docker-apps-onder-windows).

## Zie ook

- [App-velden](/nl/apps/fields/): `port`, `killMode`.
- [Stoppen en herstarten](/nl/apps/stop-and-restart/)
- [Probleemoplossing](/nl/support/troubleshooting/#twee-apps-gebruiken-dezelfde-poort)
- [Een npm dev-server op de achtergrond uitvoeren op Windows](/nl/guides/run-npm-dev-server-in-background-windows/)
