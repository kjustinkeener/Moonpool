---
title: "Agent AI konfiguruje i obsługuje Moonpool: szybki start"
description: "Trzy sposoby, aby agent AI lub skrypt skonfigurował i obsługiwał Moonpool, który wybrać dla swojego agenta i ta sama czynność pokazana w każdym z nich."
---

Są trzy drogi. Wybór zależy od tego, co potrafi agent.

| Cel | Użyj | Zacznij tutaj |
| --- | --- | --- |
| Agent ma raz znaleźć aplikacje i je dodać | **Kopiuj prompt** na pustym ekranie huba | Poniżej |
| Agent ma uruchamiać, zatrzymywać i odczytywać aplikacje jako wywołania narzędzi | Serwer MCP, `moonpool.exe mcp` | [Konfiguracja MCP](/pl/automation/mcp-setup/) |
| Skrypt lub agent bez MCP | Polecenia wiersza poleceń | [Wiersz poleceń](/pl/automation/command-line/) |

## Kopiuj prompt

Gdy nie jest otwarta żadna karta, panel CLI pokazuje gotowy prompt („Pierwszy raz tutaj? Przekaż to agentowi AI, aby
skonfigurował Twoje aplikacje”). **Kopiuj prompt** umieszcza go w schowku. Wklej go do
agenta. Kieruje on agenta do `AI-README.md` i `apps.json` w folderze konfiguracji i prosi go,
aby znalazł aplikacje i je zarejestrował. Gdy skończy, wybierz **Odśwież**.

Moonpool przepisuje `AI-README.md` obok `apps.json` przy każdym uruchomieniu, więc zawsze odpowiada
używanej wersji. Nie należy w nim przechowywać własnych zmian.

## Ta sama czynność na trzy sposoby

| Czynność | Wiersz poleceń | Polecenie kanału sterowania | Narzędzie MCP |
| --- | --- | --- | --- |
| Uruchomienie aplikacji | `moonpool.exe launch <id>` | `launch` | `moonpool_start_app` |
| Zatrzymanie aplikacji | `moonpool.exe stop <id>` | `stop` | `moonpool_stop_app` |
| Ponowne uruchomienie aplikacji | `moonpool.exe restart <id>` | `restart` | `moonpool_restart_app` |
| Odczyt wyjścia aplikacji | `moonpool.exe dump <id> [out-path]` | `dump` | `moonpool_app_output` |
| Lista aplikacji i stanu | odczyt `state.json` | `list` | `moonpool_list_apps` |
| Ponowny odczyt `apps.json` | `moonpool.exe reload` | `reload` | `moonpool_reload_config` |
| Odczyt `apps.json` | `moonpool.exe read-config` | `read-config` | `moonpool_read_config` |
| Zastąpienie `apps.json` | `moonpool.exe write-config <file> [token]` | `write-config` | `moonpool_write_config` |
| Wycofanie `apps.json` | `moonpool.exe restore-config [n]` | `restore-config` | `moonpool_restore_config` |
| Pokazanie okna | `moonpool.exe show` | `show` | `moonpool_raise_launcher` |
| Uruchomienie Moonpool | `moonpool.exe` | brak | `moonpool_bootup_launcher` |
| Zakończenie Moonpool | `moonpool.exe quit` | `quit` | `moonpool_shutdown_launcher` |
| Pokazanie używanych folderów | `moonpool.exe paths` | `paths` | `moonpool_launcher_paths` |

Wiersz poleceń niczego nie wypisuje; wynik odczytuje się przez `--ticket` (zob.
[Odczyt wyniku](/pl/automation/command-line/#odczyt-wyniku)). Kanał i MCP
odpowiadają bezpośrednio.

## Gdy narzędzia agenta zawodzą

- `Moonpool is not running - call moonpool_bootup_launcher first`: uruchom Moonpool albo pozwól
  agentowi wywołać to narzędzie.
- Zmiana „nie zadziałała”: poproś agenta o `moonpool_launcher_paths`. Jeśli foldery huba i MCP
  się różnią, agent odczytuje inny `apps.json`. Zob.
  [Hosty w piaskownicy](/pl/automation/mcp-setup/#hosty-w-piaskownicy).
- Kilka kopii Moonpool: zarejestruj każdą pod własną nazwą. Zob.
  [Więcej niż jeden Moonpool](/pl/automation/mcp-setup/#więcej-niż-jeden-moonpool).

Przykład dla Claude Code, Codex i Cursor znajduje się w poradniku
[Udostępnienie agentowi AI serwera MCP do uruchamiania i zatrzymywania lokalnych aplikacji](/pl/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/).

Więcej objawów w [Rozwiązywaniu problemów](/pl/support/troubleshooting/#błędy-mcp-i-skryptów).
