---
title: "Referencia del canal de control y los verbos de Moonpool"
description: "Cómo funciona el canal de control de Moonpool (canalización con nombre o socket Unix), su protocolo y los verbos a los que responde, con argumentos y respuestas."
---

## Dónde escucha

Cada copia de Moonpool tiene su propio canal, así que el Moonpool instalado y cualquier copia portable
pueden ejecutarse en paralelo sin responder unos por otros. En Windows, el Moonpool instalado escucha en
la canalización con nombre `\\.\pipe\moonpool`. Una copia portable añade un id construido a partir de su
carpeta: `\\.\pipe\moonpool-<id>`.

`<id>` son 8 dígitos hexadecimales derivados de la ruta de la carpeta `moonpool-config` de la copia, de
modo que se mantiene igual para esa carpeta entre reinicios y actualizaciones, y cambia si mueves la
carpeta. El `moonpool.exe` de una copia, incluido `moonpool.exe mcp`, siempre encuentra el canal de su
propia copia.

En Linux escucha en cambio en un socket de dominio Unix, con modo `0600`:

| Caso | Ruta del socket |
| --- | --- |
| Normal | `$XDG_RUNTIME_DIR/moonpool.sock` cuando esa variable está definida; si no, `moonpool.sock` en la carpeta de configuración de Moonpool |
| Modo portable | `moonpool.sock` en la carpeta de configuración de la copia portable, de modo que una copia portable nunca colisiona con una instalada |
| Ruta demasiado larga para un socket (unos 100 caracteres) | `/tmp/moonpool-<uid>/moonpool.sock`, en un directorio que solo tú puedes abrir (`moonpool-<id>.sock` para una copia portable) |

Un archivo de socket que queda tras un fallo se detecta y se reemplaza en el siguiente inicio. Un socket en
el que algo sigue respondiendo nunca se toma. El archivo se elimina cuando Moonpool sale con normalidad.

El canal es también la forma en que el [servidor MCP](/es/automation/mcp-setup/) sabe si Moonpool está en
ejecución: si se responde a un `ping`, lo está, y si falta la canalización o el socket, no lo está. Los
mismos verbos también son accesibles desde la [línea de comandos](/es/automation/command-line/), salvo los
verbos de diagnóstico de más abajo.

## Protocolo

Un objeto JSON por línea de entrada, una línea JSON de salida, en orden. Una conexión puede llevar muchas
solicitudes.

```json title="request"
{"cmd": "restart", "args": ["my-app"]}
```

```jsonl title="replies"
{"ok": true, "result": "..."}
{"ok": false, "error": "unknown app id: ..."}
```

Una solicitud y su respuesta desde PowerShell:

Para una copia portable, usa el nombre de su canalización (`moonpool-<id>`, que muestra el verbo `paths`)
en lugar de `moonpool`.

```powershell frame="terminal"
$p = New-Object System.IO.Pipes.NamedPipeClientStream('.', 'moonpool', 'InOut')
$p.Connect(2000)
$w = New-Object System.IO.StreamWriter($p); $w.AutoFlush = $true
$r = New-Object System.IO.StreamReader($p)
$w.WriteLine('{"cmd":"ping"}')
$r.ReadLine()
```

```json title="reply"
{"ok":true,"result":"pong"}
```

- `args` es una lista de cadenas y se puede omitir. Los demás campos se ignoran.
- `result` es una cadena o null. Los verbos que devuelven datos estructurados los devuelven como una
  cadena JSON.
- Una línea que no es JSON válido recibe `{"ok": false, "error": "bad request: ..."}`.
- Un `cmd` desconocido recibe `unknown cmd: <name>`.
- Un verbo que pasa por la ventana (`launch`, `stop`, `restart`, `reload`, `refresh-icons`, `help`,
  `open-window`) se responde cuando la acción termina, o con un error de tiempo de espera tras 45 s. Si la
  interfaz de la ventana del hub no se ha cargado, falla de inmediato con `frontend not loaded`.
- Un Moonpool que arranca mientras uno anterior todavía está saliendo reintenta enlazar el canal durante
  unos 8 segundos. Si aun así no puede, lo registra y sigue en ejecución sin él.

## Verbos

