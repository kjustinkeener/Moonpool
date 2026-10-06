---
title: "Ajouter une app ou un serveur de développement à Moonpool"
description: "Enregistrez une app locale ou un serveur de développement avec sa commande de lancement, son dossier de travail et son environnement, pour que Moonpool la gère."
---

Chaque app de Moonpool est une entrée composée d'une commande de lancement, d'un dossier de
travail et, en option, d'un environnement. Moonpool exécute la commande dans son propre
terminal géré.

## Ajouter une app

1. Ouvrez le menu **...** en haut de la barre latérale et choisissez **Ajouter une app**.
2. Saisissez un **name** et choisissez un **group**.
3. Choisissez le **type** : `web` (serveur sur un port), `desktop` (app native), `static` (une page) ou `cli` (une commande).
4. Définissez la **command** et le **cwd** dans lequel elle s'exécute.
5. Renseignez ce dont le type a besoin : **port** et **url** pour web, **processName** pour desktop,
   **url** pour static. Une app `static` avec seulement une `url` n'a besoin ni de **command** ni de **cwd**.
6. Enregistrez. L'app apparaît dans la barre latérale. Utilisez son contrôle **Lancer** pour la démarrer.

![La liste déroulante type (1) et le champ port (2) dans l'éditeur d'app, avec cwd et command entre les deux](../../../../assets/screenshots/edit-app-type-and-port.png)

1. La liste déroulante **type** ; son indication explique comment ce type s'exécute.
2. Le champ **port**, utilisé par les apps `web`.

Le résultat est une entrée dans `apps.json`, par exemple :

```json title="apps.json"
{
  "id": "my-api",
  "name": "My API",
  "group": "Dev",
  "type": "web",
  "command": "npm run dev",
  "cwd": "C:\\code\\my-api",
  "port": 3000,
  "url": "http://localhost:3000"
}
```

Un clic sur le nom d'une app ouvre seulement son onglet de terminal ; voir [États d'une app](/fr/support/glossary/#états-dune-app).

## L'éditeur d'app

- **Groupe.** Choisissez un groupe dans la liste, ou choisissez **+ Nouveau groupe...** et saisissez un nom.
  **retour à la liste** revient à la liste. Un groupe vide est enregistré sous le nom `Apps`.
- Les **champs grisés** ne sont pas utilisés par le type sélectionné. Ils sont tout de même enregistrés.
- **Enregistrer sans nom** affiche `le nom est obligatoire.`
- **Échap** ou la fermeture de l'éditeur avec des modifications non enregistrées affiche « Abandonner vos modifications ? ».
- Pour modifier une app plus tard, utilisez le crayon sur sa ligne, ou faites un clic droit dessus et choisissez **Modifier**.

## Modifier à la main

Choisissez **Modifier apps.json** dans le même menu, enregistrez le fichier, puis choisissez **Recharger**. Le
format, les règles de validation et les options de récupération se trouvent dans la
[Vue d'ensemble de la configuration](/fr/apps/apps-json/).

## Pour aller plus loin

- [Champs d'une app](/fr/apps/fields/) : chaque clé et son rôle.
- [Types d'app](/fr/apps/types/) : comment chaque type se lance et affiche « en cours ».
- [Arrêter et redémarrer](/fr/apps/stop-and-restart/) : que régler quand Arrêter laisse quelque chose en cours d'exécution, et pourquoi les apps Docker demandent de l'attention.
- [Chemins et environnement](/fr/apps/paths-and-environment/) : `{MP_HOME}`, chemins `./` et `env`.
- [Exemples](/fr/apps/examples/) : des entrées complètes à copier.
- [Guides pratiques](/fr/guides/run-npm-dev-server-in-background-windows/) : serveurs de développement en arrière-plan, scripts Python, ports.
- [Mode portable](/fr/data/portable-mode/)
- [Mises à jour](/fr/data/updating/)
