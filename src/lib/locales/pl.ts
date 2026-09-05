// Polski. Keys absent here fall back to English at runtime.
import type { PartialDict } from "../i18n.svelte";

export const pl: PartialDict = {
  "common.close": "Zamknij",
  "common.cancel": "Anuluj",
  "common.save": "Zapisz",
  "common.delete": "Usuń",
  "common.edit": "Edytuj",
  "common.rename": "Zmień nazwę",
  "common.reload": "Odśwież",
  "common.settings": "Ustawienia",
  "common.about": "O programie",
  "common.tryAgain": "Spróbuj ponownie",
  "common.dismiss": "Ukryj",
  "common.copied": "Skopiowano!",
  "common.autoSystem": "Automatycznie (system)",

  "titlebar.minimize": "Minimalizuj",
  "titlebar.maximize": "Maksymalizuj",
  "titlebar.restore": "Przywróć",

  "sidebar.menu": "Menu",
  "sidebar.addApp": "Dodaj aplikację",
  "sidebar.editJson": "Edytuj apps.json",
  "sidebar.installMoonpool": "Zainstaluj Moonpool…",
  "sidebar.filterPlaceholder": "Filtruj aplikacje...",
  "sidebar.filterLabel": "Filtruj aplikacje",
  "sidebar.showCli": "Pokaż panel CLI",
  "sidebar.updateOpenToInstall": "Dostępna aktualizacja, otwórz, aby zainstalować",
  "sidebar.updateShowCli": "Dostępna aktualizacja, pokaż panel CLI",
  "sidebar.statusRunning": "działa",
  "sidebar.statusStarting": "uruchamianie...",
  "sidebar.statusStopped": "zatrzymana",
  "sidebar.working": "Pracuję...",
  "sidebar.restart": "Uruchom ponownie",
  "sidebar.stop": "Zatrzymaj",
  "sidebar.launch": "Uruchom",
  "sidebar.setIcon": "Wybierz ikonę...",
  "sidebar.portConflict": "{names} używają tego samego portu {port}",
  "sidebar.portConflictBadge": "port {port}: {names}",
  "sidebar.noMatch": "Żadna aplikacja nie pasuje do „{filter}”.",

  "app.dragToResize": "Przeciągnij, aby zmienić rozmiar",
  "app.closeTab": "Zamknij kartę",
  "app.hideCli": "Ukryj panel CLI",
  "app.pickApp": "Wybierz aplikację po lewej, aby ją uruchomić.",
  "app.updateAvailable": "Moonpool {version} jest dostępny (masz {current}).",
  "app.downloading": "Pobieranie {version}…",
  "app.installing": "Instalowanie…",
  "app.downloadInstall": "Pobierz i zainstaluj",
  "app.updateInstalled": "Aktualizacja zainstalowana. Ponowne uruchamianie…",
  "app.updateFailed": "Aktualizacja nie powiodła się: {error}",
  "app.confirmDelete": "Usunąć „{name}”?",
  "app.iconDialogTitle": "Ikona dla {name}",
  "app.imagesFilter": "Obrazy",
  "app.newHere":
    "Pierwszy raz tutaj? Przekaż to agentowi AI, aby skonfigurował Twoje aplikacje:",
  "app.copyPrompt": "Kopiuj prompt",
  "app.editFile": "Edytuj plik",
  "app.orUseHint":
    "Albo użyj {add} powyżej lub {edit}, aby edytować {file} bezpośrednio.",

  "about.tagline":
    "Centrum uruchamiania w zasobniku systemowym dla lokalnych aplikacji i serwerów deweloperskich, z wbudowanym terminalem dla każdej aplikacji.",
  "about.version": "wersja {version}",
  "about.checkUpdates": "Sprawdź aktualizacje",
  "about.checking": "Sprawdzanie aktualizacji...",
  "about.upToDate": "Masz najnowszą wersję.",
  "about.updateDownloading": "Dostępna aktualizacja {version} - pobieranie...",
  "about.updateInstalled": "Aktualizacja zainstalowana. Ponowne uruchamianie...",
  "about.checkFailed": "Sprawdzanie aktualizacji nie powiodło się: {error}",
  "about.builtWith": "Zbudowano przy użyciu",
  "about.byLine": "MIT License · autor: {author}",

  "settings.title": "Ustawienia",
  "settings.theme": "Motyw",
  "settings.language": "Język",
  "settings.languageHint": "Język, w którym wyświetlane są teksty samego Moonpool.",
  "settings.closeToTray": "Zamykanie do zasobnika",
  "settings.closeToTrayHint":
    "Zamknięcie okna ukrywa Moonpool w zasobniku systemowym (znika też z paska zadań). Wyłączone: zamknięcie kończy pracę programu.",
  "settings.minimizeToTray": "Minimalizowanie do zasobnika",
  "settings.minimizeToTrayHint":
    "Minimalizacja ukrywa Moonpool w zasobniku systemowym (znika też z paska zadań). Wyłączone: minimalizuje do paska zadań.",
  "settings.alwaysOnTop": "Zawsze na wierzchu",
  "settings.alwaysOnTopHint":
    "Utrzymuje Moonpool oraz jego okna Ustawień i O programie nad innymi oknami.",
  "settings.transparency": "Przezroczystość tła",
  "settings.transparencyHint": "Prześwitujące tło okna. 0% oznacza pełne krycie.",
  "settings.checkOnStartup": "Sprawdzaj aktualizacje przy starcie",
  "settings.checkOnStartupHint":
    "Przy uruchomieniu dyskretnie sprawdza w serwisie GitHub, czy jest nowsza wersja, i pokazuje baner, jeśli ją znajdzie.",
  "settings.debugLogging": "Zapisuj informacje diagnostyczne do pliku",
  "settings.debugLoggingHint":
    "Rejestruje wczytywanie manifestu, uruchomienia i błędy w pliku {file}.",
  "settings.openLog": "Otwórz dziennik",

  "theme.dark": "Ciemny",
  "theme.light": "Jasny",

  "installer.tagline":
    "Program uruchamiający w zasobniku dla Twoich lokalnych aplikacji i serwerów deweloperskich.",
  "installer.pickFolder": "Wybierz folder dla przenośnego Moonpool",
  "installer.poke": "kliknij lewym, aby szturchnąć, prawym, aby zresetować",
  "installer.installed": "Zainstalowano, uruchamianie Moonpool…",
  "installer.portableDone": "Tryb przenośny, uruchamianie Moonpool…",
  "installer.failed": "Instalacja nie powiodła się: {error}",
  "installer.alreadyInstalledTitle": "Ta kopia jest już zainstalowana na tym komputerze.",
  "installer.alreadyInstalled": "Już zainstalowano",
  "installer.installing": "Instalowanie…",
  "installer.install": "Zainstaluj Moonpool",
  "installer.desktopShortcut": "Dodaj skrót na pulpicie",
  "installer.portableHint":
    "Uruchamiaj Moonpool z wybranego przez siebie folderu (pendrive, archiwum zip) i przenoś go dowolnie. Dane pozostają obok pliku .exe.",
  "installer.installPortable": "Instalacja przenośna",
  "installer.installPath": "Ścieżka instalacji: {dir}",

  "editor.addApp": "Dodaj aplikację",
  "editor.editApp": "Edytuj aplikację",
  "editor.backToList": "powrót do listy",
  "editor.nameHint": "Nazwa wyświetlana na pasku bocznym. Pole wymagane.",
  "editor.namePlaceholder": "Moja aplikacja",
  "editor.groupHint":
    "Nagłówek na pasku bocznym, pod którym znajdzie się ta aplikacja. Wybierz istniejącą grupę albo „+ Nowa grupa”, aby dodać nową.",
  "editor.newGroupPlaceholder": "Nazwa nowej grupy",
  "editor.pickExistingGroup": "Wybierz istniejącą grupę",
  "editor.newGroupOption": "+ Nowa grupa...",
  "editor.typeHint":
    "Sposób, w jaki Moonpool uruchamia i śledzi aplikację. web = serwer deweloperski na porcie. desktop = aplikacja natywna śledzona po nazwie procesu. static = tylko otwiera adres URL. cli = uruchamia polecenie w terminalu.",
  "editor.hintWeb":
    "Uruchamia serwer deweloperski w terminalu; pokazuje stan „działa”, gdy jego port odpowiada, i otwiera przeglądarkę, gdy serwer jest gotowy.",
  "editor.hintDesktop":
    "Uruchamia aplikację natywną; pokazuje stan „działa”, gdy znajdzie proces o nazwie processName.",
  "editor.hintStatic": "Tylko otwiera url w przeglądarce, bez terminala i bez polecenia.",
  "editor.hintCli": "Uruchamia polecenie i utrzymuje otwartą interaktywną powłokę w cwd.",
  "editor.portableWarn":
    "Ścieżka bezwzględna - nie przeniesie się razem z tym folderem. Użyj {MP_HOME}\\... lub ścieżki ./, aby zachować przenośność.",
  "editor.notPortable": "nieprzenośna",
  "editor.cwdHint":
    "Katalog roboczy, w którym uruchamiane jest polecenie - zwykle folder projektu aplikacji. Podaj ścieżkę bezwzględną albo {MP_HOME}\\... / ./, aby zachować przenośność.",
  "editor.commandHint":
    "Polecenie uruchamiane we wbudowanym terminalu, aby wystartować aplikację, np. 'npm run dev' lub 'python app.py'. Zostaw puste dla wpisu z samym adresem URL.",
  "editor.portHint":
    "Lokalny port TCP, na którym nasłuchuje aplikacja. Moonpool pokazuje stan „działa”, gdy ten port odpowiada, i zwalnia go po zatrzymaniu. Używany przez aplikacje typu web.",
  "editor.processNameHint":
    "Dla aplikacji desktopowych: nazwa procesu lub pliku wykonywalnego (bez .exe) używana do wykrywania działania i zatrzymywania aplikacji. W systemie Linux może mieć najwyżej 15 znaków.",
  "editor.urlHint":
    "Adres URL do otwarcia: http://localhost:<port> dla aplikacji web albo file:///path/to/index.html dla strony statycznej. Użyj file:///{MP_HOME}/..., aby zachować przenośność.",
  "editor.openBrowser": "otwórz przeglądarkę",
  "editor.openBrowserHint":
    "Automatycznie otwiera adres URL w domyślnej przeglądarce, gdy aplikacja stanie się dostępna.",
  "editor.envLabel": "env (KEY=VALUE w wierszu)",
  "editor.envHint":
    "Zmienne środowiskowe przekazywane do polecenia, po jednym KEY=VALUE w wierszu (np. PORT=3000).",
  "editor.noteHint":
    "Opcjonalny tekst wyświetlany jako podpowiedź po najechaniu na tę aplikację na pasku bocznym.",
  "editor.notePlaceholder": "opcjonalna podpowiedź",
  "editor.phFolder": "ścieżka do folderu aplikacji",
  "editor.phLaunchCommand": "polecenie uruchamiające aplikację",
  "editor.phCommand": "polecenie do uruchomienia",
  "editor.discardChanges": "Odrzucić wprowadzone zmiany?",
  "editor.nameRequired": "nazwa jest wymagana.",

  "term.processExited": "[proces zakończony]",
  "term.copyAll": "Kopiuj wszystko",
  "term.copied": "Skopiowano",
};
