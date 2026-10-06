---
title: "Pfade, MP_HOME-Token und Umgebungsvariablen in Apps verwenden"
description: "Nutzen Sie die Token {MP_HOME} und {MP_DATA} und relative ./-Pfade in App-Einträgen, sehen Sie, welche Felder sie auflösen, und setzen Sie env und cwd."
---

## Token

| Token | Wird zu |
| --- | --- |
| `{MP_HOME}` | Portabel: der Ordner mit `moonpool.exe` (der Ordner `.moonpool\`). Installiert unter Windows: `%USERPROFILE%\.moonpool`. Linux: `$XDG_CONFIG_HOME/Moonpool`, sonst `~/.config/Moonpool`, derselbe Ordner wie `{MP_DATA}`. |
| `{MP_DATA}` | Der Konfigurationsordner, der `apps.json` enthält. |

Ein Token, das sich nicht auflösen lässt, bleibt wie geschrieben stehen.

## Welche Felder aufgelöst werden

| Feld | Token | Führendes `./` oder `.\` |
| --- | --- | --- |
| `cwd` | ja | ja, an `{MP_HOME}` verankert |
| `command` | ja | nein |
| `stopCommand` | ja | nein (er läuft in `cwd`, das verankert ist) |
| `url` | ja | nein |
| `icon` | ja | ja, an `{MP_HOME}` verankert |
| `env`-Werte, `processName`, `note` | nein | nein |

Ein relativer Pfad ohne `./` (etwa `apps\tool`) bleibt unverändert und wird relativ zum
eigenen Arbeitsordner von Moonpool aufgelöst, was selten gewünscht ist. Bevorzugen Sie `./` oder ein Token.

```text
./apps/notes                       anchored to {MP_HOME}
{MP_HOME}\apps\notes\notes.exe     token
{MP_DATA}\dumps                    token
apps\tool                          left alone, resolves against Moonpool's working folder
```

```json title="apps.json"
{ "id": "notes", "name": "Notes", "group": "Desktop apps", "type": "desktop",
  "cwd": "./apps/notes",
  "command": "{MP_HOME}\\apps\\notes\\notes.exe",
  "processName": "notes" }
```

Beide Formen funktionieren weiter, wenn Sie den portablen Ordner verschieben. Ein fester Pfad wie
`C:\tools\notes` zieht nicht mit um. Im portablen Modus kennzeichnet der Dialog „App bearbeiten“ absolute `cwd`-
und `url`-Werte mit dem Hinweis „nicht portabel“. Siehe [Portabler Modus](/de/data/portable-mode/).

## Umgebung

`env` ist ein Objekt aus Zeichenketten. Der Dialog bearbeitet es als eine `KEY=VALUE` pro Zeile; er trennt
jede Zeile am ersten `=`, entfernt auf beiden Seiten Leerraum und ignoriert Zeilen ohne eines.

Im Dialog:

```text
PORT=8091
NODE_ENV=development
```

In `apps.json` als Schlüssel `env` des Eintrags:

```json title="apps.json (one entry)"
{ "id": "habits", "name": "Habits", "group": "Web apps", "type": "web", "command": "python app.py",
  "env": { "PORT": "8091", "NODE_ENV": "development" } }
```

- Der gestartete Befehl erbt die Umgebung von Moonpool plus `env`. Einträge in `env` haben Vorrang.
- `env` wird auch auf `stopCommand` angewendet.
- Werte werden so verwendet, wie sie geschrieben sind: keine `{MP_HOME}`-Auflösung und keine `%VAR%`-Auflösung durch Moonpool.
- Moonpool verweist sein eigenes WebView2 über
  `WEBVIEW2_USER_DATA_FOLDER` auf einen privaten Profilordner. Gestartete Apps erben das nicht. Wenn Sie die
  Variable selbst vor dem Start von Moonpool gesetzt hatten, erhalten sie Ihren Wert; andernfalls ist sie nicht gesetzt.
  Ein `env`-Eintrag kann sie trotzdem überschreiben.

## Arbeitsordner

Der Befehl und `stopCommand` laufen in `cwd`. Ist `cwd` nicht angegeben, läuft der Befehl im
eigenen Arbeitsordner von Moonpool, setzen Sie `cwd` also für alles, was relative Pfade verwendet.
