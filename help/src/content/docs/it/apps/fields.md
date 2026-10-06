---
title: "Tutti i campi di apps.json: tipo, valore predefinito e funzione"
description: "Consulta ogni chiave di una voce apps.json con il suo tipo, il valore predefinito e i tipi di app che la usano, con gli stessi nomi della finestra Modifica app."
---

La finestra Modifica app mostra gli stessi campi con gli stessi nomi. I campi che non si
applicano al tipo selezionato sono attenuati nella finestra ma vengono comunque salvati, con
un'eccezione: `stopCommand` viene salvato solo quando `killMode` è `command`.

![La finestra Modifica app dal nome fino a stopCommand, con la selezione killMode evidenziata; i campi inutilizzati come processName e stopCommand sono attenuati](../../../../assets/screenshots/edit-app-dialog.png)

1. La selezione `killMode`. I campi che non usa restano attenuati.

| Campo | Tipo | Obbligatorio | Usato da | Cosa fa |
| --- | --- | --- | --- | --- |
| `id` | stringa | sì | tutti | Chiave univoca. Lettere, cifre, `.`, `_`, `-`, senza iniziare con `-`. Vedi [Panoramica](/it/apps/apps-json/#lid). |
| `name` | stringa | sì | tutti | Etichetta nella barra laterale. Non vuota. |
| `group` | stringa | sì | tutti | Intestazione della barra laterale sotto cui l'app è elencata. Non vuota in una modifica manuale; la finestra salva un gruppo vuoto come `Apps`. Qualsiasi testo; un nome nuovo crea un nuovo gruppo. |
| `type` | stringa | sì | tutti | `web`, `desktop`, `static` o `cli`. Vedi [Tipi di app](/it/apps/types/). |
| `command` | stringa | tutti tranne `static` | tutti | Eseguito in un terminale per avviare l'app, tramite `cmd /c` su Windows e `$SHELL -c` altrove (`/bin/sh` se `SHELL` non è impostata). Facoltativo per `static`. |
| `cwd` | stringa | no | tutti con un `command` | Cartella in cui viene eseguito il comando. Per impostazione predefinita è la cartella di lavoro di Moonpool stesso. Supporta token e `./`. Vedi [Percorsi e ambiente](/it/apps/paths-and-environment/). |
| `port` | intero, da 1 a 65535 | no | qualsiasi | In esecuzione finché qualcosa risponde su questa porta in localhost (IPv4 o IPv6). Letta da `killMode` `port`. |
| `processName` | stringa | no | qualsiasi, soprattutto `desktop` | In esecuzione finché esiste un processo con questo nome. Senza distinzione tra maiuscole e minuscole, con o senza `.exe`, quindi `my-app` corrisponde a `my-app.exe`. Su Linux, al massimo 15 caratteri. Letto da `killMode` `processName`. |
| `mcpProcessName` | stringa | no | qualsiasi con un `processName` | Modello con caratteri jolly per il nome del processo del server MCP di questa app. `*` corrisponde a qualsiasi sequenza di caratteri, `?` a un solo carattere. Senza distinzione tra maiuscole e minuscole, confrontato con l'intero nome, e `.exe` è facoltativo. Un processo corrispondente conta come server MCP dell'app (la voce secondaria MCP nella barra laterale) e non richiede `mcp` come primo argomento. Vedi [mcpProcessName](#mcpprocessname). |
| `url` | stringa | solo `static` | `web`, `static` | Pagina da aprire. Vengono aperti solo gli URL `http://`, `https://`, `mailto:` e `file://`. |
| `openBrowser` | booleano, predefinito `false` | no | qualsiasi tipo con un `url` (la finestra lo attenua per `desktop` e `cli`) | Apre `url` automaticamente non appena Moonpool rileva che l'app è attiva (vedi sotto). |
| `killMode` | stringa | no | tutti | Pulizia aggiuntiva con Arresta e Riavvia: `processName`, `port`, `command` o `none`. Vedi [Arresto e riavvio](/it/apps/stop-and-restart/). |
| `stopCommand` | stringa | no | `killMode` `command` | Comando eseguito con Arresta. Ignorato in ogni altra modalità. |
| `env` | oggetto di stringhe | no | tutti | Variabili d'ambiente aggiuntive. La finestra le modifica come una `KEY=VALUE` per riga. |
| `icon` | stringa | no | tutti | Immagine della barra laterale: un percorso di file, un URL `http(s)` o un URI `data:`. Impostala con **Scegli icona...** nel menu contestuale dell'app o a mano. |
| `note` | stringa | no | tutti | Suggerimento mostrato quando passi il cursore sull'app nella barra laterale. |

Una voce che usa `env` e `killMode`:

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000", "NODE_ENV": "development" },
  "killMode": "port"
}
```

Vedi [Trovare e terminare il processo che usa una porta](/it/guides/find-and-kill-process-using-port-windows/)
per capire come lavorano insieme `port` e `killMode`.

## mcpProcessName

Per impostazione predefinita Moonpool considera un processo il server MCP dell'app quando il suo
nome corrisponde a `processName` e il suo primo argomento è `mcp`, come `notes-app.exe mcp`.
Imposta `mcpProcessName` quando il server viene eseguito con un nome diverso: un'app che
monitora un exe mentre il suo server MCP è un altro (`mog.exe mcp`), oppure una copia
rinominata del server.

Il valore è un modello con caratteri jolly. `*` corrisponde a qualsiasi sequenza di caratteri
(anche vuota) e `?` a esattamente uno. Viene confrontato senza distinzione tra maiuscole e
minuscole con l'intero nome del processo, e un modello senza `.exe` corrisponde anche al nome
con `.exe`. Un valore vuoto equivale a non impostato.

```json
{
  "id": "destiny",
  "name": "Destiny",
  "group": "Desktop apps",
  "type": "desktop",
  "processName": "destiny",
  "mcpProcessName": "destiny-mcp-*"
}
```

Questo corrisponde a una copia rinominata come `destiny-mcp-2706210170.exe`. Un processo che
corrisponde a `mcpProcessName` è il server indipendentemente dal fatto che sia stato avviato con
`mcp`, e non conta mai come l'app stessa in esecuzione. Se il modello corrisponde anche a
`processName` stesso (ad esempio `destiny*`), Moonpool richiede comunque l'argomento `mcp`, così
l'app vera non viene mai scambiata per il suo server MCP. Vedi
[Configurazione MCP](/it/automation/mcp-setup/#app-che-hanno-un-proprio-server-mcp).

## openBrowser

Moonpool apre `url` una sola volta, quando un'app avviata da Moonpool risulta per la prima volta
in esecuzione. Per rilevarlo servono una `port` o un `processName`. Senza nessuno dei due,
«in esecuzione» significa soltanto che il processo del terminale è attivo e il browser non viene
aperto automaticamente. Disattiva `openBrowser` se il tuo comando apre già un browser da solo.
Una voce `static` senza comando apre `url` ogni volta che premi Avvia, a prescindere da
`openBrowser`.

Due app configurate con la stessa `port` vengono segnalate nella barra laterale.

## Icone

L'icona di un'app è la prima tra queste che esiste:

1. Il campo `icon`.
2. `icons\<id>.<ext>` nella cartella di configurazione, ad esempio `icons\site.png`.
3. Un file di icona nella cartella dell'app stessa (la sua `cwd`, o la cartella di un `url` `file:///`).
4. Per `desktop`, l'icona del suo `.exe` compilato o in esecuzione.
5. Per `web` e `static`, il `/favicon.ico` del sito, non appena il server è attivo.
6. Un glifo per il tipo.

La maggior parte delle app non richiede alcuna impostazione dell'icona.
