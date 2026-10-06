---
title: "Instale e use o Moonpool no Linux"
description: "Instale o Moonpool no Linux, contorne a limitação da bandeja no GNOME, entenda como funcionam as atualizações e veja quais recursos diferem da versão para Windows."
---

O Moonpool roda no Linux por meio do WebKitGTK. Ele é desenvolvido principalmente no Windows, então o
Linux é suportado, mas menos testado. No Linux não há cartão de instalação nem seletor de modo portátil, e
o menu "..." não tem o item **Instalar o Moonpool…**.

## Instalar

Baixe um pacote na página de Releases do projeto.

| Pacote | Atualizações |
| --- | --- |
| AppImage | O Moonpool se atualiza sozinho |
| `.deb` | Seu gerenciador de pacotes |
| RPM (instale com a ferramenta RPM da sua distribuição) | Seu gerenciador de pacotes |

```bash title="AppImage" frame="terminal"
chmod +x Moonpool_*.AppImage
./Moonpool_*.AppImage
```

```bash title=".deb" frame="terminal"
sudo apt install ./Moonpool_*_amd64.deb
```

```bash title="RPM" frame="terminal"
sudo dnf install ./Moonpool-*.x86_64.rpm
```

O `.deb` instala as dependências de execução. O AppImage precisa que as bibliotecas WebKitGTK e
AppIndicator estejam presentes, por exemplo no Debian ou Ubuntu:

```bash frame="terminal"
sudo apt-get install -y libwebkit2gtk-4.1-0 libayatana-appindicator3-1
```

No Fedora ou Arch, use os equivalentes:

```bash title="Fedora" frame="terminal"
sudo dnf install webkit2gtk4.1 libayatana-appindicator-gtk3
```

```bash title="Arch" frame="terminal"
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator
```

## Bandeja no GNOME

O GNOME padrão não mostra ícones de bandeja, então o ícone da bandeja do Moonpool não aparece até a
extensão AppIndicator ser instalada e ativada:

```bash frame="terminal"
sudo apt-get install -y gnome-shell-extension-appindicator
gnome-extensions enable ubuntu-appindicators@ubuntu.com
```

Depois saia da sessão e entre de novo. A janela do hub e os terminais embutidos funcionam sem ela. KDE,
Cinnamon, XFCE e MATE mostram a bandeja de fábrica.

## Atualizações

Só o AppImage se atualiza sozinho. Ele lê o `linux-update.json` do GitHub Releases, verifica a assinatura
minisign e substitui o arquivo AppImage no lugar, então mantenha-o em uma pasta em que você possa
gravar. As instalações `.deb` e RPM nunca são sobrescritas pelo Moonpool: a verificação de atualizações
ainda pode informar uma versão mais nova, mas instalá-la pelo Moonpool falha com uma mensagem para usar o
gerenciador de pacotes. Veja [Atualização](/pt-br/data/updating/).

## Local da configuração

```text
~/.config/Moonpool/              (or $XDG_CONFIG_HOME/Moonpool/)
~/.config/Moonpool/apps.json
~/.config/Moonpool/dashboards/examples/   (example dashboards)
```

O `apps.json` é gerado a partir do exemplo na primeira execução. Veja
[Visão geral da configuração](/pt-br/apps/apps-json/).

## Diferenças em relação ao Windows

- Os comandos de inicialização rodam por `$SHELL -c <command>` (`/bin/sh` se `SHELL` não estiver definido), então use uma sintaxe que o seu shell entenda.
- Parar encerra o grupo de processos e depois faz a limpeza extra escolhida por `killMode`. Liberar uma porta com `killMode: "port"` usa `lsof`, com `fuser` como alternativa; instale o `lsof` se a sua distribuição não o incluir. Veja [Parar e reiniciar](/pt-br/apps/stop-and-restart/).
- O `processName` de um app `desktop` deve ter 15 caracteres ou menos. O Linux trunca o nome de um processo em 15 caracteres, então um nome mais longo nunca é detectado como em execução e não pode ser parado pelo nome. Os apps `web` correspondem pela porta e não são afetados.
- Os ícones são buscados em `src-tauri/icons/`, `public/favicon.*`, `icon.png` do app ou no favicon ao vivo. Extrair um ícone de um binário só é possível no Windows.
- Os botões de abrir abrem a pasta que contém o arquivo em vez de selecioná-lo.
- Os arquivos de configuração abrem no seu editor de texto padrão (resolvido pela associação `text/plain`).
- O instalador do Windows, os atalhos e a entrada de Adicionar ou remover programas não se aplicam.

## Veja também

- [Windows](/pt-br/platforms/windows/#o-que-difere-por-plataforma): uma tabela do que difere por plataforma.
- [Atualização](/pt-br/data/updating/#linux)