| Verbo | Args | Resultado |
| --- | --- | --- |
| `ping` | ninguno | `pong`. Solo canal. |
| `list` | ninguno | Cadena JSON `{"apps": [...], "statuses": [...]}` leída de la memoria del hub en ejecución, con la misma estructura de `apps` y `statuses` que `state.json`. Añade `"statusNotReady": true` cuando hay apps registradas pero todavía no se ha hecho la primera comprobación de estado. Mientras `apps.json` no se puede cargar, añade `"manifestError": "<message>"` (las apps son entonces la última lista que se cargó) y, cuando no se ha cargado ninguna lista desde el arranque, `"manifestLoaded": false`. Solo canal. |
| `show` | ninguno | null. Trae la ventana al frente. |
| `quit` | ninguno | null. Sale de Moonpool. |
| `launch` | `<id>` | null si tiene éxito, u `opened` para una entrada `static` con solo una `url`. Errores: `unknown app id: <id>`, `did not reach running in time`. |
| `stop` | `<id>` | null si tiene éxito, o `stopped` para una entrada `static` con solo una `url`. Error: `still running after stop`. |
| `restart` | `<id>` | Mismos resultados y errores que `launch`. |
| `reload` | ninguno | null si tiene éxito. |
| `refresh-icons` | ninguno | null si tiene éxito. |
| `help` | ninguno | null. Abre la ventana de Ayuda. |
| `dump` | `<id>` [`out-path`] | Ruta del registro de sesión de la app, o de la copia en texto plano en `out-path`. |
| `paths` | ninguno | Informe de varias líneas de las carpetas y el exe que usa el hub. |
| `read-config` | ninguno | Ruta de `dumps\read-config.json`, que contiene `token`, `valid`, `error`, `path`, `manifest_text`. |
| `write-config` | `<source-file>` [`token`] | El nuevo token de versión. Errores: `stale token: ...`, `rejected invalid manifest: ...`, `cannot read source ...`. |
| `restore-config` | [`index` o `filename`] | Sin argumento: ruta de `dumps\restore-config.json` (`count`, `snapshots`). Con uno: `restored <file> (<n> apps); new version token <token>`. |
| `argv` | los argumentos de la línea de comandos | null, de inmediato. Los ejecuta exactamente como lo haría un segundo `moonpool.exe <args>` de esta copia, incluido `--ticket`. Así es como ese segundo inicio entrega sus argumentos antes de terminar. |

Ejemplos de intercambios:

```jsonl title="request, reply"
{"cmd": "launch", "args": ["nope"]}
{"ok": false, "error": "unknown app id: nope"}

{"cmd": "restore-config", "args": ["1"]}
{"ok": true, "result": "restored 1767225600000.json (6 apps); new version token <token>"}
```

`write-config` y `restore-config` cargan el nuevo manifiesto de inmediato, registran una instantánea en
`apps.json.history\` y actualizan la ventana.

## Verbos de diagnóstico (pruebas)

Solo canal: la línea de comandos no los acepta. Todos funcionan en Windows y Linux excepto
`screenshot`, que es exclusivo de Windows y responde `screenshot is not supported on this
platform (Windows only)` en los demás.

| Verbo | Args | Resultado |
| --- | --- | --- |
| `screenshot` | [`window`] [`max_dim`] | Solo Windows. Base64 de un PNG de esa ventana de Moonpool (`main` por defecto). `max_dim` opcional limita el lado más largo en píxeles (se ajusta a 320-2400, 320 por defecto; la herramienta MCP siempre usa el valor predeterminado). Un `max_dim` no entero es un error. Ventanas permitidas: `main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`. Errores: `unknown window '<name>'`, `window '<name>' is not open`. No se escribe en disco. |
| `open-window` | `<kind>` [`<id>`] | null. Abre una ventana como lo hace su opción de menú. `kind`: `settings`, `about`, `installer`, `help`, `themes`, `editor` (un `<id>` opcional abre el diálogo Editar app de esa app; sin él abre Añadir app), `terminal` (`<id>` obligatorio: selecciona la pestaña de terminal de esa app y ensancha el hub para que se vea el panel CLI; no la inicia), `cli` (solo ensancha el hub). Errores: `unknown window kind '<kind>'`, `terminal needs an app id`, `unknown app id: <id>`. Se responde a través de la ventana del hub, como `launch`. |
| `window-state` | [`window`] | Cadena JSON: `{"open":false}`, o `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. |
| `stop-mcp` | `<id>` | `stopped`. Termina el ayudante `<processName> mcp` de la app, no la app. Errores: `missing app id`, `unknown app id: <id>`. |
| `reset-mcp-seen` | [`<id>`] | `<id>: cleared` o `<id>: was not marked seen`; sin id, `cleared <n> entries`. Borra los avistamientos recordados de ayudantes MCP. |

El `--ticket` de la línea de comandos y los registros de resultado de `state.json` pertenecen al otro
canal; consulta [Línea de comandos](/es/automation/command-line/#lectura-del-resultado). Las solicitudes del
canal reciben su respuesta en la propia respuesta.

## Véase también

- [Línea de comandos](/es/automation/command-line/)
- [Agentes de IA: inicio rápido](/es/automation/quick-start/#la-misma-acción-de-tres-formas)
