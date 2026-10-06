---
title: "Entenda a barra lateral: pontos de status, grupos, filtro e menu da linha"
description: "Saiba o que cada linha da barra lateral mostra, como funcionam os pontos de status e os grupos, como filtrar apps, usar o menu de contexto da linha e redimensionar a barra."
---

A barra lateral lista todos os apps do `apps.json`, agrupados pelo campo `group` de cada app. Veja [Campos do app](/pt-br/apps/fields/).

## Linhas

Cada linha mostra um ponto de status, o ícone do app (ou um glifo de tipo se não houver ícone), o nome, a porta se definida (`:3000`) e os controles.

| Ponto | Significado |
| --- | --- |
| Fixo | em execução |
| Pulsando | iniciando: o Moonpool iniciou o app, mas ele ainda não foi detectado como ativo |
| Cinza | parado |

Passe o mouse sobre o ponto para ver a palavra.

![A barra lateral com dois apps web em execução destacados: pontos acesos e botões Parar](../../../../assets/screenshots/sidebar-running-narrow.png)

1. Dois apps em execução. Os pontos deles estão acesos e Parar (o quadrado) substitui Iniciar.

| Controle | O que faz |
| --- | --- |
| Lápis | Editar o app. |
| Reiniciar | Parar e iniciar de novo. Em um app parado, simplesmente o inicia. |
| Iniciar (reproduzir) | Inicia o app e abre a aba de terminal dele. Aparece quando o app está parado. |
| Parar (quadrado) | Para o app. Aparece enquanto ele está em execução ou iniciando. |

![Uma linha em execução, ampliada: ponto de status, ícone de tipo, nome e porta, depois os botões de editar, reiniciar e parar](../../../../assets/screenshots/sidebar-row-controls.png)

1. Ponto de status (aceso enquanto em execução).
2. Ícone de tipo.
3. Editar (lápis).
4. Reiniciar.
5. Parar (aparece no lugar de Iniciar enquanto em execução).

Enquanto uma inicialização ou parada está em andamento, os controles são substituídos por um indicador giratório (`Trabalhando...`).

Clicar no **nome** de um app abre ou foca a aba de terminal dele e nunca inicia nada. A aba de um app parado mostra o log desta sessão. Use Iniciar ou Reiniciar para colocá-lo em execução. Um app `static` com apenas uma `url` e sem `command` não tem terminal: Iniciar abre a URL no navegador.

### Dica

Passar o mouse sobre o nome mostra a `note` do app, se houver; caso contrário, o nome dele. Defina `note` no editor ou no `apps.json`.

### Subitem MCP

Quando um cliente de IA usou as ferramentas MCP próprias de um app, uma subfila esmaecida `Servidor MCP` aparece sob o app. O ponto dela fica aceso e a dica diz "Cliente MCP conectado" enquanto o cliente estiver conectado. Um botão de parar encerra esse processo.

A linha encontra o processo por `processName` mais o argumento `mcp`, ou pelo padrão `mcpProcessName` do app quando definido. Veja [campos](/pt-br/apps/fields/#mcpprocessname).

Oculte essas linhas com **Mostrar processos MCP** em Configurações. Veja
[Configuração do MCP](/pt-br/automation/mcp-setup/#apps-que-têm-o-próprio-servidor-mcp).

## Grupos

![Barra lateral ociosa com os cinco cabeçalhos de grupo destacados, cada um com a contagem de apps à direita](../../../../assets/screenshots/sidebar-groups-narrow.png)

- Clique no cabeçalho de um grupo para recolher ou expandir. A contagem ao lado é o número de apps exibidos. Os grupos recolhidos são lembrados.
- Dentro de um grupo, o app iniciado mais recentemente fica no topo. Os apps nunca iniciados mantêm a ordem do `apps.json`. Um app recém-iniciado brilha e sobe para o topo.

## Caixa de filtro

Digite em **Filtrar apps...** para restringir a lista. Ela corresponde ao nome do app e ao nome do grupo, sem diferenciar maiúsculas de minúsculas. Se nada corresponder, a lista mostra:

```text
Nenhum app corresponde a “<texto>”.
```

O app mostra aspas curvas em volta do texto, aqui e no aviso de Excluir abaixo.

## Menu do clique direito

Clique com o botão direito em uma linha para ver:

| Item | O que faz |
| --- | --- |
| Editar | Abre o editor de apps. |
| Renomear | Transforma o nome em um campo editável. **Enter** ou clicar fora salva, **Esc** cancela. Um nome vazio ou inalterado é ignorado. |
| Definir ícone... | Escolha um arquivo de imagem (png, jpg, jpeg, gif, svg, webp, ico) para usar como ícone. |
| Excluir | Pergunta `Excluir “<nome>”?` e remove a entrada do `apps.json`. Se o Moonpool estiver executando o app, ele é parado primeiro. |

**Esc** fecha o menu sem fazer nada.

## Redimensionar

Arraste o divisor entre a barra lateral e o painel CLI. A largura é limitada de 180 a 620 px (padrão 280) e é lembrada. O divisor fica travado enquanto o painel CLI está recolhido. Veja [Abas de terminal](/pt-br/using/terminal-tabs/).
