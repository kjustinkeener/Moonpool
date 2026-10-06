---
title: "Manter um script Python rodando em segundo plano no Windows"
description: "Execute em segundo plano no Windows um script Python de longa duração ou um pequeno app web, veja a saída dele e pare-o de forma limpa, com pythonw e com o Moonpool."
---

Um script Python executado em uma janela de console para quando você fecha essa janela. As soluções
usuais no Windows são o `pythonw.exe` (o mesmo interpretador sem janela de console, então a saída não
vai para lugar nenhum), `Start-Process pythonw -ArgumentList worker.py` para iniciá-lo desacoplado, ou
uma tarefa agendada para algo que deva rodar no login ou em um intervalo. Em todos os casos, você
precisa achar o processo no Gerenciador de Tarefas quando quiser encerrá-lo.

## O jeito do Moonpool

O Moonpool executa o comando em sua própria aba de terminal, então você mantém a saída e um botão Parar
sem ter uma janela de console própria. Para um script que roda até você pará-lo, use um app `cli`. O
`-u` faz o Python descarregar a saída imediatamente, para a aba mostrá-la ao vivo:

```json title="apps.json"
{
  "id": "worker",
  "name": "Queue worker",
  "group": "Scripts",
  "type": "cli",
  "cwd": "C:\\code\\worker",
  "command": ".venv\\Scripts\\python.exe -u worker.py"
}
```

Inicie-o e clique no nome do app para ver a saída. Um app `cli` conta como em execução enquanto o
comando roda e fica cinza quando o script termina, com `[process exited]` (processo encerrado) na aba.
**Parar** encerra o script e tudo o que ele iniciou. Usar o `python.exe` do ambiente virtual pelo caminho
significa que não é preciso nenhum passo de ativação.

Se o script serve HTTP (Flask, FastAPI, `python -m http.server`), torne-o um app `web` para que "em
execução" acompanhe a porta dele:

```json title="apps.json"
{
  "id": "docs-api",
  "name": "Docs API",
  "group": "Scripts",
  "type": "web",
  "cwd": "C:\\code\\docs-api",
  "command": ".venv\\Scripts\\python.exe -u app.py",
  "port": 8091,
  "url": "http://127.0.0.1:8091",
  "env": { "PORT": "8091" }
}
```

## Limites

- Mantenha o Moonpool em execução. Fechar a janela dele o encerra por padrão, e no Windows isso
  também para todos os apps que ele iniciou. Ative **Fechar para a bandeja** para só ocultar a janela; veja
  [Bandeja, fechar e minimizar](/pt-br/using/tray-and-closing/).
- O Moonpool não reinicia um script que falha, nem o inicia sozinho no login do Windows. Veja
  [Iniciar um script ou servidor de desenvolvimento automaticamente no login do Windows](/pt-br/guides/start-app-at-windows-login/).
- Evite aspas duplas aninhadas em `command`: o wrapper `cmd /c` as estraga.

## Veja também

- [Tipos de app](/pt-br/apps/types/#cli): como os apps `cli` e `web` são acompanhados.
- [Parar e reiniciar](/pt-br/apps/stop-and-restart/)
- [Exemplos](/pt-br/apps/examples/)
- [Logs](/pt-br/data/logs/): onde a saída das sessões é mantida.
