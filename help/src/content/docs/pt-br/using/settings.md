---
title: "Altere as configurações do Moonpool: todas as opções de Configurações e Sobre"
description: "Lista completa dos controles das janelas Configurações e Sobre do Moonpool, a chave do settings.json que cada um grava e como redefinir uma configuração."
---

Abra **Configurações** no menu "..." do hub. As alterações são salvas conforme você as faz. Esc fecha a
janela. Esta página é a lista completa de configurações. Cada uma é salva em `settings.json` sob a chave
indicada; o próprio arquivo é descrito em
[settings.json](/pt-br/data/settings-json/).

![Janela de Configurações: alternâncias e controles deslizantes na coluna da esquerda, opções de log na da direita](../../../../assets/screenshots/settings-window.png)

## Redefinir um controle

Clique com o botão direito em qualquer caixa de seleção, controle deslizante ou campo numérico para
redefinir só essa configuração para o padrão. A dica ao passar o mouse em cada controle informa isso. Os
seletores de Idioma e Tema não têm redefinição.

## Coluna da esquerda

| Controle | Chave | Padrão | O que faz |
| --- | --- | --- | --- |
| Idioma | `locale` | Automático (sistema) | Idioma do texto do próprio Moonpool. Aplica na hora. Veja [Temas, idioma e transparência](/pt-br/using/themes-and-language/). |
| Tema | nenhuma (armazenamento do navegador) | Automático (sistema) | Tema de cores. O botão abre um navegador de temas com uma prévia de cada tema; clicar em um o aplica na hora. Veja [Temas, idioma e transparência](/pt-br/using/themes-and-language/). |
| Fechar para a bandeja | `closeToTray` | desligado | Ligado: fechar a janela oculta o Moonpool na bandeja. Desligado: fechar encerra o app. |
| Minimizar para a bandeja | `minimizeToTray` | ligado | Ligado: minimizar oculta o Moonpool na bandeja e ele sai da barra de tarefas. Desligado: minimiza para a barra de tarefas. |
| Sempre visível | `alwaysOnTop` | desligado | Mantém todas as janelas do Moonpool acima das outras janelas. |
| Mostrar na bandeja | `showInTray` | ligado | Mantém o ícone da bandeja visível. |
| Mostrar na barra de tarefas | `showInTaskbar` | ligado | Mantém o botão da barra de tarefas visível. |
| Mostrar a barra de status de CPU/memória | `showStatusbar` | ligado | Barra ao vivo de CPU e memória na parte inferior do hub. |
| Mostrar processos MCP | `showMcpProcesses` | ligado | Mostra o processo MCP de um app como subitem MCP na barra lateral enquanto as ferramentas MCP dele estão em uso. |
| Transparência do fundo | `transparency` | 0% | Controle deslizante de 0 a 90 em passos de 5. Veja [Temas, idioma e transparência](/pt-br/using/themes-and-language/#transparência). |
| Procurar atualizações ao iniciar | `checkOnStartup` | ligado | Consulta o GitHub ao abrir em busca de uma versão mais nova e mostra um banner se encontrar. Veja [Atualização](/pt-br/data/updating/). |

### Bloqueio de bandeja e barra de tarefas

Pelo menos um entre **Mostrar na bandeja** e **Mostrar na barra de tarefas** precisa ficar ligado, senão
uma janela oculta não teria como voltar. Quando só um está ligado, a caixa dele fica desativada até você
ligar o outro de novo.

## Coluna da direita: logs

| Controle | Chave | Padrão | O que faz |
| --- | --- | --- | --- |
| Manter os logs de saída dos apps entre sessões | `cliLogging` | desligado | A saída do terminal da sessão em andamento é sempre mantida para as próprias abas. Ligado: os logs de sessões anteriores permanecem no disco em `cli-output\`, limitados pela configuração de retenção. Desligado: são apagados na próxima vez que esse app for iniciado. |
| Retenção de logs por app | `logRetentionMb` | 10 MB | Limite do total de logs de cada app. Mínimo 1. Desativado enquanto a alternância acima estiver desligada. O log da sessão atual conta para o limite, mas nunca é truncado nem apagado por ele. |
| Gravar informações de depuração em um arquivo | `debugLogging` | desligado | Registra em `moonpool.log` os carregamentos do `apps.json`, as inicializações e os erros. |

Sob cada grupo de logs, um campo de caminho mostra o local, com dois botões:

- **Abrir** abre a pasta no gerenciador de arquivos (**Abrir a pasta de logs CLI** para `cli-output\`, **Abrir o log** para `moonpool.log`).
- **Copiar** coloca o caminho na área de transferência (**Copiar o caminho da pasta de logs CLI**, **Copiar o caminho do arquivo de log**).

Os formatos dos arquivos de log, o separador de reinicialização e as regras de retenção estão em
[Logs](/pt-br/data/logs/).

Se uma caixa de seleção não puder ser salva, uma mensagem vermelha no topo da janela avisa e a caixa
volta ao estado anterior.

## Janela Sobre

Abra **Sobre** no menu "...".

![Janela Sobre com a linha de versão, links e os botões Procurar atualizações e Fechar](../../../../assets/screenshots/about-window.png)

Ela mostra:

- A versão e a data da compilação.
- Links para o site do projeto, o repositório do GitHub e o endereço de contato.
- **Procurar atualizações**. Se existir uma versão mais nova, ela a baixa, verifica e instala, e depois reinicia o Moonpool. Caso contrário, informa que você está na versão mais recente, ou o erro se a verificação falhou.
- Os créditos das bibliotecas com que o Moonpool é feito e o autor.

Esc a fecha. A janela Sobre acompanha em tempo real as configurações de tema, transparência e idioma.
