// English - the source catalog. Every other locale is typed against this one
// and falls back to it key by key, so this file is also the schema.
//
// WHAT IS DELIBERATELY NOT IN HERE
//
// Localizing the wrong string is worse than leaving it English, because it
// silently breaks something. The judgement calls, all of them made once here:
//
//  - **apps.json field labels** ("name", "group", "cwd", "command", "port",
//    "processName", "url", "env", "note"). They are the literal JSON keys the
//    user types into the file the editor is a front end for. A translated label
//    beside an untranslatable key is a worse experience, not a better one. The
//    *hints* next to them are translated; the identifiers are not.
//  - **The type values** "web" / "desktop" / "static" / "cli", same reason: they
//    are config values, not words.
//  - **Default group names** ("Web apps", "Docs", ...). They are seeded *data* -
//    they get written into apps.json and become the user's own group names. If
//    they were localized, switching language later would leave headings in the
//    old language with no way to tell they came from us.
//  - **The AI setup prompt** in App.svelte. It is addressed to an agent, not a
//    person, and it cites English filenames and an English guide.
//  - **Control-channel replies** written to state.json ("opened", "stopped",
//    "unknown command: ..."). That is a machine-readable protocol for scripts.
//    Localizing an API breaks every caller.
//  - **Named theme palettes** (Nord, Dracula, Gruvbox, ...). Proper nouns. Only
//    the three generic ones - auto, dark, light - are translated.
//  - **Technical placeholder examples** ("npm run dev", "5173", "PORT=3000",
//    "http://localhost:3000"). Prose placeholders like "path to the app folder"
//    ARE translated; sample commands are not.
//  - **Filenames, URLs, the brand, the author, "MIT License".**
//
// `{MP_HOME}` inside editor.portableWarn / editor.cwdHint / editor.urlHint is
// LITERAL TEXT the user types, not a slot. It survives translation because
// interpolation leaves unknown placeholders untouched - but a translator must
// copy it through verbatim.

