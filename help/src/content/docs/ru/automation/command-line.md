---
title: "Управление Moonpool из командной строки"
description: "Управляйте запущенным Moonpool командами moonpool.exe из терминала или скрипта, помечайте команду тикетом и читайте результат из state.json."
---

Повторный запуск `moonpool.exe`, пока этот же Moonpool уже работает, не открывает второе окно. Второй процесс передаёт свои аргументы работающему через его
[канал управления](/ru/automation/control-verbs/) и завершается. Moonpool должен уже работать:
если ничего не запущено, та же команда запускает новый Moonpool, а команда не выполняется.

«Тот же Moonpool» означает ту же папку. Установленный Moonpool и каждая портативная копия работают
независимо, поэтому команда доходит до той копии, чей `moonpool.exe` вы запустили, и никогда до другой.
См. [Портативный режим](/ru/data/portable-mode/#несколько-копий-одновременно).

Используйте путь к нужной копии. Для установленной:

```powershell frame="terminal"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
```

Если работает несколько копий, `Get-Process moonpool` перечисляет их все, поэтому выбирайте по `Path`,
а не берите первую. Он также перечисляет простаивающие вспомогательные процессы `moonpool.exe mcp`, которые
запустили узлы MCP, так что наличие процесса `moonpool` не доказывает, что хаб работает. Вместо этого спросите канал управления
командой `ping` ([Команды управления](/ru/automation/control-verbs/)).

## Команды

Регистр команды не важен. `<id>` - это `id` приложения из `apps.json`.

| Команда | Действие |
| --- | --- |
| `moonpool.exe` | Без команды: выводит окно на передний план. |
| `moonpool.exe show` | Выводит окно на передний план. |
| `moonpool.exe launch <id>` | Запускает приложение и открывает его вкладку терминала. |
| `moonpool.exe stop <id>` | Останавливает приложение. |
| `moonpool.exe restart <id>` | Остановка, ожидание освобождения порта и процесса, запуск. |
| `moonpool.exe reload` | Заново читает `apps.json`. |
| `moonpool.exe refresh-icons` | Заново загружает все значки. |
| `moonpool.exe help` | Открывает окно «Справка». |
| `moonpool.exe quit` | Завершает Moonpool, как пункт меню трея. |
| `moonpool.exe dump <id> [out-path]` | Без `out-path` сообщает путь к журналу приложения за этот сеанс. С ним копирует журнал туда как обычный текст без кодов ANSI. |
| `moonpool.exe paths` | Сообщает папку конфигурации, `apps.json`, `state.json`, журнал, папку dumps, папку значков, признак портативного режима и путь к exe, которые использует работающий Moonpool. |
| `moonpool.exe read-config` | Записывает `dumps\read-config.json` в папке конфигурации с полями `token`, `valid`, `error`, `path` и `manifest_text` (точное содержимое `apps.json`). |
| `moonpool.exe write-config <file> [token]` | Заменяет `apps.json` манифестом из `<file>`, если манифест корректен и, когда указан `token`, `apps.json` всё ещё ему соответствует. |
| `moonpool.exe restore-config [index or filename]` | Без аргумента записывает список снимков в `dumps\restore-config.json`. С аргументом восстанавливает этот снимок, если он корректен. |

```powershell frame="terminal"
& $mp restart my-app
```

```powershell frame="terminal"
& $mp dump my-app C:\temp\my-app.log
```

Неизвестная команда игнорируется. У программы есть и собственные аргументы запуска:
`moonpool.exe mcp` ([Настройка MCP](/ru/automation/mcp-setup/)), `--uninstall` (используется «Установкой и удалением
программ») и `--wait-pid <pid>` (используется, когда Moonpool перезапускает сам себя). Они учитываются
только как первый аргумент, поэтому id приложения вроде `--uninstall` не может их вызвать.

## Чтение результата

Командная строка ничего не выводит, поэтому пометьте команду `--ticket <key>` (любой уникальный ключ, в
любой позиции) и прочитайте результат из `state.json` в папке конфигурации. Это
`%USERPROFILE%\.moonpool\moonpool-config\` для установленной версии, `<your .moonpool folder>\moonpool-config\`
для портативной копии и `~/.config/Moonpool/` в Linux (см.
[Обзор конфигурации](/ru/apps/apps-json/#где-находится-конфигурация)). `show` и `quit`
тикет не записывают.

```powershell frame="terminal"
& $mp launch my-app --ticket t1
```

В `state.json` есть `apps`, `statuses` (для каждого приложения `id`, `running`, `managed`, `mcpRunning`, `mcpSeen`)
и `tickets`. Работающий Moonpool перезаписывает его каждые пару секунд и после каждой
команды и не удаляет при выходе, поэтому оставшийся файл не означает, что Moonpool
работает. Чтобы узнать, работает ли он, или получить актуальный список приложений, используйте команды `ping`
и `list` канала управления ([Команды управления](/ru/automation/control-verbs/)) или инструменты MCP. Опрашивайте
свой тикет, пока `status` не перестанет быть `pending`:

| `status` | Значение |
| --- | --- |
| `pending` | Получено; Moonpool ещё выполняет. |
| `ok` | Готово. Для `dump`, `read-config`, `write-config`, `restore-config` и `paths` в `detail` лежит путь, токен или отчёт. |
| `error` | Сбой; `detail` объясняет причину, например `unknown app id: x`, `did not reach running in time`, `unknown command`. |

Каждый тикет имеет вид `{ ticket, action, arg, status, detail, ts }`, где `ts` в миллисекундах Unix:

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

Завершённые тикеты удаляются через 24 часа, а список сокращается примерно до 50 записей, когда
завершённым тикетам не менее 5 минут.

Агент с поддержкой MCP может обойтись без опроса: см. [Настройка MCP](/ru/automation/mcp-setup/).

## См. также

- [ИИ-агенты: быстрый старт](/ru/automation/quick-start/)
- [Команды управления](/ru/automation/control-verbs/)
