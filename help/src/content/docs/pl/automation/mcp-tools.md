---
title: "Opis narzędzi MCP Moonpool: parametry i wyniki"
description: "Każde narzędzie udostępniane agentom przez serwer MCP Moonpool, z jego parametrami, zwracanym wynikiem i przypadkami błędów, które mogą się pojawić."
---

Wszystkie narzędzia zwracają tekst, z wyjątkiem `moonpool_screenshot`, które zwraca obraz PNG. Niepowodzenie
wraca jako wynik narzędzia oznaczony jako błąd, z przyczyną w postaci tekstu. Konfigurację opisano w sekcji
[Konfiguracja MCP](/pl/automation/mcp-setup/).

Narzędzia przyjmujące `app_id` wymagają `id` aplikacji z `apps.json`. Może ono zawierać wyłącznie litery,
cyfry, `.`, `_` i `-` i nie może zaczynać się od `-`, w przeciwnym razie wywołanie kończy się błędem „invalid
app_id”.

Większość narzędzi działających na hubie kończy się tym komunikatem, gdy hub nie działa.
`moonpool_bootup_launcher`, `moonpool_shutdown_launcher`, `moonpool_raise_launcher` i
`moonpool_launcher_paths` same obsługują ten przypadek (zob. ich wiersze). W przypadku kopii przenośnej
komunikat podaje nazwę kopii, na przykład `Moonpool (<folder>)`.

```text
Moonpool is not running - call moonpool_bootup_launcher first
```

Wywołania, które czekają na wynik, przekraczają limit czasu po 45 sekundach.

## Program uruchamiający i aplikacje

Przykładowy wynik `moonpool_list_apps`:

```text
site  [running] (managed by Moonpool)  Site
notes-app  [stopped]  [mcp: stopped]  Notes App
```

| Narzędzie | Parametry | Działanie |
| --- | --- | --- |
| `moonpool_list_apps` | brak | Jeden wiersz na aplikację: `id  [running]` lub `[stopped]`, `(managed by Moonpool)`, gdy dotyczy, `[mcp: running]` lub `[mcp: stopped]`, gdy zaobserwowano proces pomocniczy MCP, a następnie nazwa. Zapytanie kierowane do działającego huba przez kanał sterowania (polecenie `list`), więc dane są aktualne. Jeśli Moonpool nie działa, kończy się błędem „Moonpool is not running”, zamiast pokazywać nieaktualną listę. Tuż po starcie Moonpool, przed pierwszym sprawdzeniem stanu, aplikacje pokazują `[status pending]`. Gdy `apps.json` zawiera błąd, wynik zaczyna się od `apps.json has an error: <message>. This list is the last one that loaded; fix the file and call moonpool_reload_config.` Jeśli plik był już uszkodzony przy starcie Moonpool, informuje, że żadne aplikacje nie są wczytane, i sugeruje także `moonpool_restore_config`. |
| `moonpool_bootup_launcher` | brak | Uruchamia sam Moonpool i czeka do 30 s, aż jego kanał sterowania odpowie. Zwraca „Moonpool started” lub „Moonpool is already running”. Jeśli nowy proces kończy się od razu (przekazał działanie Moonpool, który jeszcze się zamykał), uruchamia jeszcze jeden. Jeśli coś zajmuje kanał, nie odpowiadając, zgłasza, że proces Moonpool może być zawieszony. |
| `moonpool_shutdown_launcher` | brak | To samo co Zakończ w menu zasobnika systemowego. Czeka do 30 s, aż kanał sterowania zniknie. Zwraca „Moonpool shut down” lub „Moonpool is not running”. |
| `moonpool_raise_launcher` | brak | Wysuwa okno Moonpool na wierzch. Zwraca „window shown”. Jeśli Moonpool nie działa, uruchamia go i zwraca „Moonpool was not running; started it”. |
| `moonpool_start_app` | `app_id` (wymagane) | Uruchamia aplikację i otwiera jej kartę terminala. Zwraca „launched”, gdy działa, albo przyczynę niepowodzenia (`unknown app id: <id>`, `did not reach running in time` po 25 s). Dla wpisu `static` z samym `url` otwiera stronę i także zwraca „launched”. |
| `moonpool_stop_app` | `app_id` (wymagane) | Zatrzymuje aplikację. Zwraca „stopped” lub błąd, taki jak `still running after stop` (po 15 s). |
| `moonpool_restart_app` | `app_id` (wymagane) | Zatrzymuje, czeka na zwolnienie portu i procesu, uruchamia. Zwraca „restarted”. |
| `moonpool_app_output` | `app_id` (wymagane), `tail_lines` (liczba całkowita, domyślnie 200, minimum 1) | Wyjście terminala aplikacji z bieżącej sesji Moonpool, z usuniętymi kodami ANSI. Gdy dziennik jest dłuższy niż `tail_lines`, tekst zaczyna się wierszem podającym ścieżkę pełnego dziennika. Kończy się błędem `no console output recorded for '<id>' (not launched this session)`, jeśli aplikacja nie była uruchamiana. Jeśli dziennik istnieje, ale jest pusty, zwraca `(no output recorded for '<id>')`. |
| `moonpool_stop_mcp_server` | `app_id` (wymagane) | Kończy dołączony proces pomocniczy MCP aplikacji i pozostawia aplikację uruchomioną. Zwraca „stopped”. Nic nie robi, jeśli aplikacja nie ma ani `processName`, ani `mcpProcessName`. |
| `moonpool_refresh_app_icons` | brak | Ponownie pobiera ikony wszystkich aplikacji. Zwraca „icons refreshed”. |

