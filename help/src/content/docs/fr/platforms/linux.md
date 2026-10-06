---
title: "Installer et utiliser Moonpool sous Linux"
description: "Installez Moonpool sous Linux, contournez la limite de la zone de notification de GNOME, et voyez comment fonctionnent les mises à jour et ce qui diffère de Windows."
---

Moonpool fonctionne sous Linux grâce à WebKitGTK. Il est développé principalement sous
Windows : Linux est donc pris en charge, mais moins éprouvé. Linux n'a ni carte d'installation
ni sélecteur de mode portable, et le menu « ... » n'a pas d'élément **Installer Moonpool…**.

## Installation

Téléchargez un paquet depuis la page des versions du projet.

| Paquet | Mises à jour |
| --- | --- |
| AppImage | Moonpool se met à jour tout seul |
| `.deb` | Votre gestionnaire de paquets |
| RPM (à installer avec l'outil RPM de votre distribution) | Votre gestionnaire de paquets |

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

Le `.deb` installe ses dépendances d'exécution. L'AppImage nécessite que les bibliothèques
WebKitGTK et AppIndicator soient présentes, par exemple sous Debian ou Ubuntu :

```bash frame="terminal"
sudo apt-get install -y libwebkit2gtk-4.1-0 libayatana-appindicator3-1
```

Sous Fedora ou Arch, utilisez les équivalents :

```bash title="Fedora" frame="terminal"
sudo dnf install webkit2gtk4.1 libayatana-appindicator-gtk3
```

```bash title="Arch" frame="terminal"
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator
```

## Zone de notification sous GNOME

GNOME standard n'affiche pas les icônes de la zone de notification : l'icône de Moonpool
n'apparaîtra donc pas tant que l'extension AppIndicator n'est pas installée et activée :

```bash frame="terminal"
sudo apt-get install -y gnome-shell-extension-appindicator
gnome-extensions enable ubuntu-appindicators@ubuntu.com
```

Fermez ensuite votre session et rouvrez-la. La fenêtre du hub et les terminaux intégrés
fonctionnent sans elle. KDE, Cinnamon, XFCE et MATE affichent la zone de notification d'office.

## Mises à jour

Seule l'AppImage se met à jour elle-même. Elle lit `linux-update.json` depuis les versions
GitHub, vérifie la signature minisign et remplace le fichier AppImage sur place : gardez-la
donc dans un dossier où vous pouvez écrire. Les installations `.deb` et RPM ne sont jamais
écrasées par Moonpool : la vérification de mise à jour peut quand même signaler une version
plus récente, mais l'installer depuis Moonpool échoue avec un message vous invitant à
utiliser votre gestionnaire de paquets. Voir [Mises à jour](/fr/data/updating/).

## Emplacement de la configuration

```text
~/.config/Moonpool/              (or $XDG_CONFIG_HOME/Moonpool/)
~/.config/Moonpool/apps.json
~/.config/Moonpool/dashboards/examples/   (example dashboards)
```

`apps.json` est initialisé à partir de l'exemple au premier lancement. Voir
[Vue d'ensemble de la configuration](/fr/apps/apps-json/).

## Différences avec Windows

- Les commandes de lancement s'exécutent via `$SHELL -c <command>` (`/bin/sh` si `SHELL` n'est pas défini) : utilisez donc une syntaxe que votre shell comprend.
- Arrêter termine le groupe de processus, puis effectue le nettoyage supplémentaire choisi par `killMode`. Libérer un port avec `killMode: "port"` utilise `lsof`, avec `fuser` en repli ; installez `lsof` si votre distribution ne le fournit pas. Voir [Arrêt et redémarrage](/fr/apps/stop-and-restart/).
- Le `processName` d'une app `desktop` doit faire 15 caractères ou moins. Linux tronque un nom de processus à 15 caractères : un nom plus long n'est donc jamais détecté comme en cours d'exécution et ne peut pas être arrêté par son nom. Les apps `web` sont identifiées par leur port et ne sont pas concernées.
- Les icônes sont trouvées à partir de `src-tauri/icons/`, `public/favicon.*`, `icon.png` d'une app, ou de son favicon en ligne. L'extraction d'une icône depuis un binaire n'existe que sous Windows.
- Les boutons d'ouverture ouvrent le dossier contenant le fichier au lieu de sélectionner celui-ci.
- Les fichiers de configuration s'ouvrent dans votre éditeur de texte par défaut (déterminé à partir de l'association `text/plain`).
- L'installeur Windows, les raccourcis et l'entrée Ajout/Suppression de programmes ne s'appliquent pas.

## Voir aussi

- [Windows](/fr/platforms/windows/#ce-qui-diffère-selon-la-plateforme) : un tableau des différences par plateforme.
- [Mises à jour](/fr/data/updating/#linux)
