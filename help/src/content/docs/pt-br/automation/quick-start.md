---
title: "Deixe um agente de IA configurar e controlar o Moonpool: início rápido"
description: "Três formas de um agente de IA ou um script configurar e controlar o Moonpool, qual escolher para o seu agente e a mesma ação mostrada em cada uma."
---

Há três formas de entrar. Escolha conforme o que o seu agente consegue fazer.

| Você quer | Use | Comece aqui |
| --- | --- | --- |
| Que um agente encontre seus apps e os adicione, uma única vez | **Copiar o prompt** na tela vazia do hub | Abaixo |
| Que um agente inicie, pare e leia apps como chamadas de ferramentas | O servidor MCP, `moonpool.exe mcp` | [Configuração do MCP](/pt-br/automation/mcp-setup/) |
| Um script, ou um agente sem MCP | Verbos da linha de comando | [Linha de comando](/pt-br/automation/command-line/) |

## Copiar o prompt

Sem nenhuma aba aberta, o painel CLI mostra um prompt pronto ("É novo por aqui? Passe isto para um agente
de IA configurar seus apps"). **Copiar o prompt** o coloca na área de transferência. Cole-o no seu agente.
Ele aponta o agente para `AI-README.md` e `apps.json` na sua pasta de configuração e pede que encontre
seus apps e os registre. Quando terminar, escolha **Recarregar**.

O Moonpool reescreve o `AI-README.md` ao lado do `apps.json` a cada inicialização, então ele sempre
corresponde à versão que você executa. Não guarde suas próprias edições nele.

## A mesma ação de três formas

| Ação | Linha de comando | Verbo do canal de controle | Ferramenta MCP |
| --- | --- | --- | --- |
| Iniciar um app | `moonpool.exe launch <id>` | `launch` | `moonpool_start_app` |
| Parar um app | `moonpool.exe stop <id>` | `stop` | `moonpool_stop_app` |
| Reiniciar um app | `moonpool.exe restart <id>` | `restart` | `moonpool_restart_app` |
| Ler a saída de um app | `moonpool.exe dump <id> [out-path]` | `dump` | `moonpool_app_output` |
| Listar apps e status | ler `state.json` | `list` | `moonpool_list_apps` |
| Ler o `apps.json` de novo | `moonpool.exe reload` | `reload` | `moonpool_reload_config` |
| Ler o `apps.json` | `moonpool.exe read-config` | `read-config` | `moonpool_read_config` |
| Substituir o `apps.json` | `moonpool.exe write-config <file> [token]` | `write-config` | `moonpool_write_config` |
| Reverter o `apps.json` | `moonpool.exe restore-config [n]` | `restore-config` | `moonpool_restore_config` |
| Mostrar a janela | `moonpool.exe show` | `show` | `moonpool_raise_launcher` |
| Iniciar o Moonpool | `moonpool.exe` | nenhum | `moonpool_bootup_launcher` |
| Encerrar o Moonpool | `moonpool.exe quit` | `quit` | `moonpool_shutdown_launcher` |
| Mostrar as pastas em uso | `moonpool.exe paths` | `paths` | `moonpool_launcher_paths` |

A linha de comando não imprime nada; leia o resultado com um `--ticket` (veja
[Lendo o resultado](/pt-br/automation/command-line/#lendo-o-resultado)). O canal e o MCP respondem
diretamente.

## Quando as ferramentas de um agente falham

- `Moonpool is not running - call moonpool_bootup_launcher first` (o Moonpool não está em execução: chame
  antes moonpool_bootup_launcher): inicie o Moonpool, ou deixe o agente chamar essa ferramenta.
- Uma edição "não pegou": peça ao agente `moonpool_launcher_paths`. Se as pastas do hub e do MCP forem
  diferentes, o agente está lendo outro `apps.json`. Veja
  [Hosts em sandbox](/pt-br/automation/mcp-setup/#hosts-em-sandbox).
- Várias cópias do Moonpool: registre cada uma com o próprio nome. Veja
  [Mais de um Moonpool](/pt-br/automation/mcp-setup/#mais-de-um-moonpool).

Há um exemplo prático para o Claude Code, o Codex e o Cursor em
[Dar a um agente de IA um servidor MCP para iniciar e parar apps locais](/pt-br/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/).

Mais sintomas em [Solução de problemas](/pt-br/support/troubleshooting/#erros-de-mcp-e-de-scripts).
