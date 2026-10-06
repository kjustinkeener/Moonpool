---
title: "Aggiungere un'app o un server di sviluppo a Moonpool"
description: "Registra un'app locale o un server di sviluppo con comando di avvio, cartella di lavoro e ambiente, così Moonpool può avviarlo, arrestarlo e monitorarlo."
---

Ogni app in Moonpool è una voce con un comando di avvio, una cartella di lavoro e un
ambiente facoltativo. Moonpool esegue il comando in un proprio terminale gestito.

## Aggiungere un'app

1. Apri il menu **...** in alto nella barra laterale e scegli **Aggiungi app**.
2. Inserisci un **name** e scegli un **group**.
3. Scegli il **type**: `web` (server su una porta), `desktop` (app nativa), `static` (una pagina) oppure `cli` (un comando).
4. Imposta il **command** e la **cwd** in cui viene eseguito.
5. Compila quanto richiede il tipo: **port** e **url** per web, **processName** per desktop,
   **url** per static. Un'app `static` con il solo `url` non richiede né **command** né **cwd**.
6. Salva. L'app compare nella barra laterale. Usa il suo controllo **Avvia** per avviarla.

![La selezione del tipo (1) e il campo port (2) nell'editor delle app, con cwd e command in mezzo](../../../../assets/screenshots/edit-app-type-and-port.png)

1. La selezione **type**; il suggerimento accanto spiega come viene eseguito quel tipo.
2. Il campo **port**, usato dalle app `web`.

Il risultato è una voce in `apps.json`, ad esempio:

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

Un clic sul nome di un'app apre soltanto la sua scheda del terminale; vedi [Stati delle app](/it/support/glossary/#stati-delle-app).

## L'editor delle app

- **Gruppo.** Scegli un gruppo dall'elenco, oppure scegli **+ Nuovo gruppo...** e digita un nome.
  **torna all'elenco** riporta all'elenco. Un gruppo vuoto viene salvato come `Apps`.
- I **campi attenuati** non sono usati dal tipo selezionato. Vengono comunque salvati.
- **Salvare senza nome** mostra `il nome è obbligatorio.`
- **Esc** o la chiusura dell'editor con modifiche non salvate chiede «Vuoi annullare le modifiche?».
- Per modificare un'app in seguito, usa la matita sulla sua riga, oppure fai clic destro su di essa e scegli **Modifica**.

## Modificare a mano

Scegli **Modifica apps.json** nello stesso menu, salva il file, poi scegli **Ricarica**. Il
formato, le regole di convalida e le opzioni di ripristino sono nella
[Panoramica della configurazione](/it/apps/apps-json/).

## Passi successivi

- [Campi delle app](/it/apps/fields/): ogni chiave e ciò che fa.
- [Tipi di app](/it/apps/types/): come si avvia ogni tipo e come viene mostrato «in esecuzione».
- [Arresto e riavvio](/it/apps/stop-and-restart/): cosa impostare quando Arresta lascia qualcosa in esecuzione, e perché le app Docker richiedono attenzione.
- [Percorsi e ambiente](/it/apps/paths-and-environment/): `{MP_HOME}`, percorsi `./` e `env`.
- [Esempi](/it/apps/examples/): voci complete da copiare.
- [Guide pratiche](/it/guides/run-npm-dev-server-in-background-windows/): server di sviluppo in background, script Python, porte.
- [Modalità portatile](/it/data/portable-mode/)
- [Aggiornamento](/it/data/updating/)
