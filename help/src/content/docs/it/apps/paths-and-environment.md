---
title: "Usare percorsi, token MP_HOME e variabili d'ambiente nelle app"
description: "Usa i token {MP_HOME} e {MP_DATA} e i percorsi relativi ./ nelle voci delle app, scopri quali campi li espandono e imposta env e la cartella di lavoro."
---

## Token

| Token | Si espande in |
| --- | --- |
| `{MP_HOME}` | Portatile: la cartella che contiene `moonpool.exe` (la cartella `.moonpool\`). Installata su Windows: `%USERPROFILE%\.moonpool`. Linux: `$XDG_CONFIG_HOME/Moonpool`, altrimenti `~/.config/Moonpool`, la stessa cartella di `{MP_DATA}`. |
| `{MP_DATA}` | La cartella di configurazione, quella che contiene `apps.json`. |

Un token che non può essere risolto viene lasciato com'è.

## Quali campi vengono espansi

| Campo | Token | `./` o `.\` iniziale |
| --- | --- | --- |
| `cwd` | sì | sì, ancorato a `{MP_HOME}` |
| `command` | sì | no |
| `stopCommand` | sì | no (viene eseguito in `cwd`, che è ancorata) |
| `url` | sì | no |
| `icon` | sì | sì, ancorato a `{MP_HOME}` |
| valori di `env`, `processName`, `note` | no | no |

Un percorso relativo senza `./` (come `apps\tool`) viene lasciato inalterato e risolto rispetto
alla cartella di lavoro di Moonpool stesso, che raramente è ciò che vuoi. Preferisci `./` o un
token.

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

Entrambe le forme continuano a funzionare quando sposti la cartella portatile. Un percorso fisso
come `C:\tools\notes` non viaggia con essa. In modalità portatile la finestra Modifica app
contrassegna i valori assoluti di `cwd` e `url` con un'etichetta «non portatile». Vedi
[Modalità portatile](/it/data/portable-mode/).

## Ambiente

`env` è un oggetto di stringhe. La finestra lo modifica come una `KEY=VALUE` per riga; divide
ogni riga al primo `=`, elimina gli spazi ai due lati e ignora le righe che non ne hanno uno.

Nella finestra:

```text
PORT=8091
NODE_ENV=development
```

In `apps.json`, come chiave `env` della voce:

```json title="apps.json (one entry)"
{ "id": "habits", "name": "Habits", "group": "Web apps", "type": "web", "command": "python app.py",
  "env": { "PORT": "8091", "NODE_ENV": "development" } }
```

- Il comando avviato eredita l'ambiente di Moonpool più `env`. Le voci in `env` hanno la precedenza.
- `env` viene applicato anche a `stopCommand`.
- I valori vengono usati così come sono scritti: Moonpool non espande né `{MP_HOME}` né `%VAR%`.
- Moonpool indirizza il proprio WebView2 a una cartella di profilo privata tramite
  `WEBVIEW2_USER_DATA_FOLDER`. Le app avviate non la ereditano. Se avevi impostato tu la
  variabile prima di avviare Moonpool, ricevono il tuo valore; altrimenti non è impostata.
  Una voce di `env` può comunque sostituirla.

## Cartella di lavoro

Il comando e `stopCommand` vengono eseguiti in `cwd`. Quando `cwd` è omessa, il comando viene
eseguito nella cartella di lavoro di Moonpool stesso, quindi imposta `cwd` per qualsiasi cosa
usi percorsi relativi.
