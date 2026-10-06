---
title: "Cambia los ajustes de Moonpool: todas las opciones de Ajustes y Acerca de"
description: "Lista completa de los controles de las ventanas Ajustes y Acerca de de Moonpool, la clave de settings.json que escribe cada uno y cómo restablecer un ajuste."
---

Abre **Ajustes** desde el menú "..." del hub. Los cambios se guardan a medida que los haces. Esc cierra
la ventana. Esta página es la lista completa de ajustes. Cada uno se guarda en `settings.json` bajo la
clave indicada; el propio archivo se describe en
[settings.json](/es/data/settings-json/).

![Ventana de Ajustes: interruptores y controles deslizantes en la columna izquierda, opciones de registro en la derecha](../../../../assets/screenshots/settings-window.png)

## Restablecer un control

Haz clic derecho en cualquier casilla, control deslizante o campo numérico para restablecer solo ese
ajuste a su valor predeterminado. La información emergente de cada control lo indica. Los selectores de
Idioma y Tema no tienen restablecimiento.

## Columna izquierda

| Control | Clave | Predeterminado | Qué hace |
| --- | --- | --- | --- |
| Idioma | `locale` | Automático (sistema) | Idioma del texto propio de Moonpool. Se aplica al instante. Consulta [Temas, idioma y transparencia](/es/using/themes-and-language/). |
| Tema | ninguna (almacenamiento del navegador) | Automático (sistema) | Tema de color. El botón abre un explorador de temas con una vista previa de cada tema; al hacer clic en uno se aplica al instante. Consulta [Temas, idioma y transparencia](/es/using/themes-and-language/). |
| Cerrar a la bandeja | `closeToTray` | desactivado | Activado: al cerrar la ventana, Moonpool se oculta en la bandeja. Desactivado: cerrar sale de la app. |
| Minimizar a la bandeja | `minimizeToTray` | activado | Activado: al minimizar, Moonpool se oculta en la bandeja y sale de la barra de tareas. Desactivado: se minimiza a la barra de tareas. |
| Siempre visible | `alwaysOnTop` | desactivado | Mantiene todas las ventanas de Moonpool por encima de las demás. |
| Mostrar en la bandeja | `showInTray` | activado | Mantiene visible el icono de la bandeja. |
| Mostrar en la barra de tareas | `showInTaskbar` | activado | Mantiene visible el botón de la barra de tareas. |
| Mostrar la barra de estado de CPU y memoria | `showStatusbar` | activado | Barra en vivo de CPU y memoria en la parte inferior del hub. |
| Mostrar procesos MCP | `showMcpProcesses` | activado | Muestra el proceso MCP de una app como subfila MCP en la barra lateral mientras se usan sus herramientas MCP. |
| Transparencia del fondo | `transparency` | 0 % | Control deslizante de 0 a 90 en pasos de 5. Consulta [Temas, idioma y transparencia](/es/using/themes-and-language/#transparencia). |
| Buscar actualizaciones al iniciar | `checkOnStartup` | activado | Consulta GitHub al arrancar en busca de una versión más reciente y muestra un aviso si la encuentra. Consulta [Actualización](/es/data/updating/). |

### Bloqueo de bandeja y barra de tareas

Al menos uno de **Mostrar en la bandeja** y **Mostrar en la barra de tareas** debe seguir activado; si
no, una ventana oculta no tendría forma de volver. Cuando solo uno está activado, su casilla queda
desactivada hasta que vuelvas a activar el otro.

## Columna derecha: registros

| Control | Clave | Predeterminado | Qué hace |
| --- | --- | --- | --- |
| Conservar los registros de salida de las apps entre sesiones | `cliLogging` | desactivado | La salida de terminal de la sesión en curso siempre se conserva para sus propias pestañas. Activado: los registros de sesiones anteriores permanecen en disco en `cli-output\`, limitados por el ajuste de retención. Desactivado: se borran la próxima vez que se inicia esa app. |
| Retención de registros por app | `logRetentionMb` | 10 MB | Límite del total de registros de cada app. Mínimo 1. Desactivado mientras el interruptor anterior está desactivado. El registro de la sesión actual cuenta para el límite, pero este nunca lo recorta ni lo borra. |
| Registrar información de depuración en un archivo | `debugLogging` | desactivado | Registra en `moonpool.log` las cargas de `apps.json`, los inicios y los errores. |

Bajo cada grupo de registros, un campo de ruta muestra la ubicación, con dos botones:

- **Abrir** abre la carpeta en el administrador de archivos (**Abrir la carpeta de registros CLI** para `cli-output\`, **Abrir el registro** para `moonpool.log`).
- **Copiar** pone la ruta en el portapapeles (**Copiar la ruta de la carpeta de registros CLI**, **Copiar la ruta del archivo de registro**).

Los formatos de los archivos de registro, el separador de reinicio y las reglas de retención están en
[Registros](/es/data/logs/).

Si una casilla no se puede guardar, un mensaje rojo en la parte superior de la ventana lo indica y la
casilla vuelve a su estado anterior.

## Ventana Acerca de

Abre **Acerca de** desde el menú "...".

![Ventana Acerca de con la línea de versión, enlaces y los botones Buscar actualizaciones y Cerrar](../../../../assets/screenshots/about-window.png)

Muestra:

- La versión y la fecha de compilación.
- Enlaces al sitio del proyecto, al repositorio de GitHub y a la dirección de contacto.
- **Buscar actualizaciones**. Si existe una versión más reciente, la descarga, la verifica y la instala, y luego reinicia Moonpool. Si no, informa de que tienes la última versión, o del error si la búsqueda falló.
- Los créditos de las bibliotecas con las que está creado Moonpool y del autor.

Esc la cierra. Acerca de sigue en vivo los ajustes de tema, transparencia e idioma.
