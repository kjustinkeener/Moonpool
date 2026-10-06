---
title: "Elige un tipo de app: web, desktop, static o cli"
description: "Aprende cómo se inician las apps web, desktop, static y cli en Moonpool, cómo se detecta que están en ejecución y qué hace Detener de forma predeterminada."
---

`type` decide qué campos importan y qué hace Detener de forma predeterminada.

| | `web` | `desktop` | `static` | `cli` |
| --- | --- | --- | --- | --- |
| Necesita | `command` | `command` | `url` | `command` |
| Normalmente también | `port`, `url` | `processName` | `command` y `port`, si se sirve a sí misma | `cwd` |
| Iniciar | Ejecuta `command` en una pestaña de terminal | Ejecuta `command` en una pestaña de terminal | Sin `command`: abre `url` en el navegador. Con uno: lo ejecuta en una pestaña de terminal | Ejecuta `command` en una pestaña de terminal |
| `killMode` predeterminado | `port` | `processName` | `none` | `none` |

![El diálogo Editar app de una app web: type establecido en web con una descripción de una línea y un campo port rellenado](../../../../assets/screenshots/edit-app-type-and-port.png)

1. El selector `type`. Su línea de ayuda describe lo que hace ese tipo.
2. El campo `port`. En una app `web`, "en ejecución" depende de si este puerto responde.

## Cómo se decide si está en ejecución

Moonpool lo comprueba cada pocos segundos. Una app está en ejecución si se cumple cualquiera de estas
condiciones, sea cual sea su tipo:

- `processName` está definido y existe un proceso con ese nombre. Los procesos auxiliares `<exe> mcp` del
  propio Moonpool no se cuentan.
- `port` está definido y responde en localhost.
- Moonpool la inició, no tiene ni `port` ni `processName`, y el proceso de la terminal sigue vivo.

Así, una app `cli` está en ejecución mientras su comando se ejecuta, y una app `web` sin `port` se
comporta igual. Una entrada `static` con solo una `url` no tiene nada que vigilar y nunca aparece como en
ejecución.

## web

Un servidor local. Define `port` para que "en ejecución" refleje si el servidor responde, y `url` junto
con `openBrowser` para abrirlo cuando se active.

## desktop

Una app nativa. Define `processName` con el nombre del ejecutable para que "en ejecución" sobreviva a que
la ventana se desacople del comando que la inició. El Detener predeterminado termina todos los procesos
con ese nombre.

## static

Una página. Con solo una `url`, Iniciar y Reiniciar la abren en tu navegador y Detener no hace nada. Se
abren las URL `http://`, `https://`, `mailto:` y `file://`, así que una página local funciona:

```json title="apps.json"
{ "id": "csv", "name": "CSV dashboard", "group": "Docs", "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html" }
```

Las páginas que necesitan un servidor (PHP, o cualquiera que cargue archivos locales) necesitan un
`command` que inicie uno y un `port` para vigilarlo. Consulta los
[ejemplos](/es/apps/examples/).

## cli

Una herramienta. `command` se ejecuta en una pestaña de terminal en `cwd`, y la app deja de estar en
ejecución cuando el comando termina. Para un shell que siga abierto, haz que el comando sea un shell, por
ejemplo este `command`:

```text title="command"
pwsh -NoLogo -NoProfile -NoExit -Command python run.py --flag
```

Evita las comillas dobles anidadas en `command`: el envoltorio `cmd /c` las estropea.

![La pestaña de terminal de una app cli que muestra la salida de un comando de PowerShell y un indicador abierto debajo](../../../../assets/screenshots/terminal-cli-output.png)

## Qué hace el clic

Al hacer clic en el nombre de una app solo se abre su pestaña de terminal. Usa los controles Iniciar,
Detener y Reiniciar para ejecutarla. Consulta [Estados de una app](/es/support/glossary/#estados-de-una-app).
