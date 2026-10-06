---
title: "Skróty klawiaturowe i myszy oraz powiększenie w Moonpool"
description: "Wszystkie skróty klawiaturowe i myszy w oknie huba Moonpool oraz sposób powiększania i pomniejszania interfejsu, aby tekst był wygodny do czytania."
---

## Klawiatura

| Klawisze | Gdzie | Działanie |
| --- | --- | --- |
| F5, Ctrl+R, Cmd+R | Okno huba | Ponownie wczytuje `apps.json` z dysku, tak samo jak **Odśwież** w menu. Sama strona nie jest odświeżana. |
| Esc | Menu kontekstowe | Zamyka je. |
| Esc | Zmiana nazwy aplikacji | Anuluje zmianę nazwy. |
| Esc | Okna Ustawień, O programie i edytora aplikacji | Zamyka okno (edytor pyta przed odrzuceniem zmian). |
| Enter | Zmiana nazwy aplikacji | Zapisuje nową nazwę. |

## Mysz

| Czynność | Gdzie | Działanie |
| --- | --- | --- |
| Ctrl + kółko | Okno huba | Powiększa interfejs. |
| Zaznaczenie tekstu | Terminal | Kopiuje i czyści zaznaczenie. |
| Środkowy przycisk | Terminal | Wkleja. |
| Prawy przycisk | Wiersz paska bocznego | Otwiera menu wiersza. Zob. [Pasek boczny i menu](/pl/using/sidebar-and-menus/). |

## Powiększenie

Należy przytrzymać Ctrl i obracać kółkiem nad oknem huba, aby powiększać. Obrót w górę powiększa, a w dół pomniejsza, w krokach około 10 procent na zdarzenie kółka.

```text
Ctrl + wheel up      zoom in
Ctrl + wheel down    zoom out
```

- Zakres wynosi od 0,5x do 3x.
- Okno zmienia rozmiar o ten sam współczynnik, więc układ jest równie zwarty przy 2x jak przy 1x. Po osiągnięciu limitu okno przestaje rosnąć.
- Współczynnik jest zapisywany jako `uiScale` w `settings.json` i stosowany przy następnym uruchomieniu. Zapisany rozmiar okna jest już rozmiarem po powiększeniu, więc nie jest skalowany ponownie. Zob. [settings.json](/pl/data/settings-json/).

- Powiększenie dotyczy tylko okna huba. Okna Ustawień, O programie, edytora aplikacji i Pomocy
  zachowują własny rozmiar.

Nie ma kontrolki `uiScale` w Ustawieniach ani klawisza resetowania. Aby wrócić do normalnego
rozmiaru, należy obrócić kółkiem z powrotem o tyle samo kroków albo zakończyć Moonpool, ustawić
`uiScale` na `1` w `settings.json` (lub usunąć klucz) i uruchomić program ponownie.

## Zobacz także

- [Motywy, język i przezroczystość](/pl/using/themes-and-language/)
- [Pasek boczny i menu](/pl/using/sidebar-and-menus/)
