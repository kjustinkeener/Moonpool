---
title: "Kanał sterowania Moonpool i opis poleceń"
description: "Jak działa kanał sterowania Moonpool (potok nazwany lub gniazdo Unix), jego protokół oraz każde polecenie, na które odpowiada działająca aplikacja, z argumentami i odpowiedziami."
---

## Gdzie nasłuchuje

Każda kopia Moonpool ma własny kanał, więc zainstalowany Moonpool i dowolne kopie przenośne
mogą działać obok siebie, nie odpowiadając za siebie nawzajem. W systemie Windows zainstalowany Moonpool
nasłuchuje na potoku nazwanym `\\.\pipe\moonpool`. Kopia przenośna dodaje identyfikator utworzony z jej
folderu: `\\.\pipe\moonpool-<id>`.

`<id>` to 8 cyfr szesnastkowych wyprowadzonych ze ścieżki folderu `moonpool-config` kopii, więc pozostaje
takie samo dla tego folderu po ponownych uruchomieniach i aktualizacjach, a zmienia się po przeniesieniu folderu.
`moonpool.exe` danej kopii, w tym `moonpool.exe mcp`, zawsze znajduje kanał własnej kopii.

W systemach Linux i macOS nasłuchuje natomiast na gnieździe domeny Unix z trybem `0600`:

| Przypadek | Ścieżka gniazda |
| --- | --- |
| Zwykły | `$XDG_RUNTIME_DIR/moonpool.sock`, gdy ta zmienna jest ustawiona, w przeciwnym razie `moonpool.sock` w folderze konfiguracji Moonpool |
| Tryb przenośny | `moonpool.sock` w folderze konfiguracji kopii przenośnej, więc kopia przenośna nigdy nie koliduje z zainstalowaną |
| Ścieżka zbyt długa dla gniazda (około 100 znaków) | `/tmp/moonpool-<uid>/moonpool.sock`, w katalogu, który może otworzyć tylko użytkownik (`moonpool-<id>.sock` dla kopii przenośnej) |

Plik gniazda pozostawiony po awarii jest wykrywany i zastępowany przy następnym uruchomieniu. Gniazdo, na którym
coś nadal odpowiada, nigdy nie jest przejmowane. Plik jest usuwany, gdy Moonpool kończy pracę
w normalny sposób.

Kanał służy też [serwerowi MCP](/pl/automation/mcp-setup/) do ustalenia, czy Moonpool
działa: jeśli `ping` otrzyma odpowiedź, to działa, a jeśli brakuje potoku lub gniazda, to nie działa. Te same
polecenia są dostępne także z [wiersza poleceń](/pl/automation/command-line/), z wyjątkiem poleceń
diagnostycznych poniżej.

## Protokół

Jeden obiekt JSON na wiersz na wejściu, jeden wiersz JSON na wyjściu, po kolei. Jedno połączenie może przenosić wiele
żądań.

```json title="request"
{"cmd": "restart", "args": ["my-app"]}
```

```jsonl title="replies"
{"ok": true, "result": "..."}
{"ok": false, "error": "unknown app id: ..."}
```

Żądanie i odpowiedź z PowerShell:

Dla kopii przenośnej należy użyć jej nazwy potoku (`moonpool-<id>`, pokazanej przez polecenie `paths`) zamiast
`moonpool`.

```powershell frame="terminal"
$p = New-Object System.IO.Pipes.NamedPipeClientStream('.', 'moonpool', 'InOut')
$p.Connect(2000)
$w = New-Object System.IO.StreamWriter($p); $w.AutoFlush = $true
$r = New-Object System.IO.StreamReader($p)
$w.WriteLine('{"cmd":"ping"}')
$r.ReadLine()
```

```json title="reply"
{"ok":true,"result":"pong"}
```

- `args` jest listą ciągów i może zostać pominięte. Inne pola są ignorowane.
- `result` jest ciągiem lub null. Polecenia zwracające dane strukturalne zwracają je jako ciąg
  JSON.
- Wiersz, który nie jest poprawnym JSON, otrzymuje `{"ok": false, "error": "bad request: ..."}`.
- Nieznane `cmd` otrzymuje `unknown cmd: <name>`.
- Polecenie przechodzące przez okno (`launch`, `stop`, `restart`, `reload`,
  `refresh-icons`, `help`, `open-window`) otrzymuje odpowiedź po zakończeniu działania albo błąd przekroczenia czasu
  po 45 s. Jeśli interfejs okna huba nie został wczytany, kończy się natychmiast błędem `frontend not
  loaded`.
- Moonpool, który startuje, gdy poprzedni jeszcze się zamyka, ponawia przez około 8 sekund próby powiązania kanału.
  Jeśli nadal się nie uda, zapisuje to w dzienniku i działa dalej bez kanału.

## Polecenia

