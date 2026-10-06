---
title: "Use caminhos, tokens MP_HOME e variáveis de ambiente nos apps"
description: "Use os tokens {MP_HOME} e {MP_DATA} e os caminhos relativos ./ nas entradas de apps, veja quais campos os expandem e defina env e a pasta de trabalho."
---

## Tokens

| Token | Expande para |
| --- | --- |
| `{MP_HOME}` | Portátil: a pasta que contém o `moonpool.exe` (a pasta `.moonpool\`). Instalado no Windows: `%USERPROFILE%\.moonpool`. Linux: `$XDG_CONFIG_HOME/Moonpool`, ou então `~/.config/Moonpool`, a mesma pasta que `{MP_DATA}`. |
| `{MP_DATA}` | A pasta de configuração, a que contém o `apps.json`. |

Um token que não pode ser resolvido é deixado como foi escrito.

## Quais campos expandem

| Campo | Tokens | `./` ou `.\` inicial |
| --- | --- | --- |
| `cwd` | sim | sim, ancorado a `{MP_HOME}` |
| `command` | sim | não |
| `stopCommand` | sim | não (roda em `cwd`, que é ancorado) |
| `url` | sim | não |
| `icon` | sim | sim, ancorado a `{MP_HOME}` |
| valores de `env`, `processName`, `note` | não | não |

Um caminho relativo sem `./` (como `apps\tool`) é deixado como está e é resolvido em relação à pasta de
trabalho do próprio Moonpool, o que raramente é o que você quer. Prefira `./` ou um token.

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

As duas formas continuam funcionando quando você move a pasta portátil. Um caminho fixo como
`C:\tools\notes` não viaja junto. No modo portátil, o diálogo Editar app marca os valores absolutos de
`cwd` e `url` com um selo "não portátil". Veja [Modo portátil](/pt-br/data/portable-mode/).

## Ambiente

`env` é um objeto de strings. O diálogo o edita como uma `KEY=VALUE` por linha; ele divide cada linha no
primeiro `=`, apara os dois lados e ignora linhas sem ele.

No diálogo:

```text
PORT=8091
NODE_ENV=development
```

No `apps.json`, como a chave `env` da entrada:

```json title="apps.json (one entry)"
{ "id": "habits", "name": "Habits", "group": "Web apps", "type": "web", "command": "python app.py",
  "env": { "PORT": "8091", "NODE_ENV": "development" } }
```

- O comando iniciado herda o ambiente do Moonpool mais `env`. As entradas de `env` têm prioridade.
- `env` também é aplicado ao `stopCommand`.
- Os valores são usados como foram escritos: o Moonpool não expande `{MP_HOME}` nem `%VAR%`.
- O Moonpool aponta o próprio WebView2 para uma pasta de perfil privada por meio de
  `WEBVIEW2_USER_DATA_FOLDER`. Os apps iniciados não herdam isso. Se você mesmo tinha definido a variável
  antes de iniciar o Moonpool, eles recebem o seu valor; caso contrário, ela fica indefinida. Uma entrada
  de `env` ainda pode sobrescrevê-la.

## Pasta de trabalho

O comando e o `stopCommand` rodam em `cwd`. Quando `cwd` é omitido, o comando roda na pasta de trabalho do
próprio Moonpool, então defina `cwd` para tudo o que usa caminhos relativos.
