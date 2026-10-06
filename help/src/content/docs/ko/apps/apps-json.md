---
title: "apps.json 편집하기: 위치, 새로 고침, 복구 방법"
description: "Moonpool이 관리하는 모든 앱이 담긴 apps.json 파일을 찾아 앱 편집기나 직접 편집하고, 새로 고치고, 잘못된 편집에서 복구하는 방법입니다."
---

Moonpool이 관리하는 모든 앱은 `apps.json`의 항목 하나입니다. 앱 편집기(앱 추가 및 앱 편집 대화 상자)로
편집하거나 직접 편집할 수 있으며, 두 방법 모두 같은 파일에 기록합니다. 일부 도구 결과와 메시지에서는 이
파일을 매니페스트라고 부릅니다.

## 설정 위치

| 모드 | 설정 폴더 |
| --- | --- |
| 설치형 (Windows) | `%USERPROFILE%\.moonpool\moonpool-config\` |
| 포터블 | `moonpool.exe` 옆의 `moonpool-config\` (`.moonpool\` 폴더 안) |
| Linux | `$XDG_CONFIG_HOME/Moonpool/`, 없으면 `~/.config/Moonpool/` |

`apps.json`은 그 폴더 안에 있으며, 다음 항목들과 같은 위치에 있습니다.

| 항목 | 용도 |
| --- | --- |
| `apps.json.history\` | 마지막 유효한 `apps.json` 파일 10개를 담은 롤백 링. |
| `settings.json` | 앱 설정. [settings.json](/ko/data/settings-json/)을 참고하세요. |
| `cli-output\<id>\` | 앱별 세션 로그. [로그](/ko/data/logs/)를 참고하세요. |
| `moonpool.log` | **디버그 정보를 파일에 기록**이 켜져 있는 동안의 디버그 로그. |
| `icons\` | 선택 사항인 `<id>.png`(`.ico`, `.svg`, `.jpg`, `.jpeg`, `.webp`도 가능) 아이콘 재정의. |
| `state.json` | 몇 초마다 갱신되는 실시간 상태 스냅숏. |
| `dumps\` | `dump`, `read-config`, `restore-config` 동사가 기록하는 파일. |
| `mcp_seen.json` | MCP 도우미가 있었던 앱의 기록. |
| `window-state.json` | 허브 창의 크기와 위치. |
| `AI-README.md` | AI 에이전트용 안내서이며, 실행할 때마다 다시 작성됩니다. |

이 중 무엇을 백업할지는 [백업 및 복구](/ko/data/backup-and-recovery/#설정-폴더)에 있습니다.

처음 실행할 때 Moonpool은 `apps.json`에 예제 항목을 채워 넣습니다. 이미 있는 파일은 절대 덮어쓰지
않습니다.

## 편집

- **대화 상자.** 사이드바 상단 **...** 메뉴의 **앱 추가**를 사용합니다. 앱을 바꾸려면 행의 연필을
  사용하거나 행을 오른쪽 클릭해서 **편집**을 선택합니다. 대화 상자는 즉시 유효성을 검사하고
  저장합니다.
- **직접 편집.** 같은 메뉴의 **apps.json 편집**은 파일을 기본 편집기에서 엽니다. 저장한 다음
  메뉴에서 **새로 고침**을 선택합니다(또는 F5나 Ctrl+R을 누릅니다).

직접 편집한 내용은 새로 고침하기 전까지 반영되지 않습니다. 새로 고침은 파일을 읽기만 할 뿐 다시
쓰지는 않습니다.

대화 상자에서 저장하면 파일 전체가 정규화되고 들여쓰기된 형태로 다시 쓰입니다. Moonpool이 알지
못하는 키는 삭제되며 JSON에는 주석이 없으므로, 메모는 `note` 필드에 남기세요.

## 형식

파일은 객체의 JSON 배열입니다. 모든 항목에 네 개의 키가 필요합니다. `id`, `name`, `group`, `type`입니다.
나머지는 모두 선택 사항입니다. [앱 필드](/ko/apps/fields/)를 참고하세요.

```json title="apps.json"
[
  { "id": "site", "name": "Site", "group": "Web apps", "type": "web",
    "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
    "url": "http://localhost:5173", "openBrowser": true }
]
```

그룹은 파일에 처음 나타난 순서대로 사이드바에 표시됩니다.

## 새로 고침이 하는 일

새로 고침은 Moonpool의 메모리 안 목록을 파일 내용으로 교체합니다. 실행, 중지, 다시 시작은 클릭할 때
항목을 읽으므로, 편집한 `command`, `cwd`, `env`, 종료 설정은 해당 앱을 다음에 시작하거나 다시 시작할 때
적용됩니다. 새로 고침은 아무것도 다시 시작하지 않습니다. 이미 실행 중인 앱은 시작할 때의 설정으로
계속 실행됩니다.

## 유효성 검사

Moonpool은 파일을 불러올 때, 저장할 때마다, 에이전트가 쓸 때마다 파일 전체의 유효성을 검사합니다.
잘못된 항목이 하나만 있어도 파일 전체가 거부됩니다.

| 규칙 | 오류에 포함되는 문구 |
| --- | --- |
| 올바른 JSON이 아니거나, 필수 키가 없거나, 값의 형식이 잘못됨 | JSON 파서 메시지 |
| `id`가 비어 있거나, `-`로 시작하거나, 문자, 숫자, `.`, `_`, `-` 이외의 문자가 있음 | `invalid id` |
| 두 항목이 같은 `id`를 가짐 | `duplicate app id` |
| `name`이 공백임 | `has an empty name` |
| `group`이 공백임 | `has an empty group` |
| `type`이 `desktop`, `web`, `static`, `cli` 중 하나가 아님 | `unknown type` |
| `port`가 `0`임(65535를 넘는 `port`는 파싱에 실패함) | `invalid port 0` |
| `url`이 없는 `static` 항목 | `requires a url` |
| `command`가 없는 그 밖의 유형 | `requires a command` |

오류는 위치로 항목을 가리킵니다. 예를 들면 다음과 같습니다.

```text
apps.json entry 2 (site) requires a command
```

### id

`id`는 항목의 영구적인 키입니다. 로그 폴더와 아이콘 파일의 이름이 되며, `moonpool.exe launch <id>`와
에이전트에 전달하는 값이기도 합니다. 대화 상자는 앱을 추가할 때 이름에서 id를 만듭니다. 이름을
소문자로 바꾸고, `a`~`z`와 `0`~`9`가 아닌 문자의 연속을 `-` 하나로 바꾸고, 양쪽 끝의 `-`를
제거합니다. 결과가 비면 `app`이 됩니다. id가 이미 사용 중이면 `-2`, `-3` 등을 붙입니다. 이후에는
id를 절대 바꾸지 않으므로 앱 이름을 바꿔도 id는 유지됩니다. 이름 `Habit Tracker`의 id는
`habit-tracker`가 됩니다.

## 파일이 손상된 경우

- **새로 고침할 때** 유효성 검사에 실패한 파일은 그대로 두고 Moonpool은 마지막으로 불러온 목록을
  유지합니다. 사이드바 위의 배너에 오류가 표시되며 파일을 여는 버튼이 있습니다. 목록은 흐리게
  표시되지만 계속 사용할 수 있습니다.
  [apps.json에 오류가 있을 때](/ko/using/hub-window/#appsjson에-오류가-있을-때)를 참고하세요.
- **시작할 때** 파일이 손상되어 있으면 유지할 목록이 없으므로 Moonpool은 앱 없이 시작하고 배너에 그
  사실이 표시됩니다. 파일을 고치고 **새로 고침**을 선택하거나, 스냅숏을 복원하세요(아래 내용 또는
  `moonpool_restore_config` 도구).
- 어느 경우든 파일을 다시 불러올 수 있을 때까지 대화 상자에서의 저장(그리고 이름 바꾸기, 삭제,
  아이콘 설정)은 거부되므로 손상된 파일이 덮어써지는 일은 없습니다. 파일을 고치고 **새로 고침**을
  선택하세요.
- **대화 상자, 에이전트, 복원**을 통한 잘못된 변경은 거부되며 디스크의 파일은 그대로 유지됩니다.

Moonpool은 `apps.json.history\`에 `apps.json`의 마지막 정상 버전 10개를 보관합니다. 롤백하는 방법은
[백업 및 복구](/ko/data/backup-and-recovery/#appsjson-되돌리기)에 있습니다. 증상과 해결 방법은
[문제 해결](/ko/support/troubleshooting/#appsjson에-오류가-있습니다)에 있습니다.

## 에이전트

AI 에이전트는 파일 대신 Moonpool의 MCP 도구로 `apps.json`을 변경해야 합니다. 그래야 오래되었거나
잘못된 쓰기가 거부되고 샌드박스 안의 에이전트가 비공개 복사본을 편집하는 일이 없습니다.
[MCP 도구](/ko/automation/mcp-tools/#구성)를 참고하세요.
