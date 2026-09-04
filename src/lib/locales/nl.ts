// Nederlands. Keys absent here fall back to English at runtime.
import type { PartialDict } from "../i18n.svelte";

export const nl: PartialDict = {
  "common.close": "Sluiten",
  "common.cancel": "Annuleren",
  "common.save": "Opslaan",
  "common.delete": "Verwijderen",
  "common.edit": "Bewerken",
  "common.rename": "Naam wijzigen",
  "common.reload": "Opnieuw laden",
  "common.settings": "Instellingen",
  "common.about": "Over",
  "common.tryAgain": "Opnieuw proberen",
  "common.dismiss": "Verbergen",
  "common.copied": "Gekopieerd!",
  "common.autoSystem": "Automatisch (systeem)",

  "titlebar.minimize": "Minimaliseren",
  "titlebar.maximize": "Maximaliseren",
  "titlebar.restore": "Herstellen",

  "sidebar.menu": "Menu",
  "sidebar.addApp": "App toevoegen",
  "sidebar.editJson": "apps.json bewerken",
  "sidebar.installMoonpool": "Moonpool installeren…",
  "sidebar.filterPlaceholder": "Apps filteren...",
  "sidebar.filterLabel": "Apps filteren",
  "sidebar.showCli": "CLI-paneel tonen",
  "sidebar.updateOpenToInstall": "Update beschikbaar, open om te installeren",
  "sidebar.updateShowCli": "Update beschikbaar, toon het CLI-paneel",
  "sidebar.statusRunning": "actief",
  "sidebar.statusStarting": "starten...",
  "sidebar.statusStopped": "gestopt",
  "sidebar.working": "Bezig...",
  "sidebar.restart": "Herstarten",
  "sidebar.stop": "Stoppen",
  "sidebar.launch": "Starten",
  "sidebar.setIcon": "Pictogram kiezen...",
  "sidebar.portConflict": "{names} gebruiken allebei poort {port}",
  "sidebar.portConflictBadge": "poort {port}: {names}",
  "sidebar.noMatch": "Geen app komt overeen met “{filter}”.",

  "app.dragToResize": "Sleep om het formaat te wijzigen",
  "app.closeTab": "Tabblad sluiten",
  "app.hideCli": "CLI-paneel verbergen",
  "app.pickApp": "Kies links een app om die te starten.",
  "app.updateAvailable": "Moonpool {version} is beschikbaar (je hebt {current}).",
  "app.downloading": "{version} downloaden…",
  "app.installing": "Installeren…",
  "app.downloadInstall": "Downloaden en installeren",
  "app.updateInstalled": "Update geïnstalleerd. Herstarten…",
  "app.updateFailed": "Update mislukt: {error}",
  "app.confirmDelete": "“{name}” verwijderen?",
  "app.iconDialogTitle": "Pictogram voor {name}",
  "app.imagesFilter": "Afbeeldingen",
  "app.newHere":
    "Nieuw hier? Geef dit aan een AI-agent om je apps in te stellen:",
  "app.copyPrompt": "Prompt kopiëren",
  "app.editFile": "Bestand bewerken",
  "app.orUseHint":
    "Of gebruik {add} hierboven, of {edit} om {file} rechtstreeks te bewerken.",

  "about.tagline":
    "Een starthub in het systeemvak voor lokale apps en dev-servers, met een ingebouwde terminal per app.",
  "about.version": "versie {version}",
  "about.checkUpdates": "Controleren op updates",
  "about.checking": "Controleren op updates...",
  "about.upToDate": "Je hebt de nieuwste versie.",
  "about.updateDownloading": "Update {version} beschikbaar: downloaden...",
  "about.updateInstalled": "Update geïnstalleerd. Herstarten...",
  "about.checkFailed": "Controle op updates mislukt: {error}",
  "about.builtWith": "Gebouwd met",
  "about.byLine": "MIT License · door {author}",

  "settings.title": "Instellingen",
  "settings.theme": "Thema",
  "settings.language": "Taal",
  "settings.languageHint": "De taal waarin Moonpools eigen teksten worden getoond.",
  "settings.closeToTray": "Sluiten naar systeemvak",
  "settings.closeToTrayHint":
    "Bij het sluiten verdwijnt Moonpool naar het systeemvak (en uit de taakbalk). Uit: sluiten stopt de app.",
  "settings.minimizeToTray": "Minimaliseren naar systeemvak",
  "settings.minimizeToTrayHint":
    "Bij het minimaliseren verdwijnt Moonpool naar het systeemvak (en uit de taakbalk). Uit: minimaliseert naar de taakbalk.",
  "settings.alwaysOnTop": "Altijd op voorgrond",
  "settings.alwaysOnTopHint":
    "Houdt Moonpool en de vensters Instellingen en Over boven andere vensters.",
  "settings.transparency": "Transparantie van de achtergrond",
  "settings.transparencyHint": "Doorschijnende vensterachtergrond. 0% is dekkend.",
  "settings.checkOnStartup": "Bij het opstarten controleren op updates",
  "settings.checkOnStartupHint":
    "Kijkt bij het starten stilletjes op GitHub of er een nieuwere versie is en toont dan een melding.",
  "settings.debugLogging": "Debug-informatie naar een bestand loggen",
  "settings.debugLoggingHint":
    "Legt het laden van manifesten, starts en fouten vast in {file}.",
  "settings.openLog": "Logbestand openen",

  "theme.dark": "Donker",
  "theme.light": "Licht",

  "installer.tagline":
    "Een starter in het systeemvak voor je lokale apps en dev-servers.",
  "installer.pickFolder": "Kies een map voor het portable Moonpool",
  "installer.poke": "linkerklik om te porren, rechterklik om te resetten",
  "installer.installed": "Geïnstalleerd, Moonpool wordt gestart…",
  "installer.portableDone": "Portable modus, Moonpool wordt gestart…",
  "installer.failed": "Installatie mislukt: {error}",
  "installer.alreadyInstalledTitle": "Deze kopie is al op deze pc geïnstalleerd.",
  "installer.alreadyInstalled": "Al geïnstalleerd",
  "installer.installing": "Installeren…",
  "installer.install": "Moonpool installeren",
  "installer.desktopShortcut": "Snelkoppeling op het bureaublad maken",
  "installer.portableHint":
    "Start Moonpool vanuit een map die je zelf kiest (usb-stick, zip) en verplaats die waarheen je wilt. De gegevens blijven naast de exe staan.",
  "installer.installPortable": "Portable installeren",
  "installer.installPath": "Installatiepad: {dir}",

  "editor.addApp": "App toevoegen",
  "editor.editApp": "App bewerken",
  "editor.backToList": "terug naar de lijst",
  "editor.nameHint": "De naam die in de zijbalk wordt getoond. Verplicht.",
  "editor.namePlaceholder": "Mijn app",
  "editor.groupHint":
    "De kop in de zijbalk waaronder deze app komt te staan. Kies een bestaande groep of kies “+ Nieuwe groep” om er een toe te voegen.",
  "editor.newGroupPlaceholder": "Naam van de nieuwe groep",
  "editor.pickExistingGroup": "Kies een bestaande groep",
  "editor.newGroupOption": "+ Nieuwe groep...",
  "editor.typeHint":
    "Hoe Moonpool de app start en volgt. web = dev-server op een poort. desktop = native app, herkend aan de procesnaam. static = opent alleen een URL. cli = voert een commando uit in een terminal.",
  "editor.hintWeb":
    "Start een dev-server in een terminal; toont Actief zodra de poort antwoordt en opent dan de browser.",
  "editor.hintDesktop":
    "Start een native app; toont Actief zodra een proces met de naam processName wordt gevonden.",
  "editor.hintStatic": "Opent alleen url in de browser, zonder terminal of commando.",
  "editor.hintCli": "Voert een commando uit en houdt een interactieve shell open in cwd.",
  "editor.portableWarn":
    "Absoluut pad - verhuist niet mee met deze map. Gebruik {MP_HOME}\\... of een ./-pad om het portable te houden.",
  "editor.notPortable": "niet portable",
  "editor.cwdHint":
    "De werkmap waarin het commando draait, meestal de projectmap van de app. Gebruik een absoluut pad, of {MP_HOME}\\... / ./ om het portable te houden.",
  "editor.commandHint":
    "Het commando dat in de ingebouwde terminal wordt uitgevoerd om de app te starten, bijv. 'npm run dev' of 'python app.py'. Laat leeg voor een item met alleen een URL.",
  "editor.portHint":
    "De lokale TCP-poort waarop de app luistert. Moonpool toont Actief zodra deze poort antwoordt en geeft de poort vrij bij Stoppen. Wordt gebruikt door web-apps.",
  "editor.processNameHint":
    "Voor desktop-apps: de proces- of programmanaam (zonder .exe) waarmee Actief wordt herkend en de app wordt gestopt. Op Linux maximaal 15 tekens.",
  "editor.urlHint":
    "De URL die geopend wordt: http://localhost:<port> voor een web-app, of file:///path/to/index.html voor een statische pagina. Gebruik file:///{MP_HOME}/... om het portable te houden.",
  "editor.openBrowser": "browser openen",
  "editor.openBrowserHint":
    "Opent de URL automatisch in je standaardbrowser zodra de app bereikbaar is.",
  "editor.envLabel": "env (KEY=VALUE per regel)",
  "editor.envHint":
    "Omgevingsvariabelen die aan het commando worden meegegeven, één KEY=VALUE per regel (bijv. PORT=3000).",
  "editor.noteHint":
    "Optionele tekst die als tooltip verschijnt wanneer je in de zijbalk over deze app zweeft.",
  "editor.notePlaceholder": "optionele tooltip",
  "editor.phFolder": "pad naar de map van de app",
  "editor.phLaunchCommand": "het startcommando van de app",
  "editor.phCommand": "uit te voeren commando",
  "editor.discardChanges": "Je wijzigingen weggooien?",
  "editor.nameRequired": "naam is verplicht.",

  "term.processExited": "[proces beëindigd]",
};
