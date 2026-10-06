---
title: "Atualize o Moonpool e resolva uma atualização que falhou"
description: "Veja como o Moonpool procura, baixa e aplica atualizações, o que o banner faz, como as cópias portáteis e do Linux se atualizam e o que fazer em caso de falha."
---

O Moonpool se atualiza sozinho. Não há um instalador separado para baixar nem um assistente para percorrer.

## Como as atualizações chegam

O Moonpool busca `update.json` (`linux-update.json` no Linux) nos Releases do GitHub do projeto, compara
as versões e só oferece uma estritamente mais nova. Ele verifica:

- na inicialização, a menos que **Procurar atualizações ao iniciar** esteja desligado em
  [Configurações](/pt-br/using/settings/);
- sempre que você pressionar **Procurar atualizações** na janela Sobre. Esse botão instala uma versão mais
  nova na hora e reinicia o Moonpool. Caso contrário, diz que você está na versão mais recente ou mostra o erro.

A janela Sobre mostra a versão que você executa, sob o nome:

![A parte de cima da janela Sobre: o logotipo, o nome (1) e a linha de versão abaixo dele](../../../../assets/screenshots/about-header.png)

1. O nome. A linha abaixo é a versão e a data da compilação.

Todo download é verificado com a chave de assinatura minisign do Moonpool antes de ser aplicado, então um
download adulterado ou corrompido é rejeitado. O Moonpool nunca instala uma versão mais antiga.

## O banner de atualização

Na inicialização, uma atualização encontrada aparece como um banner na tela vazia do hub:

```text
O Moonpool {version} está disponível (você tem a {current}).
```

O banner só aparece enquanto nenhuma aba de app está aberta e o painel CLI está expandido. Com o painel
recolhido, a seta ao lado da caixa de filtro pulsa em vez disso. Com uma aba aberta, não há nenhum sinal.
Para ver o banner, feche todas as abas (e expanda o painel), ou use **Procurar atualizações** na janela
Sobre.

Clique em **Baixar e instalar** e o Moonpool se substitui e reinicia, ou dispense o banner com o x.

## Cópias portáteis

Uma cópia portátil atualiza o `moonpool.exe` da própria pasta `.moonpool\` da mesma forma. Cada cópia
verifica e se atualiza por conta própria. A pasta precisa permitir gravação, então uma cópia em um pen
drive ou compartilhamento somente leitura não consegue se atualizar; copie à mão um `moonpool.exe` mais
novo por cima.

## Linux

Só o AppImage se atualiza sozinho. Ele substitui o arquivo AppImage no lugar, então mantenha-o em uma
pasta em que você possa gravar. Uma instalação `.deb` ou RPM é atualizada pelo seu gerenciador de pacotes:
instalar pelo Moonpool falha com

```text
automatic updates are available for the AppImage only; update the .deb or RPM with your package manager
```

(as atualizações automáticas só estão disponíveis para o AppImage; atualize o .deb ou o RPM com o seu
gerenciador de pacotes). Veja [Linux](/pt-br/platforms/linux/#atualizações).

## Quando uma atualização falha

O banner mostra o motivo e o botão fica disponível de novo para você tentar outra vez:

```text
Falha na atualização: <error>
```

| O erro contém | Causa provável | O que fazer |
| --- | --- | --- |
| `download failed` | Sem conexão, um proxy, ou o GitHub limitando as requisições | Aguarde e tente de novo, ou atualize manualmente. |
| `signature verification FAILED - refusing to install` | O download está corrompido ou foi alterado | Tente de novo. Se continuar falhando, atualize manualmente pela página de Releases. |
| `rename self aside` ou `write new exe` | A pasta é somente leitura, ou um antivírus está segurando o arquivo | Torne a pasta gravável, ou permita o `moonpool.exe` no seu antivírus, e tente de novo. |
| `refusing to install ... not newer than current` | A versão oferecida não é mais nova | Nada a fazer. |

### Atualizar manualmente

Saia do Moonpool, baixe o `moonpool.exe` da
[página de Releases](https://github.com/kjustinkeener/Moonpool/releases) do projeto e copie-o por cima do
antigo: `%USERPROFILE%\.moonpool\moonpool.exe` no modo instalado, ou o da sua pasta `.moonpool\` em uma
cópia portátil. A sua pasta de configuração não é tocada. No Linux, substitua o AppImage ou use o seu
gerenciador de pacotes.

## A ajuda também é atualizada

Esta ajuda é distribuída dentro do Moonpool, então cada atualização do programa traz a ajuda
correspondente. A cópia offline sempre corresponde à versão que você executa.

## Veja também

- [Novidades](/pt-br/getting-started/whats-new/)
- [Janela de Configurações](/pt-br/using/settings/)
