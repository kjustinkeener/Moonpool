---
title: "Trouver et gérer les journaux de session et le journal de débogage de Moonpool"
description: "Repérez le journal de session de chaque app, le journal de débogage, les dumps et l'historique du terminal, leur durée de conservation et comment les copier."
---

Moonpool conserve quatre types de sortie :

| Type | Emplacement | Conservation |
| --- | --- | --- |
| Journal de session | `cli-output\<id>\<session-start-ms>.log` dans le dossier de configuration | Toujours pour cette session ; les anciennes sessions selon les règles de conservation ci-dessous |
| `moonpool.log` | Le dossier de configuration | Écrit uniquement tant que **Journaliser les infos de débogage dans un fichier** est activé |
| Dump | Là où vous le demandez, ou le chemin du journal de session lui-même | Jusqu'à ce que vous le supprimiez |
| Historique du terminal | Dans l'onglet du terminal | 10 000 lignes, jusqu'à ce que Moonpool se ferme |

Le dossier de configuration est indiqué dans [Où se trouve la configuration](/fr/apps/apps-json/#où-se-trouve-la-configuration).

## Journaux de session

Tout ce qu'une app affiche dans son terminal est aussi écrit dans un fichier journal :

```text
<config folder>\cli-output\<id>\<session-start-ms>.log
```

- Un fichier par app et par session de Moonpool. Le nombre correspond au moment où ce processus Moonpool a démarré.
- Arrêter puis relancer une app continue d'ajouter au même fichier. Une ligne de séparation atténuée marque
  le début de chaque nouvelle exécution, et le même repère s'affiche dans l'onglet du terminal :

  ```text title="1767225600000.log"
  Local:   http://localhost:5173/
  ---------- restarted 2026-10-05 09:14:02 ----------
  Local:   http://localhost:5173/
  ```

- Les caractères d'un `id` autres que les lettres, les chiffres, `-` et `_` deviennent `_` dans le nom du
  dossier. Ainsi `.` devient `_`. Les lettres non anglaises sont conservées.
- Le fichier contient la sortie brute du terminal, y compris les codes de couleur. Utilisez un dump pour du texte brut.

Rouvrir l'onglet d'une app rejoue le journal de cette session : vous voyez donc sa sortie précédente.

## Conservation

La conservation ne concerne que les journaux des sessions précédentes de Moonpool. Elle s'exécute quand vous lancez une app,
pour le dossier de cette app uniquement, en commençant par les plus anciens.

| **Conserver les journaux de sortie des apps entre les sessions** (`cliLogging`) | Ce qui arrive aux journaux des sessions précédentes |
| --- | --- |
| désactivé (par défaut) | Supprimés au prochain lancement de l'app. |
| activé | Conservés jusqu'à ce que la taille totale du dossier dépasse **Conservation des journaux par app** (`logRetentionMb`, 10 Mo par défaut), puis les plus anciens sont supprimés. |

Le fichier de la session en cours compte dans ce total, mais il n'est jamais supprimé ni tronqué.
Un seul journal courant très volumineux peut donc évincer tous les plus anciens.

![La section Journalisation des Paramètres : la case de conservation des journaux, la taille de conservation par app en Mo et la case du journal de débogage, chacune avec une ligne de chemin de dossier](../../../../assets/screenshots/settings-logging-section.png)

1. **Conserver les journaux de sortie des apps entre les sessions** correspond à `cliLogging`. **Conservation des journaux par app** juste en dessous correspond à `logRetentionMb`.

## moonpool.log

Lorsque **Journaliser les infos de débogage dans un fichier** (`debugLogging`) est activé, Moonpool ajoute des lignes horodatées à
`moonpool.log` dans le dossier de configuration : chargements de `apps.json`, lancements (avec la commande et le dossier),
commandes de contrôle et erreurs. Activez-le avant de reproduire un problème.

## Ouvrir et copier

Dans les [Paramètres](/fr/using/settings/#colonne-de-droite--journaux), sous chaque groupe de journaux :

- **Ouvrir le dossier des journaux CLI** et **Ouvrir le journal** ouvrent le dossier dans votre gestionnaire de fichiers.
- **Copier le chemin du dossier des journaux CLI** et **Copier le chemin du fichier journal** placent le chemin dans le presse-papiers.

Dans un onglet de terminal, **Tout copier** copie tout l'historique sous forme de texte.

## Dumps

Le verbe `dump` vous donne un journal de session depuis un script :

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" dump my-app C:\temp\my-app.log
```

Avec un chemin de sortie, il écrit une copie en texte brut, codes de couleur supprimés. Sans chemin, il indique
le chemin du journal de session lui-même. Un agent obtient le même texte, déjà nettoyé, avec
`moonpool_app_output`. Voir [Ligne de commande](/fr/automation/command-line/).
