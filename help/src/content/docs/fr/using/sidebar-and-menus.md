---
title: "Lire la barre latérale : voyants d'état, groupes, filtre et menu d'une ligne"
description: "Voyez ce que montre chaque ligne de la barre latérale, comment fonctionnent voyants et groupes, et comment filtrer, utiliser le menu contextuel et redimensionner."
---

La barre latérale liste chaque app d'`apps.json`, groupée selon le champ `group` de chaque app. Voir [Champs d'une app](/fr/apps/fields/).

## Lignes

Chaque ligne affiche un voyant d'état, l'icône de l'app (ou un glyphe de type s'il n'y a pas d'icône), le nom, le port s'il est défini (`:3000`) et des commandes.

| Voyant | Signification |
| --- | --- |
| Plein | en cours |
| Pulsant | démarrage... : Moonpool a démarré l'app mais elle n'est pas encore détectée active |
| Gris | arrêtée |

Survolez le voyant pour voir le mot.

![La barre latérale avec deux apps web en cours d'exécution encadrées : voyants allumés et boutons Arrêter](../../../../assets/screenshots/sidebar-running-narrow.png)

1. Deux apps en cours d'exécution. Leurs voyants sont allumés et Arrêter (le carré) remplace Lancer.

| Commande | Action |
| --- | --- |
| Crayon | Modifier l'app. |
| Redémarrer | Arrêter puis relancer. Sur une app arrêtée, la lance simplement. |
| Lancer (lecture) | Démarre l'app et ouvre son onglet de terminal. Affiché quand l'app est arrêtée. |
| Arrêter (carré) | Arrête l'app. Affiché pendant qu'elle est en cours d'exécution ou en démarrage. |

![Une ligne d'app en cours, agrandie : voyant d'état, icône de type, nom et port, puis boutons modifier, redémarrer et arrêter](../../../../assets/screenshots/sidebar-row-controls.png)

1. Voyant d'état (allumé pendant l'exécution).
2. Icône de type.
3. Modifier (crayon).
4. Redémarrer.
5. Arrêter (affiché à la place de Lancer pendant l'exécution).

Pendant qu'un lancement ou un arrêt est en cours, les commandes sont remplacées par un indicateur de chargement (`En cours...`).

Cliquer sur le **nom** d'une app ouvre ou active son onglet de terminal et ne lance jamais rien. Un onglet d'app arrêtée affiche le journal de cette session. Utilisez Lancer ou Redémarrer pour la démarrer. Une app `static` qui n'a qu'une `url` et pas de `command` n'a pas de terminal : Lancer ouvre l'URL dans votre navigateur.

### Infobulle

Survoler le nom affiche la `note` de l'app s'il y en a une, sinon son nom. Définissez `note` dans l'éditeur ou dans `apps.json`.

### Sous-ligne MCP

Quand un client IA a utilisé les outils MCP propres à une app, une sous-ligne atténuée
`Serveur MCP` apparaît sous l'app. Son voyant est allumé et l'infobulle indique « Client MCP
connecté » tant que le client est connecté. Un bouton d'arrêt met fin à ce processus.

La ligne trouve le processus par `processName` plus l'argument `mcp`, ou par le motif
`mcpProcessName` de l'app quand il est défini. Voir [champs](/fr/apps/fields/#mcpprocessname).

Masquez ces lignes avec **Afficher les processus MCP** dans les Paramètres. Voir
[Configuration MCP](/fr/automation/mcp-setup/#apps-qui-ont-leur-propre-serveur-mcp).

## Groupes

![Barre latérale au repos avec les cinq en-têtes de groupe encadrés, chacun avec son nombre d'apps à droite](../../../../assets/screenshots/sidebar-groups-narrow.png)

- Cliquez sur un titre de groupe pour le replier ou le déplier. Le nombre à côté est le nombre d'apps affichées. Les groupes repliés sont mémorisés.
- Dans un groupe, l'app démarrée le plus récemment est en haut. Les apps jamais démarrées gardent l'ordre de `apps.json`. Une app tout juste lancée brille et monte en haut.

## Zone de filtre

Saisissez dans **Filtrer les apps...** pour réduire la liste. Elle correspond au nom de l'app et au nom du groupe, sans tenir compte de la casse. Si rien ne correspond, la liste affiche :

```text
Aucune app ne correspond à « <text> ».
```

L'app affiche des guillemets typographiques autour du texte, ici et dans l'invite de suppression ci-dessous.

## Menu du clic droit

Faites un clic droit sur une ligne pour obtenir :

| Élément | Action |
| --- | --- |
| Modifier | Ouvre l'éditeur d'app. |
| Renommer | Transforme le nom en champ de saisie. **Entrée** ou un clic ailleurs enregistre, **Échap** annule. Un nom vide ou inchangé est ignoré. |
| Choisir une icône... | Choisissez un fichier image (png, jpg, jpeg, gif, svg, webp, ico) à utiliser comme icône. |
| Supprimer | Demande `Supprimer « <name> » ?` et retire l'entrée d'`apps.json`. Si Moonpool exécute l'app, elle est d'abord arrêtée. |

**Échap** ferme le menu sans agir.

## Redimensionnement

Faites glisser le séparateur entre la barre latérale et le panneau CLI. La largeur est limitée de 180 à 620 px (280 par défaut) et mémorisée. Le séparateur est verrouillé tant que le panneau CLI est réduit. Voir [Onglets de terminal](/fr/using/terminal-tabs/).
