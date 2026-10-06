---
title: "Escolha um tipo de app: web, desktop, static ou cli"
description: "Saiba como os apps web, desktop, static e cli são iniciados no Moonpool, como o estado em execução é detectado em cada um e o que o botão Parar faz por padrão."
---

`type` decide quais campos importam e o que Parar faz por padrão.

| | `web` | `desktop` | `static` | `cli` |
| --- | --- | --- | --- | --- |
| Precisa de | `command` | `command` | `url` | `command` |
| Normalmente também | `port`, `url` | `processName` | `command` e `port`, se ele mesmo se serve | `cwd` |
| Iniciar | Executa `command` em uma aba de terminal | Executa `command` em uma aba de terminal | Sem `command`: abre a `url` no navegador. Com um: executa-o em uma aba de terminal | Executa `command` em uma aba de terminal |
| `killMode` padrão | `port` | `processName` | `none` | `none` |

![O diálogo Editar app de um app web: type definido como web com uma descrição de uma linha e um campo port preenchido](../../../../assets/screenshots/edit-app-type-and-port.png)

1. O seletor `type`. A linha de dica descreve o que esse tipo faz.
2. O campo `port`. Em um app `web`, "em execução" depende de essa porta responder.

## Como se decide o estado em execução

O Moonpool verifica a cada poucos segundos. Um app está em execução se qualquer uma destas condições for
verdadeira, seja qual for o tipo:

- `processName` está definido e existe um processo com esse nome. Os processos auxiliares `<exe> mcp` do
  próprio Moonpool não são contados.
- `port` está definido e responde no localhost.
- O Moonpool o iniciou, ele não tem nem `port` nem `processName` e o processo do terminal ainda está vivo.

Assim, um app `cli` está em execução enquanto o comando roda, e um app `web` sem `port` se comporta da
mesma forma. Uma entrada `static` com apenas uma `url` não tem nada a acompanhar e nunca aparece como em
execução.

## web

Um servidor local. Defina `port` para que "em execução" reflita se o servidor está respondendo, e `url`
junto com `openBrowser` para abri-lo quando ele subir.

## desktop

Um app nativo. Defina `processName` como o nome do executável para que "em execução" sobreviva ao
desacoplamento da janela do comando que a iniciou. O Parar padrão encerra todos os processos com esse
nome.

## static

Uma página. Com apenas uma `url`, Iniciar e Reiniciar a abrem no navegador e Parar não faz nada. URLs
`http://`, `https://`, `mailto:` e `file://` são abertas, então uma página local funciona:

```json title="apps.json"
{ "id": "csv", "name": "CSV dashboard", "group": "Docs", "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html" }
```

Páginas que precisam de um servidor (PHP, ou qualquer uma que busque arquivos locais) precisam de um
`command` que inicie um e de um `port` para acompanhá-lo. Veja os
[exemplos](/pt-br/apps/examples/).

## cli

Uma ferramenta. O `command` roda em uma aba de terminal em `cwd`, e o app deixa de estar em execução
quando o comando termina. Para um shell que continue aberto, faça do comando um shell, por exemplo este
`command`:

```text title="command"
pwsh -NoLogo -NoProfile -NoExit -Command python run.py --flag
```

Evite aspas duplas aninhadas em `command`: o wrapper `cmd /c` as estraga.

![A aba de terminal de um app cli mostrando a saída de um comando do PowerShell e um prompt aberto abaixo dela](../../../../assets/screenshots/terminal-cli-output.png)

## O que o clique faz

Clicar no nome de um app só abre a aba de terminal dele. Use os controles Iniciar, Parar e Reiniciar para
executá-lo. Veja [Estados do app](/pt-br/support/glossary/#estados-do-app).