export const en = {
  // -- shared ---------------------------------------------------------------
  "common.close": "Close",
  "common.cancel": "Cancel",
  "common.save": "Save",
  "common.delete": "Delete",
  "common.edit": "Edit",
  "common.rename": "Rename",
  "common.reload": "Reload",
  "common.settings": "Settings",
  "common.about": "About",
  "common.tryAgain": "Try again",
  "common.dismiss": "Dismiss",
  "common.copied": "Copied!",
  "common.autoSystem": "Auto (system)",

  // -- title bar ------------------------------------------------------------
  "titlebar.minimize": "Minimize",
  "titlebar.maximize": "Maximize",
  "titlebar.restore": "Restore",

  // -- sidebar --------------------------------------------------------------
  "sidebar.menu": "Menu",
  "sidebar.addApp": "Add app",
  "sidebar.editJson": "Edit apps.json",
  "sidebar.installMoonpool": "Install Moonpool…",
  "sidebar.filterPlaceholder": "Filter apps...",
  "sidebar.filterLabel": "Filter apps",
  "sidebar.showCli": "Show CLI pane",
  "sidebar.updateOpenToInstall": "Update available, open to install",
  "sidebar.updateShowCli": "Update available, show CLI pane",
  "sidebar.statusRunning": "running",
  "sidebar.statusStarting": "starting...",
  "sidebar.statusStopped": "stopped",
  "sidebar.working": "Working...",
  "sidebar.restart": "Restart",
  "sidebar.stop": "Stop",
  "sidebar.launch": "Launch",
  "sidebar.setIcon": "Set icon...",
  // {names} is already a formatted list ("A and B") - see Intl.ListFormat use.
  "sidebar.portConflict": "{names} are both on port {port}",
  "sidebar.portConflictBadge": "port {port}: {names}",
  "sidebar.noMatch": "No apps match “{filter}”.",

  // -- hub ------------------------------------------------------------------
  "app.dragToResize": "Drag to resize",
  "app.closeTab": "Close tab",
  "app.hideCli": "Hide CLI pane",
  "app.pickApp": "Pick an app on the left to launch it.",
  "app.updateAvailable": "Moonpool {version} is available (you have {current}).",
  "app.downloading": "Downloading {version}…",
  "app.installing": "Installing…",
  "app.downloadInstall": "Download & install",
  "app.updateInstalled": "Update installed. Restarting…",
  "app.updateFailed": "Update failed: {error}",
  "app.confirmDelete": "Delete “{name}”?",
  "app.iconDialogTitle": "Icon for {name}",
  "app.imagesFilter": "Images",
  "app.newHere": "New here? Hand this to an AI agent to set up your apps:",
  "app.copyPrompt": "Copy prompt",
  "app.editFile": "Edit file",
  // Split-sentence key: {add} and {edit} are rendered as styled inline elements.
  // Kept as ONE string so translators can move the clauses; see tSplit().
  "app.orUseHint": "Or use {add} above, or {edit} to edit {file} directly.",

  // -- about ----------------------------------------------------------------
  "about.tagline":
    "A system-tray launcher hub for local apps and dev servers, with an embedded terminal per app.",
  "about.version": "version {version}",
  "about.checkUpdates": "Check for updates",
  "about.checking": "Checking for updates...",
  "about.upToDate": "You're on the latest version.",
  "about.updateDownloading": "Update {version} available - downloading...",
  "about.updateInstalled": "Update installed. Restarting...",
  "about.checkFailed": "Update check failed: {error}",
  "about.builtWith": "Built with",
  "about.byLine": "MIT License · by {author}",

  // -- settings -------------------------------------------------------------
  "settings.title": "Settings",
  "settings.theme": "Theme",
  "settings.language": "Language",
  "settings.languageHint": "The language Moonpool's own text is shown in.",
  "settings.closeToTray": "Close to tray",
  "settings.closeToTrayHint":
    "Closing the window hides Moonpool to the tray (leaves the taskbar). Off: closing quits.",
  "settings.minimizeToTray": "Minimize to tray",
  "settings.minimizeToTrayHint":
    "Minimizing hides Moonpool to the tray (leaves the taskbar). Off: minimizes to the taskbar.",
  "settings.alwaysOnTop": "Always on top",
  "settings.alwaysOnTopHint":
    "Keep Moonpool and its Settings/About windows above other windows.",
  "settings.transparency": "Background transparency",
  "settings.transparencyHint": "See-through window background. 0% is solid.",
  "settings.checkOnStartup": "Check for updates on startup",
  "settings.checkOnStartupHint":
    "On launch, quietly checks GitHub for a newer version and shows a banner if one is found.",
  "settings.debugLogging": "Log debug info to a file",
  "settings.debugLoggingHint": "Records manifest loads, launches, and errors to {file}.",
  "settings.openLog": "Open log",

  // -- theme picker (only the generic three; named palettes are proper nouns) -
  "theme.dark": "Dark",
  "theme.light": "Light",

  // -- installer ------------------------------------------------------------
  "installer.tagline": "A tray launcher for your local apps & dev servers.",
  "installer.pickFolder": "Choose a folder for portable Moonpool",
  "installer.poke": "left-click to poke, right-click to reset",
  "installer.installed": "Installed, starting Moonpool…",
  "installer.portableDone": "Portable mode, starting Moonpool…",
  "installer.failed": "Install failed: {error}",
  "installer.alreadyInstalledTitle": "This copy is already installed on this PC.",
  "installer.alreadyInstalled": "Already installed",
  "installer.installing": "Installing…",
  "installer.install": "Install Moonpool",
  "installer.desktopShortcut": "Add a desktop shortcut",
  "installer.portableHint":
    "Run Moonpool from a folder you choose (USB stick, zip) - move it anywhere. Data stays beside the exe.",
  "installer.installPortable": "Install portable",
  "installer.installPath": "Install path: {dir}",

  // -- app editor -----------------------------------------------------------
  "editor.addApp": "Add app",
  "editor.editApp": "Edit app",
  "editor.backToList": "back to list",
  "editor.nameHint": "The display name shown in the sidebar. Required.",
  "editor.namePlaceholder": "My App",
  "editor.groupHint":
    "The sidebar heading this app is listed under. Pick an existing group or choose '+ New group' to add one.",
  "editor.newGroupPlaceholder": "New group name",
  "editor.pickExistingGroup": "Pick an existing group",
  "editor.newGroupOption": "+ New group...",
  "editor.typeHint":
    "How Moonpool runs and tracks the app. web = dev server on a port. desktop = native app tracked by process name. static = just opens a URL. cli = runs a command in a terminal.",
  "editor.hintWeb":
    "Runs a dev server in a terminal; shows Running when its port answers, and opens the browser once it's live.",
  "editor.hintDesktop":
    "Launches a native app; shows Running when a process named processName is found.",
  "editor.hintStatic": "Just opens url in the browser, no terminal or command.",
  "editor.hintCli": "Runs a command and keeps an interactive shell open in cwd.",
  // {MP_HOME} below is literal text the user types, NOT a slot. Copy verbatim.
  "editor.portableWarn":
    "Absolute path - won't move with this folder. Use {MP_HOME}\\... or a ./ path to keep it portable.",
  "editor.notPortable": "not portable",
  "editor.cwdHint":
    "The working directory the command runs in - usually the app's project folder. Use an absolute path, or {MP_HOME}\\... / ./ to stay portable.",
  "editor.commandHint":
    "The command run in the embedded terminal to start the app, e.g. 'npm run dev' or 'python app.py'. Leave blank for a static URL-only entry.",
  "editor.portHint":
    "The local TCP port the app listens on. Moonpool shows Running when this port answers, and frees it on Stop. Used by web apps.",
  "editor.processNameHint":
    "For desktop apps: the process/executable name (without .exe) used to detect Running and to stop it. On Linux it must be 15 characters or fewer.",
  "editor.urlHint":
    "The URL to open: http://localhost:<port> for a web app, or file:///path/to/index.html for a static page. Use file:///{MP_HOME}/... to stay portable.",
  "editor.openBrowser": "open browser",
  "editor.openBrowserHint":
    "Automatically open the URL in your default browser when the app becomes reachable.",
  "editor.envLabel": "env (KEY=VALUE per line)",
  "editor.envHint":
    "Environment variables passed to the command, one KEY=VALUE per line (e.g. PORT=3000).",
  "editor.noteHint": "Optional text shown as a tooltip when you hover this app in the sidebar.",
  "editor.notePlaceholder": "optional tooltip",
  "editor.phFolder": "path to the app folder",
  "editor.phLaunchCommand": "the app's launch command",
  "editor.phCommand": "command to run",
  "editor.discardChanges": "Discard your changes?",
  "editor.nameRequired": "name is required.",

  // -- terminal -------------------------------------------------------------
  "term.processExited": "[process exited]",
  "term.copyAll": "Copy all",
  "term.copied": "Copied",
} as const;
