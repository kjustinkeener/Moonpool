---
title: "Uruchamianie serwera deweloperskiego npm w tle w systemie Windows bez okna terminala"
description: "Utrzymuj npm run dev, Vite lub inny serwer deweloperski w systemie Windows bez okna konsoli do pilnowania, a następnie uruchamiaj, zatrzymuj i czytaj jego wyjście z zasobnika."
---

Serwer deweloperski uruchomiony poleceniem `npm run dev` działa w terminalu, który go uruchomił,
więc zamknięcie tego okna go kończy. Zwykły sposób w systemie Windows na utrzymanie go to
proces ukryty, na przykład `Start-Process npm.cmd -ArgumentList "run","dev" -WindowStyle Hidden`
w PowerShell, ale wtedy nie ma wyjścia do przeczytania, a zatrzymanie go oznacza szukanie
właściwego `node.exe` (zob.
[Znajdowanie i kończenie procesu używającego portu](/pl/guides/find-and-kill-process-using-port-windows/)).

## Sposób Moonpool

Moonpool uruchamia polecenie we własnej wbudowanej karcie terminala w oknie huba, więc nie ma
osobnego okna konsoli, które trzeba trzymać otwarte. Po ukryciu okna huba w zasobniku serwer
nadal działa. Aplikację dodaje się raz:

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173",
  "openBrowser": true
}
```

Należy kliknąć kontrolkę **Uruchom** przy aplikacji. Kropka stanu staje się pełna, gdy `port`
zacznie odpowiadać, a przeglądarka otwiera `url` dzięki `openBrowser`. Kliknięcie nazwy aplikacji
pokazuje jej wyjście w jej własnej karcie. **Zatrzymaj** kończy terminal i wszystko, co uruchomił,
oraz zwalnia port (`killMode` `port` jest domyślny dla `web`).

## Utrzymanie działania po zamknięciu okna

Domyślnie przycisk zamknięcia kończy Moonpool, a w systemie Windows zakończenie programu zatrzymuje
każdą uruchomioną przez niego aplikację. Włączenie **Zamykanie do zasobnika** w
[Ustawieniach](/pl/using/settings/) sprawia, że zamknięcie okna tylko je ukrywa. Ikona w zasobniku
(lub **Pokaż Moonpool**) przywraca okno. Szczegóły są w
[Zasobnik, zamykanie i minimalizowanie](/pl/using/tray-and-closing/).

## Przewidywalny port

Moonpool ustala stan działania na podstawie `port`. Vite przechodzi na następny wolny port, gdy
jego port jest zajęty, co sprawiłoby, że Moonpool obserwowałby zły port. Należy przekazać
`--strictPort`, aby Vite zakończył działanie, i ustawić `port` odpowiednio:

```text title="command"
npm run dev -- --port 5173 --strictPort
```

Jeśli port jest już zajęty, zob.
[Naprawa EADDRINUSE i „Port 5173 is in use”](/pl/support/port-already-in-use/).

## Ograniczenia

- Moonpool nie uruchamia ponownie serwera, który uległ awarii. Pokazuje aplikację jako
  zatrzymaną, a karta wypisuje `[proces zakończony]`.
- Moonpool nie uruchamia się sam przy logowaniu do systemu Windows. Zob.
  [Automatyczne uruchamianie skryptu lub serwera deweloperskiego przy logowaniu do systemu Windows](/pl/guides/start-app-at-windows-login/).

## Zobacz także

- [Pola aplikacji](/pl/apps/fields/): `port`, `openBrowser`, `killMode`.
- [Typy aplikacji](/pl/apps/types/): jak ustalany jest stan działania dla `web`.
- [Zatrzymywanie i ponowne uruchamianie](/pl/apps/stop-and-restart/)
- [Przykłady](/pl/apps/examples/#serwer-deweloperski-web)
