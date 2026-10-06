---
title: "Пусть ИИ-агент настроит Moonpool и управляет им: быстрый старт"
description: "Три способа, которыми ИИ-агент или скрипт могут настроить Moonpool и управлять им, какой выбрать для вашего агента и одно и то же действие в каждом из них."
---

Есть три входа. Выбирайте по тому, что умеет ваш агент.

| Вам нужно | Используйте | С чего начать |
| --- | --- | --- |
| Чтобы агент один раз нашёл ваши приложения и добавил их | **Скопировать запрос** на пустом экране хаба | Ниже |
| Чтобы агент запускал, останавливал и читал приложения вызовами инструментов | MCP-сервер, `moonpool.exe mcp` | [Настройка MCP](/ru/automation/mcp-setup/) |
| Скрипт или агент без MCP | Команды командной строки | [Командная строка](/ru/automation/command-line/) |

## Скопировать запрос

Когда ни одна вкладка не открыта, панель CLI показывает готовый запрос («Впервые здесь? Передайте это ИИ-агенту, чтобы он настроил ваши приложения»).
**Скопировать запрос** помещает его в буфер обмена. Вставьте его в вашего
агента. Он указывает агенту на `AI-README.md` и `apps.json` в вашей папке конфигурации и просит
найти ваши приложения и зарегистрировать их. Когда он закончит, выберите **Обновить**.

Moonpool перезаписывает `AI-README.md` рядом с `apps.json` при каждом запуске, поэтому он всегда соответствует
запущенной вами версии. Не храните в нём собственные правки.

## Одно и то же действие тремя способами

| Действие | Командная строка | Команда канала управления | Инструмент MCP |
| --- | --- | --- | --- |
| Запустить приложение | `moonpool.exe launch <id>` | `launch` | `moonpool_start_app` |
| Остановить приложение | `moonpool.exe stop <id>` | `stop` | `moonpool_stop_app` |
| Перезапустить приложение | `moonpool.exe restart <id>` | `restart` | `moonpool_restart_app` |
| Прочитать вывод приложения | `moonpool.exe dump <id> [out-path]` | `dump` | `moonpool_app_output` |
| Перечислить приложения и состояние | прочитать `state.json` | `list` | `moonpool_list_apps` |
| Заново прочитать `apps.json` | `moonpool.exe reload` | `reload` | `moonpool_reload_config` |
| Прочитать `apps.json` | `moonpool.exe read-config` | `read-config` | `moonpool_read_config` |
| Заменить `apps.json` | `moonpool.exe write-config <file> [token]` | `write-config` | `moonpool_write_config` |
| Откатить `apps.json` | `moonpool.exe restore-config [n]` | `restore-config` | `moonpool_restore_config` |
| Показать окно | `moonpool.exe show` | `show` | `moonpool_raise_launcher` |
| Запустить Moonpool | `moonpool.exe` | нет | `moonpool_bootup_launcher` |
| Выйти из Moonpool | `moonpool.exe quit` | `quit` | `moonpool_shutdown_launcher` |
| Показать используемые папки | `moonpool.exe paths` | `paths` | `moonpool_launcher_paths` |

Командная строка ничего не выводит; результат читайте через `--ticket` (см.
[Чтение результата](/ru/automation/command-line/#чтение-результата)). Канал и MCP
отвечают напрямую.

## Когда инструменты агента не работают

- `Moonpool is not running - call moonpool_bootup_launcher first`: запустите Moonpool или пусть
  агент вызовет этот инструмент.
- Правка «не вступила в силу»: попросите агента вызвать `moonpool_launcher_paths`. Если папки хаба и MCP
  различаются, агент читает другой `apps.json`. См.
  [Хосты в песочнице](/ru/automation/mcp-setup/#хосты-в-песочнице).
- Несколько копий Moonpool: зарегистрируйте каждую под своим именем. См.
  [Несколько копий Moonpool](/ru/automation/mcp-setup/#несколько-копий-moonpool).

Разобранный пример для Claude Code, Codex и Cursor приведён в
[Дайте ИИ-агенту MCP-сервер для запуска и остановки локальных приложений](/ru/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/).

Больше симптомов в разделе [Устранение неполадок](/ru/support/troubleshooting/#ошибки-mcp-и-скриптов).
