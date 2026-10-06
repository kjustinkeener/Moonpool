---
title: "Glosario de Moonpool: apps, estados, archivos y ajustes"
description: "Definiciones sencillas de las palabras que usa la ayuda de Moonpool para sus partes, los estados de las apps, los archivos y los ajustes."
---

## Apps

| Término | Significado |
| --- | --- |
| app | Algo que Moonpool gestiona: un servidor de desarrollo, una app de escritorio, una página o un comando. |
| entrada | El registro de una app en `apps.json`. Se usa solo al hablar del JSON. |
| fila de app | La línea de una app en la barra lateral, con su punto de estado y sus controles. |
| grupo | El encabezado de la barra lateral bajo el que aparece una app, a partir de su campo `group`. |
| tipo | `web`, `desktop`, `static` o `cli`. Decide qué campos importan. Consulta [Tipos de app](/es/apps/types/). |
| id | La clave permanente de una app, usada en nombres de archivo, comandos y herramientas de agentes. Consulta [El id](/es/apps/apps-json/#el-id). |

## Estados de una app

| Estado | Significado |
| --- | --- |
| iniciando | Moonpool inició la app pero todavía no la ha visto activa. Punto parpadeante. |
| en ejecución | Su `port` responde, su `processName` existe o, si no se ha definido ninguno, la terminal que inició Moonpool sigue viva. Punto fijo. Consulta [Cómo se decide si está en ejecución](/es/apps/types/#cómo-se-decide-si-está-en-ejecución). |
| detenida | Ninguna de las anteriores. Punto gris. |
| gestionada | Moonpool la inició en esta sesión. Una app en ejecución que no está gestionada se inició de otra forma, y Salir la deja en paz. |

Una pestaña de terminal y una app en ejecución son cosas distintas. Al hacer clic en el nombre de una app
solo se abre su pestaña de terminal; nunca inicia la app. Cerrar una pestaña nunca detiene la app.

## Ventanas y partes

| Término | Significado |
| --- | --- |
| hub | El proceso residente de Moonpool y su ventana principal. Los nombres de las herramientas lo llaman "launcher" (lanzador). |
| ventana del hub | La ventana principal: la barra lateral a la izquierda, el panel CLI a la derecha. |
| bandeja | El icono de la bandeja del sistema y su menú (**Mostrar Moonpool**, **Salir**). |
| barra lateral | El lado izquierdo de la ventana del hub: cuadro de filtro, menú **...** y filas de apps. |
| panel CLI | El lado derecho de la ventana del hub, que contiene las pestañas de terminal. |
| pestaña de terminal | La terminal de una app en el panel CLI. |
| subfila MCP | Una fila atenuada bajo una app que muestra su propio proceso ayudante `<exe> mcp`. |
| editor de apps | El diálogo Añadir app y Editar app. |

## Archivos y carpetas

| Término | Significado |
| --- | --- |
| carpeta de configuración | La carpeta que contiene `apps.json` y los demás archivos de Moonpool. El token `{MP_DATA}`. Consulta [Dónde está la configuración](/es/apps/apps-json/#dónde-está-la-configuración). |
| `{MP_HOME}` | La carpeta de Moonpool: `%USERPROFILE%\.moonpool` en una instalación, la carpeta `.moonpool\` de una copia portable, la carpeta de configuración en Linux. |
| sesión | Una ejecución del hub, desde el inicio hasta Salir. |
| registro de sesión | El archivo que contiene todo lo que una app imprimió durante una sesión, en `cli-output\`. Consulta [Registros](/es/data/logs/). |
| `moonpool.log` | El registro de depuración del propio Moonpool, que se escribe solo con **Registrar información de depuración en un archivo** activado. |
| volcado | Una copia en texto plano de un registro de sesión hecha con el verbo `dump`. |
| instantánea | Una copia de un `apps.json` correcto en `apps.json.history\`. Consulta [Copia de seguridad y recuperación](/es/data/backup-and-recovery/). |

## Modos

| Término | Significado |
| --- | --- |
| instalado | Un Moonpool en `%USERPROFILE%\.moonpool`, con acceso directo en el menú Inicio y entrada en Agregar o quitar programas. Solo Windows. |
| portable | Un Moonpool en una carpeta `.moonpool\` que elegiste, marcado por un archivo `moonpool.portable`. Consulta [Modo portable](/es/data/portable-mode/). |
| copia | Una carpeta de Moonpool, instalada o portable. Cada copia se ejecuta por su cuenta. |

## Detener y automatización

| Término | Significado |
| --- | --- |
| `killMode` | El paso adicional que da Detener tras terminar la terminal de la app. Consulta [Detener y reiniciar](/es/apps/stop-and-restart/). |
| `stopCommand` | El comando que ejecuta Detener cuando `killMode` es `command`. |
| `processName` | El nombre de proceso que Moonpool vigila, y que termina en el modo `processName`. |
| canal de control | La canalización con nombre (Windows) o el socket Unix (Linux, macOS) en el que responde el hub. Consulta [Verbos de control](/es/automation/control-verbs/). |
| verbo | Una palabra de comando como `launch` o `reload`, que se indica en la línea de comandos o en el canal de control. |
| ticket | Una clave que añades con `--ticket` para leer en `state.json` el resultado de un comando. |
| token | El sello de versión de `apps.json` que debe llevar una escritura de configuración. |
| ayudante MCP (shim) | Un proceso `<exe> mcp` que un host de IA inicia para llegar a las herramientas propias de una app. |
