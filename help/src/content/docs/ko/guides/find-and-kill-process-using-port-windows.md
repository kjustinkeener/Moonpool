---
title: "Windows에서 포트(3000, 5173, 8080)를 사용 중인 프로세스 찾아 종료하기"
description: "netstat 또는 PowerShell로 Windows에서 3000, 5173 포트를 쥐고 있는 프로세스를 찾아 taskkill로 종료하고, 앱을 중지할 때 Moonpool이 포트를 해제하게 합니다."
---

개발 서버가 포트가 이미 사용 중이라며 실패한다면 다른 무언가가 그 포트에서 대기하고 있는
것입니다. 명령 프롬프트에서 소유 프로세스 ID와 함께 대기 중인 항목을 나열한 뒤 종료합니다.

```text frame="terminal"
netstat -ano | findstr :3000
taskkill /PID 12345 /F
```

`LISTENING` 행의 마지막 열이 PID입니다(`findstr :3000`은 `:30001`도 일치시키므로 로컬 주소를
확인하세요). `tasklist /FI "PID eq 12345"`로 어떤 프로그램인지 확인할 수 있습니다. PowerShell에서는
같은 조회를 다음과 같이 합니다.

```powershell frame="terminal"
Get-NetTCPConnection -LocalPort 3000 -State Listen | Select-Object LocalPort, OwningProcess
Get-Process -Id 12345
Stop-Process -Id 12345 -Force
```

`taskkill`에 `/T`를 추가하면 해당 프로세스의 자식 프로세스도 함께 종료합니다. 다른 사용자나 시스템이
소유한 프로세스는 관리자 권한으로 실행한 창이 필요할 수 있습니다.

## Moonpool 방식

Moonpool로 실행하는 앱이라면 PID를 직접 조회할 필요가 없습니다. 앱에 `port`를 지정하면 중지할 때
포트가 해제됩니다. `web` 앱에서는 이것이 기본 `killMode`이며, 여기서는 명시적으로 적었습니다.

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "npm start",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

중지하면 먼저 Moonpool이 시작한 터미널을 종료하고, 그 뒤에도 `port`에서 계속 대기 중인 대상을
강제로 종료합니다. Windows에서는 위와 같은 조회(`Get-NetTCPConnection -LocalPort
<port> -State Listen`)를 한 다음 각 소유자에 대해 `taskkill /PID <pid> /T /F`를 실행합니다.

- 직접 시작하지 않은 무언가가 포트를 쥐고 있으면 Moonpool은 앱을 실행 중으로 표시하되
  "managed by Moonpool"로는 표시하지 않습니다. 그 앱에서 **중지**를 누르세요. `port` 단계는
  그래도 실행됩니다.
- Moonpool은 Docker Desktop 백엔드, `svchost`, WSL 호스트 등 공유되는 Windows 프로세스의 고정 목록은
  포트 기준으로 종료하기를 거부합니다. Docker 앱에는 `killMode`를 `command` 또는 `none`으로
  사용하고 `port`는 절대 사용하지 마세요.
  [Windows의 Docker 앱](/ko/apps/stop-and-restart/#windows의-docker-앱)을 참고하세요.
- 이 방법은 `apps.json`에 등록된 앱의 포트에만 동작합니다. 다른 포트에는 맨 위의 명령을
  사용하세요.
- `port` 모드는 직접 실행한 복사본을 포함해 대기 중인 모든 대상을 종료하므로, 컴퓨터의 다른 어떤
  것도 필요로 하지 않는 포트에만 사용하세요.

## 함께 보기

- [EADDRINUSE와 "Port 5173 is in use" 해결](/ko/support/port-already-in-use/)
- [중지와 다시 시작](/ko/apps/stop-and-restart/)
- [앱 필드](/ko/apps/fields/): `port`와 `killMode`.
- [두 앱이 같은 포트를 사용하는 경우](/ko/support/troubleshooting/#두-앱이-같은-포트를-사용합니다)
