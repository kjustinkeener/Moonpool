---
title: "Referencia de las herramientas MCP de Moonpool: parámetros y resultados"
description: "Todas las herramientas que el servidor MCP de Moonpool expone a los agentes, con sus parámetros, lo que devuelven y los casos de error que puedes encontrar."
---

Todas las herramientas devuelven texto, salvo `moonpool_screenshot`, que devuelve una imagen PNG. Un fallo
llega como un resultado de herramienta marcado como error, con el motivo como texto. Para la
configuración, consulta [Configuración de MCP](/es/automation/mcp-setup/).

Las herramientas que toman `app_id` necesitan el `id` de la app de `apps.json`. Debe usar solo letras,
dígitos, `.`, `_` y `-`, y no empezar por `-`; de lo contrario la llamada falla con "invalid app_id".

La mayoría de las herramientas que actúan sobre el hub fallan con este mensaje cuando no está en
ejecución. `moonpool_bootup_launcher`, `moonpool_shutdown_launcher`, `moonpool_raise_launcher` y
`moonpool_launcher_paths` manejan ese caso por sí mismas (consulta sus filas). Para una copia portable, el
mensaje nombra la copia, por ejemplo `Moonpool (<folder>)`.

```text
Moonpool is not running - call moonpool_bootup_launcher first
```

(Moonpool no está en ejecución: llama primero a moonpool_bootup_launcher.)

Las llamadas que esperan un resultado caducan tras 45 segundos.

## Lanzador y apps

Ejemplo de resultado de `moonpool_list_apps`:

```text
site  [running] (managed by Moonpool)  Site
notes-app  [stopped]  [mcp: stopped]  Notes App
```

| Herramienta | Parámetros | Comportamiento |
| --- | --- | --- |
| `moonpool_list_apps` | ninguno | Una línea por app: `id  [running]` o `[stopped]`, `(managed by Moonpool)` cuando corresponde, `[mcp: running]` o `[mcp: stopped]` cuando se ha visto un ayudante MCP, y luego el nombre. Se pregunta al hub en ejecución a través del canal de control (verbo `list`), así que es en vivo. Si Moonpool no está en ejecución, falla con "Moonpool is not running" en lugar de mostrar una lista obsoleta. Justo después de que Moonpool arranque, antes de su primera comprobación de estado, las apps muestran `[status pending]`. Mientras `apps.json` tiene un error, el resultado empieza con `apps.json has an error: <message>. This list is the last one that loaded; fix the file and call moonpool_reload_config.` Si el archivo ya estaba dañado cuando Moonpool arrancó, indica que no hay apps cargadas y sugiere también `moonpool_restore_config`. |
| `moonpool_bootup_launcher` | ninguno | Inicia el propio Moonpool y espera hasta 30 s a que responda su canal de control. Devuelve "Moonpool started" o "Moonpool is already running". Si el nuevo proceso termina de inmediato (cedió el control a un Moonpool que todavía se estaba cerrando), inicia otro más. Si algo ocupa el canal sin responder, informa de que un proceso de Moonpool puede estar colgado. |
| `moonpool_shutdown_launcher` | ninguno | Igual que Salir en el menú de la bandeja. Espera hasta 30 s a que desaparezca el canal de control. Devuelve "Moonpool shut down" o "Moonpool is not running". |
| `moonpool_raise_launcher` | ninguno | Trae la ventana de Moonpool al frente. Devuelve "window shown". Si Moonpool no está en ejecución, lo inicia y devuelve "Moonpool was not running; started it". |
| `moonpool_start_app` | `app_id` (obligatorio) | Inicia la app y abre su pestaña de terminal. Devuelve "launched" cuando está en ejecución, o el motivo por el que no lo está (`unknown app id: <id>`, `did not reach running in time` a los 25 s). Para una entrada `static` con solo una `url`, abre la página y también devuelve "launched". |
| `moonpool_stop_app` | `app_id` (obligatorio) | Detiene la app. Devuelve "stopped", o un error como `still running after stop` (a los 15 s). |
| `moonpool_restart_app` | `app_id` (obligatorio) | Detiene, espera a que se liberen el puerto y el proceso, e inicia. Devuelve "restarted". |
| `moonpool_app_output` | `app_id` (obligatorio), `tail_lines` (entero, predeterminado 200, mínimo 1) | La salida de terminal de la app en la sesión actual de Moonpool, sin códigos ANSI. Cuando el registro es más largo que `tail_lines`, el texto empieza con una línea que indica la ruta del registro completo. Falla con `no console output recorded for '<id>' (not launched this session)` si la app no se ha ejecutado. Si el registro existe pero está vacío, devuelve `(no output recorded for '<id>')`. |
| `moonpool_stop_mcp_server` | `app_id` (obligatorio) | Termina el proceso ayudante MCP conectado de la app y deja la app en ejecución. Devuelve "stopped". No hace nada si la app no tiene ni `processName` ni `mcpProcessName`. |
| `moonpool_refresh_app_icons` | ninguno | Vuelve a obtener todos los iconos de las apps. Devuelve "icons refreshed". |

