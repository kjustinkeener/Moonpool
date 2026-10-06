---
title: "Moonpool-problemen oplossen: systeemvak, apps die niet starten, updates"
description: "Los veelvoorkomende Moonpool-problemen op aan de hand van wat je ziet: ontbrekend pictogram in het systeemvak, apps die niet starten of stoppen, foute statusstippen, mislukte updates en MCP-fouten."
---

Zoek het symptoom op en volg dan de oplossing. Geciteerde tekst is wat Moonpool toont. Om een
exacte melding op te zoeken, zie [Foutmeldingen uitgelegd](/nl/support/error-messages/).

## Ik zie het pictogram in het systeemvak niet

- **Windows.** Het pictogram kan in het gebied met verborgen pictogrammen staan. Klik op de pijl **^**
  rechts in de taakbalk. Sleep het pictogram naar de taakbalk om het zichtbaar te houden.
- **Linux op standaard GNOME.** GNOME toont zonder de AppIndicator-extensie geen pictogrammen in
  het systeemvak. Zie [Linux](/nl/platforms/linux/#systeemvak-op-gnome).
- **Instellingen.** **Weergeven in systeemvak** staat misschien uit. Open de hub via de taakbalk of
  het Startmenu en zet het weer aan in [Instellingen](/nl/using/settings/).

## Het installatieprogramma toont een fout

| Melding | Wat te doen |
| --- | --- |
| `Install failed: <error>` | De tekst na de dubbele punt noemt de stap die is mislukt, bijvoorbeeld `copy exe: ...`. Als een bestand in gebruik is, sluit je elke Moonpool die vanuit `%USERPROFILE%\.moonpool` draait af en probeer je het opnieuw. |
| `target folder does not exist` | De map die je voor een draagbare kopie hebt gekozen, bestaat niet meer. Kies een bestaande map. |
| `that folder already has a .moonpool with an exe of this name that isn't a portable Moonpool - pick an empty folder` | Kies een lege map, of verwijder eerst die map `.moonpool`. |

## Windows protected your PC verschijnt als ik het installatieprogramma uitvoer

Dat is Windows SmartScreen, omdat `moonpool.exe` niet met een codehandtekening is ondertekend. Klik op
**More info** (Meer info) en daarna op **Run anyway** (Toch uitvoeren). Zie
[Windows heeft uw pc beschermd](/nl/support/windows-protected-your-pc/).

## Het Moonpool-venster is leeg of opent nooit op Windows

De Microsoft Edge WebView2 Runtime ontbreekt mogelijk. Zie
[WebView2-runtime ontbreekt](/nl/support/webview2-runtime-missing/).

## Een app start niet

1. Klik op de naam van de app om het terminaltabblad te openen en de uitvoer te lezen. Een agent kan
   dezelfde tekst lezen met `moonpool_app_output`.
2. Controleer `cwd`. Een ontbrekende map, of een relatief pad zonder `./`, is de gebruikelijke oorzaak. Zie
   [Paden en omgeving](/nl/apps/paths-and-environment/).
3. Controleer `command`. Voer het met de hand uit in een terminal in `cwd`. Vermijd op Windows geneste
   dubbele aanhalingstekens; `cmd /c` maakt er een puinhoop van.
4. Zet **Debug-informatie naar een bestand loggen** aan in Instellingen en start opnieuw.
   `moonpool.log` legt het exacte commando en de map vast. Zie [Logs](/nl/data/logs/).

| Melding | Betekenis |
| --- | --- |
| `already running` | Moonpool heeft al een terminal voor deze app. Stop de app eerst, of gebruik Herstarten. |
| `stopped during launch` | Er is op Stoppen gedrukt terwijl de start nog bezig was. |
| `did not reach running in time` | Vanuit een script of agent: de app werd binnen 25 seconden niet als Actief gezien. Controleer `port` of `processName` en de uitvoer. |

## De statusstip klopt niet

Moonpool bepaalt Actief aan de hand van `port`, dan `processName`, en dan of de eigen terminal nog
leeft. Zie [Hoe Actief wordt bepaald](/nl/apps/types/#hoe-actief-wordt-bepaald).

- **Wordt nooit egaal.** De `port` van een `web`-app antwoordt niet, of de `processName` van een
  `desktop`-app komt niet overeen. Op Linux moet `processName` maximaal 15 tekens lang zijn.
- **Wordt grijs direct na het starten.** Een `cli`-app is niet meer Actief zodra het commando
  eindigt. Gebruik een shell met `-NoExit` als je wilt dat die open blijft.
- **Een `static`-app toont nooit Actief.** Dat is normaal voor een item met alleen een `url`.
- **Toont Actief terwijl je de app niet hebt gestart.** Iets anders gebruikt die poort of
  procesnaam. Moonpool toont het als actief maar niet "managed by Moonpool".

## Error: listen EADDRINUSE of "Port 5173 is in use"

Iets anders luistert al op de poort die je server wil gebruiken. Zoek het en beëindig het, of stel
`port` in op de app zodat Stoppen de poort vrijgeeft. Zie
[EADDRINUSE en "Port 5173 is in use" oplossen](/nl/support/port-already-in-use/) en
[Het proces vinden en beëindigen dat een poort gebruikt](/nl/guides/find-and-kill-process-using-port-windows/).

## Twee apps gebruiken dezelfde poort

Onderaan het menu **...** verschijnt een waarschuwingsrij, bijvoorbeeld `poort 3000: App A en App B`.
Wijzig de `port` van een van de apps (en zijn `env`, als die `PORT` leest). Zie
[Waarschuwing bij poortconflict](/nl/using/hub-window/#waarschuwing-bij-poortconflict).

## De app blijft draaien na Stoppen

Vanuit een script of agent is de fout `still running after stop` (na 15 seconden).

- De app overleeft zijn terminal. Stel `killMode` in op `port` of `processName`. Zie
  [Stoppen en herstarten](/nl/apps/stop-and-restart/).
- Een Docker-app op Windows: gebruik `killMode` `command` met een `stopCommand` zoals
  `docker compose stop app`. Nooit `port`.

## apps.json bevat een fout

De zijbalk toont een banner, "apps.json bevat een fout; de laatst geladen lijst wordt getoond." of,
bij het opstarten, "apps.json bevat een fout, dus er zijn geen apps geladen." Opslaan vanuit
Moonpool is gepauzeerd totdat het bestand weer laadt.

Typische fouten:

```text
apps.json entry 2 (site) requires a command
apps.json entry 3 has invalid id "my app"; use letters, digits, '.', '_', and '-' without a leading '-'
duplicate app id "site"
apps.json entry 4 (api) has invalid port 0
```

1. Kies **apps.json bewerken** in de banner, herstel het item, sla op en kies daarna **Opnieuw laden** (F5).
2. Of ga terug naar een recente goede kopie. Zie
   [Back-up en herstel](/nl/data/backup-and-recovery/#appsjson-terugdraaien).

De volledige lijst met regels staat in [Validatie](/nl/apps/apps-json/#validatie).

Als een instelling niet kan worden gewijzigd en de melding eindigt op `Repair settings.json and restart
Moonpool before changing settings`, herstel of verwijder dan `settings.json` in de configuratiemap en
start Moonpool opnieuw. Verwijderen zet elke instelling terug naar de standaardwaarde.

## Mijn wijziging werkt niet

- Handmatige wijzigingen vereisen **Opnieuw laden** (of F5). Moonpool bewaakt het bestand niet.
- Opnieuw laden herstart geen draaiende apps. Herstart de app om een gewijzigde `command`, `cwd`
  of `env` te gebruiken.
- Een agent bewerkt mogelijk een andere `apps.json`. Vraag hem `moonpool_launcher_paths` aan te
  roepen en de map van de hub met zijn eigen map te vergelijken. Controleer bij meerdere kopieën van
  Moonpool welke kopie je bewerkt.

## De voorbeeld-apps ontbreken

Voorbeelden worden alleen geschreven als er geen `apps.json` bestaat. Om ze terug te krijgen, zie
[Terugzetten naar de voorbeelden](/nl/data/backup-and-recovery/#terugzetten-naar-de-voorbeelden), of kopieer
de items uit [Voorbeelddashboards](/nl/getting-started/example-dashboards/#voorbeeld-apps-verschijnen-alleen-bij-de-eerste-keer).

## Een update is mislukt

De banner toont `Update failed: <error>`. Zie
[Wanneer een update mislukt](/nl/data/updating/#wanneer-een-update-mislukt).

## Een weblink opent niet

`refusing to open non-web url: <url>` betekent dat de `url` niet `http://`, `https://`,
`mailto:` of `file://` is. Corrigeer de `url`.

## MCP- en scriptfouten

| Melding | Wat te doen |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Start Moonpool, of laat de agent `moonpool_bootup_launcher` aanroepen. |
| `frontend not loaded` | Het hubvenster is nog niet klaar met laden. Wacht even en probeer opnieuw. |
| `stale token: ...` | `apps.json` is gewijzigd sinds de agent het las. Lees het opnieuw en schrijf dan. |
| `rejected invalid manifest: ...` | De nieuwe `apps.json` voldeed niet aan de validatie. Het bestand is niet gewijzigd. |
| `... A Moonpool process may be hung ...` | Iets houdt het besturingskanaal vast zonder te antwoorden. Sluit Moonpool af vanuit het systeemvak, of beëindig het proces, en start het opnieuw. |

Meer in [MCP-installatie](/nl/automation/mcp-setup/#opmerkingen) en
[MCP-tools](/nl/automation/mcp-tools/).

## Vensterproblemen

- **Buiten het scherm.** Moonpool negeert een opgeslagen positie die op geen enkel aangesloten
  beeldscherm ligt. Als het venster nog steeds zoek is, sluit je Moonpool af en verwijder je
  `window-state.json` in de configuratiemap.
- **Zoom blijft te groot of te klein.** Ctrl + wiel boven de hub wijzigt het. Zie
  [Sneltoetsen en zoom](/nl/using/keyboard-shortcuts/#zoom).
- **Instellingen opent achter de hub.** Zet **Altijd op voorgrond** uit, of aan, in Instellingen.
  Het geldt voor elk Moonpool-venster, zodat ze op dezelfde laag blijven.

## Waar staan de logs?

Zie [Logs](/nl/data/logs/).

## Back-up, reset of verwijderen

Zie [Back-up en herstel](/nl/data/backup-and-recovery/) en
[Verwijderen](/nl/getting-started/install/#verwijderen).

## FAQ

**Stopt het sluiten van het venster mijn apps?**
Standaard sluit sluiten Moonpool af, en op Windows stopt afsluiten de apps die het heeft gestart. Zet
**Sluiten naar systeemvak** aan om Moonpool te laten draaien als je het venster sluit. Zie
[Systeemvak, sluiten en minimaliseren](/nl/using/tray-and-closing/).

**Kan ik Moonpool twee keer uitvoeren?**
Eén per map. Dezelfde kopie opnieuw starten brengt het venster terug. De geïnstalleerde kopie en
draagbare kopieën kunnen naast elkaar draaien. Zie
[Draagbare modus](/nl/data/portable-mode/#meerdere-kopieën-tegelijk).

**Belt Moonpool naar huis?**
Alleen om op updates te controleren: het haalt het releasebestand (`update.json`) op van GitHub bij het
opstarten (als **Bij het opstarten controleren op updates** aan staat) en wanneer je op **Controleren op
updates** drukt. Elke download wordt gecontroleerd aan de hand van de ondertekeningssleutel van Moonpool
voordat die wordt gebruikt.

**Welke shell voert mijn commando's uit?**
`cmd /c` op Windows, `$SHELL -c` op Linux en macOS.

**Waar zet ik geheimen?**
`env`-waarden worden als gewone tekst in `apps.json` opgeslagen. Gebruik liever een bestand dat je
app zelf leest, of een variabele die al in je gebruikersomgeving is ingesteld, want gestarte apps
erven die.

**Is het besturingskanaal beveiligd?**
Het heeft geen aanmelding of token. Elk proces dat als je draait, kan er commando's naartoe sturen. Op
Linux en macOS is de socket alleen leesbaar voor je gebruiker. Zie
[Veiligheidseigenschappen](/nl/automation/overview/#veiligheidseigenschappen).
