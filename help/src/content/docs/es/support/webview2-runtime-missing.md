---
title: "Falta el runtime de WebView2: soluciona una ventana de Moonpool en blanco o ausente en Windows"
description: "Si la ventana de Moonpool nunca se abre o se queda en blanco en Windows, puede faltar el runtime de Microsoft Edge WebView2. Cómo comprobarlo e instalarlo."
---

Si la ventana de Moonpool nunca se abre, o se abre y se queda en blanco, en Windows, la causa probable es
que falte el runtime de Microsoft Edge WebView2. Moonpool es una app de Tauri, y sus ventanas son páginas
web dibujadas por WebView2.

WebView2 se incluye con Windows 11 y con el Windows 10 actualizado, así que la mayoría de los PC ya lo
tienen. Puede faltar en un Windows 10 antiguo o recortado, o en un PC donde se haya eliminado. El código
fuente de Moonpool no muestra un mensaje específico para este caso, así que aquí no se cita ningún texto
de error: el síntoma es que la ventana no aparece o está vacía.

## Comprueba si está instalado

En PowerShell, busca la versión del runtime en el registro (la primera ruta es la instalación para todo el
sistema, la segunda una por usuario):

```powershell frame="terminal"
Get-ItemProperty "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
Get-ItemProperty "HKCU:\Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
```

Un número de versión como `120.0.2210.91` significa que está instalado. Un error en ambas significa que
no lo está.

## Instálalo

Descarga el runtime **Evergreen** de WebView2 desde la página de WebView2 de Microsoft (busca "WebView2
Runtime download"), ejecuta el instalador y vuelve a iniciar Moonpool. El runtime Evergreen se actualiza
solo.

## Si está instalado y la ventana sigue en blanco

- Sal de todos los Moonpool desde la bandeja (o termina `moonpool.exe` en el Administrador de tareas) y
  vuelve a iniciarlo.
- Activa **Registrar información de depuración en un archivo** en [Ajustes](/es/using/settings/) si puedes
  acceder a él, y revisa `moonpool.log`. Consulta [Registros](/es/data/logs/).
- Si la ventana se abre pero está fuera de la pantalla, consulta
  [Problemas con la ventana](/es/support/troubleshooting/#problemas-con-la-ventana).

## Véase también

- [Windows](/es/platforms/windows/#antes-de-ejecutarlo)
- [Instalación](/es/getting-started/install/)
- [Solución de problemas](/es/support/troubleshooting/)
