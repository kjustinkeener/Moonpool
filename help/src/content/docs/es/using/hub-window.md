---
title: "Conoce la ventana del hub de Moonpool"
description: "Un recorrido por el hub de Moonpool: barra lateral, pestañas de terminal, barra de estado, menú, aviso de error de apps.json y cómo recuerda su tamaño."
---

![El hub con tres apps en ejecución: dos filas de apps web en ejecución (1), la tira de pestañas (2), la salida en vivo de la app activa (3) y la barra de estado (4)](../../../../assets/screenshots/hub-window.png)

1. Dos de las apps en ejecución: un punto de estado encendido y un botón de detener en lugar del de reproducir.
2. La tira de pestañas, una pestaña por app abierta, con la pestaña activa resaltada.
3. La salida en vivo de la app activa.
4. La barra de estado de CPU y memoria.

## Diseño

| Zona | Qué contiene |
| --- | --- |
| Barra de título | Minimizar, maximizar y cerrar. |
| Barra lateral | El cuadro de filtro, el menú **...** y tus apps agrupadas por `group`. Consulta [Barra lateral y menús](/es/using/sidebar-and-menus/). |
| Panel CLI | Una pestaña de terminal por cada app abierta. Consulta [Pestañas de terminal](/es/using/terminal-tabs/). |
| Barra de estado | CPU y memoria en vivo, a lo largo de la parte inferior. |

Arrastra el divisor entre la barra lateral y el panel CLI para cambiar el ancho de la barra lateral.

## Barra de estado

![La barra de estado: barras de CPU por núcleo a la izquierda, la barra de memoria a la derecha](../../../../assets/screenshots/status-bar.png)

La barra de estado muestra una barra fina por cada núcleo de CPU (pasa el cursor para ver "Uso de CPU por núcleo") y luego una barra de memoria con una etiqueta `used/total GB`. Desactívala con **Mostrar la barra de estado de CPU y memoria** en Ajustes (`showStatusbar`; consulta [Ventana de Ajustes](/es/using/settings/)). El cambio se aplica de inmediato.

## El menú ...

El botón **...** a la izquierda del cuadro de filtro abre el menú.

![El botón de menú ... (1) y el cuadro Filtrar apps (2) en la parte superior de la barra lateral](../../../../assets/screenshots/sidebar-filter-and-menu.png)

1. El botón de menú **...**.
2. El cuadro **Filtrar apps...**.

| Elemento | Qué hace |
| --- | --- |
| Añadir app | Abre el editor de apps. Consulta [Añadir apps](/es/apps/add-an-app/). |
| Editar apps.json | Abre `apps.json` en tu editor predeterminado para editarlo a mano. |
| Recargar | Vuelve a leer `apps.json` del disco (también F5, consulta [Atajos y zoom](/es/using/keyboard-shortcuts/)). |
| Ajustes | Abre la ventana de Ajustes. |
| Ayuda | Abre esta ayuda. |
| Acerca de | Abre la ventana Acerca de, con la versión y la búsqueda de actualizaciones. |
| Instalar Moonpool… | Solo en Windows. Abre la ventana del instalador, para instalar la app o crear una copia portable. Consulta [Instalación](/es/getting-started/install/) y [Modo portable](/es/data/portable-mode/). |

### Aviso de conflicto de puertos

Si dos apps de `apps.json` usan el mismo `port`, aparece una fila de aviso al final del menú, por ejemplo:

```text
port 3000: App A / App B
```

Pasa el cursor por encima para ver la frase completa. Corrige el conflicto en `apps.json` o en el editor de apps; la fila desaparece cuando ya no se comparte ningún puerto.

## Cuando apps.json tiene un error

Si Recargar (o F5) detecta que `apps.json` ya no se puede analizar o no es válido, Moonpool conserva la
lista que ya tenía. Un aviso en la parte superior de la barra lateral dice "apps.json tiene un error; se
muestra la última lista que se cargó.", seguido del error (pasa el cursor para ver el texto completo). La
lista de debajo aparece atenuada pero sigue funcionando, así que puedes iniciar y detener apps como de
costumbre. **Editar apps.json** en el aviso abre el archivo; corrígelo y elige **Recargar**, y el aviso
desaparece.

Hasta que el archivo vuelva a cargarse, Moonpool no guardará cambios del editor de apps, de cambiar
nombre, de eliminar ni de elegir icono, de modo que un archivo dañado nunca se sobrescribe.

Si el archivo ya está dañado cuando Moonpool arranca, no hay una lista anterior que conservar: el aviso
dice que no hay apps cargadas y la barra lateral está vacía. Corrige el archivo y recarga, o vuelve a una
copia reciente que funcione (consulta [Si el archivo está dañado](/es/apps/apps-json/#si-el-archivo-está-dañado)).

## Pantalla vacía

Sin ninguna pestaña abierta, el panel CLI muestra "Elige una app a la izquierda para iniciarla." También
contiene dos elementos que aparecen solo mientras no hay ninguna pestaña abierta:

- **El aviso de actualización**, cuando se encontró una versión más reciente al arrancar. Consulta
  [Actualización](/es/data/updating/).
- **Copiar el prompt**, un prompt ya preparado que encarga a un agente de IA la configuración de tus apps.
  Consulta [Agentes de IA: inicio rápido](/es/automation/quick-start/#copiar-el-prompt).

El icono de la bandeja, cerrar, minimizar, Salir y siempre visible están en
[Bandeja, cerrar y minimizar](/es/using/tray-and-closing/).

## Tamaño, posición y estado maximizado

Moonpool recuerda el tamaño, la posición y el estado maximizado de la ventana del hub entre ejecuciones.
La primera ejecución se abre a 1200x780, en la posición que elige Windows.

Si la posición guardada ya no está en ninguna pantalla conectada (por ejemplo, un monitor desconectado),
la posición se ignora y se usa el tamaño guardado en la ubicación predeterminada. El archivo es
`window-state.json` en la carpeta de configuración (consulta
[Dónde está la configuración](/es/apps/apps-json/#dónde-está-la-configuración)).

También se recuerdan el ancho de la barra lateral y si el panel CLI está contraído.
