---
title: "Een Python-script op de achtergrond laten draaien op Windows"
description: "Laat een langlopend Python-script of kleine web-app op Windows op de achtergrond draaien, bekijk de uitvoer en stop het netjes, met pythonw en met Moonpool."
---

Een Python-script dat vanuit een consolevenster wordt uitgevoerd, stopt zodra je dat venster
sluit. De gebruikelijke oplossingen onder Windows zijn `pythonw.exe` (dezelfde interpreter
zonder consolevenster, zodat de uitvoer nergens heen gaat), `Start-Process pythonw
-ArgumentList worker.py` om het losgekoppeld te starten, of een geplande taak voor iets wat
bij het aanmelden of op een timer moet draaien. Bij elk daarvan moet je zelf het proces in
Taakbeheer opzoeken als je het weg wilt hebben.

## De Moonpool-manier

Moonpool voert het commando uit in een eigen terminaltabblad, zodat je de uitvoer en een
knop Stoppen houdt zonder eigen consolevenster. Gebruik voor een script dat draait tot je het
stopt een `cli`-app. `-u` laat Python de uitvoer direct wegschrijven, zodat het tabblad die live toont:

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

Start de app en klik op de naam om de uitvoer te bekijken. Een `cli`-app telt als Actief
zolang het commando draait en wordt grijs wanneer het script eindigt, waarbij
`[process exited]` in het tabblad blijft staan. **Stoppen** beëindigt het script en alles wat
het heeft gestart. Doordat je `python.exe` van de virtuele omgeving via het pad aanroept,
is geen activeringsstap nodig.

Als het script HTTP aanbiedt (Flask, FastAPI, `python -m http.server`), maak er dan een
`web`-app van, zodat Actief de poort volgt:

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

## Beperkingen

- Laat Moonpool draaien. Het sluiten van het venster sluit Moonpool standaard af, en op
  Windows stopt afsluiten elke app die het heeft gestart. Zet **Sluiten naar systeemvak** aan
  om het venster in plaats daarvan te verbergen; zie
  [Systeemvak, sluiten en minimaliseren](/nl/using/tray-and-closing/).
- Moonpool herstart een script dat crasht niet en start het niet zelf bij het aanmelden bij
  Windows. Zie [Een script of dev-server automatisch starten bij het aanmelden bij Windows](/nl/guides/start-app-at-windows-login/).
- Vermijd geneste dubbele aanhalingstekens in `command`: de `cmd /c`-wrapper maakt er
  een puinhoop van.

## Zie ook

- [App-typen](/nl/apps/types/#cli): hoe `cli`- en `web`-apps worden gevolgd.
- [Stoppen en herstarten](/nl/apps/stop-and-restart/)
- [Voorbeelden](/nl/apps/examples/)
- [Logs](/nl/data/logs/): waar de sessie-uitvoer wordt bewaard.
