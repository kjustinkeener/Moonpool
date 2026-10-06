---
title: "Windows a protégé votre ordinateur : exécuter quand même l'installateur de Moonpool (SmartScreen)"
description: "SmartScreen affiche « Windows a protégé votre ordinateur » avec moonpool.exe. Pourquoi, et comment choisir Informations complémentaires puis Exécuter quand même."
---

Quand vous exécutez le `moonpool.exe` téléchargé, Windows peut afficher une boîte bleue intitulée
**Windows a protégé votre ordinateur** (« Windows protected your PC »), avec la ligne
« Microsoft Defender SmartScreen a empêché le démarrage d'une application non reconnue.
L'exécution de cette application peut mettre votre ordinateur en danger. » (en anglais :
"Microsoft Defender SmartScreen prevented an unrecognized app from starting. Running this app
might put your PC at risk.")

## Pourquoi ce message apparaît

SmartScreen avertit pour les programmes récents ou qu'il n'a pas vus s'exécuter sur de nombreux
PC. `moonpool.exe` n'est pas signé numériquement : Windows n'a donc aucun éditeur à qui faire
confiance et peut afficher l'avertissement la première fois. C'est une vérification de
réputation, pas la constatation que le fichier est malveillant.

## Que faire

1. Dans la boîte, cliquez sur **Informations complémentaires** (« More info »). L'éditeur
   s'affiche comme « Éditeur inconnu » (« Unknown publisher »).
2. Cliquez sur **Exécuter quand même** (« Run anyway »). La carte d'installation s'ouvre. Voir
   [Installation](/fr/getting-started/install/).

Si vous voulez d'abord être prudent, téléchargez uniquement depuis le site officiel de Moonpool ou
ses versions GitHub, et vérifiez que le nom du fichier est `moonpool.exe`.

## S'il n'y a pas de bouton Exécuter quand même

Sur certains PC gérés, l'administrateur désactive l'option, et vous ne verrez pas **Exécuter quand
même**. Demandez à votre administrateur, ou utilisez un PC que vous gérez. Un fichier arrivé dans
un zip téléchargé peut aussi porter un blocage : faites un clic droit sur le fichier, choisissez
**Propriétés**, cochez **Débloquer** s'il apparaît, puis **OK** et exécutez-le de nouveau.

## Avertissements de l'antivirus

Un nouvel exe non signé qui se copie dans votre profil et se remplace lui-même lors d'une mise à
jour peut aussi déclencher un antivirus. Si le vôtre bloque ou met en quarantaine `moonpool.exe`,
autorisez-le pour le dossier `.moonpool`. Voir
[Windows](/fr/platforms/windows/#avant-de-lexécuter).

## Voir aussi

- [Installation](/fr/getting-started/install/)
- [Windows](/fr/platforms/windows/)
- [L'installateur affiche une erreur](/fr/support/troubleshooting/#linstallateur-affiche-une-erreur)
