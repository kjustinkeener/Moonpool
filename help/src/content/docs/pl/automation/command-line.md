---
title: "Sterowanie Moonpool z wiersza poleceń"
description: "Steruj działającym Moonpool poleceniami moonpool.exe z terminala lub skryptu, oznaczaj polecenie zgłoszeniem i odczytuj wynik z pliku state.json."
---

Ponowne uruchomienie `moonpool.exe`, gdy ten sam Moonpool już działa, nie otwiera
drugiego okna. Drugi proces przekazuje swoje argumenty działającemu przez jego
[kanał sterowania](/pl/automation/control-verbs/) i kończy pracę. Moonpool musi już działać:
gdy nic nie jest rezydentne, to samo polecenie uruchamia nowy Moonpool, a polecenie sterujące nie jest wykonywane.

„Ten sam Moonpool” oznacza ten sam folder. Zainstalowany Moonpool i każda kopia przenośna
działają osobno, więc polecenie trafia do kopii, której `moonpool.exe` uruchomiono, nigdy do innej.
Zob. [Tryb przenośny](/pl/data/portable-mode/#kilka-kopii-naraz).

Należy użyć ścieżki właściwej kopii. Dla zainstalowanej:

```powershell frame="terminal"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
```

Przy kilku działających kopiach `Get-Process moonpool` wypisuje je wszystkie, więc należy wybierać według `Path`,
a nie brać pierwszej. Wypisuje też bezczynne procesy pomocnicze `moonpool.exe mcp`, które uruchomiły
hosty MCP, więc proces `moonpool` nie dowodzi, że hub działa. Zamiast tego należy zapytać kanał sterowania
poleceniem `ping` ([Polecenia sterujące](/pl/automation/control-verbs/)).

## Polecenia

Wielkość liter w poleceniu nie ma znaczenia. `<id>` to `id` aplikacji z `apps.json`.

| Polecenie | Efekt |
| --- | --- |
| `moonpool.exe` | Bez polecenia: wysuwa okno na wierzch. |
| `moonpool.exe show` | Wysuwa okno na wierzch. |
| `moonpool.exe launch <id>` | Uruchamia aplikację i otwiera jej kartę terminala. |
| `moonpool.exe stop <id>` | Zatrzymuje aplikację. |
| `moonpool.exe restart <id>` | Zatrzymuje, czeka na zwolnienie portu i procesu, uruchamia. |
| `moonpool.exe reload` | Ponownie odczytuje `apps.json`. |
| `moonpool.exe refresh-icons` | Ponownie pobiera wszystkie ikony. |
| `moonpool.exe help` | Otwiera okno Pomocy. |
| `moonpool.exe quit` | Zamyka Moonpool, tak samo jak menu zasobnika systemowego. |
| `moonpool.exe dump <id> [out-path]` | Bez `out-path` zwraca ścieżkę dziennika aplikacji dla tej sesji. Z nim kopiuje tam dziennik jako zwykły tekst z usuniętymi kodami ANSI. |
| `moonpool.exe paths` | Zwraca folder konfiguracji, `apps.json`, `state.json`, dziennik, folder zrzutów, folder ikon, znacznik trybu przenośnego i ścieżkę exe używane przez działający Moonpool. |
| `moonpool.exe read-config` | Zapisuje `dumps\read-config.json` w folderze konfiguracji, zawierający `token`, `valid`, `error`, `path` i `manifest_text` (dokładną zawartość `apps.json`). |
| `moonpool.exe write-config <file> [token]` | Zastępuje `apps.json` manifestem z `<file>`, jeśli manifest jest poprawny i, gdy podano `token`, `apps.json` nadal mu odpowiada. |
| `moonpool.exe restore-config [index or filename]` | Bez argumentu zapisuje listę migawek w `dumps\restore-config.json`. Z argumentem przywraca tę migawkę, jeśli jest poprawna. |

```powershell frame="terminal"
& $mp restart my-app
```

```powershell frame="terminal"
& $mp dump my-app C:\temp\my-app.log
```

Nieznane polecenie jest ignorowane. Program ma także własne argumenty startowe:
`moonpool.exe mcp` ([Konfiguracja MCP](/pl/automation/mcp-setup/)), `--uninstall` (używany przez Dodaj/Usuń
programy) oraz `--wait-pid <pid>` (używany, gdy Moonpool uruchamia się ponownie). Są one honorowane
tylko jako pierwszy argument, więc identyfikator aplikacji, taki jak `--uninstall`, ich nie wywoła.

## Odczyt wyniku

Wiersz poleceń niczego nie wypisuje, więc należy oznaczyć polecenie przez `--ticket <key>` (dowolny unikalny klucz, na
dowolnej pozycji) i odczytać wynik z `state.json` w folderze konfiguracji. To jest
`%USERPROFILE%\.moonpool\moonpool-config\` w instalacji, `<your .moonpool folder>\moonpool-config\`
dla kopii przenośnej i `~/.config/Moonpool/` w systemie Linux (zob.
[Przegląd konfiguracji](/pl/apps/apps-json/#gdzie-znajduje-się-konfiguracja)). `show` i `quit`
nie zapisują zgłoszenia.

```powershell frame="terminal"
& $mp launch my-app --ticket t1
```

`state.json` zawiera `apps`, `statuses` (`id`, `running`, `managed`, `mcpRunning`, `mcpSeen` dla każdej
aplikacji) i `tickets`. Działający Moonpool przepisuje go co kilka sekund i po każdym
poleceniu, a przy zamykaniu go nie usuwa, więc pozostały plik nie oznacza, że Moonpool
działa. Aby sprawdzić, czy działa, lub pobrać bieżącą listę aplikacji, należy użyć poleceń `ping`
i `list` kanału sterowania ([Polecenia sterujące](/pl/automation/control-verbs/)) albo narzędzi MCP. Należy odpytywać
swoje zgłoszenie, dopóki `status` nie przestanie mieć wartości `pending`:

| `status` | Znaczenie |
| --- | --- |
| `pending` | Odebrane; Moonpool nadal je realizuje. |
| `ok` | Gotowe. Dla `dump`, `read-config`, `write-config`, `restore-config` i `paths` pole `detail` zawiera ścieżkę, token lub raport. |
| `error` | Niepowodzenie; `detail` podaje przyczynę, na przykład `unknown app id: x`, `did not reach running in time`, `unknown command`. |

Każde zgłoszenie ma postać `{ ticket, action, arg, status, detail, ts }`, gdzie `ts` jest w milisekundach uniksowych:

```json title="state.json (tickets entry)"
{
  "ticket": "t1",
  "action": "launch",
  "arg": "my-app",
  "status": "error",
  "detail": "did not reach running in time",
  "ts": 1767225600000
}
```

Zakończone zgłoszenia są usuwane po 24 godzinach, a lista jest przycinana w kierunku 50 wpisów, gdy
zakończone zgłoszenia mają co najmniej 5 minut.

Agent obsługujący MCP może pominąć odpytywanie: zob. [Konfiguracja MCP](/pl/automation/mcp-setup/).

## Zobacz też

- [Agenci AI: szybki start](/pl/automation/quick-start/)
- [Polecenia sterujące](/pl/automation/control-verbs/)
