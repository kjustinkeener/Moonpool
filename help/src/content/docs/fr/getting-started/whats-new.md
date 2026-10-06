---
title: "Notes de version de Moonpool et changements récents"
description: "Découvrez ce qui a changé dans les dernières versions de Moonpool, la configuration requise et où trouver les notes de version complètes sur GitHub."
---

Les notes complètes de chaque version se trouvent sur la
[page des versions](https://github.com/kjustinkeener/Moonpool/releases) du projet. Cette aide
est livrée dans Moonpool : elle décrit donc toujours la version que vous utilisez. Moonpool se
met à jour tout seul ; voir [Mises à jour](/fr/data/updating/).

## 0.3.16

- **Plusieurs Moonpool à la fois.** Le Moonpool installé et un nombre quelconque de copies
  portables peuvent s'exécuter côte à côte, une par dossier, chacune avec ses propres apps,
  son icône dans la zone de notification et son canal de contrôle. Voir
  [Mode portable](/fr/data/portable-mode/#plusieurs-copies-à-la-fois).
- **Navigateur de thèmes.** 68 thèmes, chacun prévisualisé dans ses propres couleurs. Voir
  [Thèmes, langue et transparence](/fr/using/themes-and-language/).
- **Exemples exécutables.** Un `apps.json` neuf contient des apps d'exemple qui fonctionnent
  toutes telles quelles. Les tableaux de bord d'exemple se trouvent désormais dans un dossier
  `dashboards/examples` géré par l'app, mis à jour avec Moonpool. Voir
  [Tableaux de bord d'exemple](/fr/getting-started/example-dashboards/).
- **Les erreurs d'apps.json sont affichées.** Un bandeau au-dessus de la barre latérale montre
  l'erreur, et un rechargement échoué conserve la dernière liste chargée. Voir
  [Quand apps.json contient une erreur](/fr/using/hub-window/#quand-appsjson-contient-une-erreur).
- **Canal de contrôle sous Linux et macOS**, via un socket Unix, plus le verbe `list`. Voir
  [Verbes de contrôle](/fr/automation/control-verbs/).
- La fenêtre À propos et l'éditeur d'app suivent en direct les changements de thème et de
  langue. L'élément de menu **Installer Moonpool…** est masqué hors Windows.

## 0.3.15

- Une app redémarrée conserve sa sortie précédente, avec un séparateur daté « restarted ». Voir
  [Onglets de terminal](/fr/using/terminal-tabs/#restart).
- Chaque app a son propre dossier `cli-output`, de sorte que l'élagage des journaux ne touche
  jamais ceux d'une autre app.
- `killMode` et `stopCommand` sont dans l'éditeur d'app. Voir
  [Arrêt et redémarrage](/fr/apps/stop-and-restart/).
- Les apps lancées n'héritent plus du profil WebView2 propre à Moonpool.

## 0.3.14

- Les journaux de session peuvent être conservés entre les sessions, avec un plafond de taille
  par app. Voir [Journaux](/fr/data/logs/).
- Boutons pour ouvrir et copier le chemin des dossiers de journaux dans les Paramètres.
- Corrections de la barre de titre de la fenêtre d'aide.

## Configuration requise

- Windows 10 ou 11 avec WebView2 (voir [Windows](/fr/platforms/windows/)).
- Linux avec WebKitGTK 4.1 et une bibliothèque AppIndicator (voir [Linux](/fr/platforms/linux/)).
- macOS : compilez depuis les sources ; pas encore distribué ni testé.
