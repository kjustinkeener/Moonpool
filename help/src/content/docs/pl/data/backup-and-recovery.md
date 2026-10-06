---
title: "Kopia zapasowa Moonpool, wycofanie apps.json i odzyskiwanie konfiguracji"
description: "Dowiedz się, co archiwizować, jak cofnąć błędny apps.json, wrócić do przykładowych aplikacji, przenieść instalację do kopii przenośnej i co usuwa dezinstalacja."
---

Wszystko, co przechowuje Moonpool, znajduje się w dwóch miejscach: w folderze konfiguracji i w folderze pulpitów.
Ścieżki dla każdego trybu podano w sekcji [Gdzie znajduje się konfiguracja](/pl/apps/apps-json/#gdzie-znajduje-się-konfiguracja).

## Folder konfiguracji

```text
moonpool-config\
  apps.json            your apps                              back up
  apps.json.history\   the last 10 good apps.json files       back up (optional)
  settings.json        app settings                           back up
  icons\               icon overrides, <id>.png and so on     back up
  cli-output\<id>\     session logs                           disposable
  moonpool.log         debug log                              disposable
  state.json           live status snapshot                   disposable
  dumps\               files written by dump and read-config  disposable
  mcp_seen.json        which apps had an MCP helper           disposable
  window-state.json    hub window size and position           disposable
  AI-README.md         rewritten at every launch              disposable
  webview\             the window's browser profile (Windows) disposable
```

Folder pulpitów to `{MP_HOME}\dashboards`: `%USERPROFILE%\.moonpool\dashboards`
w instalacji, `<your .moonpool folder>\dashboards` w trybie przenośnym oraz `dashboards/` wewnątrz folderu
konfiguracji w systemie Linux. Należy archiwizować w nim wszystko, co jest własne. Jego folder `examples` należy do Moonpool
i jest przepisywany przy aktualizacji.

Motyw jest przechowywany w pamięci przeglądarki okna, a nie w pliku, który można skopiować. Nie
jest przenoszony wraz z kopią zapasową; po przywróceniu należy wybrać go ponownie.

## Tworzenie kopii zapasowej

1. Zamknij Moonpool, aby żaden plik nie był zapisany częściowo.
2. Skopiuj `apps.json`, `settings.json` i `icons\` z folderu konfiguracji oraz własne pliki
   z `dashboards\`.

```powershell frame="terminal"
$cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
Copy-Item "$cfg\apps.json", "$cfg\settings.json" D:\backup\
Copy-Item "$cfg\icons" D:\backup\ -Recurse
```

Aby przywrócić, zamknij Moonpool, skopiuj pliki z powrotem i uruchom go.

## Wycofanie apps.json

Każdy udany zapis, zapis agenta i przywrócenie oraz każde odświeżenie, które wykryje zmienioną zawartość,
kopiuje zwalidowany `apps.json` do `apps.json.history\`, zachowując 10 najnowszych. Każdy plik
nosi nazwę według czasu wykonania, na przykład `1767225600000.json`. Plik `apps.json.bak`
nie istnieje.

- **Ręcznie.** Skopiuj migawkę na `apps.json`, a następnie wybierz **Odśwież**.

  ```powershell frame="terminal"
  $cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
  Copy-Item "$cfg\apps.json.history\<snapshot>" "$cfg\apps.json"
  ```

- **Ze skryptu.** `moonpool.exe restore-config` wypisuje migawki;
  `moonpool.exe restore-config 1` przywraca najnowszą. Zob.
  [Wiersz poleceń](/pl/automation/command-line/).
- **Od agenta.** `moonpool_restore_config`. Zob. [Narzędzia MCP](/pl/automation/mcp-tools/#konfiguracja).

Nic nie jest przywracane automatycznie.

## Uszkodzony plik

- **apps.json.** Moonpool nigdy nie nadpisuje uszkodzonego pliku. Zob.
  [Jeśli plik jest błędny](/pl/apps/apps-json/#jeśli-plik-jest-błędny).
- **settings.json.** Popraw go albo usuń, aby zresetować wszystkie ustawienia, a następnie uruchom Moonpool ponownie. Zob.
  [settings.json](/pl/data/settings-json/#odczyt-i-naprawa).

## Powrót do przykładów

Moonpool zapisuje przykładowe aplikacje tylko wtedy, gdy nie ma `apps.json`. Aby zacząć od nowa, zamknij
Moonpool (lub zostaw go uruchomionego), zmień nazwę albo usuń `apps.json`, a następnie uruchom Moonpool lub wybierz
**Odśwież**. Zostanie zapisany nowy `apps.json` z przykładami.

## Z instalacji do trybu przenośnego

Nowa kopia przenośna zaczyna od przykładowych aplikacji. Aby przenieść własne, zob.
[Tryb przenośny](/pl/data/portable-mode/#wybór-trybu-przenośnego-w-instalatorze). W ten sam sposób skopiuj `icons\` i
`settings.json`, jeśli są potrzebne.

## Dezinstalacja

Dezinstalacja zainstalowanego Moonpool usuwa cały folder `%USERPROFILE%\.moonpool`,
w tym folder konfiguracji i pulpity. Najpierw wykonaj kopię zapasową. Zob.
[Dezinstalacja](/pl/getting-started/install/#odinstalowanie). Kopię przenośną usuwa się,
kasując jej folder `.moonpool\`.
