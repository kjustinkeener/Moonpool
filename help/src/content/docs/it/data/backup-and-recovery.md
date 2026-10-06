---
title: "Eseguire il backup di Moonpool, ripristinare apps.json e recuperare una configurazione"
description: "Scopri cosa includere nel backup, come ripristinare un apps.json errato, tornare alle app di esempio, passare a una copia portatile e cosa rimuove la disinstallazione."
---

Tutto ciò che Moonpool conserva si trova in due posti: la cartella di configurazione e la
cartella delle dashboard. I percorsi per ogni modalità sono in
[Dove si trova la configurazione](/it/apps/apps-json/#dove-si-trova-la-configurazione).

## La cartella di configurazione

```text
moonpool-config\
  apps.json            your apps                              back up
  apps.json.history\   the last 10 good apps.json files       back up (optional)
  settings.json        app settings                           back up
  icons\               icon overrides, <id>.png and so on     back up
  cli-output\<id>\     session logs                           disposable
  moonpool.log         debug log                              disposable
  state.json           live status snapshot                   disposable
  dumps\               files written by dump and read-config  disposable
  mcp_seen.json        which apps had an MCP helper           disposable
  window-state.json    hub window size and position           disposable
  AI-README.md         rewritten at every launch              disposable
  webview\             the window's browser profile (Windows) disposable
```

Legenda: `back up` = da salvare nel backup, `disposable` = eliminabile.

La cartella delle dashboard è `{MP_HOME}\dashboards`: `%USERPROFILE%\.moonpool\dashboards`
nella copia installata, `<your .moonpool folder>\dashboards` in quella portatile e `dashboards/`
dentro la cartella di configurazione su Linux. Salva nel backup tutto ciò che vi hai messo tu.
La sua sottocartella `examples` appartiene a Moonpool e viene riscritta a ogni aggiornamento.

Il tema è conservato nell'archivio del browser della finestra, non in un file che puoi copiare.
Non viaggia con un backup; sceglilo di nuovo dopo un ripristino.

## Backup

1. Chiudi Moonpool, in modo che nessun file sia scritto a metà.
2. Copia `apps.json`, `settings.json` e `icons\` dalla cartella di configurazione, e i tuoi file
   da `dashboards\`.

```powershell frame="terminal"
$cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
Copy-Item "$cfg\apps.json", "$cfg\settings.json" D:\backup\
Copy-Item "$cfg\icons" D:\backup\ -Recurse
```

Per ripristinare, chiudi Moonpool, copia di nuovo i file al loro posto e avvialo.

## Ripristinare apps.json

Ogni salvataggio riuscito, scrittura di un agente e ripristino, e ogni Ricarica che trova
contenuto modificato, copia l'`apps.json` convalidato in `apps.json.history\`, mantenendo i 10
più recenti. Ogni file prende il nome dall'ora in cui è stato creato, ad esempio
`1767225600000.json`. Non esiste alcun `apps.json.bak`.

- **A mano.** Copia un'istantanea sopra `apps.json`, poi scegli **Ricarica**.

  ```powershell frame="terminal"
  $cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
  Copy-Item "$cfg\apps.json.history\<snapshot>" "$cfg\apps.json"
  ```

- **Da uno script.** `moonpool.exe restore-config` elenca le istantanee;
  `moonpool.exe restore-config 1` ripristina la più recente. Vedi
  [Riga di comando](/it/automation/command-line/).
- **Da un agente.** `moonpool_restore_config`. Vedi [Strumenti MCP](/it/automation/mcp-tools/#configurazione).

Nulla viene ripristinato automaticamente.

## Un file danneggiato

- **apps.json.** Moonpool non sovrascrive mai un file danneggiato. Vedi
  [Se il file è errato](/it/apps/apps-json/#se-il-file-è-errato).
- **settings.json.** Correggilo, oppure eliminalo per reimpostare ogni impostazione, poi riavvia
  Moonpool. Vedi [settings.json](/it/data/settings-json/#lettura-e-riparazione).

## Tornare agli esempi

Moonpool scrive le sue app di esempio solo quando non esiste un `apps.json`. Per ricominciare,
chiudi Moonpool (o lascialo in esecuzione), rinomina o elimina `apps.json`, poi avvia Moonpool
oppure scegli **Ricarica**. Viene scritto un nuovo `apps.json` con gli esempi.

## Da installata a portatile

Una nuova copia portatile parte con le app di esempio. Per portare le tue, vedi
[Modalità portatile](/it/data/portable-mode/#scegliere-la-modalità-portatile-dal-programma-di-installazione). Copia allo
stesso modo `icons\` e `settings.json` se le vuoi.

## Disinstallazione

La disinstallazione del Moonpool installato elimina l'intera cartella `%USERPROFILE%\.moonpool`,
compresi la cartella di configurazione e le dashboard. Fai prima un backup. Vedi
[Disinstallazione](/it/getting-started/install/#disinstallazione). Una copia portatile si rimuove
eliminando la sua cartella `.moonpool\`.
