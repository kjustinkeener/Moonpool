---
title: "Как устроен settings.json и как починить повреждённый файл"
description: "Структура settings.json в Moonpool, какие ключи Moonpool записывает за вас, и как прочитать файл и починить его, если он повреждён."
---

Общие настройки приложения хранятся в `settings.json` в папке конфигурации (см.
[Где находится конфигурация](/ru/apps/apps-json/#где-находится-конфигурация)). Меняйте их в
[окне «Настройки»](/ru/using/settings/), где перечислен каждый параметр с его ключом JSON и
значением по умолчанию. Журналы и их хранение описаны на странице [Журналы](/ru/data/logs/).

## Структура

Один объект JSON. Пропущенные ключи принимают значения по умолчанию:

```json title="settings.json"
{
  "closeToTray": true,
  "transparency": 20,
  "debugLogging": true,
  "cliLogging": true,
  "logRetentionMb": 25
}
```

| Ключ | По умолчанию |
| --- | --- |
| `closeToTray` | `false` |
| `minimizeToTray` | `true` |
| `showInTray` | `true` |
| `showInTaskbar` | `true` |
| `alwaysOnTop` | `false` |
| `transparency` | `0` (от 0 до 90) |
| `showStatusbar` | `true` |
| `showMcpProcesses` | `true` |
| `checkOnStartup` | `true` |
| `locale` | `"auto"` |
| `debugLogging` | `false` |
| `cliLogging` | `false` |
| `logRetentionMb` | `10` (минимум 1) |

## Ключи, которые записываются за вас

Moonpool также хранит в этом файле масштаб интерфейса (`uiScale`, от 0,5 до 3,0) и определённый
язык (`localeResolved`). Задавать их не нужно. Темы здесь нет: она
хранится в хранилище webview (см. [Темы, язык и прозрачность](/ru/using/themes-and-language/)).

## Чтение и починка

Moonpool читает файл при запуске. Правки, сделанные во время его работы, не подхватываются; сначала закройте его.

Если файл повреждён, Moonpool запускается со значениями по умолчанию и отказывается менять настройки.
Ошибка заканчивается словами `Repair settings.json and restart Moonpool before changing settings`.
Исправьте файл или удалите его, чтобы сбросить все настройки, затем снова запустите Moonpool.
