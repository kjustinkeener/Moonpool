---
title: "Encuentra y gestiona los registros de sesión y el registro de depuración de Moonpool"
description: "Localiza el registro de sesión de cada app, el registro de depuración, los volcados y el historial de desplazamiento, y consulta cuánto se conserva cada uno."
---

Moonpool conserva cuatro tipos de salida:

| Tipo | Dónde | Se conserva |
| --- | --- | --- |
| Registro de sesión | `cli-output\<id>\<session-start-ms>.log` en la carpeta de configuración | La sesión actual siempre; las sesiones anteriores según las reglas de retención de abajo |
| `moonpool.log` | La carpeta de configuración | Solo se escribe mientras **Registrar información de depuración en un archivo** está activado |
| Volcado | Donde lo indiques, o la propia ruta del registro de sesión | Hasta que lo elimines |
| Historial de desplazamiento | En la pestaña de terminal | 10.000 líneas, hasta que Moonpool se cierre |

La carpeta de configuración figura en [Dónde está la configuración](/es/apps/apps-json/#dónde-está-la-configuración).

## Registros de sesión

Todo lo que una app imprime en su terminal también se escribe en un archivo de registro:

```text
<config folder>\cli-output\<id>\<session-start-ms>.log
```

- Un archivo por app y por sesión de Moonpool. El número es el momento en que se inició ese proceso de Moonpool.
- Detener y volver a iniciar una app sigue añadiendo al mismo archivo. Una línea separadora atenuada marca
  dónde empieza cada nueva ejecución, y la misma marca aparece en la pestaña de terminal:

  ```text title="1767225600000.log"
  Local:   http://localhost:5173/
  ---------- restarted 2026-10-05 09:14:02 ----------
  Local:   http://localhost:5173/
  ```

- Los caracteres de un `id` distintos de letras, dígitos, `-` y `_` pasan a ser `_` en el nombre de la
  carpeta. Así, `.` pasa a ser `_`. Las letras que no son del inglés se conservan.
- El archivo contiene la salida bruta de la terminal, con los códigos de color. Usa un volcado para obtener texto plano.

Al reabrir la pestaña de una app se reproduce el registro de esta sesión, de modo que ves su salida anterior.

## Retención

La retención solo afecta a los registros de sesiones anteriores de Moonpool. Se ejecuta cuando inicias una
app, solo para la carpeta de esa app, empezando por los más antiguos.

| **Conservar los registros de salida de las apps entre sesiones** (`cliLogging`) | Qué ocurre con los registros de sesiones anteriores |
| --- | --- |
| desactivado (predeterminado) | Se eliminan en el siguiente inicio de la app. |
| activado | Se conservan hasta que el tamaño total de la carpeta supera **Retención de registros por app** (`logRetentionMb`, 10 MB de forma predeterminada); entonces se eliminan los más antiguos. |

El archivo de la sesión actual cuenta para ese total, pero nunca se elimina ni se recorta. Así, un único
registro actual muy grande puede desplazar a todos los anteriores.

![La sección de registro de Ajustes: la casilla de conservar registros, el tamaño de retención por app en MB y la casilla del registro de depuración, cada una con una fila de ruta de carpeta](../../../../assets/screenshots/settings-logging-section.png)

1. **Conservar los registros de salida de las apps entre sesiones** es `cliLogging`. **Retención de registros por app**, debajo, es `logRetentionMb`.

## moonpool.log

Con **Registrar información de depuración en un archivo** (`debugLogging`) activado, Moonpool añade líneas
con marca de tiempo a `moonpool.log` en la carpeta de configuración: cargas de `apps.json`, inicios (con
el comando y la carpeta), comandos de control y errores. Actívalo antes de reproducir un problema.

## Abrir y Copiar

En [Ajustes](/es/using/settings/#columna-derecha-registros), bajo cada grupo de registros:

- **Abrir la carpeta de registros CLI** y **Abrir el registro** abren la carpeta en tu administrador de archivos.
- **Copiar la ruta de la carpeta de registros CLI** y **Copiar la ruta del archivo de registro** ponen la ruta en el portapapeles.

En una pestaña de terminal, **Copiar todo** copia todo el historial de desplazamiento como texto.

## Volcados

El verbo `dump` te da un registro de sesión desde un script:

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" dump my-app C:\temp\my-app.log
```

Con una ruta de salida escribe una copia en texto plano, sin los códigos de color. Sin ella, informa de la
propia ruta del registro de sesión. Un agente obtiene el mismo texto, ya limpio, de
`moonpool_app_output`. Consulta [Línea de comandos](/es/automation/command-line/).
