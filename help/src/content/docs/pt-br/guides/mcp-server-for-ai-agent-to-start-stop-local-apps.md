---
title: "Dar a um agente de IA (Claude Code, Codex, Cursor) um servidor MCP para iniciar e parar apps locais"
description: "Registre o Moonpool como servidor MCP para o Claude Code, o Codex ou o Cursor iniciarem, pararem e reiniciarem seus servidores de desenvolvimento e lerem a saída."
---

Um agente de IA para programar costuma executar seu servidor de desenvolvimento digitando `npm run dev`
no próprio shell. Isso pode travar o agente, deixar um processo órfão ocupando a porta ou iniciar uma
segunda cópia de algo que você já tem rodando. Um servidor MCP permite que o agente chame ferramentas
para iniciar e parar o app que você já configurou, em vez de reconstruir a linha de comando dele.

## O jeito do Moonpool

O executável do Moonpool é o próprio servidor MCP: registre o `moonpool.exe` com o único argumento `mcp`
como servidor stdio. Quando o app está no `apps.json`, o agente o inicia pelo id.

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

Registre o servidor. No Claude Code, é um comando (Moonpool instalado; use o caminho completo do seu
próprio exe):

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

Hosts que leem um arquivo JSON de servidores MCP, como o `mcp.json` do Cursor, aceitam o mesmo formato
(com as barras invertidas dobradas):

```json title="mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

Para o Codex, adicione na configuração dele (`~/.codex/config.toml`) um servidor com o mesmo comando e o
argumento `mcp`:

```toml title="config.toml"
[mcp_servers.moonpool]
command = 'C:\Users\you\.moonpool\moonpool.exe'
args = ["mcp"]
```

O arquivo e os nomes das chaves exatos pertencem a cada host, então consulte a documentação de MCP dele
se a sua versão for diferente. O Moonpool precisa apenas do caminho completo do `moonpool.exe` e de `mcp`
como argumento. Reinicie o host depois.

## O que o agente pode fazer

As ferramentas aparecem como `moonpool_*`. As do dia a dia:

| Ferramenta | Uso |
| --- | --- |
| `moonpool_list_apps` | Encontrar o id de um app e ver se ele está em execução. |
| `moonpool_start_app` | Iniciar um app pelo id e abrir a aba de terminal dele. |
| `moonpool_stop_app` | Pará-lo, incluindo os processos filhos. |
| `moonpool_restart_app` | Parar, esperar a porta ser liberada e iniciar. Use depois de uma mudança no código. |
| `moonpool_app_output` | Ler o que o app imprimiu, com `tail_lines` para limitar. |
| `moonpool_bootup_launcher` | Iniciar o próprio Moonpool se ele não estiver em execução. |

Um ciclo típico é `moonpool_restart_app` e depois `moonpool_app_output`. As demais ferramentas (ler e
gravar `apps.json`, capturas de tela) estão em [Ferramentas MCP](/pt-br/automation/mcp-tools/).

## Se não funcionar

Se todas as ferramentas respondem `Moonpool is not running - call moonpool_bootup_launcher first` (o
Moonpool não está em execução: chame antes moonpool_bootup_launcher), o Moonpool ainda não foi iniciado.
Uma edição que não aparece geralmente significa que o agente está olhando outro `apps.json`: chame
`moonpool_launcher_paths`. Veja
[Se as ferramentas não funcionarem](/pt-br/automation/mcp-setup/#se-as-ferramentas-não-funcionarem).

## Veja também

- [Configuração do MCP](/pt-br/automation/mcp-setup/)
- [Ferramentas MCP](/pt-br/automation/mcp-tools/)
- [Agentes de IA: início rápido](/pt-br/automation/quick-start/)
- [Executar um servidor de desenvolvimento npm em segundo plano no Windows](/pt-br/guides/run-npm-dev-server-in-background-windows/)
