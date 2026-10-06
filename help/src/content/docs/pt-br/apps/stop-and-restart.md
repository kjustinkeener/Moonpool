---
title: "Pare um servidor de desenvolvimento e tudo o que ele iniciou"
description: "Faça Parar e Reiniciar encerrarem um app e seus processos filhos de forma limpa com killMode e stopCommand, com os padrões por tipo e o Docker no Windows."
---

Parar sempre faz isto primeiro: o Moonpool encerra o terminal que iniciou para o app, incluindo tudo o que
esse terminal executou. Para muitos apps, isso é tudo de que se precisa.

Alguns apps sobrevivem a esse terminal (uma janela desktop se desacopla do servidor de desenvolvimento que
a iniciou, ou um subprocesso do servidor continua ocupando a porta). O **`killMode`** escolhe uma etapa
extra executada depois.

| `killMode` | Etapa extra ao Parar | Lê | Padrão para |
| --- | --- | --- | --- |
| `processName` | Força o encerramento de todos os processos com esse nome. No Windows, também dos filhos (`taskkill /IM <name>.exe /T /F`). Nos demais, `pkill -KILL -x <name>`: uma correspondência exata do nome, diferenciando maiúsculas, sem incluir os filhos. | `processName` | `desktop` |
| `port` | Força o encerramento do processo que estiver escutando em `port`. | `port` | `web` |
| `command` | Executa o `stopCommand` em `cwd` e espera terminar. | `stopCommand`, `cwd`, `env` | nenhum |
| `none` | Nada. | nada | `static`, `cli` |

Omita `killMode` para obter o padrão do tipo do app e defina-o apenas quando Parar deixar algo em
execução.

![O seletor killMode no diálogo Editar app, definido como "padrão (por tipo)", com a linha de dica que lista o que cada tipo faz por padrão](../../../../assets/screenshots/edit-app-killmode.png)

1. O seletor `killMode`. "padrão (por tipo)" equivale a omitir a chave.

- Se o campo de que o modo precisa estiver vazio (modo `port` sem `port`, por exemplo), a etapa extra é
  ignorada. Não é um erro.
- `killMode` é independente de `type`: `port` funciona em um app `cli`, `processName` em um app `web`.
- Uma string vazia ou um valor não reconhecido não faz nada extra. Não recorre ao padrão do tipo.

Para um app desktop, o modo `processName` executa o equivalente a:

```powershell frame="terminal"
taskkill /IM notes-app.exe /T /F
```

## Vários Moonpools, ou seus próprios processos

`processName` e `port` não sabem quem iniciou um processo. `processName` encerra todos os processos com
esse nome, e `port` encerra o que estiver escutando na porta, inclusive um iniciado por outra cópia do
Moonpool (a instalada e as cópias portáteis rodam de forma independente; veja
[Modo portátil](/pt-br/data/portable-mode/#várias-cópias-ao-mesmo-tempo)) e um que você mesmo iniciou. Use esses
modos apenas para apps que não entrem em conflito dessa forma: um nome ou uma porta que nada mais na
máquina use. Se duas cópias registrarem o mesmo app, ou você também o executar à mão, dê a ele `killMode`
`none` ou um `command` que pare apenas a própria instância.

## stopCommand

Usado apenas quando `killMode` é `command`. Roda por `cmd /c` no Windows e `$SHELL -c` nos demais, em
`cwd`, com o seu `env` adicionado. `{MP_HOME}` e `{MP_DATA}` funcionam nele. O Moonpool espera que ele
termine antes de fazer qualquer outra coisa, então um Reiniciar nunca reinicia o app enquanto ele ainda
estiver rodando. O código de saída é ignorado. Se ainda estiver rodando após 60 segundos, o Moonpool o
encerra junto com os filhos e segue em frente.

## Reiniciar

Reiniciar é Parar seguido de Iniciar com o mesmo `command`. O Moonpool espera até 4 segundos para que a
instância antiga apareça como parada (para a porta estar livre) antes de iniciar de novo. Uma entrada
`static` com apenas uma `url` não tem nada a parar: Reiniciar apenas abre a página de novo.

## Apps Docker no Windows

Use `none`, ou `command` com um comando de parada real, como `docker compose stop app`. Não use `port`.

O Docker Desktop publica a porta de cada contêiner por meio de um único processo compartilhado em segundo
plano. No Windows, "o que estiver escutando na porta" é esse processo compartilhado, então o modo `port`
forçaria o encerramento do Docker Desktop e derrubaria todos os contêineres, não só este app. Como
salvaguarda, o Moonpool se recusa a encerrar por porta uma lista fixa de processos compartilhados do
Windows: os processos de backend, proxy e serviço do Docker Desktop, `dockerd`, `vpnkit`, os processos
host do WSL e processos centrais do sistema como `svchost`. Isso não substitui escolher o modo certo.

Se o seu `command` já recria o contêiner (`docker compose up -d --build`), `none` é o correto: Reiniciar
simplesmente o executa de novo.

Veja também [Encontrar e encerrar o processo que usa uma porta](/pt-br/guides/find-and-kill-process-using-port-windows/)
e [Resolver EADDRINUSE e "Port 5173 is in use"](/pt-br/support/port-already-in-use/).

## Exemplos

Um servidor de desenvolvimento que às vezes deixa um processo node ocupando a porta (este é o padrão de
`web`, mostrado aqui de forma explícita):

```json title="apps.json"
{ "id": "site", "name": "Site", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
  "killMode": "port" }
```

Um app Docker Compose:

```json title="apps.json"
{ "id": "api", "name": "API", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\api", "command": "docker compose up -d --build", "port": 8080,
  "killMode": "command", "stopCommand": "docker compose stop app" }
```
