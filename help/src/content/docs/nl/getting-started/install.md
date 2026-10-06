---
title: "Moonpool installeren op Windows of Linux"
description: "Installeer Moonpool in een paar klikken, kies tussen geïnstalleerde en draagbare modus, gebruik later het menu-item Moonpool installeren en verwijder het netjes."
---

Deze pagina is voor Windows. Op Windows is Moonpool zijn eigen installatieprogramma: de
download is één enkele `moonpool.exe`. Linux heeft geen installatiekaart en geen keuze voor
draagbare modus; zie [Linux](/nl/platforms/linux/).

## Geïnstalleerde modus

Voer de gedownloade `moonpool.exe` uit. Bij de eerste start toont het de installatiekaart.
Die heeft drie bedieningselementen: de knop **Moonpool installeren**, een selectievakje
**Snelkoppeling op het bureaublad maken** (standaard aan) en een koppeling **Portable installeren**.

Bij het installeren wordt Moonpool gekopieerd naar je gebruikersprofiel onder `.moonpool\`,
wordt een snelkoppeling in het Startmenu toegevoegd (en een op het bureaublad als het vakje
is aangevinkt) en wordt een item geregistreerd in Programma's toevoegen/verwijderen.
Daarna start het de geïnstalleerde kopie en sluit zichzelf. Het bestand dat je hebt
gedownload blijft waar het was; je kunt het verwijderen. Daarna start je Moonpool via de
snelkoppeling, zoals elke andere app.

![De installatiekaart: de knop Moonpool installeren, het selectievakje voor de snelkoppeling op het bureaublad, de koppeling Portable installeren en het installatiepad](../../../../assets/screenshots/installer-window.png)

Alles wat Moonpool nodig heeft staat in die ene map: het programma, je configuratie en de
meegeleverde help.

```text title="Installed layout"
%USERPROFILE%\.moonpool\
```

## Moonpool installeren... via het menu

Op Windows heeft het menu "..." in beide modi **Moonpool installeren…**. Het opent dezelfde
installatiekaart. Vanuit een draagbare kopie kun je Moonpool alsnog netjes installeren.
Vanuit een geïnstalleerde kopie is **Moonpool installeren** uitgeschakeld ("Al
geïnstalleerd") en blijft **Portable installeren** beschikbaar.

## Verwijderen

Gebruik Programma's toevoegen/verwijderen van Windows (Geïnstalleerde apps), of voer de
geïnstalleerde kopie uit met `--uninstall`. Het staat niet in je PATH, dus geef het volledige pad op:

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" --uninstall
```

Dit verwijdert de snelkoppelingen in het Startmenu en op het bureaublad, het
registeritem en de hele map `%USERPROFILE%\.moonpool`, **inclusief je configuratie**
(`apps.json`, instellingen en logs). Maak eerst een back-up van deze map als je je
configuratie wilt bewaren:

```text
%USERPROFILE%\.moonpool\moonpool-config
```

Een draaiende Moonpool wordt als onderdeel van het verwijderen gestopt.

## Draagbare modus

Liever een usb-stick of een verplaatsbare map? Klik op de installatiekaart op **Portable
installeren** en kies een map. Zie [Draagbare modus](/nl/data/portable-mode/).

## Volgende

- [Windows heeft uw pc beschermd](/nl/support/windows-protected-your-pc/): als SmartScreen het installatieprogramma blokkeert.
- [WebView2-runtime ontbreekt](/nl/support/webview2-runtime-missing/): als het venster leeg blijft.
- [Je eerste app](/nl/getting-started/first-app/)
