---
title: "Rozwiązywanie problemów z Moonpool: zasobnik, aplikacje, które się nie uruchamiają, aktualizacje"
description: "Napraw typowe problemy z Moonpool według objawów: brak ikony w zasobniku, aplikacje, które się nie uruchamiają lub nie zatrzymują, błędne kropki stanu, nieudane aktualizacje i błędy MCP."
---

Należy znaleźć objaw, a następnie zastosować rozwiązanie. Cytowany tekst to to, co pokazuje
Moonpool. Aby sprawdzić dokładny komunikat, zob. [Objaśnienie komunikatów o błędach](/pl/support/error-messages/).

## Nie widzę ikony w zasobniku

- **Windows.** Ikona może znajdować się w obszarze ukrytych ikon. Należy kliknąć strzałkę **^**
  po prawej stronie paska zadań. Przeciągnięcie ikony na pasek zadań sprawia, że pozostaje widoczna.
- **Linux ze standardowym GNOME.** GNOME nie pokazuje ikon w zasobniku bez rozszerzenia
  AppIndicator. Zob. [Linux](/pl/platforms/linux/#zasobnik-w-gnome).
- **Ustawienia.** Opcja **Pokaż w zasobniku** może być wyłączona. Należy otworzyć okno huba z paska
  zadań lub menu Start i włączyć ją ponownie w [Ustawieniach](/pl/using/settings/).

## Instalator pokazuje błąd

| Komunikat | Co zrobić |
| --- | --- |
| `Instalacja nie powiodła się: <error>` | Tekst po dwukropku wskazuje krok, który się nie powiódł, na przykład `copy exe: ...`. Jeśli plik jest w użyciu, należy zakończyć każdy Moonpool uruchomiony z `%USERPROFILE%\.moonpool` i spróbować ponownie. |
| `target folder does not exist` | Folder wybrany dla kopii przenośnej już nie istnieje. Należy wybrać istniejący folder. |
| `that folder already has a .moonpool with an exe of this name that isn't a portable Moonpool - pick an empty folder` | Należy wybrać pusty folder albo najpierw usunąć ten folder `.moonpool`. |

## Przy uruchamianiu instalatora pojawia się komunikat System Windows ochronił ten komputer

To Windows SmartScreen, ponieważ `moonpool.exe` nie jest podpisany cyfrowo. Należy kliknąć **More
info** („Więcej informacji”), a następnie **Run anyway** („Uruchom mimo to”). Zob.
[System Windows ochronił ten komputer](/pl/support/windows-protected-your-pc/).

## Okno Moonpool jest puste lub nigdy się nie otwiera w systemie Windows

Może brakować środowiska Microsoft Edge WebView2 Runtime. Zob.
[Brak środowiska uruchomieniowego WebView2](/pl/support/webview2-runtime-missing/).

## Aplikacja się nie uruchamia

1. Należy kliknąć nazwę aplikacji, aby otworzyć jej kartę terminala i przeczytać wyjście. Agent
   może odczytać ten sam tekst przez `moonpool_app_output`.
2. Należy sprawdzić `cwd`. Zwykłą przyczyną jest brakujący folder lub ścieżka względna bez `./`.
   Zob. [Ścieżki i środowisko](/pl/apps/paths-and-environment/).
3. Należy sprawdzić `command`. Należy uruchomić je ręcznie w terminalu w `cwd`. W systemie Windows
   unikać zagnieżdżonych cudzysłowów podwójnych; `cmd /c` je psuje.
4. Należy włączyć w Ustawieniach opcję **Zapisuj informacje diagnostyczne do pliku** i uruchomić
   ponownie. `moonpool.log` rejestruje dokładne polecenie i folder. Zob. [Dzienniki](/pl/data/logs/).

| Komunikat | Znaczenie |
| --- | --- |
| `already running` | Moonpool ma już terminal dla tej aplikacji. Należy ją najpierw zatrzymać albo użyć Uruchom ponownie. |
| `stopped during launch` | Naciśnięto Zatrzymaj, gdy uruchamianie jeszcze trwało. |
| `did not reach running in time` | Ze skryptu lub od agenta: aplikacja nie została rozpoznana jako działająca w ciągu 25 sekund. Należy sprawdzić jej `port` lub `processName` oraz jej wyjście. |

## Kropka stanu jest nieprawidłowa

Moonpool ustala stan Działa na podstawie `port`, następnie `processName`, a potem tego, czy jego
własny terminal nadal żyje. Zob. [Jak ustalany jest stan Działa](/pl/apps/types/#jak-ustalany-jest-stan-działa).

- **Nigdy nie staje się pełna.** `port` aplikacji `web` nie odpowiada albo `processName` aplikacji
  `desktop` nie pasuje. W systemie Linux `processName` może mieć najwyżej 15 znaków.
- **Robi się szara zaraz po uruchomieniu.** Aplikacja `cli` przestaje działać, gdy jej polecenie się
  zakończy. Jeśli ma pozostać otwarta, należy użyć powłoki z `-NoExit`.
- **Aplikacja `static` nigdy nie pokazuje stanu Działa.** Jest to oczekiwane dla wpisu tylko z `url`.
- **Pokazuje Działa, choć nie została uruchomiona przez Ciebie.** Coś innego używa tego portu lub
  tej nazwy procesu. Moonpool pokazuje ją jako działającą, ale nie „zarządzaną przez Moonpool”.

## Error: listen EADDRINUSE lub „Port 5173 is in use”

Coś innego już nasłuchuje na porcie, którego chce serwer. Należy to znaleźć i zakończyć albo
ustawić `port` w aplikacji, aby Zatrzymaj go zwolnił. Zob.
[Naprawa EADDRINUSE i „Port 5173 is in use”](/pl/support/port-already-in-use/) oraz
[Znajdowanie i kończenie procesu używającego portu](/pl/guides/find-and-kill-process-using-port-windows/).

## Dwie aplikacje używają tego samego portu

U dołu menu **...** pojawia się wiersz ostrzeżenia, na przykład `port 3000: App A / App B`.
Należy zmienić `port` jednej z aplikacji (oraz jej `env`, jeśli czyta `PORT`). Zob.
[Ostrzeżenie o konflikcie portów](/pl/using/hub-window/#ostrzeżenie-o-konflikcie-portów).

## Aplikacja nadal działa po Zatrzymaj

Ze skryptu lub od agenta błąd brzmi `still running after stop` (po 15 sekundach).

- Aplikacja przeżywa swój terminal. Należy ustawić `killMode` na `port` lub `processName`. Zob.
  [Zatrzymywanie i ponowne uruchamianie](/pl/apps/stop-and-restart/).
- Aplikacja Docker w systemie Windows: należy użyć `killMode` `command` z `stopCommand`, na przykład
  `docker compose stop app`. Nigdy `port`.

## apps.json zawiera błąd

Pasek boczny pokazuje baner „apps.json zawiera błąd; wyświetlana jest ostatnio wczytana lista.” lub,
przy uruchomieniu, „apps.json zawiera błąd, więc nie wczytano żadnych aplikacji.” Zapisywanie z
Moonpool jest wstrzymane, dopóki plik nie wczyta się ponownie.

Typowe błędy:

```text
apps.json entry 2 (site) requires a command
apps.json entry 3 has invalid id "my app"; use letters, digits, '.', '_', and '-' without a leading '-'
duplicate app id "site"
apps.json entry 4 (api) has invalid port 0
```

1. Należy wybrać **Edytuj apps.json** w banerze, poprawić wpis, zapisać, a następnie **Odśwież** (F5).
2. Albo wrócić do niedawnej dobrej kopii. Zob.
   [Kopia zapasowa i odzyskiwanie](/pl/data/backup-and-recovery/#wycofanie-appsjson).

Pełną listę reguł podano w [Walidacji](/pl/apps/apps-json/#walidacja).

Jeśli ustawienia nie da się zmienić, a komunikat kończy się słowami `Repair settings.json and restart
Moonpool before changing settings`, należy naprawić lub usunąć `settings.json` w folderze
konfiguracji i uruchomić Moonpool ponownie. Usunięcie go przywraca wszystkim ustawieniom wartości
domyślne.

## Moja zmiana nie zadziałała

- Ręczne zmiany wymagają **Odśwież** (lub F5). Moonpool nie obserwuje pliku.
- Odśwież nie uruchamia ponownie działających aplikacji. Aby użyć zmienionego `command`, `cwd` lub
  `env`, należy ponownie uruchomić aplikację.
- Agent może edytować inny `apps.json`. Należy poprosić go o wywołanie `moonpool_launcher_paths` i
  porównanie folderu huba z własnym. Przy kilku kopiach Moonpool należy sprawdzić, którą kopię
  edytuje agent.

## Brakuje przykładowych aplikacji

Przykłady są zapisywane tylko wtedy, gdy nie istnieje `apps.json`. Aby je odzyskać, zob.
[Przywracanie przykładów](/pl/data/backup-and-recovery/#powrót-do-przykładów) albo skopiować
wpisy z [Przykładowych pulpitów](/pl/getting-started/example-dashboards/#przykładowe-aplikacje-pojawiają-się-tylko-przy-pierwszym-uruchomieniu).

## Aktualizacja się nie powiodła

Baner pokazuje `Aktualizacja nie powiodła się: <error>`. Zob.
[Gdy aktualizacja się nie powiedzie](/pl/data/updating/#gdy-aktualizacja-się-nie-powiedzie).

## Link internetowy się nie otwiera

`refusing to open non-web url: <url>` oznacza, że `url` nie zaczyna się od `http://`, `https://`,
`mailto:` ani `file://`. Należy poprawić `url`.

## Błędy MCP i skryptów

| Komunikat | Co zrobić |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Należy uruchomić Moonpool albo pozwolić agentowi wywołać `moonpool_bootup_launcher`. |
| `frontend not loaded` | Okno huba nie zakończyło jeszcze wczytywania. Należy chwilę poczekać i spróbować ponownie. |
| `stale token: ...` | `apps.json` zmienił się od chwili jego odczytu przez agenta. Należy odczytać go ponownie, a potem zapisać. |
| `rejected invalid manifest: ...` | Nowy `apps.json` nie przeszedł walidacji. Plik nie został zmieniony. |
| `... A Moonpool process may be hung ...` | Coś zajmuje kanał sterowania i nie odpowiada. Należy zakończyć Moonpool z zasobnika albo zakończyć proces i uruchomić go ponownie. |

Więcej w [Konfiguracji MCP](/pl/automation/mcp-setup/#uwagi) i
[Narzędziach MCP](/pl/automation/mcp-tools/).

## Problemy z oknem

- **Poza ekranem.** Moonpool ignoruje zapisane położenie, które nie znajduje się na żadnym
  podłączonym ekranie. Jeśli okno nadal jest zgubione, należy zakończyć Moonpool i usunąć
  `window-state.json` w folderze konfiguracji.
- **Powiększenie zablokowane na zbyt dużym lub zbyt małym.** Zmienia je Ctrl + kółko nad oknem huba.
  Zob. [Skróty i powiększenie](/pl/using/keyboard-shortcuts/#powiększenie).
- **Ustawienia otwierają się za oknem huba.** Należy wyłączyć lub włączyć w Ustawieniach opcję
  **Zawsze na wierzchu**. Dotyczy ona każdego okna Moonpool, więc wszystkie pozostają w tej samej
  warstwie.

## Gdzie są dzienniki?

Zob. [Dzienniki](/pl/data/logs/).

## Kopia zapasowa, reset lub odinstalowanie

Zob. [Kopia zapasowa i odzyskiwanie](/pl/data/backup-and-recovery/) oraz
[Odinstalowanie](/pl/getting-started/install/#odinstalowanie).

## FAQ

**Czy zamknięcie okna zatrzymuje moje aplikacje?**
Domyślnie zamknięcie kończy Moonpool, a w systemie Windows zakończenie programu zatrzymuje
uruchomione przez niego aplikacje. Włączenie **Zamykanie do zasobnika** utrzymuje Moonpool w
działaniu po zamknięciu okna. Zob.
[Zasobnik, zamykanie i minimalizowanie](/pl/using/tray-and-closing/).

**Czy mogę uruchomić Moonpool dwa razy?**
Jedną kopię na folder. Ponowne uruchomienie tej samej kopii przywraca jej okno. Kopia zainstalowana
i kopie przenośne mogą działać obok siebie. Zob.
[Tryb przenośny](/pl/data/portable-mode/#kilka-kopii-naraz).

**Czy Moonpool łączy się z internetem w swoich sprawach?**
Tylko po to, aby sprawdzić aktualizacje: pobiera plik wydania (`update.json`) z GitHub przy starcie
(jeśli włączona jest opcja **Sprawdzaj aktualizacje przy starcie**) i po naciśnięciu **Sprawdź
aktualizacje**. Każde pobranie jest weryfikowane kluczem podpisu Moonpool, zanim zostanie użyte.

**Która powłoka uruchamia moje polecenia?**
`cmd /c` w systemie Windows, `$SHELL -c` w systemie Linux.

**Gdzie umieszczać sekrety?**
Wartości `env` są przechowywane jako zwykły tekst w `apps.json`. Lepiej użyć pliku, który aplikacja
czyta sama, albo zmiennej już ustawionej w środowisku użytkownika, którą dziedziczą uruchamiane
aplikacje.

**Czy kanał sterowania jest chroniony?**
Nie ma logowania ani tokenu. Każdy proces działający jako Ty może wysyłać mu polecenia. W
systemie Linux gniazdo jest czytelne tylko dla Twojego użytkownika. Zob.
[Właściwości bezpieczeństwa](/pl/automation/overview/#właściwości-bezpieczeństwa).
