---
title: "Moonpool bijwerken en een mislukte update oplossen"
description: "Zie hoe Moonpool updates controleert, downloadt en toepast, wat de updatebanner doet, hoe draagbare en Linux-kopieën bijwerken en wat je bij een fout doet."
---

Moonpool werkt zichzelf bij. Er is geen apart installatieprogramma om te downloaden en geen wizard om
doorheen te klikken.

## Hoe updates binnenkomen

Moonpool haalt `update.json` (`linux-update.json` onder Linux) op van de GitHub
Releases van het project, vergelijkt versies en biedt alleen een strikt nieuwere aan. Het controleert:

- bij het opstarten, tenzij **Bij het opstarten controleren op updates** uit staat in
  [Instellingen](/nl/using/settings/);
- telkens wanneer je in het venster Over op **Controleren op updates** drukt. Die knop installeert een nieuwere
  versie meteen en herstart Moonpool. Anders meldt hij dat je de nieuwste
  versie hebt, of toont hij de fout.

Het venster Over toont de versie die je draait, onder de naam:

![De bovenkant van het venster Over: het logo, de naam (1) en de versieregel eronder](../../../../assets/screenshots/about-header.png)

1. De naam. De regel eronder is de versie en de builddatum.

Elke download wordt geverifieerd met de minisign-ondertekeningssleutel van Moonpool voordat hij wordt toegepast, zodat
een gemanipuleerde of beschadigde download wordt geweigerd. Moonpool installeert nooit een oudere versie.

## De updatebanner

Bij het opstarten verschijnt een gevonden update als banner op het lege scherm van de hub:

```text
Moonpool {version} is available (you have {current}).
```

(In de app: "Moonpool {version} is beschikbaar (je hebt {current}).")

De banner verschijnt alleen als er geen app-tabblad open is en het CLI-paneel is uitgeklapt. Als het paneel
is ingeklapt, pulseert in plaats daarvan het chevron naast het filtervak. Met een open tabblad is er helemaal geen
teken. Om de banner te zien sluit je alle tabbladen (en klap je het paneel uit), of gebruik je **Controleren op
updates** in het venster Over.

Klik op **Downloaden en installeren** en Moonpool vervangt zichzelf en start opnieuw, of verberg de
banner met de x.

## Draagbare kopieën

Een draagbare kopie werkt de `moonpool.exe` in zijn eigen map `.moonpool\` op dezelfde manier bij.
Elke kopie controleert en werkt zelfstandig bij. De map moet beschrijfbaar zijn, dus een kopie op een
alleen-lezen stick of share kan zichzelf niet bijwerken; kopieer handmatig een nieuwere `moonpool.exe` eroverheen.

## Linux

Alleen de AppImage werkt zichzelf bij. Hij vervangt het AppImage-bestand ter plekke, dus bewaar hem in een
map waarin je kunt schrijven. Een `.deb`- of RPM-installatie wordt bijgewerkt door je pakketbeheerder:
installeren vanuit Moonpool mislukt met

```text
automatic updates are available for the AppImage only; update the .deb or RPM with your package manager
```

Zie [Linux](/nl/platforms/linux/#updates).

## Wanneer een update mislukt

De banner toont de reden en de knop wordt weer beschikbaar zodat je het opnieuw kunt proberen:

```text
Update failed: <error>
```

(In de app: "Update mislukt: {error}".)

| De fout bevat | Waarschijnlijke oorzaak | Wat te doen |
| --- | --- | --- |
| `download failed` | Geen verbinding, een proxy, of GitHub die verzoeken beperkt | Wacht en probeer opnieuw, of werk handmatig bij. |
| `signature verification FAILED - refusing to install` | De download is beschadigd of gewijzigd | Probeer opnieuw. Blijft het mislukken, werk dan handmatig bij vanaf de Releases-pagina. |
| `rename self aside` of `write new exe` | De map is alleen-lezen, of een antivirusprogramma houdt het bestand vast | Maak de map beschrijfbaar, of sta `moonpool.exe` toe in je antivirusprogramma, en probeer opnieuw. |
| `refusing to install ... not newer than current` | De aangeboden versie is niet nieuwer | Niets te doen. |

### Handmatig bijwerken

Sluit Moonpool af, download `moonpool.exe` van de
[Releases-pagina](https://github.com/kjustinkeener/Moonpool/releases) van het project en kopieer het over de
oude heen: `%USERPROFILE%\.moonpool\moonpool.exe` bij een geïnstalleerde, of die in je map `.moonpool\`
bij een draagbare kopie. Je configuratiemap blijft onaangeroerd. Vervang onder Linux de
AppImage, of gebruik je pakketbeheerder.

## Ook de help wordt bijgewerkt

Deze help wordt met Moonpool meegeleverd, dus elke programma-update brengt de bijbehorende help mee.
De offlinekopie komt altijd overeen met de versie die je draait.

## Zie ook

- [Wat is er nieuw](/nl/getting-started/whats-new/)
- [Venster Instellingen](/nl/using/settings/)
