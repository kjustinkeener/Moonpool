---
title: "Een dev-server en alles wat hij heeft gestart stoppen"
description: "Laat Stoppen en Herstarten een app en zijn onderliggende processen netjes beëindigen met killMode en stopCommand, inclusief standaarden per type en Docker onder Windows."
---

Stoppen doet altijd eerst dit: Moonpool beëindigt de terminal die het voor de app heeft gestart, inclusief
alles wat die terminal heeft gestart. Voor veel apps is dat alles wat nodig is.

Sommige apps overleven die terminal (een desktopvenster maakt zich los van de dev-server die het
startte, of een subproces van de server blijft zijn poort bezet houden). **`killMode`** kiest één
extra stap die daarna wordt uitgevoerd.

| `killMode` | Extra stap bij Stoppen | Leest | Standaard voor |
| --- | --- | --- | --- |
| `processName` | Beëindigt geforceerd elk proces met die naam. Onder Windows ook de onderliggende processen (`taskkill /IM <name>.exe /T /F`). Elders `pkill -KILL -x <name>`: een exacte, hoofdlettergevoelige naamovereenkomst, zonder onderliggende processen. | `processName` | `desktop` |
| `port` | Beëindigt geforceerd het proces dat op `port` luistert. | `port` | `web` |
| `command` | Voert `stopCommand` uit in `cwd` en wacht tot het klaar is. | `stopCommand`, `cwd`, `env` | niets |
| `none` | Niets. | niets | `static`, `cli` |

Laat `killMode` weg om de standaard voor het type van de app te krijgen, en stel hem alleen in wanneer Stoppen
iets laat doorlopen.

![De keuzelijst killMode in het dialoogvenster App bewerken, ingesteld op "standaard (per type)", met de hintregel die toont wat elk type standaard doet](../../../../assets/screenshots/edit-app-killmode.png)

1. De keuzelijst `killMode`. "standaard (per type)" is hetzelfde als de sleutel weglaten.

- Als het veld dat de modus nodig heeft leeg is (bijvoorbeeld modus `port` zonder `port`), wordt de extra
  stap overgeslagen. Dat is geen fout.
- `killMode` is onafhankelijk van `type`: `port` werkt bij een `cli`-app, `processName` bij een
  `web`-app.
- Een lege string of een onbekende waarde doet niets extra. Er wordt niet teruggevallen op de
  standaard van het type.

Voor een desktop-app voert de modus `processName` het equivalent uit van:

```powershell frame="terminal"
taskkill /IM notes-app.exe /T /F
```

## Meerdere Moonpools, of je eigen processen

`processName` en `port` weten niet wie een proces heeft gestart. `processName` beëindigt elk
proces met die naam, en `port` beëindigt alles wat op de poort luistert, ook wat
een andere Moonpool-kopie heeft gestart (de geïnstalleerde en draagbare kopieën draaien onafhankelijk; zie
[Draagbare modus](/nl/data/portable-mode/#meerdere-kopieën-tegelijk)) en wat je zelf hebt gestart.
Gebruik deze modi alleen voor apps die elkaar zo niet in de weg zitten: een naam of poort die niets anders op
de machine gebruikt. Als twee kopieën dezelfde app registreren, of je hem ook handmatig draait, geef hem dan
`killMode` `none` of een `command` dat alleen zijn eigen instantie stopt.

## stopCommand

Alleen gebruikt wanneer `killMode` `command` is. Het draait via `cmd /c` onder Windows en `$SHELL -c`
elders, in `cwd`, met je `env` erbij. `{MP_HOME}` en `{MP_DATA}` werken erin. Moonpool
wacht tot het klaar is voordat het iets anders doet, zodat een Herstart nooit opnieuw start terwijl het
nog loopt. De afsluitcode wordt genegeerd. Als het na 60 seconden nog loopt,
beëindigt Moonpool het en zijn onderliggende processen en gaat verder.

## Herstarten

Herstarten is Stoppen gevolgd door Starten van hetzelfde `command`. Moonpool wacht maximaal 4 seconden tot
de oude instantie als gestopt wordt gelezen (zodat de poort vrij is) voordat het opnieuw start. Een `static`-item
met alleen een `url` heeft niets om te stoppen: Herstarten opent de pagina gewoon opnieuw.

## Docker-apps onder Windows

Gebruik `none`, of `command` met een echt stopcommando zoals `docker compose stop app`. Gebruik
`port` niet.

Docker Desktop publiceert de poort van elke container via één gedeeld achtergrondproces. Onder
Windows is "wat er op de poort luistert" dat gedeelde proces, dus de modus `port` zou
Docker Desktop geforceerd beëindigen en elke container laten uitvallen, niet alleen deze app. Als vangnet
weigert Moonpool een vaste lijst gedeelde Windows-processen via de poort te beëindigen: de backend-, proxy- en
serviceprocessen van Docker Desktop, `dockerd`, `vpnkit`, de WSL-hostprocessen en kernprocessen van het
systeem zoals `svchost`. Dat vervangt het kiezen van de juiste modus niet.

Als je `command` de container al opnieuw aanmaakt (`docker compose up -d --build`), is `none`
juist: Herstarten voert het gewoon opnieuw uit.

Zie ook [Het proces vinden en beëindigen dat een poort gebruikt](/nl/guides/find-and-kill-process-using-port-windows/)
en [EADDRINUSE en "Port 5173 is in use" oplossen](/nl/support/port-already-in-use/).

## Voorbeelden

Een dev-server die soms een node-proces achterlaat dat zijn poort bezet houdt (dit is de standaard voor
`web`, hier expliciet getoond):

```json title="apps.json"
{ "id": "site", "name": "Site", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
  "killMode": "port" }
```

Een Docker Compose-app:

```json title="apps.json"
{ "id": "api", "name": "API", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\api", "command": "docker compose up -d --build", "port": 8080,
  "killMode": "command", "stopCommand": "docker compose stop app" }
```
