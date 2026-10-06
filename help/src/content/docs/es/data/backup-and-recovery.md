---
title: "Haz copias de seguridad de Moonpool, revierte apps.json y recupera una configuración"
description: "Qué respaldar, cómo revertir un apps.json dañado, volver a las apps de ejemplo, trasladar una configuración a una copia portable y qué elimina la desinstalación."
---

Todo lo que Moonpool guarda está en dos lugares: la carpeta de configuración y la carpeta de paneles.
Las rutas de cada modo están en [Dónde está la configuración](/es/apps/apps-json/#dónde-está-la-configuración).

## La carpeta de configuración

```text
moonpool-config\
  apps.json            tus apps                                respaldar
  apps.json.history\   los últimos 10 apps.json correctos      respaldar (opcional)
  settings.json        ajustes de la app                       respaldar
  icons\               iconos personalizados, <id>.png, etc.   respaldar
  cli-output\<id>\     registros de sesión                     desechable
  moonpool.log         registro de depuración                  desechable
  state.json           instantánea del estado en vivo          desechable
  dumps\               archivos de dump y read-config          desechable
  mcp_seen.json        apps que tuvieron un ayudante MCP       desechable
  window-state.json    tamaño y posición de la ventana         desechable
  AI-README.md         se reescribe en cada inicio             desechable
  webview\             perfil de navegador (Windows)           desechable
```

La carpeta de paneles es `{MP_HOME}\dashboards`: `%USERPROFILE%\.moonpool\dashboards` en una instalación,
`<tu carpeta .moonpool>\dashboards` en modo portable, y `dashboards/` dentro de la carpeta de
configuración en Linux. Respalda todo lo tuyo que haya en ella. Su carpeta `examples` pertenece a
Moonpool y se reescribe al actualizar.

El tema se guarda en el almacenamiento del navegador de la ventana, no en un archivo que puedas copiar. No
viaja con una copia de seguridad; vuelve a elegirlo tras restaurar.

## Hacer una copia de seguridad

1. Sal de Moonpool, para que ningún archivo quede a medio escribir.
2. Copia `apps.json`, `settings.json` e `icons\` de la carpeta de configuración, y tus propios archivos
   de `dashboards\`.

```powershell frame="terminal"
$cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
Copy-Item "$cfg\apps.json", "$cfg\settings.json" D:\backup\
Copy-Item "$cfg\icons" D:\backup\ -Recurse
```

Para restaurar, sal de Moonpool, copia los archivos de vuelta e inícialo.

## Revertir apps.json

Cada guardado correcto, escritura de un agente y restauración, y cada Recargar que encuentra contenido
modificado, copia el `apps.json` validado en `apps.json.history\`, conservando los 10 más recientes. Cada
archivo se llama según el momento en que se tomó, por ejemplo `1767225600000.json`. No existe
`apps.json.bak`.

- **A mano.** Copia una instantánea sobre `apps.json` y luego elige **Recargar**.

  ```powershell frame="terminal"
  $cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
  Copy-Item "$cfg\apps.json.history\<snapshot>" "$cfg\apps.json"
  ```

- **Desde un script.** `moonpool.exe restore-config` lista las instantáneas;
  `moonpool.exe restore-config 1` restaura la más reciente. Consulta
  [Línea de comandos](/es/automation/command-line/).
- **Desde un agente.** `moonpool_restore_config`. Consulta [Herramientas MCP](/es/automation/mcp-tools/#configuración).

No se restaura nada automáticamente.

## Un archivo dañado

- **apps.json.** Moonpool nunca sobrescribe un archivo dañado. Consulta
  [Si el archivo está dañado](/es/apps/apps-json/#si-el-archivo-está-dañado).
- **settings.json.** Corrígelo, o elimínalo para restablecer todos los ajustes, y luego reinicia Moonpool.
  Consulta [settings.json](/es/data/settings-json/#lectura-y-reparación).

## Volver a los ejemplos

Moonpool escribe sus apps de ejemplo solo cuando no hay `apps.json`. Para empezar de nuevo, sal de
Moonpool (o déjalo en ejecución), cambia el nombre de `apps.json` o elimínalo, y luego inicia Moonpool o
elige **Recargar**. Se escribe un `apps.json` nuevo con los ejemplos.

## De instalado a portable

Una copia portable nueva empieza con las apps de ejemplo. Para traer las tuyas, consulta
[Modo portable](/es/data/portable-mode/#elegir-portable-desde-el-instalador). Copia `icons\` y
`settings.json` de la misma forma si quieres conservarlos.

## Desinstalación

Desinstalar el Moonpool instalado elimina toda la carpeta `%USERPROFILE%\.moonpool`, incluidas la carpeta
de configuración y los paneles. Haz antes una copia de seguridad. Consulta
[Desinstalación](/es/getting-started/install/#desinstalación). Una copia portable se elimina borrando su
carpeta `.moonpool\`.
