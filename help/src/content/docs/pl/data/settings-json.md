---
title: "Plik settings.json i naprawa uszkodzonego pliku"
description: "Poznaj strukturę pliku settings.json Moonpool, klucze zapisywane automatycznie oraz sposób jego odczytu i naprawy, gdy plik jest uszkodzony."
---

Ustawienia całej aplikacji znajdują się w `settings.json` w folderze konfiguracji (zob.
[Gdzie znajduje się konfiguracja](/pl/apps/apps-json/#gdzie-znajduje-się-konfiguracja)). Zmienia się je w
[oknie Ustawienia](/pl/using/settings/), które wymienia każde ustawienie wraz z kluczem JSON i
wartością domyślną. Dzienniki i ich przechowywanie opisano na stronie [Dzienniki](/pl/data/logs/).

## Struktura

Jeden obiekt JSON. Pominięte klucze przyjmują wartości domyślne:

```json title="settings.json"
{
  "closeToTray": true,
  "transparency": 20,
  "debugLogging": true,
  "cliLogging": true,
  "logRetentionMb": 25
}
```

| Klucz | Wartość domyślna |
| --- | --- |
| `closeToTray` | `false` |
| `minimizeToTray` | `true` |
| `showInTray` | `true` |
| `showInTaskbar` | `true` |
| `alwaysOnTop` | `false` |
| `transparency` | `0` (od 0 do 90) |
| `showStatusbar` | `true` |
| `showMcpProcesses` | `true` |
| `checkOnStartup` | `true` |
| `locale` | `"auto"` |
| `debugLogging` | `false` |
| `cliLogging` | `false` |
| `logRetentionMb` | `10` (minimum 1) |

## Klucze zapisywane automatycznie

Moonpool zapisuje w tym pliku także skalę interfejsu (`uiScale`, od 0,5 do 3,0) oraz ustalony język
(`localeResolved`). Nie trzeba ustawiać żadnego z nich. Motywu tu nie ma: jest
przechowywany w pamięci webview (zob. [Motywy, język i przezroczystość](/pl/using/themes-and-language/)).

## Odczyt i naprawa

Moonpool odczytuje plik przy starcie. Zmiany wprowadzone w trakcie działania nie są uwzględniane; najpierw należy go zamknąć.

Jeśli plik jest nieprawidłowo sformatowany, Moonpool startuje z ustawieniami domyślnymi i odmawia zmiany ustawień.
Błąd kończy się tekstem `Repair settings.json and restart Moonpool before changing settings`.
Popraw plik albo usuń go, aby zresetować wszystkie ustawienia, a następnie uruchom Moonpool ponownie.
