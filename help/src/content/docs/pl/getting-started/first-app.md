---
title: "Dodawanie i uruchamianie pierwszej aplikacji w Moonpool"
description: "Od pierwszego uruchomienia do własnej działającej aplikacji w kilka minut: dodaj ją, uruchom, zatrzymaj i znajdź później okno huba oraz tę pomoc."
---

## 1. Uruchomienie Moonpool

W systemie Windows należy uruchomić `moonpool.exe` i kliknąć **Zainstaluj Moonpool** (zob.
[Instalacja](/pl/getting-started/install/)). W systemie Linux należy uruchomić AppImage lub
zainstalowany pakiet.

Przy pierwszym uruchomieniu Moonpool wypełnia pasek boczny przykładowymi aplikacjami (w systemie
Windows Notatnik, powłoka, mały serwer WWW i dołączone pulpity). Działają bez zmian (serwer WWW
wymaga Pythona), więc można je wypróbować, a potem edytować lub usunąć. Moonpool umieszcza też
ikonę w zasobniku systemowym. W systemie Windows, jeśli ikony nie widać, należy kliknąć strzałkę
**^** po prawej stronie paska zadań.

## 2. Dodanie aplikacji

1. Otwórz menu **...** u góry paska bocznego i wybierz **Dodaj aplikację**.
2. Wpisz **name**. Grupa początkowo ma wartość `Web apps`; można ją zachować lub wybrać inną.
3. Dla serwera deweloperskiego pozostaw **type** jako `web`.
4. Ustaw **cwd** na folder projektu, a **command** na polecenie, które wpisuje się, aby go
   uruchomić, na przykład `npm run dev`.
5. Ustaw **port** na port, na którym aplikacja nasłuchuje, a **url** na stronę do otwarcia.
6. Zapisz.

Szczegóły każdego pola opisano w [Dodawaniu aplikacji](/pl/apps/add-an-app/).

## 3. Uruchomienie

Kliknij przycisk **Uruchom** przy aplikacji (ikona odtwarzania w jej wierszu). Otworzy się jej
karta terminala i pokaże wyjście. Kropka stanu pulsuje podczas uruchamiania aplikacji, a następnie
staje się pełna, gdy jej port zacznie odpowiadać. Jeśli opcja **openBrowser** jest włączona,
strona się otworzy.

Kliknięcie nazwy aplikacji tylko otwiera jej kartę terminala. Nigdy nie uruchamia aplikacji.

## 4. Zatrzymanie

Kliknij przycisk **Zatrzymaj** (kwadrat) w wierszu. Kropka robi się szara.

Jeśli po zatrzymaniu coś nadal działa, zob. [Zatrzymywanie i ponowne uruchamianie](/pl/apps/stop-and-restart/).

## Zlecenie tego agentowi

Gdy żadna karta nie jest otwarta, panel CLI pokazuje przycisk **Kopiuj prompt**. Wklejenie
promptu do agenta AI sprawia, że agent znajduje Twoje aplikacje i je dodaje. Zob.
[Agenci AI: szybki start](/pl/automation/quick-start/).

## Odnajdywanie huba później

- Kliknij lewym przyciskiem ikonę w zasobniku systemowym, aby pokazać okno huba. Kliknięcie
  prawym otwiera menu z pozycjami **Pokaż Moonpool** i **Zakończ**.
- Domyślnie zamknięcie okna kończy pracę Moonpool. Włącz w Ustawieniach opcję **Zamykanie do
  zasobnika**, aby zamiast tego ukrywać program w zasobniku i utrzymywać go w działaniu. Zob.
  [Zasobnik, zamykanie i minimalizowanie](/pl/using/tray-and-closing/).

## Uzyskiwanie pomocy

**Pomoc** w menu **...** u góry paska bocznego otwiera tę pomoc we własnym oknie.
Działa offline i zawsze odpowiada używanej wersji.

![Okno pomocy z zaznaczoną nawigacją po sekcjach po lewej stronie i stroną po prawej](../../../../assets/screenshots/help-window.png)

## Dalej

- [Dodawanie aplikacji](/pl/apps/add-an-app/)
- [Rozwiązywanie problemów](/pl/support/troubleshooting/)
