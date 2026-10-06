---
title: "Eseguire Moonpool da una chiavetta USB o da una cartella sincronizzata"
description: "Tieni Moonpool e tutti i suoi dati in un'unica cartella spostabile per portarla su una chiavetta USB o sincronizzarla, ed esegui più copie affiancate."
---

La modalità portatile mantiene Moonpool e tutto ciò che scrive dentro un'unica cartella
`.moonpool\`, così puoi portarla su una chiavetta USB o metterla in una cartella sincronizzata
ed eseguirla su qualsiasi PC.

## Come funziona

Quando installi in modalità portatile, Moonpool crea una cartella `.moonpool\` all'interno della
posizione che scegli. Quella cartella contiene il programma, la tua configurazione e i contenuti
della guida. Nulla viene scritto in AppData di Windows, quindi spostare o copiare la cartella
sposta con sé l'intera configurazione.

```text
<chosen location>\.moonpool\
```

## Differenze rispetto alla versione installata

| | Installata | Portatile |
| --- | --- | --- |
| Programma | `%USERPROFILE%\.moonpool\moonpool.exe` | `<chosen location>\.moonpool\moonpool.exe` |
| Cartella di configurazione | `%USERPROFILE%\.moonpool\moonpool-config\` | `<chosen location>\.moonpool\moonpool-config\` |
| Profilo del browser della finestra, dimensioni e posizione della finestra | Nella cartella di configurazione | Nella cartella di configurazione, quindi viaggiano anch'essi |
| Menu Start, collegamento sul desktop, voce di Add/Remove | Sì | Nessuno |
| Aggiornamenti | Sostituisce il proprio exe | Lo stesso, dentro la cartella `.moonpool\`. Vedi [Aggiornamento](/it/data/updating/#copie-portatili). |
| Rimozione | Add/Remove Programs (programmi installati) o `--uninstall` | Elimina la cartella |

Nessuna delle due modalità scrive in AppData di Windows.

### Cartelle sincronizzate

Puoi tenere una copia portatile in una cartella sincronizzata (OneDrive, Dropbox e simili), ma
eseguila su un solo PC alla volta. Moonpool scrive `state.json` ogni paio di secondi e registra
mentre le app girano, quindi due PC che eseguono la stessa cartella si contendono gli stessi
file, e un conflitto di sincronizzazione può lasciare un `apps.json` danneggiato. Chiudilo su un
PC prima di avviarlo su un altro.

## Più copie contemporaneamente

Viene eseguito un Moonpool per cartella. Il Moonpool installato e un numero qualsiasi di copie
portatili, ciascuna nella propria cartella, possono funzionare contemporaneamente, e ognuna è
completamente separata: ha le proprie app, la propria icona nell'area di notifica, la propria
finestra, le proprie impostazioni, i propri registri e il proprio
[canale di controllo](/it/automation/control-verbs/).

- Il suggerimento dell'icona nell'area di notifica e il nome nella barra delle applicazioni
  indicano quale copia è quale: `Moonpool` per quella installata, `Moonpool (<folder>)` per una
  portatile, dove `<folder>` è la cartella che hai scelto (quella che contiene `.moonpool\`).
- Avviare una seconda volta la stessa copia ne riporta in primo piano la finestra anziché aprirne
  un'altra. Avviare una copia diversa apre quella copia.
- Per dare a un agente IA più di una copia, registra ciascuna con il proprio nome; vedi
  [Configurazione MCP](/it/automation/mcp-setup/#più-di-un-moonpool).
- Spostare o rinominare una cartella portatile le dà una nuova identità (un nuovo nome del
  canale di controllo). Chiudila prima di spostarla.
- Le copie non sanno nulla delle app delle altre. Due copie che avviano lo stesso server sulla
  stessa porta entreranno comunque in conflitto, e un Arresta che opera per nome del processo o
  per porta può terminare qualcosa che ha avviato un'altra copia; vedi
  [Arresto e riavvio](/it/apps/stop-and-restart/#più-copie-di-moonpool-o-processi-tuoi).

## Far viaggiare anche le tue app

Usa il token `{MP_HOME}` nel percorso di un'app in modo che punti dentro la cartella portatile
anziché a una posizione fissa su un solo computer. In una copia portatile `{MP_HOME}` è la
cartella che contiene `moonpool.exe`, cioè la cartella `.moonpool\` stessa, non la cartella che
hai scelto:

```json title="apps.json"
{ "cwd": "{MP_HOME}/my-app" }
```

Qui `{MP_HOME}/my-app` è `<chosen location>\.moonpool\my-app`. Un percorso che inizia con
`./` è ancorato allo stesso modo. I token e i percorsi `./` funzionano anche in un Moonpool
installato. Vedi [Percorsi e ambiente](/it/apps/paths-and-environment/) per come vengono risolti
i percorsi.

## Scegliere la modalità portatile dal programma di installazione

La modalità portatile si configura dalla scheda di installazione, che offre **Installa versione
portatile** accanto a **Installa Moonpool**.

![La scheda di installazione: il collegamento Installa versione portatile si trova sotto il pulsante principale Installa Moonpool](../../../../assets/screenshots/installer-window.png)

Scegli una cartella e Moonpool vi crea la cartella `.moonpool\`, vi copia se stesso e avvia la
nuova copia con una configurazione nuova.

La scheda si trova anche nel menu "..." come **Installa Moonpool…**, sia in modalità installata
sia portatile. Usare **Installa versione portatile** da lì fa chiudere il Moonpool in
esecuzione e avviare al suo posto la nuova copia portatile. Il Moonpool da cui hai iniziato resta
dov'era, quindi puoi avviarlo di nuovo in seguito.

Una copia portatile parte da zero e non copia le tue app esistenti. Per portarle con te, chiudi
la copia portatile e copia `apps.json` a mano:

| | Percorso |
| --- | --- |
| Da (installata) | `%USERPROFILE%\.moonpool\moonpool-config\apps.json` |
| A (portatile) | `<chosen location>\.moonpool\moonpool-config\apps.json` |

Le voci con percorsi assoluti continuano a funzionare sullo stesso PC, ma non viaggiano. La
finestra Modifica app le contrassegna come «non portatile».

## Come Moonpool sa di essere portatile

Una copia è portatile finché un file chiamato `moonpool.portable` si trova accanto al suo
`moonpool.exe`. Nient'altro la contrassegna, e nulla viene registrato in Windows.

Per rimuovere una copia portatile, chiudila ed elimina la sua cartella `.moonpool\`.
`--uninstall` rimuove solo il Moonpool installato, mai una copia portatile.
