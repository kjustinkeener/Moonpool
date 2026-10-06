---
title: "Mantén Moonpool en la bandeja: comportamiento de cerrar, minimizar y salir"
description: "Controla qué hacen el icono de la bandeja, cerrar, minimizar y Salir, mantén la ventana siempre visible y evita ocultar a la vez la bandeja y la barra de tareas."
---

## Icono de la bandeja

| Acción | Resultado |
| --- | --- |
| Clic izquierdo | Muestra la ventana del hub (la restaura si estaba minimizada u oculta). |
| Clic derecho | Menú solo con **Mostrar Moonpool** y **Salir** (en tu idioma). |

Si hay varias copias de Moonpool en ejecución, cada una tiene su propio icono en la bandeja. La información
emergente indica de qué copia se trata. Consulta [Modo portable](/es/data/portable-mode/#varias-copias-a-la-vez).

## Salir

**Salir** cierra Moonpool y, en Windows, detiene todas las apps que Moonpool inició, incluidos sus
procesos hijo. Las apps que ya estaban en ejecución antes de que Moonpool las viera (se muestran como en
ejecución sin "gestionada por Moonpool") no se tocan. En Linux y macOS, salir no detiene de forma fiable
las apps iniciadas.

## Cerrar y minimizar

El botón de cerrar sale de Moonpool de forma predeterminada (`closeToTray` es `false`). Activa **Cerrar a
la bandeja** en Ajustes y al cerrar se oculta la ventana en la bandeja. Moonpool sigue en ejecución, y el
icono de la bandeja o **Mostrar Moonpool** la vuelve a mostrar.

**Minimizar a la bandeja** (`minimizeToTray`, activado de forma predeterminada) oculta la ventana en la
bandeja al minimizarla, y sale de la barra de tareas. Desactívalo para minimizar a la barra de tareas como
de costumbre.

![Ajustes: Cerrar a la bandeja y Minimizar a la bandeja (1), y el control deslizante Transparencia del fondo (2)](../../../../assets/screenshots/settings-tray-and-transparency.png)

1. **Cerrar a la bandeja** y **Minimizar a la bandeja**.
2. **Transparencia del fondo**. Consulta [Temas, idioma y transparencia](/es/using/themes-and-language/#transparencia).

## Bloqueo de bandeja y barra de tareas

**Mostrar en la bandeja** y **Mostrar en la barra de tareas** controlan si el icono de la bandeja y el
botón de la barra de tareas están visibles. Al menos uno debe seguir activado; si no, una ventana oculta
no tendría forma de volver. Cuando solo uno está activado, su casilla queda desactivada hasta que vuelvas
a activar el otro.

## Siempre visible

**Siempre visible** en Ajustes mantiene todas las ventanas de Moonpool (el hub, Ajustes, Acerca de, el
editor de apps, el explorador de temas, el instalador y Ayuda) por encima de las demás. Está desactivado
de forma predeterminada.

## Véase también

- [Ejecutar un servidor de desarrollo de npm en segundo plano en Windows](/es/guides/run-npm-dev-server-in-background-windows/)
- [Iniciar un script o un servidor de desarrollo automáticamente al iniciar sesión en Windows](/es/guides/start-app-at-windows-login/)
- [Ventana de Ajustes](/es/using/settings/)
- [La ventana del hub](/es/using/hub-window/)
