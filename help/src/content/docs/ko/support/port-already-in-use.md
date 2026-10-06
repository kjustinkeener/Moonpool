---
title: "Error: listen EADDRINUSE 및 Vite의 Port 5173 is in use 해결하기"
description: "Node의 EADDRINUSE와 Vite의 Port 5173 is in use를 해결합니다. 포트를 쓰는 프로세스를 찾아 해제하고, port와 killMode 필드로 재발을 막는 방법입니다."
---

```text
Error: listen EADDRINUSE: address already in use :::3000
```

이 Node.js 오류는 다른 프로세스가 이미 3000번 포트에서 수신 중이라는 뜻입니다(`:::`는 "모든 주소"의 IPv6 표기이며 `127.0.0.1:3000`으로 보일 수도 있습니다). 이전에 시작해서 중지하지 않은 같은 서버의 복사본인 경우가 많습니다.

Vite는 같은 상황을 다르게 처리합니다. 기본적으로 다음과 같이 출력합니다.

```text
Port 5173 is in use, trying another one...
```

그리고 다음에 비어 있는 포트에서 시작하므로, 서버는 떠 있지만 예상한 곳에 있지 않습니다. `--strictPort`(또는 `server.strictPort: true`)를 쓰면 Vite는 대신 `Error: Port 5173 is already in use`와 함께 종료합니다.

## 직접 해결하기

1. 포트를 소유한 프로세스를 찾아 종료합니다. Windows에서는 다음과 같이 합니다.

   ```text frame="terminal"
   netstat -ano | findstr :3000
   taskkill /PID 12345 /F
   ```

   PowerShell 버전을 포함한 단계별 안내는 [포트를 사용 중인 프로세스 찾아서 종료하기](/ko/guides/find-and-kill-process-using-port-windows/)에 있습니다.
2. 또는 서버를 다른 포트에서 시작합니다. 많은 Node 서버는 `PORT=3001`, Vite는 `--port 5174`처럼 지정할 수 있습니다.

## Moonpool이 돕는 방법

Moonpool을 통해 서버를 실행한다면 해당 항목에 `port`를 설정하세요. 그러면 Moonpool은 다음과 같이 동작합니다.

- 해당 포트에 무언가 응답하는 동안 앱을 실행 중으로 표시합니다. 따라서 포트를 잡고 있는 남은 서버는 실행 중이지만 "Moonpool에서 관리됨"은 아닌 것으로 나타납니다.
- **중지**와 **다시 시작**할 때, `killMode`가 `port`(`web` 앱의 기본값)이면 `port`에서 아직 수신 중인 것을 종료하므로, 다음 실행에서 포트가 비어 있게 됩니다.
- 같은 `port`로 설정된 두 앱을 표시합니다.

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "node server.js",
  "port": 3000,
  "env": { "PORT": "3000" },
  "killMode": "port"
}
```

Moonpool은 실행하기 전에 포트를 확인하지 않습니다. 포트가 아직 사용 중이면 명령이 앱의 터미널 탭에 위의 오류를 출력합니다. **중지**(포트를 해제합니다)를 누른 뒤 다시 **실행**하세요.

Vite에서는 `--strictPort`를 전달하고 `port`를 요청한 포트와 같게 유지하세요.

```text title="command"
npm run dev -- --port 5173 --strictPort
```

이 옵션이 없으면 Vite가 5174로 옮겨 가는 동안 Moonpool은 계속 5173을 감시하여 상태 점이 채워지지 않습니다.

`killMode`의 `port`는 해당 포트의 모든 프로세스를 종료하므로, 다른 곳에서 필요하지 않은 포트에만 사용하세요. Windows의 Docker 앱에서는 절대 사용하지 마세요. [중지와 다시 시작](/ko/apps/stop-and-restart/#windows의-docker-앱)을 참고하세요.

## 관련 항목

- [앱 필드](/ko/apps/fields/): `port`, `killMode`.
- [중지와 다시 시작](/ko/apps/stop-and-restart/)
- [문제 해결](/ko/support/troubleshooting/#두-앱이-같은-포트를-사용합니다)
- [Windows에서 npm 개발 서버를 백그라운드로 실행하기](/ko/guides/run-npm-dev-server-in-background-windows/)
