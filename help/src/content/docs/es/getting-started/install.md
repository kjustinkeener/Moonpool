---
title: "Instala Moonpool en Windows o Linux"
description: "Instala Moonpool con unos pocos clics, elige el modo instalado o portable, usa más tarde la opción Instalar Moonpool del menú y desinstálalo limpiamente cuando termines."
---

Esta página es para Windows. En Windows, Moonpool es su propio instalador: la descarga es un único
`moonpool.exe`. Linux no tiene tarjeta de instalación ni selector de modo portable; consulta
[Linux](/es/platforms/linux/).

## Modo instalado

Ejecuta el `moonpool.exe` descargado. En el primer inicio muestra la tarjeta de instalación. Tiene tres
controles: el botón **Instalar Moonpool**, una casilla **Añadir un acceso directo en el escritorio**
(activada de forma predeterminada) y un enlace **Instalar portable**.

La instalación copia Moonpool en tu perfil de usuario, en `.moonpool\`, añade un acceso directo en el menú
Inicio (y otro en el escritorio si la casilla está marcada) y registra una entrada en Agregar o quitar
programas. Después inicia la copia instalada y se cierra. El archivo que descargaste se queda donde
estaba; puedes eliminarlo. A partir de ahí, inicia Moonpool desde el acceso directo como cualquier otra
app.

![La tarjeta de instalación: botón Instalar Moonpool, casilla de acceso directo en el escritorio, enlace Instalar portable y la ruta de instalación](../../../../assets/screenshots/installer-window.png)

Todo lo que Moonpool necesita vive en esa única carpeta: el programa, tu configuración y su ayuda
incluida.

```text title="Installed layout"
%USERPROFILE%\.moonpool\
```

## Instalar Moonpool... desde el menú

En Windows, el menú "..." tiene **Instalar Moonpool…** en ambos modos. Abre la misma tarjeta de
instalación. Desde una copia portable puedes instalarlo correctamente. Desde una copia instalada,
**Instalar Moonpool** aparece desactivado ("Ya está instalado") y **Instalar portable** sigue
disponible.

## Desinstalación

Usa Agregar o quitar programas de Windows (Aplicaciones instaladas), o ejecuta la copia instalada con
`--uninstall`. No está en tu PATH, así que indica su ruta completa:

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" --uninstall
```

Esto elimina los accesos directos del menú Inicio y del escritorio, la entrada del registro y toda la
carpeta `%USERPROFILE%\.moonpool`, **incluida tu configuración** (`apps.json`, los ajustes y los
registros). Haz antes una copia de seguridad de esta carpeta si quieres conservar tu configuración:

```text
%USERPROFILE%\.moonpool\moonpool-config
```

Cualquier Moonpool en ejecución se detiene como parte de la desinstalación.

## Modo portable

¿Prefieres una memoria USB o una carpeta que puedas mover? Haz clic en **Instalar portable** en la
tarjeta de instalación y elige una carpeta. Consulta [Modo portable](/es/data/portable-mode/).

## Siguiente

- [Windows protected your PC](/es/support/windows-protected-your-pc/) (Windows protegió tu PC): si SmartScreen bloquea el instalador.
- [Falta el runtime de WebView2](/es/support/webview2-runtime-missing/): si la ventana se queda en blanco.
- [Tu primera app](/es/getting-started/first-app/)
