---
title: "Installer Moonpool sous Windows ou Linux"
description: "Installez Moonpool en quelques clics, choisissez le mode installé ou portable, utilisez plus tard Installer Moonpool dans le menu et désinstallez proprement."
---

Cette page concerne Windows. Sous Windows, Moonpool est son propre installeur : le
téléchargement est un seul `moonpool.exe`. Linux n'a ni carte d'installation ni sélecteur
portable ; voir [Linux](/fr/platforms/linux/).

## Mode installé

Exécutez le `moonpool.exe` téléchargé. Au premier lancement, il affiche la carte
d'installation. Elle comporte trois commandes : le bouton **Installer Moonpool**, une case
**Ajouter un raccourci sur le bureau** (cochée par défaut) et un lien **Installer en portable**.

L'installation copie Moonpool dans votre profil utilisateur sous `.moonpool\`, ajoute un
raccourci dans le menu Démarrer (et un sur le bureau si la case est cochée) et enregistre une
entrée dans Ajout/Suppression de programmes. Elle démarre ensuite la copie installée et se
ferme. Le fichier que vous avez téléchargé reste où il était ; vous pouvez le supprimer.
Ensuite, lancez Moonpool depuis le raccourci, comme n'importe quelle autre app.

![La carte d'installation : bouton Installer Moonpool, case de raccourci sur le bureau, lien Installer en portable et chemin d'installation](../../../../assets/screenshots/installer-window.png)

Tout ce dont Moonpool a besoin se trouve sous ce dossier unique : le programme, votre
configuration et son aide intégrée.

```text title="Installed layout"
%USERPROFILE%\.moonpool\
```

## Installer Moonpool... depuis le menu

Sous Windows, le menu « ... » contient **Installer Moonpool…** dans les deux modes. Il ouvre la même
carte d'installation. Depuis une copie portable, vous pouvez l'installer pour de bon. Depuis
une copie installée, **Installer Moonpool** est désactivé (« Déjà installé ») et **Installer en
portable** reste disponible.

## Désinstallation

Utilisez Ajout/Suppression de programmes de Windows (Applications installées), ou exécutez la
copie installée avec `--uninstall`. Elle n'est pas dans votre PATH : indiquez son chemin
complet.

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" --uninstall
```

Cela supprime les raccourcis du menu Démarrer et du bureau, l'entrée de registre et tout le
dossier `%USERPROFILE%\.moonpool`, **y compris votre configuration** (`apps.json`, paramètres
et journaux). Sauvegardez d'abord ce dossier si vous voulez conserver votre configuration :

```text
%USERPROFILE%\.moonpool\moonpool-config
```

Tout Moonpool en cours d'exécution est arrêté lors de la désinstallation.

## Mode portable

Vous préférez une clé USB ou un dossier déplaçable ? Cliquez sur **Installer en portable**
sur la carte d'installation et choisissez un dossier. Voir [Mode portable](/fr/data/portable-mode/).

## Ensuite

- [Windows a protégé votre ordinateur](/fr/support/windows-protected-your-pc/) : si SmartScreen bloque l'installeur.
- [Runtime WebView2 manquant](/fr/support/webview2-runtime-missing/) : si la fenêtre reste vide.
- [Votre première app](/fr/getting-started/first-app/)
