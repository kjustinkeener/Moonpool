---
title: "Edita apps.json: dónde está, cómo recargarlo y recuperarlo"
description: "Encuentra el archivo apps.json que Moonpool lee para cada app gestionada, edítalo en el editor de apps o a mano, recárgalo y recupérate de una edición errónea."
---

Cada app que gestiona Moonpool es una entrada de `apps.json`. Puedes editarlo desde el editor de apps
(el diálogo Añadir app y Editar app) o a mano. Ambos escriben el mismo archivo. Algunos resultados de
herramientas y mensajes llaman a este archivo el manifiesto.

## Dónde está la configuración

| Modo | Carpeta de configuración |
| --- | --- |
| Instalado (Windows) | `%USERPROFILE%\.moonpool\moonpool-config\` |
| Portable | `moonpool-config\` junto a `moonpool.exe` (dentro de la carpeta `.moonpool\`) |
| Linux | `$XDG_CONFIG_HOME/Moonpool/`, o si no `~/.config/Moonpool/` |

`apps.json` está en esa carpeta, junto a estos elementos:

| Elemento | Finalidad |
| --- | --- |
| `apps.json.history\` | Anillo de reversión con los últimos 10 archivos `apps.json` válidos. |
| `settings.json` | Ajustes de la app. Consulta [settings.json](/es/data/settings-json/). |
| `cli-output\<id>\` | Registros de sesión por app. Consulta [Registros](/es/data/logs/). |
| `moonpool.log` | Registro de depuración, mientras **Registrar información de depuración en un archivo** está activado. |
| `icons\` | Iconos `<id>.png` opcionales (también `.ico`, `.svg`, `.jpg`, `.jpeg`, `.webp`) que sustituyen a los predeterminados. |
| `state.json` | Instantánea del estado en vivo, actualizada cada pocos segundos. |
| `dumps\` | Archivos escritos por los verbos `dump`, `read-config` y `restore-config`. |
| `mcp_seen.json` | Qué apps han tenido un ayudante MCP. |
| `window-state.json` | El tamaño y la posición de la ventana del hub. |
| `AI-README.md` | La guía para agentes de IA, reescrita en cada inicio. |

Cuáles de estos conviene respaldar está en [Copia de seguridad y recuperación](/es/data/backup-and-recovery/#la-carpeta-de-configuración).

En la primera ejecución, Moonpool genera `apps.json` con entradas de ejemplo. Un archivo que ya existe
nunca se sobrescribe.

## Edición

- **Diálogo.** Usa **Añadir app** en el menú **...** de la parte superior de la barra lateral. Para
  cambiar una app, usa el lápiz de su fila o haz clic derecho en ella y elige **Editar**. El diálogo
  valida y guarda al instante.
- **A mano.** **Editar apps.json** en el mismo menú abre el archivo en tu editor predeterminado.
  Guárdalo y luego elige **Recargar** en el menú (o pulsa F5 o Ctrl+R).

Las ediciones a mano no se aplican hasta que recargas. Recargar solo lee el archivo; no lo reescribe.

Al guardar desde el diálogo se reescribe todo el archivo en una forma normalizada y con sangría. Las
claves que Moonpool no conoce se descartan, y JSON no admite comentarios, así que guarda las notas en el
campo `note`.

## Estructura

El archivo es una matriz JSON de objetos. Cuatro claves son obligatorias en cada entrada: `id`, `name`,
`group`, `type`. Todo lo demás es opcional. Consulta [Campos de las apps](/es/apps/fields/).

```json title="apps.json"
[
  { "id": "site", "name": "Site", "group": "Web apps", "type": "web",
    "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
    "url": "http://localhost:5173", "openBrowser": true }
]
```

Los grupos aparecen en la barra lateral en el orden en que aparecen por primera vez en el archivo.

## Qué hace recargar

Recargar sustituye la lista en memoria de Moonpool por el contenido del archivo. Iniciar, Detener y
Reiniciar leen la entrada cuando haces clic en ellos, así que un `command`, `cwd`, `env` o ajuste de
terminación editado se aplica la próxima vez que inicies o reinicies esa app. Recargar nunca reinicia
nada: una app que ya está en ejecución sigue en ejecución con los ajustes con los que se inició.

## Validación

Moonpool valida todo el archivo al cargarlo, en cada guardado y en cada escritura de un agente. Una sola
entrada errónea rechaza todo el archivo.

| Regla | El error contiene |
| --- | --- |
| No es JSON válido, falta una clave obligatoria o un valor tiene el tipo equivocado | el mensaje del analizador JSON |
| `id` está vacío, empieza por `-` o tiene caracteres distintos de letras, dígitos, `.`, `_`, `-` | `invalid id` |
| Dos entradas comparten un `id` | `duplicate app id` |
| `name` está en blanco | `has an empty name` |
| `group` está en blanco | `has an empty group` |
| `type` no es `desktop`, `web`, `static` ni `cli` | `unknown type` |
| `port` es `0` (un `port` superior a 65535 no se puede analizar) | `invalid port 0` |
| Entrada `static` sin `url` | `requires a url` |
| Cualquier otro tipo sin `command` | `requires a command` |

Los errores nombran la entrada por su posición, por ejemplo:

```text
apps.json entry 2 (site) requires a command
```

### El id

El `id` es la clave permanente de la entrada. Da nombre a la carpeta de registros y al archivo de icono, y
es lo que pasas a `moonpool.exe launch <id>` y a los agentes. El diálogo lo deriva del nombre cuando
añades una app. Pasa el nombre a minúsculas, convierte cada secuencia de caracteres que no sean de `a` a
`z` ni de `0` a `9` en un solo `-` y recorta los `-` de ambos extremos. Un resultado vacío pasa a ser
`app`. Si el id ya está en uso, añade `-2`, `-3` y así sucesivamente. Nunca cambia el id después, así que
cambiar el nombre de una app conserva su id. El nombre `Habit Tracker` recibe el id `habit-tracker`.

## Si el archivo está dañado

- **Al recargar**, un archivo que no supera la validación se deja intacto y Moonpool conserva la última
  lista que se cargó. Un aviso sobre la barra lateral muestra el error, con un botón para abrir el
  archivo; la lista sigue siendo utilizable pero atenuada. Consulta
  [Cuando apps.json tiene un error](/es/using/hub-window/#cuando-appsjson-tiene-un-error).
- **Al arrancar**, un archivo dañado significa que no hay ninguna lista que conservar, así que Moonpool
  arranca sin apps y el aviso lo indica. Corrige el archivo y elige **Recargar**, o restaura una
  instantánea (más abajo, o con la herramienta `moonpool_restore_config`).
- En cualquier caso, los guardados desde el diálogo (y cambiar nombre, eliminar, elegir icono) se
  rechazan hasta que el archivo vuelva a cargarse, de modo que el archivo dañado nunca se sobrescribe.
  Corrige el archivo y elige **Recargar**.
- **Desde el diálogo, un agente o una restauración**, un cambio no válido se rechaza y el archivo en
  disco se queda como estaba.

Moonpool conserva las últimas 10 versiones correctas de `apps.json` en `apps.json.history\`. Cómo volver
atrás está en [Copia de seguridad y recuperación](/es/data/backup-and-recovery/#revertir-appsjson).
Los síntomas y las soluciones están en [Solución de problemas](/es/support/troubleshooting/#appsjson-tiene-un-error).

## Agentes

Un agente de IA debe cambiar `apps.json` mediante las herramientas MCP de Moonpool en lugar del archivo,
de modo que una escritura obsoleta o no válida se rechace y un agente aislado nunca edite una copia
privada. Consulta [Herramientas MCP](/es/automation/mcp-tools/#configuración).
