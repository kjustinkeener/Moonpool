---
title: "Aktualizacja Moonpool i naprawa nieudanej aktualizacji"
description: "Zobacz, jak Moonpool sprawdza, pobiera i stosuje aktualizacje, co robi baner aktualizacji, jak aktualizują się kopie przenośne i linuksowe oraz co zrobić po błędzie."
---

Moonpool aktualizuje się sam. Nie ma osobnego instalatora do pobrania ani kreatora, przez który
trzeba by przechodzić.

## Jak docierają aktualizacje

Moonpool pobiera `update.json` (`linux-update.json` w systemie Linux) z wydań projektu na GitHubie,
porównuje wersje i oferuje wyłącznie wersję ściśle nowszą. Sprawdza:

- przy starcie, chyba że opcja **Sprawdzaj aktualizacje przy starcie** jest wyłączona w
  [Ustawieniach](/pl/using/settings/);
- za każdym razem, gdy w oknie O programie naciśnięty zostanie przycisk **Sprawdź aktualizacje**. Ten przycisk instaluje nowszą
  wersję od razu i uruchamia Moonpool ponownie. W przeciwnym razie informuje, że używana jest najnowsza
  wersja, albo pokazuje błąd.

Okno O programie pokazuje używaną wersję pod nazwą:

![Górna część okna O programie: logo, nazwa (1) i wiersz wersji pod nią](../../../../assets/screenshots/about-header.png)

1. Nazwa. Wiersz pod nią to wersja i data kompilacji.

Każde pobranie jest weryfikowane kluczem podpisu minisign Moonpool przed zastosowaniem, więc
zmanipulowane lub uszkodzone pobranie jest odrzucane. Moonpool nigdy nie instaluje starszej wersji.

## Baner aktualizacji

Przy starcie znaleziona aktualizacja pojawia się jako baner na pustym ekranie huba:

```text
Moonpool {version} jest dostępny (masz {current}).
```

Baner pokazuje się tylko wtedy, gdy nie jest otwarta żadna karta aplikacji, a panel CLI jest rozwinięty. Gdy panel jest
zwinięty, pulsuje natomiast strzałka obok pola filtru. Przy otwartej karcie nie ma żadnego
sygnału. Aby zobaczyć baner, zamknij wszystkie karty (i rozwiń panel) albo użyj **Sprawdź
aktualizacje** w oknie O programie.

Kliknij **Pobierz i zainstaluj**, a Moonpool zastąpi się i uruchomi ponownie, albo ukryj
baner przyciskiem x.

## Kopie przenośne

Kopia przenośna aktualizuje `moonpool.exe` w swoim własnym folderze `.moonpool\`, w ten sam sposób.
Każda kopia sprawdza i aktualizuje się osobno. Folder musi być zapisywalny, więc kopia na
pendrivie lub udziale tylko do odczytu nie może zaktualizować się sama; należy ręcznie skopiować na nią nowszy `moonpool.exe`.

## Linux

Samoaktualizuje się tylko AppImage. Zastępuje plik AppImage w miejscu, więc należy go trzymać w
folderze, do którego można zapisywać. Instalacje `.deb` lub RPM aktualizuje menedżer pakietów:
instalacja z poziomu Moonpool kończy się komunikatem

```text
automatic updates are available for the AppImage only; update the .deb or RPM with your package manager
```

Zob. [Linux](/pl/platforms/linux/#aktualizacje).

## Gdy aktualizacja się nie powiedzie

Baner pokazuje przyczynę, a przycisk znów staje się dostępny, aby można było spróbować ponownie:

```text
Aktualizacja nie powiodła się: <error>
```

| Błąd zawiera | Prawdopodobna przyczyna | Co zrobić |
| --- | --- | --- |
| `download failed` | Brak połączenia, serwer proxy lub ograniczanie żądań przez GitHub | Poczekać i spróbować ponownie albo zaktualizować ręcznie. |
| `signature verification FAILED - refusing to install` | Pobrany plik jest uszkodzony lub został zmieniony | Spróbować ponownie. Jeśli błąd się powtarza, zaktualizować ręcznie ze strony wydań. |
| `rename self aside` lub `write new exe` | Folder jest tylko do odczytu albo program antywirusowy blokuje plik | Udostępnić folder do zapisu albo zezwolić na `moonpool.exe` w programie antywirusowym, a następnie spróbować ponownie. |
| `refusing to install ... not newer than current` | Oferowana wersja nie jest nowsza | Nie trzeba nic robić. |

### Aktualizacja ręczna

Zamknij Moonpool, pobierz `moonpool.exe` ze
[strony wydań](https://github.com/kjustinkeener/Moonpool/releases) projektu i skopiuj go na
stary plik: `%USERPROFILE%\.moonpool\moonpool.exe` w instalacji albo plik w folderze `.moonpool\`
w przypadku kopii przenośnej. Folder konfiguracji pozostaje nietknięty. W systemie Linux zastąp
AppImage albo użyj menedżera pakietów.

## Pomoc też się aktualizuje

Ta pomoc jest dostarczana wewnątrz Moonpool, więc każda aktualizacja programu przynosi odpowiadającą jej pomoc.
Kopia offline zawsze odpowiada używanej wersji.

## Zobacz też

- [Co nowego](/pl/getting-started/whats-new/)
- [Okno Ustawienia](/pl/using/settings/)
