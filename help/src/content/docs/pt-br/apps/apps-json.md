---
title: "Edite o apps.json: onde ele fica, como recarregá-lo e recuperá-lo"
description: "Encontre o arquivo apps.json que o Moonpool lê para cada app gerenciado, edite-o no editor de apps ou manualmente, recarregue-o e recupere-se de uma edição errada."
---

Cada app que o Moonpool gerencia é uma entrada no `apps.json`. Você pode editá-lo pelo editor de apps (o
diálogo Adicionar app e Editar app) ou manualmente. Os dois gravam o mesmo arquivo. Alguns resultados de
ferramentas e mensagens chamam esse arquivo de manifesto.

## Onde fica a configuração

| Modo | Pasta de configuração |
| --- | --- |
| Instalado (Windows) | `%USERPROFILE%\.moonpool\moonpool-config\` |
| Portátil | `moonpool-config\` ao lado do `moonpool.exe` (dentro da pasta `.moonpool\`) |
| Linux | `$XDG_CONFIG_HOME/Moonpool/`, ou então `~/.config/Moonpool/` |

O `apps.json` está nessa pasta, ao lado destes itens:

| Item | Finalidade |
| --- | --- |
| `apps.json.history\` | Anel de reversão com os últimos 10 arquivos `apps.json` válidos. |
| `settings.json` | Configurações do app. Veja [settings.json](/pt-br/data/settings-json/). |
| `cli-output\<id>\` | Logs de sessão por app. Veja [Logs](/pt-br/data/logs/). |
| `moonpool.log` | Log de depuração, enquanto **Gravar informações de depuração em um arquivo** estiver ligado. |
| `icons\` | Ícones `<id>.png` opcionais (também `.ico`, `.svg`, `.jpg`, `.jpeg`, `.webp`) que substituem os padrão. |
| `state.json` | Instantâneo do status ao vivo, atualizado a cada poucos segundos. |
| `dumps\` | Arquivos gravados pelos verbos `dump`, `read-config` e `restore-config`. |
| `mcp_seen.json` | Quais apps já tiveram um auxiliar MCP. |
| `window-state.json` | O tamanho e a posição da janela do hub. |
| `AI-README.md` | O guia para agentes de IA, reescrito a cada inicialização. |

Quais deles incluir no backup está em [Backup e recuperação](/pt-br/data/backup-and-recovery/#a-pasta-de-configuração).

Na primeira execução, o Moonpool cria o `apps.json` com entradas de exemplo. Um arquivo que já existe
nunca é sobrescrito.

## Edição

- **Diálogo.** Use **Adicionar app** no menu **...** no topo da barra lateral. Para alterar um app, use o
  lápis na linha dele ou clique com o botão direito nele e escolha **Editar**. O diálogo valida e salva
  imediatamente.
- **Manualmente.** **Editar apps.json** no mesmo menu abre o arquivo no seu editor padrão. Salve-o e
  depois escolha **Recarregar** no menu (ou pressione F5 ou Ctrl+R).

As edições manuais não são aplicadas até você recarregar. Recarregar apenas lê o arquivo; não o reescreve.

Salvar pelo diálogo reescreve o arquivo inteiro em uma forma normalizada e indentada. As chaves que o
Moonpool não conhece são descartadas, e o JSON não tem comentários, então guarde anotações no campo
`note`.

## Formato

O arquivo é um array JSON de objetos. Quatro chaves são obrigatórias em cada entrada: `id`, `name`,
`group`, `type`. Todo o resto é opcional. Veja [Campos do app](/pt-br/apps/fields/).

```json title="apps.json"
[
  { "id": "site", "name": "Site", "group": "Web apps", "type": "web",
    "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
    "url": "http://localhost:5173", "openBrowser": true }
]
```

Os grupos aparecem na barra lateral na ordem em que aparecem pela primeira vez no arquivo.

## O que o recarregamento faz

Recarregar substitui a lista em memória do Moonpool pelo conteúdo do arquivo. Iniciar, Parar e Reiniciar
leem a entrada quando você clica neles, então um `command`, `cwd`, `env` ou configuração de encerramento
editado vale na próxima vez que você iniciar ou reiniciar esse app. Recarregar nunca reinicia nada: um
app que já está em execução continua rodando com as configurações com que foi iniciado.

## Validação

O Moonpool valida o arquivo inteiro ao carregar, a cada salvamento e a cada gravação de um agente. Uma
única entrada ruim rejeita o arquivo inteiro.

| Regra | O erro contém |
| --- | --- |
| Não é JSON válido, falta uma chave obrigatória ou um valor tem o tipo errado | a mensagem do analisador JSON |
| `id` está vazio, começa com `-` ou tem caracteres diferentes de letras, dígitos, `.`, `_`, `-` | `invalid id` |
| Duas entradas compartilham um `id` | `duplicate app id` |
| `name` está em branco | `has an empty name` |
| `group` está em branco | `has an empty group` |
| `type` não é `desktop`, `web`, `static` nem `cli` | `unknown type` |
| `port` é `0` (um `port` acima de 65535 falha ao ser analisado) | `invalid port 0` |
| Entrada `static` sem `url` | `requires a url` |
| Qualquer outro tipo sem `command` | `requires a command` |

Os erros identificam a entrada pela posição, por exemplo:

```text
apps.json entry 2 (site) requires a command
```

### O id

O `id` é a chave permanente da entrada. Ele dá nome à pasta de logs e ao arquivo de ícone, e é o que você
passa para `moonpool.exe launch <id>` e para os agentes. O diálogo o deriva do nome quando você adiciona
um app. Ele converte o nome para minúsculas, transforma cada sequência de caracteres diferentes de `a` a
`z` e `0` a `9` em um único `-` e remove os `-` das duas pontas. Um resultado vazio vira `app`. Se o id já
estiver em uso, acrescenta `-2`, `-3` e assim por diante. Nunca muda o id depois, então renomear um app
mantém o id. O nome `Habit Tracker` recebe o id `habit-tracker`.

## Se o arquivo estiver com problema

- **Ao recarregar**, um arquivo que falha na validação é deixado intacto e o Moonpool mantém a última
  lista que carregou. Um banner sobre a barra lateral mostra o erro, com um botão para abrir o arquivo; a
  lista continua utilizável, mas esmaecida. Veja
  [Quando o apps.json tem um erro](/pt-br/using/hub-window/#quando-o-appsjson-tem-um-erro).
- **Na inicialização**, um arquivo quebrado significa que não há lista para manter, então o Moonpool
  inicia sem apps e o banner avisa. Corrija o arquivo e escolha **Recarregar**, ou restaure um
  instantâneo (abaixo, ou pela ferramenta `moonpool_restore_config`).
- Em qualquer caso, os salvamentos pelo diálogo (e renomear, excluir, definir ícone) são recusados até o
  arquivo carregar de novo, para que o arquivo quebrado nunca seja sobrescrito. Corrija o arquivo e
  escolha **Recarregar**.
- **Pelo diálogo, por um agente ou em uma restauração**, uma alteração inválida é rejeitada e o arquivo
  no disco permanece como estava.

O Moonpool mantém as últimas 10 versões boas do `apps.json` em `apps.json.history\`. Como reverter está em
[Backup e recuperação](/pt-br/data/backup-and-recovery/#reverter-o-appsjson).
Os sintomas e as soluções estão em [Solução de problemas](/pt-br/support/troubleshooting/#appsjson-tem-um-erro).

## Agentes

Um agente de IA deve alterar o `apps.json` pelas ferramentas MCP do Moonpool em vez do arquivo, para que
uma gravação desatualizada ou inválida seja rejeitada e um agente em sandbox nunca edite uma cópia
privada. Veja [Ferramentas MCP](/pt-br/automation/mcp-tools/#configuração).
