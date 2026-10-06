---
title: "Essayez les tableaux de bord d'exemple fournis avec Moonpool"
description: "Ouvrez les tableaux de bord d'exemple hors ligne fournis, voyez où ils se trouvent et comment les apps les référencent, puis ajoutez-les à une configuration."
---

Moonpool fournit, à l'intérieur du programme, un ensemble de tableaux de bord autonomes. Ils
fonctionnent entièrement hors ligne, sans serveur ni CDN.

| Tableau de bord | Description |
| --- | --- |
| CSV explorer | Déposez un fichier CSV ou TSV : il analyse les colonnes et affiche les données. |
| JSON explorer | Déposez du JSON (tableaux, objets imbriqués ou dictionnaires). |
| Excel explorer | Déposez un fichier `.xlsx` ou `.xls`, analysé hors ligne. |
| Moonpool Docs | Un navigateur de documentation Markdown hors ligne. |

## Où ils se trouvent

Au démarrage, Moonpool écrit les tableaux de bord dans `{MP_HOME}\dashboards\examples` :

| Mode | Dossier |
| --- | --- |
| Installé (Windows) | `%USERPROFILE%\.moonpool\dashboards\examples` |
| Portable | `<your .moonpool folder, the one holding moonpool.exe>\dashboards\examples` |
| Linux | `~/.config/Moonpool/dashboards/examples` (ou `$XDG_CONFIG_HOME/Moonpool/dashboards/examples`) |

Le dossier `examples` appartient à Moonpool : il est remplacé à chaque mise à jour de
Moonpool, donc les modifications qu'on y fait sont perdues. Pour personnaliser un tableau de
bord, copiez son dossier et le dossier partagé `_lib` dans `dashboards`, puis faites pointer
votre app vers la copie. Moonpool ne modifie jamais rien d'autre dans `dashboards`.

Les versions antérieures à 0.3.16 écrivaient les exemples directement dans `dashboards`. Ces
copies restent où elles sont et ne reçoivent plus de mises à jour ; les apps qui pointent vers
elles continuent de fonctionner. Pour obtenir les versions à jour, changez leur `url` vers le
chemin `dashboards/examples/...` ci-dessous.

## Comment les apps les référencent

Chacun est une app `static` dont l'`url` est une URL `file:///` ancrée sur `{MP_HOME}` :

```text
file:///{MP_HOME}/dashboards/examples/csv/index.html
```

`{MP_HOME}` se résout vers le dossier d'installation ou, en mode portable, vers le dossier du
bundle, de sorte que l'entrée continue de fonctionner après le déplacement du bundle. Les URL
`file://` sont autorisées. Voir [Chemins et environnement](/fr/apps/paths-and-environment/).

## Les apps d'exemple n'apparaissent qu'au premier lancement

Les entrées d'exemple ne sont écrites dans `apps.json` que si aucun fichier de configuration
n'existe encore. Si vous avez déjà un `apps.json`, ajoutez vous-même les entrées des tableaux
de bord (**Modifier apps.json** dans le menu « ... », puis **Recharger**). Ajoutez ces quatre
entrées dans le tableau de premier niveau, séparées de vos autres entrées par des virgules :

```jsonc title="apps.json (excerpt)"
{
  "id": "csv-explorer",
  "name": "Sample CSV Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html",
  "openBrowser": true
},
{
  "id": "json-explorer",
  "name": "Sample JSON Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/json/index.html",
  "openBrowser": true
},
{
  "id": "xlsx-explorer",
  "name": "Sample Excel Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/xlsx/index.html",
  "openBrowser": true
},
{
  "id": "docs-browser",
  "name": "Moonpool Docs",
  "group": "Docs",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/docs/index.html",
  "openBrowser": true
}
```

La signification des champs est décrite dans [Champs d'une app](/fr/apps/fields/).

## Voir aussi

- [Exemples](/fr/apps/examples/) : des entrées plus complètes à copier.
- [Types d'app](/fr/apps/types/#static)
