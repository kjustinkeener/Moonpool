---
title: "AI 에이전트(Claude Code, Codex, Cursor)에게 로컬 앱을 시작하고 중지하는 MCP 서버 제공하기"
description: "Moonpool을 MCP 서버로 등록해 Claude Code, Codex, Cursor가 중복 실행 없이 개발 서버를 시작, 중지, 재시작하고 출력을 읽을 수 있게 합니다."
---

AI 코딩 에이전트는 보통 자신의 셸에 `npm run dev`를 입력해 개발 서버를 실행합니다. 그러면 에이전트가
멈추거나, 포트를 쥔 고아 프로세스가 남거나, 이미 실행 중인 앱의 복사본이 하나 더 시작될 수
있습니다. MCP 서버를 사용하면 에이전트가 명령줄을 재구성하는 대신 이미 설정해 둔 앱을 시작하고
중지하는 도구를 호출할 수 있습니다.

## Moonpool 방식

Moonpool의 실행 파일은 그 자체가 MCP 서버입니다. `moonpool.exe`를 인수 `mcp` 하나와 함께 stdio
서버로 등록하세요. 앱이 `apps.json`에 있으면 에이전트가 id로 앱을 시작합니다.

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

서버를 등록합니다. Claude Code에서는 명령 하나면 됩니다(설치된 Moonpool 기준이며, 자신의 exe 전체
경로를 사용하세요).

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

Cursor의 `mcp.json`처럼 MCP 서버를 JSON 파일에서 읽는 호스트도 같은 형태를 사용합니다(백슬래시는
두 번 씁니다).

```json title="mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

Codex에서는 설정 파일(`~/.codex/config.toml`)에 같은 명령과 `mcp` 인수로 서버를 추가합니다.

```toml title="config.toml"
[mcp_servers.moonpool]
command = 'C:\Users\you\.moonpool\moonpool.exe'
args = ["mcp"]
```

정확한 파일 이름과 키 이름은 호스트마다 다르므로, 버전이 다르다면 해당 호스트의 MCP 문서를
확인하세요. Moonpool에 필요한 것은 `moonpool.exe`의 전체 경로와 인수 `mcp`뿐입니다. 등록한 뒤에는
호스트를 다시 시작하세요.

## 에이전트가 할 수 있는 일

도구는 `moonpool_*` 이름으로 나타납니다. 일상적인 작업에 쓰는 도구는 다음과 같습니다.

| 도구 | 용도 |
| --- | --- |
| `moonpool_list_apps` | 앱의 id를 찾고 실행 중인지 확인합니다. |
| `moonpool_start_app` | id로 앱을 시작하고 해당 터미널 탭을 엽니다. |
| `moonpool_stop_app` | 자식 프로세스를 포함해 앱을 중지합니다. |
| `moonpool_restart_app` | 중지하고, 포트가 해제되기를 기다린 뒤, 시작합니다. 코드를 변경한 뒤에 사용합니다. |
| `moonpool_app_output` | 앱이 출력한 내용을 읽으며, `tail_lines`로 분량을 제한합니다. |
| `moonpool_bootup_launcher` | Moonpool이 실행 중이 아니면 Moonpool 자체를 시작합니다. |

일반적인 흐름은 `moonpool_restart_app` 다음에 `moonpool_app_output`을 호출하는 것입니다. 나머지 도구
(`apps.json` 읽기와 쓰기, 스크린샷)는 [MCP 도구](/ko/automation/mcp-tools/)에 있습니다.

## 동작하지 않을 때

모든 도구가 `Moonpool is not running - call moonpool_bootup_launcher first`라고 응답한다면
Moonpool이 아직 시작되지 않은 것입니다. 편집한 내용이 반영되지 않는다면 대개 에이전트가 다른
`apps.json`을 보고 있는 경우이니 `moonpool_launcher_paths`를 호출해 보세요.
[도구가 동작하지 않을 때](/ko/automation/mcp-setup/#도구가-동작하지-않을-때)를 참고하세요.

## 함께 보기

- [MCP 설정](/ko/automation/mcp-setup/)
- [MCP 도구](/ko/automation/mcp-tools/)
- [AI 에이전트: 빠른 시작](/ko/automation/quick-start/)
- [Windows에서 npm 개발 서버를 백그라운드에서 실행](/ko/guides/run-npm-dev-server-in-background-windows/)
