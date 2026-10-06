---
title: "Brak środowiska WebView2: naprawa pustego lub niewidocznego okna Moonpool w systemie Windows"
description: "Jeśli okno Moonpool nie otwiera się lub pozostaje puste w systemie Windows, może brakować Microsoft Edge WebView2 Runtime. Jak to sprawdzić i je zainstalować."
---

Jeśli okno Moonpool w systemie Windows nigdy się nie otwiera albo otwiera się i pozostaje puste,
prawdopodobną przyczyną jest brak środowiska Microsoft Edge WebView2 Runtime. Moonpool jest
aplikacją Tauri, a jego okna to strony WWW rysowane przez WebView2.

WebView2 jest dostarczany z systemem Windows 11 i aktualnym Windows 10, więc większość komputerów
już go ma. Może go brakować w starszym lub okrojonym Windows 10 albo na komputerze, na którym go
usunięto. Kod źródłowy Moonpool nie pokazuje dla tego przypadku osobnego komunikatu, więc nie
cytujemy tu żadnego tekstu błędu: objawem jest okno, które się nie pojawia lub jest puste.

## Sprawdzenie, czy jest zainstalowany

W PowerShell należy sprawdzić w rejestrze wersję środowiska (pierwsza ścieżka to instalacja
ogólnosystemowa, druga to instalacja użytkownika):

```powershell frame="terminal"
Get-ItemProperty "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
Get-ItemProperty "HKCU:\Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
```

Numer wersji, taki jak `120.0.2210.91`, oznacza, że środowisko jest zainstalowane. Błąd dla obu
ścieżek oznacza, że go nie ma.

## Instalacja

Należy pobrać środowisko **Evergreen** WebView2 Runtime ze strony WebView2 firmy Microsoft
(wyszukać „WebView2 Runtime download”), uruchomić instalator, a następnie ponownie uruchomić
Moonpool. Środowisko Evergreen aktualizuje się samo.

## Jeśli jest zainstalowane, a okno nadal jest puste

- Należy zakończyć każdą kopię Moonpool z zasobnika (lub zakończyć `moonpool.exe` w Menedżerze
  zadań) i uruchomić go ponownie.
- Należy włączyć **Zapisuj informacje diagnostyczne do pliku** w [Ustawieniach](/pl/using/settings/),
  jeśli da się do nich dotrzeć, i sprawdzić `moonpool.log`. Zob. [Dzienniki](/pl/data/logs/).
- Jeśli okno otwiera się, ale jest poza ekranem, zob.
  [Problemy z oknem](/pl/support/troubleshooting/#problemy-z-oknem).

## Zobacz także

- [Windows](/pl/platforms/windows/#przed-uruchomieniem)
- [Instalacja](/pl/getting-started/install/)
- [Rozwiązywanie problemów](/pl/support/troubleshooting/)
