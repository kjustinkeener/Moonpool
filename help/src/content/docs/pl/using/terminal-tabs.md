---
title: "Karty terminala Moonpool: otwieranie, zamykanie, kopiowanie, ponowne uruchamianie"
description: "Praca z kartami terminala poszczególnych aplikacji w oknie huba: otwieranie i zamykanie kart, zwijanie panelu, kopiowanie i wklejanie, restart sesji i dzienniki sesji."
---

Każda aplikacja działa we własnej karcie terminala w panelu CLI.

## Karty

![Pasek kart z aktywną kartą Metrics Dashboard (zaznaczona) i jej bieżącym dziennikiem poniżej; każda karta ma kropkę i znak x](../../../../assets/screenshots/hub-terminal-tab.png)

- Uruchomienie aplikacji lub kliknięcie jej nazwy na pasku bocznym otwiera jej kartę. Kliknięcie nazwy niczego nie uruchamia; zob. [Stany aplikacji](/pl/support/glossary/#stany-aplikacji).
- Kropka na karcie świeci, gdy aplikacja działa.
- Znak **x** na karcie zamyka kartę. Nie zatrzymuje aplikacji. Ponowne kliknięcie nazwy otwiera kartę ponownie; pokazuje ona dziennik z bieżącej sesji.

## Zwijanie panelu

Znak **x** na samym prawym końcu paska kart („Ukryj panel CLI”) zwija panel CLI i zmniejsza okno
do samego paska bocznego. Terminale nadal działają i zachowują swoją historię przewijania.

Obok pola filtru pojawia się strzałka, która przywraca panel w poprzedniej szerokości.
Strzałka pulsuje, gdy czeka aktualizacja, ponieważ baner aktualizacji znajduje się w panelu.

## Kopiowanie i wklejanie

| Czynność | Wynik |
| --- | --- |
| Zaznaczenie tekstu myszą | Skopiowanie do schowka po puszczeniu przycisku, po czym zaznaczenie jest czyszczone. |
| Kliknięcie środkowym przyciskiem | Wkleja zawartość schowka do terminala. |
| Przycisk **Kopiuj wszystko** (u góry po prawej, pojawia się po najechaniu) | Kopiuje całą historię przewijania jako tekst. |

## Historia przewijania

Każdy terminal przechowuje 10 000 wierszy.

## Gdy proces się kończy

Gdy proces się kończy, terminal wypisuje:

```text
[proces zakończony]
```

Karta pozostaje otwarta z nienaruszonym wyjściem. Wiersz `[proces zakończony]` jest pokazywany w
Twoim języku.

## Restart

**Uruchom ponownie** (lub Uruchom przy zatrzymanej aplikacji) rozpoczyna nowe uruchomienie w tej
samej karcie. Karta jest budowana od nowa, a wcześniejsze wyjście tej sesji jest do niej
odtwarzane z dziennika sesji.

Jeśli aplikacja działała już wcześniej w tej sesji, Moonpool najpierw zapisuje w dzienniku sesji
przyciemniony separator, który jest widoczny między starym wyjściem a nowym uruchomieniem:

```text
---------- restarted 2026-10-05 09:14:02 ----------
```

Jeśli nowe uruchomienie zaczyna się od czyszczenia ekranu, wcześniejsze wyjście trafia do
historii przewijania zamiast zostać wymazane.

## Dzienniki sesji

Wszystko, co aplikacja wypisuje, jest też zapisywane w pliku dziennika w `cli-output\`, po jednym
pliku na aplikację na sesję Moonpool. Lokalizację, przechowywanie i ustawienie **Zachowuj dzienniki wyjścia aplikacji między sesjami** opisano w [Dziennikach](/pl/data/logs/).
