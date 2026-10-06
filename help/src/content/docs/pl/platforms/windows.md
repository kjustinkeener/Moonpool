---
title: "Używanie Moonpool w systemie Windows"
description: "Windows jest główną platformą Moonpool: jak go zainstalować oraz tabela różnic między systemami Windows i Linux, aby wiedzieć, czego się spodziewać."
---

Windows jest główną platformą Moonpool. Instalacja przebiega zgodnie z opisem w
[Instalacji](/pl/getting-started/install/).

## Przed uruchomieniem

- **SmartScreen.** Plik `moonpool.exe` nie jest podpisany cyfrowo, więc przy pierwszym uruchomieniu
  system Windows może pokazać „Windows protected your PC” („System Windows ochronił ten komputer”). Należy
  wybrać **More info** („Więcej informacji”), a następnie **Run anyway** („Uruchom mimo to”).
- **Antywirus.** Nowy, niepodpisany plik exe, który kopiuje się i zastępuje przy aktualizacji,
  może wzbudzić alarm programu antywirusowego. Jeśli Twój program blokuje lub poddaje kwarantannie
  `moonpool.exe`, należy zezwolić na niego w folderze `.moonpool`.
- **WebView2.** Okna Moonpool używają Microsoft Edge WebView2, który jest dostarczany z systemem
  Windows 11 i aktualnym Windows 10. Jeśli okno pozostaje puste lub nigdy się nie otwiera, należy
  zainstalować od Microsoft środowisko Evergreen WebView2 Runtime.

Więcej: [System Windows ochronił ten komputer](/pl/support/windows-protected-your-pc/),
[Brak środowiska uruchomieniowego WebView2](/pl/support/webview2-runtime-missing/) i
[Automatyczne uruchamianie skryptu lub serwera deweloperskiego przy logowaniu do systemu Windows](/pl/guides/start-app-at-windows-login/).

## Zasobnik

W systemie Windows 11 nowa ikona w zasobniku często trafia do obszaru ukrytych ikon. Należy
kliknąć strzałkę **^** po prawej stronie paska zadań, aby ją znaleźć, i przeciągnąć ją na pasek
zadań, aby pozostała widoczna.

## Polecenia

- Polecenia są wykonywane przez `cmd /c`. W `command` należy unikać zagnieżdżonych cudzysłowów
  podwójnych; `cmd /c` je psuje. Dla skryptu, który ma zostawić otwartą powłokę, należy użyć
  `pwsh -NoLogo -NoProfile -NoExit -Command <script and args>` bez cudzysłowów wokół części ze
  skryptem.
- `processName` jest dopasowywane z `.exe` lub bez, bez względu na wielkość liter.
- Zatrzymaj kończy całe drzewo procesów uruchomionych przez Moonpool, łącznie z procesami, które
  się od niego odłączyły.
- Aplikacje Docker Desktop wymagają `killMode` o wartości `none` lub `command`, nigdy `port`. Zob.
  [Aplikacje Docker w systemie Windows](/pl/apps/stop-and-restart/#aplikacje-docker-w-systemie-windows).

## Czym różnią się platformy

| | Windows | Linux |
| --- | --- | --- |
| Instalacja | Samoinstalujący się `moonpool.exe` lub wersja przenośna | AppImage, `.deb` lub RPM; brak karty instalacji |
| Samoczynna aktualizacja | Tak, wersja zainstalowana i przenośna | Tylko AppImage |
| Folder konfiguracji | `%USERPROFILE%\.moonpool\moonpool-config\` | `~/.config/Moonpool/` |
| Powłoka dla poleceń | `cmd /c` | `$SHELL -c` |
| `processName` | Dowolna długość, `.exe` opcjonalne, bez względu na wielkość liter | Najwyżej 15 znaków, dokładna wielkość liter |
| Zatrzymanie według `processName` | Kończy proces i jego procesy potomne | Kończy tylko procesy o dokładnie tej nazwie |
| Kanał sterowania | Potok nazwany | Gniazdo Unix |
| Zrzuty ekranu okien (testowanie) | Tak | Nie |
| Ikony z pliku programu | Tak | Nie |
| Zasobnik | Działa od razu | Wymaga AppIndicator; standardowy GNOME wymaga rozszerzenia |

Szczegóły dotyczące systemu Linux znajdują się na stronie [Linux](/pl/platforms/linux/).
