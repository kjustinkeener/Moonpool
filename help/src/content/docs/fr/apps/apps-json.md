---
title: "Modifier apps.json : emplacement, rechargement et récupération"
description: "Trouvez le fichier apps.json lu par Moonpool pour chaque app gérée, modifiez-le dans l'éditeur d'app ou à la main, rechargez-le et récupérez après une erreur."
---

Chaque app gérée par Moonpool correspond à une entrée de `apps.json`. Vous pouvez la modifier depuis l'éditeur d'app
(la boîte de dialogue « Ajouter une app » et « Modifier l'app ») ou à la main. Les deux écrivent le même fichier. Certains résultats d'outils et
messages appellent ce fichier le manifeste.

## Où se trouve la configuration

| Mode | Dossier de configuration |
| --- | --- |
| Installé (Windows) | `%USERPROFILE%\.moonpool\moonpool-config\` |
| Portable | `moonpool-config\` à côté de `moonpool.exe` (dans le dossier `.moonpool\`) |
| Linux | `$XDG_CONFIG_HOME/Moonpool/`, sinon `~/.config/Moonpool/` |

`apps.json` se trouve dans ce dossier, à côté des éléments suivants :

| Élément | Rôle |
| --- | --- |
| `apps.json.history\` | Anneau de retour arrière des 10 derniers fichiers `apps.json` valides. |
| `settings.json` | Paramètres de l'app. Voir [settings.json](/fr/data/settings-json/). |
| `cli-output\<id>\` | Journaux de session par app. Voir [Journaux](/fr/data/logs/). |
| `moonpool.log` | Journal de débogage, tant que **Journaliser les infos de débogage dans un fichier** est activé. |
| `icons\` | Remplacements d'icône facultatifs `<id>.png` (également `.ico`, `.svg`, `.jpg`, `.jpeg`, `.webp`). |
| `state.json` | Instantané de l'état en direct, actualisé toutes les quelques secondes. |
| `dumps\` | Fichiers écrits par les verbes `dump`, `read-config` et `restore-config`. |
| `mcp_seen.json` | Les apps pour lesquelles un assistant MCP a été vu. |
| `window-state.json` | La taille et la position de la fenêtre du hub. |
| `AI-README.md` | Le guide pour les agents IA, réécrit à chaque lancement. |

Ce qu'il faut sauvegarder parmi ces éléments est décrit dans [Sauvegarde et récupération](/fr/data/backup-and-recovery/#le-dossier-de-configuration).

Au premier lancement, Moonpool crée `apps.json` avec des entrées d'exemple. Un fichier déjà existant
n'est jamais écrasé.

## Modification

- **Boîte de dialogue.** Utilisez **Ajouter une app** dans le menu **...** en haut de la barre latérale. Pour modifier une
  app, utilisez le crayon sur sa ligne ou faites un clic droit dessus et choisissez **Modifier**. La boîte de dialogue
  valide et enregistre immédiatement.
- **À la main.** **Modifier apps.json** dans le même menu ouvre le fichier dans votre éditeur par défaut.
  Enregistrez-le, puis choisissez **Recharger** dans le menu (ou appuyez sur F5 ou Ctrl+R).

Les modifications manuelles ne sont prises en compte qu'après un rechargement. Recharger ne fait que lire le fichier ; il ne
le réécrit pas.

Un enregistrement depuis la boîte de dialogue réécrit tout le fichier sous une forme normalisée et indentée. Les clés que Moonpool
ne connaît pas sont supprimées, et JSON n'accepte pas les commentaires : conservez vos notes dans le champ `note`.

## Structure

Le fichier est un tableau JSON d'objets. Quatre clés sont obligatoires dans chaque entrée : `id`, `name`,
`group`, `type`. Tout le reste est facultatif. Voir [Champs d'une app](/fr/apps/fields/).

```json title="apps.json"
[
  { "id": "site", "name": "Site", "group": "Web apps", "type": "web",
    "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
    "url": "http://localhost:5173", "openBrowser": true }
]
```

Les groupes apparaissent dans la barre latérale dans l'ordre de leur première occurrence dans le fichier.

## Ce que fait le rechargement

Recharger remplace la liste en mémoire de Moonpool par le contenu du fichier. Lancer, Arrêter et Redémarrer
lisent l'entrée au moment du clic : une `command`, un `cwd`, un `env` ou un réglage d'arrêt modifié
s'applique donc la prochaine fois que vous démarrez ou redémarrez cette app. Recharger ne redémarre jamais rien : une
app déjà en cours d'exécution continue de tourner avec les réglages avec lesquels elle a démarré.

## Validation

Moonpool valide le fichier entier à son chargement, à chaque enregistrement et à chaque écriture par un agent.
Une seule entrée incorrecte fait rejeter tout le fichier.

| Règle | L'erreur contient |
| --- | --- |
| JSON non valide, clé obligatoire manquante ou valeur du mauvais type | le message de l'analyseur JSON |
| `id` vide, commençant par `-`, ou contenant des caractères autres que lettres, chiffres, `.`, `_`, `-` | `invalid id` |
| Deux entrées partagent un `id` | `duplicate app id` |
| `name` est vide | `has an empty name` |
| `group` est vide | `has an empty group` |
| `type` n'est pas `desktop`, `web`, `static` ou `cli` | `unknown type` |
| `port` vaut `0` (un `port` supérieur à 65535 échoue à l'analyse) | `invalid port 0` |
| Entrée `static` sans `url` | `requires a url` |
| Tout autre type sans `command` | `requires a command` |

Les erreurs désignent l'entrée par sa position, par exemple :

```text
apps.json entry 2 (site) requires a command
```

### L'id

L'`id` est la clé permanente de l'entrée. Il nomme le dossier de journaux et le fichier d'icône, et c'est ce que
vous passez à `moonpool.exe launch <id>` et aux agents. La boîte de dialogue le dérive du nom
quand vous ajoutez une app. Elle met le nom en minuscules, transforme chaque suite
de caractères autres que `a` à `z` et `0` à `9` en un seul `-`, et supprime les `-` aux deux
extrémités. Un résultat vide devient `app`. Si l'id est déjà pris, elle ajoute `-2`, `-3`, etc. Elle
ne modifie jamais l'id par la suite : renommer une app conserve donc son id. Le nom `Habit Tracker` donne l'id
`habit-tracker`.

## Si le fichier est incorrect

- **Au rechargement**, un fichier qui échoue à la validation reste intact et Moonpool conserve la dernière
  liste chargée. Une bannière au-dessus de la barre latérale affiche l'erreur, avec un bouton pour ouvrir le
  fichier ; la liste reste utilisable mais atténuée. Voir
  [Quand apps.json contient une erreur](/fr/using/hub-window/#quand-appsjson-contient-une-erreur).
- **Au démarrage**, un fichier endommagé signifie qu'il n'y a aucune liste à conserver : Moonpool démarre donc sans
  aucune app et la bannière l'indique. Corrigez le fichier et choisissez **Recharger**, ou restaurez un instantané
  (ci-dessous, ou avec l'outil `moonpool_restore_config`).
- Dans les deux cas, les enregistrements depuis la boîte de dialogue (ainsi que renommer, supprimer, choisir une icône) sont refusés tant que le
  fichier n'est pas rechargé correctement, afin que le fichier endommagé ne soit jamais écrasé. Corrigez le fichier et choisissez
  **Recharger**.
- **Depuis la boîte de dialogue, un agent ou une restauration**, une modification non valide est rejetée et le fichier
  sur le disque reste tel qu'il était.

Moonpool conserve les 10 dernières bonnes versions de `apps.json` dans `apps.json.history\`. La façon de revenir en arrière
est décrite dans [Sauvegarde et récupération](/fr/data/backup-and-recovery/#revenir-en-arrière-sur-appsjson).
Les symptômes et les solutions se trouvent dans [Dépannage](/fr/support/troubleshooting/#appsjson-contient-une-erreur).

## Agents

Un agent IA doit modifier `apps.json` via les outils MCP de Moonpool plutôt que via le fichier, afin qu'une
écriture obsolète ou non valide soit rejetée et qu'un agent isolé dans un bac à sable ne modifie jamais une copie privée. Voir
[Outils MCP](/fr/automation/mcp-tools/#configuration).
