---
title: "MCP-сервер, позволяющий ИИ-агенту (Claude Code, Codex, Cursor) запускать и останавливать локальные приложения"
description: "Зарегистрируйте Moonpool как MCP-сервер, чтобы Claude Code, Codex или Cursor могли запускать, останавливать, перезапускать серверы разработки и читать их вывод без лишних копий."
---

ИИ-агент для программирования обычно запускает ваш сервер разработки, вводя `npm run dev` в
собственной оболочке. Это может заблокировать агента, оставить осиротевший процесс, занимающий
порт, или запустить вторую копию того, что у вас уже работает. MCP-сервер позволяет агенту
вызывать инструменты, которые запускают и останавливают уже настроенное вами приложение, вместо
того чтобы восстанавливать его командную строку.

## Способ Moonpool

Исполняемый файл Moonpool сам является MCP-сервером: зарегистрируйте `moonpool.exe` с единственным
аргументом `mcp` как stdio-сервер. Когда приложение есть в `apps.json`, агент запускает его по id.

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

Зарегистрируйте сервер. В Claude Code это одна команда (для установленного Moonpool; укажите
полный путь к вашему exe-файлу):

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

Хосты, читающие JSON-файл с MCP-серверами, например `mcp.json` в Cursor, принимают ту же
структуру (обратные косые черты удвоены):

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

Для Codex добавьте в его конфигурацию (`~/.codex/config.toml`) сервер с той же командой и
аргументом `mcp`:

```toml title="config.toml"
[mcp_servers.moonpool]
command = 'C:\Users\you\.moonpool\moonpool.exe'
args = ["mcp"]
```

Точные имена файла и ключей определяет каждый хост, поэтому, если ваша версия отличается,
обратитесь к его документации по MCP. Moonpool нужны только полный путь к `moonpool.exe` и
аргумент `mcp`. После этого перезапустите хост.

## Что может агент

Инструменты называются `moonpool_*`. Те, что нужны для повседневной работы:

| Инструмент | Назначение |
| --- | --- |
| `moonpool_list_apps` | Найти id приложения и узнать, работает ли оно. |
| `moonpool_start_app` | Запустить приложение по id и открыть вкладку его терминала. |
| `moonpool_stop_app` | Остановить его вместе с дочерними процессами. |
| `moonpool_restart_app` | Остановить, дождаться освобождения порта, запустить. Используйте после изменения кода. |
| `moonpool_app_output` | Прочитать вывод приложения; параметр `tail_lines` ограничивает объём. |
| `moonpool_bootup_launcher` | Запустить сам Moonpool, если он не запущен. |

Типичный цикл: `moonpool_restart_app`, затем `moonpool_app_output`. Остальные инструменты
(чтение и запись `apps.json`, снимки экрана) описаны в разделе [Инструменты MCP](/ru/automation/mcp-tools/).

## Если не работает

Если каждый инструмент отвечает `Moonpool is not running - call moonpool_bootup_launcher first`
(Moonpool не запущен, сначала вызовите moonpool_bootup_launcher), значит, Moonpool ещё не
запущен. Если правка не появляется, обычно агент смотрит на другой `apps.json`: вызовите
`moonpool_launcher_paths`. См.
[Если инструменты не работают](/ru/automation/mcp-setup/#если-инструменты-не-работают).

## См. также

- [Настройка MCP](/ru/automation/mcp-setup/)
- [Инструменты MCP](/ru/automation/mcp-tools/)
- [ИИ-агенты: быстрый старт](/ru/automation/quick-start/)
- [Запуск сервера разработки npm в фоне в Windows](/ru/guides/run-npm-dev-server-in-background-windows/)
