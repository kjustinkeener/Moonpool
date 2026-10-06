---
title: "Przykładowe pulpity dołączone do Moonpool"
description: "Otwórz dołączone przykładowe pulpity działające offline, sprawdź, gdzie się znajdują i jak odwołują się do nich przykładowe aplikacje, oraz dodaj je do istniejącej konfiguracji."
---

Moonpool zawiera zestaw samodzielnych pulpitów wbudowanych w program. Działają całkowicie
offline, bez serwera i bez CDN.

| Pulpit | Czym jest |
| --- | --- |
| CSV explorer | Wystarczy upuścić plik CSV lub TSV; pulpit profiluje kolumny i wyświetla dane. |
| JSON explorer | Wystarczy upuścić plik JSON (tablice, obiekty zagnieżdżone lub mapy). |
| Excel explorer | Wystarczy upuścić plik `.xlsx` lub `.xls`, który jest analizowany offline. |
| Moonpool Docs | Przeglądarka dokumentacji Markdown działająca offline. |

## Gdzie się znajdują

Przy uruchomieniu Moonpool zapisuje pulpity w `{MP_HOME}\dashboards\examples`:

| Tryb | Folder |
| --- | --- |
| Zainstalowany (Windows) | `%USERPROFILE%\.moonpool\dashboards\examples` |
| Przenośny | `<twój folder .moonpool, ten zawierający moonpool.exe>\dashboards\examples` |
| Linux | `~/.config/Moonpool/dashboards/examples` (lub `$XDG_CONFIG_HOME/Moonpool/dashboards/examples`) |

Folder `examples` należy do Moonpool: jest zastępowany przy każdej aktualizacji Moonpool, więc
zmiany wprowadzone w nim zostaną utracone. Aby dostosować pulpit, należy skopiować jego folder
oraz wspólny folder `_lib` wyżej, do `dashboards`, i wskazać aplikacji kopię. Moonpool nigdy nie
zmienia niczego innego w `dashboards`.

Wersje starsze niż 0.3.16 zapisywały przykłady bezpośrednio w `dashboards`. Te kopie pozostają
tam, gdzie były, i nie otrzymują już aktualizacji; aplikacje, które na nie wskazują, nadal
działają. Aby otrzymać zaktualizowane wersje, należy zmienić ich `url` na poniższą ścieżkę
`dashboards/examples/...`.

## Jak aplikacje się do nich odwołują

Każdy pulpit to aplikacja typu `static`, której `url` jest adresem `file:///` zakotwiczonym w `{MP_HOME}`:

```text
file:///{MP_HOME}/dashboards/examples/csv/index.html
```

`{MP_HOME}` jest zamieniane na folder instalacji lub, w trybie przenośnym, na folder pakietu,
więc wpis nadal działa po przeniesieniu pakietu. Adresy `file://` są dozwolone. Zob.
[Ścieżki i środowisko](/pl/apps/paths-and-environment/).

## Przykładowe aplikacje pojawiają się tylko przy pierwszym uruchomieniu

Przykładowe wpisy są zapisywane w `apps.json` tylko wtedy, gdy plik konfiguracji jeszcze nie
istnieje. Jeśli `apps.json` już istnieje, wpisy pulpitów należy dodać samodzielnie (**Edytuj apps.json**
w menu „...”, a następnie **Odśwież**). Należy dodać te cztery wpisy wewnątrz tablicy najwyższego
poziomu, oddzielając je przecinkami od pozostałych wpisów:

```jsonc title="apps.json (excerpt)"
{
  "id": "csv-explorer",
  "name": "Sample CSV Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html",
  "openBrowser": true
},
{
  "id": "json-explorer",
  "name": "Sample JSON Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/json/index.html",
  "openBrowser": true
},
{
  "id": "xlsx-explorer",
  "name": "Sample Excel Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/xlsx/index.html",
  "openBrowser": true
},
{
  "id": "docs-browser",
  "name": "Moonpool Docs",
  "group": "Docs",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/docs/index.html",
  "openBrowser": true
}
```

Znaczenie pól opisano w [Polach aplikacji](/pl/apps/fields/).

## Zobacz także

- [Przykłady](/pl/apps/examples/): pełniejsze wpisy do skopiowania.
- [Typy aplikacji](/pl/apps/types/#static)
