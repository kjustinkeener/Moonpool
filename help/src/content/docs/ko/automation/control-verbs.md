---
title: "Moonpool 제어 채널과 동사 레퍼런스"
description: "Moonpool 제어 채널(이름 있는 파이프 또는 Unix 소켓)의 동작 방식과 프로토콜, 실행 중인 앱이 응답하는 모든 동사의 인수와 응답을 정리했습니다."
---

## 수신 위치

Moonpool 복사본마다 자체 채널이 있으므로, 설치된 Moonpool과 포터블 복사본은 서로 대신 응답하지 않고 나란히 실행할 수 있습니다. Windows에서 설치된 Moonpool은 이름 있는 파이프 `\\.\pipe\moonpool`에서 수신합니다. 포터블 복사본은 폴더에서 만든 id를 붙여 `\\.\pipe\moonpool-<id>`를 사용합니다.

`<id>`는 복사본의 `moonpool-config` 폴더 경로에서 만든 16진수 8자리로, 재시작이나 업데이트를 거쳐도 같은 폴더에서는 같은 값이 유지되고 폴더를 옮기면 바뀝니다. 복사본의 `moonpool.exe`(`moonpool.exe mcp` 포함)는 항상 자신의 복사본 채널을 찾습니다.

Linux와 macOS에서는 대신 `0600` 모드의 Unix 도메인 소켓에서 수신합니다.

| 경우 | 소켓 경로 |
| --- | --- |
| 일반 | `$XDG_RUNTIME_DIR`이 설정되어 있으면 `$XDG_RUNTIME_DIR/moonpool.sock`, 아니면 Moonpool 설정 폴더의 `moonpool.sock` |
| 포터블 모드 | 포터블 복사본 설정 폴더의 `moonpool.sock`이므로, 포터블 복사본이 설치된 복사본과 충돌하지 않습니다 |
| 소켓 경로가 너무 긴 경우(약 100자) | 본인만 열 수 있는 디렉터리의 `/tmp/moonpool-<uid>/moonpool.sock`(포터블 복사본은 `moonpool-<id>.sock`) |

충돌로 남은 소켓 파일은 다음 시작 때 감지되어 교체됩니다. 무언가 아직 응답하는 소켓은 가로채지 않습니다. 정상 종료 시에는 파일이 삭제됩니다.

이 채널은 [MCP 서버](/ko/automation/mcp-setup/)가 Moonpool의 실행 여부를 아는 방법이기도 합니다. `ping`에 응답하면 실행 중이고, 파이프나 소켓이 없으면 실행 중이 아닙니다. 아래의 진단용 동사를 제외한 같은 동사를 [명령줄](/ko/automation/command-line/)에서도 사용할 수 있습니다.

## 프로토콜

입력은 한 줄에 JSON 객체 하나, 출력은 한 줄에 JSON 하나이며 순서대로 처리됩니다. 하나의 연결로 여러 요청을 보낼 수 있습니다.

```json title="request"
{"cmd": "restart", "args": ["my-app"]}
```

```jsonl title="replies"
{"ok": true, "result": "..."}
{"ok": false, "error": "unknown app id: ..."}
```

PowerShell에서 요청과 응답을 주고받는 예:

포터블 복사본은 `moonpool` 대신 해당 파이프 이름(`paths` 동사가 보여 주는 `moonpool-<id>`)을 사용하세요.

```powershell frame="terminal"
$p = New-Object System.IO.Pipes.NamedPipeClientStream('.', 'moonpool', 'InOut')
$p.Connect(2000)
$w = New-Object System.IO.StreamWriter($p); $w.AutoFlush = $true
$r = New-Object System.IO.StreamReader($p)
$w.WriteLine('{"cmd":"ping"}')
$r.ReadLine()
```

```json title="reply"
{"ok":true,"result":"pong"}
```

- `args`는 문자열 목록이며 생략할 수 있습니다. 다른 필드는 무시됩니다.
- `result`는 문자열 또는 null입니다. 구조화된 데이터를 반환하는 동사는 JSON 문자열로 반환합니다.
- 유효한 JSON이 아닌 줄에는 `{"ok": false, "error": "bad request: ..."}`가 돌아옵니다.
- 알 수 없는 `cmd`에는 `unknown cmd: <name>`이 돌아옵니다.
- 창을 거치는 동사(`launch`, `stop`, `restart`, `reload`, `refresh-icons`, `help`, `open-window`)는 동작이 끝나면 응답하고, 45초가 지나면 시간 초과 오류로 응답합니다. 허브 창의 UI가 아직 로드되지 않았다면 `frontend not loaded`로 즉시 실패합니다.
- 이전 Moonpool이 아직 종료 중일 때 시작한 Moonpool은 약 8초 동안 채널 바인딩을 재시도합니다. 그래도 안 되면 이를 기록하고 채널 없이 계속 실행합니다.

## 동사

