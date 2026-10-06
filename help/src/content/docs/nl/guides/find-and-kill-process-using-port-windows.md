---
title: "Het proces vinden en beëindigen dat een poort gebruikt op Windows (3000, 5173, 8080)"
description: "Zoek welk proces poort 3000 of 5173 op Windows bezet houdt met netstat of PowerShell, beëindig het met taskkill en laat Moonpool de poort vrijgeven bij het stoppen."
---

Als een dev-server niet start omdat de poort al in gebruik is, luistert iets anders op die
poort. Toon in de opdrachtprompt de luisteraars met het bijbehorende proces-ID en beëindig het proces dan:

```text frame="terminal"
netstat -ano | findstr :3000
taskkill /PID 12345 /F
```

De laatste kolom van de regel `LISTENING` is de PID (`findstr :3000` komt ook overeen met
`:30001`, dus lees het lokale adres). `tasklist /FI "PID eq 12345"` toont om welk programma
het gaat. In PowerShell is dezelfde opzoeking:

```powershell frame="terminal"
Get-NetTCPConnection -LocalPort 3000 -State Listen | Select-Object LocalPort, OwningProcess
Get-Process -Id 12345
Stop-Process -Id 12345 -Force
```

Voeg `/T` toe aan `taskkill` om ook de onderliggende processen van het proces te beëindigen.
Processen van een andere gebruiker of van het systeem kunnen een verhoogd venster
(beheerder) nodig hebben.

## De Moonpool-manier

Voor een app die je via Moonpool uitvoert, zoek je de PID niet op. Geef de app een `port` en
Stoppen geeft de poort vrij. Voor een `web`-app is dat de standaard `killMode`, hier
uitgeschreven:

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

Stoppen beëindigt eerst de terminal die Moonpool heeft gestart en beëindigt daarna geforceerd
alles wat nog op `port` luistert. Op Windows is dat dezelfde opzoeking als hierboven
(`Get-NetTCPConnection -LocalPort
<port> -State Listen`), gevolgd door `taskkill /PID <pid> /T /F` voor elke eigenaar.

- Als iets wat je niet zelf hebt gestart de poort bezet houdt, toont Moonpool de app als
  actief maar niet "managed by Moonpool". Druk op **Stoppen** bij die app: de stap met `port`
  wordt nog steeds uitgevoerd.
- Moonpool weigert een vaste lijst gedeelde Windows-processen via de poort te beëindigen, zoals
  de backend van Docker Desktop, `svchost` en de WSL-host. Gebruik voor een Docker-app
  `killMode` `command` of `none`, nooit `port`. Zie
  [Docker-apps op Windows](/nl/apps/stop-and-restart/#docker-apps-onder-windows).
- Dit werkt alleen voor poorten van apps die in `apps.json` staan. Gebruik voor elke andere
  poort de commando's bovenaan.
- De modus `port` beëindigt alles wat luistert, ook een kopie die je met de hand hebt gestart,
  dus gebruik hem alleen voor poorten die niets anders op de computer nodig heeft.

## Zie ook

- [EADDRINUSE en "Port 5173 is in use" oplossen](/nl/support/port-already-in-use/)
- [Stoppen en herstarten](/nl/apps/stop-and-restart/)
- [App-velden](/nl/apps/fields/): `port` en `killMode`.
- [Twee apps gebruiken dezelfde poort](/nl/support/troubleshooting/#twee-apps-gebruiken-dezelfde-poort)
