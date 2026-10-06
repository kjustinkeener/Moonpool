---
title: "Ścieżki, tokeny MP_HOME i zmienne środowiskowe w aplikacjach"
description: "Używaj tokenów {MP_HOME} i {MP_DATA} oraz ścieżek względnych ./ we wpisach aplikacji, sprawdź, które pola je rozwijają, i ustaw env oraz folder roboczy."
---

## Tokeny

| Token | Rozwija się do |
| --- | --- |
| `{MP_HOME}` | Tryb przenośny: folder zawierający `moonpool.exe` (folder `.moonpool\`). Instalacja w systemie Windows: `%USERPROFILE%\.moonpool`. Linux: `$XDG_CONFIG_HOME/Moonpool`, a w razie braku `~/.config/Moonpool`, czyli ten sam folder co `{MP_DATA}`. |
| `{MP_DATA}` | Folder konfiguracji, ten, który zawiera `apps.json`. |

Token, którego nie można rozwiązać, pozostaje bez zmian.

## Które pola rozwijają tokeny

| Pole | Tokeny | Początkowe `./` lub `.\` |
| --- | --- | --- |
| `cwd` | tak | tak, zakotwiczone w `{MP_HOME}` |
| `command` | tak | nie |
| `stopCommand` | tak | nie (działa w `cwd`, które jest zakotwiczone) |
| `url` | tak | nie |
| `icon` | tak | tak, zakotwiczone w `{MP_HOME}` |
| wartości `env`, `processName`, `note` | nie | nie |

Ścieżka względna bez `./` (taka jak `apps\tool`) pozostaje bez zmian i jest rozwiązywana względem
własnego folderu roboczego Moonpool, co rzadko bywa pożądane. Lepiej używać `./` lub tokenu.

```text
./apps/notes                       anchored to {MP_HOME}
{MP_HOME}\apps\notes\notes.exe     token
{MP_DATA}\dumps                    token
apps\tool                          left alone, resolves against Moonpool's working folder
```

```json title="apps.json"
{ "id": "notes", "name": "Notes", "group": "Desktop apps", "type": "desktop",
  "cwd": "./apps/notes",
  "command": "{MP_HOME}\\apps\\notes\\notes.exe",
  "processName": "notes" }
```

Obie formy nadal działają po przeniesieniu folderu przenośnego. Stała ścieżka, taka jak
`C:\tools\notes`, nie jest przenoszona. W trybie przenośnym okno Edytuj aplikację oznacza ścieżki bezwzględne w `cwd`
i `url` plakietką „nieprzenośna”. Zob. [Tryb przenośny](/pl/data/portable-mode/).

## Środowisko

`env` jest obiektem ciągów. Okno edytuje go jako jedno `KEY=VALUE` w wierszu; dzieli
każdy wiersz na pierwszym `=`, przycina obie strony i ignoruje wiersze bez znaku `=`.

W oknie:

```text
PORT=8091
NODE_ENV=development
```

W `apps.json`, jako klucz `env` wpisu:

```json title="apps.json (one entry)"
{ "id": "habits", "name": "Habits", "group": "Web apps", "type": "web", "command": "python app.py",
  "env": { "PORT": "8091", "NODE_ENV": "development" } }
```

- Uruchamiane polecenie dziedziczy środowisko Moonpool wraz z `env`. Wpisy w `env` mają pierwszeństwo.
- `env` jest stosowane także do `stopCommand`.
- Wartości są używane tak, jak je zapisano: Moonpool nie rozwija ani `{MP_HOME}`, ani `%VAR%`.
- Moonpool kieruje własny WebView2 do prywatnego folderu profilu przez
  `WEBVIEW2_USER_DATA_FOLDER`. Uruchamiane aplikacje tego nie dziedziczą. Jeśli zmienna
  została ustawiona samodzielnie przed uruchomieniem Moonpool, otrzymujesz tę wartość; w przeciwnym razie nie jest ustawiona.
  Wpis w `env` nadal może ją zastąpić.

## Folder roboczy

Polecenie i `stopCommand` działają w `cwd`. Gdy `cwd` pominięto, polecenie działa we
własnym folderze roboczym Moonpool, więc `cwd` należy ustawić wszędzie tam, gdzie używane są ścieżki względne.
