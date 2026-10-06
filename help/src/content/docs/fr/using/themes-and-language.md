---
title: "Changer le thème, la langue et la transparence de Moonpool"
description: "Choisissez un thème de couleurs et la langue de l'interface, réglez la transparence du fond et l'échelle, et voyez-les s'appliquer à toutes les fenêtres ouvertes."
---

Le thème, la langue et la transparence se règlent dans la [fenêtre des Paramètres](/fr/using/settings/).
Les trois s'appliquent instantanément à toutes les fenêtres de Moonpool ouvertes.

![Sélecteurs de Langue (1) et de Thème (2) en haut des Paramètres](../../../../assets/screenshots/settings-language-theme.png)

1. Sélecteur de langue.
2. Bouton de thème. Il affiche le nom du thème actuel et ouvre le navigateur de thèmes.

## Thèmes

Le navigateur de thèmes est une fenêtre à part. Il contient une carte d'aperçu par thème, chacune
dessinée dans les couleurs propres au thème (texte, panneau, champ, bouton, voyants d'état,
dégradé de la jauge et jeu de 16 couleurs du terminal), regroupés en Essentiels, Néon, Chauds,
Froids, Verts, Neutres, Clairs, Rosés, Vifs, Pastel clair et Pastel. Cliquez sur une carte pour
l'appliquer : toutes les fenêtres de Moonpool ouvertes changent en même temps et le choix est
enregistré. La fenêtre reste ouverte pour que vous puissiez comparer ; appuyez sur Échap pour la
fermer.

Il y a 68 thèmes plus Automatique, et environ la moitié sont clairs. Les noms de thèmes sont des
noms propres et ne sont pas traduits ; seuls Automatique (système), Sombre et Clair le sont.

**Automatique (système)** suit la préférence claire ou sombre du système d'exploitation et change
en direct quand celui-ci change. Tout autre choix est fixe. Les 16 couleurs ANSI du terminal
suivent aussi le thème.

Si vous aviez enregistré un thème dans une version antérieure, il est conservé. Un nom enregistré
que Moonpool ne connaît plus revient à Automatique. Certains libellés diffèrent de ceux d'avant
(par exemple Matrix s'appelle désormais Terminal, Nord s'appelle Arctic, Dracula s'appelle
Nocturne, Gruvbox s'appelle Retro et Solarized s'appelle Solar) ; le choix enregistré lui-même est
inchangé.

Le thème est stocké dans le `localStorage` de la webview, pas dans `settings.json`. Si le stockage
est indisponible, il revient à Automatique.

```text
localStorage key: moonpool.theme
```

## Langues

Automatique suit la langue du système d'exploitation. Sinon, choisissez l'une des 14, chacune
affichée dans sa propre langue :

```text
English, Deutsch, Español, Français, Italiano, 日本語, 한국어, Nederlands, Polski,
Português (Brasil), Русский, Türkçe, 简体中文, 繁體中文
```

Le sélecteur s'applique instantanément au hub et aux autres fenêtres. Le choix est enregistré sous
`locale` dans `settings.json`.

## Transparence

**Transparence du fond** rend le fond de la fenêtre translucide, de 0 % (opaque) à 90 %.

- Survoler une fenêtre avec le pointeur la rend immédiatement totalement opaque. Quand le pointeur la quitte, elle revient à votre réglage en fondu en environ 2 secondes.
- Les terminaux suivent la même teinte au lieu d'ajouter la leur.
- Chaque fenêtre (hub, Paramètres, À propos, éditeur d'app et navigateur de thèmes) applique le réglage elle-même, et les Paramètres mettent à jour les autres en direct pendant que vous faites glisser le curseur.

## Échelle de l'interface

Zoomez toute l'interface avec Ctrl + molette de la souris. Il n'y a pas de zoom au clavier. Voir
[Raccourcis et zoom](/fr/using/keyboard-shortcuts/).

## Voir aussi

- [Fenêtre des paramètres](/fr/using/settings/)
- [Raccourcis et zoom](/fr/using/keyboard-shortcuts/)
