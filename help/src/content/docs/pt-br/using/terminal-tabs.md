---
title: "Use as abas de terminal do Moonpool: abrir, fechar, copiar, reiniciar"
description: "Trabalhe com as abas de terminal de cada app no hub: abra e feche abas, recolha o painel, copie e cole, reinicie uma sessão e encontre os logs de sessão."
---

Cada app roda em sua própria aba de terminal no painel CLI.

## Abas

![Faixa de abas com Metrics Dashboard ativa (destacada) e o log ao vivo abaixo; cada aba tem um ponto e um x](../../../../assets/screenshots/hub-terminal-tab.png)

- Iniciar um app, ou clicar no nome dele na barra lateral, abre a aba dele. Clicar em um nome não inicia nada; veja [Estados do app](/pt-br/support/glossary/#estados-do-app).
- Um ponto na aba fica aceso enquanto o app está em execução.
- O **x** de uma aba fecha a aba. Não para o app. Clique no nome de novo para reabrir a aba; ela mostra o log desta sessão.

## Recolher o painel

O **x** na extremidade direita da faixa de abas ("Ocultar o painel CLI") recolhe o painel CLI e reduz a
janela a apenas a barra lateral. Os terminais continuam em execução e mantêm o histórico de rolagem.

Uma seta aparece ao lado da caixa de filtro para trazer o painel de volta com a largura anterior. A seta
pulsa quando há uma atualização esperando, porque o banner de atualização fica no painel.

## Copiar e colar

| Ação | Resultado |
| --- | --- |
| Selecionar texto com o mouse | Copiado para a área de transferência ao soltar, e então a seleção é limpa. |
| Clique do meio | Cola a área de transferência no terminal. |
| Botão **Copiar tudo** (no canto superior direito, aparece ao passar o mouse) | Copia todo o histórico de rolagem como texto. |

## Histórico de rolagem

Cada terminal mantém 10.000 linhas.

## Quando um processo termina

Quando o processo termina, o terminal imprime:

```text
[process exited]
```

A aba permanece aberta com a saída intacta. A linha `[process exited]` é exibida no seu idioma (em
português: `[processo encerrado]`).

## Reiniciar

**Reiniciar** (ou Iniciar em um app parado) começa uma nova execução na mesma aba. A aba é reconstruída e
a saída anterior desta sessão é reproduzida nela a partir do log da sessão.

Se o app já foi executado antes nesta sessão, o Moonpool primeiro grava um separador esmaecido no log da
sessão, de modo que ele aparece entre a saída antiga e a nova execução:

```text
---------- restarted 2026-10-05 09:14:02 ----------
```

Se a nova execução começa limpando a tela, a saída anterior é empurrada para o histórico de rolagem em
vez de ser apagada.

## Logs de sessão

Tudo o que um app imprime também é gravado em um arquivo de log em `cli-output\`, um arquivo por app por sessão do Moonpool. O local, a retenção e a configuração **Manter os logs de saída dos apps entre sessões** estão em [Logs](/pt-br/data/logs/).
