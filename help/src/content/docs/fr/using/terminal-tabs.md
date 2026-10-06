---
title: "Utiliser les onglets de terminal de Moonpool : ouvrir, fermer, copier, redémarrer"
description: "Travaillez avec les onglets de terminal par app du hub : ouvrir et fermer, replier le panneau, copier et coller, redémarrer une session et trouver les journaux."
---

Chaque app s'exécute dans son propre onglet de terminal dans le panneau CLI.

## Onglets

![Bande d'onglets avec Metrics Dashboard actif (encadré) et son journal en direct dessous ; chaque onglet a un voyant et un x](../../../../assets/screenshots/hub-terminal-tab.png)

- Lancer une app, ou cliquer sur son nom dans la barre latérale, ouvre son onglet. Cliquer sur un nom ne démarre rien ; voir [États d'une app](/fr/support/glossary/#états-dune-app).
- Un voyant sur l'onglet est allumé pendant que l'app s'exécute.
- Le **x** d'un onglet ferme l'onglet. Il n'arrête pas l'app. Cliquez de nouveau sur le nom pour rouvrir l'onglet ; il affiche le journal de cette session.

## Replier le panneau

Le **x** tout à droite de la bande d'onglets (« Masquer le panneau CLI ») replie le panneau CLI et
réduit la fenêtre à la seule barre latérale. Les terminaux continuent de s'exécuter et gardent leur
historique de défilement.

Un chevron apparaît à côté de la zone de filtre pour rétablir le panneau à sa largeur précédente.
Le chevron pulse quand une mise à jour est en attente, car le bandeau de mise à jour se trouve dans
le panneau.

## Copier et coller

| Action | Résultat |
| --- | --- |
| Sélectionner du texte à la souris | Copié dans le presse-papiers au relâchement, puis la sélection est effacée. |
| Clic central | Colle le presse-papiers dans le terminal. |
| Bouton **Tout copier** (en haut à droite, apparaît au survol) | Copie tout l'historique de défilement sous forme de texte. |

## Historique de défilement

Chaque terminal conserve 10 000 lignes.

## Quand un processus se termine

Quand le processus se termine, le terminal affiche :

```text
[process exited]
```

L'onglet reste ouvert avec sa sortie intacte. La ligne `[process exited]` s'affiche dans votre
langue (en français : `[processus terminé]`).

## Restart

**Redémarrer** (ou Lancer sur une app arrêtée) démarre une nouvelle exécution dans le même onglet.
L'onglet est reconstruit, et la sortie précédente de cette session y est rejouée depuis le journal
de session.

Si l'app s'est déjà exécutée plus tôt dans cette session, Moonpool écrit d'abord un séparateur
atténué dans le journal de session, de sorte qu'il apparaisse entre l'ancienne sortie et la
nouvelle exécution :

```text
---------- restarted 2026-10-05 09:14:02 ----------
```

Si la nouvelle exécution commence par effacer l'écran, la sortie précédente est repoussée dans
l'historique de défilement au lieu d'être effacée.

## Journaux de session

Tout ce qu'une app affiche est aussi écrit dans un fichier journal sous `cli-output\`, un fichier par app et par session de Moonpool. L'emplacement, la conservation et le réglage **Conserver les journaux de sortie des apps entre les sessions** sont décrits dans [Journaux](/fr/data/logs/).
