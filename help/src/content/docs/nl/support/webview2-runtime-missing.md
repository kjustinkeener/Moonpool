---
title: "WebView2-runtime ontbreekt: een leeg of ontbrekend Moonpool-venster op Windows oplossen"
description: "Als het venster van Moonpool op Windows nooit opent of leeg blijft, ontbreekt mogelijk de Microsoft Edge WebView2 Runtime. Zo controleert en installeer je die."
---

Als het venster van Moonpool op Windows nooit opent, of opent en leeg blijft, is de waarschijnlijke
oorzaak een ontbrekende Microsoft Edge WebView2 Runtime. Moonpool is een Tauri-app en de vensters
zijn webpagina's die door WebView2 worden getekend.

WebView2 wordt meegeleverd met Windows 11 en met het huidige Windows 10, dus de meeste pc's hebben
het al. Het kan ontbreken op een oudere of uitgeklede Windows 10, of op een pc waar het is
verwijderd. In de broncode van Moonpool is voor dit geval geen aparte melding te vinden, dus er
wordt hier geen fouttekst geciteerd: het symptoom is dat het venster niet verschijnt of leeg is.

## Controleren of het is geïnstalleerd

Zoek in PowerShell de versie van de runtime in het register (het eerste pad is de installatie voor
het hele systeem, het tweede een installatie per gebruiker):

```powershell frame="terminal"
Get-ItemProperty "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
Get-ItemProperty "HKCU:\Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
```

Een versienummer zoals `120.0.2210.91` betekent dat het is geïnstalleerd. Een fout bij beide
betekent dat het niet is geïnstalleerd.

## Installeren

Download de **Evergreen** WebView2 Runtime van de WebView2-pagina van Microsoft (zoek op "WebView2
Runtime download"), voer het installatieprogramma uit en start Moonpool daarna opnieuw. De
Evergreen-runtime werkt zichzelf bij.

## Als het is geïnstalleerd en het venster toch leeg blijft

- Sluit elke Moonpool af vanuit het systeemvak (of beëindig `moonpool.exe` in Taakbeheer) en start
  het opnieuw.
- Zet **Debug-informatie naar een bestand loggen** aan in [Instellingen](/nl/using/settings/) als je
  erbij kunt, en bekijk `moonpool.log`. Zie [Logs](/nl/data/logs/).
- Als het venster opent maar buiten het scherm staat, zie
  [Vensterproblemen](/nl/support/troubleshooting/#vensterproblemen).

## Zie ook

- [Windows](/nl/platforms/windows/#voordat-je-het-uitvoert)
- [Installeren](/nl/getting-started/install/)
- [Probleemoplossing](/nl/support/troubleshooting/)
