---
title: "Windows에서 터미널 창 없이 npm 개발 서버를 백그라운드에서 실행하기"
description: "콘솔 창을 지키지 않고도 Windows에서 npm run dev, Vite 등 개발 서버를 계속 실행하고, 트레이에서 시작, 중지하고 출력을 확인합니다."
---

`npm run dev`로 시작한 개발 서버는 실행한 터미널에서 동작하므로 그 창을 닫으면 종료됩니다. 서버를
유지하는 평범한 Windows 방식은 숨겨진 프로세스를 쓰는 것으로, 예를 들어 PowerShell에서
`Start-Process npm.cmd -ArgumentList "run","dev" -WindowStyle Hidden`을 실행합니다. 하지만 그러면
읽을 출력이 없고, 중지하려면 알맞은 `node.exe`를 직접 찾아야 합니다
([포트를 사용 중인 프로세스 찾기 및 종료](/ko/guides/find-and-kill-process-using-port-windows/) 참고).

## Moonpool 방식

Moonpool은 명령을 허브 창 안의 내장 터미널 탭에서 실행하므로, 열어 둘 별도의 콘솔 창이 없습니다.
허브를 트레이로 숨겨도 서버는 계속 실행됩니다. 앱을 한 번만 추가하면 됩니다.

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173",
  "openBrowser": true
}
```

앱의 **실행** 컨트롤을 클릭합니다. `port`가 응답하면 상태 점이 단색이 되고, `openBrowser` 설정에
따라 브라우저가 `url`로 열립니다. 앱 이름을 클릭하면 전용 탭에서 출력을 읽을 수 있습니다. **중지**하면
터미널과 터미널이 시작한 모든 것이 종료되고 포트가 해제됩니다(`web`의 기본값은 `killMode`
`port`입니다).

## 창을 닫아도 계속 실행하기

기본적으로 닫기 버튼은 Moonpool을 종료하며, Windows에서는 종료할 때 Moonpool이 실행한 모든 앱이
중지됩니다. [설정](/ko/using/settings/)에서 **닫을 때 트레이로 숨기기**를 켜면 창을 닫아도
숨겨지기만 합니다. 트레이 아이콘(또는 **Moonpool 표시**)으로 다시 불러올 수 있습니다. 자세한 내용은
[트레이, 닫기, 최소화](/ko/using/tray-and-closing/)에 있습니다.

## 포트를 예측 가능하게 유지하기

Moonpool은 `port`를 기준으로 실행 여부를 판단합니다. Vite는 지정한 포트가 사용 중이면 다음 빈
포트로 옮겨 가는데, 그러면 Moonpool이 엉뚱한 포트를 감시하게 됩니다. `--strictPort`를 지정하면
Vite가 대신 종료되므로, `port`를 그에 맞게 설정하세요.

```text title="command"
npm run dev -- --port 5173 --strictPort
```

포트가 이미 사용 중이라면
[EADDRINUSE와 "Port 5173 is in use" 해결](/ko/support/port-already-in-use/)을 참고하세요.

## 제한 사항

- Moonpool은 중단된 서버를 다시 시작하지 않습니다. 앱을 중지됨으로 표시하고 탭에
  `[프로세스가 종료됨]`이 출력됩니다.
- Moonpool은 Windows 로그인 시 스스로 시작하지 않습니다.
  [Windows 로그인 시 스크립트나 개발 서버 자동 시작](/ko/guides/start-app-at-windows-login/)을
  참고하세요.

## 함께 보기

- [앱 필드](/ko/apps/fields/): `port`, `openBrowser`, `killMode`.
- [앱 유형](/ko/apps/types/): `web`의 실행 여부를 판단하는 방식.
- [중지와 다시 시작](/ko/apps/stop-and-restart/)
- [예제](/ko/apps/examples/#웹-개발-서버)
