---
title: "Edycja apps.json: gdzie się znajduje, jak go odświeżyć i odzyskać"
description: "Znajdź plik apps.json, który Moonpool odczytuje dla każdej zarządzanej aplikacji, edytuj go w edytorze lub ręcznie, odśwież i odzyskaj po błędnej zmianie."
---

Każda aplikacja zarządzana przez Moonpool to jeden wpis w `apps.json`. Można go edytować w edytorze aplikacji
(okno Dodaj aplikację i Edytuj aplikację) lub ręcznie. Obie metody zapisują ten sam plik. W niektórych wynikach narzędzi
i komunikatach plik ten nazywany jest manifestem.

## Gdzie znajduje się konfiguracja

| Tryb | Folder konfiguracji |
| --- | --- |
| Zainstalowany (Windows) | `%USERPROFILE%\.moonpool\moonpool-config\` |
| Przenośny | `moonpool-config\` obok `moonpool.exe` (wewnątrz folderu `.moonpool\`) |
| Linux | `$XDG_CONFIG_HOME/Moonpool/`, a w razie braku `~/.config/Moonpool/` |

`apps.json` znajduje się w tym folderze, obok następujących elementów:

| Element | Przeznaczenie |
| --- | --- |
| `apps.json.history\` | Kolejka wycofywania z ostatnimi 10 poprawnymi plikami `apps.json`. |
| `settings.json` | Ustawienia aplikacji. Zob. [settings.json](/pl/data/settings-json/). |
| `cli-output\<id>\` | Dzienniki sesji poszczególnych aplikacji. Zob. [Dzienniki](/pl/data/logs/). |
| `moonpool.log` | Dziennik diagnostyczny, gdy włączona jest opcja **Zapisuj informacje diagnostyczne do pliku**. |
| `icons\` | Opcjonalne zastępcze ikony `<id>.png` (także `.ico`, `.svg`, `.jpg`, `.jpeg`, `.webp`). |
| `state.json` | Bieżąca migawka stanu, odświeżana co kilka sekund. |
| `dumps\` | Pliki zapisywane przez polecenia `dump`, `read-config` i `restore-config`. |
| `mcp_seen.json` | Informacja, które aplikacje miały pomocnika MCP. |
| `window-state.json` | Rozmiar i położenie okna huba. |
| `AI-README.md` | Przewodnik dla agentów AI, przepisywany przy każdym uruchomieniu. |

Które z tych elementów warto archiwizować, opisano w [Kopii zapasowej i odzyskiwaniu](/pl/data/backup-and-recovery/#folder-konfiguracji).

Przy pierwszym uruchomieniu Moonpool tworzy `apps.json` z przykładowymi wpisami. Istniejący plik
nigdy nie jest nadpisywany.

## Edycja

- **Okno dialogowe.** Użyj **Dodaj aplikację** w menu **...** u góry paska bocznego. Aby zmienić
  aplikację, użyj ołówka w jej wierszu albo kliknij ją prawym przyciskiem myszy i wybierz **Edytuj**. Okno
  sprawdza poprawność i zapisuje od razu.
- **Ręcznie.** **Edytuj apps.json** w tym samym menu otwiera plik w domyślnym edytorze.
  Zapisz go, a następnie wybierz **Odśwież** w menu (lub naciśnij F5 albo Ctrl+R).

Ręczne zmiany nie są uwzględniane, dopóki plik nie zostanie odświeżony. Odświeżenie tylko odczytuje plik; nie
zapisuje go ponownie.

Zapis z okna dialogowego przepisuje cały plik w znormalizowanej, wciętej postaci. Klucze nieznane
Moonpool są usuwane, a JSON nie ma komentarzy, więc notatki należy trzymać w polu `note`.

## Struktura

Plik jest tablicą JSON obiektów. Cztery klucze są wymagane w każdym wpisie: `id`, `name`,
`group`, `type`. Wszystko inne jest opcjonalne. Zob. [Pola aplikacji](/pl/apps/fields/).

```json title="apps.json"
[
  { "id": "site", "name": "Site", "group": "Web apps", "type": "web",
    "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
    "url": "http://localhost:5173", "openBrowser": true }
]
```

Grupy pojawiają się na pasku bocznym w kolejności, w jakiej po raz pierwszy występują w pliku.

## Co robi odświeżenie

Odświeżenie zastępuje listę w pamięci Moonpool zawartością pliku. Uruchom, Zatrzymaj i Uruchom ponownie
odczytują wpis w chwili kliknięcia, więc zmieniony `command`, `cwd`, `env` lub ustawienie kończenia
zaczyna obowiązywać przy następnym uruchomieniu lub ponownym uruchomieniu tej aplikacji. Odświeżenie niczego nie uruchamia ponownie:
aplikacja, która już działa, nadal działa z ustawieniami, z którymi wystartowała.

## Walidacja

Moonpool sprawdza cały plik przy wczytywaniu, przy każdym zapisie i przy każdym zapisie agenta.
Jeden błędny wpis powoduje odrzucenie całego pliku.

| Reguła | Błąd zawiera |
| --- | --- |
| Nieprawidłowy JSON, brak wymaganego klucza lub wartość niewłaściwego typu | komunikat parsera JSON |
| `id` jest puste, zaczyna się od `-` albo zawiera znaki inne niż litery, cyfry, `.`, `_`, `-` | `invalid id` |
| Dwa wpisy mają to samo `id` | `duplicate app id` |
| `name` jest puste | `has an empty name` |
| `group` jest puste | `has an empty group` |
| `type` nie jest równe `desktop`, `web`, `static` ani `cli` | `unknown type` |
| `port` wynosi `0` (`port` powyżej 65535 nie daje się sparsować) | `invalid port 0` |
| Wpis `static` bez `url` | `requires a url` |
| Dowolny inny typ bez `command` | `requires a command` |

Błędy wskazują wpis według pozycji, na przykład:

```text
apps.json entry 2 (site) requires a command
```

### Identyfikator id

`id` jest trwałym kluczem wpisu. Nazywa folder dziennika i plik ikony oraz jest tym, co
przekazuje się do `moonpool.exe launch <id>` i agentom. Okno dialogowe tworzy go z nazwy
przy dodawaniu aplikacji. Zamienia nazwę na małe litery, każdy ciąg
znaków innych niż `a` do `z` i `0` do `9` zamienia na jeden `-` i usuwa `-` z obu
końców. Pusty wynik zamienia na `app`. Jeśli id jest zajęte, dodaje `-2`, `-3` i tak dalej.
Później już go nie zmienia, więc zmiana nazwy aplikacji zachowuje jej id. Nazwa `Habit Tracker` otrzymuje id
`habit-tracker`.

## Jeśli plik jest błędny

- **Przy odświeżaniu** plik, który nie przechodzi walidacji, pozostaje nietknięty, a Moonpool zachowuje ostatnią
  wczytaną listę. Baner nad paskiem bocznym pokazuje błąd i przycisk otwierający
  plik; lista pozostaje użyteczna, ale przyciemniona. Zob.
  [Gdy apps.json zawiera błąd](/pl/using/hub-window/#gdy-appsjson-zawiera-błąd).
- **Przy starcie** uszkodzony plik oznacza brak listy do zachowania, więc Moonpool startuje bez
  aplikacji, o czym informuje baner. Popraw plik i wybierz **Odśwież** albo przywróć migawkę
  (poniżej lub narzędziem `moonpool_restore_config`).
- W obu przypadkach zapisy z okna dialogowego (oraz zmiana nazwy, usunięcie, ustawienie ikony) są odrzucane, dopóki
  plik się nie wczyta, więc uszkodzony plik nigdy nie zostanie nadpisany. Popraw plik i wybierz
  **Odśwież**.
- **Z okna dialogowego, od agenta lub przy przywracaniu** nieprawidłowa zmiana jest odrzucana, a plik
  na dysku pozostaje bez zmian.

Moonpool przechowuje ostatnie 10 poprawnych wersji `apps.json` w `apps.json.history\`. Jak wrócić
do wcześniejszej wersji, opisano w [Kopii zapasowej i odzyskiwaniu](/pl/data/backup-and-recovery/#wycofanie-appsjson).
Objawy i rozwiązania znajdują się w [Rozwiązywaniu problemów](/pl/support/troubleshooting/#appsjson-zawiera-błąd).

## Agenci

Agent AI powinien zmieniać `apps.json` za pomocą narzędzi MCP Moonpool, a nie przez plik, aby
nieaktualny lub nieprawidłowy zapis został odrzucony, a agent w piaskownicy nigdy nie edytował prywatnej kopii. Zob.
[Narzędzia MCP](/pl/automation/mcp-tools/#konfiguracja).
