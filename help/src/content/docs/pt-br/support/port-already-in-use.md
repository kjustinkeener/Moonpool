---
title: "Resolva Error: listen EADDRINUSE: address already in use :::3000 e Vite Port 5173 is in use"
description: "Resolva o EADDRINUSE do Node e o Port 5173 is in use do Vite: encontre o que ocupa a porta, libere-a e use os campos port e killMode do Moonpool para evitar que se repita."
---

```text
Error: listen EADDRINUSE: address already in use :::3000
```

Este erro do Node.js significa que outro processo já está escutando na porta 3000 (o `:::` é a forma IPv6
de "todos os endereços"; você também pode ver `127.0.0.1:3000`). Muitas vezes é uma cópia do mesmo
servidor que você iniciou antes e nunca parou.

O Vite trata a mesma situação de outra forma. Por padrão, ele imprime:

```text
Port 5173 is in use, trying another one...
```

(a porta 5173 está em uso, tentando outra) e inicia na próxima porta livre, então o servidor está no ar,
mas não onde você espera. Com `--strictPort` (ou `server.strictPort: true`), o Vite termina em vez disso,
com `Error: Port 5173 is already in use`.

## Resolva você mesmo

1. Encontre o processo que é dono da porta e encerre-o. No Windows:

   ```text frame="terminal"
   netstat -ano | findstr :3000
   taskkill /PID 12345 /F
   ```

   Passo a passo, com a versão do PowerShell, em
   [Encontrar e encerrar o processo que usa uma porta](/pt-br/guides/find-and-kill-process-using-port-windows/).
2. Ou inicie o seu servidor em outra porta, por exemplo `PORT=3001` para muitos servidores Node ou
   `--port 5174` para o Vite.

## Como o Moonpool ajuda

Se você executa o servidor pelo Moonpool, defina `port` na entrada dele. Então o Moonpool:

- mostra o app como em execução enquanto algo responde nessa porta, de modo que um servidor restante que a
  ocupa aparece como em execução, mas não "gerenciado pelo Moonpool";
- ao **Parar** e **Reiniciar**, encerra o que ainda estiver escutando em `port` quando `killMode` é `port`,
  que é o padrão dos apps `web`, para que o próximo Iniciar encontre a porta livre;
- sinaliza dois apps configurados com a mesma `port`.

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "node server.js",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

O Moonpool não verifica a porta antes de iniciar. Se a porta ainda estiver ocupada, o comando imprime o
erro acima na aba de terminal do app. Clique em **Parar** (que libera a porta) e em **Iniciar** de novo.

Para o Vite, passe `--strictPort` e mantenha `port` igual à porta que você pede:

```text title="command"
npm run dev -- --port 5173 --strictPort
```

Sem ele, o Vite pode passar para a 5174 enquanto o Moonpool continua vigiando a 5173, e o ponto de status
nunca fica fixo.

`killMode` `port` encerra qualquer processo na porta, então use-o apenas para portas de que nada mais
precise. Para apps Docker no Windows, nunca o use. Veja
[Parar e reiniciar](/pt-br/apps/stop-and-restart/#apps-docker-no-windows).

## Veja também

- [Campos do app](/pt-br/apps/fields/): `port`, `killMode`.
- [Parar e reiniciar](/pt-br/apps/stop-and-restart/)
- [Solução de problemas](/pt-br/support/troubleshooting/#dois-apps-usam-a-mesma-porta)
- [Executar um servidor de desenvolvimento npm em segundo plano no Windows](/pt-br/guides/run-npm-dev-server-in-background-windows/)
