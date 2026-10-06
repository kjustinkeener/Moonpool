---
title: "Windows heeft uw pc beschermd: het Moonpool-installatieprogramma toch uitvoeren (SmartScreen)"
description: "Windows SmartScreen toont Windows protected your PC bij het uitvoeren van moonpool.exe. Waarom dat gebeurt, hoe je More info en daarna Run anyway kiest en wat je eerst controleert."
---

Als je de gedownloade `moonpool.exe` uitvoert, kan Windows een blauw vak tonen met de titel **Windows
protected your PC**, met de regel "Microsoft Defender SmartScreen prevented an unrecognized
app from starting. Running this app might put your PC at risk." (in het Nederlands: SmartScreen
heeft voorkomen dat een niet-herkende app is gestart; het uitvoeren van deze app kan je pc in
gevaar brengen).

## Waarom het verschijnt

SmartScreen waarschuwt voor programma's die nieuw zijn of die het niet op veel pc's heeft zien
draaien. `moonpool.exe` is niet met een codehandtekening ondertekend, dus Windows heeft geen
uitgever om te vertrouwen en kan de eerste keer de waarschuwing tonen. Het is een
reputatiecontrole, geen vaststelling dat het bestand schadelijk is.

## Wat te doen

1. Klik in het vak op **More info** (Meer info). De uitgever wordt getoond als "Unknown publisher"
   (Onbekende uitgever).
2. Klik op **Run anyway** (Toch uitvoeren). De installatiekaart opent. Zie [Installeren](/nl/getting-started/install/).

Als je eerst voorzichtig wilt zijn, download dan alleen van de officiële site van Moonpool of de
GitHub-releases ervan en controleer of de bestandsnaam `moonpool.exe` is.

## Als er geen knop Run anyway is

Op sommige beheerde pc's zet de beheerder de optie uit en zie je geen **Run anyway**. Vraag het je
beheerder, of gebruik een pc die je zelf beheert. Een bestand dat in een gedownload zipbestand zat,
kan ook een blokkade dragen: klik met rechts op het bestand, kies **Properties** (Eigenschappen),
vink **Unblock** (Blokkering opheffen) aan als die verschijnt, kies dan **OK** en voer het opnieuw uit.

## Waarschuwingen van antivirussoftware

Een nieuwe niet-ondertekende exe die zichzelf naar je profiel kopieert en bij een update vervangt,
kan ook antivirussoftware activeren. Als je virusscanner `moonpool.exe` blokkeert of in quarantaine plaatst,
sta het dan toe voor de map `.moonpool`. Zie [Windows](/nl/platforms/windows/#voordat-je-het-uitvoert).

## Zie ook

- [Installeren](/nl/getting-started/install/)
- [Windows](/nl/platforms/windows/)
- [Het installatieprogramma toont een fout](/nl/support/troubleshooting/#het-installatieprogramma-toont-een-fout)
