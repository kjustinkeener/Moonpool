---
title: "Encontrar y terminar el proceso que usa un puerto en Windows (3000, 5173, 8080)"
description: "Averigua qué proceso ocupa el puerto 3000 o 5173 en Windows con netstat o PowerShell, termínalo con taskkill y deja que Moonpool libere el puerto al detener una app."
---

Cuando un servidor de desarrollo falla porque el puerto ya está en uso, hay otra cosa escuchando en ese
puerto. En el Símbolo del sistema, lista los procesos que escuchan junto con el ID del proceso
propietario y luego termínalo:

```text frame="terminal"
netstat -ano | findstr :3000
taskkill /PID 12345 /F
```

La última columna de la fila `LISTENING` es el PID (`findstr :3000` también coincide con `:30001`, así
que lee la dirección local). `tasklist /FI "PID eq 12345"` muestra de qué programa se trata. En
PowerShell, la misma búsqueda es:

```powershell frame="terminal"
Get-NetTCPConnection -LocalPort 3000 -State Listen | Select-Object LocalPort, OwningProcess
Get-Process -Id 12345
Stop-Process -Id 12345 -Force
```

Añade `/T` a `taskkill` para terminar también los procesos hijo. Los procesos que pertenecen a otro
usuario o al sistema pueden requerir una ventana con privilegios elevados (administrador).

## La forma de Moonpool

Para una app que ejecutas con Moonpool no tienes que buscar el PID. Dale a la app un `port` y Detener lo
libera. En una app `web` esto es el `killMode` predeterminado, escrito aquí de forma explícita:

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

Detener primero termina la terminal que inició Moonpool y después fuerza el cierre de lo que siga
escuchando en `port`. En Windows es la misma búsqueda de arriba (`Get-NetTCPConnection -LocalPort
<port> -State Listen`), seguida de `taskkill /PID <pid> /T /F` para cada propietario.

- Si algo que no iniciaste tú ocupa el puerto, Moonpool muestra la app como en ejecución pero no
  "gestionada por Moonpool". Pulsa **Detener** en ella: el paso de `port` se ejecuta igualmente.
- Moonpool se niega a terminar por puerto una lista fija de procesos compartidos de Windows, como el
  backend de Docker Desktop, `svchost` y el host de WSL. Para una app de Docker usa `killMode`
  `command` o `none`, nunca `port`. Consulta
  [Apps de Docker en Windows](/es/apps/stop-and-restart/#apps-de-docker-en-windows).
- Esto solo funciona con los puertos de las apps incluidas en `apps.json`. Para cualquier otro puerto,
  usa los comandos del principio.
- El modo `port` termina todo lo que escuche, incluida una copia que hayas iniciado a mano, así que
  úsalo solo con puertos que ninguna otra cosa del equipo necesite.

## Véase también

- [Solucionar EADDRINUSE y "Port 5173 is in use"](/es/support/port-already-in-use/)
- [Detener y reiniciar](/es/apps/stop-and-restart/)
- [Campos de las apps](/es/apps/fields/): `port` y `killMode`.
- [Dos apps usan el mismo puerto](/es/support/troubleshooting/#dos-apps-usan-el-mismo-puerto)
