---
title: "De zijbalk lezen: statusstippen, groepen, filter en rijmenu"
description: "Leer wat elke zijbalkrij toont, hoe statusstippen en groepen werken, hoe je apps filtert, het contextmenu van een rij gebruikt en de zijbalk vergroot of verkleint."
---

De zijbalk toont elke app in `apps.json`, gegroepeerd op het veld `group` van elke app. Zie [App-velden](/nl/apps/fields/).

## Rijen

Elke rij toont een statusstip, het pictogram van de app (of een typesymbool als er geen pictogram is), de naam, de poort indien ingesteld (`:3000`) en bedieningselementen.

| Stip | Betekenis |
| --- | --- |
| Egaal | actief |
| Pulserend | starten: Moonpool heeft de app gestart maar nog niet als actief gedetecteerd |
| Grijs | gestopt |

Houd de aanwijzer boven de stip voor het woord.

![De zijbalk met twee actieve web-apps omlijnd: verlichte stippen en Stoppen-knoppen](../../../../assets/screenshots/sidebar-running-narrow.png)

1. Twee actieve apps. Hun stippen zijn verlicht en Stoppen (het vierkant) vervangt Starten.

| Bedieningselement | Functie |
| --- | --- |
| Potlood | De app bewerken. |
| Herstarten | Stoppen en opnieuw starten. Bij een gestopte app wordt deze gewoon gestart. |
| Starten (afspelen) | Start de app en opent het terminaltabblad. Getoond als de app gestopt is. |
| Stoppen (vierkant) | Stopt de app. Getoond terwijl de app actief is of start. |

![Eén actieve rij, uitvergroot: statusstip, typepictogram, naam en poort, daarna de knoppen bewerken, herstarten en stoppen](../../../../assets/screenshots/sidebar-row-controls.png)

1. Statusstip (verlicht terwijl de app actief is).
2. Typepictogram.
3. Bewerken (potlood).
4. Herstarten.
5. Stoppen (getoond in plaats van Starten terwijl de app actief is).

Zolang een start of stop bezig is, worden de bedieningselementen vervangen door een draaiend symbool (`Bezig...`).

Een klik op de **naam** van een app opent of focust het terminaltabblad en start nooit iets. Een tabblad van een gestopte app toont het log van deze sessie. Gebruik Starten of Herstarten om de app te starten. Een `static`-app met alleen een `url` en geen `command` heeft geen terminal: Starten opent de URL in je browser.

### Tooltip

Als je de aanwijzer boven de naam houdt, wordt de `note` van de app getoond als die er is, anders de naam. Stel `note` in de editor of in `apps.json` in.

### MCP-subrij

Als een AI-client de eigen MCP-tools van een app heeft gebruikt, verschijnt onder de app een gedimde
subrij `MCP-server`. De stip is verlicht en de tooltip luidt "MCP-client verbonden" zolang de client
is verbonden. Een stopknop beëindigt dat proces.

De rij vindt het proces via `processName` plus het argument `mcp`, of via het patroon
`mcpProcessName` van de app als dat is ingesteld. Zie [velden](/nl/apps/fields/#mcpprocessname).

Verberg deze rijen met **MCP-processen tonen** in Instellingen. Zie
[MCP-installatie](/nl/automation/mcp-setup/#apps-die-een-eigen-mcp-server-hebben).

## Groepen

![Inactieve zijbalk met de vijf groepskoppen omlijnd, elk met het aantal apps rechts](../../../../assets/screenshots/sidebar-groups-narrow.png)

- Klik op een groepskop om de groep samen te klappen of uit te klappen. Het getal ernaast is het aantal getoonde apps. Samengeklapte groepen worden onthouden.
- Binnen een groep staat de laatst gestarte app bovenaan. Apps die nooit zijn gestart, behouden hun volgorde uit `apps.json`. Een zojuist gestarte app licht op en stijgt naar boven.

## Filtervak

Typ in **Apps filteren...** om de lijst te beperken. Het komt overeen met de naam van de app en de naam van de groep, zonder onderscheid tussen hoofdletters en kleine letters. Als niets overeenkomt, toont de lijst:

```text
Geen app komt overeen met “<tekst>”.
```

De app toont gekrulde aanhalingstekens rond de tekst, hier en in de melding bij Verwijderen hieronder.

## Rechtsklikmenu

Klik met rechts op een rij voor:

| Item | Functie |
| --- | --- |
| Bewerken | Opent de app-editor. |
| Naam wijzigen | Verandert de naam in een bewerkbaar veld. **Enter** of ergens anders klikken slaat op, **Esc** annuleert. Een lege of ongewijzigde naam wordt genegeerd. |
| Pictogram kiezen... | Kies een afbeeldingsbestand (png, jpg, jpeg, gif, svg, webp, ico) om als pictogram te gebruiken. |
| Verwijderen | Vraagt `“<naam>” verwijderen?` en verwijdert het item uit `apps.json`. Als Moonpool de app laat draaien, wordt die eerst gestopt. |

**Esc** sluit het menu zonder iets te doen.

## Formaat wijzigen

Sleep de scheidingslijn tussen de zijbalk en het CLI-paneel. De breedte is beperkt tot 180 tot 620 px (standaard 280) en wordt onthouden. De scheidingslijn is vergrendeld terwijl het CLI-paneel is samengeklapt. Zie [Terminaltabbladen](/nl/using/terminal-tabs/).
