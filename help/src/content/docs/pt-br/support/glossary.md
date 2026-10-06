---
title: "Glossário do Moonpool: apps, estados, arquivos e configurações"
description: "Definições simples das palavras que a ajuda do Moonpool usa para as partes, os estados dos apps, os arquivos e as configurações."
---

## Apps

| Termo | Significado |
| --- | --- |
| app | Uma coisa que o Moonpool gerencia: um servidor de desenvolvimento, um app desktop, uma página ou um comando. |
| entrada | O registro de um app no `apps.json`. Usado só ao falar do JSON. |
| linha do app | A linha de um app na barra lateral, com o ponto de status e os controles. |
| grupo | O título da barra lateral sob o qual um app é listado, a partir do campo `group`. |
| tipo | `web`, `desktop`, `static` ou `cli`. Decide quais campos importam. Veja [Tipos de app](/pt-br/apps/types/). |
| id | A chave permanente de um app, usada em nomes de arquivo, comandos e ferramentas de agentes. Veja [O id](/pt-br/apps/apps-json/#o-id). |

## Estados do app

| Estado | Significado |
| --- | --- |
| iniciando | O Moonpool iniciou o app mas ainda não o viu ativo. Ponto pulsando. |
| em execução | A `port` dele responde, o `processName` dele existe ou, sem nenhum dos dois definido, o terminal que o Moonpool iniciou ainda está vivo. Ponto fixo. Veja [Como se decide o estado em execução](/pt-br/apps/types/#como-se-decide-o-estado-em-execução). |
| parado | Nenhuma das anteriores. Ponto cinza. |
| gerenciado | O Moonpool o iniciou nesta sessão. Um app em execução que não é gerenciado foi iniciado de outra forma, e Sair o deixa em paz. |

Uma aba de terminal e um app em execução são coisas separadas. Clicar no nome de um app só abre a aba de
terminal dele; nunca inicia o app. Fechar uma aba nunca para o app.

## Janelas e partes

| Termo | Significado |
| --- | --- |
| hub | O processo residente do Moonpool e a janela principal dele. Os nomes das ferramentas o chamam de "launcher" (inicializador). |
| janela do hub | A janela principal: a barra lateral à esquerda, o painel CLI à direita. |
| bandeja | O ícone da bandeja do sistema e o menu dele (**Mostrar o Moonpool**, **Sair**). |
| barra lateral | O lado esquerdo da janela do hub: caixa de filtro, menu **...** e linhas de apps. |
| painel CLI | O lado direito da janela do hub, que contém as abas de terminal. |
| aba de terminal | O terminal de um app no painel CLI. |
| subfila MCP | Uma linha esmaecida sob um app que mostra o próprio processo auxiliar `<exe> mcp` dele. |
| editor de apps | O diálogo Adicionar app e Editar app. |

## Arquivos e pastas

| Termo | Significado |
| --- | --- |
| pasta de configuração | A pasta que contém o `apps.json` e os demais arquivos do Moonpool. O token `{MP_DATA}`. Veja [Onde fica a configuração](/pt-br/apps/apps-json/#onde-fica-a-configuração). |
| `{MP_HOME}` | A pasta do Moonpool: `%USERPROFILE%\.moonpool` no modo instalado, a pasta `.moonpool\` de uma cópia portátil, a pasta de configuração no Linux. |
| sessão | Uma execução do hub, do início até Sair. |
| log de sessão | O arquivo que contém tudo o que um app imprimiu durante uma sessão, em `cli-output\`. Veja [Logs](/pt-br/data/logs/). |
| `moonpool.log` | O log de depuração do próprio Moonpool, gravado apenas com **Gravar informações de depuração em um arquivo** ligado. |
| dump | Uma cópia em texto simples de um log de sessão feita pelo verbo `dump`. |
| instantâneo | Uma cópia de um `apps.json` bom em `apps.json.history\`. Veja [Backup e recuperação](/pt-br/data/backup-and-recovery/). |

## Modos

| Termo | Significado |
| --- | --- |
| instalado | Um Moonpool em `%USERPROFILE%\.moonpool`, com atalho no menu Iniciar e entrada em Adicionar ou remover programas. Somente Windows. |
| portátil | Um Moonpool em uma pasta `.moonpool\` que você escolheu, marcado por um arquivo `moonpool.portable`. Veja [Modo portátil](/pt-br/data/portable-mode/). |
| cópia | Uma pasta do Moonpool, instalada ou portátil. Cada cópia roda por conta própria. |

## Parar e automação

| Termo | Significado |
| --- | --- |
| `killMode` | A etapa extra que Parar executa depois de encerrar o terminal do app. Veja [Parar e reiniciar](/pt-br/apps/stop-and-restart/). |
| `stopCommand` | O comando que Parar executa quando `killMode` é `command`. |
| `processName` | O nome de processo que o Moonpool observa, e que encerra no modo `processName`. |
| canal de controle | O pipe nomeado (Windows) ou socket Unix (Linux, macOS) em que o hub responde. Veja [Verbos de controle](/pt-br/automation/control-verbs/). |
| verbo | Uma palavra de comando como `launch` ou `reload`, dada na linha de comando ou no canal de controle. |
| ticket | Uma chave que você anexa com `--ticket` para ler o resultado de um comando em `state.json`. |
| token | O carimbo de versão do `apps.json` que uma gravação de configuração deve levar. |
| auxiliar MCP (shim) | Um processo `<exe> mcp` que um host de IA inicia para alcançar as ferramentas próprias de um app. |
