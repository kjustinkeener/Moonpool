---
title: "Moonpool bedienen vanaf de opdrachtregel"
description: "Stuur een draaiende Moonpool aan met moonpool.exe-verbs vanuit een terminal of script, label een commando met een ticket en lees de uitkomst uit state.json."
---

Wanneer je een `moonpool.exe` opnieuw uitvoert terwijl diezelfde Moonpool al draait, opent dat geen
tweede venster. Het tweede proces geeft zijn argumenten via het
[besturingskanaal](/nl/automation/control-verbs/) door aan het draaiende proces en sluit af. Moonpool moet al draaien:
als er niets resident is, start hetzelfde commando een nieuwe Moonpool en wordt de verb niet uitgevoerd.

"Dezelfde Moonpool" betekent dezelfde map. De geïnstalleerde Moonpool en elke draagbare kopie draaien
elk op zichzelf, dus een commando bereikt de kopie waarvan je de `moonpool.exe` hebt uitgevoerd, nooit een andere.
Zie [Draagbare modus](/nl/data/portable-mode/#meerdere-kopieën-tegelijk).

Gebruik het pad van de kopie die je bedoelt. Voor de geïnstalleerde:

```powershell frame="terminal"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
```

Als er meerdere kopieën draaien, toont `Get-Process moonpool` ze allemaal, dus kies op `Path`
in plaats van de eerste te nemen. Het toont ook inactieve `moonpool.exe mcp`-helpers die MCP-hosts
hebben gestart, dus een proces `moonpool` bewijst niet dat een hub draait. Vraag het aan het besturingskanaal
met `ping` ([Besturings-verbs](/nl/automation/control-verbs/)).

## Verbs

De verb is niet hoofdlettergevoelig. `<id>` is de `id` van een app uit `apps.json`.

| Commando | Effect |
| --- | --- |
| `moonpool.exe` | Geen verb: brengt het venster naar voren. |
| `moonpool.exe show` | Brengt het venster naar voren. |
| `moonpool.exe launch <id>` | Start de app en opent het terminaltabblad ervan. |
| `moonpool.exe stop <id>` | Stopt de app. |
| `moonpool.exe restart <id>` | Stoppen, wachten tot de poort en het proces vrij zijn, starten. |
| `moonpool.exe reload` | Leest `apps.json` opnieuw. |
| `moonpool.exe refresh-icons` | Haalt elk pictogram opnieuw op. |
| `moonpool.exe help` | Opent het Help-venster. |
| `moonpool.exe quit` | Sluit Moonpool af, net als het systeemvakmenu ("Afsluiten"). |
| `moonpool.exe dump <id> [out-path]` | Zonder `out-path` meldt het het pad van het log van de app voor deze sessie. Met `out-path` kopieert het het log daarheen als platte tekst zonder ANSI-codes. |
| `moonpool.exe paths` | Meldt de configuratiemap, `apps.json`, `state.json`, het log, de dumpmap, de pictogrammap, de draagbare vlag en het exe-pad die de draaiende Moonpool gebruikt. |
| `moonpool.exe read-config` | Schrijft `dumps\read-config.json` in de configuratiemap, met `token`, `valid`, `error`, `path` en `manifest_text` (de exacte inhoud van `apps.json`). |
| `moonpool.exe write-config <file> [token]` | Vervangt `apps.json` door het manifest in `<file>`, als het manifest geldig is en, wanneer `token` is opgegeven, `apps.json` er nog mee overeenkomt. |
| `moonpool.exe restore-config [index or filename]` | Zonder argument schrijft het de lijst met momentopnamen naar `dumps\restore-config.json`. Met een argument herstelt het die momentopname als die geldig is. |

```powershell frame="terminal"
& $mp restart my-app
```

```powershell frame="terminal"
& $mp dump my-app C:\temp\my-app.log
```

Een onbekende verb wordt genegeerd. Het programma heeft ook eigen opstartargumenten:
`moonpool.exe mcp` ([MCP-installatie](/nl/automation/mcp-setup/)), `--uninstall` (gebruikt door Programma's
toevoegen/verwijderen) en `--wait-pid <pid>` (gebruikt wanneer Moonpool zichzelf opnieuw start). Deze worden alleen
gehonoreerd als eerste argument, dus een app-id zoals `--uninstall` kan ze niet activeren.

## De uitkomst lezen

De opdrachtregel drukt niets af, dus label een commando met `--ticket <key>` (elke unieke sleutel, op
elke positie) en lees het resultaat uit `state.json` in de configuratiemap. Dat is
`%USERPROFILE%\.moonpool\moonpool-config\` bij een geïnstalleerde, `<your .moonpool folder>\moonpool-config\`
bij een draagbare kopie en `~/.config/Moonpool/` onder Linux (zie
[Overzicht van de configuratie](/nl/apps/apps-json/#waar-de-configuratie-staat)). `show` en `quit`
schrijven geen ticket.

```powershell frame="terminal"
& $mp launch my-app --ticket t1
```

`state.json` heeft `apps`, `statuses` (`id`, `running`, `managed`, `mcpRunning`, `mcpSeen` per
app) en `tickets`. De draaiende Moonpool herschrijft het om de paar seconden en na elk
commando, en verwijdert het niet wanneer het afsluit, dus een achtergebleven bestand betekent niet dat Moonpool
draait. Om te vragen of het draait, of om de live app-lijst te krijgen, gebruik je de verbs `ping`
en `list` van het besturingskanaal ([Besturings-verbs](/nl/automation/control-verbs/)) of de MCP-tools. Poll je
ticket tot `status` niet `pending` is:

| `status` | Betekenis |
| --- | --- |
| `pending` | Ontvangen; Moonpool is er nog mee bezig. |
| `ok` | Klaar. Bij `dump`, `read-config`, `write-config`, `restore-config` en `paths` bevat `detail` het pad, token of rapport. |
| `error` | Mislukt; `detail` zegt waarom, bijvoorbeeld `unknown app id: x`, `did not reach running in time`, `unknown command`. |

Elk ticket is `{ ticket, action, arg, status, detail, ts }` met `ts` in Unix-milliseconden:

```json title="state.json (tickets entry)"
{
  "ticket": "t1",
  "action": "launch",
  "arg": "my-app",
  "status": "error",
  "detail": "did not reach running in time",
  "ts": 1767225600000
}
```

Afgeronde tickets worden na 24 uur verwijderd, en de lijst wordt ingekort richting 50 items zodra
afgeronde tickets minstens 5 minuten oud zijn.

Een agent die MCP ondersteunt kan het pollen overslaan: zie [MCP-installatie](/nl/automation/mcp-setup/).

## Zie ook

- [AI-agents: snelstart](/nl/automation/quick-start/)
- [Besturings-verbs](/nl/automation/control-verbs/)
