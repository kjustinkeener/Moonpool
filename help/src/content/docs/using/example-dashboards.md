---
title: Example dashboards
description: The bundled offline dashboards, where they live, and how to add the example apps to an existing config.
---

Moonpool ships a set of self-contained dashboards inside the program. They run entirely
offline, with no server and no CDN.

| Dashboard | What it is |
| --- | --- |
| CSV explorer | Drop in a CSV or TSV file; it profiles the columns and renders the data. |
| JSON explorer | Drop in JSON (arrays, nested objects or maps). |
| Excel explorer | Drop in an `.xlsx` or `.xls` file, parsed offline. |
| Moonpool Docs | An offline Markdown docs browser. |

## Where they live

On startup Moonpool writes the dashboards to `{MP_HOME}\dashboards\examples`:

```text
Installed (Windows)  %USERPROFILE%\.moonpool\dashboards\examples
Portable             <your .moonpool folder, the one holding moonpool.exe>\dashboards\examples
Linux                ~/.config/Moonpool/dashboards/examples   (or $XDG_CONFIG_HOME/Moonpool/dashboards/examples)
```

The `examples` folder belongs to Moonpool: it is replaced whenever Moonpool updates, so
edits there are lost. To customise a dashboard, copy its folder and the shared `_lib`
folder up into `dashboards` and point your app at the copy. Moonpool never changes
anything else in `dashboards`.

Versions before 0.3.16 wrote the examples straight into `dashboards`. Those copies stay
where they are and no longer receive updates; apps pointing at them keep working. To get
the updated versions, change their `url` to the `dashboards/examples/...` path below.

## How the apps reference them

Each is a `static` app whose `url` is a `file:///` URL anchored on `{MP_HOME}`:

```text
file:///{MP_HOME}/dashboards/examples/csv/index.html
```

`{MP_HOME}` resolves to the install folder or, in portable mode, the bundle folder, so the entry still
works from a moved bundle. `file://` URLs are allowed. See
[Paths and environment](/configuration/paths-and-environment/).

## Example apps appear only on first run

The example entries are written to `apps.json` only when no config file exists yet. If you
already have an `apps.json`, add the dashboard entries yourself (**Edit apps.json** in the
"..." menu, then **Reload**). Add these four to the array:

```json title="apps.json"
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

Field meanings are in [App fields](/configuration/fields/).

## See also

- [Examples](/configuration/examples/): more complete entries to copy.
- [App types](/configuration/app-types/#static)
