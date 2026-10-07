---
title: "Referência das ferramentas MCP do Moonpool: parâmetros e resultados"
description: "Todas as ferramentas que o servidor MCP do Moonpool expõe aos agentes, com os parâmetros, o que retornam e os casos de erro que você pode encontrar."
---

Todas as ferramentas retornam texto, exceto `moonpool_screenshot`, que retorna uma imagem PNG. Uma falha
chega como um resultado de ferramenta marcado como erro, com o motivo em texto. Para a configuração, veja
[Configuração do MCP](/pt-br/automation/mcp-setup/).

As ferramentas que recebem `app_id` precisam do `id` do app no `apps.json`. Ele deve usar apenas letras,
dígitos, `.`, `_` e `-`, e não começar com `-`; caso contrário a chamada falha com "invalid app_id".

A maioria das ferramentas que agem sobre o hub falha com esta mensagem quando ele não está em execução.
`moonpool_bootup_launcher`, `moonpool_shutdown_launcher`, `moonpool_raise_launcher` e
`moonpool_launcher_paths` tratam esse caso por conta própria (veja as linhas delas). Para uma cópia
portátil, a mensagem nomeia a cópia, por exemplo `Moonpool (<folder>)`.

```text
Moonpool is not running - call moonpool_bootup_launcher first
```

(O Moonpool não está em execução: chame antes moonpool_bootup_launcher.)

As chamadas que esperam um resultado expiram após 45 segundos.

## Inicializador e apps

Exemplo de resultado de `moonpool_list_apps`:

```text
site  [running] (managed by Moonpool)  Site
notes-app  [stopped]  [mcp: stopped]  Notes App
```

| Ferramenta | Parâmetros | Comportamento |
| --- | --- | --- |
| `moonpool_list_apps` | nenhum | Uma linha por app: `id  [running]` ou `[stopped]`, `(managed by Moonpool)` quando aplicável, `[mcp: running]` ou `[mcp: stopped]` quando um auxiliar MCP foi visto, e depois o nome. É perguntado ao hub em execução pelo canal de controle (verbo `list`), então é ao vivo. Se o Moonpool não estiver em execução, falha com "Moonpool is not running" em vez de mostrar uma lista desatualizada. Logo após o Moonpool iniciar, antes da primeira verificação de status, os apps mostram `[status pending]`. Enquanto o `apps.json` tem um erro, o resultado começa com `apps.json has an error: <message>. This list is the last one that loaded; fix the file and call moonpool_reload_config.` Se o arquivo já estava quebrado quando o Moonpool iniciou, diz que nenhum app foi carregado e sugere também `moonpool_restore_config`. |
| `moonpool_bootup_launcher` | nenhum | Inicia o próprio Moonpool e espera até 30 s que o canal de controle responda. Retorna "Moonpool started" ou "Moonpool is already running". Se o novo processo terminar de imediato (passou o controle a um Moonpool que ainda estava encerrando), inicia mais um. Se algo ocupar o canal sem responder, informa que um processo do Moonpool pode estar travado. |
| `moonpool_shutdown_launcher` | nenhum | O mesmo que Sair no menu da bandeja. Espera até 30 s que o canal de controle desapareça. Retorna "Moonpool shut down" ou "Moonpool is not running". |
| `moonpool_raise_launcher` | nenhum | Traz a janela do Moonpool para a frente. Retorna "window shown". Se o Moonpool não estiver em execução, o inicia e retorna "Moonpool was not running; started it". |
| `moonpool_start_app` | `app_id` (obrigatório) | Inicia o app e abre a aba de terminal dele. Retorna "launched" quando está em execução, ou o motivo de não estar (`unknown app id: <id>`, `did not reach running in time` após 25 s). Para uma entrada `static` com apenas uma `url`, abre a página e também retorna "launched". |
| `moonpool_stop_app` | `app_id` (obrigatório) | Para o app. Retorna "stopped", ou um erro como `still running after stop` (após 15 s). |
| `moonpool_restart_app` | `app_id` (obrigatório) | Para, espera a porta e o processo serem liberados, inicia. Retorna "restarted". |
| `moonpool_app_output` | `app_id` (obrigatório), `tail_lines` (inteiro, padrão 200, mínimo 1) | A saída do terminal do app na sessão atual do Moonpool, sem códigos ANSI. Quando o log é mais longo que `tail_lines`, o texto começa com uma linha que informa o caminho do log completo. Falha com `no console output recorded for '<id>' (not launched this session)` se o app não foi executado. Se o log existe mas está vazio, retorna `(no output recorded for '<id>')`. |
| `moonpool_stop_mcp_server` | `app_id` (obrigatório) | Encerra o processo auxiliar MCP conectado do app e deixa o app em execução. Retorna "stopped". Não faz nada se o app não tem nem `processName` nem `mcpProcessName`. |
| `moonpool_refresh_app_icons` | nenhum | Busca todos os ícones dos apps de novo. Retorna "icons refreshed". |

