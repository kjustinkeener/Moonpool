---
title: "Het Moonpool-thema, de taal en de transparantie wijzigen"
description: "Kies een kleurthema en interfacetaal, stel achtergrondtransparantie en UI-schaal in en zie ze direct toegepast in elk geopend Moonpool-venster."
---

Thema, taal en transparantie stel je in het [venster Instellingen](/nl/using/settings/) in.
Alle drie gelden direct voor elk geopend Moonpool-venster.

![Keuzes voor Taal (1) en Thema (2) bovenaan Instellingen](../../../../assets/screenshots/settings-language-theme.png)

1. Taalkeuze.
2. Themaknop. Hij toont de naam van het huidige thema en opent de themabrowser.

## Thema's

De themabrowser is een eigen venster. Hij heeft één voorbeeldkaart per thema, elk getekend in de
eigen kleuren van dat thema (tekst, paneel, invoer, knop, statusstippen, het verloop van de meter
en de terminalset van 16 kleuren), gegroepeerd als Basis, Neon, Warm, Koel, Groen, Neutraal, Licht,
Blush, Helder, Licht pastel en Pastel. Klik op een kaart om hem toe te passen: elk geopend
Moonpool-venster verandert tegelijk en de keuze wordt opgeslagen. Het venster blijft open zodat je
kunt vergelijken; druk op Esc om te sluiten.

Er zijn 68 thema's plus Automatisch, en ongeveer de helft is licht. Themanamen zijn eigennamen en
worden niet vertaald; alleen Automatisch (systeem), Donker en Licht worden vertaald.

**Automatisch (systeem)** volgt de voorkeur voor licht of donker van het besturingssysteem en
schakelt live mee wanneer het besturingssysteem dat doet. Elke andere keuze is vast. De 16
ANSI-kleuren van de terminal volgen het thema ook.

Als je in een eerdere versie een thema hebt opgeslagen, blijft dat behouden. Een opgeslagen naam die
Moonpool niet meer kent, valt terug op Automatisch. Sommige labels verschillen van vroeger
(bijvoorbeeld Matrix heet nu Terminal, Nord heet Arctic, Dracula heet Nocturne, Gruvbox heet Retro
en Solarized heet Solar); de opgeslagen keuze zelf is ongewijzigd.

Het thema wordt opgeslagen in de `localStorage` van de webview, niet in `settings.json`. Als de
opslag niet beschikbaar is, valt het terug op Automatisch.

```text
localStorage key: moonpool.theme
```

## Talen

Automatisch volgt de taal van het besturingssysteem. Anders kies je een van de 14, elk getoond in de eigen taal:

```text
English, Deutsch, Español, Français, Italiano, 日本語, 한국어, Nederlands, Polski,
Português (Brasil), Русский, Türkçe, 简体中文, 繁體中文
```

De keuze geldt direct voor de hub en de andere vensters. De keuze wordt opgeslagen als
`locale` in `settings.json`.

## Transparantie

**Transparantie van de achtergrond** maakt de vensterachtergrond doorschijnend, van 0% (dekkend) tot
90%.

- Als je de aanwijzer boven een venster houdt, wordt het meteen volledig dekkend. Als de aanwijzer weggaat, vervaagt het in ongeveer 2 seconden terug naar je instelling.
- Terminals volgen dezelfde tint in plaats van een eigen tint toe te voegen.
- Elk venster (hub, Instellingen, Over, de app-editor en de themabrowser) past de instelling zelf toe, en Instellingen werkt de andere live bij terwijl je de schuifregelaar sleept.

## UI-schaal

Zoom de hele interface met Ctrl + muiswiel. Er is geen zoom via het toetsenbord. Zie
[Sneltoetsen en zoom](/nl/using/keyboard-shortcuts/).

## Zie ook

- [Venster Instellingen](/nl/using/settings/)
- [Sneltoetsen en zoom](/nl/using/keyboard-shortcuts/)
