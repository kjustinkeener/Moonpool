---
title: "Deja que un agente de IA configure y controle Moonpool: inicio rápido"
description: "Tres formas de que un agente de IA o un script configure y controle Moonpool, cuál elegir para tu agente y la misma acción mostrada en cada una."
---

Hay tres formas de entrar. Elige según lo que pueda hacer tu agente.

| Quieres | Usa | Empieza aquí |
| --- | --- | --- |
| Que un agente encuentre tus apps y las añada, una sola vez | **Copiar el prompt** en la pantalla vacía del hub | Más abajo |
| Que un agente inicie, detenga y lea apps como llamadas a herramientas | El servidor MCP, `moonpool.exe mcp` | [Configuración de MCP](/es/automation/mcp-setup/) |
| Un script, o un agente sin MCP | Verbos de la línea de comandos | [Línea de comandos](/es/automation/command-line/) |

## Copiar el prompt

Sin ninguna pestaña abierta, el panel CLI muestra un prompt ya preparado ("¿Es tu primera vez? Pásale esto
a un agente de IA para que configure tus apps"). **Copiar el prompt** lo pone en el portapapeles. Pégalo
en tu agente. Dirige al agente a `AI-README.md` y `apps.json` en tu carpeta de configuración y le pide que
encuentre tus apps y las registre. Cuando termine, elige **Recargar**.

Moonpool reescribe `AI-README.md` junto a `apps.json` en cada inicio, de modo que siempre coincide con la
versión que ejecutas. No conserves en él tus propias ediciones.

## La misma acción de tres formas

| Acción | Línea de comandos | Verbo del canal de control | Herramienta MCP |
| --- | --- | --- | --- |
| Iniciar una app | `moonpool.exe launch <id>` | `launch` | `moonpool_start_app` |
| Detener una app | `moonpool.exe stop <id>` | `stop` | `moonpool_stop_app` |
| Reiniciar una app | `moonpool.exe restart <id>` | `restart` | `moonpool_restart_app` |
| Leer la salida de una app | `moonpool.exe dump <id> [out-path]` | `dump` | `moonpool_app_output` |
| Listar apps y estado | leer `state.json` | `list` | `moonpool_list_apps` |
| Volver a leer `apps.json` | `moonpool.exe reload` | `reload` | `moonpool_reload_config` |
| Leer `apps.json` | `moonpool.exe read-config` | `read-config` | `moonpool_read_config` |
| Sustituir `apps.json` | `moonpool.exe write-config <file> [token]` | `write-config` | `moonpool_write_config` |
| Revertir `apps.json` | `moonpool.exe restore-config [n]` | `restore-config` | `moonpool_restore_config` |
| Mostrar la ventana | `moonpool.exe show` | `show` | `moonpool_raise_launcher` |
| Iniciar Moonpool | `moonpool.exe` | ninguno | `moonpool_bootup_launcher` |
| Salir de Moonpool | `moonpool.exe quit` | `quit` | `moonpool_shutdown_launcher` |
| Mostrar las carpetas en uso | `moonpool.exe paths` | `paths` | `moonpool_launcher_paths` |

La línea de comandos no imprime nada; lee el resultado con un `--ticket` (consulta
[Lectura del resultado](/es/automation/command-line/#lectura-del-resultado)). El canal y MCP responden
directamente.

## Cuando fallan las herramientas de un agente

- `Moonpool is not running - call moonpool_bootup_launcher first` (Moonpool no está en ejecución: llama
  primero a moonpool_bootup_launcher): inicia Moonpool, o deja que el agente llame a esa herramienta.
- Una edición "no se aplicó": pide al agente `moonpool_launcher_paths`. Si las carpetas del hub y de MCP
  difieren, el agente está leyendo otro `apps.json`. Consulta
  [Hosts aislados](/es/automation/mcp-setup/#hosts-aislados).
- Varias copias de Moonpool: registra cada una con su propio nombre. Consulta
  [Más de un Moonpool](/es/automation/mcp-setup/#más-de-un-moonpool).

Hay un ejemplo práctico para Claude Code, Codex y Cursor en
[Dar a un agente de IA un servidor MCP para iniciar y detener apps locales](/es/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/).

Más síntomas en [Solución de problemas](/es/support/troubleshooting/#errores-de-mcp-y-de-scripts).
