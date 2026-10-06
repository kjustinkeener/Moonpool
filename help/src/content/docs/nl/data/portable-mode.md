---
title: "Moonpool draaien vanaf een usb-stick of gesynchroniseerde map"
description: "Houd Moonpool en al zijn gegevens in één verplaatsbare map, zodat je het op een usb-stick kunt meenemen of kunt synchroniseren, en draai meerdere kopieën naast elkaar."
---

De draagbare modus houdt Moonpool en alles wat het schrijft in één map `.moonpool\`, zodat
je het op een usb-stick kunt meenemen of in een gesynchroniseerde map kunt zetten en op elke pc kunt draaien.

## Hoe het werkt

Wanneer je draagbaar installeert, maakt Moonpool een map `.moonpool\` aan op de locatie die je
kiest. Die map bevat het programma, je configuratie en de help-inhoud. Er wordt niets
naar Windows AppData geschreven, dus het verplaatsen of kopiëren van de map neemt je hele opzet
mee.

```text
<chosen location>\.moonpool\
```

## Wat anders is dan bij geïnstalleerd

| | Geïnstalleerd | Draagbaar |
| --- | --- | --- |
| Programma | `%USERPROFILE%\.moonpool\moonpool.exe` | `<chosen location>\.moonpool\moonpool.exe` |
| Configuratiemap | `%USERPROFILE%\.moonpool\moonpool-config\` | `<chosen location>\.moonpool\moonpool-config\` |
| Browserprofiel, grootte en positie van het venster | In de configuratiemap | In de configuratiemap, dus ze reizen ook mee |
| Startmenu, bureaubladsnelkoppeling, vermelding bij Programma's toevoegen/verwijderen | Ja | Geen |
| Updates | Vervangt zijn eigen exe | Hetzelfde, binnen de map `.moonpool\`. Zie [Bijwerken](/nl/data/updating/#draagbare-kopieën). |
| Verwijderen | Programma's toevoegen/verwijderen of `--uninstall` | Wis de map |

Geen van beide modi schrijft naar Windows AppData.

### Gesynchroniseerde mappen

Je kunt een draagbare kopie in een gesynchroniseerde map bewaren (OneDrive, Dropbox en dergelijke), maar draai hem
op één pc tegelijk. Moonpool schrijft om de paar seconden `state.json` en logt terwijl apps
draaien, dus twee pc's die dezelfde map draaien vechten om dezelfde bestanden, en een synchronisatieconflict kan
een kapotte `apps.json` achterlaten. Sluit het op de ene pc af voordat je het op een andere start.

## Meerdere kopieën tegelijk

Per map draait één Moonpool. De geïnstalleerde Moonpool en een willekeurig aantal draagbare kopieën, elk
in een eigen map, kunnen tegelijk draaien, en elk is volledig gescheiden: eigen apps, systeemvakpictogram,
venster, instellingen, logs en [besturingskanaal](/nl/automation/control-verbs/).

- De tooltip van het systeemvak en de naam op de taakbalk geven aan welke kopie welke is: `Moonpool` voor de
  geïnstalleerde, `Moonpool (<folder>)` voor een draagbare, waarbij `<folder>` de map is die je
  hebt gekozen (die met `.moonpool\`).
- Dezelfde kopie een tweede keer starten brengt het venster terug in plaats van er nog een
  te openen. Een andere kopie starten opent die kopie.
- Om een AI-agent meer dan één kopie te geven, registreer je elke onder een eigen naam; zie
  [MCP-installatie](/nl/automation/mcp-setup/#meer-dan-één-moonpool).
- Het verplaatsen of hernoemen van een draagbare map geeft hem een nieuwe identiteit (een nieuwe naam van het besturingskanaal).
  Sluit hem af voordat je hem verplaatst.
- Kopieën weten niets van elkaars apps. Twee kopieën die dezelfde server op
  dezelfde poort starten, botsen nog steeds, en een Stoppen dat via procesnaam of poort werkt kan
  iets beëindigen wat een andere kopie heeft gestart; zie
  [Stoppen en herstarten](/nl/apps/stop-and-restart/#meerdere-moonpools-of-je-eigen-processen).

## Je apps ook laten meereizen

Gebruik het token `{MP_HOME}` in het pad van een app, zodat het binnen de draagbare map wijst in plaats van
naar een vaste locatie op één machine. In een draagbare kopie is `{MP_HOME}` de map die
`moonpool.exe` bevat, dus de map `.moonpool\` zelf, niet de map die je hebt gekozen:

```json title="apps.json"
{ "cwd": "{MP_HOME}/my-app" }
```

Hier is `{MP_HOME}/my-app` gelijk aan `<chosen location>\.moonpool\my-app`. Een pad dat begint met
`./` wordt op dezelfde manier verankerd. Tokens en `./`-paden werken ook in een geïnstalleerde Moonpool.
Zie [Paden en omgeving](/nl/apps/paths-and-environment/) voor hoe paden worden omgezet.

## Draagbaar kiezen in het installatieprogramma

De draagbare modus stel je in via de installatiekaart, die **Portable installeren** aanbiedt
naast **Moonpool installeren**.

![De installatiekaart: de link Portable installeren staat onder de hoofdknop Moonpool installeren](../../../../assets/screenshots/installer-window.png)

Kies een map en Moonpool maakt daar de map `.moonpool\` aan, kopieert zichzelf erin en
start de nieuwe kopie met een nieuwe configuratie.

De kaart staat ook in het menu "..." als **Moonpool installeren…**, zowel in de geïnstalleerde als in de
draagbare modus. Als je daar **Portable installeren** gebruikt, sluit de draaiende Moonpool af en
start de nieuwe draagbare kopie in zijn plaats. De Moonpool waarmee je begon blijft staan waar hij
stond, zodat je hem daarna weer kunt starten.

Een draagbare kopie begint leeg en neemt je bestaande apps niet over. Om ze over te brengen,
sluit je de draagbare kopie af en kopieer je `apps.json` handmatig:

| | Pad |
| --- | --- |
| Van (geïnstalleerd) | `%USERPROFILE%\.moonpool\moonpool-config\apps.json` |
| Naar (draagbaar) | `<chosen location>\.moonpool\moonpool-config\apps.json` |

Items met absolute paden werken nog op dezelfde pc, maar reizen niet mee. Het dialoogvenster App bewerken
markeert ze met "niet portable".

## Hoe Moonpool weet dat het draagbaar is

Een kopie is draagbaar zolang er een bestand met de naam `moonpool.portable` naast zijn `moonpool.exe` staat.
Niets anders markeert dit, en er wordt niets bij Windows geregistreerd.

Om een draagbare kopie te verwijderen, sluit je hem af en wis je zijn map `.moonpool\`. `--uninstall` verwijdert alleen
de geïnstalleerde Moonpool, nooit een draagbare kopie.
