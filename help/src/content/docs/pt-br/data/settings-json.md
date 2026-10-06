---
title: "Entenda o settings.json e repare um quebrado"
description: "Veja o formato do settings.json do Moonpool, quais chaves o Moonpool grava para você e como lê-lo e repará-lo quando o arquivo estiver quebrado."
---

As configurações do app inteiro ficam em `settings.json` na pasta de configuração (veja
[Onde fica a configuração](/pt-br/apps/apps-json/#onde-fica-a-configuração)). Altere-as na
[janela de Configurações](/pt-br/using/settings/), que lista cada configuração com a chave JSON e o
padrão. Os logs e a retenção deles estão na página [Logs](/pt-br/data/logs/).

## Formato

Um objeto JSON. As chaves que você omite assumem os padrões:

```json title="settings.json"
{
  "closeToTray": true,
  "transparency": 20,
  "debugLogging": true,
  "cliLogging": true,
  "logRetentionMb": 25
}
```

| Chave | Padrão |
| --- | --- |
| `closeToTray` | `false` |
| `minimizeToTray` | `true` |
| `showInTray` | `true` |
| `showInTaskbar` | `true` |
| `alwaysOnTop` | `false` |
| `transparency` | `0` (de 0 a 90) |
| `showStatusbar` | `true` |
| `showMcpProcesses` | `true` |
| `checkOnStartup` | `true` |
| `locale` | `"auto"` |
| `debugLogging` | `false` |
| `cliLogging` | `false` |
| `logRetentionMb` | `10` (mínimo 1) |

## Chaves gravadas para você

O Moonpool também guarda neste arquivo o zoom da interface (`uiScale`, de 0,5 a 3,0) e o idioma resolvido
(`localeResolved`). Você não precisa definir nenhum dos dois. O tema não está aqui: ele fica no
armazenamento do webview (veja [Temas, idioma e transparência](/pt-br/using/themes-and-language/)).

## Leitura e reparo

O Moonpool lê o arquivo na inicialização. As edições feitas enquanto ele roda não são aplicadas; saia
primeiro.

Se o arquivo estiver malformado, o Moonpool inicia com os padrões e se recusa a alterar configurações. O
erro termina com `Repair settings.json and restart Moonpool before changing settings` (repare o
settings.json e reinicie o Moonpool antes de alterar configurações). Corrija o arquivo, ou exclua-o para
redefinir todas as configurações, e inicie o Moonpool de novo.
