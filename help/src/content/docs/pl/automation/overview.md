---
title: "Automatyzacja Moonpool za pomocą skryptów i agentów AI"
description: "Trzy sposoby sterowania działającym Moonpool ze skryptów i agentów AI (MCP, wiersz poleceń i polecenia sterujące), ich wzajemne relacje oraz to, co każdy może zmienić."
---

Moonpool można sterować bez dotykania jego okna. Istnieją trzy powierzchnie, obsługiwane przez
ten sam rezydentny Moonpool (instancję w zasobniku systemowym, nazywaną tu hubem).

Każda kopia Moonpool jest własnym hubem: zainstalowana i każda kopia przenośna działają
niezależnie, każda z własnym kanałem sterowania. Powierzchnia zawsze trafia do kopii, której
`moonpool.exe` używa. Zob. [Tryb przenośny](/pl/data/portable-mode/#kilka-kopii-naraz).

| Powierzchnia | Co to jest | Dokumentacja |
| --- | --- | --- |
| Serwer MCP | `moonpool.exe mcp`, serwer stdio [MCP](https://modelcontextprotocol.io) uruchamiany przez host AI. | [Konfiguracja MCP](/pl/automation/mcp-setup/), [Narzędzia MCP](/pl/automation/mcp-tools/) |
| Wiersz poleceń | `moonpool.exe <verb> [args]`. Drugie uruchomienie tej samej kopii przekazuje polecenie jej hubowi przez kanał sterowania i kończy pracę. | [Wiersz poleceń](/pl/automation/command-line/) |
| Kanał sterowania | Potok nazwany `\\.\pipe\moonpool` (`\\.\pipe\moonpool-<id>` dla kopii przenośnej) w systemie Windows i gniazdo Unix w systemach Linux i macOS, obsługujące jedno żądanie JSON na wiersz. | [Polecenia sterujące](/pl/automation/control-verbs/) |

## Jak są powiązane

- Hub jest właścicielem wszystkiego: uruchamiania aplikacji, dzienników sesji, `apps.json`.
- Serwer MCP jest klientem huba, a nie jego drugą kopią. Większość wywołań narzędzi jest
  przekazywana do huba przez kanał sterowania, a odpowiedź wraca jako wynik narzędzia.
  Wyjątki: `moonpool_bootup_launcher` uruchamia sam `moonpool.exe`;
  `moonpool_app_output` i narzędzia konfiguracji proszą hub o zapis pliku, a następnie go odczytują;
  `moonpool_launcher_paths` dodaje do ścieżek huba własne ścieżki procesu MCP.
- To, czy hub działa, ustala się przez ping tego kanału, a nie przez szukanie
  procesu. Hub, który odpowiada, działa; brak potoku lub gniazda oznacza, że nie działa.
- Każda powierzchnia uruchamia te same procedury obsługi co okno, więc polecenie robi to, co odpowiadające mu kliknięcie.
- Jeśli żaden hub nie działa, narzędzia, które na nim działają, w tym `moonpool_list_apps`, odmawiają z komunikatem
  „Moonpool is not running”. Nie ma nieaktualnej listy. `moonpool_bootup_launcher` go uruchamia.
  Jeśli coś zajmuje kanał, ale nie odpowiada w ciągu kilku sekund, błąd mówi, że proces
  Moonpool może być zawieszony.
- Serwer MCP nie wraca już do sterowania kompilacją huba sprzed kanału sterowania.
  Należy zaktualizować tę kopię albo zamknąć ją i uruchomić ponownie.

## Co może coś zmieniać

| Może zmienić | Powierzchnie |
| --- | --- |
| Uruchomić, zatrzymać lub ponownie uruchomić aplikację | MCP, wiersz poleceń, potok |
| Przepisać `apps.json` | MCP (`moonpool_write_config`, `moonpool_restore_config`), wiersz poleceń, potok |
| Zakończyć Moonpool | MCP (`moonpool_shutdown_launcher`), wiersz poleceń (`quit`), potok |
| Zakończyć proces pomocniczy MCP aplikacji | MCP (`moonpool_stop_mcp_server`), potok (`stop-mcp`) |
| Odświeżyć `apps.json`, ponownie pobrać ikony, pokazać okno | MCP (`moonpool_reload_config`, `moonpool_refresh_app_icons`, `moonpool_raise_launcher`), wiersz poleceń (`reload`, `refresh-icons`, `show`), potok |
| Otworzyć okno lub kartę terminala | potok (`open-window`) |
| Wyczyścić zapamiętane obserwacje procesów pomocniczych MCP | MCP (`moonpool_reset_mcp_seen`), potok (`reset-mcp-seen`) |

Narzędzia tylko do odczytu: `moonpool_list_apps`, `moonpool_app_output`, `moonpool_read_config`,
`moonpool_launcher_paths`, `moonpool_window_state`, `moonpool_screenshot`.

## Właściwości bezpieczeństwa

- **Zapisy konfiguracji są chronione.** Zapis musi zawierać token wersji z ostatniego odczytu,
  nieaktualny token jest odrzucany, a nowy `apps.json` jest walidowany, zanim cokolwiek zostanie zapisane.
  Odrzucony zapis pozostawia `apps.json` nietknięty. Zob. [Narzędzia MCP](/pl/automation/mcp-tools/#konfiguracja).
- **Identyfikatory aplikacji są ograniczone.** Serwer MCP akceptuje tylko litery, cyfry, `.`, `_` i `-`,
  i nigdy początkowego `-`, więc identyfikator nie może zostać odczytany jako flaga wiersza poleceń.
- **Zrzuty ekranu dotyczą tylko Moonpool.** `moonpool_screenshot` przechwytuje jedno z własnych sześciu okien Moonpool
  (`main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`), nigdy ekranu ani
  innej aplikacji. PNG jest budowany w pamięci i zwracany jako wbudowany; Moonpool nie zapisuje go do
  pliku.
- **Brak uwierzytelniania kanału.** Moonpool nie dodaje logowania ani tokenu do potoku sterowania ani
  gniazda. Każdy proces, który może go otworzyć, może wysyłać polecenia. W systemach Linux i macOS plik gniazda jest
  tworzony z trybem `0600`, więc może go otworzyć tylko ten sam użytkownik.
- **Hosty w piaskownicy są wykrywane.** Jeśli serwer MCP stwierdzi, że działa w pakietowej
  piaskownicy (Store/MSIX), w której widziałby prywatną kopię plików Moonpool, narzędzia, które
  odczytują lub zapisują pliki (`moonpool_app_output`, `moonpool_read_config`,
  `moonpool_write_config`, `moonpool_restore_config`), zwracają błąd wyjaśniający przyczynę zamiast
  nieaktualnych danych. Narzędzia korzystające tylko z kanału sterowania nie są blokowane. Zob.
  [Konfiguracja MCP](/pl/automation/mcp-setup/#hosty-w-piaskownicy).

## Platforma

Kanał sterowania istnieje na każdej platformie: potok nazwany w systemie Windows, gniazdo Unix w systemach Linux
i macOS (lokalizacja w sekcji [Polecenia sterujące](/pl/automation/control-verbs/#gdzie-nasłuchuje)). Tylko
`screenshot` (a więc `moonpool_screenshot`) działa wyłącznie w systemie Windows; w systemach Linux i macOS zwraca
„not supported on this platform”. Polecenia wiersza poleceń działają na każdej platformie.

## Zobacz też

- [Agenci AI: szybki start](/pl/automation/quick-start/)
- [Konfiguracja MCP](/pl/automation/mcp-setup/)
