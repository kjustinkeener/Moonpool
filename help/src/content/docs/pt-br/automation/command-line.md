---
title: "Controle o Moonpool pela linha de comando"
description: "Controle um Moonpool em execução com os verbos do moonpool.exe a partir de um terminal ou script, marque um comando com um ticket e leia o resultado em state.json."
---

Executar `moonpool.exe` de novo enquanto esse mesmo Moonpool já está em execução não abre uma segunda
janela. O segundo processo passa os argumentos dele para o que está em execução pelo
[canal de controle](/pt-br/automation/control-verbs/) e termina. O Moonpool já precisa estar em execução:
sem nenhum residente, o mesmo comando inicia um novo Moonpool e o verbo não é executado.

"O mesmo Moonpool" significa a mesma pasta. O Moonpool instalado e cada cópia portátil rodam por conta
própria, então um comando chega à cópia cujo `moonpool.exe` você executou, nunca a outra. Veja
[Modo portátil](/pt-br/data/portable-mode/#várias-cópias-ao-mesmo-tempo).

Use o caminho da cópia que você quer. Para a instalada:

```powershell frame="terminal"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
```

Com várias cópias em execução, `Get-Process moonpool` lista todas, então escolha pelo `Path` em vez de
pegar a primeira. Ele também lista os auxiliares `moonpool.exe mcp` ociosos que hosts MCP iniciaram, então
um processo `moonpool` não prova que há um hub em execução. Pergunte ao canal de controle com `ping`
([Verbos de controle](/pt-br/automation/control-verbs/)).

## Verbos

O verbo não diferencia maiúsculas de minúsculas. `<id>` é o `id` de um app do `apps.json`.

| Comando | Efeito |
| --- | --- |
| `moonpool.exe` | Sem verbo: traz a janela para a frente. |
| `moonpool.exe show` | Traz a janela para a frente. |
| `moonpool.exe launch <id>` | Inicia o app e abre a aba de terminal dele. |
| `moonpool.exe stop <id>` | Para o app. |
| `moonpool.exe restart <id>` | Para, espera a porta e o processo serem liberados, inicia. |
| `moonpool.exe reload` | Lê o `apps.json` de novo. |
| `moonpool.exe refresh-icons` | Busca todos os ícones de novo. |
| `moonpool.exe help` | Abre a janela de Ajuda. |
| `moonpool.exe quit` | Encerra o Moonpool, igual ao menu da bandeja. |
| `moonpool.exe dump <id> [out-path]` | Sem `out-path`, informa o caminho do log do app nesta sessão. Com ele, copia o log para lá como texto simples sem códigos ANSI. |
| `moonpool.exe paths` | Informa a pasta de configuração, `apps.json`, `state.json`, o log, a pasta de dumps, a pasta de ícones, o indicador de portátil e o caminho do exe que o Moonpool em execução usa. |
| `moonpool.exe read-config` | Grava `dumps\read-config.json` na pasta de configuração, com `token`, `valid`, `error`, `path` e `manifest_text` (o conteúdo exato do `apps.json`). |
| `moonpool.exe write-config <file> [token]` | Substitui o `apps.json` pelo manifesto de `<file>`, se o manifesto for válido e, quando `token` é informado, o `apps.json` ainda corresponder a ele. |
| `moonpool.exe restore-config [index or filename]` | Sem argumento, grava a lista de instantâneos em `dumps\restore-config.json`. Com um, restaura esse instantâneo se for válido. |

```powershell frame="terminal"
& $mp restart my-app
```

```powershell frame="terminal"
& $mp dump my-app C:\temp\my-app.log
```

Um verbo desconhecido é ignorado. O programa também tem argumentos de inicialização próprios:
`moonpool.exe mcp` ([Configuração do MCP](/pt-br/automation/mcp-setup/)), `--uninstall` (usado por
Adicionar ou remover programas) e `--wait-pid <pid>` (usado quando o Moonpool se reinicia). Eles só são
aceitos como primeiro argumento, então um id de app como `--uninstall` não consegue acioná-los.

## Lendo o resultado

A linha de comando não imprime nada, então marque um comando com `--ticket <key>` (qualquer chave única,
em qualquer posição) e leia o resultado em `state.json` na pasta de configuração. Ela é
`%USERPROFILE%\.moonpool\moonpool-config\` no modo instalado, `<sua pasta .moonpool>\moonpool-config\`
em uma cópia portátil e `~/.config/Moonpool/` no Linux (veja
[Visão geral da configuração](/pt-br/apps/apps-json/#onde-fica-a-configuração)). `show` e `quit` não gravam
ticket.

```powershell frame="terminal"
& $mp launch my-app --ticket t1
```

O `state.json` tem `apps`, `statuses` (`id`, `running`, `managed`, `mcpRunning`, `mcpSeen` por app) e
`tickets`. O Moonpool em execução o reescreve a cada poucos segundos e após cada comando, e não o exclui
ao encerrar, então um arquivo restante não significa que o Moonpool esteja em execução. Para perguntar se
está, ou para obter a lista de apps ao vivo, use os verbos `ping` e `list` do canal de controle
([Verbos de controle](/pt-br/automation/control-verbs/)) ou as ferramentas MCP. Consulte o seu ticket até
`status` deixar de ser `pending`:

| `status` | Significado |
| --- | --- |
| `pending` | Recebido; o Moonpool ainda está agindo sobre ele. |
| `ok` | Concluído. Para `dump`, `read-config`, `write-config`, `restore-config` e `paths`, `detail` contém o caminho, o token ou o relatório. |
| `error` | Falhou; `detail` diz por quê, por exemplo `unknown app id: x`, `did not reach running in time`, `unknown command`. |

Cada ticket é `{ ticket, action, arg, status, detail, ts }` com `ts` em milissegundos Unix:

```json title="state.json (tickets entry)"
{
  "ticket": "t1",
  "action": "launch",
  "arg": "my-app",
  "status": "error",
  "detail": "did not reach running in time",
  "ts": 1767225600000
}
```

Os tickets concluídos são descartados após 24 horas, e a lista é reduzida em direção a 50 entradas quando
os tickets concluídos têm pelo menos 5 minutos.

Um agente com suporte a MCP pode dispensar a consulta periódica: veja
[Configuração do MCP](/pt-br/automation/mcp-setup/).

## Veja também

- [Agentes de IA: início rápido](/pt-br/automation/quick-start/)
- [Verbos de controle](/pt-br/automation/control-verbs/)
