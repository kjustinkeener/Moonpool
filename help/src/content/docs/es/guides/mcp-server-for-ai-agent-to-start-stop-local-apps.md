---
title: "Dar a un agente de IA (Claude Code, Codex, Cursor) un servidor MCP para iniciar y detener apps locales"
description: "Registra Moonpool como servidor MCP para que Claude Code, Codex o Cursor inicien, detengan y reinicien tus servidores de desarrollo y lean su salida."
---

Un agente de IA para programar suele ejecutar tu servidor de desarrollo escribiendo `npm run dev` en su
propio shell. Eso puede bloquear al agente, dejar un proceso huérfano ocupando el puerto o iniciar una
segunda copia de algo que ya tienes en ejecución. Un servidor MCP permite que el agente llame a
herramientas para iniciar y detener la app que ya has configurado, en lugar de reconstruir su línea de
comandos.

## La forma de Moonpool

El ejecutable de Moonpool es su propio servidor MCP: registra `moonpool.exe` con el único argumento
`mcp` como servidor stdio. Cuando la app está en `apps.json`, el agente la inicia por su id.

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

Registra el servidor. En Claude Code, con un solo comando (Moonpool instalado; usa la ruta completa de tu
propio exe):

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

Los hosts que leen un archivo JSON de servidores MCP, como el `mcp.json` de Cursor, aceptan la misma
estructura (con las barras invertidas duplicadas):

```json title="mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

Para Codex, añade en su configuración (`~/.codex/config.toml`) un servidor con el mismo comando y el
argumento `mcp`:

```toml title="config.toml"
[mcp_servers.moonpool]
command = 'C:\Users\you\.moonpool\moonpool.exe'
args = ["mcp"]
```

El archivo y los nombres de las claves exactos dependen de cada host, así que consulta su documentación
de MCP si tu versión difiere. Lo único que Moonpool necesita es la ruta completa a `moonpool.exe` y
`mcp` como argumento. Reinicia el host después.

## Qué puede hacer el agente

Las herramientas aparecen como `moonpool_*`. Las del trabajo diario:

| Herramienta | Uso |
| --- | --- |
| `moonpool_list_apps` | Encontrar el id de una app y ver si está en ejecución. |
| `moonpool_start_app` | Iniciar una app por su id y abrir su pestaña de terminal. |
| `moonpool_stop_app` | Detenerla, incluidos sus procesos hijo. |
| `moonpool_restart_app` | Detener, esperar a que se libere el puerto e iniciar. Úsala tras un cambio de código. |
| `moonpool_app_output` | Leer lo que imprimió la app, con `tail_lines` para limitarlo. |
| `moonpool_bootup_launcher` | Iniciar el propio Moonpool si no está en ejecución. |

Un ciclo típico es `moonpool_restart_app` y luego `moonpool_app_output`. El resto de las herramientas
(leer y escribir `apps.json`, capturas de pantalla) están en [Herramientas MCP](/es/automation/mcp-tools/).

## Si no funciona

Si todas las herramientas responden `Moonpool is not running - call moonpool_bootup_launcher first`
(Moonpool no está en ejecución: llama primero a moonpool_bootup_launcher), es que Moonpool todavía no se
ha iniciado. Un cambio que no aparece suele significar que el agente está mirando otro `apps.json`:
llama a `moonpool_launcher_paths`. Consulta
[Si las herramientas no funcionan](/es/automation/mcp-setup/#si-las-herramientas-no-funcionan).

## Véase también

- [Configuración de MCP](/es/automation/mcp-setup/)
- [Herramientas MCP](/es/automation/mcp-tools/)
- [Agentes de IA: inicio rápido](/es/automation/quick-start/)
- [Ejecutar un servidor de desarrollo de npm en segundo plano en Windows](/es/guides/run-npm-dev-server-in-background-windows/)
