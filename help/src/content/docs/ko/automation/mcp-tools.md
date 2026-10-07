---
title: "Moonpool MCP 도구 레퍼런스: 매개변수와 결과"
description: "Moonpool MCP 서버가 에이전트에 제공하는 모든 도구의 매개변수, 반환값, 발생할 수 있는 오류 사례를 정리했습니다."
---

모든 도구는 PNG 이미지를 반환하는 `moonpool_screenshot`을 제외하고 텍스트를 반환합니다. 실패는 이유를 텍스트로 담고 오류로 표시된 도구 결과로 돌아옵니다. 설정 방법은 [MCP 설정](/ko/automation/mcp-setup/)을 참고하세요.

`app_id`를 받는 도구에는 `apps.json`에 있는 앱의 `id`가 필요합니다. 영문자, 숫자, `.`, `_`, `-`만 사용해야 하며 `-`로 시작하면 안 됩니다. 그렇지 않으면 "invalid app_id"로 호출이 실패합니다.

허브에 작용하는 대부분의 도구는 허브가 실행 중이 아닐 때 이 메시지와 함께 실패합니다. `moonpool_bootup_launcher`, `moonpool_shutdown_launcher`, `moonpool_raise_launcher`, `moonpool_launcher_paths`는 이 경우를 직접 처리합니다(각 행 참고). 포터블 복사본에서는 메시지에 `Moonpool (<folder>)`처럼 복사본 이름이 표시됩니다.

```text
Moonpool is not running - call moonpool_bootup_launcher first
```

결과를 기다리는 호출은 45초 후 시간 초과됩니다.

## 런처와 앱

`moonpool_list_apps` 결과 예시:

```text
site  [running] (managed by Moonpool)  Site
notes-app  [stopped]  [mcp: stopped]  Notes App
```

| 도구 | 매개변수 | 동작 |
| --- | --- | --- |
| `moonpool_list_apps` | 없음 | 앱마다 한 줄: `id  [running]` 또는 `[stopped]`, 해당하는 경우 `(managed by Moonpool)`, MCP 도우미가 감지된 적이 있으면 `[mcp: running]` 또는 `[mcp: stopped]`, 그다음 이름. 실행 중인 허브에 제어 채널(`list` 동사)로 물어보므로 실시간 정보입니다. Moonpool이 실행 중이 아니면 오래된 목록을 보여 주는 대신 "Moonpool is not running"으로 실패합니다. Moonpool을 시작한 직후 첫 상태 확인 전에는 앱이 `[status pending]`으로 표시됩니다. `apps.json`에 오류가 있는 동안에는 결과가 `apps.json has an error: <message>. This list is the last one that loaded; fix the file and call moonpool_reload_config.`로 시작합니다. Moonpool을 시작할 때 이미 파일이 손상되어 있었다면, 불러온 앱이 없다고 알리고 `moonpool_restore_config`도 제안합니다. |
| `moonpool_bootup_launcher` | 없음 | Moonpool 자체를 시작하고 제어 채널이 응답할 때까지 최대 30초 기다립니다. "Moonpool started" 또는 "Moonpool is already running"을 반환합니다. 새 프로세스가 곧바로 종료되면(아직 종료 중이던 Moonpool에 넘겨진 경우) 한 번 더 시작합니다. 채널을 점유한 무언가가 응답하지 않으면 Moonpool 프로세스가 멈췄을 수 있다고 알립니다. |
| `moonpool_shutdown_launcher` | 없음 | 트레이 메뉴의 종료와 같습니다. 제어 채널이 사라질 때까지 최대 30초 기다립니다. "Moonpool shut down" 또는 "Moonpool is not running"을 반환합니다. |
| `moonpool_raise_launcher` | 없음 | Moonpool 창을 맨 앞으로 가져옵니다. "window shown"을 반환합니다. Moonpool이 실행 중이 아니면 시작하고 "Moonpool was not running; started it"을 반환합니다. |
| `moonpool_start_app` | `app_id`(필수) | 앱을 시작하고 터미널 탭을 엽니다. 실행 상태가 되면 "launched"를, 그렇지 않으면 이유(`unknown app id: <id>`, 25초 후 `did not reach running in time`)를 반환합니다. `url`만 있는 `static` 항목은 페이지를 열고 마찬가지로 "launched"를 반환합니다. |
| `moonpool_stop_app` | `app_id`(필수) | 앱을 중지합니다. "stopped" 또는 `still running after stop`(15초 후) 같은 오류를 반환합니다. |
| `moonpool_restart_app` | `app_id`(필수) | 중지하고, 포트와 프로세스가 해제될 때까지 기다린 뒤 시작합니다. "restarted"를 반환합니다. |
| `moonpool_app_output` | `app_id`(필수), `tail_lines`(정수, 기본값 200, 최솟값 1) | 현재 Moonpool 세션에서 앱의 터미널 출력이며 ANSI 코드는 제거됩니다. 로그가 `tail_lines`보다 길면 텍스트가 전체 로그 경로를 알리는 줄로 시작합니다. 앱이 실행된 적이 없으면 `no console output recorded for '<id>' (not launched this session)`로 실패합니다. 로그는 있지만 비어 있으면 `(no output recorded for '<id>')`를 반환합니다. |
| `moonpool_stop_mcp_server` | `app_id`(필수) | 앱에 연결된 MCP 도우미 프로세스를 종료하고 앱은 계속 실행합니다. "stopped"를 반환합니다. 앱에 `processName`도 `mcpProcessName`도 없으면 아무 일도 하지 않습니다. |
| `moonpool_refresh_app_icons` | 없음 | 모든 앱 아이콘을 다시 가져옵니다. "icons refreshed"를 반환합니다. |

