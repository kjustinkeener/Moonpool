---
title: "Moonpool 오류 메시지 설명: already running, requires a command 등"
description: "already running, requires a command, stale token, Update failed 같은 Moonpool 오류 메시지의 정확한 문구와 의미, 해결 방법을 찾아보세요."
---

보이는 메시지를 페이지 검색에 붙여 넣거나 표를 훑어보세요. 메시지는 Moonpool이 표시하는 그대로 인용했습니다. `<angle brackets>`로 둘러싼 텍스트는 값(앱 id, 경로, 시스템 오류)으로 바뀝니다. 오류 메시지가 아닌 증상은 [문제 해결과 FAQ](/ko/support/troubleshooting/)에 있습니다.

## 앱 시작과 중지

| 메시지 | 의미와 해결 |
| --- | --- |
| `already running` | Moonpool이 이미 이 앱의 터미널을 가지고 있습니다. 먼저 중지하거나 다시 시작을 사용하세요. |
| `stopped during launch` | 실행이 아직 시작되는 중에 중지를 눌렀습니다. 다시 실행하세요. |
| `app has no launch command` | 항목에 `command`가 없습니다. 앱 편집기나 `apps.json`에서 추가하세요. `url`이 있는 `static` 항목만 없어도 됩니다. |
| `unknown app: <id>` | 해당 `id`의 앱이 로드되어 있지 않습니다. id를 확인하고, `apps.json`을 직접 수정했다면 새로 고침하세요. |
| `unknown app id: <id>` | 같은 문제를 스크립트나 에이전트에 알리는 메시지입니다. `moonpool_list_apps`로 앱 목록을 확인하세요. |
| `did not reach running in time` | 스크립트나 에이전트에서: 앱이 25초 안에 실행 중으로 인식되지 않았습니다. `port` 또는 `processName`을 확인하고 출력을 읽어 보세요. [상태 점이 올바르지 않을 때](/ko/support/troubleshooting/#상태-점이-올바르지-않습니다)를 참고하세요. |
| `still running after stop` | 15초 후에도 앱이 실행 중으로 인식됩니다. `killMode`를 설정하세요. [중지와 다시 시작](/ko/apps/stop-and-restart/)을 참고하세요. |
| `refusing to open non-web url: <url>` | `url`이 `http://`, `https://`, `mailto:`, `file://` 중 어느 것도 아닙니다. `url`을 고치세요. |
| `[프로세스가 종료됨]` | 오류가 아닙니다. 앱의 명령이 끝났습니다. 터미널 탭에 표시됩니다. |

## apps.json 검증

Moonpool은 규칙을 어기는 `apps.json`을 거부하고 마지막으로 불러온 목록을 유지합니다. `<n>`은 파일에서 항목의 위치이며 1부터 셉니다.

| 메시지 | 해결 |
| --- | --- |
| `apps.json entry <n> has invalid id "<id>"; use letters, digits, '.', '_', and '-' without a leading '-'` | `id`의 이름을 바꾸세요. |
| `duplicate app id "<id>"` | 두 항목이 같은 `id`를 사용합니다. 각각 고유하게 만드세요. |
| `apps.json entry <n> (<id>) has an empty name` | `name`을 채우세요. |
| `apps.json entry <n> (<id>) has an empty group` | `group`을 채우세요. |
| `apps.json entry <n> (<id>) has unknown type "<type>"` | `type`은 `web`, `desktop`, `static`, `cli` 중 하나여야 합니다. |
| `apps.json entry <n> (<id>) has invalid port 0` | `port`는 1에서 65535 사이여야 합니다. |
| `apps.json entry <n> (<id>) requires a url` | `static` 항목에는 `url`이 필요합니다. |
| `apps.json entry <n> (<id>) requires a command` | 그 외의 모든 유형에는 `command`가 필요합니다. |

앱 편집기에서 이름 없이 저장하면 `name은 필수입니다.`가 표시됩니다. 배너 문구 "apps.json에 오류가 있어 마지막으로 불러온 목록을 표시합니다." 또는 "apps.json에 오류가 있어 불러온 앱이 없습니다.", 그리고 복구 방법은 [apps.json에 오류가 있을 때](/ko/support/troubleshooting/#appsjson에-오류가-있습니다)에 있습니다. 배너에 저장이 일시 중지되었다고 나오면 메시지는 `Repair apps.json and reload it before saving from Moonpool`로 끝납니다. 전체 규칙 목록은 [검증](/ko/apps/apps-json/#유효성-검사)에 있습니다.

## 설정, 업데이트, 설치 프로그램

| 메시지 | 의미와 해결 |
| --- | --- |
| `... Repair settings.json and restart Moonpool before changing settings` | `settings.json`의 형식이 잘못되었습니다. 고치거나 삭제한 뒤 다시 시작하세요. [settings.json](/ko/data/settings-json/#읽기와-복구)을 참고하세요. |
| `업데이트 실패: <error>` | 업데이트의 다운로드 또는 설치에 실패했습니다. [업데이트가 실패할 때](/ko/data/updating/#업데이트가-실패할-때)를 참고하세요. |
| `업데이트 확인 실패: <error>` | 정보 창의 업데이트 확인에 실패했습니다. 콜론 뒤의 텍스트가 이유를 알려 줍니다. 나중에 다시 시도하세요. |
| `설치 실패: <error>` | 콜론 뒤에 이름이 나온 단계(예: `copy exe: ...`)에서 설치 프로그램이 멈췄습니다. `%USERPROFILE%\.moonpool`에서 실행 중인 Moonpool을 모두 종료한 뒤 재시도하세요. |
| `target folder does not exist` | 포터블 복사본용으로 고른 폴더가 없어졌습니다. 존재하는 폴더를 고르세요. |

## MCP와 스크립트

| 메시지 | 의미와 해결 |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Moonpool을 시작하거나 에이전트가 해당 도구를 호출하게 하세요. 포터블 복사본에서는 메시지에 복사본 이름이 표시됩니다. |
| `frontend not loaded` | 허브 창이 아직 로드를 마치지 않았습니다. 기다렸다가 재시도하세요. |
| `invalid app_id: use only letters, digits, '.', '_', '-' (no leading '-')` | 에이전트가 MCP 서버가 받아들이지 않는 id를 전달했습니다. `moonpool_list_apps`의 id를 사용하세요. |
| `stale token: apps.json changed since it was read ...` | `apps.json`을 다시 읽고 수정 내용을 다시 적용한 뒤 쓰세요. |
| `rejected invalid manifest: ...` | 새 `apps.json`이 검증에 실패했습니다(위 참고). 파일은 변경되지 않았습니다. |
| `no console output recorded for '<id>' (not launched this session)` | Moonpool을 시작한 뒤 실행된 적이 없는 앱에 `moonpool_app_output`을 요청했습니다. |

더 많은 내용은 [MCP 도구](/ko/automation/mcp-tools/)와 [MCP 설정](/ko/automation/mcp-setup/#도구가-동작하지-않을-때)에 있습니다.

## 다른 프로그램의 오류

- [`Error: listen EADDRINUSE: address already in use :::3000` 및 `Port 5173 is in use`](/ko/support/port-already-in-use/)
- [`Windows protected your PC`](/ko/support/windows-protected-your-pc/)
- [WebView2 런타임 없음](/ko/support/webview2-runtime-missing/)
