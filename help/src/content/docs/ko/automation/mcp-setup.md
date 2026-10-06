---
title: "MCP로 AI 에이전트를 Moonpool에 연결하기"
description: "설치형 또는 포터블 moonpool.exe mcp를 stdio MCP 서버로 호스트에 등록하는 방법과, Moonpool이 앱의 자체 MCP 도우미를 추적하는 방식을 설명합니다."
---

Moonpool의 실행 파일이 곧 MCP 서버입니다. 인수 `mcp` 하나만 주고 `moonpool.exe`를 실행하는 stdio 서버로 호스트에 등록하세요.

## 서버 등록

설치형은 `%USERPROFILE%\.moonpool\moonpool.exe`입니다. 포터블은 `.moonpool\` 폴더 안의 `moonpool.exe`입니다. 그 전체 경로를 `command`로 사용하세요. `.mcp.json`을 읽는 호스트의 경우:

```json title=".mcp.json" {5}
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

JSON 파일에서는 위처럼 백슬래시를 두 번 써야 합니다. Claude Code처럼 명령줄로 등록할 수 있는 호스트는 한 단계로 추가할 수 있습니다.

```powershell frame="terminal"
claude mcp add moonpool -- "$env:USERPROFILE\.moonpool\moonpool.exe" mcp
```

서버는 자신을 `moonpool`로 알리고, MCP 프로토콜 리비전 `2025-06-18`을 사용하며, 도구만 제공합니다(리소스나 프롬프트는 제공하지 않습니다). 도구는 에이전트에 `moonpool_*`로 표시됩니다. [MCP 도구](/ko/automation/mcp-tools/)를 참고하세요.

## Moonpool이 둘 이상일 때

설치된 Moonpool과 각 포터블 복사본은 서로 별개의 런처이며 각자의 앱을 가지고 있고, 모두 동시에 실행할 수 있습니다. 복사본의 `moonpool.exe mcp`는 항상 그 복사본을 제어합니다. 에이전트가 여러 개를 사용하게 하려면, 각 복사본의 exe를 가리키는 서로 다른 이름으로 각각 등록하세요.

```json title=".mcp.json"
{
  "mcpServers": {
    "moonpool": {
      "type": "stdio",
      "command": "C:\\Users\\you\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    },
    "moonpool-work": {
      "type": "stdio",
      "command": "D:\\Work\\.moonpool\\moonpool.exe",
      "args": ["mcp"]
    }
  }
}
```

```powershell frame="terminal"
claude mcp add moonpool-work -- "D:\Work\.moonpool\moonpool.exe" mcp
```

두 복사본을 같은 이름으로 등록하면 대부분의 호스트에서 한쪽이 다른 쪽을 덮어씁니다. 도구 이름은 모든 복사본에서 같으므로, 호스트는 등록한 이름으로 복사본을 구분합니다. 포터블 복사본은 자신을 `moonpool (<folder>)`로 알리고 서버 지침에 폴더 이름을 넣으므로, 에이전트는 어느 복사본과 통신하는지 알 수 있습니다.

## 참고 사항

