---
title: "Windows protected your PC: ejecuta de todos modos el instalador de Moonpool (SmartScreen)"
description: "Windows SmartScreen muestra Windows protected your PC al ejecutar moonpool.exe. Por qué aparece, cómo elegir More info y luego Run anyway, y qué comprobar antes."
---

Al ejecutar el `moonpool.exe` descargado, Windows puede mostrar un cuadro azul titulado **Windows
protected your PC** (Windows protegió tu PC), con la línea "Microsoft Defender SmartScreen prevented an
unrecognized app from starting. Running this app might put your PC at risk." (Microsoft Defender
SmartScreen impidió que se iniciara una aplicación no reconocida. Ejecutar esta aplicación podría poner en
riesgo tu PC).

## Por qué aparece

SmartScreen avisa de los programas nuevos o que no ha visto ejecutarse en muchos PC. `moonpool.exe` no
está firmado con código, así que Windows no tiene un editor en el que confiar y puede mostrar el aviso la
primera vez. Es una comprobación de reputación, no un hallazgo de que el archivo sea malicioso.

## Qué hacer

1. En el cuadro, haz clic en **More info** (Más información). El editor aparece como "Unknown publisher"
   (editor desconocido).
2. Haz clic en **Run anyway** (Ejecutar de todas formas). Se abre la tarjeta de instalación. Consulta
   [Instalación](/es/getting-started/install/).

Si quieres ser prudente antes, descarga solo desde el sitio oficial de Moonpool o sus releases de GitHub,
y comprueba que el nombre del archivo sea `moonpool.exe`.

## Si no hay botón Run anyway

En algunos PC administrados, el administrador desactiva la opción y no verás **Run anyway**. Pregunta a tu
administrador, o usa un PC que gestiones tú. Un archivo que llegó en un zip descargado también puede
llevar un bloqueo: haz clic derecho en el archivo, elige **Propiedades**, marca **Desbloquear** si
aparece, luego **Aceptar** y ejecútalo de nuevo.

## Avisos del antivirus

Un exe nuevo y sin firmar que se copia a sí mismo en tu perfil y se reemplaza al actualizarse también puede
activar el software antivirus. Si el tuyo bloquea o pone en cuarentena `moonpool.exe`, permítelo para la
carpeta `.moonpool`. Consulta [Windows](/es/platforms/windows/#antes-de-ejecutarlo).

## Véase también

- [Instalación](/es/getting-started/install/)
- [Windows](/es/platforms/windows/)
- [El instalador muestra un error](/es/support/troubleshooting/#el-instalador-muestra-un-error)
