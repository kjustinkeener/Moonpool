---
title: "Naslag van het besturingskanaal en de verbs van Moonpool"
description: "Hoe het besturingskanaal van Moonpool (named pipe of Unix-socket) werkt, het protocol ervan en elke verb waarop de draaiende app antwoordt, met argumenten en antwoorden."
---

## Waar het luistert

Elke Moonpool-kopie heeft een eigen kanaal, dus de geïnstalleerde Moonpool en alle draagbare kopieën
kunnen naast elkaar draaien zonder voor elkaar te antwoorden. Onder Windows luistert de geïnstalleerde Moonpool
op de named pipe `\\.\pipe\moonpool`. Een draagbare kopie voegt een id toe die uit zijn
map is afgeleid: `\\.\pipe\moonpool-<id>`.

`<id>` bestaat uit 8 hexadecimale cijfers, afgeleid van het pad van de map `moonpool-config` van de kopie, dus hij blijft
hetzelfde voor die map bij herstarts en updates, en verandert als je de map verplaatst. De
`moonpool.exe` van een kopie, inclusief `moonpool.exe mcp`, vindt altijd het kanaal van zijn eigen kopie.

Onder Linux en macOS luistert het in plaats daarvan op een Unix-domeinsocket, met modus `0600`:

| Geval | Socketpad |
| --- | --- |
| Normaal | `$XDG_RUNTIME_DIR/moonpool.sock` wanneer die variabele is ingesteld, anders `moonpool.sock` in de configuratiemap van Moonpool |
| Draagbare modus | `moonpool.sock` in de configuratiemap van de draagbare kopie, zodat een draagbare kopie nooit botst met een geïnstalleerde |
| Pad te lang voor een socket (ongeveer 100 tekens) | `/tmp/moonpool-<uid>/moonpool.sock`, in een map die alleen je kunt openen (`moonpool-<id>.sock` voor een draagbare kopie) |

Een socketbestand dat door een crash is achtergebleven wordt bij de volgende start gedetecteerd en vervangen. Een socket waarop
nog iets antwoordt wordt nooit overgenomen. Het bestand wordt verwijderd wanneer Moonpool normaal afsluit.

Het kanaal is ook de manier waarop de [MCP-server](/nl/automation/mcp-setup/) weet of Moonpool
draait: als een `ping` wordt beantwoord, is dat zo, en als de pipe of socket ontbreekt, niet. Dezelfde
verbs zijn ook bereikbaar vanaf de [opdrachtregel](/nl/automation/command-line/), behalve de
diagnostische verbs hieronder.

## Protocol

Eén JSON-object per regel erin, één JSON-regel eruit, op volgorde. Een verbinding kan veel
verzoeken dragen.

```json title="request"
{"cmd": "restart", "args": ["my-app"]}
```

```jsonl title="replies"
{"ok": true, "result": "..."}
{"ok": false, "error": "unknown app id: ..."}
```

Een verzoek en het antwoord ervan vanuit PowerShell:

Gebruik voor een draagbare kopie de pipenaam ervan (`moonpool-<id>`, getoond door de verb `paths`) in
plaats van `moonpool`.

```powershell frame="terminal"
$p = New-Object System.IO.Pipes.NamedPipeClientStream('.', 'moonpool', 'InOut')
$p.Connect(2000)
$w = New-Object System.IO.StreamWriter($p); $w.AutoFlush = $true
$r = New-Object System.IO.StreamReader($p)
$w.WriteLine('{"cmd":"ping"}')
$r.ReadLine()
```

```json title="reply"
{"ok":true,"result":"pong"}
```

- `args` is een lijst met strings en mag worden weggelaten. Andere velden worden genegeerd.
- `result` is een string of null. Verbs die gestructureerde gegevens teruggeven, geven die als JSON-string
  terug.
- Een regel die geen geldige JSON is krijgt `{"ok": false, "error": "bad request: ..."}`.
- Een onbekende `cmd` krijgt `unknown cmd: <name>`.
- Een verb die via het venster loopt (`launch`, `stop`, `restart`, `reload`,
  `refresh-icons`, `help`, `open-window`) wordt beantwoord wanneer de actie klaar is, of met een time-outfout
  na 45 s. Als de UI van het hubvenster niet is geladen, mislukt het meteen met `frontend not
  loaded`.
- Een Moonpool die start terwijl een vorige nog aan het afsluiten is, probeert ongeveer 8 seconden opnieuw het kanaal te binden.
  Lukt dat nog steeds niet, dan logt het dat en blijft het draaien zonder kanaal.

## Verbs

