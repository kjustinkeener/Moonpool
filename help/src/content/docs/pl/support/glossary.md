---
title: "Słownik Moonpool: aplikacje, stany, pliki i ustawienia"
description: "Proste definicje słów, których pomoc Moonpool używa dla swoich części, stanów aplikacji, plików i ustawień, aby łatwiej śledzić resztę dokumentacji."
---

## Aplikacje

| Termin | Znaczenie |
| --- | --- |
| aplikacja | Jedna rzecz, którą zarządza Moonpool: serwer deweloperski, aplikacja desktopowa, strona lub polecenie. |
| wpis | Rekord aplikacji w `apps.json`. Używany tylko przy mówieniu o JSON. |
| wiersz aplikacji | Wiersz aplikacji na pasku bocznym, z kropką stanu i kontrolkami. |
| grupa | Nagłówek paska bocznego, pod którym wymieniona jest aplikacja, z jej pola `group`. |
| typ | `web`, `desktop`, `static` lub `cli`. Decyduje, które pola mają znaczenie. Zob. [Typy aplikacji](/pl/apps/types/). |
| id | Stały klucz aplikacji, używany w nazwach plików, poleceniach i narzędziach agentów. Zob. [Identyfikator id](/pl/apps/apps-json/#identyfikator-id). |

## Stany aplikacji

| Stan | Znaczenie |
| --- | --- |
| uruchamianie | Moonpool uruchomił aplikację, ale jeszcze nie zobaczył, że działa. Pulsująca kropka. |
| Działa | Jej `port` odpowiada, jej `processName` istnieje albo, gdy nie ustawiono żadnego z nich, uruchomiony przez Moonpool terminal nadal żyje. Pełna kropka. Zob. [Jak ustalany jest stan Działa](/pl/apps/types/#jak-ustalany-jest-stan-działa). |
| zatrzymana | Nic z powyższego. Szara kropka. |
| zarządzana | Moonpool uruchomił ją w tej sesji. Działająca aplikacja, która nie jest zarządzana, została uruchomiona w inny sposób, a Zakończ ją pomija. |

Karta terminala i działająca aplikacja to dwie osobne rzeczy. Kliknięcie nazwy aplikacji tylko
otwiera jej kartę terminala; nigdy nie uruchamia aplikacji. Zamknięcie karty nigdy nie zatrzymuje
aplikacji.

## Okna i części

| Termin | Znaczenie |
| --- | --- |
| hub | Rezydentny proces Moonpool i jego główne okno. Nazwy narzędzi nazywają go „launcher”. |
| okno huba | Główne okno: pasek boczny po lewej, panel CLI po prawej. |
| zasobnik systemowy | Ikona w zasobniku systemowym i jej menu (**Pokaż Moonpool**, **Zakończ**). |
| pasek boczny | Lewa strona okna huba: pole filtru, menu **...** i wiersze aplikacji. |
| panel CLI | Prawa strona okna huba, zawierająca karty terminala. |
| karta terminala | Terminal jednej aplikacji w panelu CLI. |
| podrzędny wiersz MCP | Przyciemniony wiersz pod aplikacją pokazujący jej własny proces pomocniczy `<exe> mcp`. |
| edytor aplikacji | Okno dialogowe Dodaj aplikację i Edytuj aplikację. |

## Pliki i foldery

| Termin | Znaczenie |
| --- | --- |
| folder konfiguracji | Folder zawierający `apps.json` i pozostałe pliki Moonpool. Token `{MP_DATA}`. Zob. [Gdzie znajduje się konfiguracja](/pl/apps/apps-json/#gdzie-znajduje-się-konfiguracja). |
| `{MP_HOME}` | Folder Moonpool: `%USERPROFILE%\.moonpool` w wersji zainstalowanej, folder `.moonpool\` kopii przenośnej, folder konfiguracji w systemie Linux. |
| sesja | Jedno uruchomienie huba, od startu do zakończenia. |
| dziennik sesji | Plik zawierający wszystko, co aplikacja wypisała podczas jednej sesji, w `cli-output\`. Zob. [Dzienniki](/pl/data/logs/). |
| `moonpool.log` | Własny dziennik diagnostyczny Moonpool, zapisywany tylko przy włączonej opcji **Zapisuj informacje diagnostyczne do pliku**. |
| zrzut (dump) | Kopia dziennika sesji w postaci zwykłego tekstu, wykonana poleceniem `dump`. |
| migawka (snapshot) | Kopia poprawnego `apps.json` w `apps.json.history\`. Zob. [Kopia zapasowa i odzyskiwanie](/pl/data/backup-and-recovery/). |

## Tryby

| Termin | Znaczenie |
| --- | --- |
| zainstalowany | Moonpool w `%USERPROFILE%\.moonpool`, ze skrótem w menu Start i wpisem Dodaj/Usuń programy. Tylko Windows. |
| przenośny | Moonpool w wybranym folderze `.moonpool\`, oznaczony plikiem `moonpool.portable`. Zob. [Tryb przenośny](/pl/data/portable-mode/). |
| kopia | Jeden folder Moonpool, zainstalowany lub przenośny. Każda kopia działa niezależnie. |

## Zatrzymywanie i automatyzacja

| Termin | Znaczenie |
| --- | --- |
| `killMode` | Dodatkowy krok, który Zatrzymaj wykonuje po zakończeniu terminala aplikacji. Zob. [Zatrzymywanie i ponowne uruchamianie](/pl/apps/stop-and-restart/). |
| `stopCommand` | Polecenie, które Zatrzymaj uruchamia, gdy `killMode` ma wartość `command`. |
| `processName` | Nazwa procesu, której szuka Moonpool i którą kończy w trybie `processName`. |
| kanał sterowania | Potok nazwany (Windows) lub gniazdo Unix (Linux, macOS), na którym odpowiada hub. Zob. [Polecenia sterujące](/pl/automation/control-verbs/). |
| polecenie (verb) | Słowo polecenia, takie jak `launch` lub `reload`, podawane w wierszu poleceń lub w kanale sterowania. |
| bilet (ticket) | Klucz dołączany przez `--ticket`, aby odczytać wynik polecenia z `state.json`. |
| token | Znacznik wersji `apps.json`, który musi towarzyszyć zapisowi konfiguracji. |
| pomocnik MCP (shim) | Proces `<exe> mcp` uruchamiany przez hosta AI, aby dotrzeć do własnych narzędzi aplikacji. |
