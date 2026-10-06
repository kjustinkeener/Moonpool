---
title: "Moonpool 백업, apps.json 되돌리기, 설정 복구"
description: "백업할 항목, 잘못된 apps.json 되돌리기, 예제 앱으로 초기화, 설치형을 포터블로 옮기기, 제거 시 삭제되는 항목을 안내합니다."
---

Moonpool이 보관하는 모든 것은 설정 폴더와 대시보드 폴더, 두 곳에 있습니다. 모드별 경로는 [설정이 있는 위치](/ko/apps/apps-json/#설정-위치)에 있습니다.

## 설정 폴더

```text
moonpool-config\
  apps.json            your apps                              back up
  apps.json.history\   the last 10 good apps.json files       back up (optional)
  settings.json        app settings                           back up
  icons\               icon overrides, <id>.png and so on     back up
  cli-output\<id>\     session logs                           disposable
  moonpool.log         debug log                              disposable
  state.json           live status snapshot                   disposable
  dumps\               files written by dump and read-config  disposable
  mcp_seen.json        which apps had an MCP helper           disposable
  window-state.json    hub window size and position           disposable
  AI-README.md         rewritten at every launch              disposable
  webview\             the window's browser profile (Windows) disposable
```

대시보드 폴더는 `{MP_HOME}\dashboards`입니다. 설치형은 `%USERPROFILE%\.moonpool\dashboards`, 포터블은 `<your .moonpool folder>\dashboards`, Linux는 설정 폴더 안의 `dashboards/`입니다. 그 안에 직접 만든 파일이 있다면 백업하세요. `examples` 폴더는 Moonpool 소유이며 업데이트할 때 다시 작성됩니다.

테마는 복사할 수 있는 파일이 아니라 창의 브라우저 저장소에 보관됩니다. 백업에 포함되지 않으므로, 복원한 뒤 다시 선택하세요.

## 백업

1. 파일이 반쯤 기록된 상태가 되지 않도록 Moonpool을 종료합니다.
2. 설정 폴더의 `apps.json`, `settings.json`, `icons\`와 `dashboards\`에 있는 직접 만든 파일을 복사합니다.

```powershell frame="terminal"
$cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
Copy-Item "$cfg\apps.json", "$cfg\settings.json" D:\backup\
Copy-Item "$cfg\icons" D:\backup\ -Recurse
```

복원하려면 Moonpool을 종료하고 파일을 다시 복사한 뒤 실행하세요.

## apps.json 되돌리기

저장, 에이전트의 쓰기, 복원이 성공할 때마다, 그리고 내용이 바뀐 것을 발견한 새로 고침마다 검증된 `apps.json`이 `apps.json.history\`에 복사되며, 최신 10개가 보관됩니다. 각 파일의 이름은 만든 시각(예: `1767225600000.json`)입니다. `apps.json.bak`은 없습니다.

- **직접 하기.** 스냅샷을 `apps.json`에 덮어 복사한 뒤 **새로 고침**을 선택합니다.

  ```powershell frame="terminal"
  $cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
  Copy-Item "$cfg\apps.json.history\<snapshot>" "$cfg\apps.json"
  ```

- **스크립트에서.** `moonpool.exe restore-config`는 스냅샷 목록을 보여 주고, `moonpool.exe restore-config 1`은 최신 스냅샷을 복원합니다. [명령줄](/ko/automation/command-line/)을 참고하세요.
- **에이전트에서.** `moonpool_restore_config`. [MCP 도구](/ko/automation/mcp-tools/#구성)를 참고하세요.

자동으로 복원되는 것은 없습니다.

## 손상된 파일

- **apps.json.** Moonpool은 손상된 파일을 절대 덮어쓰지 않습니다. [파일에 문제가 있을 때](/ko/apps/apps-json/#파일이-손상된-경우)를 참고하세요.
- **settings.json.** 파일을 고치거나, 삭제해서 모든 설정을 초기화한 다음 Moonpool을 다시 시작하세요. [settings.json](/ko/data/settings-json/#읽기와-복구)을 참고하세요.

## 예제로 초기화

Moonpool은 `apps.json`이 없을 때만 예제 앱을 씁니다. 처음부터 다시 시작하려면 Moonpool을 종료하거나(또는 실행한 채로 두고) `apps.json`의 이름을 바꾸거나 삭제한 다음, Moonpool을 시작하거나 **새로 고침**을 선택하세요. 예제가 담긴 새 `apps.json`이 작성됩니다.

## 설치형에서 포터블로

새 포터블 복사본은 예제 앱으로 시작합니다. 내 앱을 옮기려면 [포터블 모드](/ko/data/portable-mode/#설치-프로그램에서-포터블-선택하기)를 참고하세요. 필요하면 `icons\`와 `settings.json`도 같은 방법으로 복사하세요.

## 제거

설치된 Moonpool을 제거하면 설정 폴더와 대시보드를 포함한 `%USERPROFILE%\.moonpool` 폴더 전체가 삭제됩니다. 먼저 백업하세요. [제거](/ko/getting-started/install/#제거)를 참고하세요. 포터블 복사본은 `.moonpool\` 폴더를 삭제하면 제거됩니다.
