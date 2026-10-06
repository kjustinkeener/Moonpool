---
title: "Scegliere un tipo di app: web, desktop, static o cli"
description: "Scopri come si avviano le app web, desktop, static e cli in Moonpool, come viene rilevato «in esecuzione» per ciascuna e cosa fa Arresta di default."
---

`type` decide quali campi contano e cosa fa Arresta per impostazione predefinita.

| | `web` | `desktop` | `static` | `cli` |
| --- | --- | --- | --- | --- |
| Richiede | `command` | `command` | `url` | `command` |
| Di solito anche | `port`, `url` | `processName` | `command` e `port`, se si serve da solo | `cwd` |
| Avvio | Esegue `command` in una scheda del terminale | Esegue `command` in una scheda del terminale | Senza `command`: apre `url` nel browser. Con uno: lo esegue in una scheda del terminale | Esegue `command` in una scheda del terminale |
| `killMode` predefinito | `port` | `processName` | `none` | `none` |

![La finestra Modifica app per un'app web: tipo impostato su web con una descrizione di una riga e un campo port compilato](../../../../assets/screenshots/edit-app-type-and-port.png)

1. La selezione `type`. La sua riga di suggerimento descrive cosa fa quel tipo.
2. Il campo `port`. Per un'app `web`, «in esecuzione» dipende dal fatto che questa porta risponda.

## Come viene stabilito «in esecuzione»

Moonpool controlla ogni paio di secondi. Un'app è in esecuzione se vale una di queste
condizioni, qualunque sia il suo tipo:

- `processName` è impostato ed esiste un processo con quel nome. I processi helper
  `<exe> mcp` di Moonpool stesso non vengono contati.
- `port` è impostata e risponde su localhost.
- Moonpool l'ha avviata, non ha né `port` né `processName` e il processo del terminale è
  ancora attivo.

Quindi un'app `cli` è in esecuzione finché il suo comando è in esecuzione, e un'app `web` senza
`port` si comporta allo stesso modo. Una voce `static` con il solo `url` non ha nulla da
monitorare e non risulta mai in esecuzione.

## web

Un server locale. Imposta `port` perché «in esecuzione» rifletta se il server risponde, e `url`
più `openBrowser` per aprirlo quando diventa attivo.

## desktop

Un'app nativa. Imposta `processName` sul nome dell'eseguibile in modo che «in esecuzione»
sopravviva al distacco della finestra dal comando che l'ha avviata. L'Arresta predefinito
termina ogni processo con quel nome.

## static

Una pagina. Con il solo `url`, Avvia e Riavvia la aprono nel browser e Arresta non fa nulla.
Gli URL `http://`, `https://`, `mailto:` e `file://` vengono aperti, quindi funziona anche una
pagina locale:

```json title="apps.json"
{ "id": "csv", "name": "CSV dashboard", "group": "Docs", "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html" }
```

Le pagine che richiedono un server (PHP, o qualsiasi cosa che carichi file locali) necessitano
di un `command` che ne avvii uno e di una `port` per monitorarlo. Vedi gli
[esempi](/it/apps/examples/).

## cli

Uno strumento. `command` viene eseguito in una scheda del terminale in `cwd`, e l'app smette di
essere in esecuzione quando il comando termina. Per una shell che resta aperta, fai in modo che
il comando sia una shell, ad esempio questo `command`:

```text title="command"
pwsh -NoLogo -NoProfile -NoExit -Command python run.py --flag
```

Evita le virgolette doppie annidate in `command`: vengono alterate dal wrapper `cmd /c`.

![La scheda del terminale di un'app cli che mostra l'output di un comando PowerShell e un prompt aperto sotto](../../../../assets/screenshots/terminal-cli-output.png)

## Cosa fa il clic

Un clic sul nome di un'app apre soltanto la sua scheda del terminale. Usa i controlli Avvia,
Arresta e Riavvia per eseguirla. Vedi [Stati delle app](/it/support/glossary/#stati-delle-app).
