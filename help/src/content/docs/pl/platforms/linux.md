---
title: "Instalacja i używanie Moonpool w systemie Linux"
description: "Zainstaluj Moonpool w systemie Linux, obejdź problem z zasobnikiem w GNOME, dowiedz się, jak działają aktualizacje, i sprawdź, czym funkcje różnią się od wersji dla Windows."
---

Moonpool działa w systemie Linux dzięki WebKitGTK. Jest rozwijany głównie w systemie Windows, więc
Linux jest wspierany, ale mniej przetestowany w praktyce. W systemie Linux nie ma karty instalacji
ani wyboru trybu przenośnego, a menu „...” nie zawiera pozycji **Zainstaluj Moonpool...**.

## Instalacja

Pakiet należy pobrać ze strony Releases projektu.

| Pakiet | Aktualizacje |
| --- | --- |
| AppImage | Moonpool aktualizuje się samodzielnie |
| `.deb` | Menedżer pakietów |
| RPM (instalacja narzędziem RPM Twojej dystrybucji) | Menedżer pakietów |

```bash title="AppImage" frame="terminal"
chmod +x Moonpool_*.AppImage
./Moonpool_*.AppImage
```

```bash title=".deb" frame="terminal"
sudo apt install ./Moonpool_*_amd64.deb
```

```bash title="RPM" frame="terminal"
sudo dnf install ./Moonpool-*.x86_64.rpm
```

Pakiet `.deb` sam doinstaluje zależności uruchomieniowe. AppImage wymaga obecności bibliotek
WebKitGTK i AppIndicator, na przykład w Debianie lub Ubuntu:

```bash frame="terminal"
sudo apt-get install -y libwebkit2gtk-4.1-0 libayatana-appindicator3-1
```

W Fedorze lub Arch należy użyć odpowiedników:

```bash title="Fedora" frame="terminal"
sudo dnf install webkit2gtk4.1 libayatana-appindicator-gtk3
```

```bash title="Arch" frame="terminal"
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator
```

## Zasobnik w GNOME

Standardowy GNOME nie pokazuje ikon w zasobniku, więc ikona Moonpool w zasobniku nie pojawi się,
dopóki rozszerzenie AppIndicator nie zostanie zainstalowane i włączone:

```bash frame="terminal"
sudo apt-get install -y gnome-shell-extension-appindicator
gnome-extensions enable ubuntu-appindicators@ubuntu.com
```

Następnie należy wylogować się i zalogować ponownie. Okno huba i wbudowane terminale działają bez
tego. KDE, Cinnamon, XFCE i MATE pokazują zasobnik od razu.

## Aktualizacje

Samodzielnie aktualizuje się tylko AppImage. Odczytuje `linux-update.json` z GitHub Releases,
weryfikuje podpis minisign i zastępuje plik AppImage w miejscu, więc należy go trzymać w folderze,
do którego można zapisywać. Instalacje `.deb` i RPM nigdy nie są nadpisywane przez Moonpool:
sprawdzanie aktualizacji może nadal zgłosić nowszą wersję, ale jej zainstalowanie z poziomu
Moonpool kończy się niepowodzeniem z komunikatem, aby użyć menedżera pakietów. Zob.
[Aktualizacje](/pl/data/updating/).

## Lokalizacja konfiguracji

```text
~/.config/Moonpool/              (or $XDG_CONFIG_HOME/Moonpool/)
~/.config/Moonpool/apps.json
~/.config/Moonpool/dashboards/examples/   (example dashboards)
```

Plik `apps.json` jest tworzony z przykładu przy pierwszym uruchomieniu. Zob.
[Przegląd konfiguracji](/pl/apps/apps-json/).

## Różnice w porównaniu z systemem Windows

- Polecenia uruchamiające są wykonywane przez `$SHELL -c <command>` (`/bin/sh`, jeśli `SHELL` nie jest ustawione), więc należy używać składni zrozumiałej dla powłoki.
- Zatrzymaj kończy grupę procesów, a następnie wykonuje dodatkowe sprzątanie wybrane przez `killMode`. Zwalnianie portu przy `killMode: "port"` używa `lsof`, a w razie jego braku `fuser`; jeśli dystrybucja nie zawiera `lsof`, należy go zainstalować. Zob. [Zatrzymywanie i ponowne uruchamianie](/pl/apps/stop-and-restart/).
- `processName` aplikacji `desktop` może mieć najwyżej 15 znaków. Linux obcina nazwę procesu do 15 znaków, więc dłuższa nazwa nigdy nie zostanie wykryta jako działająca i nie można jej zatrzymać według nazwy. Aplikacje `web` są dopasowywane według portu i nie dotyczy ich to.
- Ikony są wyszukiwane w `src-tauri/icons/` aplikacji, `public/favicon.*`, `icon.png` lub jej bieżącym favicon. Wyodrębnianie ikony z pliku binarnego jest możliwe tylko w systemie Windows.
- Przyciski otwierania folderu otwierają folder nadrzędny zamiast zaznaczać plik.
- Pliki konfiguracji otwierają się w domyślnym edytorze tekstu (ustalanym ze skojarzenia `text/plain`).
- Instalator systemu Windows, skróty i wpis Dodaj/Usuń programy nie mają zastosowania.

## Zobacz także

- [Windows](/pl/platforms/windows/#czym-różnią-się-platformy): tabela różnic między platformami.
- [Aktualizacje](/pl/data/updating/#linux)
