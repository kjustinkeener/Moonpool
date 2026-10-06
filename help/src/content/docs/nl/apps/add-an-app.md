---
title: "Een app of dev-server toevoegen aan Moonpool"
description: "Registreer een lokale app of dev-server met een startcommando, werkmap en omgeving, zodat Moonpool die voor jou kan starten, stoppen en bewaken."
---

Elke app in Moonpool is één item met een startcommando, een werkmap en een optionele
omgeving. Moonpool voert het commando uit in een eigen beheerde terminal.

## Een app toevoegen

1. Open het menu **...** bovenaan de zijbalk en kies **App toevoegen**.
2. Voer een **name** in en kies een **group**.
3. Kies het **type**: `web` (server op een poort), `desktop` (native app), `static` (een pagina) of `cli` (een commando).
4. Stel het **command** en de **cwd** in waarin het wordt uitgevoerd.
5. Vul in wat het type nodig heeft: **port** en **url** voor web, **processName** voor desktop,
   **url** voor static. Een `static`-app met alleen een `url` heeft geen **command** of **cwd** nodig.
6. Sla op. De app verschijnt in de zijbalk. Gebruik de knop **Starten** om de app te starten.

![De typekeuze (1) en het veld port (2) in de app-editor, met cwd en command ertussen](../../../../assets/screenshots/edit-app-type-and-port.png)

1. De keuzelijst **type**; de hint ervan legt uit hoe dat type draait.
2. Het veld **port**, gebruikt door `web`-apps.

Het resultaat is één item in `apps.json`, bijvoorbeeld:

```json title="apps.json"
{
  "id": "my-api",
  "name": "My API",
  "group": "Dev",
  "type": "web",
  "command": "npm run dev",
  "cwd": "C:\\code\\my-api",
  "port": 3000,
  "url": "http://localhost:3000"
}
```

Een klik op de naam van een app opent alleen het terminaltabblad ervan; zie [App-statussen](/nl/support/glossary/#app-statussen).

## De app-editor

- **Group.** Kies een groep uit de lijst, of kies **+ Nieuwe groep...** en typ een naam.
  **terug naar de lijst** keert terug naar de lijst. Een lege groep wordt opgeslagen als `Apps`.
- **Gedimde velden** worden niet gebruikt door het gekozen type. Ze worden wel opgeslagen.
- **Opslaan zonder naam** toont `name is required.` (in de app: "naam is verplicht.")
- **Esc** of het sluiten van de editor met niet-opgeslagen wijzigingen vraagt "Je wijzigingen weggooien?".
- Om een app later te wijzigen gebruik je het potlood op de rij, of klik je er met de rechtermuisknop op en kies je **Bewerken**.

## Handmatig bewerken

Kies **apps.json bewerken** in hetzelfde menu, sla het bestand op en kies dan **Opnieuw laden**. Het
formaat, de validatieregels en de herstelopties staan in het
[Overzicht van de configuratie](/nl/apps/apps-json/).

## Waar je verder kunt kijken

- [App-velden](/nl/apps/fields/): elke sleutel en wat die doet.
- [App-typen](/nl/apps/types/): hoe elk type start en Actief toont.
- [Stoppen en herstarten](/nl/apps/stop-and-restart/): wat je instelt als Stoppen iets laat doorlopen, en waarom Docker-apps zorg vragen.
- [Paden en omgeving](/nl/apps/paths-and-environment/): `{MP_HOME}`, `./`-paden en `env`.
- [Voorbeelden](/nl/apps/examples/): complete items om te kopiëren.
- [Handleidingen](/nl/guides/run-npm-dev-server-in-background-windows/): dev-servers op de achtergrond, Python-scripts, poorten.
- [Draagbare modus](/nl/data/portable-mode/)
- [Bijwerken](/nl/data/updating/)
