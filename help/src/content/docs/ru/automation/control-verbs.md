---
title: "Канал управления Moonpool и справочник команд"
description: "Как работает канал управления Moonpool (именованный канал или сокет Unix), его протокол и каждая команда, на которую отвечает работающее приложение, с аргументами и ответами."
---

## Где он слушает

У каждой копии Moonpool свой канал, поэтому установленный Moonpool и любые портативные копии
могут работать рядом, не отвечая друг за друга. В Windows установленный Moonpool
слушает именованный канал `\\.\pipe\moonpool`. Портативная копия добавляет идентификатор, полученный из её
папки: `\\.\pipe\moonpool-<id>`.

`<id>` - это 8 шестнадцатеричных цифр, выведенных из пути к папке `moonpool-config` копии, поэтому он остаётся
тем же для этой папки при перезапусках и обновлениях и меняется, если вы перемещаете папку.
`moonpool.exe` копии, включая `moonpool.exe mcp`, всегда находит канал своей копии.

В Linux и macOS вместо этого он слушает сокет домена Unix с режимом `0600`:

| Случай | Путь к сокету |
| --- | --- |
| Обычный | `$XDG_RUNTIME_DIR/moonpool.sock`, если эта переменная задана, иначе `moonpool.sock` в папке конфигурации Moonpool |
| Портативный режим | `moonpool.sock` в папке конфигурации портативной копии, чтобы она никогда не конфликтовала с установленной |
| Путь слишком длинный для сокета (около 100 символов) | `/tmp/moonpool-<uid>/moonpool.sock`, в каталоге, который можете открыть только вы (`moonpool-<id>.sock` для портативной копии) |

Файл сокета, оставшийся после сбоя, обнаруживается и заменяется при следующем запуске. Сокет, на котором
что-то ещё отвечает, никогда не перехватывается. Файл удаляется при обычном выходе из Moonpool.

Через канал [MCP-сервер](/ru/automation/mcp-setup/) также узнаёт, работает ли Moonpool:
если на `ping` приходит ответ, значит работает, а если канала или сокета нет, значит нет. Те же
команды доступны и из [командной строки](/ru/automation/command-line/), кроме
диагностических команд ниже.

## Протокол

Один объект JSON на строку на входе, одна строка JSON на выходе, по порядку. По одному соединению можно передавать много
запросов.

```json title="request"
{"cmd": "restart", "args": ["my-app"]}
```

```jsonl title="replies"
{"ok": true, "result": "..."}
{"ok": false, "error": "unknown app id: ..."}
```

Запрос и ответ из PowerShell:

Для портативной копии используйте имя её канала (`moonpool-<id>`, его показывает команда `paths`) вместо
`moonpool`.

```powershell frame="terminal"
$p = New-Object System.IO.Pipes.NamedPipeClientStream('.', 'moonpool', 'InOut')
$p.Connect(2000)
$w = New-Object System.IO.StreamWriter($p); $w.AutoFlush = $true
$r = New-Object System.IO.StreamReader($p)
$w.WriteLine('{"cmd":"ping"}')
$r.ReadLine()
```

```json title="reply"
{"ok":true,"result":"pong"}
```

- `args` - это список строк, его можно опустить. Остальные поля игнорируются.
- `result` - строка или null. Команды, возвращающие структурированные данные, возвращают их как строку
  JSON.
- На строку, не являющуюся корректным JSON, приходит `{"ok": false, "error": "bad request: ..."}`.
- На неизвестный `cmd` приходит `unknown cmd: <name>`.
- Команда, проходящая через окно (`launch`, `stop`, `restart`, `reload`,
  `refresh-icons`, `help`, `open-window`), получает ответ, когда действие завершено, или ошибку тайм-аута
  через 45 с. Если интерфейс окна-хаба не загружен, она сразу завершается ошибкой `frontend not
  loaded`.
- Moonpool, который запускается, пока предыдущий ещё завершается, пытается занять канал
  примерно 8 секунд. Если не получается, он записывает это в журнал и продолжает работать без канала.

## Команды

