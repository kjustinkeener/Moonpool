---
title: "Installa Moonpool su Windows o Linux"
description: "Installa Moonpool in pochi clic, scegli la modalità installata o portatile, usa più tardi la voce Installa Moonpool del menu e disinstalla in modo pulito."
---

Questa pagina riguarda Windows. Su Windows, Moonpool è il proprio installer: il download è un
unico `moonpool.exe`. Linux non ha la scheda di installazione né la scelta della modalità
portatile; vedi [Linux](/it/platforms/linux/).

## Modalità installata

Esegui il `moonpool.exe` scaricato. Al primo avvio mostra la scheda di installazione, che ha tre
controlli: il pulsante **Installa Moonpool**, una casella **Aggiungi un collegamento sul desktop**
(attiva per impostazione predefinita) e un link **Installa versione portatile**.

L'installazione copia Moonpool nel tuo profilo utente in `.moonpool\`, aggiunge un collegamento
nel menu Start (e uno sul desktop se la casella è selezionata) e registra una voce in
Installazione applicazioni. Poi avvia la copia installata e si chiude. Il file che hai scaricato
resta dov'era; puoi eliminarlo. Da quel momento avvia Moonpool dal collegamento, come qualsiasi
altra app.

![La scheda di installazione: pulsante Installa Moonpool, casella del collegamento sul desktop, link Installa versione portatile e percorso di installazione](../../../../assets/screenshots/installer-window.png)

Tutto ciò di cui Moonpool ha bisogno si trova in quell'unica cartella: il programma, la tua
configurazione e la guida inclusa.

```text title="Installed layout"
%USERPROFILE%\.moonpool\
```

## Installa Moonpool... dal menu

Su Windows il menu «...» contiene **Installa Moonpool...** in entrambe le modalità. Apre la
stessa scheda di installazione. Da una copia portatile puoi installarlo in modo definitivo. Da
una copia installata **Installa Moonpool** è disabilitato («Già installato») e **Installa
versione portatile** resta disponibile.

## Disinstallazione

Usa Installazione applicazioni di Windows (App installate), oppure esegui la copia installata con
`--uninstall`. Non è nel tuo PATH, quindi indica il percorso completo:

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" --uninstall
```

Questo rimuove i collegamenti nel menu Start e sul desktop, la voce del registro e l'intera
cartella `%USERPROFILE%\.moonpool`, **inclusa la tua configurazione** (`apps.json`, impostazioni
e registri). Fai prima un backup di questa cartella se vuoi conservare la tua configurazione:

```text
%USERPROFILE%\.moonpool\moonpool-config
```

Qualsiasi Moonpool in esecuzione viene arrestato durante la disinstallazione.

## Modalità portatile

Preferisci una chiavetta USB o una cartella spostabile? Fai clic su **Installa versione
portatile** nella scheda di installazione e scegli una cartella. Vedi
[Modalità portatile](/it/data/portable-mode/).

## Passi successivi

- [Windows protected your PC](/it/support/windows-protected-your-pc/): se SmartScreen blocca l'installer.
- [WebView2 runtime missing](/it/support/webview2-runtime-missing/): se la finestra resta vuota.
- [La tua prima app](/it/getting-started/first-app/)
