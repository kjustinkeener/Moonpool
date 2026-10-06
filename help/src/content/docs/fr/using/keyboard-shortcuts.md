---
title: "Raccourcis clavier, raccourcis souris et zoom de Moonpool"
description: "Découvrez tous les raccourcis clavier et souris du hub de Moonpool, et comment zoomer l'interface avant et arrière pour que le texte soit confortable à lire."
---

## Clavier

| Touches | Où | Action |
| --- | --- | --- |
| F5, Ctrl+R, Cmd+R | Hub | Recharge `apps.json` depuis le disque, comme **Recharger** dans le menu. La page elle-même n'est pas actualisée. |
| Échap | Menu contextuel | Le ferme. |
| Échap | Renommage d'une app | Annule le renommage. |
| Échap | Fenêtres Paramètres, À propos et éditeur d'app | Ferme la fenêtre (l'éditeur demande avant d'abandonner les modifications). |
| Entrée | Renommage d'une app | Enregistre le nouveau nom. |

## Souris

| Action | Où | Résultat |
| --- | --- | --- |
| Ctrl + molette | Hub | Zoome l'interface. |
| Sélectionner du texte | Terminal | Copie et efface la sélection. |
| Clic central | Terminal | Colle. |
| Clic droit | Ligne de la barre latérale | Ouvre le menu de la ligne. Voir [Barre latérale et menus](/fr/using/sidebar-and-menus/). |

## Zoom

Maintenez Ctrl et faites tourner la molette sur le hub pour zoomer. Vers le haut, cela zoome
avant ; vers le bas, cela zoome arrière, par pas d'environ 10 pour cent par événement de molette.

```text
Ctrl + wheel up      zoom in
Ctrl + wheel down    zoom out
```

- La plage va de 0,5x à 3x.
- La fenêtre se redimensionne du même facteur, de sorte que la disposition reste aussi compacte à
  2x qu'à 1x. Une fois la limite atteinte, la fenêtre cesse de grandir.
- Le facteur est enregistré sous `uiScale` dans `settings.json` et appliqué au prochain démarrage.
  La taille de fenêtre enregistrée est déjà la taille zoomée : elle n'est donc pas mise à l'échelle
  une seconde fois. Voir [settings.json](/fr/data/settings-json/).

- Le zoom ne s'applique qu'à la fenêtre du hub. Les Paramètres, À propos, l'éditeur d'app et l'Aide
  gardent leur propre taille.

Il n'y a pas de commande de Paramètres pour `uiScale` ni de touche de réinitialisation. Pour revenir
à la taille normale, faites défiler la molette du même nombre de crans en sens inverse, ou quittez
Moonpool, définissez `uiScale` sur `1` dans `settings.json` (ou supprimez la clé), et redémarrez-le.

## Voir aussi

- [Thèmes, langue et transparence](/fr/using/themes-and-language/)
- [Barre latérale et menus](/fr/using/sidebar-and-menus/)
