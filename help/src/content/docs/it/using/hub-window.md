---
title: "Orientarsi nella finestra hub di Moonpool"
description: "Un giro della finestra hub di Moonpool: barra laterale, schede del terminale, barra di stato, il menu, il banner di errore di apps.json e la posizione salvata."
---

![L'hub con tre app in esecuzione: due righe di app web in esecuzione (1), la barra delle schede (2), l'output in tempo reale dell'app attiva (3) e la barra di stato (4)](../../../../assets/screenshots/hub-window.png)

1. Due delle app in esecuzione: un pallino di stato acceso e un pulsante di arresto al posto di quello di avvio.
2. La barra delle schede, una scheda per ogni app aperta, con la scheda attiva evidenziata.
3. L'output in tempo reale dell'app attiva.
4. La barra di stato di CPU e memoria.

## Struttura

| Area | Cosa contiene |
| --- | --- |
| Barra del titolo | Riduci a icona, ingrandisci e chiudi. |
| Barra laterale | La casella del filtro, il menu **...** e le tue app raggruppate per `group`. Vedi [Barra laterale e menu](/it/using/sidebar-and-menus/). |
| Pannello CLI | Una scheda del terminale per ogni app aperta. Vedi [Schede del terminale](/it/using/terminal-tabs/). |
| Barra di stato | CPU e memoria in tempo reale, lungo il bordo inferiore. |

Trascina il divisore tra la barra laterale e il pannello CLI per ridimensionare la barra laterale.

## Barra di stato

![La barra di stato: barre della CPU per core a sinistra, la barra della memoria a destra](../../../../assets/screenshots/status-bar.png)

La barra di stato mostra una sottile barra per ogni core della CPU (passa il cursore per vedere
«Utilizzo della CPU per core»), poi una barra della memoria con un'etichetta `used/total GB`.
Disattivala con **Mostra la barra di stato CPU/memoria** nelle Impostazioni (`showStatusbar`; vedi
[Finestra delle impostazioni](/it/using/settings/)). La modifica si applica subito.

## Il menu ...

Il pulsante **...** a sinistra della casella del filtro apre il menu.

![Il pulsante del menu ... (1) e la casella Filtra le app... (2) in cima alla barra laterale](../../../../assets/screenshots/sidebar-filter-and-menu.png)

1. Il pulsante del menu **...**.
2. La casella **Filtra le app...**.

| Voce | Cosa fa |
| --- | --- |
| Aggiungi app | Apre l'editor delle app. Vedi [Aggiungere app](/it/apps/add-an-app/). |
| Modifica apps.json | Apre `apps.json` nel tuo editor predefinito per la modifica a mano. |
| Ricarica | Rilegge `apps.json` dal disco (anche F5, vedi [Scorciatoie e zoom](/it/using/keyboard-shortcuts/)). |
| Impostazioni | Apre la finestra Impostazioni. |
| Aiuto | Apre questa guida. |
| Informazioni | Apre la finestra Informazioni, con la versione e il controllo degli aggiornamenti. |
| Installa Moonpool... | Solo Windows. Apre la finestra dell'installer, per installare l'app o creare una copia portatile. Vedi [Installazione](/it/getting-started/install/) e [Modalità portatile](/it/data/portable-mode/). |

### Avviso di conflitto di porte

Se due app in `apps.json` usano la stessa `port`, in fondo al menu compare una riga di avviso, ad esempio:

```text
port 3000: App A / App B
```

(nell'interfaccia in italiano: `porta 3000: App A e App B`). Passa il cursore per leggere la frase
completa. Risolvi il conflitto in `apps.json` o nell'editor delle app; la riga scompare quando
nessuna porta è più condivisa.

## Quando apps.json contiene un errore

Se Ricarica (o F5) rileva che `apps.json` non si interpreta più o non supera la validazione,
Moonpool mantiene l'elenco che già aveva. Un banner in cima alla barra laterale dice «apps.json
contiene un errore: viene mostrato l'ultimo elenco caricato», seguito dall'errore (passa il
cursore per leggere il testo completo). L'elenco sottostante è attenuato ma funziona ancora,
quindi puoi avviare e arrestare le app come al solito. **Modifica apps.json** nel banner apre il
file; correggilo e scegli **Ricarica**, e il banner scompare.

Finché il file non si carica di nuovo, Moonpool non salva le modifiche dall'editor delle app,
dalla rinomina, dall'eliminazione o dall'impostazione dell'icona, così un file errato non viene
mai sovrascritto.

Se il file è già danneggiato quando Moonpool si avvia, non c'è un elenco precedente da mantenere:
il banner dice che nessuna app è caricata e la barra laterale è vuota. Correggi il file e
ricarica, oppure torna a una copia valida recente (vedi
[Se il file è errato](/it/apps/apps-json/#se-il-file-è-errato)).

## Schermata vuota

Quando nessuna scheda è aperta, il pannello CLI mostra «Scegli un'app a sinistra per avviarla.»
Contiene anche due elementi che compaiono solo quando nessuna scheda è aperta:

- **Il banner di aggiornamento**, quando all'avvio è stata trovata una versione più recente. Vedi
  [Aggiornamenti](/it/data/updating/).
- **Copia il prompt**, un prompt già pronto che affida a un agente IA la configurazione delle tue
  app. Vedi [Agenti IA: guida rapida](/it/automation/quick-start/#copia-il-prompt).

L'icona nell'area di notifica, la chiusura, la riduzione a icona, Esci e Sempre in primo piano
sono in [Area di notifica, chiusura e riduzione a icona](/it/using/tray-and-closing/).

## Dimensioni, posizione e stato di ingrandimento

Moonpool ricorda tra un'esecuzione e l'altra le dimensioni, la posizione e lo stato di
ingrandimento della finestra hub. Il primo avvio si apre a 1200x780, nella posizione scelta da
Windows.

Se la posizione salvata non si trova più su nessun display collegato (ad esempio un monitor
scollegato), la posizione viene ignorata e le dimensioni salvate vengono usate nella posizione
predefinita. Il file è `window-state.json` nella cartella di configurazione (vedi
[Dove si trova la configurazione](/it/apps/apps-json/#dove-si-trova-la-configurazione)).

Vengono ricordate anche la larghezza della barra laterale e se il pannello CLI è compresso.
