---
title: "Установка и использование Moonpool в Linux"
description: "Установите Moonpool в Linux, обойдите особенность трея в GNOME, узнайте, как работают обновления, и чем функции отличаются от версии для Windows."
---

Moonpool работает в Linux через WebKitGTK. Он разрабатывается в основном в Windows, поэтому Linux
поддерживается, но проверен в меньшей степени. В Linux нет карточки установки и выбора
портативного режима, а в меню «...» нет пункта **Установить Moonpool...**.

## Установка

Скачайте пакет со страницы выпусков проекта.

| Пакет | Обновления |
| --- | --- |
| AppImage | Moonpool обновляется сам |
| `.deb` | Ваш менеджер пакетов |
| RPM (устанавливается средством RPM вашего дистрибутива) | Ваш менеджер пакетов |

```bash title="AppImage" frame="terminal"
chmod +x Moonpool_*.AppImage
./Moonpool_*.AppImage
```

```bash title=".deb" frame="terminal"
sudo apt install ./Moonpool_*_amd64.deb
```

```bash title="RPM" frame="terminal"
sudo dnf install ./Moonpool-*.x86_64.rpm
```

Пакет `.deb` подтягивает свои зависимости времени выполнения. Для AppImage должны быть
установлены библиотеки WebKitGTK и AppIndicator, например в Debian или Ubuntu:

```bash frame="terminal"
sudo apt-get install -y libwebkit2gtk-4.1-0 libayatana-appindicator3-1
```

В Fedora или Arch используйте аналогичные пакеты:

```bash title="Fedora" frame="terminal"
sudo dnf install webkit2gtk4.1 libayatana-appindicator-gtk3
```

```bash title="Arch" frame="terminal"
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator
```

## Трей в GNOME

Стандартный GNOME не показывает значки в трее, поэтому значок Moonpool в трее не появится, пока
не будет установлено и включено расширение AppIndicator:

```bash frame="terminal"
sudo apt-get install -y gnome-shell-extension-appindicator
gnome-extensions enable ubuntu-appindicators@ubuntu.com
```

Затем выйдите из сеанса и войдите снова. Главное окно и встроенные терминалы работают и без него.
KDE, Cinnamon, XFCE и MATE показывают трей сразу.

## Обновления

Самостоятельно обновляется только AppImage. Он читает `linux-update.json` из выпусков GitHub,
проверяет подпись minisign и заменяет файл AppImage на месте, поэтому храните его в папке, в
которую можно писать. Установки `.deb` и RPM Moonpool никогда не перезаписывает: проверка
обновлений по-прежнему может сообщить о более новой версии, но её установка из Moonpool
завершается ошибкой с сообщением о необходимости использовать менеджер пакетов. См.
[Обновление](/ru/data/updating/).

## Расположение конфигурации

```text
~/.config/Moonpool/              (or $XDG_CONFIG_HOME/Moonpool/)
~/.config/Moonpool/apps.json
~/.config/Moonpool/dashboards/examples/   (example dashboards)
```

`apps.json` создаётся из примера при первом запуске. См.
[Обзор конфигурации](/ru/apps/apps-json/).

## Отличия от Windows

- Команды запуска выполняются через `$SHELL -c <command>` (`/bin/sh`, если `SHELL` не задан),
  поэтому используйте синтаксис, который понимает ваша оболочка.
- Остановка завершает группу процессов, а затем выполняет дополнительную очистку, выбранную в `killMode`. Освобождение порта при `killMode: "port"` использует `lsof` с запасным вариантом `fuser`; установите `lsof`, если его нет в вашем дистрибутиве. См. [Остановка и перезапуск](/ru/apps/stop-and-restart/).
- `processName` приложения типа `desktop` должно содержать не более 15 символов. Linux обрезает имя процесса до 15 символов, поэтому более длинное имя никогда не определяется как работающее и не может быть остановлено по имени. Приложения `web` определяются по порту и это ограничение их не затрагивает.
- Значки ищутся в `src-tauri/icons/` приложения, в `public/favicon.*`, в `icon.png` или среди его актуальных favicon. Извлечение значка из исполняемого файла доступно только в Windows.
- Кнопки «Открыть» открывают содержащую папку, а не выделяют файл.
- Файлы конфигурации открываются в текстовом редакторе по умолчанию (определяется по ассоциации `text/plain`).
- Установщик Windows, ярлыки и запись в «Установке и удалении программ» не применимы.

## См. также

- [Windows](/ru/platforms/windows/#чем-отличаются-платформы): таблица отличий между платформами.
- [Обновление](/ru/data/updating/#linux)
