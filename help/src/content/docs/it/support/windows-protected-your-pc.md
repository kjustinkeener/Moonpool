---
title: "Windows protected your PC: esegui comunque l'installer di Moonpool (SmartScreen)"
description: "Windows SmartScreen mostra Windows protected your PC quando esegui moonpool.exe. Perché compare, come usare More info e Run anyway e cosa controllare prima."
---

Quando esegui il `moonpool.exe` scaricato, Windows può mostrare una finestra blu intitolata
**Windows protected your PC** (Windows ha protetto il PC), con la riga «Microsoft Defender
SmartScreen prevented an unrecognized app from starting. Running this app might put your PC at
risk.» (Microsoft Defender SmartScreen ha impedito l'avvio di un'app non riconosciuta.
L'esecuzione di questa app potrebbe mettere a rischio il PC.)

## Perché compare

SmartScreen avvisa per i programmi nuovi o che non ha visto eseguire su molti PC.
`moonpool.exe` non è firmato digitalmente, quindi Windows non ha un autore di cui fidarsi e la
prima volta può mostrare l'avviso. È un controllo di reputazione, non la constatazione che il
file sia dannoso.

## Cosa fare

1. Nella finestra, fai clic su **More info** (Ulteriori informazioni). L'autore compare come
   «Unknown publisher» (editore sconosciuto).
2. Fai clic su **Run anyway** (Esegui comunque). Si apre la scheda di installazione. Vedi
   [Installazione](/it/getting-started/install/).

Se vuoi essere prudente prima, scarica solo dal sito ufficiale di Moonpool o dalle sue release
su GitHub e controlla che il nome del file sia `moonpool.exe`.

## Se non c'è il pulsante Run anyway

Su alcuni PC gestiti l'amministratore disattiva l'opzione e non vedrai **Run anyway**. Chiedi al
tuo amministratore, oppure usa un PC che gestisci tu. Anche un file arrivato in uno zip scaricato
può portare con sé un blocco: fai clic destro sul file, scegli **Proprietà**, seleziona
**Sblocca** se compare, poi **OK** ed eseguilo di nuovo.

## Avvisi dell'antivirus

Un nuovo exe non firmato che si copia nel tuo profilo e si sostituisce a ogni aggiornamento può
far scattare anche un antivirus. Se il tuo blocca o mette in quarantena `moonpool.exe`,
consentilo per la cartella `.moonpool`. Vedi [Windows](/it/platforms/windows/#prima-di-eseguirlo).

## Vedi anche

- [Installazione](/it/getting-started/install/)
- [Windows](/it/platforms/windows/)
- [L'installer mostra un errore](/it/support/troubleshooting/#linstaller-mostra-un-errore)
