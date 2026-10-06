---
title: "Cambia el tema, el idioma y la transparencia de Moonpool"
description: "Elige un tema de color y el idioma de la interfaz, ajusta la transparencia del fondo y la escala, y comprueba que se aplican al instante en todas las ventanas."
---

El tema, el idioma y la transparencia se definen en la [ventana de Ajustes](/es/using/settings/).
Los tres se aplican al instante en todas las ventanas de Moonpool abiertas.

![Selectores de Idioma (1) y Tema (2) en la parte superior de Ajustes](../../../../assets/screenshots/settings-language-theme.png)

1. Selector de Idioma.
2. Botón de Tema. Muestra el nombre del tema actual y abre el explorador de temas.

## Temas

El explorador de temas es una ventana propia. Tiene una tarjeta de vista previa por tema, dibujada con los
colores de ese tema (texto, panel, campo, botón, puntos de estado, el degradado del indicador y el juego
de 16 colores de la terminal), agrupadas como Básicos, Neón, Cálidos, Fríos, Verdes, Neutros, Claros,
Rubor, Vivos, Pastel claro y Pastel. Haz clic en una tarjeta para aplicarla: todas las ventanas de
Moonpool abiertas cambian a la vez y la elección se guarda. La ventana permanece abierta para que puedas
comparar; pulsa Esc para cerrarla.

Hay 68 temas además de Automático, y alrededor de la mitad son claros. Los nombres de los temas son
nombres propios y no se traducen; solo se traducen Automático (sistema), Oscuro y Claro.

**Automático (sistema)** sigue la preferencia de modo claro u oscuro del sistema operativo y cambia en
vivo cuando lo hace el sistema. Cualquier otra opción es fija. Los 16 colores ANSI de la terminal también
siguen el tema.

Si guardaste un tema de una versión anterior, se conserva. Un nombre guardado que Moonpool ya no conoce
vuelve a Automático. Algunas etiquetas difieren de las de antes (por ejemplo, Matrix ahora se llama
Terminal, Nord es Arctic, Dracula es Nocturne, Gruvbox es Retro y Solarized es Solar); la elección
guardada en sí no cambia.

El tema se guarda en el `localStorage` del webview, no en `settings.json`. Si el almacenamiento no está
disponible, vuelve a Automático.

```text
localStorage key: moonpool.theme
```

## Idiomas

Automático sigue el idioma del sistema operativo. Si no, elige uno de 14, cada uno mostrado en su propio idioma:

```text
English, Deutsch, Español, Français, Italiano, 日本語, 한국어, Nederlands, Polski,
Português (Brasil), Русский, Türkçe, 简体中文, 繁體中文
```

El selector se aplica al instante al hub y a las demás ventanas. La elección se guarda como
`locale` en `settings.json`.

## Transparencia

**Transparencia del fondo** hace que el fondo de la ventana sea translúcido, de 0 % (opaco) a
90 %.

- Al pasar el puntero por encima de una ventana, esta pasa al instante a ser totalmente opaca. Cuando el puntero se va, vuelve a tu ajuste con un desvanecimiento de unos 2 segundos.
- Las terminales siguen el mismo tinte en lugar de añadir el suyo.
- Cada ventana (hub, Ajustes, Acerca de, el editor de apps y el explorador de temas) aplica el ajuste por sí misma, y Ajustes actualiza las demás en vivo mientras arrastras el control deslizante.

## Escala de la interfaz

Cambia el zoom de toda la interfaz con Ctrl + rueda del ratón. No hay zoom con el teclado. Consulta
[Atajos y zoom](/es/using/keyboard-shortcuts/).

## Véase también

- [Ventana de Ajustes](/es/using/settings/)
- [Atajos y zoom](/es/using/keyboard-shortcuts/)
