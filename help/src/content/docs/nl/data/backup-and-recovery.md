---
title: "Moonpool back-uppen, apps.json terugdraaien en een opzet herstellen"
description: "Weet wat je moet back-uppen, draai een foute apps.json terug, zet de voorbeeld-apps terug, verplaats een geïnstalleerde opzet naar een draagbare kopie en zie wat verwijderen wist."
---

Alles wat Moonpool bewaart staat op twee plaatsen: de configuratiemap en de dashboardmap.
De paden per modus staan in [Waar de configuratie staat](/nl/apps/apps-json/#waar-de-configuratie-staat).

## De configuratiemap

```text
moonpool-config\
  apps.json            your apps                              back up
  apps.json.history\   the last 10 good apps.json files       back up (optional)
  settings.json        app settings                           back up
  icons\               icon overrides, <id>.png and so on     back up
  cli-output\<id>\     session logs                           disposable
  moonpool.log         debug log                              disposable
  state.json           live status snapshot                   disposable
  dumps\               files written by dump and read-config  disposable
  mcp_seen.json        which apps had an MCP helper           disposable
  window-state.json    hub window size and position           disposable
  AI-README.md         rewritten at every launch              disposable
  webview\             the window's browser profile (Windows) disposable
```

De dashboardmap is `{MP_HOME}\dashboards`: `%USERPROFILE%\.moonpool\dashboards`
geïnstalleerd, `<your .moonpool folder>\dashboards` draagbaar, en `dashboards/` in de
configuratiemap onder Linux. Back up alles van uzelf dat erin staat. De map `examples` ervan hoort bij Moonpool
en wordt bij een update overschreven.

Het thema wordt bewaard in de browseropslag van het venster, niet in een bestand dat je kunt kopiëren. Het
reist niet mee met een back-up; kies het opnieuw na een herstel.

## Back-up maken

1. Sluit Moonpool af, zodat geen enkel bestand half is geschreven.
2. Kopieer `apps.json`, `settings.json` en `icons\` uit de configuratiemap, en je eigen bestanden
   uit `dashboards\`.

```powershell frame="terminal"
$cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
Copy-Item "$cfg\apps.json", "$cfg\settings.json" D:\backup\
Copy-Item "$cfg\icons" D:\backup\ -Recurse
```

Om te herstellen sluit je Moonpool af, kopieer je de bestanden terug en start je het.

## apps.json terugdraaien

Elke geslaagde opslag, schrijfactie van een agent en herstelactie, en elke Opnieuw laden die gewijzigde inhoud vindt,
kopieert de gevalideerde `apps.json` naar `apps.json.history\`, waarbij de nieuwste 10 worden bewaard. Elk bestand
is genoemd naar het tijdstip waarop het is gemaakt, bijvoorbeeld `1767225600000.json`. Er is geen
`apps.json.bak`.

- **Handmatig.** Kopieer een momentopname over `apps.json` heen en kies dan **Opnieuw laden**.

  ```powershell frame="terminal"
  $cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
  Copy-Item "$cfg\apps.json.history\<snapshot>" "$cfg\apps.json"
  ```

- **Vanuit een script.** `moonpool.exe restore-config` toont de momentopnamen;
  `moonpool.exe restore-config 1` herstelt de nieuwste. Zie
  [Opdrachtregel](/nl/automation/command-line/).
- **Vanuit een agent.** `moonpool_restore_config`. Zie [MCP-tools](/nl/automation/mcp-tools/#configuratie).

Er wordt niets automatisch hersteld.

## Een kapot bestand

- **apps.json.** Moonpool overschrijft nooit een kapot bestand. Zie
  [Als het bestand fout is](/nl/apps/apps-json/#als-het-bestand-fout-is).
- **settings.json.** Herstel het, of verwijder het om alle instellingen terug te zetten, en start Moonpool dan opnieuw. Zie
  [settings.json](/nl/data/settings-json/#lezen-en-repareren).

## Terugzetten naar de voorbeelden

Moonpool schrijft zijn voorbeeld-apps alleen als er geen `apps.json` is. Om opnieuw te beginnen sluit je
Moonpool af (of laat je het draaien), hernoemt of verwijder je `apps.json`, en start je Moonpool of kies je
**Opnieuw laden**. Er wordt een nieuwe `apps.json` met de voorbeelden geschreven.

## Van geïnstalleerd naar draagbaar

Een nieuwe draagbare kopie begint met de voorbeeld-apps. Om je eigen apps over te brengen, zie
[Draagbare modus](/nl/data/portable-mode/#draagbaar-kiezen-in-het-installatieprogramma). Kopieer `icons\` en
`settings.json` op dezelfde manier als je ze wilt hebben.

## Verwijderen

Het verwijderen van de geïnstalleerde Moonpool wist de hele map `%USERPROFILE%\.moonpool`,
inclusief de configuratiemap en de dashboards. Maak eerst een back-up. Zie
[Verwijderen](/nl/getting-started/install/#verwijderen). Een draagbare kopie verwijder je door
de map `.moonpool\` ervan te wissen.
