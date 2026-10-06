---
title: "Objaśnienie komunikatów o błędach Moonpool: already running, requires a command i inne"
description: "Sprawdź dokładny tekst komunikatów o błędach Moonpool, takich jak already running, requires a command, stale token i Aktualizacja nie powiodła się, wraz ze znaczeniem i rozwiązaniem."
---

Należy wkleić zobaczony komunikat w wyszukiwarkę strony albo przejrzeć tabele. Komunikaty są
cytowane tak, jak pokazuje je Moonpool. Tekst w `<nawiasach ostrych>` jest zastępowany wartością
(identyfikatorem aplikacji, ścieżką lub błędem z systemu). Objawy, które nie są komunikatem o
błędzie, opisano w [Rozwiązywaniu problemów i FAQ](/pl/support/troubleshooting/).

## Uruchamianie i zatrzymywanie aplikacji

| Komunikat | Znaczenie i rozwiązanie |
| --- | --- |
| `already running` | Moonpool ma już terminal dla tej aplikacji. Należy ją najpierw zatrzymać albo użyć Uruchom ponownie. |
| `stopped during launch` | Naciśnięto Zatrzymaj, gdy uruchamianie jeszcze trwało. Należy uruchomić ponownie. |
| `app has no launch command` | Wpis nie ma `command`. Należy dodać je w edytorze aplikacji lub w `apps.json`. Bez niego może się obejść tylko wpis `static` z `url`. |
| `unknown app: <id>` | Nie wczytano aplikacji o takim `id`. Należy sprawdzić identyfikator, a po ręcznej edycji `apps.json` użyć Odśwież. |
| `unknown app id: <id>` | Ten sam problem, zgłoszony skryptowi lub agentowi. Aplikacje można wylistować przez `moonpool_list_apps`. |
| `did not reach running in time` | Ze skryptu lub od agenta: aplikacja nie została rozpoznana jako działająca w ciągu 25 sekund. Należy sprawdzić `port` lub `processName` i przeczytać wyjście. Zob. [Kropka stanu jest nieprawidłowa](/pl/support/troubleshooting/#kropka-stanu-jest-nieprawidłowa). |
| `still running after stop` | Po 15 sekundach aplikacja nadal jest rozpoznawana jako działająca. Należy ustawić `killMode`. Zob. [Zatrzymywanie i ponowne uruchamianie](/pl/apps/stop-and-restart/). |
| `refusing to open non-web url: <url>` | `url` nie zaczyna się od `http://`, `https://`, `mailto:` ani `file://`. Należy poprawić `url`. |
| `[proces zakończony]` | To nie błąd: polecenie aplikacji się zakończyło. Pokazywane w karcie terminala. |

## Walidacja apps.json

Moonpool odrzuca `apps.json`, który łamie regułę, i zachowuje ostatnią wczytaną listę.
`<n>` to pozycja wpisu w pliku, licząc od 1.

| Komunikat | Rozwiązanie |
| --- | --- |
| `apps.json entry <n> has invalid id "<id>"; use letters, digits, '.', '_', and '-' without a leading '-'` | Należy zmienić `id`. |
| `duplicate app id "<id>"` | Dwa wpisy mają wspólny `id`. Każdy musi być unikalny. |
| `apps.json entry <n> (<id>) has an empty name` | Należy uzupełnić `name`. |
| `apps.json entry <n> (<id>) has an empty group` | Należy uzupełnić `group`. |
| `apps.json entry <n> (<id>) has unknown type "<type>"` | `type` musi mieć wartość `web`, `desktop`, `static` lub `cli`. |
| `apps.json entry <n> (<id>) has invalid port 0` | `port` musi mieścić się w zakresie od 1 do 65535. |
| `apps.json entry <n> (<id>) requires a url` | Wpis `static` wymaga `url`. |
| `apps.json entry <n> (<id>) requires a command` | Każdy inny typ wymaga `command`. |

W edytorze aplikacji zapis bez nazwy pokazuje `nazwa jest wymagana.`.
Tekst banera, „apps.json zawiera błąd; wyświetlana jest ostatnio wczytana lista.” lub „apps.json
zawiera błąd, więc nie wczytano żadnych aplikacji.”, oraz sposób naprawy opisano w
[apps.json zawiera błąd](/pl/support/troubleshooting/#appsjson-zawiera-błąd). Jeśli baner informuje,
że zapisywanie jest wstrzymane, komunikat kończy się słowami `Repair apps.json and reload it before saving from
Moonpool`. Pełną listę reguł podano w [Walidacji](/pl/apps/apps-json/#walidacja).

## Ustawienia, aktualizacje i instalator

| Komunikat | Znaczenie i rozwiązanie |
| --- | --- |
| `... Repair settings.json and restart Moonpool before changing settings` | `settings.json` jest uszkodzony. Należy go naprawić lub usunąć i uruchomić Moonpool ponownie. Zob. [settings.json](/pl/data/settings-json/#odczyt-i-naprawa). |
| `Aktualizacja nie powiodła się: <error>` | Pobranie lub instalacja aktualizacji nie powiodły się. Zob. [Gdy aktualizacja się nie powiedzie](/pl/data/updating/#gdy-aktualizacja-się-nie-powiedzie). |
| `Sprawdzanie aktualizacji nie powiodło się: <error>` | Sprawdzanie aktualizacji w oknie O programie nie powiodło się. Tekst po dwukropku podaje powód. Należy spróbować później. |
| `Instalacja nie powiodła się: <error>` | Instalator zatrzymał się na kroku nazwanym po dwukropku, na przykład `copy exe: ...`. Należy zakończyć każdy Moonpool uruchomiony z `%USERPROFILE%\.moonpool` i spróbować ponownie. |
| `target folder does not exist` | Folder wybrany dla kopii przenośnej już nie istnieje. Należy wybrać istniejący. |

## MCP i skrypty

| Komunikat | Znaczenie i rozwiązanie |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Należy uruchomić Moonpool albo pozwolić agentowi wywołać to narzędzie. Dla kopii przenośnej komunikat podaje nazwę kopii. |
| `frontend not loaded` | Okno huba nie zakończyło jeszcze wczytywania. Należy poczekać i spróbować ponownie. |
| `invalid app_id: use only letters, digits, '.', '_', '-' (no leading '-')` | Agent przekazał identyfikator, którego serwer MCP nie akceptuje. Należy użyć identyfikatora z `moonpool_list_apps`. |
| `stale token: apps.json changed since it was read ...` | Należy ponownie odczytać `apps.json`, ponownie zastosować zmianę, a potem zapisać. |
| `rejected invalid manifest: ...` | Nowy `apps.json` nie przeszedł walidacji (zob. wyżej). Plik nie został zmieniony. |
| `no console output recorded for '<id>' (not launched this session)` | O `moonpool_app_output` zapytano dla aplikacji, która nie działała od uruchomienia Moonpool. |

Więcej w [Narzędziach MCP](/pl/automation/mcp-tools/) i
[Konfiguracji MCP](/pl/automation/mcp-setup/#gdy-narzędzia-nie-działają).

## Błędy z innych programów

- [`Error: listen EADDRINUSE: address already in use :::3000` i `Port 5173 is in use`](/pl/support/port-already-in-use/)
- [`Windows protected your PC`](/pl/support/windows-protected-your-pc/)
- [Brak środowiska uruchomieniowego WebView2](/pl/support/webview2-runtime-missing/)
