---
title: "Exemplos de apps.json para copiar e colar em configurações comuns"
description: "Entradas completas e válidas de apps.json para um servidor de desenvolvimento, um app desktop, uma página estática, uma ferramenta CLI, Docker Compose e um app portátil."
---

Cada trecho é uma entrada. Coloque-os dentro do array de nível superior do `apps.json`, separados por
vírgulas. Altere os ids, os nomes e os caminhos para corresponder à sua própria configuração.

## Servidor de desenvolvimento web

Em execução enquanto a porta 5173 responde. O navegador abre quando ela responde. Parar também libera a
porta, que é o padrão de `web`.

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173",
  "openBrowser": true
}
```

## App web que lê a porta do ambiente

```json title="apps.json"
{
  "id": "habit-tracker",
  "name": "Habit Tracker",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\habits",
  "command": "python app.py",
  "port": 8091,
  "url": "http://127.0.0.1:8091",
  "openBrowser": true,
  "env": { "PORT": "8091" },
  "note": "Moved off 8000 to avoid a clash"
}
```

## App desktop

Em execução enquanto existe um processo chamado `notes-app`. Parar encerra esse processo pelo nome.

```json title="apps.json"
{
  "id": "notes-app",
  "name": "Notes App",
  "group": "Desktop apps",
  "type": "desktop",
  "cwd": "C:\\code\\notes-app",
  "command": "npm run tauri dev",
  "processName": "notes-app"
}
```

## Página estática que já está hospedada

Sem terminal. Iniciar abre a página.

```json title="apps.json"
{
  "id": "team-board",
  "name": "Team board",
  "group": "Docs",
  "type": "static",
  "url": "https://example.com/board"
}
```

## Pasta estática servida por um comando

O Moonpool executa o servidor em um terminal, o acompanha pela porta e abre a página quando ele responde.

```json title="apps.json"
{
  "id": "docs-site",
  "name": "Docs",
  "group": "Docs",
  "type": "static",
  "cwd": "C:\\code\\docs\\public",
  "command": "python -m http.server 8090",
  "port": 8090,
  "url": "http://localhost:8090",
  "openBrowser": true,
  "killMode": "port"
}
```

## Ferramenta CLI

Roda em uma aba de terminal. O `-NoExit` mantém o shell aberto depois que o script termina.

```json title="apps.json"
{
  "id": "backup",
  "name": "Backup script",
  "group": "CLI tools",
  "type": "cli",
  "cwd": "C:\\code\\scripts",
  "command": "pwsh -NoLogo -NoProfile -NoExit -Command .\\backup.ps1 -Verbose"
}
```

## Docker Compose

O `command` já recria o contêiner e termina, então "em execução" vem da porta. Parar executa o
`stopCommand` em vez de encerrar o dono da porta, que no Windows seria o Docker Desktop. Veja
[Parar e reiniciar](/pt-br/apps/stop-and-restart/#apps-docker-no-windows).

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "docker compose up -d --build",
  "port": 8080,
  "url": "http://localhost:8080",
  "killMode": "command",
  "stopCommand": "docker compose stop app"
}
```

Se você preferir deixar o contêiner em execução ao parar o app, use
`"killMode": "none"` e remova o `stopCommand`.

## App portátil

Os caminhos se ancoram à pasta portátil, então a entrada continua funcionando depois que a pasta é movida.

```json title="apps.json"
{
  "id": "notes",
  "name": "Notes",
  "group": "Desktop apps",
  "type": "desktop",
  "cwd": "./apps/notes",
  "command": "{MP_HOME}\\apps\\notes\\notes.exe",
  "processName": "notes",
  "icon": "{MP_HOME}\\icons\\notes.png"
}
```

## Veja também

- [Painéis de exemplo](/pt-br/getting-started/example-dashboards/): os painéis que acompanham o Moonpool.
- [Campos do app](/pt-br/apps/fields/)
- [Executar um servidor de desenvolvimento npm em segundo plano no Windows](/pt-br/guides/run-npm-dev-server-in-background-windows/)
- [Manter um script Python rodando em segundo plano no Windows](/pt-br/guides/keep-python-script-running-background-windows/)
