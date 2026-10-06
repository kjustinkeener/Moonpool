---
title: "Moonpool installeren en gebruiken op Linux"
description: "Installeer Moonpool op Linux, omzeil het probleem met het systeemvak op GNOME, leer hoe updates werken en zie welke functies afwijken van de Windows-versie."
---

Moonpool draait op Linux via WebKitGTK. Het wordt vooral op Windows ontwikkeld, dus Linux wordt
ondersteund maar is minder grondig beproefd. Op Linux is er geen installatiekaart of keuze voor
draagbare modus, en het menu "..." heeft geen item **Moonpool installeren…**.

## Installeren

Download een pakket van de Releases-pagina van het project.

| Pakket | Updates |
| --- | --- |
| AppImage | Moonpool werkt zichzelf bij |
| `.deb` | Je pakketbeheerder |
| RPM (installeren met het RPM-hulpprogramma van je distributie) | Je pakketbeheerder |

```bash title="AppImage" frame="terminal"
chmod +x Moonpool_*.AppImage
./Moonpool_*.AppImage
```

```bash title=".deb" frame="terminal"
sudo apt install ./Moonpool_*_amd64.deb
```

```bash title="RPM" frame="terminal"
sudo dnf install ./Moonpool-*.x86_64.rpm
```

Het `.deb`-pakket haalt de benodigde runtime-afhankelijkheden zelf binnen. De AppImage heeft de
bibliotheken WebKitGTK en AppIndicator nodig, bijvoorbeeld op Debian of Ubuntu:

```bash frame="terminal"
sudo apt-get install -y libwebkit2gtk-4.1-0 libayatana-appindicator3-1
```

Gebruik op Fedora of Arch de equivalenten:

```bash title="Fedora" frame="terminal"
sudo dnf install webkit2gtk4.1 libayatana-appindicator-gtk3
```

```bash title="Arch" frame="terminal"
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator
```

## Systeemvak op GNOME

Standaard GNOME toont geen pictogrammen in het systeemvak, dus het pictogram van Moonpool
verschijnt pas nadat de AppIndicator-extensie is geïnstalleerd en ingeschakeld:

```bash frame="terminal"
sudo apt-get install -y gnome-shell-extension-appindicator
gnome-extensions enable ubuntu-appindicators@ubuntu.com
```

Meld je daarna af en weer aan. Het hubvenster en de ingebouwde terminals werken ook zonder. KDE,
Cinnamon, XFCE en MATE tonen het systeemvak standaard.

## Updates

Alleen de AppImage werkt zichzelf bij. Die leest `linux-update.json` uit GitHub Releases,
controleert de minisign-handtekening en vervangt het AppImage-bestand ter plekke, dus bewaar het
in een map waarin je kunt schrijven. Installaties met `.deb` en RPM worden nooit door Moonpool
overschreven: de updatecontrole kan nog steeds een nieuwere versie melden, maar installeren
vanuit Moonpool mislukt dan met een melding dat je je pakketbeheerder moet gebruiken. Zie
[Bijwerken](/nl/data/updating/).

## Locatie van de configuratie

```text
~/.config/Moonpool/              (or $XDG_CONFIG_HOME/Moonpool/)
~/.config/Moonpool/apps.json
~/.config/Moonpool/dashboards/examples/   (example dashboards)
```

`apps.json` wordt bij de eerste start gevuld vanuit het voorbeeld. Zie
[Overzicht van de configuratie](/nl/apps/apps-json/).

## Verschillen met Windows

- Startcommando's draaien via `$SHELL -c <command>` (`/bin/sh` als `SHELL` niet is ingesteld), dus gebruik syntaxis die je shell begrijpt.
- Stoppen beëindigt de procesgroep en voert daarna de extra opruimstap uit die door `killMode` is gekozen. Een poort vrijgeven onder `killMode: "port"` gebruikt `lsof`, met `fuser` als terugval; installeer `lsof` als je distributie het niet meelevert. Zie [Stoppen en herstarten](/nl/apps/stop-and-restart/).
- De `processName` van een `desktop`-app mag maximaal 15 tekens lang zijn. Linux kapt een procesnaam af tot 15 tekens, dus een langere naam wordt nooit als actief gedetecteerd en kan niet op naam worden gestopt. `web`-apps worden op hun poort herkend en hebben hier geen last van.
- Pictogrammen worden gevonden in `src-tauri/icons/`, `public/favicon.*`, `icon.png` van een app of in de live favicon. Een pictogram uit een binair bestand halen kan alleen op Windows.
- De knoppen om te openen tonen de map waarin het bestand staat in plaats van het bestand te selecteren.
- Configuratiebestanden openen in je standaardteksteditor (bepaald via de koppeling `text/plain`).
- Het Windows-installatieprogramma, de snelkoppelingen en het item in Programma's toevoegen/verwijderen zijn niet van toepassing.

## Zie ook

- [Windows](/nl/platforms/windows/#wat-per-platform-verschilt): een tabel met wat per platform verschilt.
- [Bijwerken](/nl/data/updating/#linux)
