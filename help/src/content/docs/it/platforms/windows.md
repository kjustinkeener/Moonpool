---
title: "Usa Moonpool su Windows"
description: "Windows è la piattaforma principale di Moonpool: come installarlo e una tabella delle differenze tra Windows e Linux, così sai cosa aspettarti."
---

Windows è la piattaforma principale di Moonpool. Installalo come descritto in
[Installazione](/it/getting-started/install/).

## Prima di eseguirlo

- **SmartScreen.** `moonpool.exe` non è firmato digitalmente, quindi la prima volta Windows può
  mostrare «Windows protected your PC» (Windows ha protetto il PC). Scegli **More info**
  (Ulteriori informazioni), poi **Run anyway** (Esegui comunque).
- **Antivirus.** Un exe nuovo e non firmato che si copia da solo e si sostituisce a ogni
  aggiornamento può far scattare un antivirus. Se il tuo blocca o mette in quarantena
  `moonpool.exe`, consentilo per la cartella `.moonpool`.
- **WebView2.** Le finestre di Moonpool usano Microsoft Edge WebView2, incluso in Windows 11 e
  nelle versioni attuali di Windows 10. Se la finestra resta vuota o non si apre mai, installa
  da Microsoft il runtime WebView2 Evergreen.

Altro: [Windows protected your PC](/it/support/windows-protected-your-pc/),
[WebView2 runtime missing](/it/support/webview2-runtime-missing/) e
[Avviare uno script o un server di sviluppo automaticamente all'accesso a Windows](/it/guides/start-app-at-windows-login/).

## Area di notifica

Su Windows 11 una nuova icona nell'area di notifica finisce spesso nell'area delle icone
nascoste. Fai clic sulla freccia **^** a destra della barra delle applicazioni per trovarla e
trascinala sulla barra per tenerla visibile.

## Comandi

- I comandi vengono eseguiti tramite `cmd /c`. Evita le virgolette doppie annidate in `command`;
  `cmd /c` le altera. Per uno script che deve lasciare una shell aperta, usa
  `pwsh -NoLogo -NoProfile -NoExit -Command <script and args>` senza virgolette attorno alla
  parte dello script.
- `processName` corrisponde con o senza `.exe`, ignorando le maiuscole.
- Arresta termina l'intero albero di processi avviato da Moonpool, inclusi i processi che si sono
  staccati da esso.
- Le app Docker Desktop richiedono `killMode` `none` o `command`, mai `port`. Vedi
  [App Docker su Windows](/it/apps/stop-and-restart/#app-docker-su-windows).

## Differenze tra piattaforme

| | Windows | Linux |
| --- | --- | --- |
| Installazione | `moonpool.exe` che si installa da solo, oppure portatile | AppImage, `.deb` o RPM; nessuna scheda di installazione |
| Aggiornamento automatico | Sì, installato e portatile | Solo AppImage |
| Cartella di configurazione | `%USERPROFILE%\.moonpool\moonpool-config\` | `~/.config/Moonpool/` |
| Shell per i comandi | `cmd /c` | `$SHELL -c` |
| `processName` | Qualsiasi lunghezza, `.exe` facoltativo, senza distinzione tra maiuscole e minuscole | 15 caratteri o meno, maiuscole e minuscole esatte |
| Arresto tramite `processName` | Termina il processo e i suoi figli | Termina solo i processi con quel nome esatto |
| Canale di controllo | Named pipe | Socket Unix |
| Schermate delle finestre (test) | Sì | No |
| Icone da un file di programma | Sì | No |
| Area di notifica | Funziona subito | Richiede AppIndicator; GNOME standard richiede un'estensione |

I dettagli su Linux sono nella pagina [Linux](/it/platforms/linux/).
