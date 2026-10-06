---
title: "Ejecutar un servidor de desarrollo de npm en segundo plano en Windows sin ventana de terminal"
description: "Mantén npm run dev, Vite u otro servidor de desarrollo en ejecución en Windows sin ventana de consola, e inícialo, deténlo y lee su salida desde la bandeja."
---

Un servidor de desarrollo iniciado con `npm run dev` se ejecuta en la terminal que lo lanzó, así que
cerrar esa ventana lo termina. La forma sencilla de Windows para mantenerlo en marcha es un proceso
oculto, por ejemplo `Start-Process npm.cmd -ArgumentList "run","dev" -WindowStyle Hidden` en PowerShell,
pero entonces no tienes salida que leer y detenerlo significa buscar el `node.exe` correcto (consulta
[Encontrar y terminar el proceso que usa un puerto](/es/guides/find-and-kill-process-using-port-windows/)).

## La forma de Moonpool

Moonpool ejecuta el comando en su propia pestaña de terminal integrada dentro de la ventana del hub, así
que no hay una ventana de consola aparte que mantener abierta. Oculta el hub en la bandeja y el servidor
sigue en ejecución. Añade la app una sola vez:

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173",
  "openBrowser": true
}
```

Haz clic en el control **Iniciar** de la app. El punto de estado se queda fijo cuando `port` responde, y
el navegador se abre en `url` gracias a `openBrowser`. Haz clic en el nombre de la app para leer su salida
en su propia pestaña. **Detener** termina la terminal y todo lo que inició, y libera el puerto
(`killMode` `port` es el valor predeterminado de `web`).

## Mantenlo en ejecución al cerrar la ventana

De forma predeterminada, el botón de cerrar sale de Moonpool, y en Windows salir detiene todas las apps
que inició. Activa **Cerrar a la bandeja** en [Ajustes](/es/using/settings/) y al cerrar la ventana solo
se oculta. El icono de la bandeja (o **Mostrar Moonpool**) la vuelve a mostrar. Los detalles están en
[Bandeja, cerrar y minimizar](/es/using/tray-and-closing/).

## Mantén el puerto predecible

Moonpool decide si está en ejecución a partir de `port`. Vite pasa al siguiente puerto libre cuando el
suyo está ocupado, lo que dejaría a Moonpool vigilando el puerto equivocado. Pasa `--strictPort` para que
Vite termine en su lugar, y establece `port` con el mismo valor:

```text title="command"
npm run dev -- --port 5173 --strictPort
```

Si el puerto ya está ocupado, consulta
[Solucionar EADDRINUSE y "Port 5173 is in use"](/es/support/port-already-in-use/).

## Límites

- Moonpool no reinicia un servidor que falla. Muestra la app como detenida y la pestaña imprime
  `[process exited]` (proceso finalizado).
- Moonpool no se inicia por sí solo al iniciar sesión en Windows. Consulta
  [Iniciar un script o un servidor de desarrollo automáticamente al iniciar sesión en Windows](/es/guides/start-app-at-windows-login/).

## Véase también

- [Campos de las apps](/es/apps/fields/): `port`, `openBrowser`, `killMode`.
- [Tipos de app](/es/apps/types/): cómo se decide si `web` está en ejecución.
- [Detener y reiniciar](/es/apps/stop-and-restart/)
- [Ejemplos](/es/apps/examples/#servidor-de-desarrollo-web)
