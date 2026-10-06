---
title: "Wybór typu aplikacji: web, desktop, static lub cli"
description: "Dowiedz się, jak w Moonpool uruchamiają się aplikacje web, desktop, static i cli, jak wykrywany jest stan „działa” oraz co domyślnie robi przycisk Zatrzymaj."
---

`type` decyduje o tym, które pola mają znaczenie i co domyślnie robi Zatrzymaj.

| | `web` | `desktop` | `static` | `cli` |
| --- | --- | --- | --- | --- |
| Wymaga | `command` | `command` | `url` | `command` |
| Zwykle także | `port`, `url` | `processName` | `command` i `port`, jeśli sama się serwuje | `cwd` |
| Uruchom | Wykonuje `command` w karcie terminala | Wykonuje `command` w karcie terminala | Bez `command`: otwiera `url` w przeglądarce. Z nim: wykonuje go w karcie terminala | Wykonuje `command` w karcie terminala |
| Domyślne `killMode` | `port` | `processName` | `none` | `none` |

![Okno Edytuj aplikację dla aplikacji web: type ustawione na web z jednowierszowym opisem i wypełnionym polem port](../../../../assets/screenshots/edit-app-type-and-port.png)

1. Lista wyboru `type`. Jej wiersz podpowiedzi opisuje, co robi dany typ.
2. Pole `port`. W aplikacji `web` stan „działa” zależy od tego, czy ten port odpowiada.

## Jak ustalany jest stan „działa”

Moonpool sprawdza co kilka sekund. Aplikacja ma stan „działa”, jeśli spełniony jest którykolwiek z warunków, niezależnie od
jej typu:

- `processName` jest ustawione i istnieje proces o tej nazwie. Własne procesy pomocnicze `<exe> mcp`
  Moonpool nie są liczone.
- `port` jest ustawiony i odpowiada na localhost.
- Moonpool ją uruchomił, nie ma ani `port`, ani `processName`, a proces terminala
  nadal żyje.

Zatem aplikacja `cli` ma stan „działa”, dopóki działa jej polecenie, a aplikacja `web` bez `port` zachowuje się
tak samo. Wpis `static` z samym `url` nie ma czego śledzić i nigdy nie pokazuje stanu „działa”.

## web

Lokalny serwer. Ustaw `port`, aby stan „działa” odzwierciedlał, czy serwer odpowiada, oraz `url`
i `openBrowser`, aby otworzyć go po uruchomieniu.

## desktop

Aplikacja natywna. Ustaw `processName` na nazwę pliku wykonywalnego, aby stan „działa” przetrwał
odłączenie okna od polecenia, które je uruchomiło. Domyślne zatrzymanie kończy każdy proces o tej
nazwie.

## static

Strona. Z samym `url` Uruchom i Uruchom ponownie otwierają ją w przeglądarce, a Zatrzymaj nic nie robi.
Otwierane są adresy `http://`, `https://`, `mailto:` i `file://`, więc działa też strona lokalna:

```json title="apps.json"
{ "id": "csv", "name": "CSV dashboard", "group": "Docs", "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html" }
```

Strony, które potrzebują serwera (PHP lub cokolwiek
pobierającego pliki lokalne), wymagają `command`, który go uruchamia, oraz `port`, aby go śledzić. Zob.
[przykłady](/pl/apps/examples/).

## cli

Narzędzie. `command` działa w karcie terminala w `cwd`, a aplikacja przestaje mieć stan „działa”, gdy
polecenie się zakończy. Aby powłoka pozostała otwarta, niech polecenie będzie powłoką, na przykład
takie `command`:

```text title="command"
pwsh -NoLogo -NoProfile -NoExit -Command python run.py --flag
```

Należy unikać zagnieżdżonych cudzysłowów podwójnych w `command`: są one zniekształcane przez opakowanie `cmd /c`.

![Karta terminala aplikacji cli z wynikiem polecenia PowerShell i otwartym znakiem zachęty poniżej](../../../../assets/screenshots/terminal-cli-output.png)

## Co robi kliknięcie

Kliknięcie nazwy aplikacji tylko otwiera jej kartę terminala. Do uruchamiania służą przyciski Uruchom, Zatrzymaj i Uruchom ponownie.
Zob. [Stany aplikacji](/pl/support/glossary/#stany-aplikacji).
