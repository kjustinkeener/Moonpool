---
title: "Je eerste app toevoegen en uitvoeren in Moonpool"
description: "Ga in een paar minuten van de eerste start naar één draaiende eigen app: toevoegen, starten, stoppen, en later de hub en deze help weer terugvinden."
---

## 1. Moonpool starten

Voer op Windows `moonpool.exe` uit en klik op **Moonpool installeren** (zie
[Installeren](/nl/getting-started/install/)). Start op Linux de AppImage of het
geïnstalleerde pakket.

De eerste keer dat Moonpool draait, vult het de zijbalk met voorbeeld-apps (Kladblok op
Windows, een shell, een kleine webserver en de meegeleverde dashboards). Ze werken zoals ze
zijn (de webserver heeft Python nodig), dus je kunt ze uitproberen en daarna bewerken of
verwijderen. Ook plaatst Moonpool een pictogram in het systeemvak. Zie je het pictogram op
Windows niet, klik dan op de pijl **^** rechts in de taakbalk.

## 2. Je app toevoegen

1. Open het menu **...** bovenaan de zijbalk en kies **App toevoegen**.
2. Voer een **name** in. De groep begint als `Web apps`; behoud die of kies een andere.
3. Laat **type** op `web` staan voor een dev-server.
4. Stel **cwd** in op je projectmap en **command** op wat je typt om de app te starten,
   bijvoorbeeld `npm run dev`.
5. Stel **port** in op de poort waarop de app luistert, en **url** op de pagina die moet worden geopend.
6. Sla op.

De details van elk veld staan in [Apps toevoegen](/nl/apps/add-an-app/).

## 3. Starten

Klik op de knop **Starten** van de app (het afspeelpictogram op de rij). Het terminaltabblad
van de app opent en toont de uitvoer. De statusstip pulseert terwijl de app start en wordt
egaal zodra de poort antwoordt. Als **openBrowser** aan staat, wordt de pagina geopend.

Een klik op de naam van de app opent alleen het terminaltabblad. De app wordt er nooit mee gestart.

## 4. Stoppen

Klik op de knop **Stoppen** (het vierkant) op de rij. De stip wordt grijs.

Blijft er na Stoppen iets draaien, zie dan [Stoppen en herstarten](/nl/apps/stop-and-restart/).

## Laat een agent het doen

Zolang er geen tabblad open is, toont het CLI-paneel een knop **Prompt kopiëren**. Plak de
prompt in een AI-agent en die zoekt je apps op en voegt ze toe. Zie
[AI-agents: snelstart](/nl/automation/quick-start/).

## De hub later terugvinden

- Klik met links op het pictogram in het systeemvak om de hub te tonen. Rechtsklikken opent
  een menu met **Moonpool tonen** en **Afsluiten**.
- Standaard sluit het sluiten van het venster Moonpool af. Zet **Sluiten naar systeemvak** aan
  in Instellingen om het venster in plaats daarvan naar het systeemvak te verbergen en
  Moonpool te laten doorlopen. Zie
  [Systeemvak, sluiten en minimaliseren](/nl/using/tray-and-closing/).

## Hulp krijgen

**Help**, in het menu **...** bovenaan de zijbalk, opent deze help in een eigen venster.
Het werkt offline en komt altijd overeen met de versie die je gebruikt.

![Helpvenster met de sectienavigatie omlijnd aan de linkerkant en een pagina aan de rechterkant](../../../../assets/screenshots/help-window.png)

## Volgende

- [Apps toevoegen](/nl/apps/add-an-app/)
- [Probleemoplossing](/nl/support/troubleshooting/)
