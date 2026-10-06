---
title: "Use o Moonpool no Windows"
description: "O Windows é a principal plataforma do Moonpool: como instalá-lo e uma tabela do que difere entre Windows e Linux para você saber o que esperar."
---

O Windows é a principal plataforma do Moonpool. Instale-o como descrito em
[Instalação](/pt-br/getting-started/install/).

## Antes de executar

- **SmartScreen.** O `moonpool.exe` não tem assinatura de código, então o Windows pode mostrar "Windows
  protected your PC" (O Windows protegeu seu computador) na primeira vez. Escolha **More info** (Mais
  informações) e depois **Run anyway** (Executar assim mesmo).
- **Antivírus.** Um exe novo e não assinado que copia a si mesmo e se substitui ao atualizar pode
  disparar um antivírus. Se o seu bloquear ou colocar em quarentena o `moonpool.exe`, permita-o para a
  pasta `.moonpool`.
- **WebView2.** As janelas do Moonpool usam o Microsoft Edge WebView2, que acompanha o Windows 11 e o
  Windows 10 atual. Se a janela ficar em branco ou nunca abrir, instale o Runtime Evergreen do WebView2 da
  Microsoft.

Mais informações: [Windows protected your PC](/pt-br/support/windows-protected-your-pc/),
[Runtime do WebView2 ausente](/pt-br/support/webview2-runtime-missing/) e
[Iniciar um script ou servidor de desenvolvimento automaticamente no login do Windows](/pt-br/guides/start-app-at-windows-login/).

## Bandeja

No Windows 11, um novo ícone de bandeja costuma ir para a área de ícones ocultos. Clique na seta **^** à
direita da barra de tarefas para encontrá-lo e arraste-o para a barra de tarefas para mantê-lo visível.

## Comandos

- Os comandos rodam por `cmd /c`. Evite aspas duplas aninhadas em `command`; o `cmd /c` as estraga. Para
  um script que deve deixar um shell aberto, use
  `pwsh -NoLogo -NoProfile -NoExit -Command <script and args>` sem aspas em volta da parte do script.
- `processName` corresponde com ou sem `.exe`, sem diferenciar maiúsculas de minúsculas.
- Parar encerra toda a árvore de processos que o Moonpool iniciou, incluindo os processos que se
  desacoplaram dela.
- Os apps do Docker Desktop precisam de `killMode` `none` ou `command`, nunca `port`. Veja
  [Apps Docker no Windows](/pt-br/apps/stop-and-restart/#apps-docker-no-windows).

## O que difere por plataforma

| | Windows | Linux |
| --- | --- | --- |
| Instalação | `moonpool.exe` autoinstalável, ou portátil | AppImage, `.deb` ou RPM; sem cartão de instalação |
| Atualização automática | Sim, instalado e portátil | Somente AppImage |
| Pasta de configuração | `%USERPROFILE%\.moonpool\moonpool-config\` | `~/.config/Moonpool/` |
| Shell dos comandos | `cmd /c` | `$SHELL -c` |
| `processName` | Qualquer tamanho, `.exe` opcional, sem diferenciar maiúsculas | 15 caracteres ou menos, maiúsculas exatas |
| Parar por `processName` | Encerra o processo e seus filhos | Encerra apenas os processos com esse nome exato |
| Canal de controle | Pipe nomeado | Socket Unix |
| Capturas de tela da janela (testes) | Sim | Não |
| Ícones a partir de um arquivo de programa | Sim | Não |
| Bandeja | Funciona de fábrica | Precisa do AppIndicator; o GNOME padrão precisa de uma extensão |

Os detalhes do Linux estão na página [Linux](/pt-br/platforms/linux/).
