---
title: "Usa Moonpool en Windows"
description: "Windows es la plataforma principal de Moonpool: cómo instalarlo y una tabla de lo que difiere entre Windows y Linux para que sepas qué esperar."
---

Windows es la plataforma principal de Moonpool. Instálalo como se describe en
[Instalación](/es/getting-started/install/).

## Antes de ejecutarlo

- **SmartScreen.** `moonpool.exe` no está firmado con código, así que Windows puede mostrar "Windows
  protected your PC" (Windows protegió tu PC) la primera vez. Elige **More info** (Más información) y
  luego **Run anyway** (Ejecutar de todas formas).
- **Antivirus.** Un exe nuevo y sin firmar que se copia a sí mismo y se reemplaza al actualizarse puede
  activar un antivirus. Si el tuyo bloquea o pone en cuarentena `moonpool.exe`, permítelo para la
  carpeta `.moonpool`.
- **WebView2.** Las ventanas de Moonpool usan Microsoft Edge WebView2, que se incluye con Windows 11 y con
  Windows 10 actualizado. Si la ventana se queda en blanco o nunca se abre, instala el runtime Evergreen
  de WebView2 de Microsoft.

Más información: [Windows protected your PC](/es/support/windows-protected-your-pc/),
[Falta el runtime de WebView2](/es/support/webview2-runtime-missing/) e
[Iniciar un script o un servidor de desarrollo automáticamente al iniciar sesión en Windows](/es/guides/start-app-at-windows-login/).

## Bandeja

En Windows 11, un icono de bandeja nuevo suele ir al área de iconos ocultos. Haz clic en la flecha **^**
a la derecha de la barra de tareas para encontrarlo y arrástralo a la barra de tareas para mantenerlo
visible.

## Comandos

- Los comandos se ejecutan mediante `cmd /c`. Evita las comillas dobles anidadas en `command`; `cmd /c`
  las estropea. Para un script que deba dejar un shell abierto, usa
  `pwsh -NoLogo -NoProfile -NoExit -Command <script and args>` sin comillas alrededor de la parte del
  script.
- `processName` coincide con o sin `.exe`, sin distinguir mayúsculas de minúsculas.
- Detener termina todo el árbol de procesos que Moonpool inició, incluidos los procesos que se
  desacoplaron de él.
- Las apps de Docker Desktop necesitan `killMode` `none` o `command`, nunca `port`. Consulta
  [Apps de Docker en Windows](/es/apps/stop-and-restart/#apps-de-docker-en-windows).

## Lo que difiere según la plataforma

| | Windows | Linux |
| --- | --- | --- |
| Instalación | `moonpool.exe` autoinstalable, o portable | AppImage, `.deb` o RPM; sin tarjeta de instalación |
| Actualización automática | Sí, instalado y portable | Solo AppImage |
| Carpeta de configuración | `%USERPROFILE%\.moonpool\moonpool-config\` | `~/.config/Moonpool/` |
| Shell para los comandos | `cmd /c` | `$SHELL -c` |
| `processName` | Cualquier longitud, `.exe` opcional, sin distinguir mayúsculas | 15 caracteres o menos, mayúsculas exactas |
| Detener por `processName` | Termina el proceso y sus hijos | Termina solo los procesos con ese nombre exacto |
| Canal de control | Canalización con nombre | Socket Unix |
| Capturas de ventana (pruebas) | Sí | No |
| Iconos desde un archivo de programa | Sí | No |
| Bandeja | Funciona de serie | Necesita AppIndicator; GNOME estándar necesita una extensión |

Los detalles de Linux están en la página de [Linux](/es/platforms/linux/).
