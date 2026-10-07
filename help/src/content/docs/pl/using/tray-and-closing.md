---
title: "Moonpool w zasobniku systemowym: zamykanie, minimalizowanie i kończenie pracy"
description: "Steruj tym, co robią ikona w zasobniku, przycisk zamknięcia, minimalizowanie i Zakończ, utrzymuj okno zawsze na wierzchu i unikaj ukrycia jednocześnie zasobnika i paska zadań."
---

## Ikona w zasobniku

| Czynność | Wynik |
| --- | --- |
| Kliknięcie lewym przyciskiem | Pokazuje okno huba (przywraca je, jeśli zostało zminimalizowane lub ukryte). |
| Kliknięcie prawym przyciskiem | Menu z wyłącznie pozycjami **Pokaż Moonpool** i **Zakończ** (w Twoim języku). |

Gdy działa kilka kopii Moonpool, każda ma własną ikonę w zasobniku. Podpowiedź informuje, o którą
kopię chodzi. Zob. [Tryb przenośny](/pl/data/portable-mode/#kilka-kopii-naraz).

## Zakończ

**Zakończ** zamyka Moonpool, a w systemie Windows zatrzymuje każdą aplikację uruchomioną przez
Moonpool, łącznie z ich procesami potomnymi. Aplikacje, które działały już, zanim Moonpool je
zobaczył (pokazane jako działające bez „zarządzane przez Moonpool”), pozostają nietknięte. W
systemie Linux zakończenie pracy nie zatrzymuje niezawodnie uruchomionych aplikacji.

## Zamykanie i minimalizowanie

Przycisk zamknięcia domyślnie kończy Moonpool (`closeToTray` ma wartość `false`). Po włączeniu
**Zamykanie do zasobnika** w Ustawieniach zamknięcie ukrywa okno w zasobniku. Moonpool nadal
działa, a ikona w zasobniku lub **Pokaż Moonpool** przywraca okno.

**Minimalizowanie do zasobnika** (`minimizeToTray`, domyślnie włączone) ukrywa okno w zasobniku po
zminimalizowaniu i znika ono z paska zadań. Po wyłączeniu okno minimalizuje się do paska zadań jak
zwykle.

![Ustawienia: Zamykanie do zasobnika i Minimalizowanie do zasobnika (1) oraz suwak Przezroczystość tła (2)](../../../../assets/screenshots/settings-tray-and-transparency.png)

1. **Zamykanie do zasobnika** i **Minimalizowanie do zasobnika**.
2. **Przezroczystość tła**. Zob. [Motywy, język i przezroczystość](/pl/using/themes-and-language/#przezroczystość).

## Blokada zasobnika i paska zadań

**Pokaż w zasobniku** i **Pokaż na pasku zadań** określają, czy ikona w zasobniku i przycisk na
pasku zadań są widoczne. Co najmniej jedno musi pozostać włączone, bo inaczej ukryte okno nie
miałoby drogi powrotnej. Gdy włączone jest tylko jedno, jego pole wyboru jest wyłączone, dopóki
drugie nie zostanie ponownie włączone.

## Zawsze na wierzchu

**Zawsze na wierzchu** w Ustawieniach utrzymuje każde okno Moonpool (hub, Ustawienia, O programie,
edytor aplikacji, przeglądarkę motywów, instalator i Pomoc) nad innymi oknami. Domyślnie jest
wyłączone.

## Zobacz także

- [Uruchamianie serwera deweloperskiego npm w tle w systemie Windows](/pl/guides/run-npm-dev-server-in-background-windows/)
- [Automatyczne uruchamianie skryptu lub serwera deweloperskiego przy logowaniu do systemu Windows](/pl/guides/start-app-at-windows-login/)
- [Okno ustawień](/pl/using/settings/)
- [Okno huba](/pl/using/hub-window/)
