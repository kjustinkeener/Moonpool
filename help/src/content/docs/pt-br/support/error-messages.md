---
title: "Mensagens de erro do Moonpool explicadas: already running, requires a command e mais"
description: "Consulte o texto exato das mensagens de erro do Moonpool, como already running, requires a command ou stale token, com o significado e a solução de cada uma."
---

Cole a mensagem que você vê na busca da página, ou percorra as tabelas. As mensagens são citadas como o
Moonpool as mostra. O texto entre `<colchetes angulares>` é substituído por um valor (um id de app, um
caminho ou um erro do sistema). Os sintomas que não são uma mensagem de erro estão em
[Solução de problemas e perguntas frequentes](/pt-br/support/troubleshooting/).

## Iniciar e parar um app

| Mensagem | Significado e solução |
| --- | --- |
| `already running` | O Moonpool já tem um terminal para este app. Pare-o primeiro, ou use Reiniciar. |
| `stopped during launch` | Parar foi pressionado enquanto a inicialização ainda estava em andamento. Inicie de novo. |
| `app has no launch command` | A entrada não tem `command`. Adicione um no editor de apps ou no `apps.json`. Só uma entrada `static` com uma `url` pode dispensá-lo. |
| `unknown app: <id>` | Nenhum app com esse `id` está carregado. Confira o id e, se você editou o `apps.json` manualmente, Recarregue. |
| `unknown app id: <id>` | O mesmo problema, informado a um script ou agente. Liste os apps com `moonpool_list_apps`. |
| `did not reach running in time` | De um script ou agente: o app não apareceu como em execução em 25 segundos. Confira `port` ou `processName` e leia a saída. Veja [O ponto de status está errado](/pt-br/support/troubleshooting/#o-ponto-de-status-está-errado). |
| `still running after stop` | Após 15 segundos, o app ainda aparece como em execução. Defina `killMode`. Veja [Parar e reiniciar](/pt-br/apps/stop-and-restart/). |
| `refusing to open non-web url: <url>` | A `url` não é `http://`, `https://`, `mailto:` nem `file://`. Corrija a `url`. |
| `[process exited]` | Não é um erro: o comando do app terminou. Mostrado na aba de terminal (em português: `[processo encerrado]`). |

## Validação do apps.json

O Moonpool rejeita um `apps.json` que quebra uma regra e mantém a última lista que carregou. `<n>` é a
posição da entrada no arquivo, contando a partir de 1.

| Mensagem | Solução |
| --- | --- |
| `apps.json entry <n> has invalid id "<id>"; use letters, digits, '.', '_', and '-' without a leading '-'` | Renomeie o `id`. |
| `duplicate app id "<id>"` | Duas entradas compartilham um `id`. Torne cada um único. |
| `apps.json entry <n> (<id>) has an empty name` | Preencha `name`. |
| `apps.json entry <n> (<id>) has an empty group` | Preencha `group`. |
| `apps.json entry <n> (<id>) has unknown type "<type>"` | `type` deve ser `web`, `desktop`, `static` ou `cli`. |
| `apps.json entry <n> (<id>) has invalid port 0` | `port` deve ser de 1 a 65535. |
| `apps.json entry <n> (<id>) requires a url` | Uma entrada `static` precisa de uma `url`. |
| `apps.json entry <n> (<id>) requires a command` | Todos os outros tipos precisam de um `command`. |

No editor de apps, salvar sem um nome mostra `o nome é obrigatório.`.
O texto do banner, "apps.json tem um erro; mostrando a última lista que carregou." ou "apps.json tem um
erro, então nenhum app foi carregado.", e como se recuperar estão em
[apps.json tem um erro](/pt-br/support/troubleshooting/#appsjson-tem-um-erro). Se o banner disser que os
salvamentos estão pausados, a mensagem termina com `Repair apps.json and reload it before saving from
Moonpool` (repare o apps.json e recarregue-o antes de salvar pelo Moonpool). A lista completa de regras
está em [Validação](/pt-br/apps/apps-json/#validação).

## Configurações, atualizações e o instalador

| Mensagem | Significado e solução |
| --- | --- |
| `... Repair settings.json and restart Moonpool before changing settings` | O `settings.json` está malformado. Corrija-o ou exclua-o e reinicie. Veja [settings.json](/pt-br/data/settings-json/#leitura-e-reparo). |
| `Falha na atualização: <error>` | O download ou a instalação de uma atualização falhou. Veja [Quando uma atualização falha](/pt-br/data/updating/#quando-uma-atualização-falha). |
| `Falha ao procurar atualizações: <error>` | A verificação de atualizações em Sobre falhou. O texto após os dois-pontos diz por quê. Tente de novo mais tarde. |
| `Falha na instalação: <error>` | O instalador parou na etapa nomeada após os dois-pontos, por exemplo `copy exe: ...`. Saia de qualquer Moonpool executando a partir de `%USERPROFILE%\.moonpool` e tente de novo. |
| `target folder does not exist` | A pasta escolhida para uma cópia portátil não existe mais. Escolha uma que exista. |

## MCP e scripts

| Mensagem | Significado e solução |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Inicie o Moonpool, ou deixe o agente chamar essa ferramenta. Para uma cópia portátil, a mensagem nomeia a cópia. |
| `frontend not loaded` | A janela do hub não terminou de carregar. Aguarde e tente de novo. |
| `invalid app_id: use only letters, digits, '.', '_', '-' (no leading '-')` | O agente passou um id que o servidor MCP não aceita. Use o id de `moonpool_list_apps`. |
| `stale token: apps.json changed since it was read ...` | Leia o `apps.json` de novo, reaplique a edição e depois grave. |
| `rejected invalid manifest: ...` | O novo `apps.json` falhou na validação (veja acima). O arquivo não foi alterado. |
| `no console output recorded for '<id>' (not launched this session)` | Foi pedido `moonpool_app_output` de um app que não rodou desde que o Moonpool iniciou. |

Mais em [Ferramentas MCP](/pt-br/automation/mcp-tools/) e
[Configuração do MCP](/pt-br/automation/mcp-setup/#se-as-ferramentas-não-funcionarem).

## Erros de outros programas

- [`Error: listen EADDRINUSE: address already in use :::3000` e `Port 5173 is in use`](/pt-br/support/port-already-in-use/)
- [`Windows protected your PC`](/pt-br/support/windows-protected-your-pc/)
- [Runtime do WebView2 ausente](/pt-br/support/webview2-runtime-missing/)
