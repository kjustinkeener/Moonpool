---
title: "Faça backup do Moonpool, reverta o apps.json e recupere uma configuração"
description: "O que incluir no backup, como reverter um apps.json ruim, voltar aos apps de exemplo, mover uma configuração para uma cópia portátil e o que a desinstalação remove."
---

Tudo o que o Moonpool guarda está em dois lugares: a pasta de configuração e a pasta de painéis.
Os caminhos de cada modo estão em [Onde fica a configuração](/pt-br/apps/apps-json/#onde-fica-a-configuração).

## A pasta de configuração

```text
moonpool-config\
  apps.json            seus apps                               incluir no backup
  apps.json.history\   os últimos 10 apps.json bons            incluir no backup (opcional)
  settings.json        configurações do app                    incluir no backup
  icons\               ícones personalizados, <id>.png etc.    incluir no backup
  cli-output\<id>\     logs de sessão                          descartável
  moonpool.log         log de depuração                        descartável
  state.json           instantâneo do status ao vivo           descartável
  dumps\               arquivos de dump e read-config          descartável
  mcp_seen.json        apps que tiveram um auxiliar MCP        descartável
  window-state.json    tamanho e posição da janela do hub      descartável
  AI-README.md         reescrito a cada inicialização          descartável
  webview\             perfil de navegador (Windows)           descartável
```

A pasta de painéis é `{MP_HOME}\dashboards`: `%USERPROFILE%\.moonpool\dashboards` no modo instalado,
`<sua pasta .moonpool>\dashboards` no portátil e `dashboards/` dentro da pasta de configuração no Linux.
Faça backup de tudo o que for seu nela. A pasta `examples` pertence ao Moonpool e é reescrita na
atualização.

O tema fica no armazenamento do navegador da janela, não em um arquivo que você possa copiar. Ele não
viaja com um backup; escolha-o de novo após uma restauração.

## Fazer backup

1. Saia do Moonpool, para que nenhum arquivo fique pela metade.
2. Copie `apps.json`, `settings.json` e `icons\` da pasta de configuração, e seus próprios arquivos de
   `dashboards\`.

```powershell frame="terminal"
$cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
Copy-Item "$cfg\apps.json", "$cfg\settings.json" D:\backup\
Copy-Item "$cfg\icons" D:\backup\ -Recurse
```

Para restaurar, saia do Moonpool, copie os arquivos de volta e inicie-o.

## Reverter o apps.json

Todo salvamento bem-sucedido, gravação de agente e restauração, e todo Recarregar que encontra conteúdo
alterado, copia o `apps.json` validado para `apps.json.history\`, mantendo os 10 mais recentes. Cada
arquivo é nomeado pelo momento em que foi feito, por exemplo `1767225600000.json`. Não existe
`apps.json.bak`.

- **Manualmente.** Copie um instantâneo sobre o `apps.json` e depois escolha **Recarregar**.

  ```powershell frame="terminal"
  $cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
  Copy-Item "$cfg\apps.json.history\<snapshot>" "$cfg\apps.json"
  ```

- **Por um script.** `moonpool.exe restore-config` lista os instantâneos;
  `moonpool.exe restore-config 1` restaura o mais recente. Veja
  [Linha de comando](/pt-br/automation/command-line/).
- **Por um agente.** `moonpool_restore_config`. Veja [Ferramentas MCP](/pt-br/automation/mcp-tools/#configuração).

Nada é restaurado automaticamente.

## Um arquivo quebrado

- **apps.json.** O Moonpool nunca sobrescreve um arquivo quebrado. Veja
  [Se o arquivo estiver com problema](/pt-br/apps/apps-json/#se-o-arquivo-estiver-com-problema).
- **settings.json.** Corrija-o, ou exclua-o para redefinir todas as configurações, e reinicie o Moonpool.
  Veja [settings.json](/pt-br/data/settings-json/#leitura-e-reparo).

## Voltar aos exemplos

O Moonpool grava seus apps de exemplo apenas quando não há `apps.json`. Para recomeçar, saia do Moonpool
(ou deixe-o em execução), renomeie ou exclua o `apps.json` e depois inicie o Moonpool ou escolha
**Recarregar**. Um novo `apps.json` com os exemplos é gravado.

## De instalado para portátil

Uma nova cópia portátil começa com os apps de exemplo. Para trazer os seus, veja
[Modo portátil](/pt-br/data/portable-mode/#escolher-portátil-no-instalador). Copie `icons\` e
`settings.json` da mesma forma, se quiser.

## Desinstalação

Desinstalar o Moonpool instalado exclui toda a pasta `%USERPROFILE%\.moonpool`, incluindo a pasta de
configuração e os painéis. Faça backup antes. Veja
[Desinstalação](/pt-br/getting-started/install/#desinstalação). Uma cópia portátil é removida excluindo a
pasta `.moonpool\` dela.
