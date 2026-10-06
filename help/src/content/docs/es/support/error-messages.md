---
title: "Mensajes de error de Moonpool explicados: already running, requires a command y más"
description: "Consulta el texto exacto de los mensajes de error de Moonpool, como already running, requires a command o stale token, con su significado y la solución."
---

Pega el mensaje que ves en el buscador de la página, o repasa las tablas. Los mensajes se citan tal como
los muestra Moonpool. El texto entre `<corchetes angulares>` se sustituye por un valor (un id de app, una
ruta o un error del sistema). Los síntomas que no son un mensaje de error están en
[Solución de problemas y preguntas frecuentes](/es/support/troubleshooting/).

## Iniciar y detener una app

| Mensaje | Significado y solución |
| --- | --- |
| `already running` | Moonpool ya tiene una terminal para esta app. Detenla primero, o usa Reiniciar. |
| `stopped during launch` | Se pulsó Detener mientras el inicio todavía estaba en curso. Vuelve a iniciar. |
| `app has no launch command` | La entrada no tiene `command`. Añade uno en el editor de apps o en `apps.json`. Solo una entrada `static` con una `url` puede prescindir de él. |
| `unknown app: <id>` | No hay cargada ninguna app con ese `id`. Comprueba el id y, si editaste `apps.json` a mano, Recarga. |
| `unknown app id: <id>` | El mismo problema, comunicado a un script o agente. Lista las apps con `moonpool_list_apps`. |
| `did not reach running in time` | Desde un script o agente: la app no figuró como en ejecución en 25 segundos. Revisa `port` o `processName` y lee la salida. Consulta [El punto de estado es incorrecto](/es/support/troubleshooting/#el-punto-de-estado-es-incorrecto). |
| `still running after stop` | Pasados 15 segundos, la app sigue figurando como en ejecución. Define `killMode`. Consulta [Detener y reiniciar](/es/apps/stop-and-restart/). |
| `refusing to open non-web url: <url>` | La `url` no es `http://`, `https://`, `mailto:` ni `file://`. Corrige la `url`. |
| `[process exited]` | No es un error: el comando de la app terminó. Se muestra en la pestaña de terminal (en español: `[proceso finalizado]`). |

## Validación de apps.json

Moonpool rechaza un `apps.json` que incumple una regla y conserva la última lista que se cargó. `<n>` es
la posición de la entrada en el archivo, contando desde 1.

| Mensaje | Solución |
| --- | --- |
| `apps.json entry <n> has invalid id "<id>"; use letters, digits, '.', '_', and '-' without a leading '-'` | Cambia el `id`. |
| `duplicate app id "<id>"` | Dos entradas comparten un `id`. Haz que cada uno sea único. |
| `apps.json entry <n> (<id>) has an empty name` | Rellena `name`. |
| `apps.json entry <n> (<id>) has an empty group` | Rellena `group`. |
| `apps.json entry <n> (<id>) has unknown type "<type>"` | `type` debe ser `web`, `desktop`, `static` o `cli`. |
| `apps.json entry <n> (<id>) has invalid port 0` | `port` debe estar entre 1 y 65535. |
| `apps.json entry <n> (<id>) requires a url` | Una entrada `static` necesita una `url`. |
| `apps.json entry <n> (<id>) requires a command` | Todos los demás tipos necesitan un `command`. |

En el editor de apps, guardar sin un nombre muestra `el nombre es obligatorio.`.
El texto del aviso, "apps.json tiene un error; se muestra la última lista que se cargó." o "apps.json tiene
un error, así que no hay apps cargadas.", y cómo recuperarse están en
[apps.json tiene un error](/es/support/troubleshooting/#appsjson-tiene-un-error). Si el aviso dice que se
han pausado los guardados, el mensaje termina con `Repair apps.json and reload it before saving from
Moonpool` (repara apps.json y recárgalo antes de guardar desde Moonpool). La lista completa de reglas está
en [Validación](/es/apps/apps-json/#validación).

## Ajustes, actualizaciones y el instalador

| Mensaje | Significado y solución |
| --- | --- |
| `... Repair settings.json and restart Moonpool before changing settings` | `settings.json` está mal formado. Corrígelo o elimínalo y reinicia. Consulta [settings.json](/es/data/settings-json/#lectura-y-reparación). |
| `Error al actualizar: <error>` | Falló la descarga o la instalación de una actualización. Consulta [Cuando falla una actualización](/es/data/updating/#cuando-falla-una-actualización). |
| `Error al buscar actualizaciones: <error>` | Falló la búsqueda de actualizaciones en Acerca de. El texto tras los dos puntos dice por qué. Inténtalo de nuevo más tarde. |
| `Error de instalación: <error>` | El instalador se detuvo en el paso que se nombra tras los dos puntos, por ejemplo `copy exe: ...`. Sal de cualquier Moonpool que se ejecute desde `%USERPROFILE%\.moonpool` y reintenta. |
| `target folder does not exist` | La carpeta elegida para una copia portable ya no existe. Elige una que exista. |

## MCP y scripts

| Mensaje | Significado y solución |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Inicia Moonpool, o deja que el agente llame a esa herramienta. Para una copia portable, el mensaje nombra la copia. |
| `frontend not loaded` | La ventana del hub no ha terminado de cargarse. Espera y reintenta. |
| `invalid app_id: use only letters, digits, '.', '_', '-' (no leading '-')` | El agente pasó un id que el servidor MCP no acepta. Usa el id de `moonpool_list_apps`. |
| `stale token: apps.json changed since it was read ...` | Vuelve a leer `apps.json`, aplica de nuevo el cambio y luego escribe. |
| `rejected invalid manifest: ...` | El nuevo `apps.json` no superó la validación (consulta arriba). El archivo no se modificó. |
| `no console output recorded for '<id>' (not launched this session)` | Se pidió `moonpool_app_output` de una app que no se ha ejecutado desde que Moonpool arrancó. |

Más en [Herramientas MCP](/es/automation/mcp-tools/) y
[Configuración de MCP](/es/automation/mcp-setup/#si-las-herramientas-no-funcionan).

## Errores de otros programas

- [`Error: listen EADDRINUSE: address already in use :::3000` y `Port 5173 is in use`](/es/support/port-already-in-use/)
- [`Windows protected your PC`](/es/support/windows-protected-your-pc/)
- [Falta el runtime de WebView2](/es/support/webview2-runtime-missing/)
