---
title: "Runtime do WebView2 ausente: resolva uma janela do Moonpool em branco ou ausente no Windows"
description: "Se a janela do Moonpool nunca abre ou fica em branco no Windows, o Runtime do Microsoft Edge WebView2 pode estar ausente. Como verificar e instalá-lo."
---

Se a janela do Moonpool nunca abre, ou abre e fica em branco, no Windows, a causa provável é a ausência do
Runtime do Microsoft Edge WebView2. O Moonpool é um app Tauri, e as janelas dele são páginas web
desenhadas pelo WebView2.

O WebView2 acompanha o Windows 11 e o Windows 10 atual, então a maioria dos PCs já o tem. Ele pode estar
ausente em um Windows 10 antigo ou enxuto, ou em um PC em que foi removido. O código-fonte do Moonpool não
mostra uma mensagem específica para esse caso, então nenhum texto de erro é citado aqui: o sintoma é a
janela não aparecer ou aparecer vazia.

## Verifique se está instalado

No PowerShell, procure a versão do runtime no registro (o primeiro caminho é a instalação para todo o
sistema, o segundo uma por usuário):

```powershell frame="terminal"
Get-ItemProperty "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
Get-ItemProperty "HKCU:\Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
```

Um número de versão como `120.0.2210.91` significa que está instalado. Um erro nos dois significa que não
está.

## Instale-o

Baixe o Runtime **Evergreen** do WebView2 na página do WebView2 da Microsoft (pesquise por "WebView2
Runtime download"), execute o instalador e inicie o Moonpool de novo. O runtime Evergreen se atualiza
sozinho.

## Se está instalado e a janela continua em branco

- Saia de todos os Moonpools pela bandeja (ou encerre o `moonpool.exe` no Gerenciador de Tarefas) e
  inicie-o de novo.
- Ligue **Gravar informações de depuração em um arquivo** em [Configurações](/pt-br/using/settings/) se
  conseguir acessá-las, e confira o `moonpool.log`. Veja [Logs](/pt-br/data/logs/).
- Se a janela abre mas está fora da tela, veja
  [Problemas com a janela](/pt-br/support/troubleshooting/#problemas-com-a-janela).

## Veja também

- [Windows](/pt-br/platforms/windows/#antes-de-executar)
- [Instalação](/pt-br/getting-started/install/)
- [Solução de problemas](/pt-br/support/troubleshooting/)
