---
title: "Notas de versão do Moonpool e mudanças recentes"
description: "Veja o que mudou nas versões recentes do Moonpool, os requisitos para executá-lo e onde encontrar as notas de versão completas no GitHub."
---

As notas completas de cada versão estão na
[página de Releases](https://github.com/kjustinkeener/Moonpool/releases) do projeto. Esta ajuda é
distribuída dentro do Moonpool, então sempre descreve a versão que você usa. O Moonpool se atualiza
sozinho; veja [Atualização](/pt-br/data/updating/).

## 0.3.17

- **Ajuda em 14 idiomas.** A ajuda abre no idioma do Moonpool: inglês, alemão, espanhol,
  francês, italiano, holandês, polonês, português do Brasil, russo, turco, japonês, coreano e
  chinês simplificado e tradicional.
- **Novos guias e páginas de suporte** sobre servidores de desenvolvimento, portas, início no
  login, agentes MCP, scripts Python e mensagens de erro comuns.
- **`mcpProcessName`.** Um padrão curinga para o nome do processo do servidor MCP de um app, para
  servidores que rodam com outro nome. Veja [mcpProcessName](/pt-br/apps/fields/#mcpprocessname).
- **O macOS não é mais suportado.** Não há builds para macOS. Nada muda no Windows e no Linux.

## 0.3.16

- **Vários Moonpools ao mesmo tempo.** O Moonpool instalado e qualquer número de cópias portáteis podem
  rodar lado a lado, uma por pasta, cada uma com seus próprios apps, ícone na bandeja e canal de
  controle. Veja [Modo portátil](/pt-br/data/portable-mode/#várias-cópias-ao-mesmo-tempo).
- **Navegador de temas.** 68 temas, cada um com uma prévia nas próprias cores. Veja
  [Temas, idioma e transparência](/pt-br/using/themes-and-language/).
- **Exemplos executáveis.** Um `apps.json` novo contém apps de exemplo que rodam do jeito que estão.
  Os painéis de exemplo agora ficam em uma pasta `dashboards/examples` de propriedade do app, que é
  atualizada com o Moonpool. Veja [Painéis de exemplo](/pt-br/getting-started/example-dashboards/).
- **Os erros do apps.json são exibidos.** Um banner sobre a barra lateral mostra o erro, e um
  Recarregar com falha mantém a última lista que carregou. Veja
  [Quando o apps.json tem um erro](/pt-br/using/hub-window/#quando-o-appsjson-tem-um-erro).
- **Canal de controle no Linux**, por meio de um socket Unix, além do verbo `list`. Veja
  [Verbos de controle](/pt-br/automation/control-verbs/).
- Sobre e o editor de apps acompanham em tempo real as mudanças de tema e idioma. O item de menu
  **Instalar o Moonpool…** fica oculto fora do Windows.

## 0.3.15

- Um app reiniciado mantém a saída anterior, com um separador "reiniciado" com data. Veja
  [Abas de terminal](/pt-br/using/terminal-tabs/#reiniciar).
- Cada app tem sua própria pasta `cli-output`, então a limpeza de logs nunca mexe nos logs de outro app.
- `killMode` e `stopCommand` estão no editor de apps. Veja
  [Parar e reiniciar](/pt-br/apps/stop-and-restart/).
- Os apps iniciados não herdam mais o perfil do WebView2 do próprio Moonpool.

## 0.3.14

- Os logs de sessão podem ser mantidos entre sessões, com um limite de tamanho por app. Veja
  [Logs](/pt-br/data/logs/).
- Botões Abrir e Copiar para as pastas de logs em Configurações.
- Correções na barra de título da janela de Ajuda.

## Requisitos

- Windows 10 ou 11 com WebView2 (veja [Windows](/pt-br/platforms/windows/)).
- Linux com WebKitGTK 4.1 e uma biblioteca AppIndicator (veja [Linux](/pt-br/platforms/linux/)).
