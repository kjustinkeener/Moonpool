---
title: "Orientacja w oknie huba Moonpool"
description: "Przewodnik po oknie huba Moonpool: pasek boczny, karty terminala, pasek stanu, menu, baner błędu apps.json oraz zapamiętywanie rozmiaru i położenia okna."
---

![Okno huba z trzema działającymi aplikacjami: dwa wiersze działających aplikacji web (1), pasek kart (2), bieżące wyjście aktywnej aplikacji (3) i pasek stanu (4)](../../../../assets/screenshots/hub-window.png)

1. Dwie z działających aplikacji: zapalona kropka stanu i przycisk zatrzymania zamiast odtwarzania.
2. Pasek kart, jedna karta na każdą otwartą aplikację, z wyróżnioną aktywną kartą.
3. Bieżące wyjście aktywnej aplikacji.
4. Pasek stanu z użyciem CPU i pamięci.

## Układ

| Obszar | Co zawiera |
| --- | --- |
| Pasek tytułu | Minimalizowanie, maksymalizowanie i zamykanie. |
| Pasek boczny | Pole filtru, menu **...** oraz Twoje aplikacje pogrupowane według `group`. Zob. [Pasek boczny i menu](/pl/using/sidebar-and-menus/). |
| Panel CLI | Jedna karta terminala na każdą otwartą aplikację. Zob. [Karty terminala](/pl/using/terminal-tabs/). |
| Pasek stanu | Bieżące użycie CPU i pamięci, wzdłuż dolnej krawędzi. |

Przeciągnięcie separatora między paskiem bocznym a panelem CLI zmienia szerokość paska bocznego.

## Pasek stanu

![Pasek stanu: paski CPU dla poszczególnych rdzeni po lewej, pasek pamięci po prawej](../../../../assets/screenshots/status-bar.png)

Pasek stanu pokazuje jeden cienki pasek na każdy rdzeń CPU (po najechaniu wyświetla się „Użycie CPU na rdzeń”), a następnie
pasek pamięci z etykietą `used/total GB`. Można go wyłączyć opcją **Pokaż pasek stanu CPU i pamięci** w Ustawieniach (`showStatusbar`; zob. [Okno ustawień](/pl/using/settings/)). Zmiana działa natychmiast.

## Menu ...

Przycisk **...** po lewej stronie pola filtru otwiera menu.

![Przycisk menu ... (1) i pole Filtruj aplikacje (2) u góry paska bocznego](../../../../assets/screenshots/sidebar-filter-and-menu.png)

1. Przycisk menu **...**.
2. Pole **Filtruj aplikacje...**.

| Pozycja | Działanie |
| --- | --- |
| Dodaj aplikację | Otwiera edytor aplikacji. Zob. [Dodawanie aplikacji](/pl/apps/add-an-app/). |
| Edytuj apps.json | Otwiera `apps.json` w domyślnym edytorze do ręcznej edycji. |
| Odśwież | Ponownie wczytuje `apps.json` z dysku (także F5, zob. [Skróty i powiększenie](/pl/using/keyboard-shortcuts/)). |
| Ustawienia | Otwiera okno Ustawień. |
| Pomoc | Otwiera tę pomoc. |
| O programie | Otwiera okno O programie, z wersją i sprawdzaniem aktualizacji. |
| Zainstaluj Moonpool... | Tylko Windows. Otwiera okno instalatora, aby zainstalować aplikację lub utworzyć kopię przenośną. Zob. [Instalacja](/pl/getting-started/install/) i [Tryb przenośny](/pl/data/portable-mode/). |

### Ostrzeżenie o konflikcie portów

Jeśli dwie aplikacje w `apps.json` używają tego samego `port`, u dołu menu pojawia się wiersz
ostrzeżenia, na przykład:

```text
port 3000: App A / App B
```

Po najechaniu wyświetla się pełne zdanie. Konflikt należy usunąć w `apps.json` lub w edytorze aplikacji; wiersz znika, gdy żaden port nie jest współdzielony.

## Gdy apps.json zawiera błąd

Jeśli Odśwież (lub F5) stwierdzi, że `apps.json` nie daje się już przetworzyć lub nie przechodzi
walidacji, Moonpool zachowuje listę, którą już miał. Baner u góry paska bocznego informuje:
„apps.json zawiera błąd; wyświetlana jest ostatnio wczytana lista.”, a po nim następuje błąd
(po najechaniu wyświetla się pełny tekst). Lista poniżej jest przyciemniona, ale nadal działa,
więc można uruchamiać i zatrzymywać aplikacje jak zwykle. **Edytuj apps.json** w banerze otwiera
plik; po jego poprawieniu należy wybrać **Odśwież** i baner znika.

Dopóki plik nie wczyta się ponownie, Moonpool nie zapisuje zmian z edytora aplikacji, zmiany
nazwy, usuwania ani ustawiania ikony, więc uszkodzony plik nigdy nie zostanie nadpisany.

Jeśli plik jest już uszkodzony przy uruchomieniu Moonpool, nie ma wcześniejszej listy do
zachowania: baner informuje, że nie wczytano żadnych aplikacji, a pasek boczny jest pusty. Należy
poprawić plik i odświeżyć albo wrócić do niedawnej dobrej kopii (zob.
[Jeśli plik jest uszkodzony](/pl/apps/apps-json/#jeśli-plik-jest-błędny)).

## Pusty ekran

Gdy żadna karta nie jest otwarta, panel CLI pokazuje „Wybierz aplikację po lewej, aby ją
uruchomić.” Zawiera też dwie rzeczy, które pojawiają się tylko wtedy, gdy żadna karta nie jest
otwarta:

- **Baner aktualizacji**, gdy przy uruchomieniu znaleziono nowszą wersję. Zob.
  [Aktualizacje](/pl/data/updating/).
- **Kopiuj prompt**, gotowy prompt, który przekazuje agentowi AI konfigurację Twoich aplikacji. Zob.
  [Agenci AI: szybki start](/pl/automation/quick-start/#kopiuj-prompt).

Ikona w zasobniku, zamykanie, minimalizowanie, Zakończ i zawsze na wierzchu opisano w
[Zasobnik, zamykanie i minimalizowanie](/pl/using/tray-and-closing/).

## Rozmiar, położenie i stan zmaksymalizowania

Moonpool zapamiętuje między uruchomieniami rozmiar, położenie i stan zmaksymalizowania okna huba.
Pierwsze uruchomienie otwiera się w rozmiarze 1200x780, w położeniu wybranym przez system Windows.

Jeśli zapisane położenie nie znajduje się już na żadnym podłączonym ekranie (na przykład
odłączono monitor), położenie jest ignorowane, a zapisany rozmiar jest używany w domyślnym
miejscu. Plikiem jest `window-state.json` w folderze konfiguracji (zob.
[Gdzie znajduje się konfiguracja](/pl/apps/apps-json/#gdzie-znajduje-się-konfiguracja)).

Szerokość paska bocznego i to, czy panel CLI jest zwinięty, są również zapamiętywane.
