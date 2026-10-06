---
title: "Encontrar e encerrar o processo que usa uma porta no Windows (3000, 5173, 8080)"
description: "Descubra qual processo ocupa a porta 3000 ou 5173 no Windows com netstat ou PowerShell, encerre-o com taskkill e deixe o Moonpool liberar a porta quando você parar um app."
---

Quando um servidor de desenvolvimento falha porque a porta já está em uso, há outra coisa escutando
nessa porta. No Prompt de Comando, liste os processos que escutam junto com o ID do processo dono e
depois encerre-o:

```text frame="terminal"
netstat -ano | findstr :3000
taskkill /PID 12345 /F
```

A última coluna da linha `LISTENING` é o PID (`findstr :3000` também corresponde a `:30001`, então
leia o endereço local). `tasklist /FI "PID eq 12345"` mostra de qual programa se trata. No PowerShell,
a mesma consulta é:

```powershell frame="terminal"
Get-NetTCPConnection -LocalPort 3000 -State Listen | Select-Object LocalPort, OwningProcess
Get-Process -Id 12345
Stop-Process -Id 12345 -Force
```

Adicione `/T` ao `taskkill` para encerrar também os processos filhos. Processos de outro usuário ou do
sistema podem exigir uma janela elevada (administrador).

## O jeito do Moonpool

Para um app que você executa pelo Moonpool, não é preciso procurar o PID. Dê ao app uma `port` e Parar a
libera. Em um app `web`, esse é o `killMode` padrão, escrito aqui de forma explícita:

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

Parar primeiro encerra o terminal que o Moonpool iniciou e depois força o encerramento do que ainda
estiver escutando em `port`. No Windows, é a mesma consulta de cima (`Get-NetTCPConnection -LocalPort
<port> -State Listen`), seguida de `taskkill /PID <pid> /T /F` para cada dono.

- Se algo que você não iniciou estiver ocupando a porta, o Moonpool mostra o app como em execução, mas
  não "gerenciado pelo Moonpool". Clique em **Parar** nele: a etapa de `port` ainda é executada.
- O Moonpool se recusa a encerrar por porta uma lista fixa de processos compartilhados do Windows, como o
  backend do Docker Desktop, o `svchost` e o host do WSL. Para um app Docker, use `killMode` `command`
  ou `none`, nunca `port`. Veja
  [Apps Docker no Windows](/pt-br/apps/stop-and-restart/#apps-docker-no-windows).
- Isso só funciona para portas de apps listados em `apps.json`. Para qualquer outra porta, use os
  comandos do início.
- O modo `port` encerra tudo o que estiver escutando, inclusive uma cópia que você iniciou à mão, então
  use-o apenas para portas de que nada mais na máquina precise.

## Veja também

- [Resolver EADDRINUSE e "Port 5173 is in use"](/pt-br/support/port-already-in-use/)
- [Parar e reiniciar](/pt-br/apps/stop-and-restart/)
- [Campos do app](/pt-br/apps/fields/): `port` e `killMode`.
- [Dois apps usam a mesma porta](/pt-br/support/troubleshooting/#dois-apps-usam-a-mesma-porta)
