---
title: "Dépanner Moonpool : zone de notification, apps qui ne démarrent pas, mises à jour"
description: "Résolvez les problèmes courants de Moonpool selon le symptôme : icône absente, apps qui ne démarrent pas, voyants erronés, mises à jour en échec, erreurs MCP."
---

Repérez le symptôme, puis suivez la solution. Le texte entre guillemets est ce que Moonpool
affiche. Pour chercher un message exact, voir
[Messages d'erreur expliqués](/fr/support/error-messages/).

## Je ne vois pas l'icône de la zone de notification

- **Windows.** L'icône est peut-être dans la zone des icônes masquées. Cliquez sur la flèche **^**
  à droite de la barre des tâches. Faites glisser l'icône sur la barre des tâches pour la garder
  visible.
- **Linux sur GNOME standard.** GNOME n'affiche pas d'icônes de zone de notification sans
  l'extension AppIndicator. Voir [Linux](/fr/platforms/linux/#zone-de-notification-sous-gnome).
- **Paramètres.** **Afficher dans la zone de notification** est peut-être désactivé. Ouvrez le hub
  depuis la barre des tâches ou le menu Démarrer et réactivez-le dans les
  [Paramètres](/fr/using/settings/).

## L'installateur affiche une erreur

| Message | Que faire |
| --- | --- |
| `Échec de l'installation : <error>` | Le texte après les deux-points nomme l'étape qui a échoué, par exemple `copy exe: ...`. Si un fichier est utilisé, quittez tout Moonpool exécuté depuis `%USERPROFILE%\.moonpool` et réessayez. |
| `target folder does not exist` | Le dossier choisi pour une copie portable n'existe plus. Choisissez un dossier existant. |
| `that folder already has a .moonpool with an exe of this name that isn't a portable Moonpool - pick an empty folder` | Choisissez un dossier vide, ou supprimez d'abord ce dossier `.moonpool`. |

## Windows a protégé votre ordinateur apparaît quand j'exécute l'installateur

C'est Windows SmartScreen, parce que `moonpool.exe` n'est pas signé numériquement. Cliquez sur
**Informations complémentaires** (« More info »), puis sur **Exécuter quand même** (« Run anyway »).
Voir [Windows a protégé votre ordinateur](/fr/support/windows-protected-your-pc/).

## La fenêtre de Moonpool est vide ou ne s'ouvre jamais sous Windows

Le runtime Microsoft Edge WebView2 est peut-être absent. Voir
[Runtime WebView2 manquant](/fr/support/webview2-runtime-missing/).

## Une app ne démarre pas

1. Cliquez sur le nom de l'app pour ouvrir son onglet de terminal et lisez la sortie. Un agent peut
   lire le même texte avec `moonpool_app_output`.
2. Vérifiez `cwd`. Un dossier manquant, ou un chemin relatif sans `./`, en est la cause habituelle.
   Voir [Chemins et environnement](/fr/apps/paths-and-environment/).
3. Vérifiez `command`. Exécutez-la à la main dans un terminal dans `cwd`. Sous Windows, évitez les
   guillemets doubles imbriqués ; `cmd /c` les déforme.
4. Activez **Journaliser les infos de débogage dans un fichier** dans les Paramètres et relancez.
   `moonpool.log` enregistre la commande et le dossier exacts. Voir [Journaux](/fr/data/logs/).

| Message | Signification |
| --- | --- |
| `already running` | Moonpool a déjà un terminal pour cette app. Arrêtez-la d'abord, ou utilisez Redémarrer. |
| `stopped during launch` | Arrêter a été actionné alors que le lancement était encore en cours de démarrage. |
| `did not reach running in time` | Depuis un script ou un agent : l'app n'a pas été vue en cours d'exécution dans les 25 secondes. Vérifiez son `port` ou son `processName`, et sa sortie. |

## Le voyant d'état est incorrect

Moonpool détermine l'état « en cours » d'après `port`, puis `processName`, puis le fait que son
propre terminal soit toujours actif. Voir
[Comment l'état « en cours » est déterminé](/fr/apps/types/#comment-létat-en-cours-est-déterminé).

- **Ne devient jamais plein.** Le `port` d'une app `web` ne répond pas, ou le `processName` d'une
  app `desktop` ne correspond pas. Sous Linux, `processName` ne doit pas dépasser 15 caractères.
- **Devient gris juste après le lancement.** Une app `cli` cesse d'être « en cours » quand sa
  commande se termine. Utilisez un shell avec `-NoExit` si vous voulez qu'il reste ouvert.
- **Une app `static` n'est jamais « en cours ».** C'est normal pour une entrée qui n'a qu'une
  `url`.
- **Affichée « en cours » alors que vous ne l'avez pas démarrée.** Autre chose utilise ce port ou
  ce nom de processus. Moonpool l'affiche comme en cours d'exécution mais pas « managed by
  Moonpool » (géré par Moonpool).

## Error: listen EADDRINUSE ou « Port 5173 is in use »

Autre chose écoute déjà sur le port que votre serveur veut utiliser. Trouvez-le et terminez-le, ou
définissez `port` sur l'app pour qu'Arrêter le libère. Voir
[Corriger EADDRINUSE et « Port 5173 is in use »](/fr/support/port-already-in-use/) et
[Trouver et terminer le processus qui utilise un port](/fr/guides/find-and-kill-process-using-port-windows/).

## Deux apps utilisent le même port

Une ligne d'avertissement apparaît en bas du menu **...**, par exemple `port 3000 : App A / App B`.
Changez le `port` d'une des apps (et son `env`, si elle lit `PORT`). Voir
[Avertissement de conflit de port](/fr/using/hub-window/#avertissement-de-conflit-de-port).

## L'app reste en cours d'exécution après Arrêter

Depuis un script ou un agent, l'erreur est `still running after stop` (après 15 secondes).

- L'app survit à son terminal. Définissez `killMode` sur `port` ou `processName`. Voir
  [Arrêt et redémarrage](/fr/apps/stop-and-restart/).
- Une app Docker sous Windows : utilisez `killMode` `command` avec un `stopCommand` tel que
  `docker compose stop app`. Jamais `port`.

## apps.json contient une erreur

La barre latérale affiche un bandeau, « apps.json contient une erreur ; affichage de la dernière
liste chargée. » ou, au démarrage, « apps.json contient une erreur ; aucune app n'est chargée. »
L'enregistrement depuis Moonpool est suspendu jusqu'à ce que le fichier se charge de nouveau.

Erreurs typiques :

```text
apps.json entry 2 (site) requires a command
apps.json entry 3 has invalid id "my app"; use letters, digits, '.', '_', and '-' without a leading '-'
duplicate app id "site"
apps.json entry 4 (api) has invalid port 0
```

1. Choisissez **Modifier apps.json** dans le bandeau, corrigez l'entrée, enregistrez, puis
   **Recharger** (F5).
2. Ou revenez à une bonne copie récente. Voir
   [Sauvegarde et récupération](/fr/data/backup-and-recovery/#revenir-en-arrière-sur-appsjson).

La liste complète des règles est dans [Validation](/fr/apps/apps-json/#validation).

Si un paramètre ne peut pas être modifié et que le message se termine par `Repair settings.json and restart
Moonpool before changing settings`, corrigez ou supprimez `settings.json` dans le dossier de
configuration et redémarrez Moonpool. Le supprimer rétablit tous les paramètres à leur valeur par
défaut.

## Ma modification n'a pas pris effet

- Les modifications à la main nécessitent **Recharger** (ou F5). Moonpool ne surveille pas le
  fichier.
- Recharger ne redémarre pas les apps en cours d'exécution. Redémarrez l'app pour utiliser une
  `command`, un `cwd` ou un `env` modifié.
- Un agent modifie peut-être un autre `apps.json`. Demandez-lui d'appeler `moonpool_launcher_paths`
  et de comparer le dossier du hub avec le sien. Avec plusieurs copies de Moonpool, vérifiez quelle
  copie vous modifiez.

## Les apps d'exemple sont absentes

Les exemples ne sont écrits que lorsqu'aucun `apps.json` n'existe. Pour les récupérer, voir
[Revenir aux exemples](/fr/data/backup-and-recovery/#revenir-aux-exemples), ou copiez les entrées
depuis [Tableaux de bord d'exemple](/fr/getting-started/example-dashboards/#les-apps-dexemple-napparaissent-quau-premier-lancement).

## Une mise à jour a échoué

Le bandeau affiche `Échec de la mise à jour : <error>`. Voir
[Quand une mise à jour échoue](/fr/data/updating/#quand-une-mise-à-jour-échoue).

## Un lien web ne s'ouvre pas

`refusing to open non-web url: <url>` signifie que l'`url` n'est pas `http://`, `https://`,
`mailto:` ni `file://`. Corrigez l'`url`.

## Erreurs MCP et de script

| Message | Que faire |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Démarrez Moonpool, ou laissez l'agent appeler `moonpool_bootup_launcher`. |
| `frontend not loaded` | La fenêtre du hub n'a pas fini de se charger. Attendez un instant et réessayez. |
| `stale token: ...` | `apps.json` a changé depuis que l'agent l'a lu. Relisez-le, puis écrivez. |
| `rejected invalid manifest: ...` | Le nouvel `apps.json` a échoué à la validation. Le fichier n'a pas été modifié. |
| `... A Moonpool process may be hung ...` | Quelque chose détient le canal de contrôle sans répondre. Quittez Moonpool depuis la zone de notification, ou terminez le processus, puis redémarrez-le. |

Plus de détails dans [Configuration MCP](/fr/automation/mcp-setup/#remarques) et
[Outils MCP](/fr/automation/mcp-tools/).

## Problèmes de fenêtre

- **Hors de l'écran.** Moonpool ignore une position enregistrée qui ne se trouve sur aucun écran
  connecté. Si la fenêtre reste introuvable, quittez Moonpool et supprimez `window-state.json` dans
  le dossier de configuration.
- **Zoom bloqué trop grand ou trop petit.** Ctrl + molette sur le hub le modifie. Voir
  [Raccourcis et zoom](/fr/using/keyboard-shortcuts/#zoom).
- **Les Paramètres s'ouvrent derrière le hub.** Désactivez, ou activez, **Toujours au premier plan**
  dans les Paramètres. Il s'applique à toutes les fenêtres de Moonpool, qui restent ainsi dans le
  même plan.

## Où sont les journaux ?

Voir [Journaux](/fr/data/logs/).

## Sauvegarder, réinitialiser ou désinstaller

Voir [Sauvegarde et récupération](/fr/data/backup-and-recovery/) et
[Désinstallation](/fr/getting-started/install/#désinstallation).

## FAQ

**Fermer la fenêtre arrête-t-il mes apps ?**
Par défaut, fermer quitte Moonpool, et sous Windows quitter arrête les apps qu'il a lancées.
Activez **Fermer vers la zone de notification** pour garder Moonpool en cours d'exécution quand
vous fermez la fenêtre. Voir
[Zone de notification, fermeture et réduction](/fr/using/tray-and-closing/).

**Puis-je exécuter Moonpool deux fois ?**
Une instance par dossier. Démarrer de nouveau la même copie ramène sa fenêtre. La copie installée
et les copies portables peuvent s'exécuter côte à côte. Voir
[Mode portable](/fr/data/portable-mode/#plusieurs-copies-à-la-fois).

**Moonpool communique-t-il avec l'extérieur ?**
Uniquement pour rechercher des mises à jour : il récupère le fichier de version (`update.json`)
depuis GitHub au démarrage (si **Rechercher des mises à jour au démarrage** est activé) et quand
vous cliquez sur **Rechercher des mises à jour**. Chaque téléchargement est vérifié avec la clé de
signature de Moonpool avant d'être utilisé.

**Quel shell exécute mes commandes ?**
`cmd /c` sous Windows, `$SHELL -c` sous Linux et macOS.

**Où mettre les secrets ?**
Les valeurs de `env` sont stockées en clair dans `apps.json`. Préférez un fichier que votre app lit
elle-même, ou une variable déjà définie dans votre environnement utilisateur, dont héritent les apps
lancées.

**Le canal de contrôle est-il protégé ?**
Il n'a ni connexion ni jeton. Tout processus qui s'exécute sous votre compte peut lui envoyer des
commandes. Sous Linux et macOS, le socket n'est lisible que par votre utilisateur. Voir
[Propriétés de sécurité](/fr/automation/overview/#propriétés-de-sécurité).
