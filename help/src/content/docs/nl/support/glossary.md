---
title: "Moonpool-woordenlijst: apps, statussen, bestanden en instellingen"
description: "Heldere definities van de woorden die de Moonpool-help gebruikt voor de onderdelen, app-statussen, bestanden en instellingen, zodat je de rest van de documentatie kunt volgen."
---

## Apps

| Term | Betekenis |
| --- | --- |
| app | Eén ding dat Moonpool beheert: een dev-server, een desktop-app, een pagina of een commando. |
| item (entry) | Het record van een app in `apps.json`. Alleen gebruikt als het over de JSON gaat. |
| apprij | De regel van een app in de zijbalk, met statusstip en bedieningselementen. |
| groep | De kop in de zijbalk waaronder een app staat, uit het veld `group`. |
| type | `web`, `desktop`, `static` of `cli`. Bepaalt welke velden ertoe doen. Zie [App-typen](/nl/apps/types/). |
| id | De vaste sleutel van een app, gebruikt in bestandsnamen, commando's en agent-tools. Zie [De id](/nl/apps/apps-json/#de-id). |

## App-statussen

| Status | Betekenis |
| --- | --- |
| starten | Moonpool heeft de app gestart maar nog niet als actief gezien. Pulserende stip. |
| Actief | De `port` antwoordt, de `processName` bestaat of, als geen van beide is ingesteld, de terminal die Moonpool heeft gestart leeft nog. Egale stip. Zie [Hoe Actief wordt bepaald](/nl/apps/types/#hoe-actief-wordt-bepaald). |
| gestopt | Niets van het bovenstaande. Grijze stip. |
| beheerd (managed) | Moonpool heeft de app in deze sessie gestart. Een actieve app die niet beheerd is, is op een andere manier gestart en Afsluiten laat die met rust. |

Een terminaltabblad en een actieve app zijn aparte dingen. Een klik op de naam van een app opent
alleen het terminaltabblad; de app wordt er nooit mee gestart. Een tabblad sluiten stopt de app nooit.

## Vensters en onderdelen

| Term | Betekenis |
| --- | --- |
| hub | Het resident draaiende Moonpool-proces en het hoofdvenster ervan. In toolnamen heet het de "launcher". |
| hubvenster | Het hoofdvenster: zijbalk links, CLI-paneel rechts. |
| systeemvak | Het pictogram in het systeemvak en het menu ervan (**Moonpool tonen**, **Afsluiten**). |
| zijbalk | De linkerkant van het hubvenster: filtervak, menu **...** en apprijen. |
| CLI-paneel | De rechterkant van het hubvenster, met de terminaltabbladen. |
| terminaltabblad | De terminal van één app in het CLI-paneel. |
| MCP-subrij | Een gedimde rij onder een app die het eigen hulpproces `<exe> mcp` van die app toont. |
| app-editor | Het dialoogvenster App toevoegen en App bewerken. |

## Bestanden en mappen

| Term | Betekenis |
| --- | --- |
| configuratiemap | De map met `apps.json` en de andere bestanden van Moonpool. De token `{MP_DATA}`. Zie [Waar de configuratie staat](/nl/apps/apps-json/#waar-de-configuratie-staat). |
| `{MP_HOME}` | De Moonpool-map: `%USERPROFILE%\.moonpool` bij een installatie, de map `.moonpool\` van een draagbare kopie, de configuratiemap op Linux. |
| sessie | Eén run van de hub, van start tot Afsluiten. |
| sessielog | Het bestand met alles wat een app tijdens één sessie heeft uitgevoerd, onder `cli-output\`. Zie [Logs](/nl/data/logs/). |
| `moonpool.log` | Het eigen debuglog van Moonpool, alleen geschreven als **Debug-informatie naar een bestand loggen** aan staat. |
| dump | Een tekstkopie van een sessielog, gemaakt met het werkwoord `dump`. |
| snapshot | Een kopie van een goede `apps.json` in `apps.json.history\`. Zie [Back-up en herstel](/nl/data/backup-and-recovery/). |

## Modi

| Term | Betekenis |
| --- | --- |
| geïnstalleerd | Een Moonpool in `%USERPROFILE%\.moonpool`, met snelkoppeling in het Startmenu en item in Programma's toevoegen/verwijderen. Alleen Windows. |
| draagbaar | Een Moonpool in een door jou gekozen map `.moonpool\`, herkenbaar aan een bestand `moonpool.portable`. Zie [Draagbare modus](/nl/data/portable-mode/). |
| kopie | Eén Moonpool-map, geïnstalleerd of draagbaar. Elke kopie draait zelfstandig. |

## Stoppen en automatisering

| Term | Betekenis |
| --- | --- |
| `killMode` | De extra stap die Stoppen uitvoert nadat de terminal van de app is beëindigd. Zie [Stoppen en herstarten](/nl/apps/stop-and-restart/). |
| `stopCommand` | Het commando dat Stoppen uitvoert als `killMode` gelijk is aan `command`. |
| `processName` | De procesnaam waarop Moonpool let, en die in de modus `processName` wordt beëindigd. |
| besturingskanaal | De named pipe (Windows) of Unix-socket (Linux, macOS) waarop de hub antwoordt. Zie [Besturingswerkwoorden](/nl/automation/control-verbs/). |
| werkwoord (verb) | Een commandowoord zoals `launch` of `reload`, opgegeven op de opdrachtregel of via het besturingskanaal. |
| ticket | Een sleutel die je met `--ticket` meegeeft om het resultaat van een commando uit `state.json` te lezen. |
| token | De versiestempel van `apps.json` die een configuratieschrijfactie moet meedragen. |
| MCP-hulpproces (shim) | Een proces `<exe> mcp` dat een AI-host start om de eigen tools van een app te bereiken. |
