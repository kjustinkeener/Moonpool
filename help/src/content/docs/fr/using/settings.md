---
title: "Modifier les paramètres de Moonpool : toutes les options de Paramètres et À propos"
description: "La liste complète des commandes des fenêtres Paramètres et À propos de Moonpool, la clé de settings.json que chacune écrit et comment réinitialiser un paramètre."
---

Ouvrez **Paramètres** depuis le menu « ... » du hub. Les modifications s'enregistrent au fur et à
mesure. Échap ferme la fenêtre. Cette page est la liste complète des paramètres. Chacun est
enregistré dans `settings.json` sous la clé indiquée ; le fichier lui-même est décrit dans
[settings.json](/fr/data/settings-json/).

![Fenêtre des Paramètres : bascules et curseurs dans la colonne de gauche, options des journaux dans celle de droite](../../../../assets/screenshots/settings-window.png)

## Réinitialiser une commande

Faites un clic droit sur une case à cocher, un curseur ou un champ numérique pour ne réinitialiser
que ce paramètre à sa valeur par défaut. L'infobulle de chaque commande l'indique. Les sélecteurs
de Langue et de Thème n'ont pas de réinitialisation.

## Colonne de gauche

| Commande | Clé | Défaut | Rôle |
| --- | --- | --- | --- |
| Langue | `locale` | Automatique (système) | Langue du texte propre à Moonpool. S'applique instantanément. Voir [Thèmes, langue et transparence](/fr/using/themes-and-language/). |
| Thème | aucune (stockage du navigateur) | Automatique (système) | Thème de couleurs. Le bouton ouvre un navigateur de thèmes avec un aperçu de chacun ; un clic en applique un instantanément. Voir [Thèmes, langue et transparence](/fr/using/themes-and-language/). |
| Fermer vers la zone de notification | `closeToTray` | désactivé | Activé : fermer la fenêtre masque Moonpool dans la zone de notification. Désactivé : fermer quitte. |
| Réduire vers la zone de notification | `minimizeToTray` | activé | Activé : réduire masque Moonpool dans la zone de notification et le retire de la barre des tâches. Désactivé : réduit vers la barre des tâches. |
| Toujours au premier plan | `alwaysOnTop` | désactivé | Garde toutes les fenêtres de Moonpool au-dessus des autres fenêtres. |
| Afficher dans la zone de notification | `showInTray` | activé | Garde l'icône de la zone de notification visible. |
| Afficher dans la barre des tâches | `showInTaskbar` | activé | Garde le bouton de la barre des tâches visible. |
| Afficher la barre d'état CPU/mémoire | `showStatusbar` | activé | Barre CPU et mémoire en direct en bas du hub. |
| Afficher les processus MCP | `showMcpProcesses` | activé | Affiche le processus MCP d'une app comme sous-ligne MCP dans la barre latérale tant que ses outils MCP sont utilisés. |
| Transparence du fond | `transparency` | 0 % | Curseur de 0 à 90 par pas de 5. Voir [Thèmes, langue et transparence](/fr/using/themes-and-language/#transparence). |
| Rechercher des mises à jour au démarrage | `checkOnStartup` | activé | Interroge GitHub au lancement pour une version plus récente et affiche un bandeau s'il en trouve une. Voir [Mises à jour](/fr/data/updating/). |

### Verrouillage de la zone de notification et de la barre des tâches

Au moins l'une des options **Afficher dans la zone de notification** et **Afficher dans la barre
des tâches** doit rester activée, sinon une fenêtre masquée n'aurait aucun moyen de revenir. Quand
une seule est activée, sa case à cocher est désactivée jusqu'à ce que vous réactiviez l'autre.

## Colonne de droite : journaux

| Commande | Clé | Défaut | Rôle |
| --- | --- | --- | --- |
| Conserver les journaux de sortie des apps entre les sessions | `cliLogging` | désactivé | La sortie du terminal de la session en cours est toujours conservée pour ses propres onglets. Activé : les journaux des sessions précédentes restent sur le disque sous `cli-output\`, plafonnés par le réglage de conservation. Désactivé : ils sont supprimés au prochain lancement de cette app. |
| Conservation des journaux par app | `logRetentionMb` | 10 Mo | Plafond des journaux cumulés de chaque app. Minimum 1. Désactivé tant que la bascule ci-dessus est désactivée. Le journal de la session en cours compte dans le plafond mais n'est jamais tronqué ni supprimé par lui. |
| Journaliser les infos de débogage dans un fichier | `debugLogging` | désactivé | Enregistre les chargements d'`apps.json`, les lancements et les erreurs dans `moonpool.log`. |

Sous chaque groupe de journaux, un champ de chemin indique l'emplacement, avec deux boutons :

- Le bouton d'ouverture ouvre le dossier dans le gestionnaire de fichiers (**Ouvrir le dossier des journaux CLI** pour `cli-output\`, **Ouvrir le journal** pour `moonpool.log`).
- Le bouton de copie place le chemin dans le presse-papiers (**Copier le chemin du dossier des journaux CLI**, **Copier le chemin du fichier journal**).

Les formats des fichiers journaux, le séparateur de redémarrage et les règles de conservation sont
décrits dans [Journaux](/fr/data/logs/).

Si une case à cocher ne peut pas être enregistrée, un message rouge en haut de la fenêtre l'indique
et la case revient à son état précédent.

## Fenêtre À propos

Ouvrez **À propos** depuis le menu « ... ».

![Fenêtre À propos avec la ligne de version, les liens et les boutons Rechercher des mises à jour et Fermer](../../../../assets/screenshots/about-window.png)

Elle affiche :

- La version et la date de compilation.
- Des liens vers le site du projet, le dépôt GitHub et l'adresse de contact.
- **Rechercher des mises à jour**. Si une version plus récente existe, elle la télécharge, la vérifie et l'installe, puis redémarre Moonpool. Sinon, elle indique que vous avez la dernière version, ou l'erreur si la recherche a échoué.
- Les crédits des bibliothèques avec lesquelles Moonpool est construit, et l'auteur.

Échap la ferme. À propos suit en direct les réglages de thème, de transparence et de langue.
