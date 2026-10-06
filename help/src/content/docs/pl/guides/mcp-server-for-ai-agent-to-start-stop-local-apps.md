---
title: "Serwer MCP dla agenta AI (Claude Code, Codex, Cursor) do uruchamiania i zatrzymywania lokalnych aplikacji"
description: "Zarejestruj Moonpool jako serwer MCP, aby Claude Code, Codex lub Cursor mogły uruchamiać, zatrzymywać, restartować serwery deweloperskie i czytać ich wyjście bez zbędnych kopii."
---

Agent AI do kodowania zwykle uruchamia serwer deweloperski, wpisując `npm run dev` we własnej
powłoce. Może to zablokować agenta, zostawić osierocony proces zajmujący port albo uruchomić drugą
kopię czegoś, co już działa. Serwer MCP pozwala agentowi wywoływać narzędzia, które uruchamiają i
zatrzymują już skonfigurowaną aplikację, zamiast odtwarzać jej wiersz polecenia.

## Sposób Moonpool

Plik wykonywalny Moonpool jest własnym serwerem MCP: należy zarejestrować `moonpool.exe` z jednym
argumentem `mcp` jako serwer stdio. Gdy aplikacja znajduje się w `apps.json`, agent uruchamia ją
według identyfikatora.

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

Zarejestruj serwer. W Claude Code wystarczy jedno polecenie (zainstalowany Moonpool; należy
użyć pełnej ścieżki własnego pliku exe):

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

Hosty, które czytają plik JSON z serwerami MCP, takie jak `mcp.json` w Cursorze, przyjmują ten
sam kształt (ukośniki odwrotne podwojone):

```json title="mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

Dla Codex należy dodać serwer z tym samym poleceniem i argumentem `mcp` w jego konfiguracji
(`~/.codex/config.toml`):

```toml title="config.toml"
[mcp_servers.moonpool]
command = 'C:\Users\you\.moonpool\moonpool.exe'
args = ["mcp"]
```

Dokładne nazwy plików i kluczy zależą od każdego hosta, więc w razie różnic w Twojej wersji
należy sprawdzić jego dokumentację MCP. Moonpool potrzebuje tylko pełnej ścieżki do
`moonpool.exe` i `mcp` jako argumentu. Następnie należy zrestartować hosta.

## Co może zrobić agent

Narzędzia pojawiają się jako `moonpool_*`. Te do codziennej pracy:

| Narzędzie | Zastosowanie |
| --- | --- |
| `moonpool_list_apps` | Znajduje identyfikator aplikacji i pokazuje, czy działa. |
| `moonpool_start_app` | Uruchamia aplikację według identyfikatora i otwiera jej kartę terminala. |
| `moonpool_stop_app` | Zatrzymuje ją wraz z jej procesami potomnymi. |
| `moonpool_restart_app` | Zatrzymuje, czeka na zwolnienie portu, uruchamia. Używać po zmianie kodu. |
| `moonpool_app_output` | Czyta to, co aplikacja wypisała, z `tail_lines` do ograniczenia ilości. |
| `moonpool_bootup_launcher` | Uruchamia samego Moonpool, jeśli nie działa. |

Typowa pętla to `moonpool_restart_app`, a potem `moonpool_app_output`. Pozostałe narzędzia
(odczyt i zapis `apps.json`, zrzuty ekranu) opisano w [Narzędziach MCP](/pl/automation/mcp-tools/).

## Jeśli to nie działa

Jeśli każde narzędzie zgłasza `Moonpool is not running - call moonpool_bootup_launcher first`,
oznacza to, że Moonpool nie został jeszcze uruchomiony. Zmiana, która się nie pojawia, zwykle
oznacza, że agent patrzy na inny `apps.json`: należy wywołać `moonpool_launcher_paths`. Zob.
[Jeśli narzędzia nie działają](/pl/automation/mcp-setup/#gdy-narzędzia-nie-działają).

## Zobacz także

- [Konfiguracja MCP](/pl/automation/mcp-setup/)
- [Narzędzia MCP](/pl/automation/mcp-tools/)
- [Agenci AI: szybki start](/pl/automation/quick-start/)
- [Uruchamianie serwera deweloperskiego npm w tle w systemie Windows](/pl/guides/run-npm-dev-server-in-background-windows/)