## Configuração

Estas ferramentas leem e alteram o `apps.json` pelo hub, nunca o arquivo no disco. Uma gravação deve levar
o token da última leitura, um token desatualizado é rejeitado e o novo arquivo é validado antes de qualquer
coisa ser gravada. Passar pelo hub importa porque a um agente em um host em sandbox pode ser mostrada uma
cópia privada da pasta de configuração em vez da real.

| Ferramenta | Parâmetros | Comportamento |
| --- | --- | --- |
| `moonpool_read_config` | nenhum | Texto JSON com `manifest_text` (o conteúdo exato do arquivo), `token`, `valid`, `error` (null quando válido) e `path`. `token` é `none` quando o arquivo está ausente ou vazio. |
| `moonpool_write_config` | `manifest` (obrigatório, o texto completo do novo `apps.json`), `expected_token` (obrigatório, da última leitura) | Valida o manifesto e substitui o `apps.json`, depois o carrega. Retorna `apps.json updated; new version token <token>`. Um token desatualizado falha com `stale token: apps.json changed since it was read ...`. Um manifesto inválido falha com `rejected invalid manifest: ...`. Em ambos os casos o arquivo fica intacto. Um `expected_token` vazio é recusado. |
| `moonpool_restore_config` | `snapshot` (opcional) | Sem valor, texto JSON listando os instantâneos salvos do mais recente para o mais antigo (`index`, `filename`, `millis`, `app_count`, `valid`). Com um índice (1 = o mais recente) ou um nome de arquivo, valida esse instantâneo e o restaura. Retorna `restored <file> (<n> apps); new version token <token>`. Não precisa de token: uma restauração sobrescreve o arquivo atual de propósito. |
| `moonpool_reload_config` | nenhum | Lê o `apps.json` de novo. Retorna "apps.json reloaded". Se o arquivo não for analisado ou validado, falha com `apps.json has an error: ...` e o Moonpool mantém a última lista que carregou. |
| `moonpool_launcher_paths` | nenhum | Lista a pasta de configuração do hub, `apps.json`, `state.json`, o log, a pasta de dumps, a pasta de ícones, o indicador de portátil e o caminho do exe, e depois a pasta de configuração do processo MCP, `apps.json`, `state.json`, a pasta de dumps, o indicador de portátil e o caminho do exe (sem log nem ícones). Se o hub não estiver em execução, a metade dele diz `hub paths unavailable: ...` e a metade do MCP ainda é mostrada. Use quando uma edição não surtir efeito. |

## Avançado: ferramentas de teste

`moonpool_screenshot` é exclusiva do Windows; no Linux falha com "screenshot is not supported on
this platform". `moonpool_window_state` e `moonpool_reset_mcp_seen` funcionam em todas as plataformas.

`window` é um entre `main`, `settings`, `about`, `installer`, `editor`, `help` ou `themes`, e o padrão é
`main`. Um nome desconhecido falha com `unknown window '<name>'`.

| Ferramenta | Parâmetros | Comportamento |
| --- | --- | --- |
| `moonpool_screenshot` | `window` (opcional) | Captura o conteúdo próprio dessa janela do Moonpool como um PNG em linha, com no máximo 320 pixels no lado mais longo. O tamanho não pode ser aumentado pelo MCP. Falha com `window '<name>' is not open` se ela não estiver sendo exibida. Não consegue capturar nenhum outro app. |
| `moonpool_window_state` | `window` (opcional) | Texto JSON: `{"open":false}` quando a janela não está aberta; caso contrário, `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. Destinada a testes. |
| `moonpool_reset_mcp_seen` | `app_id` (opcional) | Somente para testes. Limpa o registro lembrado de "um auxiliar MCP foi visto" de um app, ou de todos os apps quando omitido, de modo que a subfila MCP da barra lateral volta a ficar oculta até que um auxiliar seja visto. |

## Veja também

- [Configuração do MCP](/pt-br/automation/mcp-setup/)
- [Linha de comando](/pt-br/automation/command-line/)
