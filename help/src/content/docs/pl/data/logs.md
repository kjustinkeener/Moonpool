---
title: "Znajdowanie dzienników sesji Moonpool i dziennika diagnostycznego"
description: "Znajdź dziennik sesji każdej aplikacji, własny dziennik diagnostyczny Moonpool, zrzuty i bufor przewijania oraz sprawdź, jak długo są przechowywane i jak je otworzyć lub skopiować."
---

Moonpool przechowuje cztery rodzaje wyjścia:

| Rodzaj | Gdzie | Przechowywanie |
| --- | --- | --- |
| Dziennik sesji | `cli-output\<id>\<session-start-ms>.log` w folderze konfiguracji | Bieżąca sesja zawsze; starsze sesje według reguł przechowywania poniżej |
| `moonpool.log` | Folder konfiguracji | Zapisywany tylko przy włączonej opcji **Zapisuj informacje diagnostyczne do pliku** |
| Zrzut | Tam, gdzie zażądano, lub ścieżka samego dziennika sesji | Do czasu usunięcia |
| Bufor przewijania | W karcie terminala | 10 000 wierszy, do zamknięcia Moonpool |

Folder konfiguracji opisano w sekcji [Gdzie znajduje się konfiguracja](/pl/apps/apps-json/#gdzie-znajduje-się-konfiguracja).

## Dzienniki sesji

Wszystko, co aplikacja wypisuje w terminalu, jest też zapisywane do pliku dziennika:

```text
<config folder>\cli-output\<id>\<session-start-ms>.log
```

- Jeden plik na aplikację na sesję Moonpool. Liczba oznacza chwilę uruchomienia tego procesu Moonpool.
- Zatrzymanie i ponowne uruchomienie aplikacji nadal dopisuje do tego samego pliku. Przyciemniony wiersz
  rozdzielający oznacza początek każdego nowego uruchomienia, a ten sam znacznik widać w karcie terminala:

  ```text title="1767225600000.log"
  Local:   http://localhost:5173/
  ---------- restarted 2026-10-05 09:14:02 ----------
  Local:   http://localhost:5173/
  ```

- Znaki w `id` inne niż litery, cyfry, `-` i `_` zamieniane są na `_` w nazwie folderu.
  Zatem `.` staje się `_`. Litery spoza alfabetu angielskiego są zachowywane.
- Plik zawiera surowe wyjście terminala, w tym kody kolorów. Aby uzyskać zwykły tekst, użyj zrzutu.

Ponowne otwarcie karty aplikacji odtwarza dziennik tej sesji, więc widać jej wcześniejsze wyjście.

## Przechowywanie

Przechowywanie dotyczy tylko dzienników z wcześniejszych sesji Moonpool. Działa przy uruchomieniu aplikacji,
tylko dla folderu tej aplikacji, od najstarszych.

| **Zachowuj dzienniki wyjścia aplikacji między sesjami** (`cliLogging`) | Co dzieje się z dziennikami wcześniejszych sesji |
| --- | --- |
| wyłączone (domyślnie) | Usuwane przy następnym uruchomieniu aplikacji. |
| włączone | Zachowywane, dopóki łączny rozmiar folderu nie przekroczy wartości **Przechowywanie dzienników na aplikację** (`logRetentionMb`, domyślnie 10 MB), po czym najstarsze są usuwane. |

Plik bieżącej sesji wlicza się do tej sumy, ale nigdy nie jest usuwany ani obcinany.
Jeden bardzo duży dziennik bieżący może więc wypchnąć wszystkie starsze.

![Sekcja dzienników w Ustawieniach: pole wyboru zachowywania dzienników, rozmiar przechowywania na aplikację w MB i pole wyboru dziennika diagnostycznego, każde z wierszem ścieżki folderu](../../../../assets/screenshots/settings-logging-section.png)

1. **Zachowuj dzienniki wyjścia aplikacji między sesjami** to `cliLogging`. **Przechowywanie dzienników na aplikację** poniżej to `logRetentionMb`.

## moonpool.log

Gdy włączona jest opcja **Zapisuj informacje diagnostyczne do pliku** (`debugLogging`), Moonpool dopisuje wiersze ze znacznikami czasu do
`moonpool.log` w folderze konfiguracji: wczytywanie `apps.json`, uruchomienia (z poleceniem i folderem),
polecenia sterujące i błędy. Należy ją włączyć przed odtworzeniem problemu.

## Otwieranie i kopiowanie

W [Ustawieniach](/pl/using/settings/#prawa-kolumna-dzienniki), w każdej grupie dzienników:

- **Otwórz folder dzienników CLI** i **Otwórz dziennik** otwierają folder w menedżerze plików.
- **Kopiuj ścieżkę folderu dzienników CLI** i **Kopiuj ścieżkę pliku dziennika** umieszczają ścieżkę w schowku.

W karcie terminala **Kopiuj wszystko** kopiuje cały bufor przewijania jako tekst.

## Zrzuty

Polecenie `dump` udostępnia dziennik sesji ze skryptu:

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" dump my-app C:\temp\my-app.log
```

Z podaną ścieżką wyjściową zapisuje kopię w postaci zwykłego tekstu, bez kodów kolorów. Bez niej zwraca
ścieżkę samego dziennika sesji. Agent otrzymuje ten sam, już oczyszczony tekst z
`moonpool_app_output`. Zob. [Wiersz poleceń](/pl/automation/command-line/).
