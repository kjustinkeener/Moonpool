---
title: "Garder Moonpool dans la zone de notification : comportement de fermeture, réduction et Quitter"
description: "Contrôlez ce que font l'icône de la zone de notification, la fermeture, la réduction et Quitter, gardez la fenêtre au premier plan et évitez de tout masquer."
---

## Icône de la zone de notification

| Action | Résultat |
| --- | --- |
| Clic gauche | Affiche la fenêtre du hub (la restaure si elle est réduite ou masquée). |
| Clic droit | Menu avec seulement **Afficher Moonpool** et **Quitter** (dans votre langue). |

Quand plusieurs copies de Moonpool s'exécutent, chacune a sa propre icône dans la zone de
notification. L'infobulle indique de quelle copie il s'agit. Voir
[Mode portable](/fr/data/portable-mode/#plusieurs-copies-à-la-fois).

## Quitter

**Quitter** ferme Moonpool et, sous Windows, arrête toutes les apps lancées par Moonpool, y
compris leurs processus enfants. Les apps qui étaient déjà en cours d'exécution avant que Moonpool
ne les voie (affichées comme en cours sans « managed by Moonpool ») ne sont pas touchées. Sous
Linux et macOS, quitter n'arrête pas de façon fiable les apps lancées.

## Fermer et réduire

Le bouton de fermeture quitte Moonpool par défaut (`closeToTray` vaut `false`). Activez **Fermer
vers la zone de notification** dans les Paramètres et fermer masque la fenêtre dans la zone de
notification à la place. Moonpool continue de s'exécuter, et l'icône de la zone de notification ou
**Afficher Moonpool** la ramène.

**Réduire vers la zone de notification** (`minimizeToTray`, activé par défaut) masque la fenêtre
dans la zone de notification quand elle est réduite, et la retire de la barre des tâches.
Désactivez-le pour réduire vers la barre des tâches comme d'habitude.

![Paramètres : Fermer vers la zone de notification et Réduire vers la zone de notification (1), et le curseur Transparence du fond (2)](../../../../assets/screenshots/settings-tray-and-transparency.png)

1. **Fermer vers la zone de notification** et **Réduire vers la zone de notification**.
2. **Transparence du fond**. Voir [Thèmes, langue et transparence](/fr/using/themes-and-language/#transparence).

## Verrouillage de la zone de notification et de la barre des tâches

**Afficher dans la zone de notification** et **Afficher dans la barre des tâches** contrôlent si
l'icône de la zone de notification et le bouton de la barre des tâches sont visibles. Au moins l'un
des deux doit rester activé, sinon une fenêtre masquée n'aurait aucun moyen de revenir. Quand un
seul est activé, sa case à cocher est désactivée jusqu'à ce que vous réactiviez l'autre.

## Toujours au premier plan

**Toujours au premier plan** dans les Paramètres garde toutes les fenêtres de Moonpool (le hub, les
Paramètres, À propos, l'éditeur d'app, le navigateur de thèmes, l'installateur et l'Aide) au-dessus
des autres fenêtres. Désactivé par défaut.

## Voir aussi

- [Exécuter un serveur de développement npm en arrière-plan sous Windows](/fr/guides/run-npm-dev-server-in-background-windows/)
- [Lancer automatiquement un script ou un serveur de développement à l'ouverture de session Windows](/fr/guides/start-app-at-windows-login/)
- [Fenêtre des paramètres](/fr/using/settings/)
- [La fenêtre du hub](/fr/using/hub-window/)
