// Français. Keys absent here fall back to English at runtime.
import type { PartialDict } from "../i18n.svelte";

export const fr: PartialDict = {
  "common.close": "Fermer",
  "common.cancel": "Annuler",
  "common.save": "Enregistrer",
  "common.delete": "Supprimer",
  "common.edit": "Modifier",
  "common.rename": "Renommer",
  "common.reload": "Recharger",
  "common.settings": "Paramètres",
  "common.about": "À propos",
  "common.tryAgain": "Réessayer",
  "common.dismiss": "Ignorer",
  "common.copied": "Copié !",
  "common.autoSystem": "Automatique (système)",

  "titlebar.minimize": "Réduire",
  "titlebar.maximize": "Agrandir",
  "titlebar.restore": "Restaurer",

  "sidebar.menu": "Menu",
  "sidebar.addApp": "Ajouter une app",
  "sidebar.editJson": "Modifier apps.json",
  "sidebar.installMoonpool": "Installer Moonpool…",
  "sidebar.filterPlaceholder": "Filtrer les apps...",
  "sidebar.filterLabel": "Filtrer les apps",
  "sidebar.showCli": "Afficher le panneau CLI",
  "sidebar.updateOpenToInstall": "Mise à jour disponible, ouvrir pour l'installer",
  "sidebar.updateShowCli": "Mise à jour disponible, afficher le panneau CLI",
  "sidebar.statusRunning": "en cours",
  "sidebar.statusStarting": "démarrage...",
  "sidebar.statusStopped": "arrêtée",
  "sidebar.working": "En cours...",
  "sidebar.restart": "Redémarrer",
  "sidebar.stop": "Arrêter",
  "sidebar.launch": "Lancer",
  "sidebar.setIcon": "Choisir une icône...",
  "sidebar.portConflict": "{names} utilisent toutes les deux le port {port}",
  "sidebar.portConflictBadge": "port {port} : {names}",
  "sidebar.noMatch": "Aucune app ne correspond à « {filter} ».",

  "app.dragToResize": "Glisser pour redimensionner",
  "app.closeTab": "Fermer l'onglet",
  "app.hideCli": "Masquer le panneau CLI",
  "app.pickApp": "Choisissez une app à gauche pour la lancer.",
  "app.updateAvailable": "Moonpool {version} est disponible (vous avez la {current}).",
  "app.downloading": "Téléchargement de {version}…",
  "app.installing": "Installation…",
  "app.downloadInstall": "Télécharger et installer",
  "app.updateInstalled": "Mise à jour installée. Redémarrage…",
  "app.updateFailed": "Échec de la mise à jour : {error}",
  "app.confirmDelete": "Supprimer « {name} » ?",
  "app.iconDialogTitle": "Icône de {name}",
  "app.imagesFilter": "Images",
  "app.newHere":
    "Vous débutez ? Donnez ceci à un agent IA pour qu'il configure vos apps :",
  "app.copyPrompt": "Copier le prompt",
  "app.editFile": "Modifier le fichier",
  "app.orUseHint": "Ou utilisez {add} ci-dessus, ou {edit} pour modifier {file} directement.",

  "about.tagline":
    "Un lanceur dans la zone de notification pour vos apps locales et serveurs de développement, avec un terminal intégré par app.",
  "about.version": "version {version}",
  "about.checkUpdates": "Rechercher des mises à jour",
  "about.checking": "Recherche de mises à jour...",
  "about.upToDate": "Vous avez la dernière version.",
  "about.updateDownloading": "Mise à jour {version} disponible : téléchargement...",
  "about.updateInstalled": "Mise à jour installée. Redémarrage...",
  "about.checkFailed": "Échec de la recherche de mises à jour : {error}",
  "about.builtWith": "Construit avec",
  "about.byLine": "Licence MIT · par {author}",

  "settings.title": "Paramètres",
  "settings.theme": "Thème",
  "settings.language": "Langue",
  "settings.languageHint": "La langue dans laquelle le texte de Moonpool s'affiche.",
  "settings.closeToTray": "Fermer vers la zone de notification",
  "settings.closeToTrayHint":
    "Fermer la fenêtre masque Moonpool dans la zone de notification (et le retire de la barre des tâches). Désactivé : fermer quitte l'app.",
  "settings.minimizeToTray": "Réduire vers la zone de notification",
  "settings.minimizeToTrayHint":
    "Réduire masque Moonpool dans la zone de notification (et le retire de la barre des tâches). Désactivé : réduit vers la barre des tâches.",
  "settings.alwaysOnTop": "Toujours au premier plan",
  "settings.alwaysOnTopHint":
    "Garde Moonpool et ses fenêtres Paramètres et À propos au-dessus des autres fenêtres.",
  "settings.transparency": "Transparence du fond",
  "settings.transparencyHint": "Fond de fenêtre translucide. 0 % est opaque.",
  "settings.checkOnStartup": "Rechercher des mises à jour au démarrage",
  "settings.checkOnStartupHint":
    "Au lancement, interroge GitHub en arrière-plan et affiche un bandeau si une version plus récente existe.",
  "settings.debugLogging": "Journaliser les infos de débogage dans un fichier",
  "settings.debugLoggingHint":
    "Enregistre les chargements du manifeste, les lancements et les erreurs dans {file}.",
  "settings.openLog": "Ouvrir le journal",

  "theme.dark": "Sombre",
  "theme.light": "Clair",

  "installer.tagline":
    "Un lanceur dans la zone de notification pour vos apps locales et serveurs de développement.",
  "installer.pickFolder": "Choisissez un dossier pour Moonpool portable",
  "installer.poke": "clic gauche pour secouer, clic droit pour réinitialiser",
  "installer.installed": "Installé, démarrage de Moonpool…",
  "installer.portableDone": "Mode portable, démarrage de Moonpool…",
  "installer.failed": "Échec de l'installation : {error}",
  "installer.alreadyInstalledTitle": "Cette copie est déjà installée sur ce PC.",
  "installer.alreadyInstalled": "Déjà installé",
  "installer.installing": "Installation…",
  "installer.install": "Installer Moonpool",
  "installer.desktopShortcut": "Ajouter un raccourci sur le bureau",
  "installer.portableHint":
    "Lancez Moonpool depuis un dossier de votre choix (clé USB, zip) et déplacez-le où vous voulez. Les données restent à côté de l'exe.",
  "installer.installPortable": "Installer en portable",
  "installer.installPath": "Chemin d'installation : {dir}",

  "editor.addApp": "Ajouter une app",
  "editor.editApp": "Modifier l'app",
  "editor.backToList": "retour à la liste",
  "editor.nameHint": "Le nom affiché dans la barre latérale. Obligatoire.",
  "editor.namePlaceholder": "Mon app",
  "editor.groupHint":
    "Le titre de la barre latérale sous lequel cette app apparaît. Choisissez un groupe existant ou « + Nouveau groupe » pour en créer un.",
  "editor.newGroupPlaceholder": "Nom du nouveau groupe",
  "editor.pickExistingGroup": "Choisir un groupe existant",
  "editor.newGroupOption": "+ Nouveau groupe...",
  "editor.typeHint":
    "Comment Moonpool lance et suit l'app. web = serveur de développement sur un port. desktop = app native suivie par son nom de processus. static = ouvre simplement une URL. cli = lance une commande dans un terminal.",
  "editor.hintWeb":
    "Lance un serveur de développement dans un terminal ; indique « en cours » dès que son port répond, et ouvre le navigateur une fois en ligne.",
  "editor.hintDesktop":
    "Lance une app native ; indique « en cours » lorsqu'un processus nommé processName est trouvé.",
  "editor.hintStatic": "Ouvre simplement url dans le navigateur, sans terminal ni commande.",
  "editor.hintCli": "Lance une commande et garde un shell interactif ouvert dans cwd.",
  "editor.portableWarn":
    "Chemin absolu : il ne suivra pas ce dossier. Utilisez {MP_HOME}\\... ou un chemin ./ pour rester portable.",
  "editor.notPortable": "non portable",
  "editor.cwdHint":
    "Le répertoire de travail dans lequel la commande s'exécute, en général le dossier du projet. Utilisez un chemin absolu, ou {MP_HOME}\\... / ./ pour rester portable.",
  "editor.commandHint":
    "La commande lancée dans le terminal intégré pour démarrer l'app, par ex. 'npm run dev' ou 'python app.py'. Laissez vide pour une entrée uniquement URL.",
  "editor.portHint":
    "Le port TCP local sur lequel l'app écoute. Moonpool l'indique « en cours » quand ce port répond, et le libère à l'arrêt. Utilisé par les apps web.",
  "editor.processNameHint":
    "Pour les apps de bureau : le nom du processus ou de l'exécutable (sans .exe) utilisé pour détecter son exécution et l'arrêter. Sous Linux il ne doit pas dépasser 15 caractères.",
  "editor.urlHint":
    "L'URL à ouvrir : http://localhost:<port> pour une app web, ou file:///path/to/index.html pour une page statique. Utilisez file:///{MP_HOME}/... pour rester portable.",
  "editor.openBrowser": "ouvrir le navigateur",
  "editor.openBrowserHint":
    "Ouvre automatiquement l'URL dans votre navigateur par défaut dès que l'app répond.",
  "editor.envLabel": "env (KEY=VALUE par ligne)",
  "editor.envHint":
    "Variables d'environnement passées à la commande, une KEY=VALUE par ligne (par ex. PORT=3000).",
  "editor.noteHint":
    "Texte facultatif affiché en infobulle au survol de cette app dans la barre latérale.",
  "editor.notePlaceholder": "infobulle facultative",
  "editor.phFolder": "chemin vers le dossier de l'app",
  "editor.phLaunchCommand": "la commande de lancement de l'app",
  "editor.phCommand": "commande à exécuter",
  "editor.discardChanges": "Abandonner vos modifications ?",
  "editor.nameRequired": "le nom est obligatoire.",

  "term.processExited": "[processus terminé]",
  "term.copyAll": "Tout copier",
  "term.copied": "Copié",
};
