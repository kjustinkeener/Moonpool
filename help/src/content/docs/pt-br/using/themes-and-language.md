---
title: "Altere o tema, o idioma e a transparência do Moonpool"
description: "Escolha um tema de cores e o idioma da interface, defina a transparência do fundo e a escala, e veja tudo aplicado na hora em todas as janelas do Moonpool."
---

O tema, o idioma e a transparência são definidos na [janela de Configurações](/pt-br/using/settings/).
Os três são aplicados na hora em todas as janelas abertas do Moonpool.

![Seletores de Idioma (1) e Tema (2) no topo de Configurações](../../../../assets/screenshots/settings-language-theme.png)

1. Seletor de Idioma.
2. Botão de Tema. Mostra o nome do tema atual e abre o navegador de temas.

## Temas

O navegador de temas é uma janela própria. Ele tem um cartão de prévia por tema, desenhado com as cores
desse tema (texto, painel, campo, botão, pontos de status, o gradiente do indicador e o conjunto de 16
cores do terminal), agrupados como Básicos, Neon, Quentes, Frios, Verdes, Neutros, Claros, Rosados,
Vibrantes, Pastel claro e Pastel. Clique em um cartão para aplicá-lo: todas as janelas abertas do
Moonpool mudam ao mesmo tempo e a escolha é salva. A janela continua aberta para você comparar; pressione
Esc para fechá-la.

Há 68 temas além do Automático, e cerca de metade deles é clara. Os nomes dos temas são nomes próprios e
não são traduzidos; só Automático (sistema), Escuro e Claro são.

**Automático (sistema)** segue a preferência de claro ou escuro do sistema operacional e muda ao vivo
quando o sistema muda. Qualquer outra escolha é fixa. As 16 cores ANSI do terminal também seguem o tema.

Se você salvou um tema de uma versão anterior, ele é mantido. Um nome salvo que o Moonpool não conhece
mais volta para Automático. Alguns rótulos diferem dos de antes (por exemplo, Matrix agora se chama
Terminal, Nord é Arctic, Dracula é Nocturne, Gruvbox é Retro e Solarized é Solar); a escolha salva em si
não muda.

O tema fica no `localStorage` do webview, não em `settings.json`. Se o armazenamento não estiver
disponível, volta para Automático.

```text
localStorage key: moonpool.theme
```

## Idiomas

Automático segue o idioma do sistema operacional. Caso contrário, escolha um de 14, cada um exibido no próprio idioma:

```text
English, Deutsch, Español, Français, Italiano, 日本語, 한국어, Nederlands, Polski,
Português (Brasil), Русский, Türkçe, 简体中文, 繁體中文
```

O seletor é aplicado na hora ao hub e às demais janelas. A escolha é salva como
`locale` em `settings.json`.

## Transparência

**Transparência do fundo** deixa o fundo da janela translúcido, de 0% (opaco) a
90%.

- Passar o ponteiro sobre uma janela a deixa totalmente opaca de imediato. Quando o ponteiro sai, ela volta à sua configuração com um esmaecimento de cerca de 2 segundos.
- Os terminais seguem a mesma tonalidade em vez de adicionar a sua.
- Cada janela (hub, Configurações, Sobre, o editor de apps e o navegador de temas) aplica a configuração por conta própria, e Configurações atualiza as outras ao vivo enquanto você arrasta o controle deslizante.

## Escala da interface

Mude o zoom de toda a interface com Ctrl + roda do mouse. Não há zoom pelo teclado. Veja
[Atalhos e zoom](/pt-br/using/keyboard-shortcuts/).

## Veja também

- [Janela de Configurações](/pt-br/using/settings/)
- [Atalhos e zoom](/pt-br/using/keyboard-shortcuts/)
