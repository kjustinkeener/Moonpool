---
title: "Instale o Moonpool no Windows ou no Linux"
description: "Instale o Moonpool com poucos cliques, escolha o modo instalado ou portátil, use depois o item de menu Instalar o Moonpool e desinstale de forma limpa quando terminar."
---

Esta página é para Windows. No Windows, o Moonpool é o próprio instalador: o download é um único
`moonpool.exe`. O Linux não tem cartão de instalação nem seletor de modo portátil; veja
[Linux](/pt-br/platforms/linux/).

## Modo instalado

Execute o `moonpool.exe` baixado. Na primeira inicialização, ele mostra o cartão de instalação. Ele tem
três controles: o botão **Instalar o Moonpool**, uma caixa **Adicionar um atalho na área de trabalho**
(marcada por padrão) e um link **Instalar portátil**.

A instalação copia o Moonpool para o seu perfil de usuário, em `.moonpool\`, adiciona um atalho no menu
Iniciar (e outro na área de trabalho, se a caixa estiver marcada) e registra uma entrada em Adicionar ou
remover programas. Depois inicia a cópia instalada e se fecha. O arquivo que você baixou fica onde
estava; você pode excluí-lo. A partir daí, inicie o Moonpool pelo atalho, como qualquer outro app.

![O cartão de instalação: botão Instalar o Moonpool, caixa de atalho na área de trabalho, link Instalar portátil e o caminho de instalação](../../../../assets/screenshots/installer-window.png)

Tudo de que o Moonpool precisa fica nessa única pasta: o programa, a sua configuração e a ajuda
incluída.

```text title="Installed layout"
%USERPROFILE%\.moonpool\
```

## Instalar o Moonpool... pelo menu

No Windows, o menu "..." tem **Instalar o Moonpool…** nos dois modos. Ele abre o mesmo cartão de
instalação. A partir de uma cópia portátil, você pode instalá-lo de forma definitiva. Em uma cópia
instalada, **Instalar o Moonpool** fica desativado ("Já instalado") e **Instalar portátil** continua
disponível.

## Desinstalação

Use Adicionar ou remover programas do Windows (Aplicativos instalados), ou execute a cópia instalada com
`--uninstall`. Ela não está no seu PATH, então informe o caminho completo:

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" --uninstall
```

Isso remove os atalhos do menu Iniciar e da área de trabalho, a entrada do registro e toda a pasta
`%USERPROFILE%\.moonpool`, **inclusive a sua configuração** (`apps.json`, as configurações e os logs).
Faça backup dessa pasta antes se quiser manter a sua configuração:

```text
%USERPROFILE%\.moonpool\moonpool-config
```

Qualquer Moonpool em execução é encerrado como parte da desinstalação.

## Modo portátil

Prefere um pen drive ou uma pasta que dá para mover? Clique em **Instalar portátil** no cartão de
instalação e escolha uma pasta. Veja [Modo portátil](/pt-br/data/portable-mode/).

## Próximo

- [Windows protected your PC](/pt-br/support/windows-protected-your-pc/) (O Windows protegeu seu PC): se o SmartScreen bloquear o instalador.
- [Runtime do WebView2 ausente](/pt-br/support/webview2-runtime-missing/): se a janela ficar em branco.
- [Seu primeiro app](/pt-br/getting-started/first-app/)
