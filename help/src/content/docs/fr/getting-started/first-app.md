---
title: "Ajoutez et lancez votre première app dans Moonpool"
description: "Passez du premier lancement à une app à vous en cours d'exécution en quelques minutes : ajoutez-la, démarrez-la, arrêtez-la, et retrouvez le hub et l'aide."
---

## 1. Démarrer Moonpool

Sous Windows, exécutez `moonpool.exe` et cliquez sur **Installer Moonpool** (voir
[Installation](/fr/getting-started/install/)). Sous Linux, lancez l'AppImage ou le paquet
installé.

Au premier lancement, Moonpool remplit la barre latérale d'apps d'exemple (Bloc-notes sous
Windows, un shell, un petit serveur web et les tableaux de bord inclus). Elles fonctionnent
telles quelles (le serveur web nécessite Python) : essayez-les, puis modifiez-les ou
supprimez-les. Moonpool place aussi une icône dans la zone de notification. Sous Windows, si
vous ne voyez pas l'icône, cliquez sur la flèche **^** à droite de la barre des tâches.

## 2. Ajouter votre app

1. Ouvrez le menu **...** en haut de la barre latérale et choisissez **Ajouter une app**.
2. Saisissez un **name**. Le groupe est `Web apps` au départ ; gardez-le ou choisissez-en un autre.
3. Gardez **type** sur `web` pour un serveur de développement.
4. Définissez **cwd** sur le dossier de votre projet et **command** sur ce que vous tapez pour
   la démarrer, par exemple `npm run dev`.
5. Définissez **port** sur le port d'écoute de l'app, et **url** sur la page à ouvrir.
6. Enregistrez.

Le détail de chaque champ se trouve dans [Ajouter des apps](/fr/apps/add-an-app/).

## 3. La démarrer

Cliquez sur le bouton **Lancer** de l'app (l'icône de lecture sur sa ligne). Son onglet de
terminal s'ouvre et affiche la sortie. Le point d'état pulse pendant le démarrage de l'app,
puis devient plein dès que son port répond. Si **openBrowser** est activé, la page s'ouvre.

Cliquer sur le nom de l'app ouvre seulement son onglet de terminal. Cela ne démarre jamais l'app.

## 4. L'arrêter

Cliquez sur le bouton **Arrêter** (le carré) sur la ligne. Le point devient gris.

Si quelque chose reste en cours d'exécution après Arrêter, voir
[Arrêt et redémarrage](/fr/apps/stop-and-restart/).

## Laisser un agent s'en charger

Quand aucun onglet n'est ouvert, le panneau CLI affiche un bouton **Copier le prompt**. Collez
le prompt dans un agent IA : il trouve vos apps et les ajoute. Voir
[Agents IA : démarrage rapide](/fr/automation/quick-start/).

## Retrouver le hub plus tard

- Faites un clic gauche sur l'icône de la zone de notification pour afficher le hub. Un clic
  droit ouvre un menu avec **Afficher Moonpool** et **Quitter**.
- Par défaut, fermer la fenêtre quitte Moonpool. Activez **Fermer vers la zone de notification**
  dans les Paramètres pour la masquer dans la zone de notification et garder Moonpool en
  cours d'exécution. Voir
  [Zone de notification, fermeture et réduction](/fr/using/tray-and-closing/).

## Obtenir de l'aide

**Aide**, dans le menu **...** en haut de la barre latérale, ouvre cette aide dans sa propre
fenêtre. Elle fonctionne hors ligne et correspond toujours à la version que vous utilisez.

![Fenêtre d'aide avec la navigation par sections entourée à gauche et une page à droite](../../../../assets/screenshots/help-window.png)

## Ensuite

- [Ajouter des apps](/fr/apps/add-an-app/)
- [Dépannage](/fr/support/troubleshooting/)
