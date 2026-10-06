---
title: "명령줄에서 Moonpool 제어하기"
description: "터미널이나 스크립트에서 moonpool.exe 동사로 실행 중인 Moonpool을 제어하고, 티켓을 붙여 state.json에서 결과를 확인하는 방법을 설명합니다."
---

같은 Moonpool이 이미 실행 중일 때 `moonpool.exe`를 다시 실행해도 두 번째 창은 열리지 않습니다. 두 번째 프로세스는 인수를 [제어 채널](/ko/automation/control-verbs/)로 실행 중인 프로세스에 전달하고 종료합니다. Moonpool이 이미 실행 중이어야 합니다. 상주하는 것이 없으면 같은 명령이 새 Moonpool을 시작하고 동사는 실행되지 않습니다.

"같은 Moonpool"은 같은 폴더를 뜻합니다. 설치된 Moonpool과 모든 포터블 복사본은 각각 독립적으로 실행되므로, 명령은 실행한 `moonpool.exe`에 해당하는 복사본에만 전달되고 다른 복사본에는 전달되지 않습니다. [포터블 모드](/ko/data/portable-mode/#여러-복사본을-동시에)를 참고하세요.

의도한 복사본의 경로를 사용하세요. 설치된 복사본이라면 다음과 같습니다.

```powershell frame="terminal"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
```

복사본이 여러 개 실행 중이면 `Get-Process moonpool`이 모두 나열하므로, 첫 번째를 고르지 말고 `Path`로 고르세요. 이 명령은 MCP 호스트가 시작한 유휴 상태의 `moonpool.exe mcp` 도우미도 나열하므로, `moonpool` 프로세스가 있다고 해서 허브가 실행 중이라는 증거는 아닙니다. 대신 제어 채널에 `ping`으로 물어보세요([제어 동사](/ko/automation/control-verbs/)).

## 동사

동사는 대소문자를 구분하지 않습니다. `<id>`는 `apps.json`에 있는 앱의 `id`입니다.

| 명령 | 효과 |
| --- | --- |
| `moonpool.exe` | 동사 없음: 창을 맨 앞으로 가져옵니다. |
| `moonpool.exe show` | 창을 맨 앞으로 가져옵니다. |
| `moonpool.exe launch <id>` | 앱을 시작하고 터미널 탭을 엽니다. |
| `moonpool.exe stop <id>` | 앱을 중지합니다. |
| `moonpool.exe restart <id>` | 중지하고, 포트와 프로세스가 해제될 때까지 기다린 뒤 시작합니다. |
| `moonpool.exe reload` | `apps.json`을 다시 읽습니다. |
| `moonpool.exe refresh-icons` | 모든 아이콘을 다시 가져옵니다. |
| `moonpool.exe help` | 도움말 창을 엽니다. |
| `moonpool.exe quit` | 트레이 메뉴와 같이 Moonpool을 종료합니다. |
| `moonpool.exe dump <id> [out-path]` | `out-path`가 없으면 이번 세션에서 해당 앱의 로그 경로를 알려 줍니다. 있으면 ANSI 코드를 제거한 일반 텍스트로 그 위치에 로그를 복사합니다. |
| `moonpool.exe paths` | 실행 중인 Moonpool이 사용하는 설정 폴더, `apps.json`, `state.json`, 로그, dumps 폴더, 아이콘 폴더, 포터블 플래그, exe 경로를 알려 줍니다. |
| `moonpool.exe read-config` | 설정 폴더에 `dumps\read-config.json`을 작성하며, `token`, `valid`, `error`, `path`, `manifest_text`(`apps.json`의 정확한 내용)를 담습니다. |
| `moonpool.exe write-config <file> [token]` | 매니페스트가 유효하고, `token`을 준 경우 `apps.json`이 그 토큰과 여전히 일치하면, `apps.json`을 `<file>`의 매니페스트로 교체합니다. |
| `moonpool.exe restore-config [index or filename]` | 인수가 없으면 스냅샷 목록을 `dumps\restore-config.json`에 작성합니다. 인수가 있으면 해당 스냅샷이 유효할 때 복원합니다. |

```powershell frame="terminal"
& $mp restart my-app
```

```powershell frame="terminal"
& $mp dump my-app C:\temp\my-app.log
```

알 수 없는 동사는 무시됩니다. 프로그램에는 자체 시작 인수도 있습니다. `moonpool.exe mcp`([MCP 설정](/ko/automation/mcp-setup/)), `--uninstall`(프로그램 추가/제거에서 사용), `--wait-pid <pid>`(Moonpool이 스스로 다시 실행할 때 사용)입니다. 이 인수들은 첫 번째 인수일 때만 적용되므로, `--uninstall` 같은 앱 id로는 동작하지 않습니다.

## 결과 확인하기

명령줄은 아무것도 출력하지 않으므로, 명령에 `--ticket <key>`(고유한 아무 키, 위치 무관)를 붙이고 설정 폴더의 `state.json`에서 결과를 읽으세요. 설치형은 `%USERPROFILE%\.moonpool\moonpool-config\`, 포터블 복사본은 `<your .moonpool folder>\moonpool-config\`, Linux는 `~/.config/Moonpool/`입니다([구성 개요](/ko/apps/apps-json/#설정-위치) 참고). `show`와 `quit`는 티켓을 남기지 않습니다.

```powershell frame="terminal"
& $mp launch my-app --ticket t1
```

`state.json`에는 `apps`, `statuses`(앱마다 `id`, `running`, `managed`, `mcpRunning`, `mcpSeen`), `tickets`가 있습니다. 실행 중인 Moonpool은 몇 초마다, 그리고 명령을 처리할 때마다 이 파일을 다시 쓰며, 종료할 때 삭제하지 않으므로 파일이 남아 있어도 Moonpool이 실행 중이라는 뜻은 아닙니다. 실행 중인지 확인하거나 실시간 앱 목록을 얻으려면 제어 채널의 `ping`과 `list` 동사([제어 동사](/ko/automation/control-verbs/))나 MCP 도구를 사용하세요. `status`가 `pending`이 아닐 때까지 티켓을 폴링하세요.

| `status` | 의미 |
| --- | --- |
| `pending` | 접수됨. Moonpool이 아직 처리 중입니다. |
| `ok` | 완료. `dump`, `read-config`, `write-config`, `restore-config`, `paths`에서는 `detail`에 경로, 토큰 또는 보고 내용이 들어 있습니다. |
| `error` | 실패. `detail`에 이유가 있습니다(예: `unknown app id: x`, `did not reach running in time`, `unknown command`). |

각 티켓은 `{ ticket, action, arg, status, detail, ts }`이며 `ts`는 Unix 밀리초입니다.

```json title="state.json (tickets entry)"
{
  "ticket": "t1",
  "action": "launch",
  "arg": "my-app",
  "status": "error",
  "detail": "did not reach running in time",
  "ts": 1767225600000
}
```

완료된 티켓은 24시간 후 삭제되며, 완료된 티켓이 5분 이상 지나면 목록이 50개 항목 근처로 정리됩니다.

MCP를 지원하는 에이전트는 폴링을 건너뛸 수 있습니다. [MCP 설정](/ko/automation/mcp-setup/)을 참고하세요.

## 관련 항목

- [AI 에이전트: 빠른 시작](/ko/automation/quick-start/)
- [제어 동사](/ko/automation/control-verbs/)