- `moonpool.exe mcp`는 창을 열지 않고 설치 프로그램도 시작하지 않습니다. 호스트가 입력을 닫으면 종료됩니다.
- 시작된 exe의 설정 폴더와 제어 채널을 사용하므로, 포터블 exe는 포터블 폴더의 데이터를 읽고 그 포터블 복사본을 제어합니다. exe 옆에 `moonpool.portable`이 있는 동안에만 포터블로 간주됩니다. 그 외의 `moonpool.exe`는 위치와 관계없이 설치된 Moonpool의 폴더(`%USERPROFILE%\.moonpool\moonpool-config\`)를 사용하고 설치된 Moonpool을 제어합니다.
- 대부분의 도구는 실행 중인 Moonpool이 필요합니다. 실행 중이 아니면 에이전트가 먼저 `moonpool_bootup_launcher`를 호출할 수 있습니다.
- `moonpool_launcher_paths`는 허브가 사용하는 폴더와 MCP 프로세스가 해석한 폴더를 나란히 보여 줍니다. 차이가 있으면 에이전트가 허브와 다른 `apps.json`을 보고 있다는 뜻입니다.

## 샌드박스 호스트

일부 호스트는 AppData를 패키지별 비공개 복사본으로 리디렉션하는 패키지(Store/MSIX) 샌드박스 안에서 도구를 실행합니다. Moonpool은 설정 폴더나 exe가 `...\Packages\<package>\LocalCache\...` 같은 경로 아래로 해석될 때 이를 감지합니다.

제어 채널은 응답하지만 `state.json`을 읽을 수 없을 때도 감지합니다. 이 경우 파일을 읽거나 쓰는 도구(`moonpool_app_output`, `moonpool_read_config`, `moonpool_write_config`, `moonpool_restore_config`)는 비어 있거나 오래된 데이터 대신 원인을 알려 주는 오류를 반환합니다. `moonpool_list_apps`처럼 제어 채널만 쓰는 도구는 채널에 연결할 수 있는 동안 차단되지 않습니다. 샌드박스가 채널까지 숨기면, 도구는 "Moonpool is not running" 대신 샌드박스 때문임을 알려 줍니다. 대신 샌드박스 밖의 셸에서 [명령줄](/ko/automation/command-line/)을 사용하세요.

## 자체 MCP 서버가 있는 앱

Moonpool의 많은 앱은 MCP 호스트가 `<exe> mcp` 도우미 프로세스를 통해 접근합니다. Moonpool은 이름이 앱의 `processName`과 일치하고 첫 번째 인수가 `mcp`인 프로세스(예: `notes-app.exe mcp`)를 찾습니다. 서버가 이름을 바꾼 복사본처럼 다른 이름으로 실행된다면, 앱의 `mcpProcessName` 와일드카드를 설정하세요([필드](/ko/apps/fields/#mcpprocessname) 참고). 그 패턴에 일치하는 프로세스는 `mcp` 인수 없이도 인정됩니다.

- 도우미가 연결되어 있는 동안 앱의 사이드바에 MCP 하위 행이 실행 중으로 표시되고, `moonpool_list_apps`는 앱의 줄 끝에 `[mcp: running]`을 덧붙입니다. 도우미는 앱 자체가 실행 중인 것으로 취급되지 않습니다.
- 한 번 감지된 도우미는 Moonpool이 기억하므로(설정 폴더의 `mcp_seen.json`), 도우미가 종료된 뒤에도 MCP 하위 행은 중지됨으로 계속 표시되고 `moonpool_list_apps`는 `[mcp: stopped]`를 표시합니다.
- MCP 하위 행은 `showMcpProcesses` 설정으로 제어합니다([설정 창](/ko/using/settings/)).
- `moonpool_stop_mcp_server`는 도우미를 종료하고 앱은 그대로 둡니다. 시작에 해당하는 도구는 없습니다. 도우미를 소유한 호스트가 다음 도구 호출 때 다시 시작합니다.

## 도구가 동작하지 않을 때

- **호스트에 `moonpool_*` 도구가 보이지 않습니다.** `command`가 `moonpool.exe`의 전체 경로이고 `args`가 `["mcp"]`인지 확인한 뒤 호스트를 다시 시작하세요.
- **모든 도구가 Moonpool이 실행 중이 아니라고 합니다.** Moonpool을 시작하거나 `moonpool_bootup_launcher`를 호출하세요. 등록한 exe가 실행 중인 복사본과 같은지 확인하세요.
- **수정 내용이 반영되지 않습니다.** `moonpool_launcher_paths`를 호출해 허브의 폴더와 MCP 프로세스의 폴더를 비교하세요. [샌드박스 호스트](#샌드박스-호스트)를 참고하세요.

더 많은 내용은 [문제 해결](/ko/support/troubleshooting/#mcp와-스크립트-오류)에 있습니다.

## 관련 항목

- [AI 에이전트(Claude Code, Codex, Cursor)에게 로컬 앱을 시작하고 중지하는 MCP 서버 제공하기](/ko/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/)
- [MCP 도구](/ko/automation/mcp-tools/)
- [AI 에이전트: 빠른 시작](/ko/automation/quick-start/)
