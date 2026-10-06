---
title: "개발 서버와 그것이 시작한 모든 것 중지하기"
description: "killMode와 stopCommand로 중지와 다시 시작이 앱과 자식 프로세스를 깔끔하게 끝내도록 설정합니다. 유형별 기본값과 Windows의 Docker도 다룹니다."
---

중지는 항상 먼저 이렇게 동작합니다. Moonpool이 앱을 위해 시작한 터미널을, 그 터미널이 실행한
모든 것과 함께 종료합니다. 많은 앱에서는 이것만으로 충분합니다.

일부 앱은 그 터미널보다 오래 살아남습니다(데스크톱 창이 자신을 실행한 개발 서버에서 분리되거나,
서버의 하위 프로세스가 계속 포트를 쥐고 있는 경우). **`killMode`**는 그 뒤에 실행되는 추가 단계를
하나 고릅니다.

| `killMode` | 중지할 때의 추가 단계 | 읽는 값 | 기본값인 유형 |
| --- | --- | --- | --- |
| `processName` | 해당 이름의 모든 프로세스를 강제 종료합니다. Windows에서는 자식 프로세스도 함께 종료합니다(`taskkill /IM <name>.exe /T /F`). 그 밖에서는 `pkill -KILL -x <name>`으로, 대소문자를 구분하는 정확한 이름 일치이며 자식 프로세스는 포함되지 않습니다. | `processName` | `desktop` |
| `port` | `port`에서 대기 중인 프로세스를 강제 종료합니다. | `port` | `web` |
| `command` | `cwd`에서 `stopCommand`를 실행하고 끝나기를 기다립니다. | `stopCommand`, `cwd`, `env` | 없음 |
| `none` | 아무것도 하지 않습니다. | 없음 | `static`, `cli` |

`killMode`를 생략하면 앱 유형의 기본값이 적용됩니다. 중지한 뒤에도 무언가 계속 실행될 때만
설정하세요.

![앱 편집 대화 상자의 killMode 선택이 "기본값 (유형별)"로 설정되어 있고, 힌트 줄에 유형별 기본 동작이 나열되어 있습니다](../../../../assets/screenshots/edit-app-killmode.png)

1. `killMode` 선택. "기본값 (유형별)"은 키를 생략한 것과 같습니다.

- 모드에 필요한 필드가 비어 있으면(예를 들어 `port` 없이 `port` 모드) 추가 단계는 건너뜁니다.
  오류가 아닙니다.
- `killMode`는 `type`과 독립적입니다. `port`는 `cli` 앱에서도, `processName`은 `web` 앱에서도 동작합니다.
- 빈 문자열이나 인식할 수 없는 값은 추가 동작을 하지 않습니다. 유형 기본값으로 되돌아가지
  않습니다.

데스크톱 앱에서 `processName` 모드는 다음과 같은 동작을 실행합니다.

```powershell frame="terminal"
taskkill /IM notes-app.exe /T /F
```

## 여러 Moonpool, 또는 직접 실행한 프로세스

`processName`과 `port`는 누가 프로세스를 시작했는지 알지 못합니다. `processName`은 해당 이름의 모든
프로세스를 종료하고, `port`는 포트에서 대기 중인 모든 대상을 종료하는데, 여기에는 다른 Moonpool
복사본이 시작한 것(설치된 복사본과 포터블 복사본은 독립적으로 실행됩니다.
[포터블 모드](/ko/data/portable-mode/#여러-복사본을-동시에) 참고)과 직접 시작한 것도 포함됩니다.
이 모드는 그런 충돌이 없는 앱에만 사용하세요. 컴퓨터의 다른 어떤 것도 사용하지 않는 이름이나
포트여야 합니다. 두 복사본이 같은 앱을 등록하거나 직접 실행하기도 한다면 `killMode`를 `none`으로
하거나 자기 인스턴스만 중지하는 `command`를 지정하세요.

## stopCommand

`killMode`가 `command`일 때만 사용됩니다. Windows에서는 `cmd /c`, 그 밖에서는 `$SHELL -c`를 거쳐
`cwd`에서 `env`를 더해 실행됩니다. `{MP_HOME}`과 `{MP_DATA}`를 사용할 수 있습니다. Moonpool은
다른 작업을 하기 전에 이 명령이 끝나기를 기다리므로, 다시 시작이 명령이 아직 실행 중일 때 앱을
다시 실행하는 일은 없습니다. 종료 코드는 무시됩니다. 60초가 지나도 실행 중이면 Moonpool이 해당
명령과 자식 프로세스를 종료하고 계속 진행합니다.

## 다시 시작

다시 시작은 중지한 뒤 같은 `command`를 실행하는 것입니다. Moonpool은 이전 인스턴스가 중지된 것으로
읽힐 때까지(포트가 해제되도록) 최대 4초를 기다렸다가 다시 실행합니다. `url`만 있는 `static` 항목은
중지할 것이 없으므로 다시 시작하면 페이지가 다시 열릴 뿐입니다.

## Windows의 Docker 앱

`none`을 사용하거나, `docker compose stop app` 같은 실제 중지 명령과 함께 `command`를 사용하세요.
`port`는 사용하지 마세요.

Docker Desktop은 모든 컨테이너의 포트를 하나의 공유 백그라운드 프로세스를 통해 게시합니다. Windows에서
"포트에서 대기 중인 대상"은 그 공유 프로세스이므로, `port` 모드는 Docker Desktop을 강제 종료해 이
앱뿐 아니라 모든 컨테이너를 중단시킵니다. 안전장치로 Moonpool은 공유되는 Windows 프로세스의 고정
목록(Docker Desktop의 백엔드, 프록시, 서비스 프로세스, `dockerd`, `vpnkit`, WSL 호스트 프로세스,
`svchost` 같은 핵심 시스템 프로세스)을 포트 기준으로 종료하기를 거부합니다. 하지만 이것이 올바른
모드를 고르는 일을 대신하지는 않습니다.

`command`가 이미 컨테이너를 다시 만든다면(`docker compose up -d --build`) `none`이 맞습니다. 다시
시작하면 그냥 다시 실행됩니다.

[포트를 사용 중인 프로세스 찾기 및 종료](/ko/guides/find-and-kill-process-using-port-windows/)와
[EADDRINUSE와 "Port 5173 is in use" 해결](/ko/support/port-already-in-use/)도 참고하세요.

## 예제

가끔 node 프로세스가 포트를 쥔 채 남는 개발 서버입니다(`web`의 기본값이며 여기서는 명시적으로
적었습니다).

```json title="apps.json"
{ "id": "site", "name": "Site", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
  "killMode": "port" }
```

Docker Compose 앱:

```json title="apps.json"
{ "id": "api", "name": "API", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\api", "command": "docker compose up -d --build", "port": 8080,
  "killMode": "command", "stopCommand": "docker compose stop app" }
```
