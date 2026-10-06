---
title: "Utiliser Moonpool sous Windows"
description: "Windows est la plateforme principale de Moonpool : comment l'installer, et un tableau des différences entre Windows et Linux pour savoir à quoi vous attendre."
---

Windows est la plateforme principale de Moonpool. Installez-le comme décrit dans
[Installation](/fr/getting-started/install/).

## Avant de l'exécuter

- **SmartScreen.** `moonpool.exe` n'est pas signé : Windows peut donc afficher « Windows a
  protégé votre ordinateur » (Windows protected your PC) la première fois. Choisissez
  **Informations complémentaires** (More info), puis **Exécuter quand même** (Run anyway).
- **Antivirus.** Un nouvel exe non signé qui se copie et se remplace lors d'une mise à jour
  peut déclencher un antivirus. Si le vôtre bloque ou met en quarantaine `moonpool.exe`,
  autorisez-le pour le dossier `.moonpool`.
- **WebView2.** Les fenêtres de Moonpool utilisent Microsoft Edge WebView2, fourni avec
  Windows 11 et les versions récentes de Windows 10. Si la fenêtre reste vide ou ne s'ouvre
  jamais, installez le runtime WebView2 Evergreen de Microsoft.

Pour en savoir plus : [Windows a protégé votre ordinateur](/fr/support/windows-protected-your-pc/),
[Runtime WebView2 manquant](/fr/support/webview2-runtime-missing/) et
[Démarrer un script ou un serveur de développement automatiquement à l'ouverture de session Windows](/fr/guides/start-app-at-windows-login/).

## Zone de notification

Sous Windows 11, une nouvelle icône de la zone de notification se place souvent dans la zone
des icônes masquées. Cliquez sur la flèche **^** à droite de la barre des tâches pour la
trouver, et faites-la glisser sur la barre des tâches pour la garder visible.

## Commandes

- Les commandes s'exécutent via `cmd /c`. Évitez les guillemets doubles imbriqués dans
  `command` ; `cmd /c` les déforme. Pour un script qui doit laisser un shell ouvert, utilisez
  `pwsh -NoLogo -NoProfile -NoExit -Command <script and args>` sans guillemets autour de la
  partie script.
- `processName` correspond avec ou sans `.exe`, sans tenir compte de la casse.
- Arrêter met fin à toute l'arborescence de processus démarrée par Moonpool, y compris les
  processus qui s'en sont détachés.
- Les apps Docker Desktop exigent `killMode` `none` ou `command`, jamais `port`. Voir
  [Apps Docker sous Windows](/fr/apps/stop-and-restart/#apps-docker-sous-windows).

## Ce qui diffère selon la plateforme

| | Windows | Linux |
| --- | --- | --- |
| Installation | `moonpool.exe` auto-installant, ou portable | AppImage, `.deb` ou RPM ; pas de carte d'installation |
| Mise à jour automatique | Oui, installé et portable | AppImage uniquement |
| Dossier de configuration | `%USERPROFILE%\.moonpool\moonpool-config\` | `~/.config/Moonpool/` |
| Shell pour les commandes | `cmd /c` | `$SHELL -c` |
| `processName` | Toute longueur, `.exe` facultatif, insensible à la casse | 15 caractères ou moins, casse exacte |
| Arrêt par `processName` | Termine le processus et ses enfants | Termine uniquement les processus portant exactement ce nom |
| Canal de contrôle | Tube nommé | Socket Unix |
| Captures d'écran de fenêtres (tests) | Oui | Non |
| Icônes depuis un fichier programme | Oui | Non |
| Zone de notification | Fonctionne d'office | Nécessite AppIndicator ; GNOME standard exige une extension |

Les détails pour Linux se trouvent sur la page [Linux](/fr/platforms/linux/).
