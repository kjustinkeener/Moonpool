---
title: "Prueba los paneles de ejemplo que incluye Moonpool"
description: "Abre los paneles de ejemplo sin conexión incluidos, mira dónde están y cómo los referencian las apps de ejemplo, y añádelos a una configuración que ya tengas."
---

Moonpool incluye en el propio programa un conjunto de paneles autónomos. Funcionan por completo
sin conexión, sin servidor y sin CDN.

| Panel | Qué es |
| --- | --- |
| CSV explorer | Suelta un archivo CSV o TSV; analiza las columnas y muestra los datos. |
| JSON explorer | Suelta un JSON (matrices, objetos anidados o mapas). |
| Excel explorer | Suelta un archivo `.xlsx` o `.xls`, que se procesa sin conexión. |
| Moonpool Docs | Un navegador de documentación en Markdown sin conexión. |

## Dónde están

Al iniciarse, Moonpool escribe los paneles en `{MP_HOME}\dashboards\examples`:

| Modo | Carpeta |
| --- | --- |
| Instalado (Windows) | `%USERPROFILE%\.moonpool\dashboards\examples` |
| Portable | `<tu carpeta .moonpool, la que contiene moonpool.exe>\dashboards\examples` |
| Linux | `~/.config/Moonpool/dashboards/examples` (o `$XDG_CONFIG_HOME/Moonpool/dashboards/examples`) |

La carpeta `examples` pertenece a Moonpool: se reemplaza cada vez que Moonpool se actualiza, así
que los cambios que hagas ahí se pierden. Para personalizar un panel, copia su carpeta y la
carpeta compartida `_lib` en `dashboards` y apunta tu app a la copia. Moonpool nunca cambia nada
más dentro de `dashboards`.

Las versiones anteriores a la 0.3.16 escribían los ejemplos directamente en `dashboards`. Esas
copias se quedan donde están y ya no reciben actualizaciones; las apps que apuntan a ellas siguen
funcionando. Para obtener las versiones actualizadas, cambia su `url` a la ruta
`dashboards/examples/...` que se indica más abajo.

## Cómo las referencian las apps

Cada una es una app `static` cuya `url` es una URL `file:///` anclada en `{MP_HOME}`:

```text
file:///{MP_HOME}/dashboards/examples/csv/index.html
```

`{MP_HOME}` se resuelve en la carpeta de instalación o, en modo portable, en la carpeta del
paquete, de modo que la entrada sigue funcionando si mueves el paquete. Se permiten las URL
`file://`. Consulta [Rutas y entorno](/es/apps/paths-and-environment/).

## Las apps de ejemplo solo aparecen la primera vez

Las entradas de ejemplo se escriben en `apps.json` solo cuando todavía no existe ningún archivo de
configuración. Si ya tienes un `apps.json`, añade tú mismo las entradas de los paneles (**Editar apps.json**
en el menú "...", y luego **Recargar**). Añade estas cuatro dentro de la matriz de nivel
superior, separadas de tus otras entradas por comas:

```jsonc title="apps.json (excerpt)"
{
  "id": "csv-explorer",
  "name": "Sample CSV Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html",
  "openBrowser": true
},
{
  "id": "json-explorer",
  "name": "Sample JSON Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/json/index.html",
  "openBrowser": true
},
{
  "id": "xlsx-explorer",
  "name": "Sample Excel Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/xlsx/index.html",
  "openBrowser": true
},
{
  "id": "docs-browser",
  "name": "Moonpool Docs",
  "group": "Docs",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/docs/index.html",
  "openBrowser": true
}
```

El significado de los campos está en [Campos de las apps](/es/apps/fields/).

## Véase también

- [Ejemplos](/es/apps/examples/): entradas más completas para copiar.
- [Tipos de app](/es/apps/types/#static)
