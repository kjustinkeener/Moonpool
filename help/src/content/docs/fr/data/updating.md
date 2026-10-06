---
title: "Mettre à jour Moonpool et résoudre un échec de mise à jour"
description: "Voyez comment Moonpool recherche, télécharge et applique les mises à jour, ce que fait la bannière, les copies portables et Linux, et que faire en cas d'échec."
---

Moonpool se met à jour lui-même. Il n'y a pas d'installeur séparé à télécharger ni d'assistant à parcourir.

## Comment arrivent les mises à jour

Moonpool récupère `update.json` (`linux-update.json` sous Linux) depuis les GitHub Releases du projet,
compare les versions et ne propose qu'une version strictement plus récente. Il recherche :

- au démarrage, sauf si **Rechercher des mises à jour au démarrage** est désactivé dans les
  [Paramètres](/fr/using/settings/) ;
- chaque fois que vous appuyez sur **Rechercher des mises à jour** dans la fenêtre À propos. Ce bouton installe immédiatement une version plus récente
  et redémarre Moonpool. Sinon, il indique que vous avez la dernière
  version, ou affiche l'erreur.

La fenêtre À propos affiche la version que vous exécutez, sous le nom :

![Le haut de la fenêtre À propos : le logo, le nom (1) et la ligne de version en dessous](../../../../assets/screenshots/about-header.png)

1. Le nom. La ligne en dessous est la version et la date de compilation.

Chaque téléchargement est vérifié avec la clé de signature minisign de Moonpool avant d'être appliqué, de sorte
qu'un téléchargement altéré ou corrompu est rejeté. Moonpool n'installe jamais une version plus ancienne.

## La bannière de mise à jour

Au démarrage, une mise à jour trouvée s'affiche sous forme de bannière sur l'écran vide du hub :

```text
Moonpool {version} est disponible (vous avez la {current}).
```

La bannière ne s'affiche que lorsqu'aucun onglet d'app n'est ouvert et que le volet CLI est développé. Lorsque le volet est
réduit, le chevron à côté de la zone de filtre pulse à la place. Lorsqu'un onglet est ouvert, il n'y a aucun
signe. Pour voir la bannière, fermez tous les onglets (et développez le volet), ou utilisez **Rechercher des mises à jour**
dans la fenêtre À propos.

Cliquez sur **Télécharger et installer** et Moonpool se remplace lui-même et se relance, ou fermez la
bannière avec le x.

## Copies portables

Une copie portable met à jour le `moonpool.exe` de son propre dossier `.moonpool\`, de la même façon.
Chaque copie recherche et se met à jour séparément. Le dossier doit être accessible en écriture : une copie sur une
clé ou un partage en lecture seule ne peut donc pas se mettre à jour elle-même ; copiez à la main un `moonpool.exe` plus récent par-dessus.

## Linux

Seul l'AppImage se met à jour lui-même. Il remplace le fichier AppImage sur place : gardez-le donc dans un
dossier accessible en écriture. Une installation `.deb` ou RPM est mise à jour par votre gestionnaire de paquets :
l'installation depuis Moonpool échoue avec

```text
automatic updates are available for the AppImage only; update the .deb or RPM with your package manager
```

Voir [Linux](/fr/platforms/linux/#mises-à-jour).

## Quand une mise à jour échoue

La bannière affiche la raison et le bouton redevient disponible pour que vous puissiez réessayer :

```text
Échec de la mise à jour : <error>
```

| L'erreur contient | Cause probable | Que faire |
| --- | --- | --- |
| `download failed` | Pas de connexion, un proxy, ou GitHub qui limite les requêtes | Patientez et réessayez, ou mettez à jour à la main. |
| `signature verification FAILED - refusing to install` | Le téléchargement est corrompu ou a été modifié | Réessayez. Si l'échec persiste, mettez à jour à la main depuis la page Releases. |
| `rename self aside` ou `write new exe` | Le dossier est en lecture seule, ou un antivirus retient le fichier | Rendez le dossier accessible en écriture, ou autorisez `moonpool.exe` dans votre antivirus, puis réessayez. |
| `refusing to install ... not newer than current` | La version proposée n'est pas plus récente | Rien à faire. |

### Mettre à jour à la main

Quittez Moonpool, téléchargez `moonpool.exe` depuis la
[page Releases](https://github.com/kjustinkeener/Moonpool/releases) du projet et copiez-le par-dessus
l'ancien : `%USERPROFILE%\.moonpool\moonpool.exe` pour la copie installée, ou celui de votre dossier `.moonpool\`
pour une copie portable. Votre dossier de configuration n'est pas touché. Sous Linux, remplacez
l'AppImage, ou utilisez votre gestionnaire de paquets.

## L'aide aussi est mise à jour

Cette aide est livrée dans Moonpool : chaque mise à jour du programme apporte donc l'aide correspondante.
La copie hors ligne correspond toujours à la version que vous exécutez.

## Voir aussi

- [Nouveautés](/fr/getting-started/whats-new/)
- [Fenêtre Paramètres](/fr/using/settings/)
