---
title: "Cambia tema, lingua e trasparenza di Moonpool"
description: "Scegli un tema di colori e la lingua dell'interfaccia, imposta trasparenza dello sfondo e scala dell'interfaccia e applicali a ogni finestra aperta."
---

Tema, lingua e trasparenza si impostano nella [finestra Impostazioni](/it/using/settings/).
Tutti e tre si applicano subito a ogni finestra di Moonpool aperta.

![Selettori Lingua (1) e Tema (2) in cima alle Impostazioni](../../../../assets/screenshots/settings-language-theme.png)

1. Selettore della lingua.
2. Pulsante Tema. Mostra il nome del tema corrente e apre il browser dei temi.

## Temi

Il browser dei temi è una finestra a sé. Ha una scheda di anteprima per ogni tema, disegnata nei
colori del tema stesso (testo, pannello, campo di input, pulsante, pallini di stato, sfumatura
del misuratore e i 16 colori del terminale), raggruppati come Base, Neon, Caldi, Freddi, Verdi,
Neutri, Chiari, Rosati, Vivaci, Pastello chiaro e Pastello. Fai clic su una scheda per applicarla:
ogni finestra di Moonpool aperta cambia contemporaneamente e la scelta viene salvata. La finestra
resta aperta così puoi confrontare; premi Esc per chiuderla.

Ci sono 68 temi più Automatico, e circa la metà sono chiari. I nomi dei temi sono nomi propri e
non vengono tradotti; solo Automatico (sistema), Scuro e Chiaro lo sono.

**Automatico (sistema)** segue la preferenza chiaro o scuro del sistema operativo e cambia in
tempo reale quando cambia il sistema. Qualsiasi altra scelta è fissa. Anche i 16 colori ANSI del
terminale seguono il tema.

Se avevi salvato un tema in una versione precedente, viene mantenuto. Un nome salvato che
Moonpool non conosce più torna ad Automatico. Alcune etichette sono diverse da prima (ad esempio
Matrix ora si chiama Terminal, Nord è Arctic, Dracula è Nocturne, Gruvbox è Retro e Solarized è
Solar); la scelta salvata in sé non cambia.

Il tema è memorizzato nel `localStorage` della webview, non in `settings.json`. Se l'archivio non
è disponibile, ripiega su Automatico.

```text
localStorage key: moonpool.theme
```

## Lingue

Automatico segue la lingua del sistema operativo. Altrimenti scegli una delle 14, ciascuna
mostrata nella propria lingua:

```text
English, Deutsch, Español, Français, Italiano, 日本語, 한국어, Nederlands, Polski,
Português (Brasil), Русский, Türkçe, 简体中文, 繁體中文
```

Il selettore si applica subito all'hub e alle altre finestre. La scelta viene salvata come
`locale` in `settings.json`.

## Trasparenza

**Trasparenza dello sfondo** rende trasparente lo sfondo della finestra, da 0% (opaco) a 90%.

- Passando il puntatore su una finestra, questa diventa subito completamente opaca. Quando il puntatore se ne va, torna gradualmente al valore impostato in circa 2 secondi.
- I terminali seguono la stessa tinta invece di aggiungerne una propria.
- Ogni finestra (hub, Impostazioni, Informazioni, l'editor delle app e il browser dei temi) applica l'impostazione per conto proprio, e Impostazioni aggiorna le altre in tempo reale mentre trascini il cursore.

## Scala dell'interfaccia

Ingrandisci o riduci l'intera interfaccia con Ctrl + rotellina del mouse. Non esiste uno zoom da
tastiera. Vedi [Scorciatoie e zoom](/it/using/keyboard-shortcuts/).

## Vedi anche

- [Finestra delle impostazioni](/it/using/settings/)
- [Scorciatoie e zoom](/it/using/keyboard-shortcuts/)
