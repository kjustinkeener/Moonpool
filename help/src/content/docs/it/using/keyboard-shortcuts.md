---
title: "Scorciatoie da tastiera, scorciatoie del mouse e zoom di Moonpool"
description: "Tutte le scorciatoie da tastiera e del mouse nell'hub di Moonpool e come ingrandire o ridurre l'interfaccia per leggere il testo con comodità."
---

## Tastiera

| Tasti | Dove | Cosa fa |
| --- | --- | --- |
| F5, Ctrl+R, Cmd+R | Hub | Ricarica `apps.json` dal disco, come **Ricarica** nel menu. La pagina stessa non viene aggiornata. |
| Esc | Menu contestuale | Lo chiude. |
| Esc | Rinomina di un'app | Annulla la rinomina. |
| Esc | Finestre Impostazioni, Informazioni ed editor delle app | Chiude la finestra (l'editor chiede conferma prima di scartare le modifiche). |
| Invio | Rinomina di un'app | Salva il nuovo nome. |

## Mouse

| Azione | Dove | Cosa fa |
| --- | --- | --- |
| Ctrl + rotellina | Hub | Ingrandisce o riduce l'interfaccia. |
| Seleziona il testo | Terminale | Copia e annulla la selezione. |
| Clic centrale | Terminale | Incolla. |
| Clic destro | Riga della barra laterale | Apre il menu della riga. Vedi [Barra laterale e menu](/it/using/sidebar-and-menus/). |

## Zoom

Tieni premuto Ctrl e scorri la rotellina sull'hub per ingrandire o ridurre. Scorrendo in alto si
ingrandisce e in basso si riduce, con passi di circa il 10 percento per ogni evento della
rotellina.

```text
Ctrl + wheel up      zoom in
Ctrl + wheel down    zoom out
```

- L'intervallo va da 0,5x a 3x.
- La finestra si ridimensiona dello stesso fattore, quindi la disposizione resta compatta a 2x
  come a 1x. Una volta raggiunto il limite la finestra smette di crescere.
- Il fattore viene salvato come `uiScale` in `settings.json` e applicato al successivo avvio. La
  dimensione salvata della finestra è già quella ingrandita, quindi non viene scalata di nuovo.
  Vedi [settings.json](/it/data/settings-json/).

- Lo zoom si applica solo alla finestra hub. Impostazioni, Informazioni, l'editor delle app e
  Aiuto mantengono la propria dimensione.

Non esiste un controllo nelle Impostazioni per `uiScale` né un tasto di ripristino. Per tornare
alla dimensione normale, scorri indietro dello stesso numero di scatti, oppure esci da Moonpool,
imposta `uiScale` su `1` in `settings.json` (o rimuovi la chiave) e riavvialo.

## Vedi anche

- [Temi, lingua e trasparenza](/it/using/themes-and-language/)
- [Barra laterale e menu](/it/using/sidebar-and-menus/)
