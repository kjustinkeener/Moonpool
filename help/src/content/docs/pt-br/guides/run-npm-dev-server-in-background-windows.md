---
title: "Executar um servidor de desenvolvimento npm em segundo plano no Windows sem janela de terminal"
description: "Mantenha o npm run dev, o Vite ou outro servidor de desenvolvimento rodando no Windows sem janela de console, e inicie, pare e leia a saída dele pela bandeja."
---

Um servidor de desenvolvimento iniciado com `npm run dev` roda no terminal que o iniciou, então fechar
essa janela o encerra. O jeito simples do Windows de mantê-lo rodando é um processo oculto, por exemplo
`Start-Process npm.cmd -ArgumentList "run","dev" -WindowStyle Hidden` no PowerShell, mas aí você não tem
saída para ler e pará-lo significa procurar o `node.exe` certo (veja
[Encontrar e encerrar o processo que usa uma porta](/pt-br/guides/find-and-kill-process-using-port-windows/)).

## O jeito do Moonpool

O Moonpool executa o comando em sua própria aba de terminal embutida na janela do hub, então não há uma
janela de console separada para manter aberta. Oculte o hub na bandeja e o servidor continua rodando.
Adicione o app uma vez:

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173",
  "openBrowser": true
}
```

Clique no controle **Iniciar** do app. O ponto de status fica fixo quando `port` responde, e o navegador
abre em `url` por causa de `openBrowser`. Clique no nome do app para ler a saída na aba dele. **Parar**
encerra o terminal e tudo o que ele iniciou, e libera a porta (`killMode` `port` é o padrão de `web`).

## Mantenha em execução ao fechar a janela

Por padrão, o botão de fechar encerra o Moonpool, e no Windows isso também para todos os apps que ele
iniciou. Ative **Fechar para a bandeja** em [Configurações](/pt-br/using/settings/), e fechar a janela só
a oculta. O ícone da bandeja (ou **Mostrar o Moonpool**) a traz de volta. Os detalhes estão em
[Bandeja, fechar e minimizar](/pt-br/using/tray-and-closing/).

## Mantenha a porta previsível

O Moonpool decide se está em execução a partir de `port`. O Vite passa para a próxima porta livre quando
a dele está ocupada, o que deixaria o Moonpool vigiando a porta errada. Passe `--strictPort` para o Vite
sair em vez disso, e defina `port` com o mesmo valor:

```text title="command"
npm run dev -- --port 5173 --strictPort
```

Se a porta já estiver ocupada, veja
[Resolver EADDRINUSE e "Port 5173 is in use"](/pt-br/support/port-already-in-use/).

## Limites

- O Moonpool não reinicia um servidor que falha. Ele mostra o app como parado e a aba imprime
  `[process exited]` (processo encerrado).
- O Moonpool não inicia sozinho no login do Windows. Veja
  [Iniciar um script ou servidor de desenvolvimento automaticamente no login do Windows](/pt-br/guides/start-app-at-windows-login/).

## Veja também

- [Campos do app](/pt-br/apps/fields/): `port`, `openBrowser`, `killMode`.
- [Tipos de app](/pt-br/apps/types/): como se decide se `web` está em execução.
- [Parar e reiniciar](/pt-br/apps/stop-and-restart/)
- [Exemplos](/pt-br/apps/examples/#servidor-de-desenvolvimento-web)
