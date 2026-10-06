---
title: "Een script of dev-server automatisch starten bij het aanmelden bij Windows"
description: "Start Moonpool bij het aanmelden bij Windows met een snelkoppeling in de map Opstarten en start daarna een dev-server of script met een klein PowerShell-script."
---

Windows kent twee gebruikelijke manieren om bij het aanmelden iets te starten: een
snelkoppeling in je map Opstarten (druk op Win+R, typ `shell:startup`, druk op Enter), of
een taak in Taakplanner met een trigger "Bij aanmelden". Beide voeren een programma of
script uit, bijvoorbeeld rechtstreeks het commando van je dev-server, maar dan houdt niets
het bij, toont niets de uitvoer en stopt niets het voor jou.

## Wat Moonpool biedt

Moonpool heeft geen instelling om bij het aanmelden te starten, en een item in `apps.json`
heeft geen veld dat de app start wanneer Moonpool start (de volledige lijst staat in
[App-velden](/nl/apps/fields/) en [settings.json](/nl/data/settings-json/)). Wat je wel kunt
doen, is Moonpool zelf bij het aanmelden starten en daarna een script de gewenste apps laten
starten, met hetzelfde werkwoord dat de [opdrachtregel](/nl/automation/command-line/) biedt.

Registreer eerst de app zoals gebruikelijk:

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

Sla dit vervolgens op als `start-moonpool-apps.ps1`. Geïnstalleerd is het programma
`%USERPROFILE%\.moonpool\moonpool.exe`; gebruik voor een draagbare kopie het pad van de
exe van die kopie.

```powershell title="start-moonpool-apps.ps1"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
Start-Process $mp
Start-Sleep -Seconds 15
& $mp launch site
```

Moonpool moet al draaien om `launch` aan Moonpool door te geven; als er niets resident is,
start hetzelfde commando een nieuwe Moonpool en wordt het werkwoord niet uitgevoerd. De
vertraging geeft Moonpool tijd om te starten, dus verhoog haar op een trage computer. Voeg
per app één regel `& $mp launch <id>` toe.

Plaats ten slotte een snelkoppeling naar het script in de map Opstarten, met dit doel:

```text title="Shortcut target"
powershell.exe -NoProfile -WindowStyle Hidden -File "C:\Users\you\start-moonpool-apps.ps1"
```

Om te controleren wat er is gebeurd, voeg je `--ticket t1` toe aan een werkwoord en lees je
het resultaat uit `state.json`
([Het resultaat lezen](/nl/automation/command-line/#de-uitkomst-lezen)).

## Kanttekeningen

- Een op deze manier gestarte dev-server wordt net als elke andere door Moonpool
  "beheerd", dus Stoppen en Afsluiten werken erop. Als dezelfde app al draait (bijvoorbeeld
  met de hand gestart), toont Moonpool hem als actief maar niet beheerd.
- Moonpool start een app die stopt niet opnieuw en onthoudt niet welke apps draaiden toen je
  het laatst afsloot.

## Zie ook

- [Opdrachtregel](/nl/automation/command-line/)
- [Systeemvak, sluiten en minimaliseren](/nl/using/tray-and-closing/)
- [Een npm dev-server op de achtergrond uitvoeren op Windows](/nl/guides/run-npm-dev-server-in-background-windows/)
