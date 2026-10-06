---
title: "Mantener un script de Python en ejecución en segundo plano en Windows"
description: "Ejecuta en segundo plano en Windows un script de Python de larga duración o una pequeña app web, mira su salida y deténlo limpiamente, con pythonw y con Moonpool."
---

Un script de Python ejecutado desde una ventana de consola se detiene cuando cierras esa ventana. Las
soluciones habituales en Windows son `pythonw.exe` (el mismo intérprete sin ventana de consola, de modo
que la salida no va a ninguna parte), `Start-Process pythonw -ArgumentList worker.py` para iniciarlo
desacoplado, o una tarea programada para algo que deba ejecutarse al iniciar sesión o con un
temporizador. En todos los casos te toca buscar el proceso en el Administrador de tareas cuando quieres
que desaparezca.

## La forma de Moonpool

Moonpool ejecuta el comando en su propia pestaña de terminal, así que conservas la salida y un botón
Detener sin tener una ventana de consola propia. Para un script que se ejecuta hasta que lo detienes,
usa una app `cli`. `-u` hace que Python vacíe la salida de inmediato para que la pestaña la muestre en
vivo:

```json title="apps.json"
{
  "id": "worker",
  "name": "Queue worker",
  "group": "Scripts",
  "type": "cli",
  "cwd": "C:\\code\\worker",
  "command": ".venv\\Scripts\\python.exe -u worker.py"
}
```

Inicia la app y haz clic en su nombre para ver su salida. Una app `cli` cuenta como en ejecución
mientras su comando se ejecuta, y se pone gris cuando el script termina, con `[process exited]`
(proceso finalizado) en la pestaña. **Detener** termina el script y todo lo que haya iniciado. Usar
el `python.exe` del entorno virtual por su ruta significa que no hace falta ningún paso de activación.

Si el script sirve HTTP (Flask, FastAPI, `python -m http.server`), conviértelo en una app `web` para
que en ejecución dependa de su puerto:

```json title="apps.json"
{
  "id": "docs-api",
  "name": "Docs API",
  "group": "Scripts",
  "type": "web",
  "cwd": "C:\\code\\docs-api",
  "command": ".venv\\Scripts\\python.exe -u app.py",
  "port": 8091,
  "url": "http://127.0.0.1:8091",
  "env": { "PORT": "8091" }
}
```

## Límites

- Mantén Moonpool en ejecución. Al cerrar su ventana se sale de él de forma predeterminada, y en Windows
  salir detiene todas las apps que inició. Activa **Cerrar a la bandeja** para ocultar la ventana en su
  lugar; consulta [Bandeja, cerrar y minimizar](/es/using/tray-and-closing/).
- Moonpool no reinicia un script que falla, ni lo inicia por sí solo al iniciar sesión en Windows.
  Consulta [Iniciar un script o un servidor de desarrollo automáticamente al iniciar sesión en Windows](/es/guides/start-app-at-windows-login/).
- Evita las comillas dobles anidadas en `command`: el envoltorio `cmd /c` las estropea.

## Véase también

- [Tipos de app](/es/apps/types/#cli): cómo se vigilan las apps `cli` y `web`.
- [Detener y reiniciar](/es/apps/stop-and-restart/)
- [Ejemplos](/es/apps/examples/)
- [Registros](/es/data/logs/): dónde se conserva la salida de las sesiones.
