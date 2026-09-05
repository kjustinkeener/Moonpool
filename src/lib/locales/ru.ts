// Русский. Ключи, отсутствующие здесь, берутся из английского каталога.
import type { PartialDict } from "../i18n.svelte";

export const ru: PartialDict = {
  "common.close": "Закрыть",
  "common.cancel": "Отмена",
  "common.save": "Сохранить",
  "common.delete": "Удалить",
  "common.edit": "Изменить",
  "common.rename": "Переименовать",
  "common.reload": "Обновить",
  "common.settings": "Настройки",
  "common.about": "О программе",
  "common.tryAgain": "Повторить",
  "common.dismiss": "Скрыть",
  "common.copied": "Скопировано!",
  "common.autoSystem": "Авто (как в системе)",

  "titlebar.minimize": "Свернуть",
  "titlebar.maximize": "Развернуть",
  "titlebar.restore": "Восстановить",

  "sidebar.menu": "Меню",
  "sidebar.addApp": "Добавить приложение",
  "sidebar.editJson": "Изменить apps.json",
  "sidebar.installMoonpool": "Установить Moonpool…",
  "sidebar.filterPlaceholder": "Фильтр приложений...",
  "sidebar.filterLabel": "Фильтр приложений",
  "sidebar.showCli": "Показать панель CLI",
  "sidebar.updateOpenToInstall": "Доступно обновление, откройте для установки",
  "sidebar.updateShowCli": "Доступно обновление, показать панель CLI",
  "sidebar.statusRunning": "работает",
  "sidebar.statusStarting": "запуск...",
  "sidebar.statusStopped": "остановлено",
  "sidebar.working": "Выполняется...",
  "sidebar.restart": "Перезапустить",
  "sidebar.stop": "Остановить",
  "sidebar.launch": "Запустить",
  "sidebar.setIcon": "Выбрать значок...",
  "sidebar.portConflict": "{names} используют один и тот же порт {port}",
  "sidebar.portConflictBadge": "порт {port}: {names}",
  "sidebar.noMatch": "Нет приложений по запросу «{filter}».",

  "app.dragToResize": "Потяните, чтобы изменить размер",
  "app.closeTab": "Закрыть вкладку",
  "app.hideCli": "Скрыть панель CLI",
  "app.pickApp": "Выберите приложение слева, чтобы запустить его.",
  "app.updateAvailable": "Доступна версия Moonpool {version} (у вас {current}).",
  "app.downloading": "Загрузка {version}…",
  "app.installing": "Установка…",
  "app.downloadInstall": "Скачать и установить",
  "app.updateInstalled": "Обновление установлено. Перезапуск…",
  "app.updateFailed": "Не удалось обновить: {error}",
  "app.confirmDelete": "Удалить «{name}»?",
  "app.iconDialogTitle": "Значок для {name}",
  "app.imagesFilter": "Изображения",
  "app.newHere":
    "Впервые здесь? Передайте это ИИ-агенту, чтобы он настроил ваши приложения:",
  "app.copyPrompt": "Скопировать запрос",
  "app.editFile": "Изменить файл",
  "app.orUseHint":
    "Или воспользуйтесь кнопкой {add} выше либо {edit}, чтобы отредактировать {file} напрямую.",

  "about.tagline":
    "Центр запуска в системном трее для локальных приложений и серверов разработки, со встроенным терминалом для каждого приложения.",
  "about.version": "версия {version}",
  "about.checkUpdates": "Проверить обновления",
  "about.checking": "Проверка обновлений...",
  "about.upToDate": "У вас последняя версия.",
  "about.updateDownloading": "Доступно обновление {version}: идёт загрузка...",
  "about.updateInstalled": "Обновление установлено. Перезапуск...",
  "about.checkFailed": "Не удалось проверить обновления: {error}",
  "about.builtWith": "Создано с помощью",
  "about.byLine": "MIT License · автор: {author}",

  "settings.title": "Настройки",
  "settings.theme": "Тема",
  "settings.language": "Язык",
  "settings.languageHint": "Язык, на котором отображается интерфейс Moonpool.",
  "settings.closeToTray": "Закрывать в трей",
  "settings.closeToTrayHint":
    "Закрытие окна сворачивает Moonpool в трей (и убирает с панели задач). Выкл.: закрытие завершает работу.",
  "settings.minimizeToTray": "Сворачивать в трей",
  "settings.minimizeToTrayHint":
    "Сворачивание убирает Moonpool в трей (и с панели задач). Выкл.: сворачивает на панель задач.",
  "settings.alwaysOnTop": "Поверх всех окон",
  "settings.alwaysOnTopHint":
    "Держать Moonpool и его окна настроек и сведений поверх остальных окон.",
  "settings.transparency": "Прозрачность фона",
  "settings.transparencyHint": "Полупрозрачный фон окна. 0 % - непрозрачный.",
  "settings.checkOnStartup": "Проверять обновления при запуске",
  "settings.checkOnStartupHint":
    "При запуске незаметно проверяет на GitHub наличие новой версии и показывает уведомление, если она найдена.",
  "settings.debugLogging": "Записывать отладочные данные в файл",
  "settings.debugLoggingHint":
    "Записывает загрузку манифестов, запуски и ошибки в {file}.",
  "settings.openLog": "Открыть журнал",

  "theme.dark": "Тёмная",
  "theme.light": "Светлая",

  "installer.tagline":
    "Панель запуска в трее для ваших локальных приложений и серверов разработки.",
  "installer.pickFolder": "Выберите папку для портативного Moonpool",
  "installer.poke": "левый клик - подтолкнуть, правый - сбросить",
  "installer.installed": "Установлено, запуск Moonpool…",
  "installer.portableDone": "Портативный режим, запуск Moonpool…",
  "installer.failed": "Ошибка установки: {error}",
  "installer.alreadyInstalledTitle": "Эта копия уже установлена на этом компьютере.",
  "installer.alreadyInstalled": "Уже установлено",
  "installer.installing": "Установка…",
  "installer.install": "Установить Moonpool",
  "installer.desktopShortcut": "Создать ярлык на рабочем столе",
  "installer.portableHint":
    "Запускайте Moonpool из выбранной вами папки (USB-накопитель, архив) и переносите куда угодно. Данные хранятся рядом с exe-файлом.",
  "installer.installPortable": "Портативная установка",
  "installer.installPath": "Путь установки: {dir}",

  "editor.addApp": "Добавить приложение",
  "editor.editApp": "Изменить приложение",
  "editor.backToList": "назад к списку",
  "editor.nameHint": "Название, отображаемое на боковой панели. Обязательное поле.",
  "editor.namePlaceholder": "Моё приложение",
  "editor.groupHint":
    "Заголовок на боковой панели, под которым будет это приложение. Выберите существующую группу или пункт «+ Новая группа», чтобы создать новую.",
  "editor.newGroupPlaceholder": "Название новой группы",
  "editor.pickExistingGroup": "Выберите существующую группу",
  "editor.newGroupOption": "+ Новая группа...",
  "editor.typeHint":
    "Как Moonpool запускает и отслеживает приложение. web = сервер разработки на порту. desktop = обычное приложение, отслеживаемое по имени процесса. static = просто открывает URL. cli = выполняет команду в терминале.",
  "editor.hintWeb":
    "Запускает сервер разработки в терминале; показывает «работает», как только порт отвечает, и открывает браузер.",
  "editor.hintDesktop":
    "Запускает обычное приложение; показывает «работает», как только найден процесс с именем processName.",
  "editor.hintStatic": "Просто открывает url в браузере, без терминала и команды.",
  "editor.hintCli":
    "Выполняет команду и оставляет открытой интерактивную оболочку в каталоге cwd.",
  "editor.portableWarn":
    "Абсолютный путь - не переедет вместе с этой папкой. Укажите {MP_HOME}\\... или путь вида ./, чтобы сохранить переносимость.",
  "editor.notPortable": "не переносимо",
  "editor.cwdHint":
    "Рабочий каталог, в котором выполняется команда, обычно папка проекта приложения. Укажите абсолютный путь либо {MP_HOME}\\... / ./, чтобы сохранить переносимость.",
  "editor.commandHint":
    "Команда, запускающая приложение во встроенном терминале, например 'npm run dev' или 'python app.py'. Для записи только с URL оставьте поле пустым.",
  "editor.portHint":
    "Локальный TCP-порт, который слушает приложение. Moonpool показывает «работает», когда этот порт отвечает, и освобождает его при остановке. Используется веб-приложениями.",
  "editor.processNameHint":
    "Для обычных приложений: имя процесса или исполняемого файла (без .exe), по которому определяется работа и выполняется остановка. В Linux - не более 15 символов.",
  "editor.urlHint":
    "Адрес для открытия: http://localhost:<port> для веб-приложения или file:///path/to/index.html для статической страницы. Укажите file:///{MP_HOME}/..., чтобы сохранить переносимость.",
  "editor.openBrowser": "открывать браузер",
  "editor.openBrowserHint":
    "Автоматически открывать этот адрес в браузере по умолчанию, как только приложение станет доступно.",
  "editor.envLabel": "env (KEY=VALUE в каждой строке)",
  "editor.envHint":
    "Переменные окружения, передаваемые команде, по одной KEY=VALUE в строке (например, PORT=3000).",
  "editor.noteHint":
    "Необязательный текст, который появится во всплывающей подсказке при наведении на это приложение в боковой панели.",
  "editor.notePlaceholder": "необязательная подсказка",
  "editor.phFolder": "путь к папке приложения",
  "editor.phLaunchCommand": "команда запуска приложения",
  "editor.phCommand": "команда для выполнения",
  "editor.discardChanges": "Отменить внесённые изменения?",
  "editor.nameRequired": "укажите название.",

  "term.processExited": "[процесс завершён]",
  "term.copyAll": "Копировать всё",
  "term.copied": "Скопировано",
};
