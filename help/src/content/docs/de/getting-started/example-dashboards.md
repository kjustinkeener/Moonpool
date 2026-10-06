---
title: "Die mit Moonpool gelieferten Beispiel-Dashboards ausprobieren"
description: "Öffnen Sie die mitgelieferten Offline-Beispiel-Dashboards, sehen Sie, wo sie liegen und wie Beispiel-Apps sie referenzieren, und fügen Sie sie einer bestehenden Konfiguration hinzu."
---

Moonpool liefert eine Reihe eigenständiger Dashboards im Programm mit. Sie laufen vollständig
offline, ohne Server und ohne CDN.

| Dashboard | Was es ist |
| --- | --- |
| CSV explorer | Legen Sie eine CSV- oder TSV-Datei ab; es erstellt ein Profil der Spalten und stellt die Daten dar. |
| JSON explorer | Legen Sie JSON ab (Arrays, verschachtelte Objekte oder Maps). |
| Excel explorer | Legen Sie eine `.xlsx`- oder `.xls`-Datei ab, die offline verarbeitet wird. |
| Moonpool Docs | Ein Offline-Browser für Markdown-Dokumentation. |

## Wo sie liegen

Beim Start schreibt Moonpool die Dashboards nach `{MP_HOME}\dashboards\examples`:

| Modus | Ordner |
| --- | --- |
| Installiert (Windows) | `%USERPROFILE%\.moonpool\dashboards\examples` |
| Portabel | `<your .moonpool folder, the one holding moonpool.exe>\dashboards\examples` |
| Linux | `~/.config/Moonpool/dashboards/examples` (oder `$XDG_CONFIG_HOME/Moonpool/dashboards/examples`) |

Der Ordner `examples` gehört Moonpool: Er wird bei jedem Update von Moonpool ersetzt, daher gehen
Änderungen dort verloren. Um ein Dashboard anzupassen, kopieren Sie seinen Ordner und den
gemeinsamen Ordner `_lib` nach oben in `dashboards` und richten Ihre App auf die Kopie. Moonpool
ändert sonst nichts in `dashboards`.

Versionen vor 0.3.16 schrieben die Beispiele direkt nach `dashboards`. Diese Kopien bleiben, wo
sie sind, und erhalten keine Updates mehr; Apps, die darauf zeigen, funktionieren weiter. Um die
aktualisierten Versionen zu erhalten, ändern Sie deren `url` auf den unten genannten Pfad
`dashboards/examples/...`.

## Wie die Apps darauf verweisen

Jedes ist eine `static`-App, deren `url` eine `file:///`-URL mit `{MP_HOME}` als Anker ist:

```text
file:///{MP_HOME}/dashboards/examples/csv/index.html
```

`{MP_HOME}` wird zum Installationsordner oder, im portablen Modus, zum Bundle-Ordner aufgelöst,
sodass der Eintrag auch aus einem verschobenen Bundle funktioniert. `file://`-URLs sind erlaubt.
Siehe [Pfade und Umgebung](/de/apps/paths-and-environment/).

## Beispiel-Apps erscheinen nur beim ersten Start

Die Beispieleinträge werden nur dann in `apps.json` geschrieben, wenn noch keine Konfigurationsdatei
existiert. Haben Sie bereits eine `apps.json`, fügen Sie die Dashboard-Einträge selbst hinzu
(**apps.json bearbeiten** im Menü „...“, dann **Neu laden**). Fügen Sie diese vier innerhalb des
Arrays der obersten Ebene ein, durch Kommas von Ihren anderen Einträgen getrennt:

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

Die Bedeutung der Felder steht unter [App-Felder](/de/apps/fields/).

## Siehe auch

- [Beispiele](/de/apps/examples/): weitere vollständige Einträge zum Kopieren.
- [App-Typen](/de/apps/types/#static)
