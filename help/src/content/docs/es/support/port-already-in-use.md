---
title: "Soluciona Error: listen EADDRINUSE: address already in use :::3000 y Vite Port 5173 is in use"
description: "Soluciona EADDRINUSE de Node y Port 5173 is in use de Vite: encuentra qué ocupa el puerto, libéralo y usa los campos port y killMode de Moonpool para que no se repita."
---

```text
Error: listen EADDRINUSE: address already in use :::3000
```

Este error de Node.js significa que otro proceso ya está escuchando en el puerto 3000 (los `:::` son la
forma IPv6 de "todas las direcciones"; también puedes ver `127.0.0.1:3000`). A menudo es una copia del
mismo servidor que iniciaste antes y nunca detuviste.

Vite trata la misma situación de otra manera. De forma predeterminada imprime:

```text
Port 5173 is in use, trying another one...
```

(el puerto 5173 está en uso, probando otro) y arranca en el siguiente puerto libre, así que el servidor
está en marcha, pero no donde esperas. Con `--strictPort` (o `server.strictPort: true`), Vite termina en
su lugar, con `Error: Port 5173 is already in use`.

## Arréglalo tú mismo

1. Encuentra el proceso que posee el puerto y termínalo. En Windows:

   ```text frame="terminal"
   netstat -ano | findstr :3000
   taskkill /PID 12345 /F
   ```

   Paso a paso, con la versión de PowerShell, en
   [Encontrar y terminar el proceso que usa un puerto](/es/guides/find-and-kill-process-using-port-windows/).
2. O inicia tu servidor en otro puerto, por ejemplo `PORT=3001` para muchos servidores Node o
   `--port 5174` para Vite.

## Cómo ayuda Moonpool

Si ejecutas el servidor con Moonpool, define `port` en su entrada. Entonces Moonpool:

- muestra la app como en ejecución mientras algo responda en ese puerto, de modo que un servidor sobrante
  que lo ocupa aparece como en ejecución pero no "gestionada por Moonpool";
- al **Detener** y **Reiniciar**, termina lo que siga escuchando en `port` cuando `killMode` es `port`,
  que es el valor predeterminado de las apps `web`, de modo que el siguiente Iniciar encuentra el puerto
  libre;
- señala dos apps configuradas con el mismo `port`.

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "node server.js",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

Moonpool no comprueba el puerto antes de iniciar. Si el puerto sigue ocupado, el comando imprime el error
anterior en la pestaña de terminal de la app. Pulsa **Detener** (que libera el puerto) e **Iniciar** de
nuevo.

Para Vite, pasa `--strictPort` y mantén `port` igual al puerto que pides:

```text title="command"
npm run dev -- --port 5173 --strictPort
```

Sin él, Vite puede pasar al 5174 mientras Moonpool sigue vigilando el 5173, y el punto de estado nunca se
queda fijo.

`killMode` `port` termina cualquier proceso del puerto, así que úsalo solo con puertos que nada más
necesite. Para apps de Docker en Windows, no lo uses nunca. Consulta
[Detener y reiniciar](/es/apps/stop-and-restart/#apps-de-docker-en-windows).

## Véase también

- [Campos de las apps](/es/apps/fields/): `port`, `killMode`.
- [Detener y reiniciar](/es/apps/stop-and-restart/)
- [Solución de problemas](/es/support/troubleshooting/#dos-apps-usan-el-mismo-puerto)
- [Ejecutar un servidor de desarrollo de npm en segundo plano en Windows](/es/guides/run-npm-dev-server-in-background-windows/)
