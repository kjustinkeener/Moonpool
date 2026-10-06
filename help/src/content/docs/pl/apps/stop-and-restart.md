---
title: "Zatrzymywanie serwera deweloperskiego i wszystkiego, co uruchomił"
description: "Dzięki killMode i stopCommand zatrzymanie i ponowne uruchomienie czysto kończą aplikację i jej procesy potomne, z ustawieniami domyślnymi i Dockerem w systemie Windows."
---

Zatrzymanie zawsze najpierw robi to: Moonpool kończy terminal uruchomiony dla aplikacji wraz
ze wszystkim, co ten terminal uruchomił. W przypadku wielu aplikacji to wystarcza.

Niektóre aplikacje przeżywają ten terminal (okno aplikacji desktopowej odłącza się od serwera deweloperskiego,
który je uruchomił, albo podproces serwera nadal zajmuje swój port). **`killMode`** wybiera jeden
dodatkowy krok wykonywany potem.

| `killMode` | Dodatkowy krok przy zatrzymaniu | Odczytuje | Domyślnie dla |
| --- | --- | --- | --- |
| `processName` | Wymusza zakończenie każdego procesu o tej nazwie. W systemie Windows także jego procesów potomnych (`taskkill /IM <name>.exe /T /F`). W innych systemach `pkill -KILL -x <name>`: dokładne dopasowanie nazwy z rozróżnianiem wielkości liter, bez procesów potomnych. | `processName` | `desktop` |
| `port` | Wymusza zakończenie procesu nasłuchującego na `port`. | `port` | `web` |
| `command` | Uruchamia `stopCommand` w `cwd` i czeka na jego zakończenie. | `stopCommand`, `cwd`, `env` | nic |
| `none` | Nic. | nic | `static`, `cli` |

Pomiń `killMode`, aby użyć wartości domyślnej dla typu aplikacji, i ustaw go tylko wtedy, gdy po zatrzymaniu
coś nadal działa.

![Lista killMode w oknie Edytuj aplikację, ustawiona na „domyślnie (wg typu)”, z wierszem podpowiedzi opisującym domyślne działanie każdego typu](../../../../assets/screenshots/edit-app-killmode.png)

1. Lista wyboru `killMode`. „domyślnie (wg typu)” to to samo, co pominięcie klucza.

- Jeśli pole potrzebne dla danego trybu jest puste (na przykład tryb `port` bez `port`), dodatkowy
  krok jest pomijany. Nie jest to błąd.
- `killMode` jest niezależne od `type`: `port` działa w aplikacji `cli`, a `processName` w aplikacji
  `web`.
- Pusty ciąg lub nierozpoznana wartość nie powoduje żadnego dodatkowego działania. Nie wraca się wtedy do
  wartości domyślnej dla typu.

Dla aplikacji desktopowej tryb `processName` wykonuje odpowiednik:

```powershell frame="terminal"
taskkill /IM notes-app.exe /T /F
```

## Kilka kopii Moonpool lub własne procesy

`processName` i `port` nie wiedzą, kto uruchomił proces. `processName` kończy każdy
proces o tej nazwie, a `port` kończy wszystko, co nasłuchuje na porcie, także proces uruchomiony przez
inną kopię Moonpool (zainstalowana i przenośne kopie działają niezależnie; zob.
[Tryb przenośny](/pl/data/portable-mode/#kilka-kopii-naraz)) oraz proces uruchomiony samodzielnie.
Tych trybów należy używać tylko dla aplikacji, które nie będą w ten sposób kolidować: nazwa lub port, których nic innego
na komputerze nie używa. Jeśli dwie kopie rejestrują tę samą aplikację albo jest ona uruchamiana także ręcznie, należy ustawić
`killMode` `none` lub `command`, który zatrzymuje tylko własną instancję.

## stopCommand

Używane tylko wtedy, gdy `killMode` ma wartość `command`. Jest wykonywane przez `cmd /c` w systemie Windows i `$SHELL -c`
w pozostałych, w `cwd`, z dodanym `env`. Działają w nim `{MP_HOME}` i `{MP_DATA}`. Moonpool
czeka na jego zakończenie, zanim zrobi cokolwiek innego, więc ponowne uruchomienie nigdy nie startuje, gdy
polecenie jeszcze działa. Jego kod wyjścia jest ignorowany. Jeśli nadal działa po 60 sekundach,
Moonpool kończy je wraz z procesami potomnymi i kontynuuje.

## Ponowne uruchomienie

Ponowne uruchomienie to Zatrzymanie, a po nim Uruchomienie tego samego `command`. Moonpool czeka do 4 sekund,
aż stara instancja będzie widoczna jako zatrzymana (aby jej port był wolny), a dopiero potem uruchamia ją ponownie. Wpis `static`
z samym `url` nie ma czego zatrzymywać: ponowne uruchomienie po prostu ponownie otwiera stronę.

## Aplikacje Docker w systemie Windows

Należy użyć `none` albo `command` z prawdziwym poleceniem zatrzymującym, takim jak `docker compose stop app`. Nie
należy używać `port`.

Docker Desktop publikuje port każdego kontenera przez jeden wspólny proces w tle. W systemie
Windows „to, co nasłuchuje na porcie”, jest właśnie tym wspólnym procesem, więc tryb `port` wymusiłby
zakończenie Docker Desktop i zatrzymałby wszystkie kontenery, a nie tylko tę aplikację. Jako zabezpieczenie
Moonpool odmawia kończenia według portu ustalonej listy wspólnych procesów systemu Windows: procesów backendu,
proxy i usługi Docker Desktop, `dockerd`, `vpnkit`, procesów hosta WSL oraz podstawowych
procesów systemowych, takich jak `svchost`. To nie zastępuje wyboru właściwego trybu.

Jeśli `command` już odtwarza kontener (`docker compose up -d --build`), poprawne jest `none`:
ponowne uruchomienie po prostu wykonuje je jeszcze raz.

Zob. także [Znajdowanie i kończenie procesu używającego portu](/pl/guides/find-and-kill-process-using-port-windows/)
oraz [Naprawa EADDRINUSE i „Port 5173 is in use”](/pl/support/port-already-in-use/).

## Przykłady

Serwer deweloperski, który czasem zostawia proces node zajmujący port (to jest wartość domyślna dla
`web`, tutaj podana jawnie):

```json title="apps.json"
{ "id": "site", "name": "Site", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
  "killMode": "port" }
```

Aplikacja Docker Compose:

```json title="apps.json"
{ "id": "api", "name": "API", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\api", "command": "docker compose up -d --build", "port": 8080,
  "killMode": "command", "stopCommand": "docker compose stop app" }
```