## Konfiguracja

Te narzędzia odczytują i zmieniają `apps.json` przez hub, nigdy plik na dysku. Zapis musi
zawierać token z ostatniego odczytu, nieaktualny token jest odrzucany, a nowy plik jest walidowany,
zanim cokolwiek zostanie zapisane. Przejście przez hub ma znaczenie, ponieważ agentowi w hoście w piaskownicy
może zostać pokazana prywatna kopia folderu konfiguracji zamiast prawdziwej.

| Narzędzie | Parametry | Działanie |
| --- | --- | --- |
| `moonpool_read_config` | brak | Tekst JSON z `manifest_text` (dokładna zawartość pliku), `token`, `valid`, `error` (null, gdy poprawny) i `path`. `token` ma wartość `none`, gdy plik nie istnieje lub jest pusty. |
| `moonpool_write_config` | `manifest` (wymagane, pełny nowy tekst `apps.json`), `expected_token` (wymagane, z ostatniego odczytu) | Waliduje manifest i zastępuje `apps.json`, a następnie go wczytuje. Zwraca `apps.json updated; new version token <token>`. Nieaktualny token kończy się błędem `stale token: apps.json changed since it was read ...`. Nieprawidłowy manifest kończy się błędem `rejected invalid manifest: ...`. W obu przypadkach plik pozostaje nietknięty. Pusty `expected_token` jest odrzucany. |
| `moonpool_restore_config` | `snapshot` (opcjonalne) | Bez wartości: tekst JSON wymieniający zapisane migawki od najnowszej (`index`, `filename`, `millis`, `app_count`, `valid`). Z indeksem (1 = najnowsza) lub nazwą pliku waliduje tę migawkę i ją przywraca. Zwraca `restored <file> (<n> apps); new version token <token>`. Token nie jest potrzebny: przywrócenie celowo nadpisuje bieżący plik. |
| `moonpool_reload_config` | brak | Ponownie odczytuje `apps.json`. Zwraca „apps.json reloaded”. Jeśli plik nie daje się sparsować lub zwalidować, kończy się błędem `apps.json has an error: ...`, a Moonpool zachowuje ostatnio wczytaną listę. |
| `moonpool_launcher_paths` | brak | Wypisuje folder konfiguracji huba, `apps.json`, `state.json`, dziennik, folder zrzutów, folder ikon, znacznik trybu przenośnego i ścieżkę exe, a następnie folder konfiguracji procesu MCP, `apps.json`, `state.json`, folder zrzutów, znacznik trybu przenośnego i ścieżkę exe (bez dziennika i ikon). Jeśli hub nie działa, jego połowa brzmi `hub paths unavailable: ...`, a połowa MCP jest nadal pokazana. Należy go użyć, gdy zmiana nie odnosi skutku. |

## Zaawansowane: narzędzia testowe

`moonpool_screenshot` działa tylko w systemie Windows; w systemie Linux kończy się błędem „screenshot is not
supported on this platform”. `moonpool_window_state` i `moonpool_reset_mcp_seen` działają na
każdej platformie.

`window` to jedna z wartości `main`, `settings`, `about`, `installer`, `editor`, `help` lub `themes`, domyślnie
`main`. Nieznana nazwa kończy się błędem `unknown window '<name>'`.

| Narzędzie | Parametry | Działanie |
| --- | --- | --- |
| `moonpool_screenshot` | `window` (opcjonalne) | Przechwytuje zawartość okna Moonpool jako wbudowany PNG, najwyżej 320 pikseli na dłuższym boku. Rozmiaru nie można zwiększyć z poziomu MCP. Kończy się błędem `window '<name>' is not open`, jeśli okno nie jest widoczne. Nie może przechwycić żadnej innej aplikacji. |
| `moonpool_window_state` | `window` (opcjonalne) | Tekst JSON: `{"open":false}`, gdy okno nie jest otwarte, w przeciwnym razie `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. Przeznaczone do testów. |
| `moonpool_reset_mcp_seen` | `app_id` (opcjonalne) | Tylko do testów. Czyści zapamiętany zapis „zaobserwowano proces pomocniczy MCP” dla jednej aplikacji albo dla wszystkich, gdy pominięto, więc podwiersz MCP na pasku bocznym znów się ukrywa, dopóki proces pomocniczy nie zostanie zaobserwowany. |

## Zobacz też

- [Konfiguracja MCP](/pl/automation/mcp-setup/)
- [Wiersz poleceń](/pl/automation/command-line/)
