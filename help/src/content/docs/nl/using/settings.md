---
title: "Moonpool-instellingen wijzigen: alle opties in Instellingen en Over"
description: "Een volledige lijst van de bedieningselementen in de vensters Instellingen en Over van Moonpool, de settings.json-sleutel die elk schrijft en hoe je een instelling reset."
---

Open **Instellingen** via het menu "..." van de hub. Wijzigingen worden opgeslagen terwijl je ze
aanbrengt. Escape sluit het venster. Deze pagina is de volledige lijst met instellingen. Elke
instelling wordt opgeslagen in `settings.json` onder de getoonde sleutel; het bestand zelf wordt
beschreven in [settings.json](/nl/data/settings-json/).

![Venster Instellingen: schakelaars en schuifregelaars in de linkerkolom, logopties in de rechter](../../../../assets/screenshots/settings-window.png)

## Een bedieningselement resetten

Klik met rechts op een selectievakje, schuifregelaar of getalveld om alleen die instelling terug
te zetten naar de standaardwaarde. De tooltip bij elk bedieningselement zegt dat ook. De keuzes
voor Taal en Thema kunnen niet worden gereset.

## Linkerkolom

| Bedieningselement | Sleutel | Standaard | Wat het doet |
| --- | --- | --- | --- |
| Taal | `locale` | Automatisch (systeem) | Taal van de eigen teksten van Moonpool. Geldt direct. Zie [Thema's, taal en transparantie](/nl/using/themes-and-language/). |
| Thema | geen (browseropslag) | Automatisch (systeem) | Kleurthema. De knop opent een themabrowser met een voorbeeld van elk thema; een klik past het direct toe. Zie [Thema's, taal en transparantie](/nl/using/themes-and-language/). |
| Sluiten naar systeemvak | `closeToTray` | uit | Aan: bij het sluiten van het venster verdwijnt Moonpool naar het systeemvak. Uit: sluiten sluit Moonpool af. |
| Minimaliseren naar systeemvak | `minimizeToTray` | aan | Aan: bij het minimaliseren verdwijnt Moonpool naar het systeemvak en uit de taakbalk. Uit: minimaliseert naar de taakbalk. |
| Altijd op voorgrond | `alwaysOnTop` | uit | Houdt elk Moonpool-venster boven andere vensters. |
| Weergeven in systeemvak | `showInTray` | aan | Houdt het pictogram in het systeemvak zichtbaar. |
| Weergeven in taakbalk | `showInTaskbar` | aan | Houdt de knop in de taakbalk zichtbaar. |
| CPU-/geheugenstatusbalk tonen | `showStatusbar` | aan | Live balk met CPU en geheugen onderaan de hub. |
| MCP-processen tonen | `showMcpProcesses` | aan | Toont het MCP-proces van een app als MCP-subrij in de zijbalk zolang de MCP-tools van die app worden gebruikt. |
| Transparantie van de achtergrond | `transparency` | 0% | Schuifregelaar van 0 tot 90 in stappen van 5. Zie [Thema's, taal en transparantie](/nl/using/themes-and-language/#transparantie). |
| Bij het opstarten controleren op updates | `checkOnStartup` | aan | Controleert bij het starten op GitHub of er een nieuwere versie is en toont een banner als die er is. Zie [Bijwerken](/nl/data/updating/). |

### Vergrendeling van systeemvak en taakbalk

Minstens één van **Weergeven in systeemvak** en **Weergeven in taakbalk** moet aan blijven, anders
zou een verborgen venster geen weg terug hebben. Als er maar één aan staat, is het selectievakje
ervan uitgeschakeld totdat je de andere weer aanzet.

## Rechterkolom: logs

| Bedieningselement | Sleutel | Standaard | Wat het doet |
| --- | --- | --- | --- |
| Uitvoerlogs van apps tussen sessies bewaren | `cliLogging` | uit | De terminaluitvoer van de lopende sessie blijft altijd bewaard voor de eigen tabbladen. Aan: logs van oudere sessies blijven op schijf onder `cli-output\`, begrensd door de bewaarlimiet. Uit: ze worden verwijderd de volgende keer dat die app start. |
| Logbewaring per app | `logRetentionMb` | 10 MB | Limiet voor de gezamenlijke logs van elke app. Minimum 1. Uitgeschakeld zolang de schakelaar hierboven uit staat. Het log van de huidige sessie telt mee voor de limiet, maar wordt er nooit door ingekort of verwijderd. |
| Debug-informatie naar een bestand loggen | `debugLogging` | uit | Legt het laden van `apps.json`, starts en fouten vast in `moonpool.log`. |

Onder elke loggroep toont een padveld de locatie, met twee knoppen:

- De eerste knop opent de map in de bestandsbeheerder (**CLI-logmap openen** voor `cli-output\`, **Logbestand openen** voor `moonpool.log`).
- De tweede knop zet het pad op het klembord (**Pad van CLI-logmap kopiëren**, **Pad van logbestand kopiëren**).

Logbestandsindelingen, de scheidingslijn bij herstarten en de bewaarregels staan in
[Logs](/nl/data/logs/).

Als een selectievakje niet kan worden opgeslagen, meldt een rode melding bovenaan het venster dat
en springt het selectievakje terug.

## Venster Over

Open **Over** via het menu "...".

![Venster Over met de versieregel, koppelingen en de knoppen Controleren op updates en Sluiten](../../../../assets/screenshots/about-window.png)

Het toont:

- De versie en de builddatum.
- Koppelingen naar de projectsite, de GitHub-repository en het contactadres.
- **Controleren op updates**. Als er een nieuwere versie is, wordt die gedownload, gecontroleerd en geïnstalleerd, waarna Moonpool opnieuw start. Anders meldt het dat je de nieuwste versie hebt, of de fout als de controle is mislukt.
- Vermelding van de bibliotheken waarmee Moonpool is gebouwd, en de auteur.

Escape sluit het. Over volgt de instellingen voor thema, transparantie en taal live.
