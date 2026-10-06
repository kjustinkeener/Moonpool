---
title: "Capire settings.json e riparare un file danneggiato"
description: "Scopri la struttura del file settings.json di Moonpool, quali chiavi scrive Moonpool per te e come leggerlo e ripararlo quando il file è danneggiato."
---

Le impostazioni dell'intera app si trovano in `settings.json` nella cartella di configurazione
(vedi [Dove si trova la configurazione](/it/apps/apps-json/#dove-si-trova-la-configurazione)). Modificale
nella [finestra Impostazioni](/it/using/settings/), che elenca ogni impostazione con la sua
chiave JSON e il valore predefinito. I registri e la loro conservazione sono nella pagina
[Registri](/it/data/logs/).

## Struttura

Un solo oggetto JSON. Le chiavi che ometti assumono i valori predefiniti:

```json title="settings.json"
{
  "closeToTray": true,
  "transparency": 20,
  "debugLogging": true,
  "cliLogging": true,
  "logRetentionMb": 25
}
```

| Chiave | Predefinito |
| --- | --- |
| `closeToTray` | `false` |
| `minimizeToTray` | `true` |
| `showInTray` | `true` |
| `showInTaskbar` | `true` |
| `alwaysOnTop` | `false` |
| `transparency` | `0` (da 0 a 90) |
| `showStatusbar` | `true` |
| `showMcpProcesses` | `true` |
| `checkOnStartup` | `true` |
| `locale` | `"auto"` |
| `debugLogging` | `false` |
| `cliLogging` | `false` |
| `logRetentionMb` | `10` (minimo 1) |

## Chiavi scritte per te

Moonpool memorizza in questo file anche lo zoom dell'interfaccia (`uiScale`, da 0,5 a 3,0) e la
lingua risolta (`localeResolved`). Non serve impostare né l'uno né l'altra. Il tema non è qui:
è conservato nell'archivio della webview (vedi [Temi, lingua e trasparenza](/it/using/themes-and-language/)).

## Lettura e riparazione

Moonpool legge il file all'avvio. Le modifiche fatte mentre è in esecuzione non vengono
recepite; chiudilo prima.

Se il file è malformato, Moonpool parte con i valori predefiniti e rifiuta di modificare le
impostazioni. L'errore termina con `Repair settings.json and restart Moonpool before changing settings`.
Correggi il file, oppure eliminalo per reimpostare ogni impostazione, poi avvia di nuovo Moonpool.
