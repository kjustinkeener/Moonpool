---
title: "Todos los campos de apps.json: tipo, valor predeterminado y qué hacen"
description: "Consulta cada clave de una entrada de apps.json con su tipo, su valor predeterminado y los tipos de app que la usan, con los mismos nombres que el diálogo Editar app."
---

El diálogo Editar app muestra los mismos campos con los mismos nombres. Los campos que no se aplican al
tipo seleccionado aparecen atenuados en el diálogo pero se guardan igualmente, con una excepción:
`stopCommand` solo se guarda mientras `killMode` es `command`.

![El diálogo Editar app desde name hasta stopCommand, con el selector killMode resaltado; los campos sin usar, como processName y stopCommand, aparecen atenuados](../../../../assets/screenshots/edit-app-dialog.png)

1. El selector `killMode`. Los campos que no usa permanecen atenuados.

| Campo | Tipo | Obligatorio | Lo usan | Qué hace |
| --- | --- | --- | --- | --- |
| `id` | string | sí | todos | Clave única. Letras, dígitos, `.`, `_`, `-`, sin empezar por `-`. Consulta [Resumen](/es/apps/apps-json/#el-id). |
| `name` | string | sí | todos | Etiqueta en la barra lateral. No puede estar en blanco. |
| `group` | string | sí | todos | Encabezado de la barra lateral bajo el que aparece la app. No puede estar en blanco en una edición a mano; el diálogo guarda un grupo en blanco como `Apps`. Cualquier texto; un nombre nuevo crea un grupo nuevo. |
| `type` | string | sí | todos | `web`, `desktop`, `static` o `cli`. Consulta [Tipos de app](/es/apps/types/). |
| `command` | string | todos salvo `static` | todos | Se ejecuta en una terminal para iniciar la app, mediante `cmd /c` en Windows y `$SHELL -c` en el resto (`/bin/sh` si `SHELL` no está definido). Opcional para `static`. |
| `cwd` | string | no | todos con un `command` | Carpeta en la que se ejecuta el comando. De forma predeterminada, la carpeta de trabajo del propio Moonpool. Admite tokens y `./`. Consulta [Rutas y entorno](/es/apps/paths-and-environment/). |
| `port` | integer, de 1 a 65535 | no | cualquiera | En ejecución mientras algo responda en este puerto de localhost (IPv4 o IPv6). Lo lee `killMode` `port`. |
| `processName` | string | no | cualquiera, sobre todo `desktop` | En ejecución mientras exista un proceso con este nombre. Sin distinguir mayúsculas de minúsculas, con o sin `.exe`, de modo que `my-app` coincide con `my-app.exe`. En Linux, 15 caracteres o menos. Lo lee `killMode` `processName`. |
| `mcpProcessName` | string | no | cualquiera con un `processName` | Patrón con comodines del nombre de proceso del servidor MCP de esta app. `*` coincide con cualquier secuencia de caracteres, `?` con un carácter. Sin distinguir mayúsculas de minúsculas, se compara con el nombre completo, y `.exe` es opcional. Un proceso que coincide cuenta como el servidor MCP de la app (la subfila MCP de la barra lateral) y no necesita `mcp` como primer argumento. Consulta [mcpProcessName](#mcpprocessname). |
| `url` | string | solo `static` | `web`, `static` | Página que se abre. Solo se abren las URL `http://`, `https://`, `mailto:` y `file://`. |
| `openBrowser` | boolean, predeterminado `false` | no | cualquier tipo con una `url` (el diálogo la atenúa para `desktop` y `cli`) | Abre `url` automáticamente cuando Moonpool detecta que la app está activa (consulta más abajo). |
| `killMode` | string | no | todos | Limpieza adicional al Detener y Reiniciar: `processName`, `port`, `command` o `none`. Consulta [Detener y reiniciar](/es/apps/stop-and-restart/). |
| `stopCommand` | string | no | `killMode` `command` | Comando que se ejecuta al Detener. Se ignora en cualquier otro modo. |
| `env` | object of strings | no | todos | Variables de entorno adicionales. El diálogo las edita como una `KEY=VALUE` por línea. |
| `icon` | string | no | todos | Imagen de la barra lateral: una ruta de archivo, una URL `http(s)` o un URI `data:`. Defínela desde **Elegir icono...** en el menú contextual de la app o a mano. |
| `note` | string | no | todos | Información emergente al pasar el cursor por la app en la barra lateral. |

Una entrada que usa `env` y `killMode`:

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000", "NODE_ENV": "development" },
  "killMode": "port"
}
```

Consulta [Encontrar y terminar el proceso que usa un puerto](/es/guides/find-and-kill-process-using-port-windows/)
para ver cómo funcionan juntos `port` y `killMode`.

## mcpProcessName

De forma predeterminada, Moonpool trata un proceso como el servidor MCP de la app cuando su nombre
coincide con `processName` y su primer argumento es `mcp`, como `notes-app.exe mcp`. Define
`mcpProcessName` cuando el servidor se ejecuta con otro nombre: una app que vigila un exe mientras su
servidor MCP es otro (`mog.exe mcp`), o una copia renombrada del servidor.

El valor es un patrón con comodines. `*` coincide con cualquier secuencia de caracteres (incluso vacía) y
`?` con exactamente uno. Se compara sin distinguir mayúsculas de minúsculas con el nombre completo del
proceso, y un patrón sin `.exe` también coincide con el nombre con `.exe`. Un valor vacío cuenta como no
definido.

```json
{
  "id": "destiny",
  "name": "Destiny",
  "group": "Desktop apps",
  "type": "desktop",
  "processName": "destiny",
  "mcpProcessName": "destiny-mcp-*"
}
```

Esto coincide con una copia renombrada como `destiny-mcp-2706210170.exe`. Un proceso que coincide con
`mcpProcessName` es el servidor, se haya iniciado o no con `mcp`, y nunca cuenta como la propia app en
ejecución. Si el patrón también coincide con `processName` (por ejemplo `destiny*`), Moonpool sigue
exigiendo el argumento `mcp`, de modo que la app real nunca se confunde con su servidor MCP. Consulta
[Configuración de MCP](/es/automation/mcp-setup/#apps-que-tienen-su-propio-servidor-mcp).

## openBrowser

Moonpool abre `url` una sola vez, cuando una app que Moonpool inició aparece por primera vez como en
ejecución. Para detectarlo hace falta un `port` o un `processName`. Sin ninguno de los dos, "en
ejecución" solo significa que el proceso de la terminal sigue vivo, y el navegador no se abre
automáticamente. Desactiva `openBrowser` si tu comando abre un navegador por sí mismo. Una entrada
`static` sin comando abre `url` cada vez que pulsas Iniciar, con independencia de `openBrowser`.

Dos apps configuradas con el mismo `port` se señalan en la barra lateral.

## Iconos

El icono de una app es el primero de estos que exista:

1. El campo `icon`.
2. `icons\<id>.<ext>` en la carpeta de configuración, por ejemplo `icons\site.png`.
3. Un archivo de icono en la propia carpeta de la app (su `cwd`, o la carpeta de una `url` `file:///`).
4. Para `desktop`, el icono de su `.exe` compilado o en ejecución.
5. Para `web` y `static`, el `/favicon.ico` del sitio, una vez que el servidor está activo.
6. Un glifo del tipo.

La mayoría de las apps no necesitan ningún ajuste de icono.
