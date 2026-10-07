---
title: "스크립트와 AI 에이전트로 Moonpool 자동화하기"
description: "실행 중인 Moonpool을 스크립트와 AI 에이전트로 제어하는 세 가지 방법(MCP, 명령줄, 제어 동사)과 서로의 관계, 각각이 바꿀 수 있는 항목을 설명합니다."
---

Moonpool은 창을 건드리지 않고도 제어할 수 있습니다. 표면(surface)은 세 가지이며, 모두 상주하는 같은 Moonpool(트레이 인스턴스, 여기서는 허브라고 부릅니다)이 처리합니다.

Moonpool 복사본마다 각자 하나의 허브입니다. 설치된 복사본과 모든 포터블 복사본은 서로 독립적으로 실행되며, 각각 자체 제어 채널을 가집니다. 어떤 표면이든 사용한 `moonpool.exe`에 해당하는 복사본에 연결됩니다. [포터블 모드](/ko/data/portable-mode/#여러-복사본을-동시에)를 참고하세요.

| 표면 | 설명 | 참고 |
| --- | --- | --- |
| MCP 서버 | `moonpool.exe mcp`. AI 호스트가 시작하는 stdio [MCP](https://modelcontextprotocol.io) 서버입니다. | [MCP 설정](/ko/automation/mcp-setup/), [MCP 도구](/ko/automation/mcp-tools/) |
| 명령줄 | `moonpool.exe <verb> [args]`. 같은 복사본을 한 번 더 실행하면 제어 채널을 통해 동사를 해당 허브에 전달하고 종료합니다. | [명령줄](/ko/automation/command-line/) |
| 제어 채널 | Windows에서는 이름 있는 파이프 `\\.\pipe\moonpool`(포터블 복사본은 `\\.\pipe\moonpool-<id>`), Linux에서는 Unix 소켓이며, 한 줄에 JSON 요청 하나를 주고받습니다. | [제어 동사](/ko/automation/control-verbs/) |

## 서로의 관계

- 앱 실행, 세션 로그, `apps.json` 등 모든 것은 허브가 소유합니다.
- MCP 서버는 허브의 복사본이 아니라 허브의 클라이언트입니다. 대부분의 도구 호출은 제어 채널을 통해 허브로 전달되고, 응답이 도구 결과로 돌아옵니다. 예외는 다음과 같습니다. `moonpool_bootup_launcher`는 `moonpool.exe`를 직접 시작합니다. `moonpool_app_output`과 설정 도구는 허브에 파일을 쓰게 한 뒤 그 파일을 읽습니다. `moonpool_launcher_paths`는 허브의 경로에 MCP 프로세스 자체의 경로를 덧붙입니다.
- 허브가 실행 중인지는 프로세스를 찾는 것이 아니라 그 채널에 ping을 보내 판단합니다. 응답하는 허브는 실행 중이고, 파이프나 소켓이 없으면 실행 중이 아닙니다.
- 모든 표면은 창과 같은 핸들러를 실행하므로, 동사는 대응하는 클릭과 같은 일을 합니다.
- 실행 중인 허브가 없으면 `moonpool_list_apps`를 포함해 허브에 작용하는 도구는 "Moonpool is not running"이라며 거부합니다. 오래된 목록을 보여 주지 않습니다. `moonpool_bootup_launcher`가 허브를 시작합니다. 채널을 점유한 무언가가 몇 초 안에 응답하지 않으면, 오류 메시지에 Moonpool 프로세스가 멈췄을 수 있다고 표시됩니다.
- MCP 서버는 제어 채널이 생기기 전의 허브 빌드를 더 이상 대신 제어하지 않습니다. 해당 복사본을 업데이트하거나, 종료한 뒤 다시 시작하세요.

## 바꿀 수 있는 것

| 변경 항목 | 표면 |
| --- | --- |
| 앱 시작, 중지, 다시 시작 | MCP, 명령줄, 파이프 |
| `apps.json` 다시 쓰기 | MCP(`moonpool_write_config`, `moonpool_restore_config`), 명령줄, 파이프 |
| Moonpool 종료 | MCP(`moonpool_shutdown_launcher`), 명령줄(`quit`), 파이프 |
| 앱의 MCP 도우미 프로세스 종료 | MCP(`moonpool_stop_mcp_server`), 파이프(`stop-mcp`) |
| `apps.json` 다시 불러오기, 아이콘 다시 가져오기, 창 표시 | MCP(`moonpool_reload_config`, `moonpool_refresh_app_icons`, `moonpool_raise_launcher`), 명령줄(`reload`, `refresh-icons`, `show`), 파이프 |
| 창 또는 터미널 탭 열기 | 파이프(`open-window`) |
| 기억된 MCP 도우미 감지 기록 지우기 | MCP(`moonpool_reset_mcp_seen`), 파이프(`reset-mcp-seen`) |

읽기 전용 도구: `moonpool_list_apps`, `moonpool_app_output`, `moonpool_read_config`, `moonpool_launcher_paths`, `moonpool_window_state`, `moonpool_screenshot`.

## 안전 속성

- **설정 쓰기는 보호됩니다.** 쓰기에는 마지막으로 읽은 시점의 버전 토큰이 있어야 하고, 오래된 토큰은 거부되며, 새 `apps.json`은 쓰기 전에 검증됩니다. 거부된 쓰기는 `apps.json`을 건드리지 않습니다. [MCP 도구](/ko/automation/mcp-tools/#구성)를 참고하세요.
- **앱 id는 제한됩니다.** MCP 서버는 영문자, 숫자, `.`, `_`, `-`만 허용하며 맨 앞의 `-`는 허용하지 않으므로, id가 명령줄 플래그로 해석될 수 없습니다.
- **스크린샷은 Moonpool만 대상으로 합니다.** `moonpool_screenshot`은 Moonpool 자체 창(`main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`) 중 하나만 캡처하며, 화면 전체나 다른 앱은 캡처하지 않습니다. PNG는 메모리에서 만들어져 그대로 반환되며, Moonpool은 이를 파일로 저장하지 않습니다.
- **채널에는 인증이 없습니다.** Moonpool은 제어 파이프나 소켓에 로그인이나 토큰을 추가하지 않습니다. 열 수 있는 프로세스는 누구나 동사를 보낼 수 있습니다. Linux에서는 소켓 파일이 `0600` 모드로 만들어지므로 사용자 본인만 열 수 있습니다.
- **샌드박스 호스트를 감지합니다.** MCP 서버가 패키지(Store/MSIX) 샌드박스 안에서 실행되어 Moonpool 파일의 비공개 복사본을 보게 된다면, 파일을 읽거나 쓰는 도구(`moonpool_app_output`, `moonpool_read_config`, `moonpool_write_config`, `moonpool_restore_config`)는 오래된 데이터를 반환하는 대신 이유를 설명하는 오류를 반환합니다. 제어 채널만 쓰는 도구는 차단되지 않습니다. [MCP 설정](/ko/automation/mcp-setup/#샌드박스-호스트)을 참고하세요.

## 플랫폼

제어 채널은 모든 플랫폼에 있습니다. Windows에서는 이름 있는 파이프, Linux에서는 Unix 소켓입니다(위치는 [제어 동사](/ko/automation/control-verbs/#수신-위치) 참고). `screenshot`(따라서 `moonpool_screenshot`)만 Windows 전용이며, Linux에서는 "not supported on this platform"을 반환합니다. 명령줄 동사는 모든 플랫폼에서 동작합니다.

## 관련 항목

- [AI 에이전트: 빠른 시작](/ko/automation/quick-start/)
- [MCP 설정](/ko/automation/mcp-setup/)
