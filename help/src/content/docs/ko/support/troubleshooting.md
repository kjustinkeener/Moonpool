---
title: "Moonpool 문제 해결: 트레이, 시작되지 않는 앱, 업데이트"
description: "트레이 아이콘이 보이지 않을 때, 앱이 시작되거나 중지되지 않을 때, 상태 점이 틀릴 때, 업데이트 실패와 MCP 오류를 증상별로 해결합니다."
---

증상을 찾은 다음 해결 방법을 따르세요. 따옴표로 묶은 텍스트는 Moonpool이 표시하는 문구입니다. 정확한 메시지를 찾아보려면 [오류 메시지 설명](/ko/support/error-messages/)을 참고하세요.

## 트레이 아이콘이 보이지 않습니다

- **Windows.** 아이콘이 숨겨진 아이콘 영역에 있을 수 있습니다. 작업 표시줄 오른쪽의 **^** 화살표를 클릭하세요. 아이콘을 작업 표시줄로 끌어 놓으면 계속 표시됩니다.
- **기본 GNOME의 Linux.** GNOME은 AppIndicator 확장 없이는 트레이 아이콘을 표시하지 않습니다. [Linux](/ko/platforms/linux/#gnome의-트레이)를 참고하세요.
- **설정.** **트레이에 표시**가 꺼져 있을 수 있습니다. 작업 표시줄이나 시작 메뉴에서 허브를 열고 [설정](/ko/using/settings/)에서 다시 켜세요.

## 설치 프로그램에 오류가 표시됩니다

| 메시지 | 해결 방법 |
| --- | --- |
| `설치 실패: <error>` | 콜론 뒤의 텍스트는 실패한 단계(예: `copy exe: ...`)를 알려 줍니다. 파일이 사용 중이면 `%USERPROFILE%\.moonpool`에서 실행 중인 Moonpool을 모두 종료하고 다시 시도하세요. |
| `target folder does not exist` | 포터블 복사본용으로 고른 폴더가 없어졌습니다. 존재하는 폴더를 고르세요. |
| `that folder already has a .moonpool with an exe of this name that isn't a portable Moonpool - pick an empty folder` | 빈 폴더를 고르거나, 먼저 그 `.moonpool` 폴더를 삭제하세요. |

## 설치 프로그램을 실행하면 Windows protected your PC가 나타납니다

`moonpool.exe`에 코드 서명이 되어 있지 않아 나타나는 Windows SmartScreen입니다. **More info**(추가 정보)를 클릭한 다음 **Run anyway**(실행)를 클릭하세요. [Windows protected your PC](/ko/support/windows-protected-your-pc/)를 참고하세요.

## Windows에서 Moonpool 창이 비어 있거나 열리지 않습니다

Microsoft Edge WebView2 런타임이 없을 수 있습니다. [WebView2 런타임 없음](/ko/support/webview2-runtime-missing/)을 참고하세요.

## 앱이 시작되지 않습니다

1. 앱 이름을 클릭해 터미널 탭을 열고 출력을 읽어 보세요. 에이전트는 `moonpool_app_output`으로 같은 텍스트를 읽을 수 있습니다.
2. `cwd`를 확인하세요. 폴더가 없거나 `./` 없이 상대 경로를 쓴 경우가 흔한 원인입니다. [경로와 환경](/ko/apps/paths-and-environment/)을 참고하세요.
3. `command`를 확인하세요. `cwd`의 터미널에서 직접 실행해 보세요. Windows에서는 중첩된 큰따옴표를 피하세요. `cmd /c`가 이를 망가뜨립니다.
4. 설정에서 **디버그 정보를 파일에 기록**을 켜고 다시 실행하세요. `moonpool.log`에 정확한 명령과 폴더가 기록됩니다. [로그](/ko/data/logs/)를 참고하세요.

| 메시지 | 의미 |
| --- | --- |
| `already running` | Moonpool에 이미 이 앱의 터미널이 있습니다. 먼저 중지하거나 다시 시작을 사용하세요. |
| `stopped during launch` | 실행이 아직 시작되는 중에 중지를 눌렀습니다. |
| `did not reach running in time` | 스크립트나 에이전트에서: 앱이 25초 안에 실행 중으로 인식되지 않았습니다. `port` 또는 `processName`과 출력을 확인하세요. |

## 상태 점이 올바르지 않습니다

Moonpool은 `port`, 그다음 `processName`, 그다음 자체 터미널이 아직 살아 있는지를 기준으로 실행 중 여부를 판단합니다. [실행 중 여부를 판단하는 방법](/ko/apps/types/#실행-중-여부를-판단하는-방식)을 참고하세요.

- **점이 채워지지 않습니다.** `web` 앱의 `port`가 응답하지 않거나 `desktop` 앱의 `processName`이 일치하지 않습니다. Linux에서 `processName`은 15자 이하여야 합니다.
- **실행 직후 회색으로 바뀝니다.** `cli` 앱은 명령이 끝나면 실행 중이 아니게 됩니다. 열린 채로 두려면 `-NoExit`가 있는 셸을 사용하세요.
- **`static` 앱이 실행 중으로 표시되지 않습니다.** `url`만 있는 항목에서는 정상입니다.
- **시작하지 않았는데 실행 중으로 표시됩니다.** 다른 무언가가 그 포트나 프로세스 이름을 사용 중입니다. Moonpool은 이를 실행 중이지만 "Moonpool에서 관리됨"은 아닌 것으로 표시합니다.

## Error: listen EADDRINUSE 또는 "Port 5173 is in use"

서버가 사용하려는 포트에서 이미 다른 무언가가 수신 중입니다. 찾아서 종료하거나, 앱에 `port`를 설정해 중지할 때 포트가 해제되게 하세요. [EADDRINUSE와 "Port 5173 is in use" 해결하기](/ko/support/port-already-in-use/)와 [포트를 사용 중인 프로세스 찾아서 종료하기](/ko/guides/find-and-kill-process-using-port-windows/)를 참고하세요.

## 두 앱이 같은 포트를 사용합니다

**...** 메뉴 아래쪽에 `port 3000: App A / App B` 같은 경고 행이 나타납니다. 한 앱의 `port`(그리고 `PORT`를 읽는다면 `env`)를 변경하세요. [포트 충돌 경고](/ko/using/hub-window/#포트-충돌-경고)를 참고하세요.

## 중지한 뒤에도 앱이 계속 실행됩니다

스크립트나 에이전트에서는 오류가 `still running after stop`(15초 후)입니다.

- 앱이 터미널보다 오래 살아남습니다. `killMode`를 `port` 또는 `processName`으로 설정하세요. [중지와 다시 시작](/ko/apps/stop-and-restart/)을 참고하세요.
- Windows의 Docker 앱: `docker compose stop app` 같은 `stopCommand`와 함께 `killMode`를 `command`로 사용하세요. `port`는 절대 사용하지 마세요.

## apps.json에 오류가 있습니다

사이드바에 "apps.json에 오류가 있어 마지막으로 불러온 목록을 표시합니다." 배너가 나타나거나, 시작할 때 "apps.json에 오류가 있어 불러온 앱이 없습니다."가 나타납니다. 파일이 다시 로드될 때까지 Moonpool에서의 저장이 일시 중지됩니다.

일반적인 오류:

```text
apps.json entry 2 (site) requires a command
apps.json entry 3 has invalid id "my app"; use letters, digits, '.', '_', and '-' without a leading '-'
duplicate app id "site"
apps.json entry 4 (api) has invalid port 0
```

1. 배너에서 **apps.json 편집**을 선택하고 항목을 고친 뒤 저장하고 **새로 고침**(F5)하세요.
2. 또는 최근의 정상 복사본으로 되돌리세요. [백업과 복구](/ko/data/backup-and-recovery/#appsjson-되돌리기)를 참고하세요.

전체 규칙 목록은 [검증](/ko/apps/apps-json/#유효성-검사)에 있습니다.

설정을 변경할 수 없고 메시지가 `Repair settings.json and restart Moonpool before changing settings`로 끝난다면, 설정 폴더의 `settings.json`을 고치거나 삭제하고 Moonpool을 다시 시작하세요. 삭제하면 모든 설정이 기본값으로 초기화됩니다.

## 수정한 내용이 반영되지 않습니다

- 직접 수정한 경우 **새로 고침**(또는 F5)이 필요합니다. Moonpool은 파일을 감시하지 않습니다.
- 새로 고침은 실행 중인 앱을 다시 시작하지 않습니다. 변경한 `command`, `cwd`, `env`를 적용하려면 앱을 다시 시작하세요.
- 에이전트가 다른 `apps.json`을 편집하고 있을 수 있습니다. 에이전트에게 `moonpool_launcher_paths`를 호출하게 하여 허브의 폴더와 자신의 폴더를 비교하게 하세요. Moonpool 복사본이 여러 개라면 어느 복사본을 편집하는지 확인하세요.

## 예제 앱이 없습니다

예제는 `apps.json`이 없을 때만 작성됩니다. 되살리려면 [예제로 초기화](/ko/data/backup-and-recovery/#예제로-초기화)를 참고하거나, [예제 대시보드](/ko/getting-started/example-dashboards/#예제-앱은-처음-실행할-때만-나타납니다)의 항목을 복사하세요.

## 업데이트가 실패했습니다

배너에 `업데이트 실패: <error>`가 표시됩니다. [업데이트가 실패할 때](/ko/data/updating/#업데이트가-실패할-때)를 참고하세요.

## 웹 링크가 열리지 않습니다

`refusing to open non-web url: <url>`은 `url`이 `http://`, `https://`, `mailto:`, `file://` 중 어느 것도 아니라는 뜻입니다. `url`을 고치세요.

## MCP와 스크립트 오류

| 메시지 | 해결 방법 |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Moonpool을 시작하거나 에이전트가 `moonpool_bootup_launcher`를 호출하게 하세요. |
| `frontend not loaded` | 허브 창이 아직 로드를 마치지 않았습니다. 잠시 기다렸다가 재시도하세요. |
| `stale token: ...` | 에이전트가 읽은 뒤 `apps.json`이 바뀌었습니다. 다시 읽은 다음 쓰세요. |
| `rejected invalid manifest: ...` | 새 `apps.json`이 검증에 실패했습니다. 파일은 변경되지 않았습니다. |
| `... A Moonpool process may be hung ...` | 무언가 응답하지 않은 채 제어 채널을 점유하고 있습니다. 트레이에서 Moonpool을 종료하거나 프로세스를 끝낸 뒤 다시 시작하세요. |

더 많은 내용은 [MCP 설정](/ko/automation/mcp-setup/#참고-사항)과 [MCP 도구](/ko/automation/mcp-tools/)에 있습니다.

## 창 문제

- **화면 밖에 있음.** Moonpool은 연결된 어떤 디스플레이에도 없는 저장된 위치를 무시합니다. 그래도 창을 찾을 수 없으면 Moonpool을 종료하고 설정 폴더의 `window-state.json`을 삭제하세요.
- **확대/축소가 너무 크거나 작게 고정됨.** 허브 위에서 Ctrl + 휠로 변경합니다. [단축키와 확대/축소](/ko/using/keyboard-shortcuts/#확대축소)를 참고하세요.
- **설정이 허브 뒤에 열림.** 설정에서 **항상 위에 표시**를 껐다가 켜 보세요. 모든 Moonpool 창에 적용되므로 같은 계층에 머무릅니다.

## 로그는 어디에 있나요?

[로그](/ko/data/logs/)를 참고하세요.

## 백업, 초기화, 제거

[백업과 복구](/ko/data/backup-and-recovery/)와 [제거](/ko/getting-started/install/#제거)를 참고하세요.

## FAQ

**창을 닫으면 앱이 중지되나요?**
기본적으로 닫으면 Moonpool이 종료되며, Windows에서는 종료할 때 Moonpool이 시작한 앱이 중지됩니다. 창을 닫아도 Moonpool을 계속 실행하려면 **닫을 때 트레이로 숨기기**를 켜세요. [트레이, 닫기, 최소화](/ko/using/tray-and-closing/)를 참고하세요.

**Moonpool을 두 번 실행할 수 있나요?**
폴더당 하나입니다. 같은 복사본을 다시 시작하면 그 창이 다시 나타납니다. 설치된 복사본과 포터블 복사본은 나란히 실행할 수 있습니다. [포터블 모드](/ko/data/portable-mode/#여러-복사본을-동시에)를 참고하세요.

**Moonpool이 외부로 정보를 보내나요?**
업데이트 확인에만 사용합니다. 시작할 때(**시작할 때 업데이트 확인**이 켜져 있으면)와 **업데이트 확인**을 누를 때 GitHub에서 릴리스 파일(`update.json`)을 가져옵니다. 모든 다운로드는 사용하기 전에 Moonpool의 서명 키로 검증됩니다.

**내 명령은 어떤 셸에서 실행되나요?**
Windows에서는 `cmd /c`, Linux에서는 `$SHELL -c`입니다.

**비밀 값은 어디에 두나요?**
`env` 값은 `apps.json`에 일반 텍스트로 저장됩니다. 앱이 직접 읽는 파일이나, 실행된 앱이 상속하는 사용자 환경에 이미 설정된 변수를 사용하는 편이 좋습니다.

**제어 채널은 보호되나요?**
로그인이나 토큰이 없습니다. 사용자 권한으로 실행되는 모든 프로세스가 명령을 보낼 수 있습니다. Linux에서는 소켓을 사용자 본인만 읽을 수 있습니다. [안전 속성](/ko/automation/overview/#안전-속성)을 참고하세요.
