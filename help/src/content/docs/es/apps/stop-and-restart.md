---
title: "Detén un servidor de desarrollo y todo lo que inició"
description: "Haz que Detener y Reiniciar terminen una app y sus procesos hijo limpiamente con killMode y stopCommand, con los valores predeterminados por tipo y Docker en Windows."
---

Detener siempre hace primero esto: Moonpool termina la terminal que inició para la app, incluido todo lo
que esa terminal lanzó. Para muchas apps eso es todo lo que hace falta.

Algunas apps sobreviven a esa terminal (una ventana de escritorio se desacopla del servidor de desarrollo
que la lanzó, o un subproceso del servidor sigue ocupando su puerto). **`killMode`** elige un paso
adicional que se ejecuta después.

| `killMode` | Paso adicional al Detener | Lee | Predeterminado para |
| --- | --- | --- | --- |
| `processName` | Fuerza el cierre de todos los procesos con ese nombre. En Windows, también de sus hijos (`taskkill /IM <name>.exe /T /F`). En el resto, `pkill -KILL -x <name>`: una coincidencia exacta del nombre, distinguiendo mayúsculas, sin incluir a los hijos. | `processName` | `desktop` |
| `port` | Fuerza el cierre del proceso que esté escuchando en `port`. | `port` | `web` |
| `command` | Ejecuta `stopCommand` en `cwd` y espera a que termine. | `stopCommand`, `cwd`, `env` | ninguno |
| `none` | Nada. | nada | `static`, `cli` |

Omite `killMode` para obtener el valor predeterminado del tipo de la app, y defínelo solo cuando Detener
deje algo en ejecución.

![El selector killMode en el diálogo Editar app, establecido en "predeterminado (según el tipo)", con su línea de ayuda que enumera lo que hace cada tipo de forma predeterminada](../../../../assets/screenshots/edit-app-killmode.png)

1. El selector `killMode`. "predeterminado (según el tipo)" equivale a omitir la clave.

- Si el campo que necesita el modo está vacío (modo `port` sin `port`, por ejemplo), el paso adicional se
  omite. No es un error.
- `killMode` es independiente de `type`: `port` funciona en una app `cli` y `processName` en una app
  `web`.
- Una cadena vacía o un valor no reconocido no hace nada adicional. No recurre al valor predeterminado
  del tipo.

Para una app de escritorio, el modo `processName` ejecuta el equivalente de:

```powershell frame="terminal"
taskkill /IM notes-app.exe /T /F
```

## Varios Moonpool, o tus propios procesos

`processName` y `port` no saben quién inició un proceso. `processName` termina todos los procesos con ese
nombre, y `port` termina lo que esté escuchando en el puerto, incluido uno que haya iniciado otra copia
de Moonpool (la instalada y las copias portables se ejecutan de forma independiente; consulta
[Modo portable](/es/data/portable-mode/#varias-copias-a-la-vez)) y uno que hayas iniciado tú mismo.
Usa estos modos solo con apps que no choquen de ese modo: un nombre o un puerto que nada más en el equipo
use. Si dos copias registran la misma app, o también la ejecutas a mano, dale `killMode` `none` o un
`command` que detenga solo su propia instancia.

## stopCommand

Se usa solo cuando `killMode` es `command`. Se ejecuta mediante `cmd /c` en Windows y `$SHELL -c` en el
resto, en `cwd`, con tu `env` añadido. `{MP_HOME}` y `{MP_DATA}` funcionan en él. Moonpool espera a que
termine antes de hacer nada más, de modo que un Reiniciar nunca vuelve a iniciar la app mientras siga en
ejecución. Su código de salida se ignora. Si sigue en ejecución pasados 60 segundos, Moonpool lo termina
junto con sus hijos y continúa.

## Reiniciar

Reiniciar es Detener seguido de Iniciar con el mismo `command`. Moonpool espera hasta 4 segundos a que la
instancia anterior figure como detenida (para que su puerto esté libre) antes de volver a iniciarla. Una
entrada `static` con solo una `url` no tiene nada que detener: Reiniciar simplemente vuelve a abrir la
página.

## Apps de Docker en Windows

Usa `none`, o `command` con un comando de detención real como `docker compose stop app`. No uses `port`.

Docker Desktop publica el puerto de cada contenedor mediante un único proceso compartido en segundo plano.
En Windows, "lo que esté escuchando en el puerto" es ese proceso compartido, así que el modo `port`
forzaría el cierre de Docker Desktop y tumbaría todos los contenedores, no solo esta app. Como red de
seguridad, Moonpool se niega a terminar por puerto una lista fija de procesos compartidos de Windows: los
procesos de backend, proxy y servicio de Docker Desktop, `dockerd`, `vpnkit`, los procesos host de WSL y
procesos básicos del sistema como `svchost`. Eso no sustituye a elegir el modo correcto.

Si tu `command` ya recrea el contenedor (`docker compose up -d --build`), `none` es lo correcto:
Reiniciar simplemente lo ejecuta de nuevo.

Consulta también [Encontrar y terminar el proceso que usa un puerto](/es/guides/find-and-kill-process-using-port-windows/)
y [Solucionar EADDRINUSE y "Port 5173 is in use"](/es/support/port-already-in-use/).

## Ejemplos

Un servidor de desarrollo que a veces deja un proceso node ocupando su puerto (este es el valor
predeterminado de `web`, mostrado aquí de forma explícita):

```json title="apps.json"
{ "id": "site", "name": "Site", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
  "killMode": "port" }
```

Una app de Docker Compose:

```json title="apps.json"
{ "id": "api", "name": "API", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\api", "command": "docker compose up -d --build", "port": 8080,
  "killMode": "command", "stopCommand": "docker compose stop app" }
```
