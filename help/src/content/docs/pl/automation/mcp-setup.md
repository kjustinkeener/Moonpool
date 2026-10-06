---
title: "Połączenie agenta AI z Moonpool przez MCP"
description: "Zarejestruj moonpool.exe mcp jako serwer MCP stdio w swoim hoście, w wersji zainstalowanej lub przenośnej, i poznaj sposób śledzenia przez Moonpool własnego procesu pomocniczego MCP aplikacji."
---

Plik wykonywalny Moonpool jest własnym serwerem MCP. Należy zarejestrować go w hoście jako serwer stdio,
który uruchamia `moonpool.exe` z jednym argumentem `mcp`.

## Rejestracja serwera

W instalacji program to `%USERPROFILE%\.moonpool\moonpool.exe`. W trybie przenośnym to `moonpool.exe` wewnątrz folderu `.moonpool\`. Tę pełną ścieżkę należy użyć jako
`command`. Dla hosta, który odczytuje `.mcp.json`:

```json title=".mcp.json" {5}
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

W pliku JSON ukośniki odwrotne muszą być podwojone, jak powyżej. Host z rejestracją z wiersza poleceń,
taki jak Claude Code, może dodać go jednym krokiem:

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

Serwer przedstawia się jako
`moonpool`, mówi w wersji protokołu MCP `2025-06-18` i udostępnia wyłącznie narzędzia (nie wymienia
żadnych zasobów ani promptów). Narzędzia są widoczne dla agenta jako `moonpool_*`; zob.
[Narzędzia MCP](/pl/automation/mcp-tools/).

## Więcej niż jeden Moonpool

Zainstalowany Moonpool i każda kopia przenośna to osobne programy uruchamiające, każdy z własnymi aplikacjami,
i wszystkie mogą działać jednocześnie. `moonpool.exe mcp` danej kopii zawsze steruje tą kopią. Aby agent mógł
używać kilku, należy zarejestrować każdą pod odrębną nazwą, wskazując plik exe tej kopii:

