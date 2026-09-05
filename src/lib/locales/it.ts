// Italiano. Keys absent here fall back to English at runtime.
import type { PartialDict } from "../i18n.svelte";

export const it: PartialDict = {
  "common.close": "Chiudi",
  "common.cancel": "Annulla",
  "common.save": "Salva",
  "common.delete": "Elimina",
  "common.edit": "Modifica",
  "common.rename": "Rinomina",
  "common.reload": "Ricarica",
  "common.settings": "Impostazioni",
  "common.about": "Informazioni",
  "common.tryAgain": "Riprova",
  "common.dismiss": "Ignora",
  "common.copied": "Copiato!",
  "common.autoSystem": "Automatico (sistema)",

  "titlebar.minimize": "Riduci a icona",
  "titlebar.maximize": "Ingrandisci",
  "titlebar.restore": "Ripristina",

  "sidebar.menu": "Menu",
  "sidebar.addApp": "Aggiungi app",
  "sidebar.editJson": "Modifica apps.json",
  "sidebar.installMoonpool": "Installa Moonpool…",
  "sidebar.filterPlaceholder": "Filtra le app...",
  "sidebar.filterLabel": "Filtra le app",
  "sidebar.showCli": "Mostra il pannello CLI",
  "sidebar.updateOpenToInstall": "Aggiornamento disponibile, apri per installarlo",
  "sidebar.updateShowCli": "Aggiornamento disponibile, mostra il pannello CLI",
  "sidebar.statusRunning": "in esecuzione",
  "sidebar.statusStarting": "avvio in corso...",
  "sidebar.statusStopped": "arrestata",
  "sidebar.working": "Operazione in corso...",
  "sidebar.restart": "Riavvia",
  "sidebar.stop": "Arresta",
  "sidebar.launch": "Avvia",
  "sidebar.setIcon": "Scegli icona...",
  "sidebar.portConflict": "{names} usano entrambe la porta {port}",
  "sidebar.portConflictBadge": "porta {port}: {names}",
  "sidebar.noMatch": "Nessuna app corrisponde a «{filter}».",

  "app.dragToResize": "Trascina per ridimensionare",
  "app.closeTab": "Chiudi scheda",
  "app.hideCli": "Nascondi il pannello CLI",
  "app.pickApp": "Scegli un'app a sinistra per avviarla.",
  "app.updateAvailable": "Moonpool {version} è disponibile (hai la {current}).",
  "app.downloading": "Download di {version} in corso…",
  "app.installing": "Installazione in corso…",
  "app.downloadInstall": "Scarica e installa",
  "app.updateInstalled": "Aggiornamento installato. Riavvio…",
  "app.updateFailed": "Aggiornamento non riuscito: {error}",
  "app.confirmDelete": "Eliminare «{name}»?",
  "app.iconDialogTitle": "Icona per {name}",
  "app.imagesFilter": "Immagini",
  "app.newHere":
    "È la prima volta? Passa questo testo a un agente IA perché configuri le tue app:",
  "app.copyPrompt": "Copia il prompt",
  "app.editFile": "Modifica il file",
  "app.orUseHint": "Oppure usa {add} qui sopra, o {edit} per modificare {file} direttamente.",

  "about.tagline":
    "Un hub di avvio nella barra delle applicazioni per app locali e server di sviluppo, con un terminale integrato per ogni app.",
  "about.version": "versione {version}",
  "about.checkUpdates": "Controlla aggiornamenti",
  "about.checking": "Ricerca aggiornamenti in corso...",
  "about.upToDate": "Hai già l'ultima versione.",
  "about.updateDownloading": "Aggiornamento {version} disponibile: download in corso...",
  "about.updateInstalled": "Aggiornamento installato. Riavvio in corso...",
  "about.checkFailed": "Controllo aggiornamenti non riuscito: {error}",
  "about.builtWith": "Realizzato con",
  "about.byLine": "Licenza MIT · di {author}",

  "settings.title": "Impostazioni",
  "settings.theme": "Tema",
  "settings.language": "Lingua",
  "settings.languageHint": "La lingua in cui viene mostrato il testo di Moonpool.",
  "settings.closeToTray": "Chiudi nella barra delle applicazioni",
  "settings.closeToTrayHint":
    "Chiudendo la finestra, Moonpool si nasconde nell'area di notifica (esce dalla barra delle applicazioni). Disattivato: chiudendo si esce.",
  "settings.minimizeToTray": "Riduci a icona nella barra delle applicazioni",
  "settings.minimizeToTrayHint":
    "Riducendo a icona, Moonpool si nasconde nell'area di notifica (esce dalla barra delle applicazioni). Disattivato: si riduce alla barra delle applicazioni.",
  "settings.alwaysOnTop": "Sempre in primo piano",
  "settings.alwaysOnTopHint":
    "Mantiene Moonpool e le sue finestre Impostazioni e Informazioni sopra le altre finestre.",
  "settings.transparency": "Trasparenza dello sfondo",
  "settings.transparencyHint": "Sfondo della finestra semitrasparente. 0% è opaco.",
  "settings.checkOnStartup": "Controlla aggiornamenti all'avvio",
  "settings.checkOnStartupHint":
    "All'avvio controlla in background se su GitHub è presente una versione più recente e, in tal caso, mostra un avviso.",
  "settings.debugLogging": "Registra le informazioni di debug su file",
  "settings.debugLoggingHint":
    "Registra i caricamenti del manifest, gli avvii e gli errori in {file}.",
  "settings.openLog": "Apri il registro",

  "theme.dark": "Scuro",
  "theme.light": "Chiaro",

  "installer.tagline":
    "Un launcher nella barra delle applicazioni per le tue app locali e i tuoi server di sviluppo.",
  "installer.pickFolder": "Scegli una cartella per Moonpool portatile",
  "installer.poke": "clic sinistro per stuzzicare, clic destro per ripristinare",
  "installer.installed": "Installato, avvio di Moonpool…",
  "installer.portableDone": "Modalità portatile, avvio di Moonpool…",
  "installer.failed": "Installazione non riuscita: {error}",
  "installer.alreadyInstalledTitle": "Questa copia è già installata su questo PC.",
  "installer.alreadyInstalled": "Già installato",
  "installer.installing": "Installazione in corso…",
  "installer.install": "Installa Moonpool",
  "installer.desktopShortcut": "Aggiungi un collegamento sul desktop",
  "installer.portableHint":
    "Esegui Moonpool da una cartella a tua scelta (chiavetta USB, zip) e spostala dove vuoi. I dati restano accanto all'exe.",
  "installer.installPortable": "Installa versione portatile",
  "installer.installPath": "Percorso di installazione: {dir}",

  "editor.addApp": "Aggiungi app",
  "editor.editApp": "Modifica app",
  "editor.backToList": "torna all'elenco",
  "editor.nameHint": "Il nome mostrato nella barra laterale. Obbligatorio.",
  "editor.namePlaceholder": "La mia app",
  "editor.groupHint":
    "L'intestazione della barra laterale sotto cui compare questa app. Scegli un gruppo esistente oppure «+ Nuovo gruppo» per crearne uno.",
  "editor.newGroupPlaceholder": "Nome del nuovo gruppo",
  "editor.pickExistingGroup": "Scegli un gruppo esistente",
  "editor.newGroupOption": "+ Nuovo gruppo...",
  "editor.typeHint":
    "Come Moonpool esegue e monitora l'app. web = server di sviluppo su una porta. desktop = app nativa individuata dal nome del processo. static = apre soltanto un URL. cli = esegue un comando in un terminale.",
  "editor.hintWeb":
    "Esegue un server di sviluppo in un terminale; la segnala in esecuzione quando la sua porta risponde e apre il browser non appena è attiva.",
  "editor.hintDesktop":
    "Avvia un'app nativa; la segnala in esecuzione quando trova un processo chiamato processName.",
  "editor.hintStatic": "Apre soltanto url nel browser, senza terminale né comando.",
  "editor.hintCli": "Esegue un comando e mantiene aperta una shell interattiva in cwd.",
  "editor.portableWarn":
    "Percorso assoluto: non seguirà questa cartella se la sposti. Usa {MP_HOME}\\... o un percorso ./ per mantenerla portatile.",
  "editor.notPortable": "non portatile",
  "editor.cwdHint":
    "La directory di lavoro in cui viene eseguito il comando, di solito la cartella del progetto. Usa un percorso assoluto, oppure {MP_HOME}\\... / ./ per restare portatile.",
  "editor.commandHint":
    "Il comando eseguito nel terminale integrato per avviare l'app, ad es. 'npm run dev' o 'python app.py'. Lascialo vuoto per una voce con solo URL statico.",
  "editor.portHint":
    "La porta TCP locale su cui l'app resta in ascolto. Moonpool la segnala in esecuzione quando questa porta risponde e la libera all'arresto. Usata dalle app web.",
  "editor.processNameHint":
    "Per le app desktop: il nome del processo o dell'eseguibile (senza .exe) usato per rilevare se è in esecuzione e per arrestarla. Su Linux non può superare i 15 caratteri.",
  "editor.urlHint":
    "L'URL da aprire: http://localhost:<port> per un'app web, oppure file:///path/to/index.html per una pagina statica. Usa file:///{MP_HOME}/... per restare portatile.",
  "editor.openBrowser": "apri il browser",
  "editor.openBrowserHint":
    "Apre automaticamente l'URL nel browser predefinito non appena l'app diventa raggiungibile.",
  "editor.envLabel": "env (KEY=VALUE per riga)",
  "editor.envHint":
    "Variabili d'ambiente passate al comando, una KEY=VALUE per riga (ad es. PORT=3000).",
  "editor.noteHint":
    "Testo facoltativo mostrato come suggerimento quando passi il cursore su questa app nella barra laterale.",
  "editor.notePlaceholder": "suggerimento facoltativo",
  "editor.phFolder": "percorso della cartella dell'app",
  "editor.phLaunchCommand": "il comando di avvio dell'app",
  "editor.phCommand": "comando da eseguire",
  "editor.discardChanges": "Vuoi annullare le modifiche?",
  "editor.nameRequired": "il nome è obbligatorio.",

  "term.processExited": "[processo terminato]",
  "term.copyAll": "Copia tutto",
  "term.copied": "Copiato",
};
