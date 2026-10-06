---
title: "Prova le dashboard di esempio incluse in Moonpool"
description: "Apri le dashboard di esempio offline incluse, scopri dove si trovano e come le app di esempio le richiamano, e aggiungile a una configurazione esistente."
---

Moonpool include una serie di dashboard autonome all'interno del programma. Funzionano
interamente offline, senza server e senza CDN.

| Dashboard | Cos'è |
| --- | --- |
| CSV explorer | Trascina un file CSV o TSV: ne analizza le colonne e mostra i dati. |
| JSON explorer | Trascina un JSON (array, oggetti annidati o mappe). |
| Excel explorer | Trascina un file `.xlsx` o `.xls`, analizzato offline. |
| Moonpool Docs | Un browser di documentazione Markdown offline. |

## Dove si trovano

All'avvio Moonpool scrive le dashboard in `{MP_HOME}\dashboards\examples`:

| Modalità | Cartella |
| --- | --- |
| Installata (Windows) | `%USERPROFILE%\.moonpool\dashboards\examples` |
| Portatile | `<la tua cartella .moonpool, quella che contiene moonpool.exe>\dashboards\examples` |
| Linux | `~/.config/Moonpool/dashboards/examples` (oppure `$XDG_CONFIG_HOME/Moonpool/dashboards/examples`) |

La cartella `examples` appartiene a Moonpool: viene sostituita a ogni aggiornamento di
Moonpool, quindi le modifiche fatte lì vanno perse. Per personalizzare una dashboard, copia la
sua cartella e la cartella condivisa `_lib` in `dashboards` e fai puntare la tua app alla
copia. Moonpool non modifica mai nient'altro in `dashboards`.

Le versioni precedenti alla 0.3.16 scrivevano gli esempi direttamente in `dashboards`. Quelle
copie restano dove sono e non ricevono più aggiornamenti; le app che le usano continuano a
funzionare. Per ottenere le versioni aggiornate, cambia il loro `url` nel percorso
`dashboards/examples/...` indicato sotto.

## Come le richiamano le app

Ognuna è un'app `static` il cui `url` è un URL `file:///` ancorato a `{MP_HOME}`:

```text
file:///{MP_HOME}/dashboards/examples/csv/index.html
```

`{MP_HOME}` corrisponde alla cartella di installazione oppure, in modalità portatile, alla
cartella del pacchetto, quindi la voce funziona anche dopo aver spostato il pacchetto. Gli URL
`file://` sono consentiti. Vedi [Percorsi e ambiente](/it/apps/paths-and-environment/).

## Le app di esempio compaiono solo al primo avvio

Le voci di esempio vengono scritte in `apps.json` solo se non esiste ancora un file di
configurazione. Se hai già un `apps.json`, aggiungi tu le voci delle dashboard (**Modifica apps.json**
nel menu «...», poi **Ricarica**). Aggiungi queste quattro all'interno dell'array di primo
livello, separandole dalle altre voci con delle virgole:

```jsonc title="apps.json (excerpt)"
{
  "id": "csv-explorer",
  "name": "Sample CSV Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html",
  "openBrowser": true
},
{
  "id": "json-explorer",
  "name": "Sample JSON Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/json/index.html",
  "openBrowser": true
},
{
  "id": "xlsx-explorer",
  "name": "Sample Excel Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/xlsx/index.html",
  "openBrowser": true
},
{
  "id": "docs-browser",
  "name": "Moonpool Docs",
  "group": "Docs",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/docs/index.html",
  "openBrowser": true
}
```

Il significato dei campi è in [Campi delle app](/it/apps/fields/).

## Vedi anche

- [Esempi](/it/apps/examples/): voci più complete da copiare.
- [Tipi di app](/it/apps/types/#static)
