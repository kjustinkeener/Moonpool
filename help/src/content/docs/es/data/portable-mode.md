---
title: "Ejecuta Moonpool desde una memoria USB o una carpeta sincronizada"
description: "Mantén Moonpool y todos sus datos en una única carpeta movible para llevarlo en una memoria USB o sincronizarlo, y ejecuta varias copias en paralelo."
---

El modo portable guarda Moonpool y todo lo que escribe dentro de una única carpeta `.moonpool\`, de modo
que puedes llevarlo en una memoria USB o colocarlo en una carpeta sincronizada y ejecutarlo en cualquier
PC.

## Cómo funciona

Cuando instalas en modo portable, Moonpool crea una carpeta `.moonpool\` dentro de la ubicación que
elijas. Esa carpeta contiene el programa, tu configuración y su contenido de ayuda. No se escribe nada en
AppData de Windows, así que mover o copiar la carpeta traslada toda tu configuración.

```text
<chosen location>\.moonpool\
```

## En qué se diferencia de lo instalado

| | Instalado | Portable |
| --- | --- | --- |
| Programa | `%USERPROFILE%\.moonpool\moonpool.exe` | `<chosen location>\.moonpool\moonpool.exe` |
| Carpeta de configuración | `%USERPROFILE%\.moonpool\moonpool-config\` | `<chosen location>\.moonpool\moonpool-config\` |
| Perfil de navegador de la ventana, tamaño y posición de la ventana | En la carpeta de configuración | En la carpeta de configuración, así que también viajan |
| Menú Inicio, acceso directo en el escritorio, entrada de Agregar o quitar programas | Sí | Ninguno |
| Actualizaciones | Reemplaza su propio exe | Igual, dentro de la carpeta `.moonpool\`. Consulta [Actualización](/es/data/updating/#copias-portables). |
| Eliminar | Agregar o quitar programas o `--uninstall` | Borrar la carpeta |

Ninguno de los dos modos escribe en AppData de Windows.

### Carpetas sincronizadas

Puedes mantener una copia portable en una carpeta sincronizada (OneDrive, Dropbox y similares), pero
ejecútala en un solo PC a la vez. Moonpool escribe `state.json` cada pocos segundos y registra mientras
las apps se ejecutan, así que dos PC que ejecuten la misma carpeta se disputan los mismos archivos, y un
conflicto de sincronización puede dejar un `apps.json` dañado. Sal de Moonpool en un PC antes de iniciarlo
en otro.

## Varias copias a la vez

Se ejecuta un Moonpool por carpeta. El Moonpool instalado y cualquier número de copias portables, cada
una en su propia carpeta, pueden ejecutarse al mismo tiempo, y cada una es totalmente independiente: sus
propias apps, icono en la bandeja, ventana, ajustes, registros y [canal de control](/es/automation/control-verbs/).

- La información emergente de la bandeja y el nombre en la barra de tareas indican qué copia es cuál:
  `Moonpool` para la instalada y `Moonpool (<folder>)` para una portable, donde `<folder>` es la carpeta
  que elegiste (la que contiene `.moonpool\`).
- Iniciar la misma copia por segunda vez vuelve a mostrar su ventana en lugar de abrir otra. Iniciar una
  copia distinta abre esa copia.
- Para dar a un agente de IA más de una copia, registra cada una con su propio nombre; consulta
  [Configuración de MCP](/es/automation/mcp-setup/#más-de-un-moonpool).
- Mover o cambiar el nombre de una carpeta portable le da una identidad nueva (un nombre nuevo de canal de
  control). Sal de ella antes de moverla.
- Las copias no conocen las apps de las demás. Dos copias que inicien el mismo servidor en el mismo puerto
  seguirán chocando, y un Detener que funcione por nombre de proceso o por puerto puede terminar algo que
  haya iniciado otra copia; consulta
  [Detener y reiniciar](/es/apps/stop-and-restart/#varios-moonpool-o-tus-propios-procesos).

## Haz que tus apps también viajen

Usa el token `{MP_HOME}` en la ruta de una app para que apunte dentro de la carpeta portable en lugar de a
una ubicación fija de un solo equipo. En una copia portable, `{MP_HOME}` es la carpeta que contiene
`moonpool.exe`, que es la propia carpeta `.moonpool\`, no la carpeta que elegiste:

```json title="apps.json"
{ "cwd": "{MP_HOME}/my-app" }
```

Aquí `{MP_HOME}/my-app` es `<chosen location>\.moonpool\my-app`. Una ruta que empieza por `./` se ancla
de la misma forma. Los tokens y las rutas `./` también funcionan en un Moonpool instalado. Consulta
[Rutas y entorno](/es/apps/paths-and-environment/) para ver cómo se resuelven las rutas.

## Elegir portable desde el instalador

El modo portable se configura desde la tarjeta del instalador, que ofrece **Instalar portable** junto a
**Instalar Moonpool**.

![La tarjeta de instalación: el enlace Instalar portable está debajo del botón principal Instalar Moonpool](../../../../assets/screenshots/installer-window.png)

Elige una carpeta y Moonpool crea allí la carpeta `.moonpool\`, se copia dentro e inicia la nueva copia
con una configuración nueva.

La tarjeta también está en el menú "..." como **Instalar Moonpool…**, tanto en modo instalado como
portable. Usar **Instalar portable** desde ahí hace que el Moonpool en ejecución se cierre y que la nueva
copia portable se inicie en su lugar. El Moonpool desde el que empezaste se queda donde estaba, así que
puedes iniciarlo de nuevo después.

Una copia portable empieza desde cero y no copia tus apps actuales. Para traerlas, sal de la copia
portable y copia `apps.json` a mano:

| | Ruta |
| --- | --- |
| Origen (instalado) | `%USERPROFILE%\.moonpool\moonpool-config\apps.json` |
| Destino (portable) | `<chosen location>\.moonpool\moonpool-config\apps.json` |

Las entradas con rutas absolutas siguen funcionando en el mismo PC, pero no viajan. El diálogo Editar app
las marca como "no portable".

## Cómo sabe Moonpool que es portable

Una copia es portable mientras haya un archivo llamado `moonpool.portable` junto a su `moonpool.exe`. Nada
más la marca, y no se registra nada en Windows.

Para eliminar una copia portable, sal de ella y borra su carpeta `.moonpool\`. `--uninstall` solo elimina
el Moonpool instalado, nunca una copia portable.
