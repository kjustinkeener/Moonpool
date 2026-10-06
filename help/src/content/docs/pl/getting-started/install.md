---
title: "Instalacja Moonpool w systemie Windows lub Linux"
description: "Zainstaluj Moonpool kilkoma kliknięciami, wybierz tryb zainstalowany lub przenośny, użyj później pozycji menu Zainstaluj Moonpool i czysto odinstaluj program."
---

Ta strona dotyczy systemu Windows. W systemie Windows Moonpool jest własnym instalatorem: pobierany
plik to pojedynczy `moonpool.exe`. Linux nie ma karty instalacji ani wyboru trybu przenośnego; zob.
[Linux](/pl/platforms/linux/).

## Tryb zainstalowany

Należy uruchomić pobrany `moonpool.exe`. Przy pierwszym uruchomieniu pokazuje on kartę instalacji.
Ma trzy elementy: przycisk **Zainstaluj Moonpool**, pole wyboru **Dodaj skrót na pulpicie**
(domyślnie włączone) oraz link **Instalacja przenośna**.

Instalacja kopiuje Moonpool do profilu użytkownika w folderze `.moonpool\`, dodaje skrót w menu
Start (oraz na pulpicie, jeśli pole jest zaznaczone) i rejestruje wpis w sekcji Dodaj/Usuń programy.
Następnie uruchamia zainstalowaną kopię i się zamyka. Pobrany plik zostaje tam, gdzie był; można
go usunąć. Odtąd Moonpool uruchamia się ze skrótu, jak każdą inną aplikację.

![Karta instalacji: przycisk Install Moonpool, pole wyboru skrótu na pulpicie, link Install portable i ścieżka instalacji](../../../../assets/screenshots/installer-window.png)

Wszystko, czego Moonpool potrzebuje, znajduje się w tym jednym folderze: program, Twoja
konfiguracja i dołączona pomoc.

```text title="Installed layout"
%USERPROFILE%\.moonpool\
```

## Zainstaluj Moonpool... z menu

W systemie Windows menu „...” zawiera pozycję **Zainstaluj Moonpool…** w obu trybach. Otwiera
ona tę samą kartę instalacji. Z kopii przenośnej można zainstalować program na stałe. W
zainstalowanej kopii pozycja **Zainstaluj Moonpool** jest wyłączona („Już zainstalowano”), a
**Instalacja przenośna** pozostaje dostępna.

## Odinstalowanie

Należy użyć sekcji Dodaj/Usuń programy w systemie Windows (Zainstalowane aplikacje) albo
uruchomić zainstalowaną kopię z parametrem `--uninstall`. Nie ma jej w PATH, więc trzeba podać
pełną ścieżkę:

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" --uninstall
```

Usuwa to skróty w menu Start i na pulpicie, wpis w rejestrze oraz cały folder
`%USERPROFILE%\.moonpool`, **łącznie z Twoją konfiguracją** (`apps.json`, ustawieniami i
dziennikami). Jeśli konfiguracja ma zostać zachowana, należy najpierw wykonać kopię zapasową tego folderu:

```text
%USERPROFILE%\.moonpool\moonpool-config
```

Każda działająca kopia Moonpool jest zatrzymywana w ramach odinstalowania.

## Tryb przenośny

Wolisz pendrive lub przenośny folder? Kliknij **Instalacja przenośna** na karcie
instalacji i wybierz folder. Zob. [Tryb przenośny](/pl/data/portable-mode/).

## Dalej

- [System Windows ochronił ten komputer](/pl/support/windows-protected-your-pc/): jeśli SmartScreen blokuje instalator.
- [Brak środowiska uruchomieniowego WebView2](/pl/support/webview2-runtime-missing/): jeśli okno pozostaje puste.
- [Pierwsza aplikacja](/pl/getting-started/first-app/)
