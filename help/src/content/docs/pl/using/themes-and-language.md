---
title: "Zmiana motywu, języka i przezroczystości Moonpool"
description: "Wybierz motyw kolorystyczny i język interfejsu, ustaw przezroczystość tła i skalę interfejsu oraz zobacz, jak natychmiast stosują się do każdego otwartego okna Moonpool."
---

Motyw, język i przezroczystość ustawia się w [oknie Ustawień](/pl/using/settings/).
Wszystkie trzy stosują się natychmiast do każdego otwartego okna Moonpool.

![Listy wyboru Języka (1) i Motywu (2) u góry Ustawień](../../../../assets/screenshots/settings-language-theme.png)

1. Lista wyboru języka.
2. Przycisk motywu. Pokazuje nazwę bieżącego motywu i otwiera przeglądarkę motywów.

## Motywy

Przeglądarka motywów jest osobnym oknem. Ma jedną kartę podglądu na każdy motyw, narysowaną we
własnych kolorach tego motywu (tekst, panel, pole wprowadzania, przycisk, kropki stanu, gradient
wskaźnika i 16-kolorowy zestaw terminala), pogrupowane jako Podstawowe, Neon, Ciepłe, Chłodne,
Zielenie, Neutralne, Jasne, Róże, Żywe, Jasny pastel i Pastel. Kliknięcie karty stosuje motyw:
każde otwarte okno Moonpool zmienia się jednocześnie, a wybór jest zapisywany. Okno pozostaje
otwarte, aby można było porównywać; klawisz Esc je zamyka.

Jest 68 motywów oraz Automatycznie, a mniej więcej połowa z nich jest jasna. Nazwy motywów są
nazwami własnymi i nie są tłumaczone; tłumaczone są tylko Automatycznie (system), Ciemny i Jasny.

**Automatycznie (system)** podąża za jasnym lub ciemnym ustawieniem systemu operacyjnego i
przełącza się na żywo, gdy zmienia się ono w systemie. Każdy inny wybór jest stały. 16 kolorów
ANSI terminala również podąża za motywem.

Jeśli motyw został zapisany w starszej wersji, zostaje zachowany. Zapisana nazwa, której Moonpool
już nie zna, wraca do Automatycznie. Niektóre etykiety różnią się od dawnych (na przykład Matrix
nazywa się teraz Terminal, Nord to Arctic, Dracula to Nocturne, Gruvbox to Retro, a Solarized to
Solar); sam zapisany wybór pozostaje niezmieniony.

Motyw jest przechowywany w `localStorage` widoku WWW, a nie w `settings.json`. Jeśli pamięć jest
niedostępna, następuje powrót do Automatycznie.

```text
localStorage key: moonpool.theme
```

## Języki

Automatycznie podąża za językiem systemu. W przeciwnym razie należy wybrać jeden z 14, każdy
pokazany w swoim własnym języku:

```text
English, Deutsch, Español, Français, Italiano, 日本語, 한국어, Nederlands, Polski,
Português (Brasil), Русский, Türkçe, 简体中文, 繁體中文
```

Lista wyboru stosuje się natychmiast w oknie huba i w pozostałych oknach. Wybór jest zapisywany
jako `locale` w `settings.json`.

## Przezroczystość

**Przezroczystość tła** sprawia, że tło okna jest półprzezroczyste, od 0% (pełne krycie) do
90%.

- Najechanie wskaźnikiem na okno natychmiast czyni je w pełni nieprzezroczystym. Gdy wskaźnik odjeżdża, okno przez około 2 sekundy wraca do ustawionej wartości.
- Terminale podążają za tym samym odcieniem, zamiast dodawać własny.
- Każde okno (hub, Ustawienia, O programie, edytor aplikacji i przeglądarka motywów) stosuje ustawienie samodzielnie, a Ustawienia aktualizują pozostałe na żywo podczas przeciągania suwaka.

## Skala interfejsu

Cały interfejs powiększa się kombinacją Ctrl + kółko myszy. Nie ma powiększania klawiaturą. Zob.
[Skróty i powiększenie](/pl/using/keyboard-shortcuts/).

## Zobacz także

- [Okno ustawień](/pl/using/settings/)
- [Skróty i powiększenie](/pl/using/keyboard-shortcuts/)
