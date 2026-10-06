---
title: "앱 유형 선택하기: web, desktop, static, cli"
description: "web, desktop, static, cli 앱이 Moonpool에서 실행되는 방식, 유형별 실행 중 감지 방법, 중지 버튼의 기본 동작을 알아봅니다."
---

`type`은 어떤 필드가 중요한지와 중지의 기본 동작을 결정합니다.

| | `web` | `desktop` | `static` | `cli` |
| --- | --- | --- | --- | --- |
| 필요한 것 | `command` | `command` | `url` | `command` |
| 보통 함께 쓰는 것 | `port`, `url` | `processName` | 자체적으로 서비스한다면 `command`와 `port` | `cwd` |
| 실행 | 터미널 탭에서 `command` 실행 | 터미널 탭에서 `command` 실행 | `command`가 없으면 브라우저에서 `url`을 엽니다. 있으면 터미널 탭에서 실행 | 터미널 탭에서 `command` 실행 |
| 기본 `killMode` | `port` | `processName` | `none` | `none` |

![web 앱의 앱 편집 대화 상자: type이 web으로 설정되어 한 줄 설명이 표시되고, port 필드가 채워져 있습니다](../../../../assets/screenshots/edit-app-type-and-port.png)

1. `type` 선택. 힌트 줄에 해당 유형의 동작이 설명됩니다.
2. `port` 필드. `web` 앱에서는 이 포트가 응답하는지에 따라 실행 중 여부가 결정됩니다.

## 실행 중 여부를 판단하는 방식

Moonpool은 몇 초마다 확인합니다. 유형과 관계없이 다음 중 하나라도 해당하면 앱은 실행 중입니다.

- `processName`이 설정되어 있고 해당 이름의 프로세스가 있습니다. Moonpool 자체의 `<exe> mcp`
  도우미 프로세스는 계산하지 않습니다.
- `port`가 설정되어 있고 localhost에서 응답합니다.
- Moonpool이 실행했고, `port`와 `processName`이 모두 없으며, 터미널의 프로세스가 아직 살아 있습니다.

따라서 `cli` 앱은 명령이 실행되는 동안 실행 중이며, `port`가 없는 `web` 앱도 같은 방식으로 동작합니다.
`url`만 있는 `static` 항목은 추적할 것이 없으므로 실행 중으로 표시되지 않습니다.

## web

로컬 서버입니다. 서버가 응답하는지에 따라 실행 중으로 표시되도록 `port`를 설정하고, 서버가 구동되면
열도록 `url`과 `openBrowser`를 설정합니다.

## desktop

네이티브 앱입니다. 창이 시작한 명령에서 분리되어도 실행 중 표시가 유지되도록 `processName`을 실행
파일 이름으로 설정하세요. 기본 중지는 해당 이름의 모든 프로세스를 종료합니다.

## static

페이지입니다. `url`만 있으면 실행과 다시 시작은 브라우저에서 페이지를 열고, 중지는 아무것도 하지
않습니다. `http://`, `https://`, `mailto:`, `file://` URL이 열리므로 로컬 페이지도 사용할 수 있습니다.

```json title="apps.json"
{ "id": "csv", "name": "CSV dashboard", "group": "Docs", "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html" }
```

서버가 필요한 페이지(PHP나 로컬 파일을 가져오는 모든 것)에는 서버를 시작하는 `command`와 이를 추적할
`port`가 필요합니다. [예제](/ko/apps/examples/)를 참고하세요.

## cli

도구입니다. `command`는 `cwd`의 터미널 탭에서 실행되며, 명령이 끝나면 앱은 더 이상 실행 중이 아닙니다.
셸을 열어 두려면 명령을 셸로 지정하세요. 예를 들어 다음 `command`와 같습니다.

```text title="command"
pwsh -NoLogo -NoProfile -NoExit -Command python run.py --flag
```

`command`에서는 중첩된 큰따옴표를 피하세요. `cmd /c` 래퍼가 따옴표를 망가뜨립니다.

![PowerShell 명령의 출력과 그 아래의 열린 프롬프트를 보여 주는 cli 앱의 터미널 탭](../../../../assets/screenshots/terminal-cli-output.png)

## 클릭하면 일어나는 일

앱 이름을 클릭하면 터미널 탭만 열립니다. 앱을 실행하려면 실행, 중지, 다시 시작 컨트롤을 사용하세요.
[앱 상태](/ko/support/glossary/#앱-상태)를 참고하세요.
