// Español. Keys absent here fall back to English at runtime.
import type { PartialDict } from "../i18n.svelte";

export const es: PartialDict = {
  "common.close": "Cerrar",
  "common.cancel": "Cancelar",
  "common.save": "Guardar",
  "common.delete": "Eliminar",
  "common.edit": "Editar",
  "common.rename": "Cambiar nombre",
  "common.reload": "Recargar",
  "common.settings": "Ajustes",
  "common.about": "Acerca de",
  "common.tryAgain": "Reintentar",
  "common.dismiss": "Descartar",
  "common.copied": "¡Copiado!",
  "common.autoSystem": "Automático (sistema)",

  "titlebar.minimize": "Minimizar",
  "titlebar.maximize": "Maximizar",
  "titlebar.restore": "Restaurar",

  "sidebar.menu": "Menú",
  "sidebar.addApp": "Añadir app",
  "sidebar.editJson": "Editar apps.json",
  "sidebar.installMoonpool": "Instalar Moonpool…",
  "sidebar.filterPlaceholder": "Filtrar apps...",
  "sidebar.filterLabel": "Filtrar apps",
  "sidebar.showCli": "Mostrar el panel CLI",
  "sidebar.updateOpenToInstall": "Hay una actualización, ábrela para instalar",
  "sidebar.updateShowCli": "Hay una actualización, mostrar el panel CLI",
  "sidebar.statusRunning": "en ejecución",
  "sidebar.statusStarting": "iniciando...",
  "sidebar.statusStopped": "detenida",
  "sidebar.working": "Trabajando...",
  "sidebar.restart": "Reiniciar",
  "sidebar.stop": "Detener",
  "sidebar.launch": "Iniciar",
  "sidebar.setIcon": "Elegir icono...",
  "sidebar.portConflict": "{names} usan el mismo puerto {port}",
  "sidebar.portConflictBadge": "puerto {port}: {names}",
  "sidebar.noMatch": "Ninguna app coincide con «{filter}».",

  "app.dragToResize": "Arrastra para redimensionar",
  "app.closeTab": "Cerrar pestaña",
  "app.hideCli": "Ocultar el panel CLI",
  "app.pickApp": "Elige una app a la izquierda para iniciarla.",
  "app.updateAvailable": "Moonpool {version} ya está disponible (tienes la {current}).",
  "app.downloading": "Descargando {version}…",
  "app.installing": "Instalando…",
  "app.downloadInstall": "Descargar e instalar",
  "app.updateInstalled": "Actualización instalada. Reiniciando…",
  "app.updateFailed": "Error al actualizar: {error}",
  "app.confirmDelete": "¿Eliminar «{name}»?",
  "app.iconDialogTitle": "Icono de {name}",
  "app.imagesFilter": "Imágenes",
  "app.newHere":
    "¿Es tu primera vez? Pásale esto a un agente de IA para que configure tus apps:",
  "app.copyPrompt": "Copiar el prompt",
  "app.editFile": "Editar el archivo",
  "app.orUseHint": "O usa {add} arriba, o {edit} para editar {file} directamente.",

  "about.tagline":
    "Un centro de lanzamiento en la bandeja del sistema para apps locales y servidores de desarrollo, con una terminal integrada por app.",
  "about.version": "versión {version}",
  "about.checkUpdates": "Buscar actualizaciones",
  "about.checking": "Buscando actualizaciones...",
  "about.upToDate": "Tienes la última versión.",
  "about.updateDownloading": "Actualización {version} disponible: descargando...",
  "about.updateInstalled": "Actualización instalada. Reiniciando...",
  "about.checkFailed": "Error al buscar actualizaciones: {error}",
  "about.builtWith": "Creado con",
  "about.byLine": "Licencia MIT · por {author}",

  "settings.title": "Ajustes",
  "settings.theme": "Tema",
  "settings.language": "Idioma",
  "settings.languageHint": "El idioma en el que se muestra el texto de Moonpool.",
  "settings.closeToTray": "Cerrar a la bandeja",
  "settings.closeToTrayHint":
    "Al cerrar la ventana, Moonpool se oculta en la bandeja (sale de la barra de tareas). Desactivado: cerrar sale de la app.",
  "settings.minimizeToTray": "Minimizar a la bandeja",
  "settings.minimizeToTrayHint":
    "Al minimizar, Moonpool se oculta en la bandeja (sale de la barra de tareas). Desactivado: se minimiza a la barra de tareas.",
  "settings.alwaysOnTop": "Siempre visible",
  "settings.alwaysOnTopHint":
    "Mantiene Moonpool y sus ventanas de Ajustes y Acerca de por encima de las demás.",
  "settings.transparency": "Transparencia del fondo",
  "settings.transparencyHint": "Fondo de ventana translúcido. 0 % es opaco.",
  "settings.checkOnStartup": "Buscar actualizaciones al iniciar",
  "settings.checkOnStartupHint":
    "Al arrancar, consulta GitHub en segundo plano y muestra un aviso si hay una versión más reciente.",
  "settings.debugLogging": "Registrar información de depuración en un archivo",
  "settings.debugLoggingHint":
    "Registra las cargas del manifiesto, los inicios y los errores en {file}.",
  "settings.openLog": "Abrir el registro",

  "theme.dark": "Oscuro",
  "theme.light": "Claro",

  "installer.tagline": "Un lanzador en la bandeja para tus apps locales y servidores de desarrollo.",
  "installer.pickFolder": "Elige una carpeta para Moonpool portable",
  "installer.poke": "clic izquierdo para tocar, clic derecho para reiniciar",
  "installer.installed": "Instalado, iniciando Moonpool…",
  "installer.portableDone": "Modo portable, iniciando Moonpool…",
  "installer.failed": "Error de instalación: {error}",
  "installer.alreadyInstalledTitle": "Esta copia ya está instalada en este PC.",
  "installer.alreadyInstalled": "Ya está instalado",
  "installer.installing": "Instalando…",
  "installer.install": "Instalar Moonpool",
  "installer.desktopShortcut": "Añadir un acceso directo en el escritorio",
  "installer.portableHint":
    "Ejecuta Moonpool desde la carpeta que elijas (memoria USB, zip) y muévela a donde quieras. Los datos se quedan junto al exe.",
  "installer.installPortable": "Instalar portable",
  "installer.installPath": "Ruta de instalación: {dir}",

  "editor.addApp": "Añadir app",
  "editor.editApp": "Editar app",
  "editor.backToList": "volver a la lista",
  "editor.nameHint": "El nombre que se muestra en la barra lateral. Obligatorio.",
  "editor.namePlaceholder": "Mi app",
  "editor.groupHint":
    "El encabezado de la barra lateral bajo el que aparece esta app. Elige un grupo existente o «+ Nuevo grupo» para crear uno.",
  "editor.newGroupPlaceholder": "Nombre del nuevo grupo",
  "editor.pickExistingGroup": "Elige un grupo existente",
  "editor.newGroupOption": "+ Nuevo grupo...",
  "editor.typeHint":
    "Cómo ejecuta y controla Moonpool la app. web = servidor de desarrollo en un puerto. desktop = app nativa identificada por el nombre del proceso. static = solo abre una URL. cli = ejecuta un comando en una terminal.",
  "editor.hintWeb":
    "Ejecuta un servidor de desarrollo en una terminal; la marca «en ejecución» cuando su puerto responde y abre el navegador en cuanto está activo.",
  "editor.hintDesktop":
    "Inicia una app nativa; la marca «en ejecución» cuando encuentra un proceso llamado processName.",
  "editor.hintStatic": "Solo abre url en el navegador, sin terminal ni comando.",
  "editor.hintCli": "Ejecuta un comando y mantiene abierta una shell interactiva en cwd.",
  "editor.portableWarn":
    "Ruta absoluta: no se moverá con esta carpeta. Usa {MP_HOME}\\... o una ruta ./ para mantenerla portable.",
  "editor.notPortable": "no portable",
  "editor.cwdHint":
    "El directorio de trabajo en el que se ejecuta el comando, normalmente la carpeta del proyecto. Usa una ruta absoluta, o {MP_HOME}\\... / ./ para mantenerla portable.",
  "editor.commandHint":
    "El comando que se ejecuta en la terminal integrada para iniciar la app, p. ej. 'npm run dev' o 'python app.py'. Déjalo vacío si la entrada es solo una URL estática.",
  "editor.portHint":
    "El puerto TCP local en el que escucha la app. Moonpool la marca «en ejecución» cuando ese puerto responde, y lo libera al detenerla. Lo usan las apps web.",
  "editor.processNameHint":
    "Para apps de escritorio: el nombre del proceso o ejecutable (sin .exe) que se usa para detectar si está en ejecución y para detenerla. En Linux no puede pasar de 15 caracteres.",
  "editor.urlHint":
    "La URL que se abre: http://localhost:<port> para una app web, o file:///path/to/index.html para una página estática. Usa file:///{MP_HOME}/... para mantenerla portable.",
  "editor.openBrowser": "abrir el navegador",
  "editor.openBrowserHint":
    "Abre la URL automáticamente en tu navegador predeterminado en cuanto la app responde.",
  "editor.envLabel": "env (KEY=VALUE por línea)",
  "editor.envHint":
    "Variables de entorno que se pasan al comando, una KEY=VALUE por línea (p. ej. PORT=3000).",
  "editor.noteHint":
    "Texto opcional que se muestra al pasar el cursor sobre la app en la barra lateral.",
  "editor.notePlaceholder": "nota opcional",
  "editor.phFolder": "ruta a la carpeta de la app",
  "editor.phLaunchCommand": "el comando de inicio de la app",
  "editor.phCommand": "comando a ejecutar",
  "editor.discardChanges": "¿Descartar los cambios?",
  "editor.nameRequired": "el nombre es obligatorio.",

  "term.processExited": "[proceso finalizado]",
};