```json title=".mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    },
    "moonpool-work": {
      "type": "stdio",
      "command": "D:\\Work\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

```powershell frame="terminal"
claude mcp add moonpool-work -- "D:\Work\.moonpool\moonpool.exe" mcp
```

Zarejestrowanie dwóch kopii pod tą samą nazwą powoduje w większości hostów, że jedna zastępuje drugą.
Nazwy narzędzi są takie same dla każdej kopii, więc host odróżnia je po nazwie, pod którą zostały
zarejestrowane. Kopia przenośna przedstawia się też jako `moonpool (<folder>)`, a instrukcje jej serwera
podają nazwę folderu, więc agent widzi, z którą kopią rozmawia.

## Uwagi

- `moonpool.exe mcp` nigdy nie otwiera okna i nigdy nie uruchamia instalatora. Kończy pracę, gdy
  host zamknie jego wejście.
- Używa folderu konfiguracji i kanału sterowania pliku exe, z którego został uruchomiony, więc
  przenośny plik exe odczytuje dane folderu przenośnego i steruje tą kopią przenośną. Plik exe
  jest uznawany za przenośny tylko wtedy, gdy obok niego znajduje się `moonpool.portable`. Każdy inny
  `moonpool.exe`, gdziekolwiek się znajduje, używa folderu zainstalowanego Moonpool
  (`%USERPROFILE%\.moonpool\moonpool-config\`) i steruje zainstalowanym Moonpool.
- Większość narzędzi wymaga działającego Moonpool. Jeśli nie działa, agent może najpierw wywołać
  `moonpool_bootup_launcher`.
- `moonpool_launcher_paths` pokazuje foldery używane przez hub obok tych, które rozwiązuje proces MCP.
  Różnica oznacza, że agent patrzy na inny `apps.json` niż hub.

## Hosty w piaskownicy

Niektóre hosty uruchamiają swoje narzędzia w pakietowej piaskownicy (Store/MSIX), która przekierowuje AppData do
prywatnej kopii dla każdego pakietu. Moonpool wykrywa to, gdy jego folder konfiguracji lub plik exe rozwiązuje się pod
ścieżką w rodzaju `...\Packages\<package>\LocalCache\...`.

Wykrywa to także wtedy, gdy kanał sterowania odpowiada, ale `state.json` nie da się odczytać. Narzędzia,
które odczytują lub zapisują pliki (`moonpool_app_output`, `moonpool_read_config`,
`moonpool_write_config`, `moonpool_restore_config`), zwracają wtedy błąd wskazujący
przyczynę, zamiast pustych lub nieaktualnych danych. Narzędzia korzystające tylko z kanału sterowania, takie jak
`moonpool_list_apps`, nie są blokowane, dopóki kanał jest osiągalny. Jeśli piaskownica ukrywa także kanał,
narzędzia zgłaszają piaskownicę zamiast „Moonpool is not running”.
Zamiast tego należy użyć [wiersza poleceń](/pl/automation/command-line/) z powłoki spoza piaskownicy.

## Aplikacje z własnym serwerem MCP

Wiele aplikacji w Moonpool jest samo osiąganych przez host MCP za pośrednictwem procesu pomocniczego `<exe> mcp`.
Moonpool szuka procesu, którego nazwa pasuje do `processName` aplikacji, a pierwszym argumentem jest
`mcp`, takiego jak `notes-app.exe mcp`. Jeśli serwer działa pod inną nazwą, na przykład jako kopia o zmienionej nazwie, należy ustawić wzorzec `mcpProcessName` aplikacji (zob. [Pola](/pl/apps/fields/#mcpprocessname)); pasujący proces jest uwzględniany bez argumentu `mcp`.

- Gdy taki proces jest dołączony, pasek boczny aplikacji pokazuje podwiersz MCP jako działający, a
  `moonpool_list_apps` dopisuje `[mcp: running]` do wiersza aplikacji. Proces pomocniczy nie jest liczony
  jako działanie samej aplikacji.
- Gdy proces pomocniczy zostanie raz zaobserwowany, Moonpool go zapamiętuje (w `mcp_seen.json` w folderze
  konfiguracji), więc podwiersz MCP pozostaje widoczny jako zatrzymany, a `moonpool_list_apps` pokazuje
  `[mcp: stopped]` po zakończeniu procesu pomocniczego.
- Podwierszem MCP steruje ustawienie `showMcpProcesses`
  ([okno Ustawienia](/pl/using/settings/)).
- `moonpool_stop_mcp_server` kończy proces pomocniczy i nie rusza aplikacji. Nie ma odpowiednika
  uruchamiającego: host, do którego należy proces pomocniczy, uruchamia go ponownie przy następnym wywołaniu narzędzia.

## Gdy narzędzia nie działają

- **Host nie pokazuje narzędzi `moonpool_*`.** Sprawdź, czy `command` jest pełną ścieżką do
  `moonpool.exe`, a `args` to `["mcp"]`, a następnie uruchom host ponownie.
- **Każde narzędzie mówi, że Moonpool nie działa.** Uruchom Moonpool albo wywołaj
  `moonpool_bootup_launcher`. Upewnij się, że zarejestrowany plik exe to kopia, którą uruchomiono.
- **Zmiana się nie pojawia.** Wywołaj `moonpool_launcher_paths` i porównaj foldery huba
  z folderami procesu MCP. Zob. [Hosty w piaskownicy](#hosty-w-piaskownicy).

Więcej w [Rozwiązywaniu problemów](/pl/support/troubleshooting/#błędy-mcp-i-skryptów).

## Zobacz też

- [Udostępnienie agentowi AI (Claude Code, Codex, Cursor) serwera MCP do uruchamiania i zatrzymywania lokalnych aplikacji](/pl/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/)
- [Narzędzia MCP](/pl/automation/mcp-tools/)
- [Agenci AI: szybki start](/pl/automation/quick-start/)