| Polecenie | Argumenty | Wynik |
| --- | --- | --- |
| `ping` | brak | `pong`. Tylko kanał. |
| `list` | brak | Ciąg JSON `{"apps": [...], "statuses": [...]}` odczytany z pamięci działającego huba, o tej samej strukturze `apps` i `statuses` co `state.json`. Dodaje `"statusNotReady": true`, gdy aplikacje są zarejestrowane, ale pierwsze sprawdzenie stanu jeszcze się nie odbyło. Gdy `apps.json` nie daje się wczytać, dodaje `"manifestError": "<message>"` (aplikacje są wtedy ostatnią wczytaną listą) oraz, gdy od startu nie wczytano żadnej listy, `"manifestLoaded": false`. Tylko kanał. |
| `show` | brak | null. Wysuwa okno na wierzch. |
| `quit` | brak | null. Kończy Moonpool. |
| `launch` | `<id>` | null w razie powodzenia albo `opened` dla wpisu `static` z samym `url`. Błędy: `unknown app id: <id>`, `did not reach running in time`. |
| `stop` | `<id>` | null w razie powodzenia albo `stopped` dla wpisu `static` z samym `url`. Błąd: `still running after stop`. |
| `restart` | `<id>` | Te same wyniki i błędy co `launch`. |
| `reload` | brak | null w razie powodzenia. |
| `refresh-icons` | brak | null w razie powodzenia. |
| `help` | brak | null. Otwiera okno Pomocy. |
| `dump` | `<id>` [`out-path`] | Ścieżka dziennika sesji aplikacji albo kopii w postaci zwykłego tekstu w `out-path`. |
| `paths` | brak | Wielowierszowy raport o folderach i pliku exe używanych przez hub. |
| `read-config` | brak | Ścieżka `dumps\read-config.json`, który zawiera `token`, `valid`, `error`, `path`, `manifest_text`. |
| `write-config` | `<source-file>` [`token`] | Nowy token wersji. Błędy: `stale token: ...`, `rejected invalid manifest: ...`, `cannot read source ...`. |
| `restore-config` | [`index` lub `filename`] | Bez argumentu: ścieżka `dumps\restore-config.json` (`count`, `snapshots`). Z argumentem: `restored <file> (<n> apps); new version token <token>`. |
| `argv` | argumenty wiersza poleceń | null, natychmiast. Wykonuje je dokładnie tak, jak drugie `moonpool.exe <args>` tej kopii, w tym `--ticket`. W ten sposób to drugie uruchomienie przekazuje swoje argumenty, zanim zakończy pracę. |

Przykładowe wymiany:

```jsonl title="request, reply"
{"cmd": "launch", "args": ["nope"]}
{"ok": false, "error": "unknown app id: nope"}

{"cmd": "restore-config", "args": ["1"]}
{"ok": true, "result": "restored 1767225600000.json (6 apps); new version token <token>"}
```

`write-config` i `restore-config` od razu wczytują nowy manifest, zapisują migawkę w
`apps.json.history\` i odświeżają okno.

## Polecenia diagnostyczne (testowanie)

Tylko kanał: wiersz poleceń ich nie przyjmuje. Wszystkie działają w systemach Windows, Linux i macOS
z wyjątkiem `screenshot`, który działa tylko w systemie Windows, a gdzie indziej odpowiada `screenshot is not supported on this
platform (Windows only)`.

| Polecenie | Argumenty | Wynik |
| --- | --- | --- |
| `screenshot` | [`window`] [`max_dim`] | Tylko Windows. Base64 pliku PNG tego okna Moonpool (domyślnie `main`). Opcjonalne `max_dim` ogranicza dłuższy bok w pikselach (wartość ograniczona do 320-2400, domyślnie 320; narzędzie MCP zawsze używa wartości domyślnej). Wartość `max_dim` niebędąca liczbą całkowitą jest błędem. Dozwolone okna: `main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`. Błędy: `unknown window '<name>'`, `window '<name>' is not open`. Nie jest zapisywany na dysku. |
| `open-window` | `<kind>` [`<id>`] | null. Otwiera okno tak, jak robi to jego pozycja menu. `kind`: `settings`, `about`, `installer`, `help`, `themes`, `editor` (opcjonalne `<id>` otwiera okno Edytuj aplikację dla tej aplikacji, brak otwiera Dodaj aplikację), `terminal` (`<id>` wymagane: wybiera kartę terminala tej aplikacji i poszerza hub, aby pokazać panel CLI; nie uruchamia aplikacji), `cli` (tylko poszerza hub). Błędy: `unknown window kind '<kind>'`, `terminal needs an app id`, `unknown app id: <id>`. Odpowiedź przechodzi przez okno huba, tak jak w `launch`. |
| `window-state` | [`window`] | Ciąg JSON: `{"open":false}` albo `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. |
| `stop-mcp` | `<id>` | `stopped`. Kończy proces pomocniczy aplikacji `<processName> mcp`, a nie samą aplikację. Błędy: `missing app id`, `unknown app id: <id>`. |
| `reset-mcp-seen` | [`<id>`] | `<id>: cleared` lub `<id>: was not marked seen`; bez id `cleared <n> entries`. Czyści zapamiętane obserwacje procesów pomocniczych MCP. |

Opcja `--ticket` wiersza poleceń i rekordy wyników w `state.json` należą do drugiego kanału;
zob. [Wiersz poleceń](/pl/automation/command-line/#odczyt-wyniku). Żądania kanału otrzymują
odpowiedź w samej odpowiedzi.

## Zobacz też

- [Wiersz poleceń](/pl/automation/command-line/)
- [Agenci AI: szybki start](/pl/automation/quick-start/#ta-sama-czynność-na-trzy-sposoby)
