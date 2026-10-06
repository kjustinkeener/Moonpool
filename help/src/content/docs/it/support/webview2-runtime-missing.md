---
title: "WebView2 runtime missing: risolvi una finestra di Moonpool vuota o assente su Windows"
description: "Se la finestra di Moonpool non si apre o resta vuota su Windows, potrebbe mancare il runtime Microsoft Edge WebView2. Come verificarlo e installarlo."
---

Se la finestra di Moonpool non si apre mai, o si apre e resta vuota, su Windows la causa
probabile è l'assenza del runtime Microsoft Edge WebView2. Moonpool è un'app Tauri e le sue
finestre sono pagine web disegnate da WebView2.

WebView2 è incluso in Windows 11 e nelle versioni attuali di Windows 10, quindi la maggior parte
dei PC lo ha già. Può mancare su un Windows 10 più vecchio o ridotto all'osso, oppure su un PC
da cui è stato rimosso. Il sorgente di Moonpool non mostra un messaggio dedicato per questo caso,
quindi qui non viene citato alcun testo di errore: il sintomo è che la finestra non compare o è
vuota.

## Verifica se è installato

In PowerShell, cerca nel registro la versione del runtime (il primo percorso è l'installazione
per tutto il sistema, il secondo quella per il solo utente):

```powershell frame="terminal"
Get-ItemProperty "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
Get-ItemProperty "HKCU:\Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
```

Un numero di versione come `120.0.2210.91` significa che è installato. Un errore per entrambi
significa che non lo è.

## Installalo

Scarica il runtime WebView2 **Evergreen** dalla pagina di Microsoft su WebView2 (cerca
«WebView2 Runtime download»), esegui l'installer, poi avvia di nuovo Moonpool. Il runtime
Evergreen si aggiorna da solo.

## Se è installato e la finestra è ancora vuota

- Esci da ogni Moonpool dall'area di notifica (oppure termina `moonpool.exe` in Gestione
  attività) e riavvialo.
- Attiva **Registra le informazioni di debug su file** nelle [Impostazioni](/it/using/settings/)
  se riesci a raggiungerle, e controlla `moonpool.log`. Vedi [Registri](/it/data/logs/).
- Se la finestra si apre ma è fuori dallo schermo, vedi
  [Problemi con la finestra](/it/support/troubleshooting/#problemi-con-la-finestra).

## Vedi anche

- [Windows](/it/platforms/windows/#prima-di-eseguirlo)
- [Installazione](/it/getting-started/install/)
- [Risoluzione dei problemi](/it/support/troubleshooting/)
