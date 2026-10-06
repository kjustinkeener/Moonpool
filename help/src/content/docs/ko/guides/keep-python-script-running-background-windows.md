---
title: "Windows에서 Python 스크립트를 백그라운드에서 계속 실행하기"
description: "Windows에서 오래 실행되는 Python 스크립트나 작은 웹 앱을 백그라운드로 실행하고, 출력을 확인하고, 깔끔하게 중지합니다. pythonw와 Moonpool 두 방법을 소개합니다."
---

콘솔 창에서 실행한 Python 스크립트는 그 창을 닫으면 멈춥니다. Windows에서 흔히 쓰는 해결책은
`pythonw.exe`(콘솔 창이 없는 같은 인터프리터이므로 출력이 어디로도 가지 않습니다),
분리된 상태로 실행하는 `Start-Process pythonw -ArgumentList worker.py`, 또는 로그인할 때나
일정 간격으로 실행해야 하는 작업을 위한 예약 작업입니다. 어느 방법이든 프로세스를 없애고 싶을 때
작업 관리자에서 직접 찾아야 합니다.

## Moonpool 방식

Moonpool은 명령을 전용 터미널 탭에서 실행하므로, 별도의 콘솔 창 없이도 출력과 중지 버튼을 사용할
수 있습니다. 중지할 때까지 계속 실행되는 스크립트에는 `cli` 앱을 사용하세요. `-u`를 지정하면
Python이 출력을 즉시 내보내므로 탭에 실시간으로 표시됩니다.

```json title="apps.json"
{
  "id": "worker",
  "name": "Queue worker",
  "group": "Scripts",
  "type": "cli",
  "cwd": "C:\\code\\worker",
  "command": ".venv\\Scripts\\python.exe -u worker.py"
}
```

앱을 실행하고 이름을 클릭해 출력을 확인하세요. `cli` 앱은 명령이 실행되는 동안 실행 중으로
표시되고, 스크립트가 종료되면 회색으로 바뀌며 탭에 `[프로세스가 종료됨]`이 남습니다. **중지**하면
스크립트와 그 스크립트가 시작한 모든 것이 종료됩니다. 가상 환경의 `python.exe`를 경로로 직접
지정하면 활성화 단계가 필요하지 않습니다.

스크립트가 HTTP를 제공한다면(Flask, FastAPI, `python -m http.server`) `web` 앱으로 만들어서 실행
상태가 포트를 따르도록 하세요.

```json title="apps.json"
{
  "id": "docs-api",
  "name": "Docs API",
  "group": "Scripts",
  "type": "web",
  "cwd": "C:\\code\\docs-api",
  "command": ".venv\\Scripts\\python.exe -u app.py",
  "port": 8091,
  "url": "http://127.0.0.1:8091",
  "env": { "PORT": "8091" }
}
```

## 제한 사항

- Moonpool을 계속 실행해 두어야 합니다. 기본적으로 창을 닫으면 Moonpool이 종료되고, Windows에서는
  종료할 때 Moonpool이 실행한 모든 앱이 중지됩니다. **닫을 때 트레이로 숨기기**를 켜면 창이 대신
  숨겨집니다. [트레이, 닫기, 최소화](/ko/using/tray-and-closing/)를 참고하세요.
- Moonpool은 중단된 스크립트를 다시 시작하지 않으며, Windows 로그인 시 스스로 시작하지도
  않습니다. [Windows 로그인 시 스크립트나 개발 서버 자동 시작](/ko/guides/start-app-at-windows-login/)을
  참고하세요.
- `command`에서는 중첩된 큰따옴표를 피하세요. `cmd /c` 래퍼가 따옴표를 망가뜨립니다.

## 함께 보기

- [앱 유형](/ko/apps/types/#cli): `cli` 앱과 `web` 앱을 추적하는 방식.
- [중지와 다시 시작](/ko/apps/stop-and-restart/)
- [예제](/ko/apps/examples/)
- [로그](/ko/data/logs/): 세션 출력이 보관되는 위치.