## 구성

이 도구들은 디스크의 파일이 아니라 허브를 통해 `apps.json`을 읽고 변경합니다. 쓰기에는 마지막으로 읽은 시점의 토큰이 필요하고, 오래된 토큰은 거부되며, 새 파일은 쓰기 전에 검증됩니다. 허브를 거치는 것이 중요한 이유는, 샌드박스 호스트의 에이전트에는 실제 설정 폴더 대신 비공개 복사본이 보일 수 있기 때문입니다.

| 도구 | 매개변수 | 동작 |
| --- | --- | --- |
| `moonpool_read_config` | 없음 | `manifest_text`(파일의 정확한 내용), `token`, `valid`, `error`(유효하면 null), `path`를 담은 JSON 텍스트. 파일이 없거나 비어 있으면 `token`은 `none`입니다. |
| `moonpool_write_config` | `manifest`(필수, 새 `apps.json` 전체 텍스트), `expected_token`(필수, 마지막으로 읽은 값) | 매니페스트를 검증하고 `apps.json`을 교체한 뒤 불러옵니다. `apps.json updated; new version token <token>`을 반환합니다. 오래된 토큰은 `stale token: apps.json changed since it was read ...`로 실패합니다. 잘못된 매니페스트는 `rejected invalid manifest: ...`로 실패합니다. 어느 쪽이든 파일은 그대로입니다. 비어 있는 `expected_token`은 거부됩니다. |
| `moonpool_restore_config` | `snapshot`(선택) | 값이 없으면 저장된 스냅샷을 최신순으로 나열한 JSON 텍스트(`index`, `filename`, `millis`, `app_count`, `valid`)를 반환합니다. 인덱스(1 = 최신) 또는 파일 이름을 주면 해당 스냅샷을 검증하고 복원합니다. `restored <file> (<n> apps); new version token <token>`을 반환합니다. 복원은 의도적으로 현재 파일을 덮어쓰므로 토큰이 필요 없습니다. |
| `moonpool_reload_config` | 없음 | `apps.json`을 다시 읽습니다. "apps.json reloaded"를 반환합니다. 파일을 파싱하거나 검증할 수 없으면 `apps.json has an error: ...`로 실패하며, Moonpool은 마지막으로 불러온 목록을 유지합니다. |
| `moonpool_launcher_paths` | 없음 | 허브의 설정 폴더, `apps.json`, `state.json`, 로그, dumps 폴더, 아이콘 폴더, 포터블 플래그, exe 경로를 나열한 다음, MCP 프로세스의 설정 폴더, `apps.json`, `state.json`, dumps 폴더, 포터블 플래그, exe 경로(로그와 아이콘은 제외)를 나열합니다. 허브가 실행 중이 아니면 허브 쪽은 `hub paths unavailable: ...`로 표시되고 MCP 쪽은 그대로 표시됩니다. 수정이 반영되지 않을 때 사용하세요. |

## 고급: 테스트용 도구

`moonpool_screenshot`은 Windows 전용입니다. Linux에서는 "screenshot is not supported on this platform"으로 실패합니다. `moonpool_window_state`와 `moonpool_reset_mcp_seen`은 모든 플랫폼에서 동작합니다.

`window`는 `main`, `settings`, `about`, `installer`, `editor`, `help`, `themes` 중 하나이며 기본값은 `main`입니다. 알 수 없는 이름은 `unknown window '<name>'`으로 실패합니다.

| 도구 | 매개변수 | 동작 |
| --- | --- | --- |
| `moonpool_screenshot` | `window`(선택) | 해당 Moonpool 창 자체의 내용을 긴 변 기준 최대 320픽셀의 인라인 PNG로 캡처합니다. MCP에서는 크기를 키울 수 없습니다. 창이 표시되어 있지 않으면 `window '<name>' is not open`으로 실패합니다. 다른 앱은 캡처할 수 없습니다. |
| `moonpool_window_state` | `window`(선택) | JSON 텍스트: 창이 열려 있지 않으면 `{"open":false}`, 열려 있으면 `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. 테스트용입니다. |
| `moonpool_reset_mcp_seen` | `app_id`(선택) | 테스트 전용. 한 앱(생략하면 모든 앱)에 대해 기억된 "MCP 도우미가 감지됨" 기록을 지워, 도우미가 다시 감지될 때까지 사이드바의 MCP 하위 행이 숨겨지게 합니다. |

## 관련 항목

- [MCP 설정](/ko/automation/mcp-setup/)
- [명령줄](/ko/automation/command-line/)
