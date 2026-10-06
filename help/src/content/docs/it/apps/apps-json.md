---
title: "Modificare apps.json: dove si trova, come ricaricarlo e ripristinarlo"
description: "Trova il file apps.json che Moonpool legge per ogni app gestita, modificalo nell'editor o a mano, ricaricalo e ripristinalo dopo una modifica sbagliata."
---

Ogni app gestita da Moonpool è una voce in `apps.json`. Puoi modificarlo dall'editor delle app
(la finestra Aggiungi app e Modifica app) oppure a mano. Entrambi scrivono lo stesso file. Alcuni
risultati degli strumenti e alcuni messaggi chiamano questo file il manifest.

## Dove si trova la configurazione

| Modalità | Cartella di configurazione |
| --- | --- |
| Installata (Windows) | `%USERPROFILE%\.moonpool\moonpool-config\` |
| Portatile | `moonpool-config\` accanto a `moonpool.exe` (dentro la cartella `.moonpool\`) |
| Linux | `$XDG_CONFIG_HOME/Moonpool/`, altrimenti `~/.config/Moonpool/` |

`apps.json` si trova in quella cartella, accanto a questi elementi:

| Elemento | Scopo |
| --- | --- |
| `apps.json.history\` | Archivio circolare per il ripristino degli ultimi 10 file `apps.json` validi. |
| `settings.json` | Impostazioni dell'app. Vedi [settings.json](/it/data/settings-json/). |
| `cli-output\<id>\` | Registri di sessione per app. Vedi [Registri](/it/data/logs/). |
| `moonpool.log` | Registro di debug, mentre **Registra le informazioni di debug su file** è attivo. |
| `icons\` | Icone sostitutive facoltative `<id>.png` (anche `.ico`, `.svg`, `.jpg`, `.jpeg`, `.webp`). |
| `state.json` | Istantanea dello stato in tempo reale, aggiornata ogni paio di secondi. |
| `dumps\` | File scritti dai verbi `dump`, `read-config` e `restore-config`. |
| `mcp_seen.json` | Quali app hanno avuto un helper MCP. |
| `window-state.json` | Dimensioni e posizione della finestra hub. |
| `AI-README.md` | La guida per gli agenti IA, riscritta a ogni avvio. |

Quali di questi file salvare in un backup è indicato in [Backup e ripristino](/it/data/backup-and-recovery/#la-cartella-di-configurazione).

Al primo avvio Moonpool inserisce in `apps.json` alcune voci di esempio. Un file già esistente
non viene mai sovrascritto.

## Modifica

- **Finestra.** Usa **Aggiungi app** nel menu **...** in alto nella barra laterale. Per modificare
  un'app, usa la matita sulla sua riga oppure fai clic destro su di essa e scegli **Modifica**.
  La finestra convalida e salva subito.
- **A mano.** **Modifica apps.json** nello stesso menu apre il file nell'editor predefinito.
  Salvalo, poi scegli **Ricarica** nel menu (oppure premi F5 o Ctrl+R).

Le modifiche manuali non vengono recepite finché non ricarichi. Ricarica si limita a leggere il
file; non lo riscrive.

Il salvataggio dalla finestra riscrive l'intero file in una forma normalizzata e indentata. Le
chiavi sconosciute a Moonpool vengono eliminate e JSON non ha commenti, quindi conserva le note
nel campo `note`.

## Struttura

Il file è un array JSON di oggetti. Quattro chiavi sono obbligatorie in ogni voce: `id`, `name`,
`group`, `type`. Tutto il resto è facoltativo. Vedi [Campi delle app](/it/apps/fields/).

```json title="apps.json"
[
  { "id": "site", "name": "Site", "group": "Web apps", "type": "web",
    "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
    "url": "http://localhost:5173", "openBrowser": true }
]
```

I gruppi compaiono nella barra laterale nell'ordine in cui appaiono per la prima volta nel file.

## Cosa fa Ricarica

Ricarica sostituisce l'elenco in memoria di Moonpool con il contenuto del file. Avvia, Arresta e
Riavvia leggono la voce nel momento in cui fai clic, quindi un `command`, una `cwd`, un `env` o
un'impostazione di terminazione modificati si applicano alla prossima volta che avvii o riavvii
quell'app. Ricarica non riavvia mai nulla: un'app già in esecuzione continua a funzionare con le
impostazioni con cui è stata avviata.

## Convalida

Moonpool convalida l'intero file quando lo carica, a ogni salvataggio e a ogni scrittura di un
agente. Una sola voce errata fa rifiutare l'intero file.

| Regola | L'errore contiene |
| --- | --- |
| JSON non valido, chiave obbligatoria mancante o valore di tipo errato | il messaggio del parser JSON |
| `id` è vuoto, inizia con `-` o contiene caratteri diversi da lettere, cifre, `.`, `_`, `-` | `invalid id` |
| Due voci condividono lo stesso `id` | `duplicate app id` |
| `name` è vuoto | `has an empty name` |
| `group` è vuoto | `has an empty group` |
| `type` non è `desktop`, `web`, `static` o `cli` | `unknown type` |
| `port` è `0` (una `port` superiore a 65535 non viene letta) | `invalid port 0` |
| Voce `static` senza `url` | `requires a url` |
| Qualsiasi altro tipo senza `command` | `requires a command` |

Gli errori indicano la voce per posizione, ad esempio:

```text
apps.json entry 2 (site) requires a command
```

### L'id

L'`id` è la chiave permanente della voce. Dà il nome alla cartella dei registri e al file
dell'icona, ed è ciò che passi a `moonpool.exe launch <id>` e agli agenti. La finestra lo ricava
dal nome quando aggiungi un'app. Converte il nome in minuscolo, trasforma ogni sequenza di
caratteri diversi da `a`-`z` e `0`-`9` in un solo `-` e rimuove i `-` a entrambe le estremità.
Un risultato vuoto diventa `app`. Se l'id è già in uso, aggiunge `-2`, `-3` e così via. Non
cambia mai l'id in seguito, quindi rinominare un'app ne mantiene l'id. Il nome `Habit Tracker`
ottiene l'id `habit-tracker`.

## Se il file è errato

- **Con Ricarica**, un file che non supera la convalida resta intatto e Moonpool mantiene
  l'ultimo elenco caricato. Un avviso sopra la barra laterale mostra l'errore, con un pulsante
  per aprire il file; l'elenco resta utilizzabile ma attenuato. Vedi
  [Quando apps.json contiene un errore](/it/using/hub-window/#quando-appsjson-contiene-un-errore).
- **All'avvio**, un file danneggiato significa che non c'è alcun elenco da conservare, quindi
  Moonpool parte senza app e l'avviso lo segnala. Correggi il file e scegli **Ricarica**, oppure
  ripristina un'istantanea (qui sotto, o con lo strumento `moonpool_restore_config`).
- In entrambi i casi, i salvataggi dalla finestra (e rinomina, eliminazione, scelta
  dell'icona) vengono rifiutati finché il file non si carica di nuovo, così il file danneggiato
  non viene mai sovrascritto. Correggi il file e scegli **Ricarica**.
- **Da una finestra, da un agente o da un ripristino**, una modifica non valida viene rifiutata
  e il file su disco resta com'era.

Moonpool conserva le ultime 10 versioni valide di `apps.json` in `apps.json.history\`. Come
tornare indietro è spiegato in [Backup e ripristino](/it/data/backup-and-recovery/#ripristinare-appsjson).
Sintomi e soluzioni sono in [Risoluzione dei problemi](/it/support/troubleshooting/#appsjson-contiene-un-errore).

## Agenti

Un agente IA dovrebbe modificare `apps.json` tramite gli strumenti MCP di Moonpool anziché
direttamente il file, così una scrittura obsoleta o non valida viene rifiutata e un agente in
sandbox non modifica mai una copia privata. Vedi
[Strumenti MCP](/it/automation/mcp-tools/#configurazione).
