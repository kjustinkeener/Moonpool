---
title: "Сообщения об ошибках Moonpool: already running, requires a command и другие"
description: "Найдите точный текст сообщений об ошибках Moonpool, таких как already running, requires a command, stale token и Update failed, их значение и способ исправления."
---

Вставьте увиденное сообщение в поиск по странице или просмотрите таблицы. Сообщения приведены
в том виде, в каком их показывает Moonpool (на английском). Текст в `<угловых скобках>` заменяется значением
(id приложения, путём или ошибкой системы). Симптомы, не являющиеся сообщением об ошибке,
описаны на странице [Устранение неполадок и FAQ](/ru/support/troubleshooting/).

## Запуск и остановка приложения

| Сообщение | Значение и исправление |
| --- | --- |
| `already running` | У Moonpool уже есть терминал для этого приложения. Сначала остановите его или воспользуйтесь перезапуском. |
| `stopped during launch` | Остановка была нажата, пока запуск ещё выполнялся. Запустите снова. |
| `app has no launch command` | В записи нет `command`. Добавьте её в редакторе приложений или в `apps.json`. Без неё может обойтись только запись `static` с `url`. |
| `unknown app: <id>` | Приложение с таким `id` не загружено. Проверьте id, затем нажмите «Обновить», если вы правили `apps.json` вручную. |
| `unknown app id: <id>` | Та же проблема, о которой сообщается скрипту или агенту. Выведите список приложений через `moonpool_list_apps`. |
| `did not reach running in time` | Из скрипта или агента: приложение не перешло в состояние «работает» за 25 секунд. Проверьте `port` или `processName` и прочитайте вывод. См. [Неверный индикатор состояния](/ru/support/troubleshooting/#неверный-индикатор-состояния). |
| `still running after stop` | Через 15 секунд приложение всё ещё считается работающим. Задайте `killMode`. См. [Остановка и перезапуск](/ru/apps/stop-and-restart/). |
| `refusing to open non-web url: <url>` | `url` не начинается с `http://`, `https://`, `mailto:` или `file://`. Исправьте `url`. |
| `[process exited]` | Не ошибка: команда приложения завершилась. Отображается во вкладке терминала. |

## Проверка apps.json

Moonpool отвергает `apps.json`, нарушающий правило, и сохраняет последний успешно загруженный
список. `<n>` - это позиция записи в файле, начиная с 1.

| Сообщение | Исправление |
| --- | --- |
| `apps.json entry <n> has invalid id "<id>"; use letters, digits, '.', '_', and '-' without a leading '-'` | Переименуйте `id`. |
| `duplicate app id "<id>"` | У двух записей одинаковый `id`. Сделайте каждый уникальным. |
| `apps.json entry <n> (<id>) has an empty name` | Заполните `name`. |
| `apps.json entry <n> (<id>) has an empty group` | Заполните `group`. |
| `apps.json entry <n> (<id>) has unknown type "<type>"` | `type` должен быть `web`, `desktop`, `static` или `cli`. |
| `apps.json entry <n> (<id>) has invalid port 0` | `port` должен быть от 1 до 65535. |
| `apps.json entry <n> (<id>) requires a url` | Записи `static` нужен `url`. |
| `apps.json entry <n> (<id>) requires a command` | Всем остальным типам нужен `command`. |

В редакторе приложений при сохранении без названия показывается `name is required.`
(на русском интерфейсе: «укажите название.»).
Текст баннера, «В apps.json ошибка, показан последний загруженный список.» или «В apps.json
ошибка, поэтому приложения не загружены.», и способ восстановления описаны в разделе
[В apps.json ошибка](/ru/support/troubleshooting/#в-appsjson-ошибка). Если баннер сообщает,
что сохранение приостановлено, сообщение заканчивается словами `Repair apps.json and reload it before saving from
Moonpool`. Полный список правил приведён в разделе [Проверка](/ru/apps/apps-json/#проверка).

## Настройки, обновления и установщик

| Сообщение | Значение и исправление |
| --- | --- |
| `... Repair settings.json and restart Moonpool before changing settings` | Файл `settings.json` повреждён. Исправьте или удалите его и перезапустите. См. [settings.json](/ru/data/settings-json/#чтение-и-починка). |
| `Update failed: <error>` | Не удалось скачать или установить обновление. См. [Если обновление не удалось](/ru/data/updating/#когда-обновление-не-удалось). |
| `Update check failed: <error>` | Не удалась проверка обновлений в окне «О программе». Текст после двоеточия объясняет причину. Повторите попытку позже. |
| `Install failed: <error>` | Установщик остановился на шаге, названном после двоеточия, например `copy exe: ...`. Завершите любой Moonpool, запущенный из `%USERPROFILE%\.moonpool`, и повторите. |
| `target folder does not exist` | Папка, выбранная для портативной копии, исчезла. Выберите существующую. |

## MCP и скрипты

| Сообщение | Значение и исправление |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Запустите Moonpool или позвольте агенту вызвать этот инструмент. Для портативной копии в сообщении указана эта копия. |
| `frontend not loaded` | Главное окно ещё не завершило загрузку. Подождите и повторите. |
| `invalid app_id: use only letters, digits, '.', '_', '-' (no leading '-')` | Агент передал id, который MCP-сервер не принимает. Используйте id из `moonpool_list_apps`. |
| `stale token: apps.json changed since it was read ...` | Прочитайте `apps.json` заново, примените правку снова, затем запишите. |
| `rejected invalid manifest: ...` | Новый `apps.json` не прошёл проверку (см. выше). Файл не был изменён. |
| `no console output recorded for '<id>' (not launched this session)` | Вывод `moonpool_app_output` запрошен для приложения, которое не запускалось с момента старта Moonpool. |

Подробнее в разделах [Инструменты MCP](/ru/automation/mcp-tools/) и
[Настройка MCP](/ru/automation/mcp-setup/#если-инструменты-не-работают).

## Ошибки других программ

- [`Error: listen EADDRINUSE: address already in use :::3000` и `Port 5173 is in use`](/ru/support/port-already-in-use/)
- [`Windows protected your PC`](/ru/support/windows-protected-your-pc/)
- [Отсутствует среда WebView2](/ru/support/webview2-runtime-missing/)
