---
title: "Adicione e execute seu primeiro app no Moonpool"
description: "Vá da primeira inicialização a um app seu em execução em poucos minutos: adicione, inicie, pare e encontre o hub e esta ajuda de novo mais tarde."
---

## 1. Inicie o Moonpool

No Windows, execute `moonpool.exe` e clique em **Instalar o Moonpool** (veja
[Instalação](/pt-br/getting-started/install/)). No Linux, inicie o AppImage ou o pacote instalado.

Na primeira vez que roda, o Moonpool preenche a barra lateral com apps de exemplo (o Bloco de Notas no
Windows, um shell, um pequeno servidor web e os painéis incluídos). Eles rodam do jeito que estão (o
servidor web precisa do Python), então você pode testá-los e depois editá-los ou excluí-los. Ele também
coloca um ícone na bandeja do sistema. No Windows, se você não vir o ícone, clique na seta **^** à
direita da barra de tarefas.

## 2. Adicione seu app

1. Abra o menu **...** no topo da barra lateral e escolha **Adicionar app**.
2. Digite um **name**. O grupo começa como `Web apps`; mantenha-o ou escolha outro.
3. Mantenha **type** como `web` para um servidor de desenvolvimento.
4. Defina **cwd** como a pasta do seu projeto e **command** como o que você digita para iniciá-lo, por
   exemplo `npm run dev`.
5. Defina **port** como a porta em que ele escuta e **url** como a página a abrir.
6. Salve.

Os detalhes de cada campo estão em [Adicionar apps](/pt-br/apps/add-an-app/).

## 3. Inicie

Clique no botão **Iniciar** do app (o ícone de reproduzir na linha dele). A aba de terminal dele abre e
mostra a saída. O ponto de status pulsa enquanto o app está iniciando e fica fixo quando a porta
responde. Se **openBrowser** estiver ativado, a página abre.

Clicar no nome do app só abre a aba de terminal dele. Nunca inicia o app.

## 4. Pare

Clique no botão **Parar** (o quadrado) na linha. O ponto fica cinza.

Se algo continuar em execução depois de Parar, veja [Parar e reiniciar](/pt-br/apps/stop-and-restart/).

## Deixe um agente fazer isso

Sem nenhuma aba aberta, o painel CLI mostra um botão **Copiar o prompt**. Cole o prompt em um agente de
IA e ele encontra seus apps e os adiciona. Veja
[Agentes de IA: início rápido](/pt-br/automation/quick-start/).

## Como encontrar o hub depois

- Clique com o botão esquerdo no ícone da bandeja para mostrar o hub. Clique com o botão direito para
  abrir um menu com **Mostrar o Moonpool** e **Sair**.
- Por padrão, fechar a janela encerra o Moonpool. Ative **Fechar para a bandeja** em Configurações para
  ocultá-lo na bandeja e mantê-lo em execução. Veja
  [Bandeja, fechar e minimizar](/pt-br/using/tray-and-closing/).

## Como obter ajuda

**Ajuda**, no menu **...** no topo da barra lateral, abre esta ajuda em uma janela própria. Ela funciona
offline e sempre corresponde à versão que você usa.

![Janela de ajuda com a navegação por seções destacada à esquerda e uma página à direita](../../../../assets/screenshots/help-window.png)

## Próximo

- [Adicionar apps](/pt-br/apps/add-an-app/)
- [Solução de problemas](/pt-br/support/troubleshooting/)
