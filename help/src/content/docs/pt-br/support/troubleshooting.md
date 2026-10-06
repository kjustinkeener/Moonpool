---
title: "Solução de problemas do Moonpool: bandeja, apps que não iniciam, atualizações"
description: "Resolva os problemas comuns do Moonpool pelo que você vê: ícone da bandeja ausente, apps que não iniciam, pontos de status errados, atualizações e erros de MCP."
---

Encontre o sintoma e siga a solução. O texto entre aspas é o que o Moonpool mostra. Para consultar uma
mensagem exata, veja [Mensagens de erro explicadas](/pt-br/support/error-messages/).

## Não vejo o ícone da bandeja

- **Windows.** O ícone pode estar na área de ícones ocultos. Clique na seta **^** à direita da barra de
  tarefas. Arraste o ícone para a barra de tarefas para mantê-lo visível.
- **Linux com GNOME padrão.** O GNOME não mostra ícones de bandeja sem a extensão AppIndicator. Veja
  [Linux](/pt-br/platforms/linux/#bandeja-no-gnome).
- **Configurações.** **Mostrar na bandeja** pode estar desligado. Abra o hub pela barra de tarefas ou pelo
  menu Iniciar e ligue-o de novo em [Configurações](/pt-br/using/settings/).

## O instalador mostra um erro

| Mensagem | O que fazer |
| --- | --- |
| `Falha na instalação: <error>` | O texto após os dois-pontos nomeia a etapa que falhou, por exemplo `copy exe: ...`. Se um arquivo estiver em uso, saia de qualquer Moonpool executando a partir de `%USERPROFILE%\.moonpool` e tente de novo. |
| `target folder does not exist` | A pasta que você escolheu para uma cópia portátil não existe mais. Escolha uma pasta que exista. |
| `that folder already has a .moonpool with an exe of this name that isn't a portable Moonpool - pick an empty folder` | Escolha uma pasta vazia, ou remova antes essa pasta `.moonpool`. |

## Aparece Windows protected your PC ao executar o instalador

É o Windows SmartScreen, porque o `moonpool.exe` não tem assinatura de código. Clique em **More info**
(Mais informações) e depois em **Run anyway** (Executar assim mesmo). Veja
[Windows protected your PC](/pt-br/support/windows-protected-your-pc/).

## A janela do Moonpool está em branco ou nunca abre no Windows

O Runtime do Microsoft Edge WebView2 pode estar ausente. Veja
[Runtime do WebView2 ausente](/pt-br/support/webview2-runtime-missing/).

## Um app não inicia

1. Clique no nome do app para abrir a aba de terminal dele e leia a saída. Um agente pode ler o mesmo
   texto com `moonpool_app_output`.
2. Confira `cwd`. Uma pasta inexistente, ou um caminho relativo sem `./`, é a causa habitual. Veja
   [Caminhos e ambiente](/pt-br/apps/paths-and-environment/).
3. Confira `command`. Execute-o manualmente em um terminal em `cwd`. No Windows, evite aspas duplas
   aninhadas; o `cmd /c` as estraga.
4. Ligue **Gravar informações de depuração em um arquivo** em Configurações e inicie de novo. O
   `moonpool.log` registra o comando e a pasta exatos. Veja [Logs](/pt-br/data/logs/).

| Mensagem | Significado |
| --- | --- |
| `already running` | O Moonpool já tem um terminal para este app. Pare-o primeiro, ou use Reiniciar. |
| `stopped during launch` | Parar foi pressionado enquanto a inicialização ainda estava em andamento. |
| `did not reach running in time` | De um script ou agente: o app não apareceu como em execução em 25 segundos. Confira a `port` ou o `processName` dele, e a saída. |

## O ponto de status está errado

O Moonpool decide se está em execução a partir de `port`, depois `processName` e depois se o próprio
terminal ainda está vivo. Veja [Como se decide o estado em execução](/pt-br/apps/types/#como-se-decide-o-estado-em-execução).

- **Nunca fica fixo.** A `port` de um app `web` não responde, ou o `processName` de um app `desktop` não
  corresponde. No Linux, `processName` deve ter 15 caracteres ou menos.
- **Fica cinza logo após iniciar.** Um app `cli` deixa de estar em execução quando o comando termina. Use
  um shell com `-NoExit` se quiser que ele continue aberto.
- **Um app `static` nunca aparece como em execução.** Isso é esperado em uma entrada com apenas uma `url`.
- **Aparece em execução embora você não o tenha iniciado.** Outra coisa está usando essa porta ou esse nome
  de processo. O Moonpool o mostra como em execução, mas não "gerenciado pelo Moonpool".

## Error: listen EADDRINUSE ou "Port 5173 is in use"

Outra coisa já está escutando na porta que o seu servidor quer. Encontre-a e encerre-a, ou defina `port`
no app para que Parar a libere. Veja
[Resolver EADDRINUSE e "Port 5173 is in use"](/pt-br/support/port-already-in-use/) e
[Encontrar e encerrar o processo que usa uma porta](/pt-br/guides/find-and-kill-process-using-port-windows/).

## Dois apps usam a mesma porta

Uma linha de aviso aparece no fim do menu **...**, por exemplo `port 3000: App A / App B`. Altere a `port`
de um dos apps (e o `env` dele, se ele lê `PORT`). Veja
[Aviso de conflito de portas](/pt-br/using/hub-window/#aviso-de-conflito-de-portas).

## O app continua em execução depois de Parar

De um script ou agente, o erro é `still running after stop` (após 15 segundos).

- O app sobrevive ao terminal dele. Defina `killMode` como `port` ou `processName`. Veja
  [Parar e reiniciar](/pt-br/apps/stop-and-restart/).
- Um app Docker no Windows: use `killMode` `command` com um `stopCommand` como
  `docker compose stop app`. Nunca `port`.

## apps.json tem um erro

A barra lateral mostra um banner, "apps.json tem um erro; mostrando a última lista que carregou." ou, na
inicialização, "apps.json tem um erro, então nenhum app foi carregado." Os salvamentos pelo Moonpool ficam
pausados até o arquivo carregar de novo.

Erros típicos:

```text
apps.json entry 2 (site) requires a command
apps.json entry 3 has invalid id "my app"; use letters, digits, '.', '_', and '-' without a leading '-'
duplicate app id "site"
apps.json entry 4 (api) has invalid port 0
```

1. Escolha **Editar apps.json** no banner, corrija a entrada, salve e depois **Recarregar** (F5).
2. Ou volte para uma cópia boa recente. Veja
   [Backup e recuperação](/pt-br/data/backup-and-recovery/#reverter-o-appsjson).

A lista completa de regras está em [Validação](/pt-br/apps/apps-json/#validação).

Se uma configuração não puder ser alterada e a mensagem terminar com `Repair settings.json and restart
Moonpool before changing settings`, corrija ou exclua o `settings.json` na pasta de configuração e inicie
o Moonpool de novo. Excluí-lo redefine todas as configurações para o padrão.

## Minha edição não surtiu efeito

- As edições manuais precisam de **Recarregar** (ou F5). O Moonpool não observa o arquivo.
- Recarregar não reinicia os apps em execução. Reinicie o app para usar um `command`, `cwd` ou `env`
  alterado.
- Um agente pode estar editando outro `apps.json`. Peça que chame `moonpool_launcher_paths` e compare a
  pasta do hub com a dele. Com várias cópias do Moonpool, confira qual cópia você está editando.

## Os apps de exemplo sumiram

Os exemplos são gravados apenas quando não existe `apps.json`. Para recuperá-los, veja
[Voltar aos exemplos](/pt-br/data/backup-and-recovery/#voltar-aos-exemplos), ou copie as entradas de
[Painéis de exemplo](/pt-br/getting-started/example-dashboards/#os-apps-de-exemplo-só-aparecem-na-primeira-execução).

## Uma atualização falhou

O banner mostra `Falha na atualização: <error>`. Veja
[Quando uma atualização falha](/pt-br/data/updating/#quando-uma-atualização-falha).

## Um link web não abre

`refusing to open non-web url: <url>` significa que a `url` não é `http://`, `https://`, `mailto:` nem
`file://`. Corrija a `url`.

## Erros de MCP e de scripts

| Mensagem | O que fazer |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Inicie o Moonpool, ou deixe o agente chamar `moonpool_bootup_launcher`. |
| `frontend not loaded` | A janela do hub não terminou de carregar. Aguarde um instante e tente de novo. |
| `stale token: ...` | O `apps.json` mudou desde que o agente o leu. Leia-o de novo e depois grave. |
| `rejected invalid manifest: ...` | O novo `apps.json` falhou na validação. O arquivo não foi alterado. |
| `... A Moonpool process may be hung ...` | Algo ocupa o canal de controle sem responder. Saia do Moonpool pela bandeja, ou encerre o processo, e inicie-o de novo. |

Mais em [Configuração do MCP](/pt-br/automation/mcp-setup/#observações) e
[Ferramentas MCP](/pt-br/automation/mcp-tools/).

## Problemas com a janela

- **Fora da tela.** O Moonpool ignora uma posição salva que não está em nenhum monitor conectado. Se a
  janela continuar perdida, saia do Moonpool e exclua o `window-state.json` na pasta de configuração.
- **O zoom ficou grande ou pequeno demais.** Ctrl + roda sobre o hub o altera. Veja
  [Atalhos e zoom](/pt-br/using/keyboard-shortcuts/#zoom).
- **Configurações abre atrás do hub.** Desligue ou ligue **Sempre visível** em Configurações. Ele se aplica
  a todas as janelas do Moonpool, então elas ficam na mesma camada.

## Onde estão os logs?

Veja [Logs](/pt-br/data/logs/).

## Backup, redefinição ou desinstalação

Veja [Backup e recuperação](/pt-br/data/backup-and-recovery/) e
[Desinstalação](/pt-br/getting-started/install/#desinstalação).

## Perguntas frequentes

**Fechar a janela para os meus apps?**
Por padrão, fechar encerra o Moonpool, e no Windows encerrá-lo para os apps que ele iniciou. Ative
**Fechar para a bandeja** para manter o Moonpool em execução quando você fechar a janela. Veja
[Bandeja, fechar e minimizar](/pt-br/using/tray-and-closing/).

**Posso executar o Moonpool duas vezes?**
Um por pasta. Iniciar a mesma cópia de novo traz a janela dela de volta. A cópia instalada e as cópias
portáteis podem rodar lado a lado. Veja
[Modo portátil](/pt-br/data/portable-mode/#várias-cópias-ao-mesmo-tempo).

**O Moonpool envia dados para algum servidor?**
Só para procurar atualizações: ele busca o arquivo de versão (`update.json`) no GitHub na inicialização
(se **Procurar atualizações ao iniciar** estiver ligado) e quando você pressiona **Procurar
atualizações**. Todo download é verificado com a chave de assinatura do Moonpool antes de ser usado.

**Qual shell executa os meus comandos?**
`cmd /c` no Windows, `$SHELL -c` no Linux e no macOS.

**Onde coloco segredos?**
Os valores de `env` são guardados em texto simples no `apps.json`. Prefira um arquivo que o próprio app
leia, ou uma variável já definida no seu ambiente de usuário, que os apps iniciados herdam.

**O canal de controle é protegido?**
Ele não tem login nem token. Qualquer processo executando como você pode enviar comandos a ele. No Linux e
no macOS, o socket só pode ser lido pelo seu usuário. Veja
[Propriedades de segurança](/pt-br/automation/overview/#propriedades-de-segurança).
