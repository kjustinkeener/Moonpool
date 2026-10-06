---
title: "Usa rutas, tokens MP_HOME y variables de entorno en las apps"
description: "Usa los tokens {MP_HOME} y {MP_DATA} y las rutas relativas ./ en las entradas de apps, comprueba qué campos los expanden y define env y la carpeta de trabajo."
---

## Tokens

| Token | Se expande a |
| --- | --- |
| `{MP_HOME}` | Portable: la carpeta que contiene `moonpool.exe` (la carpeta `.moonpool\`). Instalado en Windows: `%USERPROFILE%\.moonpool`. Linux: `$XDG_CONFIG_HOME/Moonpool`, o si no `~/.config/Moonpool`, la misma carpeta que `{MP_DATA}`. |
| `{MP_DATA}` | La carpeta de configuración, la que contiene `apps.json`. |

Un token que no se puede resolver se deja tal como está escrito.

## Qué campos se expanden

| Campo | Tokens | `./` o `.\` inicial |
| --- | --- | --- |
| `cwd` | sí | sí, anclado a `{MP_HOME}` |
| `command` | sí | no |
| `stopCommand` | sí | no (se ejecuta en `cwd`, que está anclado) |
| `url` | sí | no |
| `icon` | sí | sí, anclado a `{MP_HOME}` |
| valores de `env`, `processName`, `note` | no | no |

Una ruta relativa sin `./` (como `apps\tool`) se deja como está y se resuelve respecto a la carpeta de
trabajo del propio Moonpool, que rara vez es lo que quieres. Prefiere `./` o un token.

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

Ambas formas siguen funcionando cuando mueves la carpeta portable. Una ruta fija como `C:\tools\notes` no
viaja con ella. En modo portable, el diálogo Editar app marca los valores absolutos de `cwd` y `url` con
una insignia "no portable". Consulta [Modo portable](/es/data/portable-mode/).

## Entorno

`env` es un objeto de cadenas. El diálogo lo edita como una `KEY=VALUE` por línea; divide cada línea en
el primer `=`, recorta ambos lados e ignora las líneas que no lo tienen.

En el diálogo:

```text
PORT=8091
NODE_ENV=development
```

En `apps.json`, como la clave `env` de la entrada:

```json title="apps.json (one entry)"
{ "id": "habits", "name": "Habits", "group": "Web apps", "type": "web", "command": "python app.py",
  "env": { "PORT": "8091", "NODE_ENV": "development" } }
```

- El comando iniciado hereda el entorno de Moonpool más `env`. Las entradas de `env` tienen prioridad.
- `env` también se aplica a `stopCommand`.
- Los valores se usan tal como están escritos: Moonpool no expande `{MP_HOME}` ni `%VAR%`.
- Moonpool apunta su propio WebView2 a una carpeta de perfil privada mediante
  `WEBVIEW2_USER_DATA_FOLDER`. Las apps iniciadas no heredan eso. Si habías definido la variable tú
  mismo antes de iniciar Moonpool, reciben tu valor; si no, no está definida. Una entrada de `env` aún
  puede sobrescribirla.

## Carpeta de trabajo

El comando y `stopCommand` se ejecutan en `cwd`. Cuando se omite `cwd`, el comando se ejecuta en la
carpeta de trabajo del propio Moonpool, así que define `cwd` para cualquier cosa que use rutas relativas.
