---
title: "Zmiana ustawień Moonpool: każda opcja w oknach Ustawień i O programie"
description: "Pełna lista kontrolek w oknach Ustawień i O programie Moonpool, klucz settings.json zapisywany przez każdą z nich oraz sposób przywracania ustawienia domyślnego."
---

Okno **Ustawienia** otwiera się z menu „...” w oknie huba. Zmiany zapisują się na bieżąco. Klawisz Esc
zamyka okno. Ta strona to pełna lista ustawień. Każde z nich jest zapisywane w `settings.json`
pod pokazanym kluczem; sam plik opisano w sekcji
[settings.json](/pl/data/settings-json/).

![Okno Ustawień: przełączniki i suwaki w lewej kolumnie, opcje dzienników w prawej](../../../../assets/screenshots/settings-window.png)

## Przywracanie ustawienia kontrolki

Kliknięcie prawym przyciskiem dowolnego pola wyboru, suwaka lub pola liczbowego przywraca tylko
to ustawienie do wartości domyślnej. Informuje o tym podpowiedź po najechaniu na każdą kontrolkę.
Listy wyboru Języka i Motywu nie mają przywracania.

## Lewa kolumna

| Kontrolka | Klucz | Domyślnie | Działanie |
| --- | --- | --- | --- |
| Język | `locale` | Automatycznie (system) | Język własnych tekstów Moonpool. Działa natychmiast. Zob. [Motywy, język i przezroczystość](/pl/using/themes-and-language/). |
| Motyw | brak (pamięć przeglądarki) | Automatycznie (system) | Motyw kolorystyczny. Przycisk otwiera przeglądarkę motywów z podglądem każdego motywu; kliknięcie motywu stosuje go natychmiast. Zob. [Motywy, język i przezroczystość](/pl/using/themes-and-language/). |
| Zamykanie do zasobnika | `closeToTray` | wyłączone | Włączone: zamknięcie okna ukrywa Moonpool w zasobniku. Wyłączone: zamknięcie kończy pracę programu. |
| Minimalizowanie do zasobnika | `minimizeToTray` | włączone | Włączone: minimalizacja ukrywa Moonpool w zasobniku i znika on z paska zadań. Wyłączone: minimalizuje do paska zadań. |
| Zawsze na wierzchu | `alwaysOnTop` | wyłączone | Utrzymuje każde okno Moonpool nad innymi oknami. |
| Pokaż w zasobniku | `showInTray` | włączone | Utrzymuje widoczną ikonę w zasobniku. |
| Pokaż na pasku zadań | `showInTaskbar` | włączone | Utrzymuje widoczny przycisk na pasku zadań. |
| Pokaż pasek stanu CPU i pamięci | `showStatusbar` | włączone | Pasek z bieżącym użyciem CPU i pamięci u dołu okna huba. |
| Pokaż procesy MCP | `showMcpProcesses` | włączone | Pokazuje proces MCP aplikacji jako podrzędny wiersz MCP na pasku bocznym, gdy jej narzędzia MCP są używane. |
| Przezroczystość tła | `transparency` | 0% | Suwak od 0 do 90 w krokach co 5. Zob. [Motywy, język i przezroczystość](/pl/using/themes-and-language/#przezroczystość). |
| Sprawdzaj aktualizacje przy starcie | `checkOnStartup` | włączone | Przy uruchomieniu sprawdza w serwisie GitHub, czy jest nowsza wersja, i pokazuje baner, jeśli ją znajdzie. Zob. [Aktualizacje](/pl/data/updating/). |

### Blokada zasobnika i paska zadań

Co najmniej jedna z opcji **Pokaż w zasobniku** i **Pokaż na pasku zadań** musi pozostać
włączona, bo inaczej ukryte okno nie miałoby drogi powrotnej. Gdy włączona jest tylko jedna, jej
pole wyboru jest wyłączone, dopóki druga nie zostanie ponownie włączona.

## Prawa kolumna: dzienniki

| Kontrolka | Klucz | Domyślnie | Działanie |
| --- | --- | --- | --- |
| Zachowuj dzienniki wyjścia aplikacji między sesjami | `cliLogging` | wyłączone | Wyjście terminala bieżącej sesji jest zawsze zachowywane dla jej własnych kart. Włączone: dzienniki starszych sesji pozostają na dysku w `cli-output\`, w granicach limitu przechowywania. Wyłączone: są usuwane przy następnym uruchomieniu tej aplikacji. |
| Przechowywanie dzienników na aplikację | `logRetentionMb` | 10 MB | Limit łącznego rozmiaru dzienników każdej aplikacji. Minimum 1. Wyłączone, gdy powyższy przełącznik jest wyłączony. Dziennik bieżącej sesji wlicza się do limitu, ale nigdy nie jest przez niego obcinany ani usuwany. |
| Zapisuj informacje diagnostyczne do pliku | `debugLogging` | wyłączone | Rejestruje wczytania `apps.json`, uruchomienia i błędy w `moonpool.log`. |

Pod każdą grupą dzienników pole ze ścieżką pokazuje lokalizację, wraz z dwoma przyciskami:

- **Otwórz** otwiera folder w menedżerze plików (**Otwórz folder dzienników CLI** dla `cli-output\`, **Otwórz dziennik** dla `moonpool.log`).
- **Kopiuj** umieszcza ścieżkę w schowku (**Kopiuj ścieżkę folderu dzienników CLI**, **Kopiuj ścieżkę pliku dziennika**).

Formaty plików dzienników, separator ponownego uruchomienia i zasady przechowywania opisano w sekcji
[Dzienniki](/pl/data/logs/).

Jeśli pola wyboru nie da się zapisać, czerwony komunikat u góry okna to zgłasza, a pole wyboru
wraca do poprzedniego stanu.

## Okno O programie

Okno **O programie** otwiera się z menu „...”.

![Okno O programie z wierszem wersji, linkami oraz przyciskami Sprawdź aktualizacje i Zamknij](../../../../assets/screenshots/about-window.png)

Pokazuje ono:

- Wersję i datę kompilacji.
- Linki do witryny projektu, repozytorium GitHub i adresu kontaktowego.
- **Sprawdź aktualizacje**. Jeśli istnieje nowsza wersja, pobiera ją, weryfikuje i instaluje, a następnie uruchamia Moonpool ponownie. W przeciwnym razie informuje, że masz najnowszą wersję, albo podaje błąd, jeśli sprawdzanie się nie powiodło.
- Podziękowania dla bibliotek, z których zbudowano Moonpool, oraz autora.

Klawisz Esc je zamyka. O programie na bieżąco odzwierciedla ustawienia motywu, przezroczystości i języka.
