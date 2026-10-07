---
title: "Mantenha o Moonpool na bandeja: comportamento de fechar, minimizar e sair"
description: "Controle o que fazem o ícone da bandeja, fechar, minimizar e Sair, mantenha a janela sempre visível e evite ocultar ao mesmo tempo a bandeja e a barra de tarefas."
---

## Ícone da bandeja

| Ação | Resultado |
| --- | --- |
| Clique esquerdo | Mostra a janela do hub (restaura se estiver minimizada ou oculta). |
| Clique direito | Menu apenas com **Mostrar o Moonpool** e **Sair** (no seu idioma). |

Com várias cópias do Moonpool em execução, cada uma tem o próprio ícone na bandeja. A dica diz de qual
cópia se trata. Veja [Modo portátil](/pt-br/data/portable-mode/#várias-cópias-ao-mesmo-tempo).

## Sair

**Sair** encerra o Moonpool e, no Windows, para todos os apps que o Moonpool iniciou, incluindo os
processos filhos deles. Apps que já estavam em execução antes de o Moonpool os ver (mostrados como em
execução sem "gerenciado pelo Moonpool") são deixados em paz. No Linux, sair não para os apps
iniciados de forma confiável.

## Fechar e minimizar

O botão de fechar encerra o Moonpool por padrão (`closeToTray` é `false`). Ative **Fechar para a
bandeja** em Configurações e fechar passa a ocultar a janela na bandeja. O Moonpool continua em execução, e
o ícone da bandeja ou **Mostrar o Moonpool** a traz de volta.

**Minimizar para a bandeja** (`minimizeToTray`, ligado por padrão) oculta a janela na bandeja ao
minimizar, e ela sai da barra de tarefas. Desligue para minimizar para a barra de tarefas como de costume.

![Configurações: Fechar para a bandeja e Minimizar para a bandeja (1), e o controle deslizante Transparência do fundo (2)](../../../../assets/screenshots/settings-tray-and-transparency.png)

1. **Fechar para a bandeja** e **Minimizar para a bandeja**.
2. **Transparência do fundo**. Veja [Temas, idioma e transparência](/pt-br/using/themes-and-language/#transparência).

## Bloqueio de bandeja e barra de tarefas

**Mostrar na bandeja** e **Mostrar na barra de tarefas** controlam se o ícone da bandeja e o botão da
barra de tarefas ficam visíveis. Pelo menos um precisa ficar ligado, senão uma janela oculta não teria
como voltar. Quando só um está ligado, a caixa dele fica desativada até você ligar o outro de novo.

## Sempre visível

**Sempre visível** em Configurações mantém todas as janelas do Moonpool (o hub, Configurações, Sobre, o
editor de apps, o navegador de temas, o instalador e a Ajuda) acima das outras janelas. Vem desligado por
padrão.

## Veja também

- [Executar um servidor de desenvolvimento npm em segundo plano no Windows](/pt-br/guides/run-npm-dev-server-in-background-windows/)
- [Iniciar um script ou servidor de desenvolvimento automaticamente no login do Windows](/pt-br/guides/start-app-at-windows-login/)
- [Janela de Configurações](/pt-br/using/settings/)
- [A janela do hub](/pt-br/using/hub-window/)
