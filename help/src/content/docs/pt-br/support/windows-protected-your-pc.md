---
title: "Windows protected your PC: execute o instalador do Moonpool mesmo assim (SmartScreen)"
description: "O Windows SmartScreen mostra Windows protected your PC ao executar o moonpool.exe. Por que aparece, como escolher More info e depois Run anyway, e o que verificar antes."
---

Ao executar o `moonpool.exe` baixado, o Windows pode mostrar uma caixa azul intitulada **Windows protected
your PC** (O Windows protegeu seu computador), com a linha "Microsoft Defender SmartScreen prevented an
unrecognized app from starting. Running this app might put your PC at risk." (O Microsoft Defender
SmartScreen impediu a inicialização de um aplicativo não reconhecido. Executar este aplicativo pode
colocar seu PC em risco).

## Por que aparece

O SmartScreen avisa sobre programas novos ou que ele não viu rodar em muitos PCs. O `moonpool.exe` não tem
assinatura de código, então o Windows não tem um editor em que confiar e pode mostrar o aviso na primeira
vez. É uma verificação de reputação, não uma constatação de que o arquivo é malicioso.

## O que fazer

1. Na caixa, clique em **More info** (Mais informações). O editor aparece como "Unknown publisher" (editor
   desconhecido).
2. Clique em **Run anyway** (Executar assim mesmo). O cartão de instalação abre. Veja
   [Instalação](/pt-br/getting-started/install/).

Se quiser ser cuidadoso antes, baixe apenas do site oficial do Moonpool ou dos releases dele no GitHub, e
confira se o nome do arquivo é `moonpool.exe`.

## Se não houver o botão Run anyway

Em alguns PCs gerenciados, o administrador desativa a opção e você não verá **Run anyway**. Pergunte ao
seu administrador, ou use um PC que você gerencie. Um arquivo que veio em um zip baixado também pode
carregar um bloqueio: clique com o botão direito no arquivo, escolha **Propriedades**, marque
**Desbloquear** se aparecer, depois **OK** e execute-o de novo.

## Avisos do antivírus

Um exe novo e não assinado que copia a si mesmo para o seu perfil e se substitui ao atualizar também pode
disparar um antivírus. Se o seu bloquear ou colocar em quarentena o `moonpool.exe`, permita-o para a pasta
`.moonpool`. Veja [Windows](/pt-br/platforms/windows/#antes-de-executar).

## Veja também

- [Instalação](/pt-br/getting-started/install/)
- [Windows](/pt-br/platforms/windows/)
- [O instalador mostra um erro](/pt-br/support/troubleshooting/#o-instalador-mostra-um-erro)
