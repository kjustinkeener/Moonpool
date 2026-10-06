---
title: "Referência do canal de controle e dos verbos do Moonpool"
description: "Como o canal de controle do Moonpool (pipe nomeado ou socket Unix) funciona, o protocolo dele e os verbos a que responde, com argumentos e respostas."
---

## Onde ele escuta

Cada cópia do Moonpool tem o próprio canal, então o Moonpool instalado e qualquer cópia portátil podem
rodar lado a lado sem responder um pelo outro. No Windows, o Moonpool instalado escuta no pipe nomeado
`\\.\pipe\moonpool`. Uma cópia portátil acrescenta um id feito a partir da pasta dela:
`\\.\pipe\moonpool-<id>`.

`<id>` são 8 dígitos hexadecimais derivados do caminho da pasta `moonpool-config` da cópia, então ele
permanece o mesmo para essa pasta entre reinicializações e atualizações, e muda se você mover a pasta. O
`moonpool.exe` de uma cópia, incluindo `moonpool.exe mcp`, sempre encontra o canal da própria cópia.

No Linux e no macOS, ele escuta em um socket de domínio Unix, com modo `0600`:

| Caso | Caminho do socket |
| --- | --- |
| Normal | `$XDG_RUNTIME_DIR/moonpool.sock` quando essa variável está definida; senão, `moonpool.sock` na pasta de configuração do Moonpool |
| Modo portátil | `moonpool.sock` na pasta de configuração da cópia portátil, para que uma cópia portátil nunca colida com uma instalada |
| Caminho longo demais para um socket (cerca de 100 caracteres) | `/tmp/moonpool-<uid>/moonpool.sock`, em um diretório que só você pode abrir (`moonpool-<id>.sock` para uma cópia portátil) |

Um arquivo de socket deixado por uma falha é detectado e substituído na próxima inicialização. Um socket
em que algo ainda responde nunca é tomado. O arquivo é removido quando o Moonpool encerra normalmente.

O canal também é como o [servidor MCP](/pt-br/automation/mcp-setup/) sabe se o Moonpool está em execução:
se um `ping` é respondido, está; se o pipe ou socket não existe, não está. Os mesmos verbos também são
acessíveis pela [linha de comando](/pt-br/automation/command-line/), exceto os verbos de diagnóstico
abaixo.

## Protocolo

Um objeto JSON por linha de entrada, uma linha JSON de saída, em ordem. Uma conexão pode carregar muitas
requisições.

```json title="request"
{"cmd": "restart", "args": ["my-app"]}
```

```jsonl title="replies"
{"ok": true, "result": "..."}
{"ok": false, "error": "unknown app id: ..."}
```

Uma requisição e a resposta dela a partir do PowerShell:

Para uma cópia portátil, use o nome do pipe dela (`moonpool-<id>`, mostrado pelo verbo `paths`) no lugar
de `moonpool`.

```powershell frame="terminal"
$p = New-Object System.IO.Pipes.NamedPipeClientStream('.', 'moonpool', 'InOut')
$p.Connect(2000)
$w = New-Object System.IO.StreamWriter($p); $w.AutoFlush = $true
$r = New-Object System.IO.StreamReader($p)
$w.WriteLine('{"cmd":"ping"}')
$r.ReadLine()
```

```json title="reply"
{"ok":true,"result":"pong"}
```

- `args` é uma lista de strings e pode ser omitido. Os outros campos são ignorados.
- `result` é uma string ou null. Os verbos que retornam dados estruturados os retornam como uma string
  JSON.
- Uma linha que não é JSON válido recebe `{"ok": false, "error": "bad request: ..."}`.
- Um `cmd` desconhecido recebe `unknown cmd: <name>`.
- Um verbo que passa pela janela (`launch`, `stop`, `restart`, `reload`, `refresh-icons`, `help`,
  `open-window`) é respondido quando a ação termina, ou com um erro de tempo limite após 45 s. Se a
  interface da janela do hub não tiver carregado, falha de imediato com `frontend not loaded`.
- Um Moonpool que inicia enquanto um anterior ainda está saindo tenta de novo vincular o canal por cerca
  de 8 segundos. Se ainda assim não conseguir, registra isso e continua em execução sem ele.

## Verbos

