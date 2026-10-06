---
title: "Adicione um app ou servidor de desenvolvimento ao Moonpool"
description: "Registre um app local ou servidor de desenvolvimento com um comando de inicialização, uma pasta de trabalho e um ambiente para o Moonpool iniciá-lo e monitorá-lo."
---

Cada app no Moonpool é uma entrada com um comando de inicialização, uma pasta de trabalho e um ambiente
opcional. O Moonpool executa o comando em seu próprio terminal gerenciado.

## Adicionar um app

1. Abra o menu **...** no topo da barra lateral e escolha **Adicionar app**.
2. Digite um **name** e escolha um **group**.
3. Escolha o **type**: `web` (servidor em uma porta), `desktop` (app nativo), `static` (uma página) ou `cli` (um comando).
4. Defina o **command** e o **cwd** em que ele roda.
5. Preencha o que o tipo exige: **port** e **url** para web, **processName** para desktop,
   **url** para static. Um app `static` com apenas uma `url` não precisa de **command** nem de **cwd**.
6. Salve. O app aparece na barra lateral. Use o controle **Iniciar** dele para iniciá-lo.

![O seletor de type (1) e o campo port (2) no editor de apps, com cwd e command entre eles](../../../../assets/screenshots/edit-app-type-and-port.png)

1. O seletor de **type**; a dica dele diz como esse tipo é executado.
2. O campo **port**, usado pelos apps `web`.

O resultado é uma entrada no `apps.json`, por exemplo:

```json title="apps.json"
{
  "id": "my-api",
  "name": "My API",
  "group": "Dev",
  "type": "web",
  "command": "npm run dev",
  "cwd": "C:\\code\\my-api",
  "port": 3000,
  "url": "http://localhost:3000"
}
```

Clicar no nome de um app só abre a aba de terminal dele; veja [Estados do app](/pt-br/support/glossary/#estados-do-app).

## O editor de apps

- **Group.** Escolha um grupo da lista, ou escolha **+ Novo grupo...** e digite um nome.
  **voltar para a lista** volta para a lista. Um grupo em branco é salvo como `Apps`.
- Os **campos esmaecidos** não são usados pelo tipo selecionado. Eles ainda são salvos.
- **Salvar sem um nome** mostra `o nome é obrigatório.`
- **Esc** ou fechar o editor com alterações não salvas pergunta "Descartar suas alterações?".
- Para alterar um app depois, use o lápis na linha dele, ou clique com o botão direito nele e escolha **Editar**.

## Editar manualmente

Escolha **Editar apps.json** no mesmo menu, salve o arquivo e depois escolha **Recarregar**. O
formato, as regras de validação e as opções de recuperação estão na
[Visão geral da configuração](/pt-br/apps/apps-json/).

## Para onde ir em seguida

- [Campos do app](/pt-br/apps/fields/): cada chave e o que ela faz.
- [Tipos de app](/pt-br/apps/types/): como cada tipo é iniciado e mostra "em execução".
- [Parar e reiniciar](/pt-br/apps/stop-and-restart/): o que definir quando Parar deixa algo em execução e por que os apps Docker exigem cuidado.
- [Caminhos e ambiente](/pt-br/apps/paths-and-environment/): `{MP_HOME}`, caminhos `./` e `env`.
- [Exemplos](/pt-br/apps/examples/): entradas completas para copiar.
- [Guias práticos](/pt-br/guides/run-npm-dev-server-in-background-windows/): servidores de desenvolvimento em segundo plano, scripts Python, portas.
- [Modo portátil](/pt-br/data/portable-mode/)
- [Atualização](/pt-br/data/updating/)
