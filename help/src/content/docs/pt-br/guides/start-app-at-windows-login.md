---
title: "Iniciar um script ou servidor de desenvolvimento automaticamente no login do Windows"
description: "Inicie o Moonpool no login do Windows com um atalho na pasta Inicializar e execute nele um servidor de desenvolvimento ou script com um script do PowerShell."
---

O Windows tem duas formas usuais de iniciar algo no login: um atalho na sua pasta Inicializar (pressione
Win+R, digite `shell:startup` e pressione Enter) ou uma tarefa do Agendador de Tarefas com um gatilho "Ao
fazer logon". Qualquer uma executa um programa ou script, que poderia ser diretamente o comando do seu
servidor de desenvolvimento, mas então nada o acompanha, mostra a saída dele nem o para para você.

## O que o Moonpool oferece

O Moonpool não tem uma configuração para iniciar no login, e uma entrada do `apps.json` não tem nenhum
campo que a inicie quando o Moonpool abre (a lista completa está em [Campos do app](/pt-br/apps/fields/)
e [settings.json](/pt-br/data/settings-json/)). O que você pode fazer é iniciar o Moonpool no login por
conta própria e depois fazer um script iniciar os apps que quiser, com o mesmo verbo que a
[linha de comando](/pt-br/automation/command-line/) oferece.

Primeiro registre o app como de costume:

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

Depois salve isto como `start-moonpool-apps.ps1`. Instalado, o programa é
`%USERPROFILE%\.moonpool\moonpool.exe`; para uma cópia portátil, use o caminho do exe dessa cópia.

```powershell title="start-moonpool-apps.ps1"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
Start-Process $mp
Start-Sleep -Seconds 15
& $mp launch site
```

O Moonpool já precisa estar em execução para que o `launch` seja entregue a ele; sem nenhum residente, o
mesmo comando inicia um novo Moonpool e o verbo não é executado. A espera dá tempo para ele iniciar,
então aumente-a em uma máquina lenta. Adicione uma linha `& $mp launch <id>` por app.

Por fim, coloque um atalho para o script na pasta Inicializar, com este destino:

```text title="Shortcut target"
powershell.exe -NoProfile -WindowStyle Hidden -File "C:\Users\you\start-moonpool-apps.ps1"
```

Para verificar o que aconteceu, adicione `--ticket t1` a um verbo e leia o resultado em `state.json`
([Lendo o resultado](/pt-br/automation/command-line/#lendo-o-resultado)).

## Ressalvas

- Um servidor de desenvolvimento iniciado assim é "gerenciado" pelo Moonpool como qualquer outro, então
  Parar e Sair funcionam nele. Se o mesmo app já estiver em execução (iniciado à mão, por exemplo), o
  Moonpool o mostra como em execução, mas não gerenciado.
- O Moonpool não reinicia um app que termina e não lembra quais apps estavam em execução quando você
  saiu pela última vez.

## Veja também

- [Linha de comando](/pt-br/automation/command-line/)
- [Bandeja, fechar e minimizar](/pt-br/using/tray-and-closing/)
- [Executar um servidor de desenvolvimento npm em segundo plano no Windows](/pt-br/guides/run-npm-dev-server-in-background-windows/)
