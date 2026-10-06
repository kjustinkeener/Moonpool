---
title: "Execute o Moonpool a partir de um pen drive ou de uma pasta sincronizada"
description: "Mantenha o Moonpool e todos os dados dele em uma única pasta móvel para levá-lo em um pen drive ou sincronizá-lo, e execute várias cópias lado a lado."
---

O modo portátil mantém o Moonpool e tudo o que ele grava dentro de uma única pasta `.moonpool\`, para que
você possa levá-lo em um pen drive ou colocá-lo em uma pasta sincronizada e executá-lo em qualquer PC.

## Como funciona

Quando você instala no modo portátil, o Moonpool cria uma pasta `.moonpool\` dentro do local que você
escolher. Essa pasta contém o programa, a sua configuração e o conteúdo da ajuda. Nada é gravado no
AppData do Windows, então mover ou copiar a pasta leva toda a sua configuração junto.

```text
<chosen location>\.moonpool\
```

## O que é diferente do instalado

| | Instalado | Portátil |
| --- | --- | --- |
| Programa | `%USERPROFILE%\.moonpool\moonpool.exe` | `<chosen location>\.moonpool\moonpool.exe` |
| Pasta de configuração | `%USERPROFILE%\.moonpool\moonpool-config\` | `<chosen location>\.moonpool\moonpool-config\` |
| Perfil de navegador da janela, tamanho e posição da janela | Na pasta de configuração | Na pasta de configuração, então também viajam |
| Menu Iniciar, atalho na área de trabalho, entrada de Adicionar ou remover programas | Sim | Nenhum |
| Atualizações | Substitui o próprio exe | O mesmo, dentro da pasta `.moonpool\`. Veja [Atualização](/pt-br/data/updating/#cópias-portáteis). |
| Remover | Adicionar ou remover programas ou `--uninstall` | Excluir a pasta |

Nenhum dos dois modos grava no AppData do Windows.

### Pastas sincronizadas

Você pode manter uma cópia portátil em uma pasta sincronizada (OneDrive, Dropbox e similares), mas
execute-a em um PC por vez. O Moonpool grava `state.json` a cada poucos segundos e registra logs enquanto
os apps rodam, então dois PCs executando a mesma pasta disputam os mesmos arquivos, e um conflito de
sincronização pode deixar um `apps.json` quebrado. Saia do Moonpool em um PC antes de iniciá-lo em outro.

## Várias cópias ao mesmo tempo

Um Moonpool roda por pasta. O Moonpool instalado e qualquer número de cópias portáteis, cada uma na
própria pasta, podem rodar ao mesmo tempo, e cada uma é totalmente separada: os próprios apps, ícone na
bandeja, janela, configurações, logs e [canal de controle](/pt-br/automation/control-verbs/).

- A dica da bandeja e o nome na barra de tarefas dizem qual cópia é qual: `Moonpool` para a instalada e
  `Moonpool (<folder>)` para uma portátil, em que `<folder>` é a pasta que você escolheu (a que contém
  `.moonpool\`).
- Iniciar a mesma cópia uma segunda vez traz a janela dela de volta em vez de abrir outra. Iniciar uma
  cópia diferente abre essa cópia.
- Para dar a um agente de IA mais de uma cópia, registre cada uma com o próprio nome; veja
  [Configuração do MCP](/pt-br/automation/mcp-setup/#mais-de-um-moonpool).
- Mover ou renomear uma pasta portátil dá a ela uma nova identidade (um novo nome de canal de controle).
  Saia dela antes de movê-la.
- As cópias não conhecem os apps umas das outras. Duas cópias que iniciam o mesmo servidor na mesma porta
  ainda vão conflitar, e um Parar que funciona por nome de processo ou porta pode encerrar algo que outra
  cópia iniciou; veja
  [Parar e reiniciar](/pt-br/apps/stop-and-restart/#vários-moonpools-ou-seus-próprios-processos).

## Faça seus apps viajarem também

Use o token `{MP_HOME}` no caminho de um app para que ele aponte para dentro da pasta portátil em vez de
um local fixo de uma única máquina. Em uma cópia portátil, `{MP_HOME}` é a pasta que contém o
`moonpool.exe`, que é a própria pasta `.moonpool\`, não a pasta que você escolheu:

```json title="apps.json"
{ "cwd": "{MP_HOME}/my-app" }
```

Aqui `{MP_HOME}/my-app` é `<chosen location>\.moonpool\my-app`. Um caminho que começa com `./` é ancorado
da mesma forma. Tokens e caminhos `./` também funcionam em um Moonpool instalado. Veja
[Caminhos e ambiente](/pt-br/apps/paths-and-environment/) para saber como os caminhos são resolvidos.

## Escolher portátil no instalador

O modo portátil é configurado a partir do cartão do instalador, que oferece **Instalar portátil** junto
com **Instalar o Moonpool**.

![O cartão de instalação: o link Instalar portátil fica sob o botão principal Instalar o Moonpool](../../../../assets/screenshots/installer-window.png)

Escolha uma pasta e o Moonpool cria a pasta `.moonpool\` ali, copia a si mesmo para dentro e inicia a nova
cópia com uma configuração nova.

O cartão também está no menu "..." como **Instalar o Moonpool…**, tanto no modo instalado quanto no
portátil. Usar **Instalar portátil** dali faz o Moonpool em execução encerrar e a nova cópia portátil
iniciar no lugar dele. O Moonpool de onde você partiu fica onde estava, então você pode iniciá-lo de novo
depois.

Uma cópia portátil começa do zero e não copia os seus apps existentes. Para trazê-los, saia da cópia
portátil e copie o `apps.json` manualmente:

| | Caminho |
| --- | --- |
| De (instalado) | `%USERPROFILE%\.moonpool\moonpool-config\apps.json` |
| Para (portátil) | `<chosen location>\.moonpool\moonpool-config\apps.json` |

Entradas com caminhos absolutos ainda funcionam no mesmo PC, mas não viajam. O diálogo Editar app as
marca como "não portátil".

## Como o Moonpool sabe que é portátil

Uma cópia é portátil enquanto um arquivo chamado `moonpool.portable` estiver ao lado do `moonpool.exe`
dela. Nada mais a marca, e nada é registrado no Windows.

Para remover uma cópia portátil, saia dela e exclua a pasta `.moonpool\`. O `--uninstall` só remove o
Moonpool instalado, nunca uma cópia portátil.
