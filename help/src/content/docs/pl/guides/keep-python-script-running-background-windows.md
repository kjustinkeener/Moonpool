---
title: "Utrzymywanie działania skryptu Pythona w tle w systemie Windows"
description: "Uruchom długo działający skrypt Pythona lub małą aplikację WWW w tle w systemie Windows, oglądaj jej wyjście i czysto ją zatrzymuj, z pythonw i z Moonpool."
---

Skrypt Pythona uruchomiony z okna konsoli przestaje działać, gdy to okno zostanie zamknięte.
Zwykłe sposoby w systemie Windows to `pythonw.exe` (ten sam interpreter bez okna konsoli, więc
wyjście trafia donikąd), `Start-Process pythonw -ArgumentList worker.py`, aby uruchomić go
odłączonego, albo zaplanowane zadanie dla czegoś, co ma działać przy logowaniu lub według
harmonogramu. Przy każdym z nich proces trzeba samodzielnie znaleźć w Menedżerze zadań, gdy ma
zniknąć.

## Sposób Moonpool

Moonpool uruchamia polecenie we własnej karcie terminala, więc wyjście zostaje zachowane i dostępny jest
przycisk Zatrzymaj bez własnego okna konsoli. Dla skryptu, który działa, dopóki go nie zatrzymamy,
należy użyć aplikacji `cli`. Opcja `-u` sprawia, że Python od razu opróżnia bufor wyjścia, więc
karta pokazuje je na żywo:

```json title="apps.json"
{
  "id": "worker",
  "name": "Queue worker",
  "group": "Scripts",
  "type": "cli",
  "cwd": "C:\\code\\worker",
  "command": ".venv\\Scripts\\python.exe -u worker.py"
}
```

Po uruchomieniu wystarczy kliknąć nazwę aplikacji, aby obserwować jej wyjście. Aplikacja `cli`
jest uznawana za działającą, dopóki działa jej polecenie, a robi się szara, gdy skrypt się
zakończy, z komunikatem `[proces zakończony]` pozostawionym w karcie. **Zatrzymaj** kończy skrypt
i wszystko, co uruchomił. Użycie `python.exe` ze środowiska wirtualnego przez ścieżkę oznacza, że
nie jest potrzebny krok aktywacji.

Jeśli skrypt udostępnia HTTP (Flask, FastAPI, `python -m http.server`), należy zrobić z niego
aplikację `web`, aby stan działania wynikał z jej portu:

```json title="apps.json"
{
  "id": "docs-api",
  "name": "Docs API",
  "group": "Scripts",
  "type": "web",
  "cwd": "C:\\code\\docs-api",
  "command": ".venv\\Scripts\\python.exe -u app.py",
  "port": 8091,
  "url": "http://127.0.0.1:8091",
  "env": { "PORT": "8091" }
}
```

## Ograniczenia

- Moonpool musi pozostać uruchomiony. Zamknięcie jego okna domyślnie kończy program, a w systemie
  Windows zakończenie programu zatrzymuje każdą uruchomioną przez niego aplikację. Włącz opcję
  **Zamykanie do zasobnika**, aby zamiast tego ukrywać okno; zob.
  [Zasobnik, zamykanie i minimalizowanie](/pl/using/tray-and-closing/).
- Moonpool nie uruchamia ponownie skryptu, który uległ awarii, i sam nie uruchamia go przy
  logowaniu do systemu Windows. Zob. [Automatyczne uruchamianie skryptu lub serwera deweloperskiego przy logowaniu do systemu Windows](/pl/guides/start-app-at-windows-login/).
- Należy unikać zagnieżdżonych cudzysłowów podwójnych w `command`: opakowanie `cmd /c` je psuje.

## Zobacz także

- [Typy aplikacji](/pl/apps/types/#cli): jak śledzone są aplikacje `cli` i `web`.
- [Zatrzymywanie i ponowne uruchamianie](/pl/apps/stop-and-restart/)
- [Przykłady](/pl/apps/examples/)
- [Dzienniki](/pl/data/logs/): gdzie przechowywane jest wyjście sesji.
