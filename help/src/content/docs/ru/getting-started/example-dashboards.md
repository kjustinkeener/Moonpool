---
title: "Примеры дашбордов, поставляемых с Moonpool"
description: "Откройте встроенные автономные примеры дашбордов, узнайте, где они хранятся и как на них ссылаются примеры приложений, и добавьте их в существующую конфигурацию."
---

Moonpool поставляется с набором самодостаточных дашбордов внутри программы. Они работают
полностью офлайн, без сервера и без CDN.

| Дашборд | Что это |
| --- | --- |
| CSV explorer | Перетащите файл CSV или TSV: дашборд проанализирует столбцы и отобразит данные. |
| JSON explorer | Перетащите JSON (массивы, вложенные объекты или словари). |
| Excel explorer | Перетащите файл `.xlsx` или `.xls`, он разбирается офлайн. |
| Moonpool Docs | Офлайн-браузер документации в формате Markdown. |

## Где они хранятся

При запуске Moonpool записывает дашборды в `{MP_HOME}\dashboards\examples`:

| Режим | Папка |
| --- | --- |
| Установленный (Windows) | `%USERPROFILE%\.moonpool\dashboards\examples` |
| Портативный | `<your .moonpool folder, the one holding moonpool.exe>\dashboards\examples` |
| Linux | `~/.config/Moonpool/dashboards/examples` (или `$XDG_CONFIG_HOME/Moonpool/dashboards/examples`) |

Папка `examples` принадлежит Moonpool: она заменяется при каждом обновлении Moonpool, поэтому
внесённые там правки теряются. Чтобы изменить дашборд, скопируйте его папку и общую папку `_lib`
на уровень выше, в `dashboards`, и направьте приложение на эту копию. Moonpool никогда не
изменяет ничего другого в `dashboards`.

Версии до 0.3.16 записывали примеры прямо в `dashboards`. Эти копии остаются на месте и больше
не обновляются; приложения, которые на них ссылаются, продолжают работать. Чтобы получить
обновлённые версии, измените их `url` на путь `dashboards/examples/...`, указанный ниже.

## Как на них ссылаются приложения

Каждый из них - приложение типа `static`, у которого `url` - это URL `file:///`, привязанный к `{MP_HOME}`:

```text
file:///{MP_HOME}/dashboards/examples/csv/index.html
```

`{MP_HOME}` разворачивается в папку установки или, в портативном режиме, в папку комплекта, поэтому запись
продолжает работать и после переноса комплекта. URL `file://` разрешены. См.
[Пути и окружение](/ru/apps/paths-and-environment/).

## Примеры приложений появляются только при первом запуске

Записи примеров попадают в `apps.json` только тогда, когда файла конфигурации ещё нет. Если
`apps.json` у вас уже есть, добавьте записи дашбордов сами (**Изменить apps.json** в меню
«...», затем **Обновить**). Добавьте эти четыре записи внутрь массива верхнего уровня, отделив
их запятыми от остальных записей:

```jsonc title="apps.json (excerpt)"
{
  "id": "csv-explorer",
  "name": "Sample CSV Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html",
  "openBrowser": true
},
{
  "id": "json-explorer",
  "name": "Sample JSON Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/json/index.html",
  "openBrowser": true
},
{
  "id": "xlsx-explorer",
  "name": "Sample Excel Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/xlsx/index.html",
  "openBrowser": true
},
{
  "id": "docs-browser",
  "name": "Moonpool Docs",
  "group": "Docs",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/docs/index.html",
  "openBrowser": true
}
```

Значения полей описаны в разделе [Поля приложения](/ru/apps/fields/).

## См. также

- [Примеры](/ru/apps/examples/): более полные записи, которые можно скопировать.
- [Типы приложений](/ru/apps/types/#static)
