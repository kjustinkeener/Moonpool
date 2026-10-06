---
title: "Conecta un agente de IA a Moonpool mediante MCP"
description: "Registra moonpool.exe mcp como servidor MCP stdio en tu host, instalado o portable, y aprende cómo Moonpool vigila el ayudante MCP propio de una app."
---

El ejecutable de Moonpool es su propio servidor MCP. Regístralo en el host como un servidor stdio que
ejecuta `moonpool.exe` con el único argumento `mcp`.

## Registrar el servidor

Instalado, el programa es `%USERPROFILE%\.moonpool\moonpool.exe`. Portable, es el `moonpool.exe` dentro de
tu carpeta `.moonpool\`. Usa esa ruta completa como `command`. Para un host que lee un `.mcp.json`:

```json title=".mcp.json" {5}
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

En un archivo JSON, las barras invertidas deben duplicarse, como arriba. Un host con registro por línea de
comandos, como Claude Code, puede añadirlo en un solo paso:

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

El servidor se anuncia como `moonpool`, habla la revisión `2025-06-18` del protocolo MCP y expone solo
herramientas (no lista recursos ni prompts). Las herramientas aparecen ante el agente como `moonpool_*`;
consulta [Herramientas MCP](/es/automation/mcp-tools/).

## Más de un Moonpool

El Moonpool instalado y cada copia portable son lanzadores independientes, cada uno con sus propias apps,
y pueden ejecutarse todos a la vez. El `moonpool.exe mcp` de una copia siempre controla esa copia. Para que
un agente use varias, registra cada una con un nombre distinto, apuntando al exe de esa copia:

```json title=".mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    },
    "moonpool-work": {
      "type": "stdio",
      "command": "D:\\Work\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

```powershell frame="terminal"
claude mcp add moonpool-work -- "D:\Work\.moonpool\moonpool.exe" mcp
```

Registrar dos copias con el mismo nombre hace que una sustituya a la otra en la mayoría de los hosts. Los
nombres de las herramientas son los mismos para todas las copias, así que el host las distingue por el
nombre con el que las registras. Una copia portable también se anuncia como `moonpool (<folder>)` y las
instrucciones de su servidor nombran la carpeta, de modo que el agente puede ver con qué copia está
hablando.

## Notas

- `moonpool.exe mcp` nunca abre una ventana ni inicia el instalador. Termina cuando el host cierra su
  entrada.
- Usa la carpeta de configuración y el canal de control del exe desde el que se inició, así que un exe
  portable lee los datos de la carpeta portable y controla esa copia portable. Un exe cuenta como portable
  solo mientras `moonpool.portable` esté junto a él. Cualquier otro `moonpool.exe`, esté donde esté, usa
  la carpeta del Moonpool instalado (`%USERPROFILE%\.moonpool\moonpool-config\`) y controla el Moonpool
  instalado.
- La mayoría de las herramientas necesitan un Moonpool en ejecución. Si no lo está, el agente puede llamar
  primero a `moonpool_bootup_launcher`.
- `moonpool_launcher_paths` muestra las carpetas que usa el hub junto a las que resuelve el proceso MCP.
  Una diferencia significa que el agente está mirando un `apps.json` distinto del que usa el hub.

## Hosts aislados

Algunos hosts ejecutan sus herramientas dentro de un entorno aislado empaquetado (Store/MSIX) que
redirige AppData a una copia privada por paquete. Moonpool lo detecta cuando su carpeta de configuración o
su exe se resuelve bajo una ruta como `...\Packages\<package>\LocalCache\...`.

También lo detecta cuando el canal de control responde pero `state.json` no se puede leer. Las
herramientas que leen o escriben archivos (`moonpool_app_output`, `moonpool_read_config`,
`moonpool_write_config`, `moonpool_restore_config`) devuelven entonces un error que nombra la causa, en
lugar de datos vacíos u obsoletos. Las herramientas que solo usan el canal de control, como
`moonpool_list_apps`, no se bloquean mientras el canal sea accesible. Si el entorno aislado también oculta
el canal, las herramientas informan del entorno aislado en lugar de "Moonpool is not running" (Moonpool no
está en ejecución). Usa en su lugar la [línea de comandos](/es/automation/command-line/) desde un shell
fuera del entorno aislado.

## Apps que tienen su propio servidor MCP

A muchas apps de Moonpool se llega desde un host MCP mediante un proceso ayudante `<exe> mcp`. Moonpool
busca un proceso cuyo nombre coincida con el `processName` de la app y cuyo primer argumento sea `mcp`,
como `notes-app.exe mcp`. Si el servidor se ejecuta con otro nombre, como una copia renombrada, define el
comodín `mcpProcessName` de la app (consulta [Campos](/es/apps/fields/#mcpprocessname)); un proceso que
coincide con él cuenta sin el argumento `mcp`.

- Mientras hay uno conectado, la barra lateral de la app muestra una subfila MCP como en ejecución, y
  `moonpool_list_apps` añade `[mcp: running]` a la línea de la app. El ayudante no cuenta como la propia
  app en ejecución.
- Una vez que se ha visto un ayudante, Moonpool lo recuerda (en `mcp_seen.json` en la carpeta de
  configuración), de modo que la subfila MCP sigue visible como detenida, y `moonpool_list_apps` muestra
  `[mcp: stopped]`, después de que el ayudante termine.
- La subfila MCP se controla con el ajuste `showMcpProcesses`
  ([Ventana de Ajustes](/es/using/settings/)).
- `moonpool_stop_mcp_server` termina el ayudante y deja la app tranquila. No existe la operación contraria
  de iniciarlo: el host que posee el ayudante lo vuelve a iniciar en su siguiente llamada a una herramienta.

## Si las herramientas no funcionan

- **El host no muestra ninguna herramienta `moonpool_*`.** Comprueba que `command` sea la ruta completa a
  `moonpool.exe` y que `args` sea `["mcp"]`, y reinicia el host.
- **Todas las herramientas dicen que Moonpool no está en ejecución.** Inicia Moonpool, o llama a
  `moonpool_bootup_launcher`. Asegúrate de que el exe registrado sea la copia que estás ejecutando.
- **Un cambio no aparece.** Llama a `moonpool_launcher_paths` y compara las carpetas del hub con las del
  proceso MCP. Consulta [Hosts aislados](#hosts-aislados).

Más en [Solución de problemas](/es/support/troubleshooting/#errores-de-mcp-y-de-scripts).

## Véase también

- [Dar a un agente de IA (Claude Code, Codex, Cursor) un servidor MCP para iniciar y detener apps locales](/es/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/)
- [Herramientas MCP](/es/automation/mcp-tools/)
- [Agentes de IA: inicio rápido](/es/automation/quick-start/)
