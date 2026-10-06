---
title: "Znajdowanie i kończenie procesu używającego portu w systemie Windows (3000, 5173, 8080)"
description: "Znajdź proces zajmujący port 3000 lub 5173 w systemie Windows za pomocą netstat lub PowerShell, zakończ go poleceniem taskkill i pozwól Moonpool zwolnić port przy zatrzymaniu aplikacji."
---

Gdy serwer deweloperski zgłasza, że port jest już używany, coś innego nasłuchuje na tym porcie.
W wierszu polecenia (Command Prompt) można wyświetlić nasłuchujące procesy wraz z identyfikatorem
procesu-właściciela, a następnie go zakończyć:

```text frame="terminal"
netstat -ano | findstr :3000
taskkill /PID 12345 /F
```

Ostatnia kolumna wiersza `LISTENING` to PID (`findstr :3000` pasuje też do `:30001`, więc należy
sprawdzić adres lokalny). Polecenie `tasklist /FI "PID eq 12345"` pokazuje, który to program. W
PowerShell to samo wyszukiwanie wygląda tak:

```powershell frame="terminal"
Get-NetTCPConnection -LocalPort 3000 -State Listen | Select-Object LocalPort, OwningProcess
Get-Process -Id 12345
Stop-Process -Id 12345 -Force
```

Dodanie `/T` do `taskkill` kończy także procesy potomne danego procesu. Procesy należące do
innego użytkownika lub do systemu mogą wymagać okna z podniesionymi uprawnieniami (administratora).

## Sposób Moonpool

Dla aplikacji uruchamianej przez Moonpool nie trzeba szukać PID. Wystarczy nadać aplikacji `port`,
a Zatrzymaj go zwolni. Dla aplikacji `web` jest to domyślny `killMode`, tu zapisany jawnie:

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

Zatrzymaj najpierw kończy terminal uruchomiony przez Moonpool, a następnie wymusza zakończenie
wszystkiego, co nadal nasłuchuje na `port`. W systemie Windows jest to to samo wyszukiwanie co
powyżej (`Get-NetTCPConnection -LocalPort
<port> -State Listen`), a potem `taskkill /PID <pid> /T /F` dla każdego właściciela.

- Jeśli port zajmuje coś, czego nie uruchomiłeś(-aś), Moonpool pokazuje aplikację jako
  działającą, ale nie „zarządzaną przez Moonpool”. Należy nacisnąć **Zatrzymaj**: krok `port`
  i tak zostanie wykonany.
- Moonpool odmawia kończenia według portu stałej listy współdzielonych procesów systemu Windows,
  takich jak zaplecze Docker Desktop, `svchost` i host WSL. W przypadku aplikacji Docker należy
  użyć `killMode` o wartości `command` lub `none`, nigdy `port`. Zob.
  [Aplikacje Docker w systemie Windows](/pl/apps/stop-and-restart/#aplikacje-docker-w-systemie-windows).
- Działa to tylko dla portów aplikacji wymienionych w `apps.json`. Dla każdego innego portu
  należy użyć poleceń z początku strony.
- Tryb `port` kończy wszystko, co nasłuchuje, także kopię uruchomioną ręcznie, więc należy go
  używać tylko dla portów, których nic innego na komputerze nie potrzebuje.

## Zobacz także

- [Naprawa EADDRINUSE i „Port 5173 is in use”](/pl/support/port-already-in-use/)
- [Zatrzymywanie i ponowne uruchamianie](/pl/apps/stop-and-restart/)
- [Pola aplikacji](/pl/apps/fields/): `port` i `killMode`.
- [Dwie aplikacje używają tego samego portu](/pl/support/troubleshooting/#dwie-aplikacje-używają-tego-samego-portu)
