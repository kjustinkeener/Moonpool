---
title: "Lee la barra lateral: puntos de estado, grupos, filtro y menú de fila"
description: "Aprende qué muestra cada fila de la barra lateral, cómo funcionan los puntos de estado y los grupos, cómo filtrar apps y usar el menú contextual de la fila."
---

La barra lateral lista todas las apps de `apps.json`, agrupadas por el campo `group` de cada app. Consulta [Campos de las apps](/es/apps/fields/).

## Filas

Cada fila muestra un punto de estado, el icono de la app (o un glifo de tipo si no tiene icono), el nombre, el puerto si está definido (`:3000`) y los controles.

| Punto | Significado |
| --- | --- |
| Fijo | en ejecución |
| Parpadeante | iniciando: Moonpool inició la app pero todavía no se detecta como activa |
| Gris | detenida |

Pasa el cursor por el punto para ver la palabra.

![La barra lateral con dos apps web en ejecución resaltadas: puntos encendidos y botones Detener](../../../../assets/screenshots/sidebar-running-narrow.png)

1. Dos apps en ejecución. Sus puntos están encendidos y Detener (el cuadrado) sustituye a Iniciar.

| Control | Qué hace |
| --- | --- |
| Lápiz | Editar la app. |
| Reiniciar | Detener y volver a iniciar. En una app detenida, simplemente la inicia. |
| Iniciar (reproducir) | Inicia la app y abre su pestaña de terminal. Se muestra cuando la app está detenida. |
| Detener (cuadrado) | Detiene la app. Se muestra mientras está en ejecución o iniciándose. |

![Una fila en ejecución, ampliada: punto de estado, icono de tipo, nombre y puerto, y luego los botones de editar, reiniciar y detener](../../../../assets/screenshots/sidebar-row-controls.png)

1. Punto de estado (encendido mientras está en ejecución).
2. Icono de tipo.
3. Editar (lápiz).
4. Reiniciar.
5. Detener (se muestra en lugar de Iniciar mientras está en ejecución).

Mientras un inicio o una detención está en curso, los controles se sustituyen por un indicador giratorio (`Trabajando...`).

Al hacer clic en el **nombre** de una app se abre o se enfoca su pestaña de terminal y nunca se inicia nada. Una pestaña de una app detenida muestra el registro de esta sesión. Usa Iniciar o Reiniciar para ponerla en marcha. Una app `static` con solo una `url` y sin `command` no tiene terminal: Iniciar abre la URL en tu navegador.

### Información emergente

Al pasar el cursor por el nombre se muestra la `note` de la app si la tiene; si no, su nombre. Define `note` en el editor o en `apps.json`.

### Subfila MCP

Cuando un cliente de IA ha usado las herramientas MCP propias de una app, aparece bajo la app una subfila atenuada `Servidor MCP`. Su punto está encendido y la información emergente dice "Cliente MCP conectado" mientras el cliente está conectado. Un botón de detener termina ese proceso.

La fila encuentra el proceso por `processName` más el argumento `mcp`, o por el patrón `mcpProcessName` de la app cuando está definido. Consulta [campos](/es/apps/fields/#mcpprocessname).

Oculta estas filas con **Mostrar procesos MCP** en Ajustes. Consulta
[Configuración de MCP](/es/automation/mcp-setup/#apps-que-tienen-su-propio-servidor-mcp).

## Grupos

![Barra lateral en reposo con los cinco encabezados de grupo resaltados, cada uno con su número de apps a la derecha](../../../../assets/screenshots/sidebar-groups-narrow.png)

- Haz clic en el encabezado de un grupo para contraerlo o expandirlo. El número que lo acompaña es la cantidad de apps mostradas. Los grupos contraídos se recuerdan.
- Dentro de un grupo, la app iniciada más recientemente va arriba. Las apps que nunca se han iniciado conservan el orden de `apps.json`. Una app recién iniciada brilla y sube a la parte superior.

## Cuadro de filtro

Escribe en **Filtrar apps...** para reducir la lista. Coincide con el nombre de la app y el del grupo, sin distinguir mayúsculas de minúsculas. Si nada coincide, la lista muestra:

```text
Ninguna app coincide con «<texto>».
```

La app muestra comillas angulares alrededor del texto, aquí y en el mensaje de Eliminar más abajo.

## Menú del clic derecho

Haz clic derecho en una fila para ver:

| Elemento | Qué hace |
| --- | --- |
| Editar | Abre el editor de apps. |
| Cambiar nombre | Convierte el nombre en un campo editable. **Intro** o hacer clic fuera lo guarda, **Esc** cancela. Un nombre vacío o sin cambios se ignora. |
| Elegir icono... | Elige un archivo de imagen (png, jpg, jpeg, gif, svg, webp, ico) para usarlo como icono. |
| Eliminar | Pregunta `¿Eliminar «<nombre>»?` y quita la entrada de `apps.json`. Si Moonpool tiene la app en ejecución, se detiene primero. |

**Esc** cierra el menú sin hacer nada.

## Cambiar el ancho

Arrastra el divisor entre la barra lateral y el panel CLI. El ancho está limitado de 180 a 620 px (280 de forma predeterminada) y se recuerda. El divisor queda bloqueado mientras el panel CLI está contraído. Consulta [Pestañas de terminal](/es/using/terminal-tabs/).
