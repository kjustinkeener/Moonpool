---
title: "Instala y usa Moonpool en Linux"
description: "Instala Moonpool en Linux, resuelve el problema de la bandeja en GNOME, aprende cómo funcionan las actualizaciones y qué difiere de la versión para Windows."
---

Moonpool se ejecuta en Linux mediante WebKitGTK. Se desarrolla principalmente en Windows, así que Linux
es compatible pero está menos probado. En Linux no hay tarjeta de instalación ni selector de modo
portable, y el menú "..." no tiene la opción **Instalar Moonpool…**.

## Instalar

Descarga un paquete desde la página de Releases del proyecto.

| Paquete | Actualizaciones |
| --- | --- |
| AppImage | Moonpool se actualiza solo |
| `.deb` | Tu gestor de paquetes |
| RPM (instálalo con la herramienta RPM de tu distribución) | Tu gestor de paquetes |

```bash title="AppImage" frame="terminal"
chmod +x Moonpool_*.AppImage
./Moonpool_*.AppImage
```

```bash title=".deb" frame="terminal"
sudo apt install ./Moonpool_*_amd64.deb
```

```bash title="RPM" frame="terminal"
sudo dnf install ./Moonpool-*.x86_64.rpm
```

El `.deb` instala sus dependencias de ejecución. El AppImage necesita que estén presentes las bibliotecas
WebKitGTK y AppIndicator, por ejemplo en Debian o Ubuntu:

```bash frame="terminal"
sudo apt-get install -y libwebkit2gtk-4.1-0 libayatana-appindicator3-1
```

En Fedora o Arch usa los equivalentes:

```bash title="Fedora" frame="terminal"
sudo dnf install webkit2gtk4.1 libayatana-appindicator-gtk3
```

```bash title="Arch" frame="terminal"
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator
```

## Bandeja en GNOME

GNOME en su configuración estándar no muestra iconos de bandeja, así que el icono de bandeja de Moonpool
no aparecerá hasta que la extensión AppIndicator esté instalada y activada:

```bash frame="terminal"
sudo apt-get install -y gnome-shell-extension-appindicator
gnome-extensions enable ubuntu-appindicators@ubuntu.com
```

Después cierra sesión y vuelve a iniciarla. La ventana del hub y las terminales integradas funcionan sin
ella. KDE, Cinnamon, XFCE y MATE muestran la bandeja de serie.

## Actualizaciones

Solo el AppImage se actualiza solo. Lee `linux-update.json` de GitHub Releases, verifica la firma minisign
y reemplaza el archivo AppImage en su sitio, así que guárdalo en una carpeta en la que puedas escribir.
Las instalaciones `.deb` y RPM nunca las sobrescribe Moonpool: la búsqueda de actualizaciones puede seguir
informando de una versión más reciente, pero instalarla desde Moonpool falla con un mensaje que indica
usar tu gestor de paquetes. Consulta [Actualización](/es/data/updating/).

## Ubicación de la configuración

```text
~/.config/Moonpool/              (or $XDG_CONFIG_HOME/Moonpool/)
~/.config/Moonpool/apps.json
~/.config/Moonpool/dashboards/examples/   (example dashboards)
```

`apps.json` se genera a partir del ejemplo en la primera ejecución. Consulta
[Resumen de la configuración](/es/apps/apps-json/).

## Diferencias con Windows

- Los comandos de inicio se ejecutan mediante `$SHELL -c <command>` (`/bin/sh` si `SHELL` no está definido), así que usa una sintaxis que tu shell entienda.
- Detener termina el grupo de procesos y luego hace la limpieza adicional elegida con `killMode`. Liberar un puerto con `killMode: "port"` usa `lsof`, con `fuser` como alternativa; instala `lsof` si tu distribución no lo incluye. Consulta [Detener y reiniciar](/es/apps/stop-and-restart/).
- El `processName` de una app `desktop` debe tener 15 caracteres o menos. Linux trunca el nombre de un proceso a 15 caracteres, así que un nombre más largo nunca se detecta como en ejecución y no se puede detener por nombre. Las apps `web` coinciden por su puerto y no se ven afectadas.
- Los iconos se buscan en `src-tauri/icons/`, `public/favicon.*`, `icon.png` de la app o en su favicon en vivo. Extraer un icono de un binario solo es posible en Windows.
- Los botones de abrir abren la carpeta contenedora en lugar de seleccionar el archivo.
- Los archivos de configuración se abren en tu editor de texto predeterminado (se resuelve a partir de la asociación `text/plain`).
- El instalador de Windows, los accesos directos y la entrada de Agregar o quitar programas no se aplican.

## Véase también

- [Windows](/es/platforms/windows/#lo-que-difiere-según-la-plataforma): una tabla de lo que difiere según la plataforma.
- [Actualización](/es/data/updating/#linux)
