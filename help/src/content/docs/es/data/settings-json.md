---
title: "Entiende settings.json y repara uno dañado"
description: "Consulta la estructura del settings.json de Moonpool, qué claves escribe Moonpool por ti y cómo leerlo y repararlo cuando el archivo está dañado."
---

Los ajustes de toda la app están en `settings.json` en la carpeta de configuración (consulta
[Dónde está la configuración](/es/apps/apps-json/#dónde-está-la-configuración)). Cámbialos en la
[ventana de Ajustes](/es/using/settings/), que lista cada ajuste con su clave JSON y su valor
predeterminado. Los registros y su retención están en la página [Registros](/es/data/logs/).

## Estructura

Un objeto JSON. Las claves que omites toman sus valores predeterminados:

```json title="settings.json"
{
  "closeToTray": true,
  "transparency": 20,
  "debugLogging": true,
  "cliLogging": true,
  "logRetentionMb": 25
}
```

| Clave | Predeterminado |
| --- | --- |
| `closeToTray` | `false` |
| `minimizeToTray` | `true` |
| `showInTray` | `true` |
| `showInTaskbar` | `true` |
| `alwaysOnTop` | `false` |
| `transparency` | `0` (de 0 a 90) |
| `showStatusbar` | `true` |
| `showMcpProcesses` | `true` |
| `checkOnStartup` | `true` |
| `locale` | `"auto"` |
| `debugLogging` | `false` |
| `cliLogging` | `false` |
| `logRetentionMb` | `10` (mínimo 1) |

## Claves que se escriben por ti

Moonpool también guarda en este archivo el zoom de la interfaz (`uiScale`, de 0,5 a 3,0) y el idioma
resuelto (`localeResolved`). No necesitas definir ninguno de los dos. El tema no está aquí: se guarda en el
almacenamiento del webview (consulta [Temas, idioma y transparencia](/es/using/themes-and-language/)).

## Lectura y reparación

Moonpool lee el archivo al arrancar. Las ediciones hechas mientras se ejecuta no se aplican; sal primero.

Si el archivo está mal formado, Moonpool arranca con los valores predeterminados y se niega a cambiar
ajustes. El error termina con `Repair settings.json and restart Moonpool before changing settings`
(repara settings.json y reinicia Moonpool antes de cambiar ajustes). Corrige el archivo, o elimínalo para
restablecer todos los ajustes, y vuelve a iniciar Moonpool.
