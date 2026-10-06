---
title: "Dodawanie aplikacji lub serwera deweloperskiego do Moonpool"
description: "Zarejestruj lokalną aplikację z poleceniem uruchamiającym, folderem roboczym i środowiskiem, aby Moonpool mógł ją uruchamiać, zatrzymywać i nadzorować."
---

Każda aplikacja w Moonpool to jeden wpis z poleceniem uruchamiającym, folderem roboczym i opcjonalnym
środowiskiem. Moonpool wykonuje to polecenie we własnym, zarządzanym terminalu.

## Dodawanie aplikacji

1. Otwórz menu **...** u góry paska bocznego i wybierz **Dodaj aplikację**.
2. Wpisz **name** i wybierz **group**.
3. Wybierz **type**: `web` (serwer na porcie), `desktop` (aplikacja natywna), `static` (strona) lub `cli` (polecenie).
4. Ustaw **command** oraz **cwd**, w którym polecenie jest uruchamiane.
5. Uzupełnij to, czego wymaga dany typ: **port** i **url** dla web, **processName** dla desktop,
   **url** dla static. Aplikacja `static` z samym `url` nie wymaga **command** ani **cwd**.
6. Zapisz. Aplikacja pojawi się na pasku bocznym. Użyj jej przycisku **Uruchom**, aby ją wystartować.

![Lista wyboru type (1) i pole port (2) w edytorze aplikacji, a między nimi cwd i command](../../../../assets/screenshots/edit-app-type-and-port.png)

1. Lista wyboru **type**; jej podpowiedź opisuje, jak działa dany typ.
2. Pole **port**, używane przez aplikacje `web`.

Wynikiem jest jeden wpis w `apps.json`, na przykład:

```json title="apps.json"
{
  "id": "my-api",
  "name": "My API",
  "group": "Dev",
  "type": "web",
  "command": "npm run dev",
  "cwd": "C:\\code\\my-api",
  "port": 3000,
  "url": "http://localhost:3000"
}
```

Kliknięcie nazwy aplikacji tylko otwiera jej kartę terminala; zob. [Stany aplikacji](/pl/support/glossary/#stany-aplikacji).

## Edytor aplikacji

- **Grupa.** Wybierz grupę z listy albo wybierz **+ Nowa grupa...** i wpisz nazwę.
  **powrót do listy** wraca do listy. Pusta grupa jest zapisywana jako `Apps`.
- **Przyciemnione pola** nie są używane przez wybrany typ. Mimo to są zapisywane.
- **Zapis bez nazwy** pokazuje komunikat `nazwa jest wymagana.`
- **Esc** lub zamknięcie edytora z niezapisanymi zmianami wyświetla pytanie „Odrzucić wprowadzone zmiany?”.
- Aby później zmienić aplikację, użyj ołówka w jej wierszu albo kliknij ją prawym przyciskiem myszy i wybierz **Edytuj**.

## Edycja ręczna

Wybierz **Edytuj apps.json** w tym samym menu, zapisz plik, a następnie wybierz **Odśwież**.
Format, reguły walidacji i opcje odzyskiwania opisano w
[Przeglądzie konfiguracji](/pl/apps/apps-json/).

## Co dalej

- [Pola aplikacji](/pl/apps/fields/): każdy klucz i jego działanie.
- [Typy aplikacji](/pl/apps/types/): jak uruchamia się każdy typ i kiedy pokazuje stan „działa”.
- [Zatrzymywanie i ponowne uruchamianie](/pl/apps/stop-and-restart/): co ustawić, gdy po zatrzymaniu coś nadal działa, i dlaczego aplikacje Docker wymagają ostrożności.
- [Ścieżki i środowisko](/pl/apps/paths-and-environment/): `{MP_HOME}`, ścieżki `./` i `env`.
- [Przykłady](/pl/apps/examples/): kompletne wpisy do skopiowania.
- [Poradniki](/pl/guides/run-npm-dev-server-in-background-windows/): serwery deweloperskie w tle, skrypty Pythona, porty.
- [Tryb przenośny](/pl/data/portable-mode/)
- [Aktualizowanie](/pl/data/updating/)
