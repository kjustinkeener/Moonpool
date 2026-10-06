---
title: "Conecte um agente de IA ao Moonpool por MCP"
description: "Registre o moonpool.exe mcp como servidor MCP stdio no seu host, instalado ou portátil, e saiba como o Moonpool acompanha o auxiliar MCP próprio de um app."
---

O executável do Moonpool é o próprio servidor MCP. Registre-o no host como um servidor stdio que executa
o `moonpool.exe` com o único argumento `mcp`.

## Registrar o servidor

Instalado, o programa é `%USERPROFILE%\.moonpool\moonpool.exe`. Portátil, é o `moonpool.exe` dentro da sua
pasta `.moonpool\`. Use esse caminho completo como `command`. Para um host que lê um `.mcp.json`:

```json title=".mcp.json" {5}
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

Em um arquivo JSON, as barras invertidas devem ser dobradas, como acima. Um host com registro por linha de
comando, como o Claude Code, pode adicioná-lo em uma etapa:

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

O servidor se anuncia como `moonpool`, fala a revisão `2025-06-18` do protocolo MCP e expõe apenas
ferramentas (não lista recursos nem prompts). As ferramentas aparecem para o agente como `moonpool_*`;
veja [Ferramentas MCP](/pt-br/automation/mcp-tools/).

## Mais de um Moonpool

O Moonpool instalado e cada cópia portátil são inicializadores separados, cada um com os próprios apps, e
todos podem rodar ao mesmo tempo. O `moonpool.exe mcp` de uma cópia sempre controla essa cópia. Para um
agente usar várias, registre cada uma com um nome distinto, apontando para o exe dessa cópia:

```json title=".mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    },
    "moonpool-work": {
      "type": "stdio",
      "command": "D:\\Work\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

```powershell frame="terminal"
claude mcp add moonpool-work -- "D:\Work\.moonpool\moonpool.exe" mcp
```

Registrar duas cópias com o mesmo nome faz uma substituir a outra na maioria dos hosts. Os nomes das
ferramentas são os mesmos para todas as cópias, então o host as distingue pelo nome com que você as
registra. Uma cópia portátil também se anuncia como `moonpool (<folder>)` e as instruções do servidor dela
nomeiam a pasta, para que o agente veja com qual cópia está falando.

## Observações

- `moonpool.exe mcp` nunca abre uma janela e nunca inicia o instalador. Ele termina quando o host fecha a
  entrada dele.
- Ele usa a pasta de configuração e o canal de controle do exe a partir do qual foi iniciado, então um exe
  portátil lê os dados da pasta portátil e controla essa cópia portátil. Um exe conta como portátil apenas
  enquanto `moonpool.portable` estiver ao lado dele. Qualquer outro `moonpool.exe`, onde quer que esteja,
  usa a pasta do Moonpool instalado (`%USERPROFILE%\.moonpool\moonpool-config\`) e controla o Moonpool
  instalado.
- A maioria das ferramentas precisa de um Moonpool em execução. Se ele não estiver, o agente pode chamar
  `moonpool_bootup_launcher` primeiro.
- `moonpool_launcher_paths` mostra as pastas que o hub usa ao lado das que o processo MCP resolve. Uma
  diferença significa que o agente está olhando um `apps.json` diferente do que o hub usa.

## Hosts em sandbox

Alguns hosts executam suas ferramentas dentro de uma sandbox empacotada (Store/MSIX) que redireciona o
AppData para uma cópia privada por pacote. O Moonpool detecta isso quando a pasta de configuração ou o exe
dele se resolve sob um caminho como `...\Packages\<package>\LocalCache\...`.

Ele também detecta isso quando o canal de controle responde mas o `state.json` não pode ser lido. As
ferramentas que leem ou gravam arquivos (`moonpool_app_output`, `moonpool_read_config`,
`moonpool_write_config`, `moonpool_restore_config`) então retornam um erro que nomeia a causa, em vez de
dados vazios ou desatualizados. As ferramentas que só usam o canal de controle, como `moonpool_list_apps`,
não são bloqueadas enquanto o canal estiver acessível. Se a sandbox também ocultar o canal, as ferramentas
informam a sandbox em vez de "Moonpool is not running" (o Moonpool não está em execução). Use a
[linha de comando](/pt-br/automation/command-line/) a partir de um shell fora da sandbox.

## Apps que têm o próprio servidor MCP

Muitos apps no Moonpool são acessados por um host MCP por meio de um processo auxiliar `<exe> mcp`. O
Moonpool procura um processo cujo nome corresponda ao `processName` do app e cujo primeiro argumento seja
`mcp`, como `notes-app.exe mcp`. Se o servidor roda com outro nome, como uma cópia renomeada, defina o
curinga `mcpProcessName` do app (veja [Campos](/pt-br/apps/fields/#mcpprocessname)); um processo que
corresponda a ele conta sem o argumento `mcp`.

- Enquanto um está conectado, a barra lateral do app mostra uma subfila MCP como em execução, e
  `moonpool_list_apps` acrescenta `[mcp: running]` à linha do app. O auxiliar não conta como o próprio
  app em execução.
- Depois que um auxiliar é visto, o Moonpool o lembra (em `mcp_seen.json` na pasta de configuração), então
  a subfila MCP continua visível como parada, e `moonpool_list_apps` mostra `[mcp: stopped]`, depois que o
  auxiliar termina.
- A subfila MCP é controlada pela configuração `showMcpProcesses`
  ([Janela de Configurações](/pt-br/using/settings/)).
- `moonpool_stop_mcp_server` encerra o auxiliar e deixa o app em paz. Não há a operação inversa de iniciá-lo:
  o host dono do auxiliar o inicia de novo na próxima chamada de ferramenta dele.

## Se as ferramentas não funcionarem

- **O host não mostra nenhuma ferramenta `moonpool_*`.** Verifique se `command` é o caminho completo do
  `moonpool.exe` e se `args` é `["mcp"]`, e reinicie o host.
- **Todas as ferramentas dizem que o Moonpool não está em execução.** Inicie o Moonpool, ou chame
  `moonpool_bootup_launcher`. Confirme que o exe registrado é a cópia que você está executando.
- **Uma edição não aparece.** Chame `moonpool_launcher_paths` e compare as pastas do hub com as do
  processo MCP. Veja [Hosts em sandbox](#hosts-em-sandbox).

Mais em [Solução de problemas](/pt-br/support/troubleshooting/#erros-de-mcp-e-de-scripts).

## Veja também

- [Dar a um agente de IA (Claude Code, Codex, Cursor) um servidor MCP para iniciar e parar apps locais](/pt-br/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/)
- [Ferramentas MCP](/pt-br/automation/mcp-tools/)
- [Agentes de IA: início rápido](/pt-br/automation/quick-start/)
