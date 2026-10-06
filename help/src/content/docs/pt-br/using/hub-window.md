---
title: "Conheça a janela do hub do Moonpool"
description: "Um tour pelo hub do Moonpool: a barra lateral, as abas de terminal, a barra de status, o menu, o aviso de erro do apps.json e como ele lembra o tamanho e a posição."
---

![O hub com três apps em execução: duas linhas de apps web em execução (1), a faixa de abas (2), a saída ao vivo do app ativo (3) e a barra de status (4)](../../../../assets/screenshots/hub-window.png)

1. Dois dos apps em execução: um ponto de status aceso e um botão de parar no lugar do de reproduzir.
2. A faixa de abas, uma aba por app aberto, com a aba ativa destacada.
3. A saída ao vivo do app ativo.
4. A barra de status de CPU e memória.

## Layout

| Área | O que contém |
| --- | --- |
| Barra de título | Minimizar, maximizar e fechar. |
| Barra lateral | A caixa de filtro, o menu **...** e seus apps agrupados por `group`. Veja [Barra lateral e menus](/pt-br/using/sidebar-and-menus/). |
| Painel CLI | Uma aba de terminal por app aberto. Veja [Abas de terminal](/pt-br/using/terminal-tabs/). |
| Barra de status | CPU e memória ao vivo, ao longo da parte inferior. |

Arraste o divisor entre a barra lateral e o painel CLI para redimensionar a barra lateral.

## Barra de status

![A barra de status: barras de CPU por núcleo à esquerda, a barra de memória à direita](../../../../assets/screenshots/status-bar.png)

A barra de status mostra uma barra fina por núcleo de CPU (passe o mouse para ver "Uso de CPU por núcleo") e depois uma barra de memória com um rótulo `used/total GB`. Desative-a com **Mostrar a barra de status de CPU/memória** em Configurações (`showStatusbar`; veja [Janela de Configurações](/pt-br/using/settings/)). A mudança é aplicada imediatamente.

## O menu ...

O botão **...** à esquerda da caixa de filtro abre o menu.

![O botão de menu ... (1) e a caixa Filtrar apps (2) no topo da barra lateral](../../../../assets/screenshots/sidebar-filter-and-menu.png)

1. O botão de menu **...**.
2. A caixa **Filtrar apps...**.

| Item | O que faz |
| --- | --- |
| Adicionar app | Abre o editor de apps. Veja [Adicionar apps](/pt-br/apps/add-an-app/). |
| Editar apps.json | Abre o `apps.json` no seu editor padrão para edição manual. |
| Recarregar | Lê o `apps.json` do disco novamente (também F5, veja [Atalhos e zoom](/pt-br/using/keyboard-shortcuts/)). |
| Configurações | Abre a janela de Configurações. |
| Ajuda | Abre esta ajuda. |
| Sobre | Abre a janela Sobre, com a versão e a verificação de atualizações. |
| Instalar o Moonpool… | Somente no Windows. Abre a janela do instalador, para instalar o app ou criar uma cópia portátil. Veja [Instalação](/pt-br/getting-started/install/) e [Modo portátil](/pt-br/data/portable-mode/). |

### Aviso de conflito de portas

Se dois apps do `apps.json` usam a mesma `port`, uma linha de aviso aparece no fim do menu, por exemplo:

```text
port 3000: App A / App B
```

Passe o mouse sobre ela para ver a frase completa. Corrija o conflito no `apps.json` ou no editor de apps; a linha some quando nenhuma porta é compartilhada.

## Quando o apps.json tem um erro

Se Recarregar (ou F5) descobre que o `apps.json` não é mais analisado ou validado, o Moonpool mantém a
lista que já tinha. Um banner no topo da barra lateral diz "apps.json tem um erro; mostrando a última
lista que carregou.", seguido do erro (passe o mouse para ver o texto completo). A lista abaixo fica
esmaecida, mas continua funcionando, então você pode iniciar e parar apps normalmente. **Editar
apps.json** no banner abre o arquivo; corrija-o e escolha **Recarregar**, e o banner desaparece.

Até que o arquivo carregue de novo, o Moonpool não salva alterações do editor de apps, de renomear,
excluir ou definir ícone, para que um arquivo com problema nunca seja sobrescrito.

Se o arquivo já estiver quebrado quando o Moonpool iniciar, não há lista anterior para manter: o banner
diz que nenhum app foi carregado e a barra lateral fica vazia. Corrija o arquivo e recarregue, ou volte
para uma cópia boa recente (veja [Se o arquivo estiver com problema](/pt-br/apps/apps-json/#se-o-arquivo-estiver-com-problema)).

## Tela vazia

Sem nenhuma aba aberta, o painel CLI mostra "Escolha um app à esquerda para iniciá-lo." Ele também
contém duas coisas que aparecem apenas enquanto nenhuma aba está aberta:

- **O banner de atualização**, quando uma versão mais nova foi encontrada na inicialização. Veja
  [Atualização](/pt-br/data/updating/).
- **Copiar o prompt**, um prompt pronto que entrega a configuração dos seus apps a um agente de IA. Veja
  [Agentes de IA: início rápido](/pt-br/automation/quick-start/#copiar-o-prompt).

O ícone da bandeja, fechar, minimizar, Sair e sempre visível estão em
[Bandeja, fechar e minimizar](/pt-br/using/tray-and-closing/).

## Tamanho, posição e estado maximizado

O Moonpool lembra o tamanho, a posição e o estado maximizado da janela do hub entre execuções. A primeira
execução abre em 1200x780, na posição que o Windows escolher.

Se a posição salva não estiver mais em nenhum monitor conectado (por exemplo, um monitor desconectado), a
posição é ignorada e o tamanho salvo é usado no local padrão. O arquivo é `window-state.json` na pasta de
configuração (veja [Onde fica a configuração](/pt-br/apps/apps-json/#onde-fica-a-configuração)).

A largura da barra lateral e se o painel CLI está recolhido também são lembradas.
