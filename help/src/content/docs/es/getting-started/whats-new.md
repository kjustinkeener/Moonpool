---
title: "Notas de versión de Moonpool y cambios recientes"
description: "Consulta qué ha cambiado en las últimas versiones de Moonpool, los requisitos para ejecutarlo y dónde encontrar las notas de versión completas en GitHub."
---

Las notas completas de cada versión están en la
[página de Releases](https://github.com/kjustinkeener/Moonpool/releases) del proyecto. Esta ayuda se
distribuye dentro de Moonpool, así que siempre describe la versión que usas. Moonpool se actualiza solo;
consulta [Actualización](/es/data/updating/).

## 0.3.18

- **La actualización ya no falla con "Acceso denegado"** mientras sigue en ejecución un
  servidor MCP de Moonpool iniciado antes de la actualización anterior.

## 0.3.17

- **Ayuda en 14 idiomas.** La ayuda se abre en el idioma de Moonpool: inglés, alemán, español,
  francés, italiano, neerlandés, polaco, portugués de Brasil, ruso, turco, japonés, coreano y
  chino simplificado y tradicional.
- **Nuevas guías y páginas de soporte** sobre servidores de desarrollo, puertos, inicio al
  iniciar sesión, agentes MCP, scripts de Python y mensajes de error habituales.
- **`mcpProcessName`.** Un patrón comodín para el nombre de proceso del servidor MCP de una app,
  para servidores que se ejecutan con otro nombre. Consulta [mcpProcessName](/es/apps/fields/#mcpprocessname).
- **macOS ya no es compatible.** No hay compilaciones para macOS. Windows y Linux no cambian.

## 0.3.16

- **Varios Moonpool a la vez.** El Moonpool instalado y cualquier número de copias portables pueden
  ejecutarse en paralelo, una por carpeta, cada una con sus propias apps, su icono en la bandeja y su
  canal de control. Consulta [Modo portable](/es/data/portable-mode/#varias-copias-a-la-vez).
- **Explorador de temas.** 68 temas, cada uno con una vista previa en sus propios colores. Consulta
  [Temas, idioma y transparencia](/es/using/themes-and-language/).
- **Ejemplos ejecutables.** Un `apps.json` nuevo contiene apps de ejemplo que se ejecutan tal cual.
  Los paneles de ejemplo ahora están en una carpeta `dashboards/examples` propiedad de la app, que se
  actualiza con Moonpool. Consulta [Paneles de ejemplo](/es/getting-started/example-dashboards/).
- **Se muestran los errores de apps.json.** Un aviso sobre la barra lateral muestra el error, y un
  Recargar fallido conserva la última lista que se cargó. Consulta
  [Cuando apps.json tiene un error](/es/using/hub-window/#cuando-appsjson-tiene-un-error).
- **Canal de control en Linux**, mediante un socket Unix, además del verbo `list`. Consulta
  [Verbos de control](/es/automation/control-verbs/).
- Acerca de y el editor de apps siguen en vivo los cambios de tema e idioma. La opción de menú
  **Instalar Moonpool…** está oculta fuera de Windows.

## 0.3.15

- Una app reiniciada conserva su salida anterior, con un separador «reiniciada» con fecha. Consulta
  [Pestañas de terminal](/es/using/terminal-tabs/#reiniciar).
- Cada app tiene su propia carpeta `cli-output`, de modo que la poda de registros nunca toca los
  registros de otra app.
- `killMode` y `stopCommand` están en el editor de apps. Consulta
  [Detener y reiniciar](/es/apps/stop-and-restart/).
- Las apps iniciadas ya no heredan el perfil de WebView2 de Moonpool.

## 0.3.14

- Los registros de sesión se pueden conservar entre sesiones, con un límite de tamaño por app. Consulta
  [Registros](/es/data/logs/).
- Botones Abrir y Copiar para las carpetas de registros en Ajustes.
- Correcciones en la barra de título de la ventana de Ayuda.

## Requisitos

- Windows 10 u 11 con WebView2 (consulta [Windows](/es/platforms/windows/)).
- Linux con WebKitGTK 4.1 y una biblioteca AppIndicator (consulta [Linux](/es/platforms/linux/)).
