---
title: "Atalhos de teclado, atalhos do mouse e zoom do Moonpool"
description: "Veja todos os atalhos de teclado e de mouse do hub do Moonpool e como aumentar e diminuir o zoom da interface para que o texto fique confortável de ler."
---

## Teclado

| Teclas | Onde | O que faz |
| --- | --- | --- |
| F5, Ctrl+R, Cmd+R | Hub | Recarrega o `apps.json` do disco, igual a **Recarregar** no menu. A página em si não é atualizada. |
| Esc | Menu de contexto | Fecha o menu. |
| Esc | Ao renomear um app | Cancela a renomeação. |
| Esc | Janelas de Configurações, Sobre e editor de apps | Fecha a janela (o editor pergunta antes de descartar as alterações). |
| Enter | Ao renomear um app | Salva o novo nome. |

## Mouse

| Ação | Onde | O que faz |
| --- | --- | --- |
| Ctrl + roda | Hub | Muda o zoom da interface. |
| Selecionar texto | Terminal | Copia e limpa a seleção. |
| Clique do meio | Terminal | Cola. |
| Clique direito | Linha da barra lateral | Abre o menu da linha. Veja [Barra lateral e menus](/pt-br/using/sidebar-and-menus/). |

## Zoom

Segure Ctrl e gire a roda sobre o hub para mudar o zoom. Girar para cima aumenta e para baixo diminui, em passos de cerca de 10 por cento por evento da roda.

```text
Ctrl + wheel up      zoom in
Ctrl + wheel down    zoom out
```

- O intervalo vai de 0,5x a 3x.
- A janela é redimensionada pelo mesmo fator, então o layout fica tão compacto em 2x quanto em 1x. Ao atingir o limite, a janela para de crescer.
- O fator é salvo como `uiScale` em `settings.json` e aplicado na próxima inicialização. O tamanho de janela salvo já é o tamanho com zoom, então não é escalado de novo. Veja [settings.json](/pt-br/data/settings-json/).

- O zoom se aplica apenas à janela do hub. Configurações, Sobre, o editor de apps e a Ajuda mantêm
  o próprio tamanho.

Não há controle em Configurações para `uiScale` nem tecla de redefinição. Para voltar ao tamanho normal,
gire a roda o mesmo número de passos no sentido contrário, ou saia do Moonpool, defina `uiScale` como `1`
em `settings.json` (ou remova a chave) e inicie-o de novo.

## Veja também

- [Temas, idioma e transparência](/pt-br/using/themes-and-language/)
- [Barra lateral e menus](/pt-br/using/sidebar-and-menus/)
