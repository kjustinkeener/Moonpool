---
title: "Encontre e gerencie os logs de sessão e o log de depuração do Moonpool"
description: "Localize o log de sessão de cada app, o log de depuração, os dumps e o histórico de rolagem, e veja por quanto tempo cada um é mantido."
---

O Moonpool mantém quatro tipos de saída:

| Tipo | Onde | Mantido |
| --- | --- | --- |
| Log de sessão | `cli-output\<id>\<session-start-ms>.log` na pasta de configuração | A sessão atual sempre; as sessões anteriores conforme as regras de retenção abaixo |
| `moonpool.log` | A pasta de configuração | Só é gravado enquanto **Gravar informações de depuração em um arquivo** estiver ligado |
| Dump | Onde você pedir, ou o próprio caminho do log de sessão | Até você excluir |
| Histórico de rolagem | Na aba de terminal | 10.000 linhas, até o Moonpool encerrar |

A pasta de configuração está listada em [Onde fica a configuração](/pt-br/apps/apps-json/#onde-fica-a-configuração).

## Logs de sessão

Tudo o que um app imprime no terminal dele também é gravado em um arquivo de log:

```text
<config folder>\cli-output\<id>\<session-start-ms>.log
```

- Um arquivo por app por sessão do Moonpool. O número é o momento em que esse processo do Moonpool iniciou.
- Parar e reiniciar um app continua acrescentando ao mesmo arquivo. Uma linha separadora esmaecida marca
  onde cada nova execução começa, e a mesma marca aparece na aba de terminal:

  ```text title="1767225600000.log"
  Local:   http://localhost:5173/
  ---------- restarted 2026-10-05 09:14:02 ----------
  Local:   http://localhost:5173/
  ```

- Caracteres de um `id` diferentes de letras, dígitos, `-` e `_` viram `_` no nome da pasta. Assim, `.`
  vira `_`. Letras fora do inglês são mantidas.
- O arquivo guarda a saída bruta do terminal, incluindo os códigos de cor. Use um dump para obter texto simples.

Reabrir a aba de um app reproduz o log desta sessão, então você vê a saída anterior dele.

## Retenção

A retenção só diz respeito aos logs de sessões anteriores do Moonpool. Ela roda quando você inicia um app,
apenas para a pasta desse app, começando pelos mais antigos.

| **Manter os logs de saída dos apps entre sessões** (`cliLogging`) | O que acontece com os logs das sessões anteriores |
| --- | --- |
| desligado (padrão) | Excluídos na próxima inicialização do app. |
| ligado | Mantidos até o tamanho total da pasta passar **Retenção de logs por app** (`logRetentionMb`, padrão 10 MB); então os mais antigos são excluídos. |

O arquivo da sessão atual conta para esse total, mas nunca é excluído nem truncado. Assim, um único log
atual muito grande pode expulsar todos os mais antigos.

![A seção de log de Configurações: a caixa de manter logs, o tamanho de retenção por app em MB e a caixa do log de depuração, cada uma com uma linha de caminho de pasta](../../../../assets/screenshots/settings-logging-section.png)

1. **Manter os logs de saída dos apps entre sessões** é `cliLogging`. **Retenção de logs por app**, abaixo dela, é `logRetentionMb`.

## moonpool.log

Com **Gravar informações de depuração em um arquivo** (`debugLogging`) ligado, o Moonpool acrescenta
linhas com carimbo de data e hora a `moonpool.log` na pasta de configuração: carregamentos do
`apps.json`, inicializações (com o comando e a pasta), comandos de controle e erros. Ligue-o antes de
reproduzir um problema.

## Abrir e Copiar

Em [Configurações](/pt-br/using/settings/#coluna-da-direita-logs), sob cada grupo de logs:

- **Abrir a pasta de logs CLI** e **Abrir o log** abrem a pasta no seu gerenciador de arquivos.
- **Copiar o caminho da pasta de logs CLI** e **Copiar o caminho do arquivo de log** colocam o caminho na área de transferência.

Em uma aba de terminal, **Copiar tudo** copia todo o histórico de rolagem como texto.

## Dumps

O verbo `dump` entrega um log de sessão a partir de um script:

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" dump my-app C:\temp\my-app.log
```

Com um caminho de saída, ele grava uma cópia em texto simples, sem os códigos de cor. Sem ele, informa o
próprio caminho do log de sessão. Um agente obtém o mesmo texto, já limpo, por
`moonpool_app_output`. Veja [Linha de comando](/pt-br/automation/command-line/).
