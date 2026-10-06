---
title: "Se repérer dans la fenêtre du hub de Moonpool"
description: "Visite du hub de Moonpool : barre latérale, onglets de terminal, barre d'état, menu, bandeau d'erreur d'apps.json, et mémorisation de la taille et de la position."
---

![Le hub avec trois apps en cours d'exécution : deux lignes d'apps web en cours (1), la bande d'onglets (2), la sortie en direct de l'app active (3) et la barre d'état (4)](../../../../assets/screenshots/hub-window.png)

1. Deux des apps en cours d'exécution : un voyant d'état allumé et un bouton d'arrêt au lieu de la lecture.
2. La bande d'onglets, un onglet par app ouverte, avec l'onglet actif mis en évidence.
3. La sortie en direct de l'app active.
4. La barre d'état CPU et mémoire.

## Disposition

| Zone | Ce qu'elle contient |
| --- | --- |
| Barre de titre | Réduire, agrandir et fermer. |
| Barre latérale | La zone de filtre, le menu **...** et vos apps groupées par `group`. Voir [Barre latérale et menus](/fr/using/sidebar-and-menus/). |
| Panneau CLI | Un onglet de terminal par app ouverte. Voir [Onglets de terminal](/fr/using/terminal-tabs/). |
| Barre d'état | CPU et mémoire en direct, en bas. |

Faites glisser le séparateur entre la barre latérale et le panneau CLI pour redimensionner la barre latérale.

## Barre d'état

![La barre d'état : barres CPU par cœur à gauche, barre de mémoire à droite](../../../../assets/screenshots/status-bar.png)

La barre d'état affiche une fine barre par cœur de CPU (survolez-la pour voir « Utilisation du CPU par cœur »), puis une
barre de mémoire avec une étiquette `used/total Go`. Désactivez-la avec **Afficher la barre d'état CPU/mémoire** dans les Paramètres (`showStatusbar` ; voir [Fenêtre des paramètres](/fr/using/settings/)). Le changement s'applique immédiatement.

## Le menu ...

Le bouton **...** à gauche de la zone de filtre ouvre le menu.

![Le bouton de menu ... (1) et la zone Filtrer les apps (2) en haut de la barre latérale](../../../../assets/screenshots/sidebar-filter-and-menu.png)

1. Le bouton de menu **...**.
2. La zone **Filtrer les apps...**.

| Élément | Action |
| --- | --- |
| Ajouter une app | Ouvre l'éditeur d'app. Voir [Ajouter des apps](/fr/apps/add-an-app/). |
| Modifier apps.json | Ouvre `apps.json` dans votre éditeur par défaut pour une modification manuelle. |
| Recharger | Relit `apps.json` depuis le disque (aussi F5, voir [Raccourcis et zoom](/fr/using/keyboard-shortcuts/)). |
| Paramètres | Ouvre la fenêtre des Paramètres. |
| Aide | Ouvre cette aide. |
| À propos | Ouvre la fenêtre À propos, avec la version et la recherche de mises à jour. |
| Installer Moonpool... | Windows uniquement. Ouvre la fenêtre d'installation, pour installer l'app ou créer une copie portable. Voir [Installation](/fr/getting-started/install/) et [Mode portable](/fr/data/portable-mode/). |

### Avertissement de conflit de port

Si deux apps de `apps.json` utilisent le même `port`, une ligne d'avertissement apparaît en bas du menu, par exemple :

```text
port 3000 : App A / App B
```

Survolez-la pour voir la phrase complète. Corrigez le conflit dans `apps.json` ou dans l'éditeur d'app ; la ligne disparaît dès qu'aucun port n'est partagé.

## Quand apps.json contient une erreur

Si Recharger (ou F5) constate qu'`apps.json` ne s'analyse plus ou ne passe plus la validation,
Moonpool conserve la liste qu'il avait déjà. Un bandeau en haut de la barre latérale indique
« apps.json contient une erreur ; affichage de la dernière liste chargée. », suivi de l'erreur
(survolez-la pour voir le texte complet). La liste en dessous est atténuée mais reste utilisable :
vous pouvez donc démarrer et arrêter des apps comme d'habitude. **Modifier apps.json** dans le
bandeau ouvre le fichier ; corrigez-le et choisissez **Recharger**, et le bandeau disparaît.

Tant que le fichier ne se charge pas de nouveau, Moonpool n'enregistre pas les modifications
venant de l'éditeur d'app, du renommage, de la suppression ou du choix d'icône : un mauvais
fichier n'est donc jamais écrasé.

Si le fichier est déjà cassé au démarrage de Moonpool, il n'y a aucune liste antérieure à
conserver : le bandeau indique qu'aucune app n'est chargée et la barre latérale est vide. Corrigez
le fichier et rechargez, ou revenez à une bonne copie récente (voir
[Si le fichier est mauvais](/fr/apps/apps-json/#si-le-fichier-est-incorrect)).

## Écran vide

Quand aucun onglet n'est ouvert, le panneau CLI affiche « Choisissez une app à gauche pour la
lancer. » Il contient aussi deux éléments qui n'apparaissent que lorsqu'aucun onglet n'est
ouvert :

- **Le bandeau de mise à jour**, quand une version plus récente a été trouvée au démarrage. Voir
  [Mises à jour](/fr/data/updating/).
- **Copier le prompt**, un prompt prêt à l'emploi qui confie la configuration de vos apps à un
  agent IA. Voir [Agents IA : démarrage rapide](/fr/automation/quick-start/#copier-le-prompt).

L'icône de la zone de notification, la fermeture, la réduction, Quitter et le premier plan
permanent sont décrits dans
[Zone de notification, fermeture et réduction](/fr/using/tray-and-closing/).

## Taille, position et état agrandi

Moonpool mémorise la taille, la position et l'état agrandi de la fenêtre du hub d'une exécution à
l'autre. Au premier lancement, elle s'ouvre en 1200x780, à la position choisie par Windows.

Si la position enregistrée ne se trouve plus sur aucun écran connecté (par exemple un moniteur
débranché), la position est ignorée et la taille enregistrée est utilisée à l'emplacement par
défaut. Le fichier est `window-state.json` dans le dossier de configuration (voir
[Où se trouve la configuration](/fr/apps/apps-json/#où-se-trouve-la-configuration)).

La largeur de la barre latérale et le fait que le panneau CLI soit réduit sont aussi mémorisés.