| Verbo | Args | Resultado |
| --- | --- | --- |
| `ping` | nenhum | `pong`. Somente canal. |
| `list` | nenhum | String JSON `{"apps": [...], "statuses": [...]}` lida da memória do hub em execução, com a mesma forma de `apps` e `statuses` do `state.json`. Acrescenta `"statusNotReady": true` quando há apps registrados mas a primeira verificação de status ainda não ocorreu. Enquanto o `apps.json` falha ao carregar, acrescenta `"manifestError": "<message>"` (os apps são então a última lista que carregou) e, quando nenhuma lista carregou desde a inicialização, `"manifestLoaded": false`. Somente canal. |
| `show` | nenhum | null. Traz a janela para a frente. |
| `quit` | nenhum | null. Encerra o Moonpool. |
| `launch` | `<id>` | null em caso de sucesso, ou `opened` para uma entrada `static` com apenas uma `url`. Erros: `unknown app id: <id>`, `did not reach running in time`. |
| `stop` | `<id>` | null em caso de sucesso, ou `stopped` para uma entrada `static` com apenas uma `url`. Erro: `still running after stop`. |
| `restart` | `<id>` | Mesmos resultados e erros que `launch`. |
| `reload` | nenhum | null em caso de sucesso. |
| `refresh-icons` | nenhum | null em caso de sucesso. |
| `help` | nenhum | null. Abre a janela de Ajuda. |
| `dump` | `<id>` [`out-path`] | Caminho do log de sessão do app, ou da cópia em texto simples em `out-path`. |
| `paths` | nenhum | Relatório de várias linhas das pastas e do exe que o hub usa. |
| `read-config` | nenhum | Caminho de `dumps\read-config.json`, que contém `token`, `valid`, `error`, `path`, `manifest_text`. |
| `write-config` | `<source-file>` [`token`] | O novo token de versão. Erros: `stale token: ...`, `rejected invalid manifest: ...`, `cannot read source ...`. |
| `restore-config` | [`index` ou `filename`] | Sem argumento: caminho de `dumps\restore-config.json` (`count`, `snapshots`). Com um: `restored <file> (<n> apps); new version token <token>`. |
| `argv` | os argumentos da linha de comando | null, de imediato. Executa-os exatamente como um segundo `moonpool.exe <args>` desta cópia faria, incluindo `--ticket`. É assim que essa segunda inicialização entrega os argumentos antes de terminar. |

Exemplos de trocas:

```jsonl title="request, reply"
{"cmd": "launch", "args": ["nope"]}
{"ok": false, "error": "unknown app id: nope"}

{"cmd": "restore-config", "args": ["1"]}
{"ok": true, "result": "restored 1767225600000.json (6 apps); new version token <token>"}
```

`write-config` e `restore-config` carregam o novo manifesto de imediato, registram um instantâneo em
`apps.json.history\` e atualizam a janela.

## Verbos de diagnóstico (testes)

Somente canal: a linha de comando não os aceita. Todos funcionam no Windows, Linux e macOS exceto
`screenshot`, que é exclusivo do Windows e responde `screenshot is not supported on this
platform (Windows only)` nos demais.

| Verbo | Args | Resultado |
| --- | --- | --- |
| `screenshot` | [`window`] [`max_dim`] | Somente Windows. Base64 de um PNG dessa janela do Moonpool (`main` por padrão). `max_dim` opcional limita o lado mais longo em pixels (ajustado para 320-2400, padrão 320; a ferramenta MCP sempre usa o padrão). Um `max_dim` não inteiro é um erro. Janelas permitidas: `main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`. Erros: `unknown window '<name>'`, `window '<name>' is not open`. Não é gravado em disco. |
| `open-window` | `<kind>` [`<id>`] | null. Abre uma janela como o item de menu dela faz. `kind`: `settings`, `about`, `installer`, `help`, `themes`, `editor` (um `<id>` opcional abre o diálogo Editar app desse app; sem ele abre Adicionar app), `terminal` (`<id>` obrigatório: seleciona a aba de terminal desse app e alarga o hub para o painel CLI aparecer; não o inicia), `cli` (apenas alarga o hub). Erros: `unknown window kind '<kind>'`, `terminal needs an app id`, `unknown app id: <id>`. Respondido pela janela do hub, como `launch`. |
| `window-state` | [`window`] | String JSON: `{"open":false}`, ou `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. |
| `stop-mcp` | `<id>` | `stopped`. Encerra o auxiliar `<processName> mcp` do app, não o app. Erros: `missing app id`, `unknown app id: <id>`. |
| `reset-mcp-seen` | [`<id>`] | `<id>: cleared` ou `<id>: was not marked seen`; sem id, `cleared <n> entries`. Limpa os avistamentos lembrados de auxiliares MCP. |

O `--ticket` da linha de comando e os registros de resultado do `state.json` pertencem ao outro canal;
veja [Linha de comando](/pt-br/automation/command-line/#lendo-o-resultado). As requisições do canal
recebem a resposta na própria resposta.

## Veja também

- [Linha de comando](/pt-br/automation/command-line/)
- [Agentes de IA: início rápido](/pt-br/automation/quick-start/#a-mesma-ação-de-três-formas)
