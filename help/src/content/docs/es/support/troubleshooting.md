---
title: "Solución de problemas de Moonpool: bandeja, apps que no inician, actualizaciones"
description: "Resuelve los problemas habituales de Moonpool según lo que veas: icono de bandeja ausente, apps que no inician, puntos de estado incorrectos y errores de MCP."
---

Busca el síntoma y sigue la solución. El texto entre comillas es lo que muestra Moonpool. Para buscar un
mensaje exacto, consulta [Mensajes de error explicados](/es/support/error-messages/).

## No veo el icono de la bandeja

- **Windows.** El icono puede estar en el área de iconos ocultos. Haz clic en la flecha **^** a la derecha
  de la barra de tareas. Arrastra el icono a la barra de tareas para mantenerlo visible.
- **Linux con GNOME estándar.** GNOME no muestra iconos de bandeja sin la extensión AppIndicator.
  Consulta [Linux](/es/platforms/linux/#bandeja-en-gnome).
- **Ajustes.** **Mostrar en la bandeja** puede estar desactivado. Abre el hub desde la barra de tareas o el
  menú Inicio y vuelve a activarlo en [Ajustes](/es/using/settings/).

## El instalador muestra un error

| Mensaje | Qué hacer |
| --- | --- |
| `Error de instalación: <error>` | El texto tras los dos puntos nombra el paso que falló, por ejemplo `copy exe: ...`. Si un archivo está en uso, sal de cualquier Moonpool que se ejecute desde `%USERPROFILE%\.moonpool` e inténtalo de nuevo. |
| `target folder does not exist` | La carpeta que elegiste para una copia portable ya no existe. Elige una carpeta que exista. |
| `that folder already has a .moonpool with an exe of this name that isn't a portable Moonpool - pick an empty folder` | Elige una carpeta vacía, o elimina antes esa carpeta `.moonpool`. |

## Aparece Windows protected your PC al ejecutar el instalador

Es Windows SmartScreen, porque `moonpool.exe` no está firmado con código. Haz clic en **More info** (Más
información) y luego en **Run anyway** (Ejecutar de todas formas). Consulta
[Windows protected your PC](/es/support/windows-protected-your-pc/).

## La ventana de Moonpool está en blanco o nunca se abre en Windows

Puede faltar el runtime de Microsoft Edge WebView2. Consulta
[Falta el runtime de WebView2](/es/support/webview2-runtime-missing/).

## Una app no inicia

1. Haz clic en el nombre de la app para abrir su pestaña de terminal y lee la salida. Un agente puede leer
   el mismo texto con `moonpool_app_output`.
2. Comprueba `cwd`. Una carpeta inexistente, o una ruta relativa sin `./`, es la causa habitual. Consulta
   [Rutas y entorno](/es/apps/paths-and-environment/).
3. Comprueba `command`. Ejecútalo a mano en una terminal en `cwd`. En Windows evita las comillas dobles
   anidadas; `cmd /c` las estropea.
4. Activa **Registrar información de depuración en un archivo** en Ajustes y vuelve a iniciar.
   `moonpool.log` registra el comando y la carpeta exactos. Consulta [Registros](/es/data/logs/).

| Mensaje | Significado |
| --- | --- |
| `already running` | Moonpool ya tiene una terminal para esta app. Detenla primero, o usa Reiniciar. |
| `stopped during launch` | Se pulsó Detener mientras el inicio todavía estaba en curso. |
| `did not reach running in time` | Desde un script o agente: la app no figuró como en ejecución en 25 segundos. Comprueba su `port` o `processName`, y su salida. |

## El punto de estado es incorrecto

Moonpool decide si está en ejecución a partir de `port`, luego `processName` y luego si su propia terminal
sigue viva. Consulta [Cómo se decide si está en ejecución](/es/apps/types/#cómo-se-decide-si-está-en-ejecución).

- **Nunca se queda fijo.** El `port` de una app `web` no responde, o el `processName` de una app `desktop`
  no coincide. En Linux, `processName` debe tener 15 caracteres o menos.
- **Se pone gris justo después de iniciar.** Una app `cli` deja de estar en ejecución cuando su comando
  termina. Usa un shell con `-NoExit` si quieres que siga abierto.
- **Una app `static` nunca aparece como en ejecución.** Es lo esperado en una entrada con solo una `url`.
- **Aparece en ejecución aunque no la iniciaste.** Otra cosa está usando ese puerto o nombre de proceso.
  Moonpool la muestra como en ejecución pero no "gestionada por Moonpool".

## Error: listen EADDRINUSE o "Port 5173 is in use"

Otra cosa ya está escuchando en el puerto que quiere tu servidor. Encuéntrala y termínala, o define `port`
en la app para que Detener lo libere. Consulta
[Solucionar EADDRINUSE y "Port 5173 is in use"](/es/support/port-already-in-use/) y
[Encontrar y terminar el proceso que usa un puerto](/es/guides/find-and-kill-process-using-port-windows/).

## Dos apps usan el mismo puerto

Aparece una fila de aviso al final del menú **...**, por ejemplo `port 3000: App A / App B`. Cambia el
`port` de una de las apps (y su `env`, si lee `PORT`). Consulta
[Aviso de conflicto de puertos](/es/using/hub-window/#aviso-de-conflicto-de-puertos).

## La app sigue en ejecución después de Detener

Desde un script o agente, el error es `still running after stop` (a los 15 segundos).

- La app sobrevive a su terminal. Define `killMode` como `port` o `processName`. Consulta
  [Detener y reiniciar](/es/apps/stop-and-restart/).
- Una app de Docker en Windows: usa `killMode` `command` con un `stopCommand` como
  `docker compose stop app`. Nunca `port`.

## apps.json tiene un error

La barra lateral muestra un aviso, "apps.json tiene un error; se muestra la última lista que se cargó." o,
al arrancar, "apps.json tiene un error, así que no hay apps cargadas." Los guardados desde Moonpool quedan
en pausa hasta que el archivo vuelva a cargarse.

Errores típicos:

```text
apps.json entry 2 (site) requires a command
apps.json entry 3 has invalid id "my app"; use letters, digits, '.', '_', and '-' without a leading '-'
duplicate app id "site"
apps.json entry 4 (api) has invalid port 0
```

1. Elige **Editar apps.json** en el aviso, corrige la entrada, guarda y luego **Recargar** (F5).
2. O vuelve a una copia reciente que funcione. Consulta
   [Copia de seguridad y recuperación](/es/data/backup-and-recovery/#revertir-appsjson).

La lista completa de reglas está en [Validación](/es/apps/apps-json/#validación).

Si un ajuste no se puede cambiar y el mensaje termina con `Repair settings.json and restart
Moonpool before changing settings`, corrige o elimina `settings.json` en la carpeta de configuración y
vuelve a iniciar Moonpool. Al eliminarlo, todos los ajustes vuelven a sus valores predeterminados.

## Mi cambio no surtió efecto

- Las ediciones a mano necesitan **Recargar** (o F5). Moonpool no vigila el archivo.
- Recargar no reinicia las apps en ejecución. Reinicia la app para usar un `command`, `cwd` o `env`
  modificado.
- Un agente puede estar editando otro `apps.json`. Pídele que llame a `moonpool_launcher_paths` y compare
  la carpeta del hub con la suya. Con varias copias de Moonpool, comprueba qué copia estás editando.

## Faltan las apps de ejemplo

Los ejemplos se escriben solo cuando no existe ningún `apps.json`. Para recuperarlos, consulta
[Volver a los ejemplos](/es/data/backup-and-recovery/#volver-a-los-ejemplos), o copia las entradas de
[Paneles de ejemplo](/es/getting-started/example-dashboards/#las-apps-de-ejemplo-solo-aparecen-la-primera-vez).

## Una actualización falló

El aviso muestra `Error al actualizar: <error>`. Consulta
[Cuando falla una actualización](/es/data/updating/#cuando-falla-una-actualización).

## Un enlace web no se abre

`refusing to open non-web url: <url>` significa que la `url` no es `http://`, `https://`, `mailto:` ni
`file://`. Corrige la `url`.

## Errores de MCP y de scripts

| Mensaje | Qué hacer |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Inicia Moonpool, o deja que el agente llame a `moonpool_bootup_launcher`. |
| `frontend not loaded` | La ventana del hub no ha terminado de cargarse. Espera un momento y reintenta. |
| `stale token: ...` | `apps.json` cambió desde que el agente lo leyó. Léelo de nuevo y luego escribe. |
| `rejected invalid manifest: ...` | El nuevo `apps.json` no superó la validación. El archivo no se modificó. |
| `... A Moonpool process may be hung ...` | Algo ocupa el canal de control sin responder. Sal de Moonpool desde la bandeja, o termina el proceso, y vuelve a iniciarlo. |

Más en [Configuración de MCP](/es/automation/mcp-setup/#notas) y
[Herramientas MCP](/es/automation/mcp-tools/).

## Problemas con la ventana

- **Fuera de la pantalla.** Moonpool ignora una posición guardada que no está en ninguna pantalla
  conectada. Si la ventana sigue perdida, sal de Moonpool y elimina `window-state.json` en la carpeta de
  configuración.
- **El zoom se queda demasiado grande o demasiado pequeño.** Ctrl + rueda sobre el hub lo cambia. Consulta
  [Atajos y zoom](/es/using/keyboard-shortcuts/#zoom).
- **Ajustes se abre detrás del hub.** Desactiva o activa **Siempre visible** en Ajustes. Se aplica a todas
  las ventanas de Moonpool, de modo que permanecen en la misma capa.

## ¿Dónde están los registros?

Consulta [Registros](/es/data/logs/).

## Copia de seguridad, restablecimiento o desinstalación

Consulta [Copia de seguridad y recuperación](/es/data/backup-and-recovery/) y
[Desinstalación](/es/getting-started/install/#desinstalación).

## Preguntas frecuentes

**¿Cerrar la ventana detiene mis apps?**
De forma predeterminada, cerrar sale de Moonpool, y en Windows salir detiene las apps que inició. Activa
**Cerrar a la bandeja** para mantener Moonpool en ejecución cuando cierres la ventana. Consulta
[Bandeja, cerrar y minimizar](/es/using/tray-and-closing/).

**¿Puedo ejecutar Moonpool dos veces?**
Uno por carpeta. Iniciar de nuevo la misma copia vuelve a mostrar su ventana. La copia instalada y las
copias portables pueden ejecutarse en paralelo. Consulta
[Modo portable](/es/data/portable-mode/#varias-copias-a-la-vez).

**¿Moonpool envía datos a algún servidor?**
Solo para buscar actualizaciones: obtiene el archivo de versión (`update.json`) de GitHub al arrancar (si
**Buscar actualizaciones al iniciar** está activado) y cuando pulsas **Buscar actualizaciones**. Cada
descarga se verifica con la clave de firma de Moonpool antes de usarse.

**¿Qué shell ejecuta mis comandos?**
`cmd /c` en Windows, `$SHELL -c` en Linux.

**¿Dónde pongo los secretos?**
Los valores de `env` se guardan en texto plano en `apps.json`. Es preferible un archivo que lea la propia
app, o una variable ya definida en tu entorno de usuario, que las apps iniciadas heredan.

**¿Está protegido el canal de control?**
No tiene inicio de sesión ni token. Cualquier proceso que se ejecute como tú puede enviarle comandos. En
Linux, el socket solo puede leerlo tu usuario. Consulta
[Propiedades de seguridad](/es/automation/overview/#propiedades-de-seguridad).
