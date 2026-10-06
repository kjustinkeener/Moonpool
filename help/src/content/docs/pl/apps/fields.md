---
title: "Wszystkie pola apps.json: typ, wartość domyślna i działanie"
description: "Sprawdź każdy klucz wpisu apps.json wraz z typem, wartością domyślną i typami aplikacji, które go używają, zgodnie z nazwami w oknie Edytuj aplikację."
---

Okno Edytuj aplikację pokazuje te same pola pod tymi samymi nazwami. Pola, które nie dotyczą
wybranego typu, są w oknie przyciemnione, ale nadal zapisywane, z jednym wyjątkiem:
`stopCommand` jest zapisywane tylko wtedy, gdy `killMode` ma wartość `command`.

![Okno Edytuj aplikację od name do stopCommand, z zaznaczoną listą killMode; nieużywane pola, takie jak processName i stopCommand, są przyciemnione](../../../../assets/screenshots/edit-app-dialog.png)

1. Lista wyboru `killMode`. Pola, których nie używa, pozostają przyciemnione.

| Pole | Typ | Wymagane | Używane przez | Działanie |
| --- | --- | --- | --- | --- |
| `id` | string | tak | wszystkie | Unikalny klucz. Litery, cyfry, `.`, `_`, `-`, nie może zaczynać się od `-`. Zob. [Przegląd](/pl/apps/apps-json/#identyfikator-id). |
| `name` | string | tak | wszystkie | Etykieta na pasku bocznym. Nie może być pusta. |
| `group` | string | tak | wszystkie | Nagłówek na pasku bocznym, pod którym wyświetlana jest aplikacja. Nie może być pusty przy ręcznej edycji; okno zapisuje pustą grupę jako `Apps`. Dowolny tekst; nowa nazwa tworzy nową grupę. |
| `type` | string | tak | wszystkie | `web`, `desktop`, `static` lub `cli`. Zob. [Typy aplikacji](/pl/apps/types/). |
| `command` | string | wszystkie poza `static` | wszystkie | Wykonywane w terminalu, aby uruchomić aplikację, przez `cmd /c` w systemie Windows i `$SHELL -c` w pozostałych (`/bin/sh`, jeśli `SHELL` nie jest ustawione). Opcjonalne dla `static`. |
| `cwd` | string | nie | wszystkie z `command` | Folder, w którym uruchamiane jest polecenie. Domyślnie własny folder roboczy Moonpool. Obsługuje tokeny i `./`. Zob. [Ścieżki i środowisko](/pl/apps/paths-and-environment/). |
| `port` | integer, od 1 do 65535 | nie | dowolne | Aplikacja działa, dopóki coś odpowiada na tym porcie na localhost (IPv4 lub IPv6). Odczytywane przez `killMode` `port`. |
| `processName` | string | nie | dowolne, głównie `desktop` | Aplikacja działa, dopóki istnieje proces o tej nazwie. Wielkość liter nie ma znaczenia, z `.exe` lub bez, więc `my-app` pasuje do `my-app.exe`. W systemie Linux najwyżej 15 znaków. Odczytywane przez `killMode` `processName`. |
| `mcpProcessName` | string | nie | dowolne z `processName` | Wzorzec z symbolami wieloznacznymi dla nazwy procesu serwera MCP tej aplikacji. `*` pasuje do dowolnego ciągu znaków, `?` do jednego znaku. Wielkość liter nie ma znaczenia, wzorzec dotyczy całej nazwy, a `.exe` jest opcjonalne. Pasujący proces jest traktowany jako serwer MCP aplikacji (podwiersz MCP na pasku bocznym) i nie wymaga `mcp` jako pierwszego argumentu. Zob. [mcpProcessName](#mcpprocessname). |
| `url` | string | tylko `static` | `web`, `static` | Strona do otwarcia. Otwierane są wyłącznie adresy `http://`, `https://`, `mailto:` i `file://`. |
| `openBrowser` | boolean, domyślnie `false` | nie | dowolny typ z `url` (okno przyciemnia je dla `desktop` i `cli`) | Automatycznie otwiera `url`, gdy Moonpool wykryje, że aplikacja działa (zob. niżej). |
| `killMode` | string | nie | wszystkie | Dodatkowe sprzątanie przy zatrzymaniu i ponownym uruchomieniu: `processName`, `port`, `command` lub `none`. Zob. [Zatrzymywanie i ponowne uruchamianie](/pl/apps/stop-and-restart/). |
| `stopCommand` | string | nie | `killMode` `command` | Polecenie uruchamiane przy zatrzymaniu. Ignorowane we wszystkich innych trybach. |
| `env` | obiekt ciągów | nie | wszystkie | Dodatkowe zmienne środowiskowe. Okno edytuje je jako jedno `KEY=VALUE` w wierszu. |
| `icon` | string | nie | wszystkie | Obraz na pasku bocznym: ścieżka pliku, adres URL `http(s)` lub URI `data:`. Ustaw go przez **Wybierz ikonę...** w menu kontekstowym aplikacji albo ręcznie. |
| `note` | string | nie | wszystkie | Podpowiedź wyświetlana po najechaniu na aplikację na pasku bocznym. |

Wpis używający `env` i `killMode`:

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000", "NODE_ENV": "development" },
  "killMode": "port"
}
```

Jak współdziałają `port` i `killMode`, opisano w poradniku
[Znajdowanie i kończenie procesu używającego portu](/pl/guides/find-and-kill-process-using-port-windows/).

## mcpProcessName

Domyślnie Moonpool uznaje proces za serwer MCP aplikacji, gdy jego nazwa pasuje do
`processName`, a pierwszym argumentem jest `mcp`, na przykład `notes-app.exe mcp`. Ustaw
`mcpProcessName`, gdy serwer działa pod inną nazwą: aplikacja, która śledzi jeden plik exe,
podczas gdy jej serwer MCP to inny (`mog.exe mcp`), lub zmieniona nazwa kopii serwera.

Wartość jest wzorcem z symbolami wieloznacznymi. `*` pasuje do dowolnego ciągu znaków (także pustego), a `?`
dokładnie do jednego. Porównanie z całą nazwą procesu odbywa się bez rozróżniania wielkości liter, a
wzorzec bez `.exe` pasuje także do nazwy z `.exe`. Pusta wartość oznacza brak ustawienia.

```json
{
  "id": "destiny",
  "name": "Destiny",
  "group": "Desktop apps",
  "type": "desktop",
  "processName": "destiny",
  "mcpProcessName": "destiny-mcp-*"
}
```

Pasuje to do kopii o zmienionej nazwie, na przykład `destiny-mcp-2706210170.exe`. Proces pasujący do
`mcpProcessName` jest serwerem niezależnie od tego, czy uruchomiono go z `mcp`, i nigdy nie jest
liczony jako działająca aplikacja. Jeśli wzorzec pasuje także do samego `processName` (na przykład
`destiny*`), Moonpool nadal wymaga argumentu `mcp`, więc prawdziwa aplikacja nigdy nie zostanie
pomylona ze swoim serwerem MCP. Zob. [Konfiguracja MCP](/pl/automation/mcp-setup/#aplikacje-z-własnym-serwerem-mcp).

## openBrowser

Moonpool otwiera `url` jednokrotnie, gdy aplikacja uruchomiona przez Moonpool po raz pierwszy ma stan „działa”. Do
wykrycia tego potrzebny jest `port` lub `processName`. Bez żadnego z nich „działa” oznacza tylko, że proces
terminala żyje, a przeglądarka nie jest otwierana automatycznie. Wyłącz `openBrowser`,
jeśli polecenie samo otwiera przeglądarkę. Wpis `static` bez polecenia otwiera `url`
przy każdym naciśnięciu Uruchom, niezależnie od `openBrowser`.

Dwie aplikacje skonfigurowane z tym samym `port` są oznaczane na pasku bocznym.

## Ikony

Ikoną aplikacji jest pierwszy istniejący z poniższych elementów:

1. Pole `icon`.
2. `icons\<id>.<ext>` w folderze konfiguracji, na przykład `icons\site.png`.
3. Plik ikony w folderze samej aplikacji (jej `cwd` lub folder `url` typu `file:///`).
4. Dla `desktop` ikona zbudowanego lub działającego pliku `.exe`.
5. Dla `web` i `static` plik `/favicon.ico` witryny, gdy serwer już działa.
6. Symbol dla danego typu.

Większość aplikacji nie wymaga ustawiania ikony.
