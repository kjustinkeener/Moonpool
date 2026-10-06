---
title: "Iniciar un script o un servidor de desarrollo automáticamente al iniciar sesión en Windows"
description: "Inicia Moonpool al iniciar sesión en Windows con un acceso directo en la carpeta Inicio y lanza en él un servidor de desarrollo o un script con PowerShell."
---

Windows tiene dos formas habituales de iniciar algo al iniciar sesión: un acceso directo en tu carpeta
Inicio (pulsa Win+R, escribe `shell:startup` y pulsa Intro), o una tarea del Programador de tareas con un
desencadenador "Al iniciar sesión". Cualquiera de las dos ejecuta un programa o script, que podría ser
directamente el comando de tu servidor de desarrollo, pero entonces nada lo vigila, muestra su salida ni
lo detiene por ti.

## Qué ofrece Moonpool

Moonpool no tiene un ajuste para iniciarse al iniciar sesión, y una entrada de `apps.json` no tiene ningún
campo que la inicie cuando arranca Moonpool (la lista completa está en
[Campos de las apps](/es/apps/fields/) y [settings.json](/es/data/settings-json/)). Lo que sí puedes hacer
es iniciar Moonpool tú mismo al iniciar sesión y hacer que un script inicie las apps que quieras, con el
mismo verbo que ofrece la [línea de comandos](/es/automation/command-line/).

Primero registra la app como de costumbre:

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

Luego guarda esto como `start-moonpool-apps.ps1`. Instalado, el programa es
`%USERPROFILE%\.moonpool\moonpool.exe`; para una copia portable usa la ruta del exe de esa copia.

```powershell title="start-moonpool-apps.ps1"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
Start-Process $mp
Start-Sleep -Seconds 15
& $mp launch site
```

Moonpool ya debe estar en ejecución para que `launch` se le entregue; si no hay ninguno residente, el
mismo comando inicia un nuevo Moonpool y el verbo no se lleva a cabo. La espera le da tiempo para
arrancar, así que auméntala en un equipo lento. Añade una línea `& $mp launch <id>` por app.

Por último, coloca un acceso directo al script en la carpeta Inicio, con este destino:

```text title="Shortcut target"
powershell.exe -NoProfile -WindowStyle Hidden -File "C:\Users\you\start-moonpool-apps.ps1"
```

Para comprobar qué ha pasado, añade `--ticket t1` a un verbo y lee el resultado en `state.json`
([Lectura del resultado](/es/automation/command-line/#lectura-del-resultado)).

## Salvedades

- Un servidor de desarrollo iniciado así está "gestionado" por Moonpool como cualquier otro, de modo que
  Detener y Salir funcionan con él. Si la misma app ya está en ejecución (iniciada a mano, por ejemplo),
  Moonpool la muestra como en ejecución pero no gestionada.
- Moonpool no vuelve a iniciar una app que termina, y no recuerda qué apps estaban en ejecución cuando
  saliste por última vez.

## Véase también

- [Línea de comandos](/es/automation/command-line/)
- [Bandeja, cerrar y minimizar](/es/using/tray-and-closing/)
- [Ejecutar un servidor de desarrollo de npm en segundo plano en Windows](/es/guides/run-npm-dev-server-in-background-windows/)