| 동사 | 인수 | 결과 |
| --- | --- | --- |
| `ping` | 없음 | `pong`. 채널 전용. |
| `list` | 없음 | 실행 중인 허브의 메모리에서 읽은 JSON 문자열 `{"apps": [...], "statuses": [...]}`로, `state.json`과 같은 `apps`, `statuses` 형태입니다. 앱은 등록되었지만 첫 상태 확인이 아직 실행되지 않았다면 `"statusNotReady": true`가 추가됩니다. `apps.json` 로드에 실패한 동안에는 `"manifestError": "<message>"`가 추가되고(이때 앱은 마지막으로 불러온 목록), 시작한 뒤 불러온 목록이 없으면 `"manifestLoaded": false`도 추가됩니다. 채널 전용. |
| `show` | 없음 | null. 창을 맨 앞으로 가져옵니다. |
| `quit` | 없음 | null. Moonpool을 종료합니다. |
| `launch` | `<id>` | 성공하면 null, `url`만 있는 `static` 항목이면 `opened`. 오류: `unknown app id: <id>`, `did not reach running in time`. |
| `stop` | `<id>` | 성공하면 null, `url`만 있는 `static` 항목이면 `stopped`. 오류: `still running after stop`. |
| `restart` | `<id>` | `launch`와 같은 결과와 오류. |
| `reload` | 없음 | 성공하면 null. |
| `refresh-icons` | 없음 | 성공하면 null. |
| `help` | 없음 | null. 도움말 창을 엽니다. |
| `dump` | `<id>` [`out-path`] | 앱의 세션 로그 경로, 또는 `out-path`에 만든 일반 텍스트 복사본의 경로. |
| `paths` | 없음 | 허브가 사용하는 폴더와 exe를 여러 줄로 보고합니다. |
| `read-config` | 없음 | `token`, `valid`, `error`, `path`, `manifest_text`를 담은 `dumps\read-config.json`의 경로. |
| `write-config` | `<source-file>` [`token`] | 새 버전 토큰. 오류: `stale token: ...`, `rejected invalid manifest: ...`, `cannot read source ...`. |
| `restore-config` | [`index` 또는 `filename`] | 인수가 없으면 `dumps\restore-config.json`(`count`, `snapshots`)의 경로. 있으면 `restored <file> (<n> apps); new version token <token>`. |
| `argv` | 명령줄 인수 | 즉시 null. 이 복사본에 대해 두 번째 `moonpool.exe <args>`를 실행했을 때와 똑같이(`--ticket` 포함) 인수를 처리합니다. 두 번째 실행이 종료하기 전에 인수를 넘기는 방식이 이것입니다. |

주고받는 예:

```jsonl title="request, reply"
{"cmd": "launch", "args": ["nope"]}
{"ok": false, "error": "unknown app id: nope"}

{"cmd": "restore-config", "args": ["1"]}
{"ok": true, "result": "restored 1767225600000.json (6 apps); new version token <token>"}
```

`write-config`와 `restore-config`는 새 매니페스트를 즉시 불러오고, `apps.json.history\`에 스냅샷을 기록하며, 창을 새로 고칩니다.

## 진단용 동사(테스트)

채널 전용이며 명령줄에서는 사용할 수 없습니다. Windows 전용인 `screenshot`을 제외하면 Windows, Linux, macOS에서 모두 동작하며, `screenshot`은 다른 곳에서 `screenshot is not supported on this platform (Windows only)`로 응답합니다.

| 동사 | 인수 | 결과 |
| --- | --- | --- |
| `screenshot` | [`window`] [`max_dim`] | Windows 전용. 해당 Moonpool 창(기본값 `main`)의 PNG를 Base64로 반환합니다. 선택 사항인 `max_dim`은 긴 변의 픽셀 수를 제한합니다(320-2400으로 조정되며 기본값 320, MCP 도구는 항상 기본값 사용). 정수가 아닌 `max_dim`은 오류입니다. 허용되는 창: `main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`. 오류: `unknown window '<name>'`, `window '<name>' is not open`. 디스크에 쓰지 않습니다. |
| `open-window` | `<kind>` [`<id>`] | null. 메뉴 항목과 같은 방식으로 창을 엽니다. `kind`: `settings`, `about`, `installer`, `help`, `themes`, `editor`(선택 사항인 `<id>`를 주면 해당 앱의 앱 편집 대화상자를, 없으면 앱 추가를 엽니다), `terminal`(`<id>` 필수: 해당 앱의 터미널 탭을 선택하고 CLI 창이 보이도록 허브를 넓힙니다. 앱을 실행하지는 않습니다), `cli`(허브만 넓힙니다). 오류: `unknown window kind '<kind>'`, `terminal needs an app id`, `unknown app id: <id>`. `launch`처럼 허브 창을 거쳐 응답합니다. |
| `window-state` | [`window`] | JSON 문자열: `{"open":false}`, 또는 `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. |
| `stop-mcp` | `<id>` | `stopped`. 앱이 아니라 앱의 `<processName> mcp` 도우미를 종료합니다. 오류: `missing app id`, `unknown app id: <id>`. |
| `reset-mcp-seen` | [`<id>`] | `<id>: cleared` 또는 `<id>: was not marked seen`. id가 없으면 `cleared <n> entries`. 기억된 MCP 도우미 감지 기록을 지웁니다. |

명령줄의 `--ticket`과 `state.json` 결과 기록은 다른 채널에 속합니다. [명령줄](/ko/automation/command-line/#결과-확인하기)을 참고하세요. 채널 요청은 응답으로 결과를 받습니다.

## 관련 항목

- [명령줄](/ko/automation/command-line/)
- [AI 에이전트: 빠른 시작](/ko/automation/quick-start/#같은-동작-세-가지-방법)
