---
title: "apps.json의 모든 필드: 형식, 기본값, 역할"
description: "앱 편집 대화 상자의 이름과 같은 이름으로, apps.json 항목의 모든 키를 형식, 기본값, 사용하는 앱 유형과 함께 찾아볼 수 있습니다."
---

앱 편집 대화 상자는 같은 필드를 같은 이름으로 보여 줍니다. 선택한 유형에 해당하지 않는 필드는
대화 상자에서 흐리게 표시되지만 그래도 저장되며, 한 가지 예외가 있습니다. `stopCommand`는
`killMode`가 `command`일 때만 저장됩니다.

![name부터 stopCommand까지의 앱 편집 대화 상자. killMode 선택이 강조 표시되어 있고 processName, stopCommand 같은 사용하지 않는 필드는 흐리게 표시됩니다](../../../../assets/screenshots/edit-app-dialog.png)

1. `killMode` 선택. 사용하지 않는 필드는 계속 흐리게 표시됩니다.

| 필드 | 형식 | 필수 | 사용하는 유형 | 역할 |
| --- | --- | --- | --- | --- |
| `id` | 문자열 | 예 | 전체 | 고유 키. 문자, 숫자, `.`, `_`, `-`이며 `-`로 시작할 수 없습니다. [개요](/ko/apps/apps-json/#id)를 참고하세요. |
| `name` | 문자열 | 예 | 전체 | 사이드바에 표시되는 레이블. 공백일 수 없습니다. |
| `group` | 문자열 | 예 | 전체 | 앱이 속하는 사이드바 제목. 직접 편집할 때는 공백일 수 없으며, 대화 상자는 비워 둔 그룹을 `Apps`로 저장합니다. 어떤 텍스트든 가능하며 새 이름은 새 그룹을 만듭니다. |
| `type` | 문자열 | 예 | 전체 | `web`, `desktop`, `static`, `cli` 중 하나. [앱 유형](/ko/apps/types/)을 참고하세요. |
| `command` | 문자열 | `static` 제외 필수 | 전체 | 앱을 시작하기 위해 터미널에서 실행하는 명령. Windows에서는 `cmd /c`, 그 밖에서는 `$SHELL -c`를 거칩니다(`SHELL`이 설정되지 않았으면 `/bin/sh`). `static`에서는 선택 사항입니다. |
| `cwd` | 문자열 | 아니요 | `command`가 있는 모든 유형 | 명령을 실행할 폴더. 기본값은 Moonpool 자체의 작업 폴더입니다. 토큰과 `./`를 지원합니다. [경로와 환경 변수](/ko/apps/paths-and-environment/)를 참고하세요. |
| `port` | 정수, 1~65535 | 아니요 | 전체 | localhost(IPv4 또는 IPv6)의 이 포트에서 응답이 있는 동안 실행 중으로 표시합니다. `killMode` `port`가 읽습니다. |
| `processName` | 문자열 | 아니요 | 전체, 주로 `desktop` | 이 이름의 프로세스가 있는 동안 실행 중으로 표시합니다. 대소문자를 구분하지 않으며 `.exe`는 있어도 없어도 되므로 `my-app`은 `my-app.exe`와 일치합니다. Linux에서는 15자 이하여야 합니다. `killMode` `processName`이 읽습니다. |
| `mcpProcessName` | 문자열 | 아니요 | `processName`이 있는 모든 유형 | 이 앱의 MCP 서버 프로세스 이름에 대한 와일드카드 패턴. `*`는 임의의 문자열, `?`는 한 글자와 일치합니다. 대소문자를 구분하지 않고 이름 전체에 일치하며 `.exe`는 생략할 수 있습니다. 일치하는 프로세스는 앱의 MCP 서버(사이드바의 MCP 하위 행)로 간주되며, 첫 번째 인수가 `mcp`일 필요가 없습니다. [mcpProcessName](#mcpprocessname)을 참고하세요. |
| `url` | 문자열 | `static`만 필수 | `web`, `static` | 열 페이지. `http://`, `https://`, `mailto:`, `file://` URL만 열립니다. |
| `openBrowser` | 불리언, 기본값 `false` | 아니요 | `url`이 있는 모든 유형(대화 상자는 `desktop`과 `cli`에서 흐리게 표시) | Moonpool이 앱의 구동을 감지하면 `url`을 자동으로 엽니다(아래 참고). |
| `killMode` | 문자열 | 아니요 | 전체 | 중지와 다시 시작 시의 추가 정리: `processName`, `port`, `command`, `none`. [중지와 다시 시작](/ko/apps/stop-and-restart/)을 참고하세요. |
| `stopCommand` | 문자열 | 아니요 | `killMode` `command` | 중지할 때 실행하는 명령. 다른 모든 모드에서는 무시됩니다. |
| `env` | 문자열 객체 | 아니요 | 전체 | 추가 환경 변수. 대화 상자에서는 한 줄에 `KEY=VALUE` 하나씩 편집합니다. |
| `icon` | 문자열 | 아니요 | 전체 | 사이드바 이미지: 파일 경로, `http(s)` URL, 또는 `data:` URI. 앱의 컨텍스트 메뉴의 **아이콘 설정...**으로 설정하거나 직접 입력합니다. |
| `note` | 문자열 | 아니요 | 전체 | 사이드바에서 앱에 마우스를 올렸을 때 표시되는 툴팁. |

`env`와 `killMode`를 사용하는 항목은 다음과 같습니다.

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000", "NODE_ENV": "development" },
  "killMode": "port"
}
```

`port`와 `killMode`가 함께 동작하는 방식은
[포트를 사용 중인 프로세스 찾기 및 종료](/ko/guides/find-and-kill-process-using-port-windows/)를 참고하세요.

## mcpProcessName

기본적으로 Moonpool은 프로세스 이름이 `processName`과 일치하고 첫 번째 인수가 `mcp`일 때(예:
`notes-app.exe mcp`) 그 프로세스를 앱의 MCP 서버로 간주합니다. 서버가 다른 이름으로 실행될 때
`mcpProcessName`을 설정하세요. 예를 들어 앱이 exe 하나를 감시하는데 MCP 서버는 다른 것(`mog.exe mcp`)인
경우, 또는 서버의 이름을 바꾼 복사본인 경우입니다.

값은 와일드카드 패턴입니다. `*`는 (없는 경우를 포함해) 임의의 문자열과, `?`는 정확히 한 글자와
일치합니다. 대소문자를 구분하지 않고 프로세스 이름 전체와 비교하며, `.exe`가 없는 패턴은 `.exe`가 붙은
이름과도 일치합니다. 빈 값은 설정하지 않은 것으로 취급합니다.

```json
{
  "id": "destiny",
  "name": "Destiny",
  "group": "Desktop apps",
  "type": "desktop",
  "processName": "destiny",
  "mcpProcessName": "destiny-mcp-*"
}
```

이 설정은 `destiny-mcp-2706210170.exe` 같은 이름을 바꾼 복사본과 일치합니다. `mcpProcessName`과 일치하는
프로세스는 `mcp`로 시작했는지와 관계없이 서버로 간주되며, 앱 자체가 실행 중인 것으로는 절대
간주되지 않습니다. 패턴이 `processName` 자체와도 일치하면(예: `destiny*`) Moonpool은 여전히 `mcp` 인수를
요구하므로 실제 앱이 MCP 서버로 오인되는 일은 없습니다.
[MCP 설정](/ko/automation/mcp-setup/#자체-mcp-서버가-있는-앱)을 참고하세요.

## openBrowser

Moonpool은 Moonpool이 실행한 앱이 처음 실행 중으로 읽힐 때 `url`을 한 번 엽니다. 이를 감지하려면
`port` 또는 `processName`이 필요합니다. 둘 다 없으면 실행 중이라는 것은 터미널 프로세스가 살아 있다는
뜻일 뿐이며 브라우저는 자동으로 열리지 않습니다. 명령이 직접 브라우저를 여는 경우에는 `openBrowser`를
끄세요. 명령이 없는 `static` 항목은 `openBrowser`와 관계없이 실행을 누를 때마다 `url`을 엽니다.

같은 `port`로 설정된 두 앱은 사이드바에 표시됩니다.

## 아이콘

앱의 아이콘은 다음 중 처음으로 존재하는 것입니다.

1. `icon` 필드.
2. 설정 폴더의 `icons\<id>.<ext>`, 예를 들어 `icons\site.png`.
3. 앱 자체 폴더(`cwd` 또는 `file:///` `url`의 폴더)에 있는 아이콘 파일.
4. `desktop`의 경우, 빌드되었거나 실행 중인 `.exe`의 아이콘.
5. `web`과 `static`의 경우, 서버가 구동된 뒤 사이트의 `/favicon.ico`.
6. 유형별 글리프.

대부분의 앱에는 아이콘 설정이 필요하지 않습니다.
