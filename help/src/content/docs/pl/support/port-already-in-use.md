---
title: "Naprawa Error: listen EADDRINUSE: address already in use :::3000 i Vite Port 5173 is in use"
description: "Napraw EADDRINUSE w Node i Port 5173 is in use w Vite: znajdź, co zajmuje port, zwolnij go i użyj pól port i killMode w Moonpool, aby problem się nie powtarzał."
---

```text
Error: listen EADDRINUSE: address already in use :::3000
```

Ten błąd Node.js oznacza, że inny proces już nasłuchuje na porcie 3000 (`:::` to forma IPv6
oznaczająca „wszystkie adresy”; można też zobaczyć `127.0.0.1:3000`). Często jest to kopia tego
samego serwera, uruchomiona wcześniej i nigdy niezatrzymana.

Vite obsługuje tę samą sytuację inaczej. Domyślnie wypisuje:

```text
Port 5173 is in use, trying another one...
```

i startuje na następnym wolnym porcie, więc serwer działa, ale nie tam, gdzie się spodziewano. Z
`--strictPort` (lub `server.strictPort: true`) Vite kończy działanie, wypisując
`Error: Port 5173 is already in use`.

## Samodzielna naprawa

1. Należy znaleźć proces, który zajmuje port, i go zakończyć. W systemie Windows:

   ```text frame="terminal"
   netstat -ano | findstr :3000
   taskkill /PID 12345 /F
   ```

   Krok po kroku, wraz z wersją dla PowerShell, w
   [Znajdowaniu i kończeniu procesu używającego portu](/pl/guides/find-and-kill-process-using-port-windows/).
2. Albo uruchomić serwer na innym porcie, na przykład `PORT=3001` dla wielu serwerów Node lub
   `--port 5174` dla Vite.

## Jak pomaga Moonpool

Jeśli serwer jest uruchamiany przez Moonpool, należy ustawić `port` w jego wpisie. Wtedy Moonpool:

- pokazuje aplikację jako Działa, dopóki coś odpowiada na tym porcie, więc pozostały serwer
  zajmujący go pojawia się jako działający, ale nie „zarządzany przez Moonpool”;
- przy **Zatrzymaj** i **Uruchom ponownie** kończy wszystko, co nadal nasłuchuje na `port`, gdy
  `killMode` ma wartość `port` (domyślnie dla aplikacji `web`), więc następne Uruchom zastaje wolny
  port;
- oznacza dwie aplikacje skonfigurowane z tym samym `port`.

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "node server.js",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

Moonpool nie sprawdza portu przed uruchomieniem. Jeśli port jest nadal zajęty, polecenie wypisuje
powyższy błąd w karcie terminala aplikacji. Należy nacisnąć **Zatrzymaj** (co zwalnia port) i
ponownie **Uruchom**.

W przypadku Vite należy przekazać `--strictPort` i utrzymywać `port` równy żądanemu portowi:

```text title="command"
npm run dev -- --port 5173 --strictPort
```

Bez tego Vite może przejść na 5174, podczas gdy Moonpool nadal obserwuje 5173, a kropka stanu
nigdy nie stanie się pełna.

`killMode` `port` kończy każdy proces na porcie, więc należy go używać tylko dla portów, których
nic innego nie potrzebuje. Dla aplikacji Docker w systemie Windows nigdy go nie używać. Zob.
[Zatrzymywanie i ponowne uruchamianie](/pl/apps/stop-and-restart/#aplikacje-docker-w-systemie-windows).

## Zobacz także

- [Pola aplikacji](/pl/apps/fields/): `port`, `killMode`.
- [Zatrzymywanie i ponowne uruchamianie](/pl/apps/stop-and-restart/)
- [Rozwiązywanie problemów](/pl/support/troubleshooting/#dwie-aplikacje-używają-tego-samego-portu)
- [Uruchamianie serwera deweloperskiego npm w tle w systemie Windows](/pl/guides/run-npm-dev-server-in-background-windows/)
