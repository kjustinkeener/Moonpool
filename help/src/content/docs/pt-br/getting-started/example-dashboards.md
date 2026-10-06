---
title: "Experimente os painéis de exemplo que acompanham o Moonpool"
description: "Abra os painéis de exemplo offline incluídos, veja onde ficam e como os apps de exemplo os referenciam, e adicione-os a uma configuração que você já tenha."
---

O Moonpool traz dentro do próprio programa um conjunto de painéis autônomos. Eles funcionam
totalmente offline, sem servidor e sem CDN.

| Painel | O que é |
| --- | --- |
| CSV explorer | Solte um arquivo CSV ou TSV; ele analisa as colunas e exibe os dados. |
| JSON explorer | Solte um JSON (arrays, objetos aninhados ou mapas). |
| Excel explorer | Solte um arquivo `.xlsx` ou `.xls`, processado offline. |
| Moonpool Docs | Um navegador de documentação em Markdown offline. |

## Onde ficam

Ao iniciar, o Moonpool grava os painéis em `{MP_HOME}\dashboards\examples`:

| Modo | Pasta |
| --- | --- |
| Instalado (Windows) | `%USERPROFILE%\.moonpool\dashboards\examples` |
| Portátil | `<sua pasta .moonpool, a que contém o moonpool.exe>\dashboards\examples` |
| Linux | `~/.config/Moonpool/dashboards/examples` (ou `$XDG_CONFIG_HOME/Moonpool/dashboards/examples`) |

A pasta `examples` pertence ao Moonpool: ela é substituída sempre que o Moonpool é atualizado,
então as edições feitas ali se perdem. Para personalizar um painel, copie a pasta dele e a pasta
compartilhada `_lib` para `dashboards` e aponte seu app para a cópia. O Moonpool nunca altera
mais nada em `dashboards`.

As versões anteriores à 0.3.16 gravavam os exemplos diretamente em `dashboards`. Essas cópias
ficam onde estão e não recebem mais atualizações; os apps que apontam para elas continuam
funcionando. Para obter as versões atualizadas, mude a `url` deles para o caminho
`dashboards/examples/...` indicado abaixo.

## Como os apps os referenciam

Cada um é um app `static` cuja `url` é uma URL `file:///` ancorada em `{MP_HOME}`:

```text
file:///{MP_HOME}/dashboards/examples/csv/index.html
```

`{MP_HOME}` se resolve para a pasta de instalação ou, no modo portátil, para a pasta do pacote,
então a entrada continua funcionando se você mover o pacote. URLs `file://` são permitidas. Veja
[Caminhos e ambiente](/pt-br/apps/paths-and-environment/).

## Os apps de exemplo só aparecem na primeira execução

As entradas de exemplo só são gravadas em `apps.json` quando ainda não existe nenhum arquivo de
configuração. Se você já tem um `apps.json`, adicione você mesmo as entradas dos painéis
(**Editar apps.json** no menu "...", depois **Recarregar**). Adicione estas quatro dentro do array
de nível superior, separadas das suas outras entradas por vírgulas:

```jsonc title="apps.json (excerpt)"
{
  "id": "csv-explorer",
  "name": "Sample CSV Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html",
  "openBrowser": true
},
{
  "id": "json-explorer",
  "name": "Sample JSON Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/json/index.html",
  "openBrowser": true
},
{
  "id": "xlsx-explorer",
  "name": "Sample Excel Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/xlsx/index.html",
  "openBrowser": true
},
{
  "id": "docs-browser",
  "name": "Moonpool Docs",
  "group": "Docs",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/docs/index.html",
  "openBrowser": true
}
```

O significado dos campos está em [Campos do app](/pt-br/apps/fields/).

## Veja também

- [Exemplos](/pt-br/apps/examples/): entradas mais completas para copiar.
- [Tipos de app](/pt-br/apps/types/#static)
