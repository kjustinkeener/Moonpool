// Português (Brasil). Keys absent here fall back to English at runtime.
import type { PartialDict } from "../i18n.svelte";

export const ptBR: PartialDict = {
  "common.close": "Fechar",
  "common.cancel": "Cancelar",
  "common.save": "Salvar",
  "common.delete": "Excluir",
  "common.edit": "Editar",
  "common.rename": "Renomear",
  "common.reload": "Recarregar",
  "common.settings": "Configurações",
  "common.about": "Sobre",
  "common.tryAgain": "Tentar de novo",
  "common.dismiss": "Dispensar",
  "common.copied": "Copiado!",
  "common.autoSystem": "Automático (sistema)",

  "titlebar.minimize": "Minimizar",
  "titlebar.maximize": "Maximizar",
  "titlebar.restore": "Restaurar",

  "sidebar.menu": "Menu",
  "sidebar.addApp": "Adicionar app",
  "sidebar.editJson": "Editar apps.json",
  "sidebar.installMoonpool": "Instalar o Moonpool…",
  "sidebar.filterPlaceholder": "Filtrar apps...",
  "sidebar.filterLabel": "Filtrar apps",
  "sidebar.showCli": "Mostrar o painel CLI",
  "sidebar.updateOpenToInstall": "Atualização disponível, abra para instalar",
  "sidebar.updateShowCli": "Atualização disponível, mostrar o painel CLI",
  "sidebar.statusRunning": "em execução",
  "sidebar.statusStarting": "iniciando...",
  "sidebar.statusStopped": "parado",
  "sidebar.working": "Trabalhando...",
  "sidebar.restart": "Reiniciar",
  "sidebar.stop": "Parar",
  "sidebar.launch": "Iniciar",
  "sidebar.setIcon": "Definir ícone...",
  "sidebar.portConflict": "{names} usam a mesma porta {port}",
  "sidebar.portConflictBadge": "porta {port}: {names}",
  "sidebar.noMatch": "Nenhum app corresponde a “{filter}”.",

  "app.dragToResize": "Arraste para redimensionar",
  "app.closeTab": "Fechar aba",
  "app.hideCli": "Ocultar o painel CLI",
  "app.pickApp": "Escolha um app à esquerda para iniciá-lo.",
  "app.updateAvailable": "O Moonpool {version} está disponível (você tem a {current}).",
  "app.downloading": "Baixando {version}…",
  "app.installing": "Instalando…",
  "app.downloadInstall": "Baixar e instalar",
  "app.updateInstalled": "Atualização instalada. Reiniciando…",
  "app.updateFailed": "Falha na atualização: {error}",
  "app.confirmDelete": "Excluir “{name}”?",
  "app.iconDialogTitle": "Ícone de {name}",
  "app.imagesFilter": "Imagens",
  "app.newHere":
    "É novo por aqui? Passe isto para um agente de IA configurar seus apps:",
  "app.copyPrompt": "Copiar o prompt",
  "app.editFile": "Editar o arquivo",
  "app.orUseHint": "Ou use {add} acima, ou {edit} para editar o {file} diretamente.",

  "about.tagline":
    "Um hub de inicialização na bandeja do sistema para apps locais e servidores de desenvolvimento, com um terminal embutido por app.",
  "about.version": "versão {version}",
  "about.checkUpdates": "Procurar atualizações",
  "about.checking": "Procurando atualizações...",
  "about.upToDate": "Você está na versão mais recente.",
  "about.updateDownloading": "Atualização {version} disponível: baixando...",
  "about.updateInstalled": "Atualização instalada. Reiniciando...",
  "about.checkFailed": "Falha ao procurar atualizações: {error}",
  "about.builtWith": "Feito com",
  "about.byLine": "Licença MIT · por {author}",

  "settings.title": "Configurações",
  "settings.theme": "Tema",
  "settings.language": "Idioma",
  "settings.languageHint": "O idioma em que o texto do próprio Moonpool aparece.",
  "settings.closeToTray": "Fechar para a bandeja",
  "settings.closeToTrayHint":
    "Fechar a janela oculta o Moonpool na bandeja (sai da barra de tarefas). Desligado: fechar encerra o app.",
  "settings.minimizeToTray": "Minimizar para a bandeja",
  "settings.minimizeToTrayHint":
    "Minimizar oculta o Moonpool na bandeja (sai da barra de tarefas). Desligado: minimiza para a barra de tarefas.",
  "settings.alwaysOnTop": "Sempre visível",
  "settings.alwaysOnTopHint":
    "Mantém o Moonpool e suas janelas de Configurações e Sobre acima das outras janelas.",
  "settings.transparency": "Transparência do fundo",
  "settings.transparencyHint": "Fundo de janela translúcido. 0% é opaco.",
  "settings.checkOnStartup": "Procurar atualizações ao iniciar",
  "settings.checkOnStartupHint":
    "Ao abrir, consulta o GitHub discretamente e mostra um aviso se houver uma versão mais nova.",
  "settings.debugLogging": "Gravar informações de depuração em um arquivo",
  "settings.debugLoggingHint":
    "Registra os carregamentos do manifesto, as inicializações e os erros em {file}.",
  "settings.openLog": "Abrir o log",

  "theme.dark": "Escuro",
  "theme.light": "Claro",

  "installer.tagline":
    "Um inicializador na bandeja para seus apps locais e servidores de desenvolvimento.",
  "installer.pickFolder": "Escolha uma pasta para o Moonpool portátil",
  "installer.poke": "clique esquerdo para cutucar, clique direito para redefinir",
  "installer.installed": "Instalado, iniciando o Moonpool…",
  "installer.portableDone": "Modo portátil, iniciando o Moonpool…",
  "installer.failed": "Falha na instalação: {error}",
  "installer.alreadyInstalledTitle": "Esta cópia já está instalada neste PC.",
  "installer.alreadyInstalled": "Já instalado",
  "installer.installing": "Instalando…",
  "installer.install": "Instalar o Moonpool",
  "installer.desktopShortcut": "Adicionar um atalho na área de trabalho",
  "installer.portableHint":
    "Rode o Moonpool a partir de uma pasta à sua escolha (pen drive, zip) e leve para onde quiser. Os dados ficam ao lado do exe.",
  "installer.installPortable": "Instalar portátil",
  "installer.installPath": "Caminho de instalação: {dir}",

  "editor.addApp": "Adicionar app",
  "editor.editApp": "Editar app",
  "editor.backToList": "voltar para a lista",
  "editor.nameHint": "O nome exibido na barra lateral. Obrigatório.",
  "editor.namePlaceholder": "Meu app",
  "editor.groupHint":
    "O título da barra lateral sob o qual este app aparece. Escolha um grupo existente ou “+ Novo grupo” para criar um.",
  "editor.newGroupPlaceholder": "Nome do novo grupo",
  "editor.pickExistingGroup": "Escolher um grupo existente",
  "editor.newGroupOption": "+ Novo grupo...",
  "editor.typeHint":
    "Como o Moonpool executa e acompanha o app. web = servidor de desenvolvimento em uma porta. desktop = app nativo identificado pelo nome do processo. static = apenas abre uma URL. cli = executa um comando em um terminal.",
  "editor.hintWeb":
    "Executa um servidor de desenvolvimento em um terminal; marca “em execução” quando a porta responde e abre o navegador assim que estiver no ar.",
  "editor.hintDesktop":
    "Inicia um app nativo; marca “em execução” quando encontra um processo chamado processName.",
  "editor.hintStatic": "Apenas abre url no navegador, sem terminal nem comando.",
  "editor.hintCli": "Executa um comando e mantém um shell interativo aberto em cwd.",
  "editor.portableWarn":
    "Caminho absoluto: não vai junto com esta pasta. Use {MP_HOME}\\... ou um caminho ./ para continuar portátil.",
  "editor.notPortable": "não portátil",
  "editor.cwdHint":
    "O diretório de trabalho em que o comando roda, normalmente a pasta do projeto. Use um caminho absoluto, ou {MP_HOME}\\... / ./ para continuar portátil.",
  "editor.commandHint":
    "O comando executado no terminal embutido para iniciar o app, por exemplo 'npm run dev' ou 'python app.py'. Deixe em branco para uma entrada só de URL.",
  "editor.portHint":
    "A porta TCP local em que o app escuta. O Moonpool marca “em execução” quando essa porta responde, e a libera ao parar. Usada pelos apps web.",
  "editor.processNameHint":
    "Para apps de desktop: o nome do processo ou executável (sem .exe) usado para detectar que está rodando e para pará-lo. No Linux precisa ter no máximo 15 caracteres.",
  "editor.urlHint":
    "A URL a abrir: http://localhost:<port> para um app web, ou file:///path/to/index.html para uma página estática. Use file:///{MP_HOME}/... para continuar portátil.",
  "editor.openBrowser": "abrir o navegador",
  "editor.openBrowserHint":
    "Abre a URL automaticamente no seu navegador padrão assim que o app fica acessível.",
  "editor.envLabel": "env (KEY=VALUE por linha)",
  "editor.envHint":
    "Variáveis de ambiente passadas ao comando, uma KEY=VALUE por linha (por exemplo PORT=3000).",
  "editor.noteHint":
    "Texto opcional exibido como dica ao passar o mouse sobre este app na barra lateral.",
  "editor.notePlaceholder": "dica opcional",
  "editor.phFolder": "caminho para a pasta do app",
  "editor.phLaunchCommand": "o comando de inicialização do app",
  "editor.phCommand": "comando a executar",
  "editor.discardChanges": "Descartar suas alterações?",
  "editor.nameRequired": "o nome é obrigatório.",

  "term.processExited": "[processo encerrado]",
};
