---
title: "Automatyczne uruchamianie skryptu lub serwera deweloperskiego przy logowaniu do systemu Windows"
description: "Uruchom Moonpool przy logowaniu do systemu Windows za pomocą skrótu w folderze Autostart, a potem uruchom w nim serwer deweloperski lub skrypt małym skryptem PowerShell. Żadne ustawienie tego nie robi."
---

System Windows ma dwa zwykłe sposoby uruchamiania czegoś przy logowaniu: skrót w folderze
Autostart (Win+R, wpisać `shell:startup`, nacisnąć Enter) albo zadanie Harmonogramu zadań z
wyzwalaczem „Przy logowaniu”. Każdy z nich uruchamia program lub skrypt, którym może być wprost
polecenie serwera deweloperskiego, ale wtedy nic go nie śledzi, nie pokazuje jego wyjścia ani nie
zatrzymuje go za Ciebie.

## Co oferuje Moonpool

Moonpool nie ma ustawienia uruchamiania przy logowaniu, a wpis w `apps.json` nie ma pola, które
uruchamiałoby go przy starcie Moonpool (pełna lista znajduje się w [Polach aplikacji](/pl/apps/fields/)
i [settings.json](/pl/data/settings-json/)). Można natomiast samodzielnie uruchomić Moonpool przy
logowaniu, a następnie skryptem uruchomić wybrane aplikacje, używając tego samego polecenia, które
oferuje [wiersz poleceń](/pl/automation/command-line/).

Najpierw należy zarejestrować aplikację jak zwykle:

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

Następnie należy zapisać poniższe jako `start-moonpool-apps.ps1`. W wersji zainstalowanej program
to `%USERPROFILE%\.moonpool\moonpool.exe`; dla kopii przenośnej należy użyć ścieżki do pliku exe
tej kopii.

```powershell title="start-moonpool-apps.ps1"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
Start-Process $mp
Start-Sleep -Seconds 15
& $mp launch site
```

Moonpool musi już działać, aby `launch` został mu przekazany; uruchomione bez działającej kopii,
to samo polecenie startuje nowy Moonpool, a polecenie nie jest wykonywane. Opóźnienie daje mu czas
na start, więc na wolnym komputerze należy je zwiększyć. Dla każdej aplikacji należy dodać jeden
wiersz `& $mp launch <id>`.

Na koniec należy umieścić skrót do skryptu w folderze Autostart, z takim celem:

```text title="Shortcut target"
powershell.exe -NoProfile -WindowStyle Hidden -File "C:\Users\you\start-moonpool-apps.ps1"
```

Aby sprawdzić, co się stało, należy dodać `--ticket t1` do polecenia i odczytać wynik z
`state.json` ([Odczyt wyniku](/pl/automation/command-line/#odczyt-wyniku)).

## Zastrzeżenia

- Serwer deweloperski uruchomiony w ten sposób jest „zarządzany” przez Moonpool jak każdy inny,
  więc Zatrzymaj i Zakończ na nim działają. Jeśli ta sama aplikacja już działa (na przykład
  uruchomiona ręcznie), Moonpool pokazuje ją jako działającą, ale niezarządzaną.
- Moonpool nie uruchamia ponownie aplikacji, która się zakończyła, i nie pamięta, które aplikacje
  działały przy ostatnim zamknięciu.

## Zobacz także

- [Wiersz poleceń](/pl/automation/command-line/)
- [Zasobnik, zamykanie i minimalizowanie](/pl/using/tray-and-closing/)
- [Uruchamianie serwera deweloperskiego npm w tle w systemie Windows](/pl/guides/run-npm-dev-server-in-background-windows/)
