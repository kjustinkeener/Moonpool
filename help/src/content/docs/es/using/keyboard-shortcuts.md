---
title: "Atajos de teclado, atajos de ratón y zoom de Moonpool"
description: "Consulta todos los atajos de teclado y de ratón del hub de Moonpool, y cómo acercar y alejar la interfaz para que el texto resulte cómodo de leer."
---

## Teclado

| Teclas | Dónde | Qué hace |
| --- | --- | --- |
| F5, Ctrl+R, Cmd+R | Hub | Recarga `apps.json` del disco, igual que **Recargar** en el menú. La página en sí no se actualiza. |
| Esc | Menú contextual | Lo cierra. |
| Esc | Al cambiar el nombre de una app | Cancela el cambio de nombre. |
| Esc | Ventanas de Ajustes, Acerca de y editor de apps | Cierra la ventana (el editor pregunta antes de descartar los cambios). |
| Intro | Al cambiar el nombre de una app | Guarda el nuevo nombre. |

## Ratón

| Acción | Dónde | Qué hace |
| --- | --- | --- |
| Ctrl + rueda | Hub | Cambia el zoom de la interfaz. |
| Seleccionar texto | Terminal | Copia y borra la selección. |
| Clic central | Terminal | Pega. |
| Clic derecho | Fila de la barra lateral | Abre el menú de la fila. Consulta [Barra lateral y menús](/es/using/sidebar-and-menus/). |

## Zoom

Mantén pulsada Ctrl y gira la rueda sobre el hub para cambiar el zoom. Girar hacia arriba acerca y hacia abajo aleja, en pasos de aproximadamente un 10 por ciento por evento de la rueda.

```text
Ctrl + wheel up      zoom in
Ctrl + wheel down    zoom out
```

- El intervalo va de 0,5x a 3x.
- La ventana cambia de tamaño en el mismo factor, de modo que el diseño queda tan ajustado a 2x como a 1x. Cuando se alcanza el límite, la ventana deja de crecer.
- El factor se guarda como `uiScale` en `settings.json` y se aplica en el siguiente inicio. El tamaño de ventana guardado ya es el tamaño con zoom, así que no se vuelve a escalar. Consulta [settings.json](/es/data/settings-json/).

- El zoom se aplica solo a la ventana del hub. Ajustes, Acerca de, el editor de apps y Ayuda mantienen
  su propio tamaño.

No hay ningún control en Ajustes para `uiScale` ni una tecla para restablecerlo. Para volver al tamaño
normal, gira la rueda el mismo número de pasos en sentido contrario, o sal de Moonpool, establece
`uiScale` en `1` en `settings.json` (o elimina la clave) y vuelve a iniciarlo.

## Véase también

- [Temas, idioma y transparencia](/es/using/themes-and-language/)
- [Barra lateral y menús](/es/using/sidebar-and-menus/)
