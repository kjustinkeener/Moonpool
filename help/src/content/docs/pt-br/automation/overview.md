---
title: "Automatize o Moonpool com scripts e agentes de IA"
description: "As três formas de controlar um Moonpool em execução a partir de scripts e agentes de IA (MCP, linha de comando e verbos de controle) e o que cada uma pode alterar."
---

O Moonpool pode ser controlado sem tocar na janela dele. Há três superfícies, todas atendidas pelo mesmo
Moonpool residente (a instância da bandeja, chamada aqui de hub).

Cada cópia do Moonpool é o próprio hub: a instalada e cada cópia portátil rodam de forma independente,
cada uma com o próprio canal de controle. Uma superfície sempre chega à cópia cujo `moonpool.exe` ela usa.
Veja [Modo portátil](/pt-br/data/portable-mode/#várias-cópias-ao-mesmo-tempo).

| Superfície | O que é | Referência |
| --- | --- | --- |
| Servidor MCP | `moonpool.exe mcp`, um servidor [MCP](https://modelcontextprotocol.io) stdio que um host de IA inicia. | [Configuração do MCP](/pt-br/automation/mcp-setup/), [Ferramentas MCP](/pt-br/automation/mcp-tools/) |
| Linha de comando | `moonpool.exe <verb> [args]`. Uma segunda execução da mesma cópia entrega o verbo ao hub dela pelo canal de controle e termina. | [Linha de comando](/pt-br/automation/command-line/) |
| Canal de controle | Um pipe nomeado, `\\.\pipe\moonpool` (`\\.\pipe\moonpool-<id>` para uma cópia portátil), no Windows e um socket Unix no Linux, que atende uma requisição JSON por linha. | [Verbos de controle](/pt-br/automation/control-verbs/) |

## Como se relacionam

- O hub é dono de tudo: iniciar apps, os logs de sessão, o `apps.json`.
- O servidor MCP é um cliente do hub, não uma segunda cópia dele. A maioria das chamadas de ferramentas é
  encaminhada ao hub pelo canal de controle, e a resposta volta como resultado da ferramenta. As exceções:
  `moonpool_bootup_launcher` inicia o `moonpool.exe` por conta própria; `moonpool_app_output` e as
  ferramentas de configuração pedem ao hub que grave um arquivo e depois o leem;
  `moonpool_launcher_paths` acrescenta os caminhos do próprio processo MCP aos do hub.
- Se um hub está em execução, isso é decidido fazendo ping nesse canal, não procurando um processo. Um hub
  que responde está em execução; um pipe ou socket ausente significa que não está.
- Todas as superfícies executam os mesmos manipuladores da janela, então um verbo faz o que o clique
  correspondente faz.
- Se nenhum hub estiver em execução, as ferramentas que agem sobre ele, incluindo `moonpool_list_apps`,
  recusam com "Moonpool is not running" (o Moonpool não está em execução). Não há lista desatualizada.
  `moonpool_bootup_launcher` o inicia. Se algo ocupar o canal mas não responder em alguns segundos, o erro
  diz que um processo do Moonpool pode estar travado.
- O servidor MCP não recorre mais a controlar um hub anterior ao canal de controle. Atualize essa cópia, ou
  saia dela e inicie-a de novo.

## O que pode alterar as coisas

| Pode alterar | Superfícies |
| --- | --- |
| Iniciar, parar ou reiniciar um app | MCP, linha de comando, pipe |
| Reescrever o `apps.json` | MCP (`moonpool_write_config`, `moonpool_restore_config`), linha de comando, pipe |
| Encerrar o Moonpool | MCP (`moonpool_shutdown_launcher`), linha de comando (`quit`), pipe |
| Encerrar o processo auxiliar MCP de um app | MCP (`moonpool_stop_mcp_server`), pipe (`stop-mcp`) |
| Recarregar o `apps.json`, buscar os ícones de novo, mostrar a janela | MCP (`moonpool_reload_config`, `moonpool_refresh_app_icons`, `moonpool_raise_launcher`), linha de comando (`reload`, `refresh-icons`, `show`), pipe |
| Abrir uma janela ou uma aba de terminal | pipe (`open-window`) |
| Limpar os avistamentos lembrados de auxiliares MCP | MCP (`moonpool_reset_mcp_seen`), pipe (`reset-mcp-seen`) |

Ferramentas somente leitura: `moonpool_list_apps`, `moonpool_app_output`, `moonpool_read_config`,
`moonpool_launcher_paths`, `moonpool_window_state`, `moonpool_screenshot`.

## Propriedades de segurança

- **As gravações de configuração são protegidas.** Uma gravação deve levar o token de versão da última
  leitura, um token desatualizado é rejeitado, e o novo `apps.json` é validado antes de qualquer coisa ser
  gravada. Uma gravação rejeitada deixa o `apps.json` intacto. Veja [Ferramentas MCP](/pt-br/automation/mcp-tools/#configuração).
- **Os ids de app são restritos.** O servidor MCP aceita apenas letras, dígitos, `.`, `_` e `-`, e nunca um
  `-` inicial, para que um id não possa ser lido como uma opção de linha de comando.
- **As capturas de tela são só do Moonpool.** `moonpool_screenshot` captura uma das seis janelas do próprio
  Moonpool (`main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`), nunca a tela nem outro
  app. O PNG é montado na memória e devolvido em linha; o Moonpool não o salva em um arquivo.
- **Sem autenticação no canal.** O Moonpool não adiciona login nem token ao pipe ou socket de controle.
  Qualquer processo que consiga abri-lo pode enviar verbos. No Linux, o arquivo do socket é
  criado com o modo `0600`, então só o seu próprio usuário consegue.
- **Hosts em sandbox são detectados.** Se o servidor MCP perceber que está rodando dentro de uma sandbox
  empacotada (Store/MSIX), onde veria uma cópia privada dos arquivos do Moonpool, as ferramentas que leem
  ou gravam arquivos (`moonpool_app_output`, `moonpool_read_config`, `moonpool_write_config`,
  `moonpool_restore_config`) retornam um erro explicando por quê, em vez de dados desatualizados. As
  ferramentas que só usam o canal de controle não são bloqueadas. Veja
  [Configuração do MCP](/pt-br/automation/mcp-setup/#hosts-em-sandbox).

## Plataforma

O canal de controle existe em todas as plataformas: um pipe nomeado no Windows, um socket Unix no Linux (local em [Verbos de controle](/pt-br/automation/control-verbs/#onde-ele-escuta)). Apenas
`screenshot` (e portanto `moonpool_screenshot`) é exclusivo do Windows; no Linux retorna
"not supported on this platform" (não suportado nesta plataforma). Os verbos da linha de comando funcionam
em todas as plataformas.

## Veja também

- [Agentes de IA: início rápido](/pt-br/automation/quick-start/)
- [Configuração do MCP](/pt-br/automation/mcp-setup/)
