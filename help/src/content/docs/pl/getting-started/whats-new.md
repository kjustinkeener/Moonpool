---
title: "Informacje o wydaniach Moonpool i ostatnie zmiany"
description: "Zobacz, co zmieniło się w ostatnich wydaniach Moonpool, jakie są wymagania uruchomieniowe i gdzie znaleźć pełne informacje o wydaniach w serwisie GitHub."
---

Pełne informacje o każdym wydaniu znajdują się na [stronie Releases](https://github.com/kjustinkeener/Moonpool/releases)
projektu. Ta pomoc jest dostarczana wraz z Moonpool, więc zawsze opisuje używaną wersję. Moonpool
aktualizuje się samodzielnie; zob. [Aktualizacje](/pl/data/updating/).

## 0.3.16

- **Kilka kopii Moonpool jednocześnie.** Zainstalowany Moonpool i dowolna liczba kopii przenośnych
  mogą działać obok siebie, po jednej na folder, każda z własnymi aplikacjami, ikoną w zasobniku i
  kanałem sterowania. Zob. [Tryb przenośny](/pl/data/portable-mode/#kilka-kopii-naraz).
- **Przeglądarka motywów.** 68 motywów, każdy z podglądem we własnych kolorach. Zob.
  [Motywy, język i przezroczystość](/pl/using/themes-and-language/).
- **Przykłady, które działają.** Nowy `apps.json` zawiera przykładowe aplikacje, które wszystkie
  działają bez zmian. Przykładowe pulpity znajdują się teraz w należącym do aplikacji folderze
  `dashboards/examples`, który jest aktualizowany razem z Moonpool. Zob.
  [Przykładowe pulpity](/pl/getting-started/example-dashboards/).
- **Błędy apps.json są pokazywane.** Baner nad paskiem bocznym wyświetla błąd, a nieudane
  odświeżenie zachowuje ostatnią listę, która została wczytana. Zob.
  [Gdy apps.json zawiera błąd](/pl/using/hub-window/#gdy-appsjson-zawiera-błąd).
- **Kanał sterowania w systemie Linux**, przez gniazdo Unix, oraz polecenie `list`. Zob.
  [Polecenia sterujące](/pl/automation/control-verbs/).
- Okno O programie i edytor aplikacji na bieżąco reagują na zmiany motywu i języka. Pozycja menu
  **Zainstaluj Moonpool...** jest ukryta poza systemem Windows.

## 0.3.15

- Ponownie uruchomiona aplikacja zachowuje wcześniejsze wyjście, z datowanym separatorem
  „restarted”. Zob. [Karty terminala](/pl/using/terminal-tabs/#restart).
- Każda aplikacja ma własny folder `cli-output`, więc przycinanie dzienników nigdy nie dotyka
  dzienników innej aplikacji.
- `killMode` i `stopCommand` są dostępne w edytorze aplikacji. Zob.
  [Zatrzymywanie i ponowne uruchamianie](/pl/apps/stop-and-restart/).
- Uruchamiane aplikacje nie dziedziczą już własnego profilu WebView2 Moonpool.

## 0.3.14

- Dzienniki sesji mogą być zachowywane między sesjami, z limitem rozmiaru na aplikację. Zob.
  [Dzienniki](/pl/data/logs/).
- Przyciski Otwórz i Kopiuj dla folderów dzienników w Ustawieniach.
- Poprawki paska tytułu okna Pomocy.

## Wymagania

- Windows 10 lub 11 z WebView2 (zob. [Windows](/pl/platforms/windows/)).
- Linux z WebKitGTK 4.1 i biblioteką AppIndicator (zob. [Linux](/pl/platforms/linux/)).
