---
title: "Paden, MP_HOME-tokens en omgevingsvariabelen gebruiken in apps"
description: "Gebruik de tokens {MP_HOME} en {MP_DATA} en relatieve ./-paden in app-items, zie welke velden ze uitbreiden en stel env en de werkmap in."
---

## Tokens

| Token | Wordt uitgebreid tot |
| --- | --- |
| `{MP_HOME}` | Draagbaar: de map met `moonpool.exe` (de map `.moonpool\`). Geïnstalleerd onder Windows: `%USERPROFILE%\.moonpool`. Linux: `$XDG_CONFIG_HOME/Moonpool`, anders `~/.config/Moonpool`, dezelfde map als `{MP_DATA}`. |
| `{MP_DATA}` | De configuratiemap, die met `apps.json`. |

Een token dat niet kan worden omgezet blijft staan zoals geschreven.

## Welke velden uitbreiden

| Veld | Tokens | Beginnend met `./` of `.\` |
| --- | --- | --- |
| `cwd` | ja | ja, verankerd aan `{MP_HOME}` |
| `command` | ja | nee |
| `stopCommand` | ja | nee (het draait in `cwd`, dat is verankerd) |
| `url` | ja | nee |
| `icon` | ja | ja, verankerd aan `{MP_HOME}` |
| `env`-waarden, `processName`, `note` | nee | nee |

Een relatief pad zonder `./` (zoals `apps\tool`) blijft ongemoeid en wordt omgezet vanuit de
eigen werkmap van Moonpool, wat zelden is wat je wilt. Gebruik liever `./` of een token.

```text
./apps/notes                       anchored to {MP_HOME}
{MP_HOME}\apps\notes\notes.exe     token
{MP_DATA}\dumps                    token
apps\tool                          left alone, resolves against Moonpool's working folder
```

```json title="apps.json"
{ "id": "notes", "name": "Notes", "group": "Desktop apps", "type": "desktop",
  "cwd": "./apps/notes",
  "command": "{MP_HOME}\\apps\\notes\\notes.exe",
  "processName": "notes" }
```

Beide vormen blijven werken wanneer je de draagbare map verplaatst. Een vast pad zoals
`C:\tools\notes` verhuist niet mee. In de draagbare modus markeert het dialoogvenster App bewerken absolute `cwd`- en
`url`-waarden met een badge "niet portable". Zie [Draagbare modus](/nl/data/portable-mode/).

## Omgeving

`env` is een object van strings. Het dialoogvenster bewerkt het als één `KEY=VALUE` per regel; het splitst
elke regel bij de eerste `=`, verwijdert witruimte aan beide kanten en negeert regels zonder `=`.

In het dialoogvenster:

```text
PORT=8091
NODE_ENV=development
```

In `apps.json`, als de sleutel `env` van het item:

```json title="apps.json (one entry)"
{ "id": "habits", "name": "Habits", "group": "Web apps", "type": "web", "command": "python app.py",
  "env": { "PORT": "8091", "NODE_ENV": "development" } }
```

- Het gestarte commando erft de omgeving van Moonpool plus `env`. Items in `env` winnen.
- `env` wordt ook toegepast op `stopCommand`.
- Waarden worden gebruikt zoals ze zijn geschreven: Moonpool doet geen `{MP_HOME}`-uitbreiding en geen `%VAR%`-uitbreiding.
- Moonpool wijst zijn eigen WebView2 naar een privéprofielmap via
  `WEBVIEW2_USER_DATA_FOLDER`. Gestarte apps erven die niet. Als je de
  variabele zelf had ingesteld voordat je Moonpool startte, krijgen ze je waarde; anders is hij niet ingesteld.
  Een `env`-item kan hem nog steeds overschrijven.

## Werkmap

Het commando en `stopCommand` draaien in `cwd`. Als `cwd` ontbreekt, draait het commando in de
eigen werkmap van Moonpool, dus stel `cwd` in voor alles wat relatieve paden gebruikt.