## Configuración

Estas herramientas leen y cambian `apps.json` a través del hub, nunca el archivo en disco. Una escritura
debe llevar el token de la última lectura, un token obsoleto se rechaza y el nuevo archivo se valida antes
de escribir nada. Pasar por el hub importa porque a un agente en un host aislado se le puede mostrar una
copia privada de la carpeta de configuración en lugar de la real.

| Herramienta | Parámetros | Comportamiento |
| --- | --- | --- |
| `moonpool_read_config` | ninguno | Texto JSON con `manifest_text` (el contenido exacto del archivo), `token`, `valid`, `error` (null si es válido) y `path`. `token` es `none` cuando el archivo falta o está vacío. |
| `moonpool_write_config` | `manifest` (obligatorio, el texto completo del nuevo `apps.json`), `expected_token` (obligatorio, de la última lectura) | Valida el manifiesto y sustituye `apps.json`, y luego lo carga. Devuelve `apps.json updated; new version token <token>`. Un token obsoleto falla con `stale token: apps.json changed since it was read ...`. Un manifiesto no válido falla con `rejected invalid manifest: ...`. En ambos casos el archivo queda intacto. Un `expected_token` vacío se rechaza. |
| `moonpool_restore_config` | `snapshot` (opcional) | Sin valor, texto JSON que lista las instantáneas guardadas de la más reciente a la más antigua (`index`, `filename`, `millis`, `app_count`, `valid`). Con un índice (1 = la más reciente) o un nombre de archivo, valida esa instantánea y la restaura. Devuelve `restored <file> (<n> apps); new version token <token>`. No hace falta token: una restauración sobrescribe el archivo actual a propósito. |
| `moonpool_reload_config` | ninguno | Vuelve a leer `apps.json`. Devuelve "apps.json reloaded". Si el archivo no se puede analizar o no es válido, falla con `apps.json has an error: ...` y Moonpool conserva la última lista que se cargó. |
| `moonpool_launcher_paths` | ninguno | Lista la carpeta de configuración del hub, `apps.json`, `state.json`, el registro, la carpeta de volcados, la carpeta de iconos, el indicador de portable y la ruta del exe, y luego la carpeta de configuración del proceso MCP, `apps.json`, `state.json`, la carpeta de volcados, el indicador de portable y la ruta del exe (sin registro ni iconos). Si el hub no está en ejecución, su mitad dice `hub paths unavailable: ...` y la mitad de MCP se muestra igualmente. Úsala cuando un cambio no surte efecto. |

## Avanzado: herramientas de prueba

`moonpool_screenshot` es exclusiva de Windows; en Linux falla con "screenshot is not supported on
this platform". `moonpool_window_state` y `moonpool_reset_mcp_seen` funcionan en todas las plataformas.

`window` es uno de `main`, `settings`, `about`, `installer`, `editor`, `help` o `themes`, y por defecto es
`main`. Un nombre desconocido falla con `unknown window '<name>'`.

| Herramienta | Parámetros | Comportamiento |
| --- | --- | --- |
| `moonpool_screenshot` | `window` (opcional) | Captura el contenido propio de esa ventana de Moonpool como un PNG en línea, de como máximo 320 píxeles en su lado más largo. El tamaño no se puede aumentar desde MCP. Falla con `window '<name>' is not open` si no se está mostrando. No puede capturar ninguna otra app. |
| `moonpool_window_state` | `window` (opcional) | Texto JSON: `{"open":false}` cuando la ventana no está abierta; si no, `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. Pensada para pruebas. |
| `moonpool_reset_mcp_seen` | `app_id` (opcional) | Solo para pruebas. Borra el registro recordado de "se ha visto un ayudante MCP" de una app, o de todas las apps si se omite, de modo que la subfila MCP de la barra lateral vuelve a ocultarse hasta que se vea un ayudante. |

## Véase también

- [Configuración de MCP](/es/automation/mcp-setup/)
- [Línea de comandos](/es/automation/command-line/)