| Команда | Аргументы | Результат |
| --- | --- | --- |
| `ping` | нет | `pong`. Только канал. |
| `list` | нет | Строка JSON `{"apps": [...], "statuses": [...]}`, прочитанная из памяти работающего хаба, с теми же `apps` и `statuses`, что и в `state.json`. Добавляет `"statusNotReady": true`, когда приложения зарегистрированы, но первая проверка состояния ещё не выполнялась. Пока `apps.json` не загружается, добавляет `"manifestError": "<message>"` (тогда приложения - это последний загруженный список) и, если со старта ни один список не загружался, `"manifestLoaded": false`. Только канал. |
| `show` | нет | null. Выводит окно на передний план. |
| `quit` | нет | null. Завершает Moonpool. |
| `launch` | `<id>` | null при успехе или `opened` для записи `static` только с `url`. Ошибки: `unknown app id: <id>`, `did not reach running in time`. |
| `stop` | `<id>` | null при успехе или `stopped` для записи `static` только с `url`. Ошибка: `still running after stop`. |
| `restart` | `<id>` | Те же результаты и ошибки, что у `launch`. |
| `reload` | нет | null при успехе. |
| `refresh-icons` | нет | null при успехе. |
| `help` | нет | null. Открывает окно «Справка». |
| `dump` | `<id>` [`out-path`] | Путь к журналу сеанса приложения или к копии в виде обычного текста по адресу `out-path`. |
| `paths` | нет | Многострочный отчёт о папках и exe, которые использует хаб. |
| `read-config` | нет | Путь к `dumps\read-config.json`, где лежат `token`, `valid`, `error`, `path`, `manifest_text`. |
| `write-config` | `<source-file>` [`token`] | Новый токен версии. Ошибки: `stale token: ...`, `rejected invalid manifest: ...`, `cannot read source ...`. |
| `restore-config` | [`index` или `filename`] | Без аргумента: путь к `dumps\restore-config.json` (`count`, `snapshots`). С аргументом: `restored <file> (<n> apps); new version token <token>`. |
| `argv` | аргументы командной строки | null, сразу. Выполняет их точно так же, как второй `moonpool.exe <args>` этой копии, включая `--ticket`. Именно так второй запуск передаёт свои аргументы перед выходом. |

Примеры обмена:

```jsonl title="request, reply"
{"cmd": "launch", "args": ["nope"]}
{"ok": false, "error": "unknown app id: nope"}

{"cmd": "restore-config", "args": ["1"]}
{"ok": true, "result": "restored 1767225600000.json (6 apps); new version token <token>"}
```

`write-config` и `restore-config` сразу загружают новый манифест, записывают снимок в
`apps.json.history\` и обновляют окно.

## Диагностические команды (для тестирования)

Только канал: командная строка их не принимает. Все работают в Windows, Linux и macOS,
кроме `screenshot`, которая работает только в Windows, а в остальных системах отвечает `screenshot is not supported on this
platform (Windows only)`.

| Команда | Аргументы | Результат |
| --- | --- | --- |
| `screenshot` | [`window`] [`max_dim`] | Только Windows. Base64 PNG окна Moonpool (по умолчанию `main`). Необязательный `max_dim` ограничивает большую сторону в пикселях (приводится к диапазону 320-2400, по умолчанию 320; инструмент MCP всегда использует значение по умолчанию). Нецелое `max_dim` - ошибка. Допустимые окна: `main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`. Ошибки: `unknown window '<name>'`, `window '<name>' is not open`. На диск не записывается. |
| `open-window` | `<kind>` [`<id>`] | null. Открывает окно так же, как его пункт меню. `kind`: `settings`, `about`, `installer`, `help`, `themes`, `editor` (необязательный `<id>` открывает диалог «Изменить приложение» этого приложения, без него открывается «Добавить приложение»), `terminal` (`<id>` обязателен: выбирает вкладку терминала этого приложения и расширяет хаб, чтобы показать панель CLI; приложение не запускает), `cli` (только расширяет хаб). Ошибки: `unknown window kind '<kind>'`, `terminal needs an app id`, `unknown app id: <id>`. Отвечает через хаб, как `launch`. |
| `window-state` | [`window`] | Строка JSON: `{"open":false}` либо `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. |
| `stop-mcp` | `<id>` | `stopped`. Завершает вспомогательный процесс приложения `<processName> mcp`, а не само приложение. Ошибки: `missing app id`, `unknown app id: <id>`. |
| `reset-mcp-seen` | [`<id>`] | `<id>: cleared` или `<id>: was not marked seen`; без id - `cleared <n> entries`. Очищает сохранённые наблюдения MCP-помощника. |

Параметр командной строки `--ticket` и записи о результатах в `state.json` относятся к другому каналу;
см. [Командная строка](/ru/automation/command-line/#чтение-результата). Запросы через канал получают
ответ в самом ответе.

## См. также

- [Командная строка](/ru/automation/command-line/)
- [ИИ-агенты: быстрый старт](/ru/automation/quick-start/#одно-и-то-же-действие-тремя-способами)
