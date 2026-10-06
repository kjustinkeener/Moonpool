---
title: "Usa las pestañas de terminal de Moonpool: abrir, cerrar, copiar, reiniciar"
description: "Trabaja con las pestañas de terminal de cada app en el hub: abre y cierra pestañas, contrae el panel, copia y pega, reinicia una sesión y encuentra los registros de sesión."
---

Cada app se ejecuta en su propia pestaña de terminal en el panel CLI.

## Pestañas

![Tira de pestañas con Metrics Dashboard activa (resaltada) y su registro en vivo debajo; cada pestaña tiene un punto y una x](../../../../assets/screenshots/hub-terminal-tab.png)

- Al iniciar una app, o al hacer clic en su nombre en la barra lateral, se abre su pestaña. Hacer clic en un nombre no inicia nada; consulta [Estados de una app](/es/support/glossary/#estados-de-una-app).
- Un punto en la pestaña está encendido mientras la app está en ejecución.
- La **x** de una pestaña cierra la pestaña. No detiene la app. Haz clic de nuevo en el nombre para reabrir la pestaña; muestra el registro de esta sesión.

## Contraer el panel

La **x** del extremo derecho de la tira de pestañas ("Ocultar el panel CLI") contrae el panel CLI y reduce
la ventana a solo la barra lateral. Las terminales siguen en ejecución y conservan su historial de
desplazamiento.

Junto al cuadro de filtro aparece una flecha para recuperar el panel con su ancho anterior. La flecha
parpadea cuando hay una actualización esperando, porque el aviso de actualización está en el panel.

## Copiar y pegar

| Acción | Resultado |
| --- | --- |
| Seleccionar texto con el ratón | Se copia al portapapeles al soltar y luego se borra la selección. |
| Clic central | Pega el portapapeles en la terminal. |
| Botón **Copiar todo** (arriba a la derecha, aparece al pasar el cursor) | Copia todo el historial de desplazamiento como texto. |

## Historial de desplazamiento

Cada terminal conserva 10.000 líneas.

## Cuando termina un proceso

Cuando el proceso termina, la terminal imprime:

```text
[process exited]
```

La pestaña permanece abierta con su salida intacta. La línea `[process exited]` se muestra en tu idioma
(en español: `[proceso finalizado]`).

## Reiniciar

**Reiniciar** (o Iniciar en una app detenida) comienza una nueva ejecución en la misma pestaña. La pestaña
se reconstruye y la salida anterior de esta sesión se reproduce en ella a partir del registro de sesión.

Si la app ya se ejecutó antes en esta sesión, Moonpool escribe primero un separador atenuado en el
registro de sesión, de modo que aparece entre la salida antigua y la nueva ejecución:

```text
---------- restarted 2026-10-05 09:14:02 ----------
```

Si la nueva ejecución empieza borrando la pantalla, la salida anterior pasa al historial de
desplazamiento en lugar de borrarse.

## Registros de sesión

Todo lo que imprime una app también se escribe en un archivo de registro dentro de `cli-output\`, un archivo por app y por sesión de Moonpool. La ubicación, la retención y el ajuste **Conservar los registros de salida de las apps entre sesiones** están en [Registros](/es/data/logs/).
