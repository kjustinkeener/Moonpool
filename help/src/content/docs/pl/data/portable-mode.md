---
title: "Uruchamianie Moonpool z pendrive'a lub zsynchronizowanego folderu"
description: "Trzymaj Moonpool i wszystkie jego dane w jednym przenośnym folderze, aby nosić go na pendrivie lub synchronizować, i uruchamiaj kilka kopii obok siebie."
---

Tryb przenośny trzyma Moonpool i wszystko, co zapisuje, w jednym folderze `.moonpool\`, dzięki czemu
można go nosić na pendrivie albo umieścić w zsynchronizowanym folderze i uruchamiać na dowolnym komputerze.

## Jak to działa

Przy instalacji w trybie przenośnym Moonpool tworzy folder `.moonpool\` w wybranej
lokalizacji. Folder ten zawiera program, konfigurację i treści pomocy. Nic
nie jest zapisywane w AppData systemu Windows, więc przeniesienie lub skopiowanie folderu przenosi całą konfigurację
razem z nim.

```text
<chosen location>\.moonpool\
```

## Czym różni się od instalacji

| | Zainstalowany | Przenośny |
| --- | --- | --- |
| Program | `%USERPROFILE%\.moonpool\moonpool.exe` | `<chosen location>\.moonpool\moonpool.exe` |
| Folder konfiguracji | `%USERPROFILE%\.moonpool\moonpool-config\` | `<chosen location>\.moonpool\moonpool-config\` |
| Profil przeglądarki okna, rozmiar i położenie okna | W folderze konfiguracji | W folderze konfiguracji, więc też są przenoszone |
| Menu Start, skrót na pulpicie, wpis w Dodaj/Usuń programy | Tak | Brak |
| Aktualizacje | Zastępują własny plik exe | To samo, wewnątrz folderu `.moonpool\`. Zob. [Aktualizowanie](/pl/data/updating/#kopie-przenośne). |
| Usuwanie | Dodaj/Usuń programy lub `--uninstall` | Usunięcie folderu |

Żaden z trybów nie zapisuje w AppData systemu Windows.

### Zsynchronizowane foldery

Kopię przenośną można trzymać w zsynchronizowanym folderze (OneDrive, Dropbox i podobne), ale należy ją uruchamiać
na jednym komputerze naraz. Moonpool zapisuje `state.json` co kilka sekund i prowadzi dzienniki w trakcie działania aplikacji,
więc dwa komputery uruchamiające ten sam folder walczą o te same pliki, a konflikt synchronizacji może
pozostawić uszkodzony `apps.json`. Przed uruchomieniem na innym komputerze należy zamknąć go na pierwszym.

## Kilka kopii naraz

Dla każdego folderu działa jeden Moonpool. Zainstalowany Moonpool i dowolna liczba kopii przenośnych, każda
w osobnym folderze, mogą działać jednocześnie i każda jest całkowicie odrębna: ma własne aplikacje, ikonę
w zasobniku systemowym, okno, ustawienia, dzienniki i [kanał sterowania](/pl/automation/control-verbs/).

- Podpowiedź ikony w zasobniku i nazwa na pasku zadań wskazują, która to kopia: `Moonpool` dla
  zainstalowanej, `Moonpool (<folder>)` dla przenośnej, gdzie `<folder>` to wybrany folder
  (ten, który zawiera `.moonpool\`).
- Ponowne uruchomienie tej samej kopii przywraca jej okno zamiast otwierać kolejne. Uruchomienie innej
  kopii otwiera tę kopię.
- Aby udostępnić agentowi AI więcej niż jedną kopię, zarejestruj każdą pod własną nazwą; zob.
  [Konfiguracja MCP](/pl/automation/mcp-setup/#więcej-niż-jeden-moonpool).
- Przeniesienie lub zmiana nazwy folderu przenośnego nadaje mu nową tożsamość (nową nazwę kanału sterowania).
  Przed przeniesieniem należy go zamknąć.
- Kopie nie wiedzą o swoich aplikacjach. Dwie kopie, które uruchamiają ten sam serwer
  na tym samym porcie, nadal będą ze sobą kolidować, a Zatrzymaj działające według nazwy procesu lub portu może zakończyć
  coś, co uruchomiła inna kopia; zob.
  [Zatrzymywanie i ponowne uruchamianie](/pl/apps/stop-and-restart/#kilka-kopii-moonpool-lub-własne-procesy).

## Przenoszenie także aplikacji

Użyj tokenu `{MP_HOME}` w ścieżce aplikacji, aby wskazywała wnętrze folderu przenośnego, a nie
stałą lokalizację na jednym komputerze. W kopii przenośnej `{MP_HOME}` to folder, który
zawiera `moonpool.exe`, czyli sam folder `.moonpool\`, a nie wybrany folder:

```json title="apps.json"
{ "cwd": "{MP_HOME}/my-app" }
```

Tutaj `{MP_HOME}/my-app` to `<chosen location>\.moonpool\my-app`. Ścieżka zaczynająca się od
`./` jest zakotwiczona w ten sam sposób. Tokeny i ścieżki `./` działają także w zainstalowanym Moonpool.
Jak rozwiązywane są ścieżki, opisano w [Ścieżki i środowisko](/pl/apps/paths-and-environment/).

## Wybór trybu przenośnego w instalatorze

Tryb przenośny ustawia się w karcie instalatora, która oferuje **Instalacja przenośna**
obok **Zainstaluj Moonpool**.

![Karta instalacji: odnośnik Instalacja przenośna znajduje się pod głównym przyciskiem Zainstaluj Moonpool](../../../../assets/screenshots/installer-window.png)

Wybierz folder, a Moonpool utworzy w nim folder `.moonpool\`, skopiuje się tam i
uruchomi nową kopię ze świeżą konfiguracją.

Karta jest też w menu „...” jako **Zainstaluj Moonpool…**, zarówno w trybie zainstalowanym, jak i
przenośnym. Użycie **Instalacja przenośna** stamtąd powoduje zamknięcie działającego Moonpool i uruchomienie
w jego miejsce nowej kopii przenośnej. Moonpool, z którego rozpoczęto, pozostaje tam, gdzie był,
więc można go później uruchomić ponownie.

Kopia przenośna zaczyna od zera i nie kopiuje dotychczasowych aplikacji. Aby je przenieść,
zamknij kopię przenośną i skopiuj `apps.json` ręcznie:

| | Ścieżka |
| --- | --- |
| Z (zainstalowany) | `%USERPROFILE%\.moonpool\moonpool-config\apps.json` |
| Do (przenośny) | `<chosen location>\.moonpool\moonpool-config\apps.json` |

Wpisy ze ścieżkami bezwzględnymi nadal działają na tym samym komputerze, ale nie są przenoszone. Okno Edytuj aplikację
oznacza je jako „nieprzenośna”.

## Skąd Moonpool wie, że jest przenośny

Kopia jest przenośna, dopóki obok jej `moonpool.exe` znajduje się plik o nazwie `moonpool.portable`.
Nic innego jej nie oznacza i nic nie jest rejestrowane w systemie Windows.

Aby usunąć kopię przenośną, zamknij ją i usuń jej folder `.moonpool\`. `--uninstall` usuwa
tylko zainstalowany Moonpool, nigdy kopię przenośną.
