---
title: "Leggere la barra laterale: pallini di stato, gruppi, filtro e menu della riga"
description: "Scopri cosa mostra ogni riga della barra laterale, come funzionano pallini di stato e gruppi, come filtrare le app, usare il menu contestuale e ridimensionare."
---

La barra laterale elenca ogni app di `apps.json`, raggruppata in base al campo `group` di ciascuna app. Vedi [Campi delle app](/it/apps/fields/).

## Righe

Ogni riga mostra un pallino di stato, l'icona dell'app (o un glifo del tipo se non c'è un'icona), il nome, la porta se impostata (`:3000`) e i controlli.

| Pallino | Significato |
| --- | --- |
| Pieno | in esecuzione |
| Pulsante | avvio in corso: Moonpool ha avviato l'app ma non risulta ancora attiva |
| Grigio | arrestata |

Passa il cursore sul pallino per vedere la parola.

![La barra laterale con due app web in esecuzione evidenziate: pallini accesi e pulsanti Arresta](../../../../assets/screenshots/sidebar-running-narrow.png)

1. Due app in esecuzione. I loro pallini sono accesi e Arresta (il quadrato) sostituisce Avvia.

| Controllo | Cosa fa |
| --- | --- |
| Matita | Modifica l'app. |
| Riavvia | Arresta e rilancia. Su un'app arrestata la avvia semplicemente. |
| Avvia (riproduzione) | Avvia l'app e apre la sua scheda del terminale. Mostrato quando l'app è arrestata. |
| Arresta (quadrato) | Arresta l'app. Mostrato mentre è in esecuzione o in avvio. |

![Una riga in esecuzione, ingrandita: pallino di stato, icona del tipo, nome e porta, poi i pulsanti di modifica, riavvio e arresto](../../../../assets/screenshots/sidebar-row-controls.png)

1. Pallino di stato (acceso mentre è in esecuzione).
2. Icona del tipo.
3. Modifica (matita).
4. Riavvia.
5. Arresta (mostrato al posto di Avvia mentre è in esecuzione).

Mentre un avvio o un arresto è in corso, i controlli vengono sostituiti da un indicatore di attività (`Operazione in corso...`).

Facendo clic sul **nome** di un'app si apre o si porta in primo piano la sua scheda del terminale e non si avvia mai nulla. La scheda di un'app arrestata mostra il registro di questa sessione. Usa Avvia o Riavvia per avviarla. Un'app `static` con solo un `url` e nessun `command` non ha un terminale: Avvia apre l'URL nel browser.

### Suggerimento

Passando il cursore sul nome si vede la `note` dell'app, se ne ha una, altrimenti il suo nome. Imposta `note` nell'editor o in `apps.json`.

### Sottoriga MCP

Quando un client IA ha usato gli strumenti MCP propri di un'app, sotto l'app compare una sottoriga attenuata `Server MCP`. Il suo pallino è acceso e il suggerimento dice «Client MCP collegato» finché il client è connesso. Un pulsante di arresto termina quel processo.

La riga trova il processo tramite `processName` più l'argomento `mcp`, oppure tramite il modello `mcpProcessName` dell'app quando ne è impostato uno. Vedi [campi](/it/apps/fields/#mcpprocessname).

Nascondi queste righe con **Mostra i processi MCP** nelle Impostazioni. Vedi
[Configurazione di MCP](/it/automation/mcp-setup/#app-che-hanno-un-proprio-server-mcp).

## Gruppi

![Barra laterale a riposo con le cinque intestazioni di gruppo evidenziate, ciascuna con il numero di app a destra](../../../../assets/screenshots/sidebar-groups-narrow.png)

- Fai clic sull'intestazione di un gruppo per comprimerlo o espanderlo. Il numero accanto è il numero di app mostrate. I gruppi compressi vengono ricordati.
- All'interno di un gruppo, l'app avviata più di recente è in cima. Le app mai avviate mantengono l'ordine di `apps.json`. Un'app appena avviata si illumina e sale in cima.

## Casella del filtro

Digita in **Filtra le app...** per restringere l'elenco. Confronta il nome dell'app e il nome del gruppo, ignorando le maiuscole. Se nulla corrisponde, l'elenco mostra:

```text
No apps match "<text>".
```

(nell'interfaccia in italiano: `Nessuna app corrisponde a «<text>».`) L'app mostra le virgolette
tipografiche attorno al testo, qui e nella richiesta di eliminazione più sotto.

## Menu del clic destro

Fai clic destro su una riga per:

| Voce | Cosa fa |
| --- | --- |
| Modifica | Apre l'editor delle app. |
| Rinomina | Trasforma il nome in un campo modificabile. **Invio** o un clic altrove salva, **Esc** annulla. Un nome vuoto o invariato viene ignorato. |
| Scegli icona... | Scegli un file immagine (png, jpg, jpeg, gif, svg, webp, ico) da usare come icona. |
| Elimina | Chiede `Delete "<name>"?` (in italiano: `Eliminare «<name>»?`) e rimuove la voce da `apps.json`. Se Moonpool sta eseguendo l'app, viene prima arrestata. |

**Esc** chiude il menu senza fare nulla.

## Ridimensionamento

Trascina il divisore tra la barra laterale e il pannello CLI. La larghezza è limitata da 180 a 620 px (predefinita 280) e viene ricordata. Il divisore è bloccato mentre il pannello CLI è compresso. Vedi [Schede del terminale](/it/using/terminal-tabs/).
