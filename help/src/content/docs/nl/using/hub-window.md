---
title: "Wegwijs in het Moonpool-hubvenster"
description: "Een rondleiding door de Moonpool-hub: zijbalk, terminaltabbladen, statusbalk, het menu, de foutbanner van apps.json en hoe de grootte en positie worden onthouden."
---

![De hub met drie apps in gebruik: twee actieve web-app-rijen (1), de tabbladenbalk (2), de live uitvoer van de actieve app (3) en de statusbalk (4)](../../../../assets/screenshots/hub-window.png)

1. Twee van de actieve apps: een verlichte statusstip en een stopknop in plaats van afspelen.
2. De tabbladenbalk, één tabblad per geopende app, met het actieve tabblad gemarkeerd.
3. Live uitvoer van de actieve app.
4. De statusbalk met CPU en geheugen.

## Indeling

| Gebied | Wat het bevat |
| --- | --- |
| Titelbalk | Minimaliseren, maximaliseren en sluiten. |
| Zijbalk | Het filtervak, het menu **...** en je apps gegroepeerd op `group`. Zie [Zijbalk en menu's](/nl/using/sidebar-and-menus/). |
| CLI-paneel | Eén terminaltabblad per geopende app. Zie [Terminaltabbladen](/nl/using/terminal-tabs/). |
| Statusbalk | Live CPU en geheugen, onderaan. |

Sleep de scheidingslijn tussen de zijbalk en het CLI-paneel om de zijbalk te vergroten of te verkleinen.

## Statusbalk

![De statusbalk: CPU-balken per kern links, de geheugenbalk rechts](../../../../assets/screenshots/status-bar.png)

De statusbalk toont één dunne balk per CPU-kern (houd de aanwijzer erboven voor "CPU-gebruik per kern"), daarna een geheugenbalk met het label `used/total GB`. Schakel hem uit met **CPU-/geheugenstatusbalk tonen** in Instellingen (`showStatusbar`; zie [Venster Instellingen](/nl/using/settings/)). De wijziging geldt direct.

## Het menu ...

De knop **...** links van het filtervak opent het menu.

![De menuknop ... (1) en het vak Apps filteren (2) bovenaan de zijbalk](../../../../assets/screenshots/sidebar-filter-and-menu.png)

1. De menuknop **...**.
2. Het vak **Apps filteren...**.

| Item | Functie |
| --- | --- |
| App toevoegen | Opent de app-editor. Zie [Apps toevoegen](/nl/apps/add-an-app/). |
| apps.json bewerken | Opent `apps.json` in je standaardeditor om met de hand te bewerken. |
| Opnieuw laden | Leest `apps.json` opnieuw van schijf (ook F5, zie [Sneltoetsen en zoom](/nl/using/keyboard-shortcuts/)). |
| Instellingen | Opent het venster Instellingen. |
| Help | Opent deze help. |
| Over | Opent het venster Over, met de versie en de controle op updates. |
| Moonpool installeren… | Alleen Windows. Opent het installatievenster, om de app te installeren of een draagbare kopie te maken. Zie [Installeren](/nl/getting-started/install/) en [Draagbare modus](/nl/data/portable-mode/). |

### Waarschuwing bij poortconflict

Als twee apps in `apps.json` dezelfde `port` gebruiken, verschijnt onderaan het menu een waarschuwingsrij, bijvoorbeeld:

```text
poort 3000: App A en App B
```

Houd de aanwijzer erboven voor de volledige zin. Los het conflict op in `apps.json` of in de app-editor; de rij verdwijnt zodra er geen poort meer wordt gedeeld.

## Wanneer apps.json een fout bevat

Als Opnieuw laden (of F5) ontdekt dat `apps.json` niet meer kan worden gelezen of gevalideerd,
behoudt Moonpool de lijst die het al had. Een banner bovenaan de zijbalk meldt "apps.json bevat
een fout; de laatst geladen lijst wordt getoond.", gevolgd door de fout (houd de aanwijzer erboven
voor de volledige tekst). De lijst eronder is gedimd maar werkt nog, dus je kunt apps gewoon
starten en stoppen. **apps.json bewerken** in de banner opent het bestand; herstel het en kies
**Opnieuw laden**, en de banner verdwijnt.

Zolang het bestand niet opnieuw laadt, slaat Moonpool geen wijzigingen op uit de app-editor,
hernoemen, verwijderen of pictogram kiezen, zodat een kapot bestand nooit wordt overschreven.

Als het bestand al kapot is wanneer Moonpool start, is er geen eerdere lijst om te behouden: de
banner meldt dat er geen apps zijn geladen en de zijbalk is leeg. Herstel het bestand en laad
opnieuw, of ga terug naar een recente goede kopie (zie [Als het bestand defect is](/nl/apps/apps-json/#als-het-bestand-fout-is)).

## Leeg scherm

Zolang er geen tabblad open is, toont het CLI-paneel "Kies links een app om die te starten." Het
bevat ook twee dingen die alleen verschijnen terwijl er geen tabblad open is:

- **De updatebanner**, als bij het opstarten een nieuwere versie is gevonden. Zie
  [Bijwerken](/nl/data/updating/).
- **Prompt kopiëren**, een kant-en-klare prompt die het instellen van je apps aan een AI-agent
  overdraagt. Zie [AI-agents: snelstart](/nl/automation/quick-start/#prompt-kopiëren).

Het pictogram in het systeemvak, sluiten, minimaliseren, Afsluiten en altijd op de voorgrond staan
op [Systeemvak, sluiten en minimaliseren](/nl/using/tray-and-closing/).

## Grootte, positie en gemaximaliseerde status

Moonpool onthoudt de grootte, de positie en de gemaximaliseerde status van het hubvenster tussen
sessies. De eerste keer opent het op 1200x780, op de positie die Windows kiest.

Als de opgeslagen positie op geen enkel aangesloten beeldscherm meer ligt (bijvoorbeeld een
losgekoppelde monitor), wordt de positie genegeerd en wordt de opgeslagen grootte gebruikt op de
standaardlocatie. Het bestand is `window-state.json` in de configuratiemap (zie
[Waar de configuratie staat](/nl/apps/apps-json/#waar-de-configuratie-staat)).

De breedte van de zijbalk en of het CLI-paneel is samengeklapt, worden ook onthouden.
