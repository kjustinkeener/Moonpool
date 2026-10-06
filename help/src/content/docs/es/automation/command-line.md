---
title: "Controla Moonpool desde la línea de comandos"
description: "Controla un Moonpool en ejecución con los verbos de moonpool.exe desde una terminal o un script, etiqueta un comando con un ticket y lee el resultado en state.json."
---

Ejecutar `moonpool.exe` otra vez mientras ese mismo Moonpool ya está en ejecución no abre una segunda
ventana. El segundo proceso pasa sus argumentos al que está en ejecución a través de su
[canal de control](/es/automation/control-verbs/) y termina. Moonpool ya debe estar en ejecución: si no
hay ninguno residente, el mismo comando inicia un nuevo Moonpool y el verbo no se ejecuta.

"El mismo Moonpool" significa la misma carpeta. El Moonpool instalado y cada copia portable se ejecutan
por su cuenta, así que un comando llega a la copia cuyo `moonpool.exe` ejecutaste, nunca a otra. Consulta
[Modo portable](/es/data/portable-mode/#varias-copias-a-la-vez).

Usa la ruta de la copia que quieres. Para la instalada:

```powershell frame="terminal"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
```

Con varias copias en ejecución, `Get-Process moonpool` las lista todas, así que elige por `Path` en lugar
de tomar la primera. También lista los ayudantes `moonpool.exe mcp` inactivos que iniciaron los hosts MCP,
de modo que un proceso `moonpool` no demuestra que haya un hub en ejecución. Pregunta mejor al canal de
control con `ping` ([Verbos de control](/es/automation/control-verbs/)).

## Verbos

El verbo no distingue mayúsculas de minúsculas. `<id>` es el `id` de una app de `apps.json`.

| Comando | Efecto |
| --- | --- |
| `moonpool.exe` | Sin verbo: trae la ventana al frente. |
| `moonpool.exe show` | Trae la ventana al frente. |
| `moonpool.exe launch <id>` | Inicia la app y abre su pestaña de terminal. |
| `moonpool.exe stop <id>` | Detiene la app. |
| `moonpool.exe restart <id>` | Detiene, espera a que se liberen el puerto y el proceso, e inicia. |
| `moonpool.exe reload` | Vuelve a leer `apps.json`. |
| `moonpool.exe refresh-icons` | Vuelve a obtener todos los iconos. |
| `moonpool.exe help` | Abre la ventana de Ayuda. |
| `moonpool.exe quit` | Sale de Moonpool, igual que el menú de la bandeja. |
| `moonpool.exe dump <id> [out-path]` | Sin `out-path`, informa de la ruta del registro de la app en esta sesión. Con él, copia allí el registro como texto plano sin códigos ANSI. |
| `moonpool.exe paths` | Informa de la carpeta de configuración, `apps.json`, `state.json`, el registro, la carpeta de volcados, la carpeta de iconos, el indicador de portable y la ruta del exe que usa el Moonpool en ejecución. |
| `moonpool.exe read-config` | Escribe `dumps\read-config.json` en la carpeta de configuración, con `token`, `valid`, `error`, `path` y `manifest_text` (el contenido exacto de `apps.json`). |
| `moonpool.exe write-config <file> [token]` | Sustituye `apps.json` por el manifiesto de `<file>`, si el manifiesto es válido y, cuando se indica `token`, `apps.json` sigue coincidiendo con él. |
| `moonpool.exe restore-config [index or filename]` | Sin argumento, escribe la lista de instantáneas en `dumps\restore-config.json`. Con uno, restaura esa instantánea si es válida. |

```powershell frame="terminal"
& $mp restart my-app
```

```powershell frame="terminal"
& $mp dump my-app C:\temp\my-app.log
```

Un verbo desconocido se ignora. El programa también tiene argumentos de inicio propios:
`moonpool.exe mcp` ([Configuración de MCP](/es/automation/mcp-setup/)), `--uninstall` (lo usa Agregar o
quitar programas) y `--wait-pid <pid>` (se usa cuando Moonpool se reinicia a sí mismo). Solo se atienden
como primer argumento, de modo que un id de app como `--uninstall` no puede activarlos.

## Lectura del resultado

La línea de comandos no imprime nada, así que etiqueta un comando con `--ticket <key>` (cualquier clave
única, en cualquier posición) y lee el resultado en `state.json` en la carpeta de configuración. Es
`%USERPROFILE%\.moonpool\moonpool-config\` en una instalación, `<tu carpeta .moonpool>\moonpool-config\`
en una copia portable, y `~/.config/Moonpool/` en Linux (consulta
[Resumen de la configuración](/es/apps/apps-json/#dónde-está-la-configuración)). `show` y `quit` no escriben
ticket.

```powershell frame="terminal"
& $mp launch my-app --ticket t1
```

`state.json` contiene `apps`, `statuses` (`id`, `running`, `managed`, `mcpRunning`, `mcpSeen` por app) y
`tickets`. El Moonpool en ejecución lo reescribe cada pocos segundos y después de cada comando, y no lo
elimina al salir, así que un archivo sobrante no significa que Moonpool esté en ejecución. Para
preguntar si lo está, o para obtener la lista de apps en vivo, usa los verbos `ping` y `list` del canal de
control ([Verbos de control](/es/automation/control-verbs/)) o las herramientas MCP. Consulta tu ticket
hasta que `status` deje de ser `pending`:

| `status` | Significado |
| --- | --- |
| `pending` | Recibido; Moonpool todavía está actuando sobre él. |
| `ok` | Hecho. Para `dump`, `read-config`, `write-config`, `restore-config` y `paths`, `detail` contiene la ruta, el token o el informe. |
| `error` | Falló; `detail` dice por qué, por ejemplo `unknown app id: x`, `did not reach running in time`, `unknown command`. |

Cada ticket es `{ ticket, action, arg, status, detail, ts }` con `ts` en milisegundos Unix:

```json title="state.json (tickets entry)"
{
  "ticket": "t1",
  "action": "launch",
  "arg": "my-app",
  "status": "error",
  "detail": "did not reach running in time",
  "ts": 1767225600000
}
```

Los tickets terminados se descartan tras 24 horas, y la lista se recorta hacia las 50 entradas cuando los
tickets terminados tienen al menos 5 minutos.

Un agente compatible con MCP puede ahorrarse la consulta periódica: consulta
[Configuración de MCP](/es/automation/mcp-setup/).

## Véase también

- [Agentes de IA: inicio rápido](/es/automation/quick-start/)
- [Verbos de control](/es/automation/control-verbs/)
