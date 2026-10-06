---
title: "Een npm dev-server op de achtergrond uitvoeren op Windows zonder terminalvenster"
description: "Laat npm run dev, Vite of een andere dev-server op Windows draaien zonder consolevenster om op te letten, en start, stop en lees de uitvoer vanuit het systeemvak."
---

Een dev-server die met `npm run dev` is gestart, draait in de terminal die hem heeft
gestart, dus het sluiten van dat venster beëindigt hem. De gewone Windows-manier om hem
draaiend te houden is een verborgen proces, bijvoorbeeld
`Start-Process npm.cmd -ArgumentList "run","dev" -WindowStyle Hidden` in PowerShell, maar
dan heb je geen uitvoer om te lezen en betekent stoppen dat je de juiste `node.exe` moet
opsporen (zie
[Het proces vinden en beëindigen dat een poort gebruikt](/nl/guides/find-and-kill-process-using-port-windows/)).

## De Moonpool-manier

Moonpool voert het commando uit in een eigen ingebouwd terminaltabblad in het hubvenster,
dus er is geen apart consolevenster dat open moet blijven. Verberg de hub naar het
systeemvak en de server blijft draaien. Voeg de app eenmalig toe:

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

Klik op de knop **Starten** van de app. De statusstip is egaal zodra `port` antwoordt en
de browser opent `url` dankzij `openBrowser`. Klik op de naam van de app om de uitvoer in
het eigen tabblad te lezen. **Stoppen** beëindigt de terminal en alles wat deze heeft
gestart, en geeft de poort vrij (`killMode` `port` is de standaard voor `web`).

## Laat het draaien als je het venster sluit

Standaard sluit de sluitknop Moonpool af, en op Windows stopt afsluiten elke app die het heeft
gestart. Zet **Sluiten naar systeemvak** aan in [Instellingen](/nl/using/settings/), en het
sluiten van het venster verbergt het alleen nog. Het pictogram in het systeemvak (of
**Moonpool tonen**) brengt het terug. Details staan in
[Systeemvak, sluiten en minimaliseren](/nl/using/tray-and-closing/).

## Houd de poort voorspelbaar

Moonpool bepaalt Actief aan de hand van `port`. Vite schuift door naar de volgende vrije
poort als de eigen poort bezet is, waardoor Moonpool de verkeerde poort in de gaten houdt.
Geef `--strictPort` mee zodat Vite in plaats daarvan stopt, en stel `port` er gelijk aan in:

```text title="command"
npm run dev -- --port 5173 --strictPort
```

Is de poort al bezet, zie dan
[EADDRINUSE en "Port 5173 is in use" oplossen](/nl/support/port-already-in-use/).

## Beperkingen

- Moonpool herstart een server die crasht niet. Het toont de app als gestopt en het
  tabblad toont `[process exited]`.
- Moonpool start niet vanzelf bij het aanmelden bij Windows. Zie
  [Een script of dev-server automatisch starten bij het aanmelden bij Windows](/nl/guides/start-app-at-windows-login/).

## Zie ook

- [App-velden](/nl/apps/fields/): `port`, `openBrowser`, `killMode`.
- [App-typen](/nl/apps/types/): hoe Actief wordt bepaald voor `web`.
- [Stoppen en herstarten](/nl/apps/stop-and-restart/)
- [Voorbeelden](/nl/apps/examples/#web-dev-server)
