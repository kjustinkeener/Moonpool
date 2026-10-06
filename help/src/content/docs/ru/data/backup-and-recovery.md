---
title: "Резервное копирование Moonpool, откат apps.json и восстановление настройки"
description: "Узнайте, что копировать в резервную копию, как откатить неудачный apps.json, сбросить настройку до примеров, перенести установленную настройку в портативную копию и что удаляет деинсталляция."
---

Всё, что хранит Moonpool, находится в двух местах: в папке конфигурации и в папке панелей.
Пути для каждого режима указаны в разделе [Где находится конфигурация](/ru/apps/apps-json/#где-находится-конфигурация).

## Папка конфигурации

```text
moonpool-config\
  apps.json            your apps                              back up
  apps.json.history\   the last 10 good apps.json files       back up (optional)
  settings.json        app settings                           back up
  icons\               icon overrides, <id>.png and so on     back up
  cli-output\<id>\     session logs                           disposable
  moonpool.log         debug log                              disposable
  state.json           live status snapshot                   disposable
  dumps\               files written by dump and read-config  disposable
  mcp_seen.json        which apps had an MCP helper           disposable
  window-state.json    hub window size and position           disposable
  AI-README.md         rewritten at every launch              disposable
  webview\             the window's browser profile (Windows) disposable
```

Папка панелей - это `{MP_HOME}\dashboards`: `%USERPROFILE%\.moonpool\dashboards`
для установленной версии, `<your .moonpool folder>\dashboards` для портативной и `dashboards/` внутри папки
конфигурации в Linux. Сохраняйте в резервную копию всё своё, что в ней лежит. Её папка `examples` принадлежит Moonpool
и перезаписывается при обновлении.

Тема хранится в браузерном хранилище окна, а не в файле, который можно скопировать. Она не
переносится с резервной копией; выберите её снова после восстановления.

## Резервное копирование

1. Закройте Moonpool, чтобы ни один файл не был записан наполовину.
2. Скопируйте `apps.json`, `settings.json` и `icons\` из папки конфигурации, а также свои
   файлы из `dashboards\`.

```powershell frame="terminal"
$cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
Copy-Item "$cfg\apps.json", "$cfg\settings.json" D:\backup\
Copy-Item "$cfg\icons" D:\backup\ -Recurse
```

Для восстановления закройте Moonpool, скопируйте файлы обратно и запустите его.

## Откат apps.json

Каждое успешное сохранение, запись агентом и восстановление, а также каждое обновление, обнаружившее изменённое содержимое,
копирует проверенный `apps.json` в `apps.json.history\`, сохраняя 10 новейших. Каждый файл
назван по времени создания, например `1767225600000.json`. Файла `apps.json.bak`
нет.

- **Вручную.** Скопируйте снимок поверх `apps.json`, затем выберите **Обновить**.

  ```powershell frame="terminal"
  $cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
  Copy-Item "$cfg\apps.json.history\<snapshot>" "$cfg\apps.json"
  ```

- **Из скрипта.** `moonpool.exe restore-config` перечисляет снимки;
  `moonpool.exe restore-config 1` восстанавливает самый новый. См.
  [Командная строка](/ru/automation/command-line/).
- **От агента.** `moonpool_restore_config`. См. [Инструменты MCP](/ru/automation/mcp-tools/#конфигурация).

Автоматически ничего не восстанавливается.

## Повреждённый файл

- **apps.json.** Moonpool никогда не перезаписывает повреждённый файл. См.
  [Если файл повреждён](/ru/apps/apps-json/#если-файл-повреждён).
- **settings.json.** Исправьте его или удалите, чтобы сбросить все настройки, затем перезапустите Moonpool. См.
  [settings.json](/ru/data/settings-json/#чтение-и-починка).

## Сброс до примеров

Moonpool записывает свои примеры приложений, только когда `apps.json` отсутствует. Чтобы начать заново, закройте
Moonpool (или оставьте его работающим), переименуйте или удалите `apps.json`, затем запустите Moonpool или выберите
**Обновить**. Будет записан новый `apps.json` с примерами.

## Из установленной версии в портативную

Новая портативная копия начинается с примеров приложений. Чтобы перенести свои, см.
[Портативный режим](/ru/data/portable-mode/#выбор-портативного-режима-в-установщике). Скопируйте `icons\` и
`settings.json` так же, если они вам нужны.

## Удаление

Удаление установленного Moonpool стирает всю папку `%USERPROFILE%\.moonpool`,
включая папку конфигурации и панели. Сначала сделайте резервную копию. См.
[Удаление](/ru/getting-started/install/#удаление). Портативная копия удаляется
стиранием её папки `.moonpool\`.
