---
title: "Moonpool gebruiken op Windows"
description: "Windows is het belangrijkste Moonpool-platform: hoe je het installeert en een tabel met wat verschilt tussen Windows en Linux, zodat je weet wat je kunt verwachten."
---

Windows is het belangrijkste platform van Moonpool. Installeer het zoals beschreven in
[Installeren](/nl/getting-started/install/).

## Voordat je het uitvoert

- **SmartScreen.** `moonpool.exe` is niet met een codehandtekening ondertekend, dus Windows kan de
  eerste keer "Windows protected your PC" tonen. Kies **More info** (Meer info) en daarna **Run anyway** (Toch uitvoeren).
- **Antivirus.** Een nieuwe, niet-ondertekende exe die zichzelf kopieert en bij een update
  vervangt, kan een virusscanner activeren. Als je virusscanner `moonpool.exe` blokkeert of in
  quarantaine plaatst, sta het dan toe voor de map `.moonpool`.
- **WebView2.** De vensters van Moonpool gebruiken Microsoft Edge WebView2, dat met Windows 11 en
  het huidige Windows 10 wordt meegeleverd. Als het venster leeg blijft of nooit opent,
  installeer je de Evergreen WebView2 Runtime van Microsoft.

Meer: [Windows heeft uw pc beschermd](/nl/support/windows-protected-your-pc/),
[WebView2-runtime ontbreekt](/nl/support/webview2-runtime-missing/) en
[Een script of dev-server automatisch starten bij het aanmelden bij Windows](/nl/guides/start-app-at-windows-login/).

## Systeemvak

Op Windows 11 komt een nieuw pictogram in het systeemvak vaak in het gebied met verborgen
pictogrammen terecht. Klik op de pijl **^** rechts in de taakbalk om het te vinden en sleep het
naar de taakbalk om het zichtbaar te houden.

## Commando's

- Commando's draaien via `cmd /c`. Vermijd geneste dubbele aanhalingstekens in `command`; `cmd /c`
  maakt er een puinhoop van. Gebruik voor een script dat een shell open moet laten
  `pwsh -NoLogo -NoProfile -NoExit -Command <script and args>` zonder aanhalingstekens rond het
  scriptgedeelte.
- `processName` komt overeen met of zonder `.exe`, zonder onderscheid tussen hoofdletters en kleine letters.
- Stoppen beëindigt de hele procesboom die Moonpool heeft gestart, inclusief processen die zich
  ervan hebben losgemaakt.
- Docker Desktop-apps hebben `killMode` `none` of `command` nodig, nooit `port`. Zie
  [Docker-apps op Windows](/nl/apps/stop-and-restart/#docker-apps-onder-windows).

## Wat per platform verschilt

| | Windows | Linux |
| --- | --- | --- |
| Installatie | Zelfinstallerende `moonpool.exe`, of draagbaar | AppImage, `.deb` of RPM; geen installatiekaart |
| Zelf bijwerken | Ja, geïnstalleerd en draagbaar | Alleen AppImage |
| Configuratiemap | `%USERPROFILE%\.moonpool\moonpool-config\` | `~/.config/Moonpool/` |
| Shell voor commando's | `cmd /c` | `$SHELL -c` |
| `processName` | Elke lengte, `.exe` optioneel, hoofdletterongevoelig | Maximaal 15 tekens, exacte hoofdletters |
| Stoppen via `processName` | Beëindigt het proces en zijn onderliggende processen | Beëindigt alleen processen met exact die naam |
| Besturingskanaal | Named pipe | Unix-socket |
| Schermafbeeldingen van vensters (testen) | Ja | Nee |
| Pictogrammen uit een programmabestand | Ja | Nee |
| Systeemvak | Werkt standaard | Heeft AppIndicator nodig; standaard GNOME heeft een extensie nodig |

Details over Linux staan op de pagina [Linux](/nl/platforms/linux/).
