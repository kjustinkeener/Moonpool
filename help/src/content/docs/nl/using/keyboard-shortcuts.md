---
title: "Sneltoetsen, muisacties en zoom in Moonpool"
description: "Bekijk elke toetsenbord- en muissnelkoppeling in de Moonpool-hub en hoe je de interface in- en uitzoomt zodat tekst prettig leesbaar blijft."
---

## Toetsenbord

| Toetsen | Waar | Functie |
| --- | --- | --- |
| F5, Ctrl+R, Cmd+R | Hub | Laadt `apps.json` opnieuw van schijf, net als **Opnieuw laden** in het menu. De pagina zelf wordt niet vernieuwd. |
| Esc | Contextmenu | Sluit het. |
| Esc | Een app hernoemen | Annuleert het hernoemen. |
| Esc | Vensters Instellingen, Over en app-editor | Sluit het venster (de editor vraagt eerst of wijzigingen mogen vervallen). |
| Enter | Een app hernoemen | Slaat de nieuwe naam op. |

## Muis

| Actie | Waar | Functie |
| --- | --- | --- |
| Ctrl + wiel | Hub | Zoomt de interface. |
| Tekst selecteren | Terminal | Kopieert en wist de selectie. |
| Middelste klik | Terminal | Plakt. |
| Rechtsklik | Zijbalkrij | Opent het rijmenu. Zie [Zijbalk en menu's](/nl/using/sidebar-and-menus/). |

## Zoom

Houd Ctrl ingedrukt en scrol met het wiel boven de hub om te zoomen. Omhoog scrollen zoomt in en omlaag zoomt uit, in stappen van ongeveer 10 procent per wielbeweging.

```text
Ctrl + wheel up      zoom in
Ctrl + wheel down    zoom out
```

- Het bereik is 0,5x tot 3x.
- Het venster wordt met dezelfde factor aangepast, zodat de indeling bij 2x even compact blijft als bij 1x. Zodra de limiet is bereikt, groeit het venster niet verder.
- De factor wordt opgeslagen als `uiScale` in `settings.json` en bij de volgende start toegepast. De opgeslagen venstergrootte is al de gezoomde grootte en wordt dus niet opnieuw geschaald. Zie [settings.json](/nl/data/settings-json/).

- Zoom geldt alleen voor het hubvenster. Instellingen, Over, de app-editor en Help behouden
  hun eigen grootte.

Er is geen bedieningselement in Instellingen voor `uiScale` en geen toets om te resetten. Om terug te
gaan naar de normale grootte, scrol je hetzelfde aantal stappen terug, of sluit je Moonpool af, zet je
`uiScale` in `settings.json` op `1` (of verwijder je de sleutel) en start je het opnieuw.

## Zie ook

- [Thema's, taal en transparantie](/nl/using/themes-and-language/)
- [Zijbalk en menu's](/nl/using/sidebar-and-menus/)
