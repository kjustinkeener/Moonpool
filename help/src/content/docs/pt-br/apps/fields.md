---
title: "Todos os campos do apps.json: tipo, padrão e o que fazem"
description: "Consulte cada chave de uma entrada do apps.json com o tipo, o valor padrão e os tipos de app que a usam, com os mesmos nomes do diálogo Editar app."
---

O diálogo Editar app mostra os mesmos campos com os mesmos nomes. Os campos que não se aplicam ao tipo
selecionado ficam esmaecidos no diálogo, mas ainda são salvos, com uma exceção:
`stopCommand` só é salvo enquanto `killMode` for `command`.

![O diálogo Editar app de name até stopCommand, com o seletor killMode destacado; campos sem uso, como processName e stopCommand, ficam esmaecidos](../../../../assets/screenshots/edit-app-dialog.png)

1. O seletor `killMode`. Os campos que ele não usa permanecem esmaecidos.

| Campo | Tipo | Obrigatório | Usado por | O que faz |
| --- | --- | --- | --- | --- |
| `id` | string | sim | todos | Chave única. Letras, dígitos, `.`, `_`, `-`, sem começar com `-`. Veja [Visão geral](/pt-br/apps/apps-json/#o-id). |
| `name` | string | sim | todos | Rótulo na barra lateral. Não pode estar em branco. |
| `group` | string | sim | todos | Título da barra lateral sob o qual o app é listado. Não pode estar em branco em uma edição manual; o diálogo salva um grupo em branco como `Apps`. Qualquer texto; um nome novo cria um grupo novo. |
| `type` | string | sim | todos | `web`, `desktop`, `static` ou `cli`. Veja [Tipos de app](/pt-br/apps/types/). |
| `command` | string | todos exceto `static` | todos | Executado em um terminal para iniciar o app, por `cmd /c` no Windows e `$SHELL -c` nos demais (`/bin/sh` se `SHELL` não estiver definido). Opcional para `static`. |
| `cwd` | string | não | todos com um `command` | Pasta em que o comando roda. O padrão é a pasta de trabalho do próprio Moonpool. Aceita tokens e `./`. Veja [Caminhos e ambiente](/pt-br/apps/paths-and-environment/). |
| `port` | integer, de 1 a 65535 | não | qualquer | Em execução enquanto algo responde nesta porta no localhost (IPv4 ou IPv6). Lido por `killMode` `port`. |
| `processName` | string | não | qualquer, principalmente `desktop` | Em execução enquanto existir um processo com este nome. Sem diferenciar maiúsculas de minúsculas, com ou sem `.exe`, então `my-app` corresponde a `my-app.exe`. No Linux, 15 caracteres ou menos. Lido por `killMode` `processName`. |
| `mcpProcessName` | string | não | qualquer com um `processName` | Padrão com curingas para o nome de processo do servidor MCP deste app. `*` corresponde a qualquer sequência de caracteres, `?` a um caractere. Sem diferenciar maiúsculas de minúsculas, comparado com o nome inteiro, e `.exe` é opcional. Um processo correspondente conta como o servidor MCP do app (a subfila MCP da barra lateral) e não precisa de `mcp` como primeiro argumento. Veja [mcpProcessName](#mcpprocessname). |
| `url` | string | somente `static` | `web`, `static` | Página a abrir. Somente URLs `http://`, `https://`, `mailto:` e `file://` são abertas. |
| `openBrowser` | boolean, padrão `false` | não | qualquer tipo com uma `url` (o diálogo o esmaece para `desktop` e `cli`) | Abre a `url` automaticamente quando o Moonpool detecta que o app está ativo (veja abaixo). |
| `killMode` | string | não | todos | Limpeza extra ao Parar e Reiniciar: `processName`, `port`, `command` ou `none`. Veja [Parar e reiniciar](/pt-br/apps/stop-and-restart/). |
| `stopCommand` | string | não | `killMode` `command` | Comando executado ao Parar. Ignorado em todos os outros modos. |
| `env` | object of strings | não | todos | Variáveis de ambiente extras. O diálogo as edita como uma `KEY=VALUE` por linha. |
| `icon` | string | não | todos | Imagem da barra lateral: um caminho de arquivo, uma URL `http(s)` ou um URI `data:`. Defina-o em **Definir ícone...** no menu de contexto do app ou manualmente. |
| `note` | string | não | todos | Dica ao passar o mouse sobre o app na barra lateral. |

Uma entrada usando `env` e `killMode`:

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000", "NODE_ENV": "development" },
  "killMode": "port"
}
```

Veja [Encontrar e encerrar o processo que usa uma porta](/pt-br/guides/find-and-kill-process-using-port-windows/)
para saber como `port` e `killMode` funcionam juntos.

## mcpProcessName

Por padrão, o Moonpool trata um processo como o servidor MCP do app quando o nome dele corresponde a
`processName` e o primeiro argumento é `mcp`, como `notes-app.exe mcp`. Defina `mcpProcessName` quando o
servidor roda com outro nome: um app que acompanha um exe enquanto o servidor MCP dele é outro
(`mog.exe mcp`), ou uma cópia renomeada do servidor.

O valor é um padrão com curingas. `*` corresponde a qualquer sequência de caracteres (inclusive nenhuma) e
`?` corresponde a exatamente um. Ele é comparado, sem diferenciar maiúsculas de minúsculas, com o nome
inteiro do processo, e um padrão sem `.exe` também corresponde ao nome com `.exe`. Um valor vazio conta
como não definido.

```json
{
  "id": "destiny",
  "name": "Destiny",
  "group": "Desktop apps",
  "type": "desktop",
  "processName": "destiny",
  "mcpProcessName": "destiny-mcp-*"
}
```

Isso corresponde a uma cópia renomeada como `destiny-mcp-2706210170.exe`. Um processo que corresponde a
`mcpProcessName` é o servidor, tenha sido iniciado ou não com `mcp`, e nunca conta como o próprio app em
execução. Se o padrão também corresponder ao próprio `processName` (por exemplo `destiny*`), o Moonpool
ainda exige o argumento `mcp`, para que o app real nunca seja confundido com o servidor MCP dele. Veja
[Configuração do MCP](/pt-br/automation/mcp-setup/#apps-que-têm-o-próprio-servidor-mcp).

## openBrowser

O Moonpool abre a `url` uma vez, quando um app que o Moonpool iniciou aparece pela primeira vez como em
execução. Para detectar isso é preciso um `port` ou `processName`. Sem nenhum dos dois, "em execução"
significa apenas que o processo do terminal está vivo, e o navegador não é aberto automaticamente.
Desligue `openBrowser` se o seu comando já abre um navegador. Uma entrada `static` sem comando abre a
`url` sempre que você aperta Iniciar, independentemente de `openBrowser`.

Dois apps configurados com a mesma `port` são sinalizados na barra lateral.

## Ícones

O ícone de um app é o primeiro destes que existir:

1. O campo `icon`.
2. `icons\<id>.<ext>` na pasta de configuração, por exemplo `icons\site.png`.
3. Um arquivo de ícone na própria pasta do app (o `cwd` dele, ou a pasta de uma `url` `file:///`).
4. Para `desktop`, o ícone do `.exe` compilado ou em execução.
5. Para `web` e `static`, o `/favicon.ico` do site, assim que o servidor estiver ativo.
6. Um glifo do tipo.

A maioria dos apps não precisa de nenhuma configuração de ícone.
