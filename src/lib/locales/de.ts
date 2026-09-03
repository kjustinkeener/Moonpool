// Deutsch. Keys absent here fall back to English at runtime.
import type { PartialDict } from "../i18n.svelte";

export const de: PartialDict = {
  "common.close": "Schließen",
  "common.cancel": "Abbrechen",
  "common.save": "Speichern",
  "common.delete": "Löschen",
  "common.edit": "Bearbeiten",
  "common.rename": "Umbenennen",
  "common.reload": "Neu laden",
  "common.settings": "Einstellungen",
  "common.about": "Über",
  "common.tryAgain": "Erneut versuchen",
  "common.dismiss": "Ausblenden",
  "common.copied": "Kopiert!",
  "common.autoSystem": "Automatisch (System)",

  "titlebar.minimize": "Minimieren",
  "titlebar.maximize": "Maximieren",
  "titlebar.restore": "Wiederherstellen",

  "sidebar.menu": "Menü",
  "sidebar.addApp": "App hinzufügen",
  "sidebar.editJson": "apps.json bearbeiten",
  "sidebar.installMoonpool": "Moonpool installieren…",
  "sidebar.filterPlaceholder": "Apps filtern...",
  "sidebar.filterLabel": "Apps filtern",
  "sidebar.showCli": "CLI-Bereich einblenden",
  "sidebar.updateOpenToInstall": "Update verfügbar, zum Installieren öffnen",
  "sidebar.updateShowCli": "Update verfügbar, CLI-Bereich einblenden",
  "sidebar.statusRunning": "läuft",
  "sidebar.statusStarting": "startet...",
  "sidebar.statusStopped": "gestoppt",
  "sidebar.working": "Arbeitet...",
  "sidebar.restart": "Neu starten",
  "sidebar.stop": "Stoppen",
  "sidebar.launch": "Starten",
  "sidebar.setIcon": "Symbol wählen...",
  "sidebar.portConflict": "{names} belegen beide Port {port}",
  "sidebar.portConflictBadge": "Port {port}: {names}",
  "sidebar.noMatch": "Keine App passt zu „{filter}“.",

  "app.dragToResize": "Zum Ändern der Größe ziehen",
  "app.closeTab": "Tab schließen",
  "app.hideCli": "CLI-Bereich ausblenden",
  "app.pickApp": "Wählen Sie links eine App aus, um sie zu starten.",
  "app.updateAvailable": "Moonpool {version} ist verfügbar (Sie haben {current}).",
  "app.downloading": "{version} wird heruntergeladen…",
  "app.installing": "Wird installiert…",
  "app.downloadInstall": "Herunterladen und installieren",
  "app.updateInstalled": "Update installiert. Neustart…",
  "app.updateFailed": "Update fehlgeschlagen: {error}",
  "app.confirmDelete": "„{name}“ löschen?",
  "app.iconDialogTitle": "Symbol für {name}",
  "app.imagesFilter": "Bilder",
  "app.newHere":
    "Neu hier? Geben Sie das einem KI-Agenten, damit er Ihre Apps einrichtet:",
  "app.copyPrompt": "Prompt kopieren",
  "app.editFile": "Datei bearbeiten",
  "app.orUseHint":
    "Oder nutzen Sie oben {add}, oder {edit}, um {file} direkt zu bearbeiten.",

  "about.tagline":
    "Ein Startzentrum im Infobereich für lokale Apps und Entwicklungsserver, mit einem eingebetteten Terminal pro App.",
  "about.version": "Version {version}",
  "about.checkUpdates": "Nach Updates suchen",
  "about.checking": "Suche nach Updates...",
  "about.upToDate": "Sie haben die neueste Version.",
  "about.updateDownloading": "Update {version} verfügbar: wird heruntergeladen...",
  "about.updateInstalled": "Update installiert. Neustart...",
  "about.checkFailed": "Update-Suche fehlgeschlagen: {error}",
  "about.builtWith": "Gebaut mit",
  "about.byLine": "MIT-Lizenz · von {author}",

  "settings.title": "Einstellungen",
  "settings.theme": "Design",
  "settings.language": "Sprache",
  "settings.languageHint": "Die Sprache, in der Moonpools eigene Texte erscheinen.",
  "settings.closeToTray": "Beim Schließen in den Infobereich",
  "settings.closeToTrayHint":
    "Das Schließen des Fensters blendet Moonpool in den Infobereich aus (und aus der Taskleiste). Aus: Schließen beendet die App.",
  "settings.minimizeToTray": "Beim Minimieren in den Infobereich",
  "settings.minimizeToTrayHint":
    "Das Minimieren blendet Moonpool in den Infobereich aus (und aus der Taskleiste). Aus: minimiert in die Taskleiste.",
  "settings.alwaysOnTop": "Immer im Vordergrund",
  "settings.alwaysOnTopHint":
    "Hält Moonpool und seine Einstellungs- und Über-Fenster über allen anderen Fenstern.",
  "settings.transparency": "Hintergrundtransparenz",
  "settings.transparencyHint": "Durchscheinender Fensterhintergrund. 0 % ist deckend.",
  "settings.checkOnStartup": "Beim Start nach Updates suchen",
  "settings.checkOnStartupHint":
    "Fragt beim Start unauffällig bei GitHub nach einer neueren Version und zeigt bei Erfolg einen Hinweis.",
  "settings.debugLogging": "Debug-Infos in eine Datei schreiben",
  "settings.debugLoggingHint":
    "Protokolliert Manifest-Ladevorgänge, Starts und Fehler in {file}.",
  "settings.openLog": "Protokoll öffnen",

  "theme.dark": "Dunkel",
  "theme.light": "Hell",

  "installer.tagline":
    "Ein Starter im Infobereich für Ihre lokalen Apps und Entwicklungsserver.",
  "installer.pickFolder": "Ordner für das portable Moonpool wählen",
  "installer.poke": "Linksklick zum Anstupsen, Rechtsklick zum Zurücksetzen",
  "installer.installed": "Installiert, Moonpool startet…",
  "installer.portableDone": "Portabler Modus, Moonpool startet…",
  "installer.failed": "Installation fehlgeschlagen: {error}",
  "installer.alreadyInstalledTitle": "Diese Kopie ist auf diesem PC bereits installiert.",
  "installer.alreadyInstalled": "Bereits installiert",
  "installer.installing": "Wird installiert…",
  "installer.install": "Moonpool installieren",
  "installer.desktopShortcut": "Verknüpfung auf dem Desktop anlegen",
  "installer.portableHint":
    "Moonpool aus einem selbst gewählten Ordner starten (USB-Stick, ZIP) und beliebig verschieben. Die Daten bleiben neben der EXE.",
  "installer.installPortable": "Portabel installieren",
  "installer.installPath": "Installationspfad: {dir}",

  "editor.addApp": "App hinzufügen",
  "editor.editApp": "App bearbeiten",
  "editor.backToList": "zurück zur Liste",
  "editor.nameHint": "Der in der Seitenleiste angezeigte Name. Pflichtfeld.",
  "editor.namePlaceholder": "Meine App",
  "editor.groupHint":
    "Die Überschrift in der Seitenleiste, unter der diese App steht. Wählen Sie eine vorhandene Gruppe oder „+ Neue Gruppe“, um eine anzulegen.",
  "editor.newGroupPlaceholder": "Name der neuen Gruppe",
  "editor.pickExistingGroup": "Vorhandene Gruppe wählen",
  "editor.newGroupOption": "+ Neue Gruppe...",
  "editor.typeHint":
    "Wie Moonpool die App startet und überwacht. web = Entwicklungsserver auf einem Port. desktop = native App, erkannt am Prozessnamen. static = öffnet nur eine URL. cli = führt einen Befehl in einem Terminal aus.",
  "editor.hintWeb":
    "Startet einen Entwicklungsserver in einem Terminal; meldet „läuft“, sobald der Port antwortet, und öffnet dann den Browser.",
  "editor.hintDesktop":
    "Startet eine native App; meldet „läuft“, sobald ein Prozess namens processName gefunden wird.",
  "editor.hintStatic": "Öffnet nur url im Browser, ohne Terminal und ohne Befehl.",
  "editor.hintCli": "Führt einen Befehl aus und hält eine interaktive Shell in cwd offen.",
  "editor.portableWarn":
    "Absoluter Pfad: zieht mit diesem Ordner nicht mit um. Nutzen Sie {MP_HOME}\\... oder einen ./-Pfad, damit es portabel bleibt.",
  "editor.notPortable": "nicht portabel",
  "editor.cwdHint":
    "Das Arbeitsverzeichnis, in dem der Befehl läuft, meist der Projektordner der App. Nutzen Sie einen absoluten Pfad, oder {MP_HOME}\\... / ./, damit es portabel bleibt.",
  "editor.commandHint":
    "Der Befehl, der im eingebetteten Terminal die App startet, z. B. 'npm run dev' oder 'python app.py'. Für einen reinen URL-Eintrag leer lassen.",
  "editor.portHint":
    "Der lokale TCP-Port, auf dem die App lauscht. Moonpool meldet „läuft“, sobald dieser Port antwortet, und gibt ihn beim Stoppen frei. Wird von Web-Apps genutzt.",
  "editor.processNameHint":
    "Für Desktop-Apps: der Prozess- bzw. Programmname (ohne .exe), an dem der Betrieb erkannt und die App gestoppt wird. Unter Linux höchstens 15 Zeichen.",
  "editor.urlHint":
    "Die zu öffnende URL: http://localhost:<port> für eine Web-App, oder file:///path/to/index.html für eine statische Seite. Nutzen Sie file:///{MP_HOME}/..., damit es portabel bleibt.",
  "editor.openBrowser": "Browser öffnen",
  "editor.openBrowserHint":
    "Öffnet die URL automatisch im Standardbrowser, sobald die App erreichbar ist.",
  "editor.envLabel": "env (KEY=VALUE pro Zeile)",
  "editor.envHint":
    "Umgebungsvariablen für den Befehl, eine KEY=VALUE pro Zeile (z. B. PORT=3000).",
  "editor.noteHint":
    "Optionaler Text, der als Tooltip erscheint, wenn Sie in der Seitenleiste über diese App fahren.",
  "editor.notePlaceholder": "optionaler Tooltip",
  "editor.phFolder": "Pfad zum App-Ordner",
  "editor.phLaunchCommand": "der Startbefehl der App",
  "editor.phCommand": "auszuführender Befehl",
  "editor.discardChanges": "Ihre Änderungen verwerfen?",
  "editor.nameRequired": "Name ist erforderlich.",

  "term.processExited": "[Prozess beendet]",
};