| Verb | Args | Resultaat |
| --- | --- | --- |
| `ping` | geen | `pong`. Alleen kanaal. |
| `list` | geen | JSON-string `{"apps": [...], "statuses": [...]}` gelezen uit het geheugen van de draaiende hub, dezelfde vorm van `apps` en `statuses` als `state.json`. Voegt `"statusNotReady": true` toe wanneer apps zijn geregistreerd maar de eerste statuscontrole nog niet is uitgevoerd. Zolang `apps.json` niet laadt, wordt `"manifestError": "<message>"` toegevoegd (de apps zijn dan de laatste lijst die is geladen) en, wanneer sinds het opstarten geen lijst is geladen, `"manifestLoaded": false`. Alleen kanaal. |
| `show` | geen | null. Brengt het venster naar voren. |
| `quit` | geen | null. Sluit Moonpool af. |
| `launch` | `<id>` | null bij succes, of `opened` voor een `static`-item met alleen een `url`. Fouten: `unknown app id: <id>`, `did not reach running in time`. |
| `stop` | `<id>` | null bij succes, of `stopped` voor een `static`-item met alleen een `url`. Fout: `still running after stop`. |
| `restart` | `<id>` | Dezelfde resultaten en fouten als `launch`. |
| `reload` | geen | null bij succes. |
| `refresh-icons` | geen | null bij succes. |
| `help` | geen | null. Opent het Help-venster. |
| `dump` | `<id>` [`out-path`] | Pad van het sessielog van de app, of van de kopie in platte tekst op `out-path`. |
| `paths` | geen | Rapport van meerdere regels met de mappen en de exe die de hub gebruikt. |
| `read-config` | geen | Pad van `dumps\read-config.json`, dat `token`, `valid`, `error`, `path`, `manifest_text` bevat. |
| `write-config` | `<source-file>` [`token`] | Het nieuwe versietoken. Fouten: `stale token: ...`, `rejected invalid manifest: ...`, `cannot read source ...`. |
| `restore-config` | [`index` of `filename`] | Zonder argument: pad van `dumps\restore-config.json` (`count`, `snapshots`). Met een argument: `restored <file> (<n> apps); new version token <token>`. |
| `argv` | de opdrachtregelargumenten | null, meteen. Voert ze precies uit zoals een tweede `moonpool.exe <args>` van deze kopie zou doen, inclusief `--ticket`. Zo geeft die tweede start zijn argumenten door voordat hij afsluit. |

Voorbeelduitwisselingen:

```jsonl title="request, reply"
{"cmd": "launch", "args": ["nope"]}
{"ok": false, "error": "unknown app id: nope"}

{"cmd": "restore-config", "args": ["1"]}
{"ok": true, "result": "restored 1767225600000.json (6 apps); new version token <token>"}
```

`write-config` en `restore-config` laden het nieuwe manifest meteen, leggen een momentopname vast in
`apps.json.history\` en vernieuwen het venster.

## Diagnostische verbs (testen)

Alleen kanaal: de opdrachtregel accepteert deze niet. Alle werken onder Windows, Linux en macOS
behalve `screenshot`, dat alleen onder Windows werkt en elders antwoordt met `screenshot is not supported on this
platform (Windows only)`.

| Verb | Args | Resultaat |
| --- | --- | --- |
| `screenshot` | [`window`] [`max_dim`] | Alleen Windows. Base64 van een PNG van dat Moonpool-venster (standaard `main`). Optionele `max_dim` begrenst de langste zijde in pixels (begrensd tot 320-2400, standaard 320; de MCP-tool gebruikt altijd de standaard). Een `max_dim` die geen geheel getal is, is een fout. Toegestane vensters: `main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`. Fouten: `unknown window '<name>'`, `window '<name>' is not open`. Niet naar schijf geschreven. |
| `open-window` | `<kind>` [`<id>`] | null. Opent een venster zoals het menu-item ervan dat doet. `kind`: `settings`, `about`, `installer`, `help`, `themes`, `editor` (optionele `<id>` opent het dialoogvenster App bewerken van die app, geen id opent App toevoegen), `terminal` (`<id>` verplicht: selecteert het terminaltabblad van die app en vergroot de hub zodat het CLI-paneel verschijnt; start de app niet), `cli` (vergroot alleen de hub). Fouten: `unknown window kind '<kind>'`, `terminal needs an app id`, `unknown app id: <id>`. Beantwoord via het hubvenster, net als `launch`. |
| `window-state` | [`window`] | JSON-string: `{"open":false}`, of `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. |
| `stop-mcp` | `<id>` | `stopped`. Beëindigt de helper `<processName> mcp` van de app, niet de app. Fouten: `missing app id`, `unknown app id: <id>`. |
| `reset-mcp-seen` | [`<id>`] | `<id>: cleared` of `<id>: was not marked seen`; zonder id `cleared <n> entries`. Wist de onthouden waarnemingen van MCP-helpers. |

De `--ticket` van de opdrachtregel en de uitkomstrecords in `state.json` horen bij het andere kanaal;
zie [Opdrachtregel](/nl/automation/command-line/#de-uitkomst-lezen). Verzoeken via het kanaal krijgen hun
antwoord in het antwoord zelf.

## Zie ook

- [Opdrachtregel](/nl/automation/command-line/)
- [AI-agents: snelstart](/nl/automation/quick-start/#dezelfde-actie-op-drie-manieren)
