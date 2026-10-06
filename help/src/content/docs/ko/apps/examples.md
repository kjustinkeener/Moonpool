---
title: "자주 쓰는 앱 설정을 위한 apps.json 복사용 예제"
description: "개발 서버, 데스크톱 앱, 정적 페이지, CLI 도구, Docker Compose, 포터블 앱을 위한 완전하고 유효한 apps.json 항목을 복사해 수정해서 사용하세요."
---

각 스니펫은 항목 하나입니다. `apps.json`의 최상위 배열 안에 쉼표로 구분해서 넣으세요. id, 이름, 경로는
자신의 환경에 맞게 바꾸세요.

## 웹 개발 서버

포트 5173이 응답하는 동안 실행 중으로 표시됩니다. 응답하면 브라우저가 열립니다. 중지하면 포트도
해제되며, 이는 `web`의 기본값입니다.

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

## 환경 변수에서 포트를 읽는 웹 앱

```json title="apps.json"
{
  "id": "habit-tracker",
  "name": "Habit Tracker",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\habits",
  "command": "python app.py",
  "port": 8091,
  "url": "http://127.0.0.1:8091",
  "openBrowser": true,
  "env": { "PORT": "8091" },
  "note": "Moved off 8000 to avoid a clash"
}
```

## 데스크톱 앱

`notes-app`이라는 이름의 프로세스가 있는 동안 실행 중으로 표시됩니다. 중지하면 해당 프로세스를
이름으로 종료합니다.

```json title="apps.json"
{
  "id": "notes-app",
  "name": "Notes App",
  "group": "Desktop apps",
  "type": "desktop",
  "cwd": "C:\\code\\notes-app",
  "command": "npm run tauri dev",
  "processName": "notes-app"
}
```

## 이미 호스팅된 정적 페이지

터미널이 없습니다. 실행하면 페이지가 열립니다.

```json title="apps.json"
{
  "id": "team-board",
  "name": "Team board",
  "group": "Docs",
  "type": "static",
  "url": "https://example.com/board"
}
```

## 명령으로 제공하는 정적 폴더

Moonpool이 터미널에서 서버를 실행하고, 포트로 추적하며, 서버가 응답하면 페이지를 엽니다.

```json title="apps.json"
{
  "id": "docs-site",
  "name": "Docs",
  "group": "Docs",
  "type": "static",
  "cwd": "C:\\code\\docs\\public",
  "command": "python -m http.server 8090",
  "port": 8090,
  "url": "http://localhost:8090",
  "openBrowser": true,
  "killMode": "port"
}
```

## CLI 도구

터미널 탭에서 실행됩니다. `-NoExit`는 스크립트가 끝난 뒤에도 셸을 열어 둡니다.

```json title="apps.json"
{
  "id": "backup",
  "name": "Backup script",
  "group": "CLI tools",
  "type": "cli",
  "cwd": "C:\\code\\scripts",
  "command": "pwsh -NoLogo -NoProfile -NoExit -Command .\\backup.ps1 -Verbose"
}
```

## Docker Compose

`command`가 이미 컨테이너를 다시 만들고 종료하므로, 실행 중 상태는 포트에서 얻습니다. 중지하면 포트
소유자를 종료하는 대신 `stopCommand`를 실행합니다. Windows에서 포트 소유자는 Docker Desktop이기
때문입니다. [중지와 다시 시작](/ko/apps/stop-and-restart/#windows의-docker-앱)을 참고하세요.

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "docker compose up -d --build",
  "port": 8080,
  "url": "http://localhost:8080",
  "killMode": "command",
  "stopCommand": "docker compose stop app"
}
```

앱을 중지할 때 컨테이너를 계속 실행해 두고 싶다면 `"killMode": "none"`을 사용하고
`stopCommand`는 삭제하세요.

## 포터블 앱

경로가 포터블 폴더를 기준으로 하므로, 폴더를 옮겨도 항목이 계속 동작합니다.

```json title="apps.json"
{
  "id": "notes",
  "name": "Notes",
  "group": "Desktop apps",
  "type": "desktop",
  "cwd": "./apps/notes",
  "command": "{MP_HOME}\\apps\\notes\\notes.exe",
  "processName": "notes",
  "icon": "{MP_HOME}\\icons\\notes.png"
}
```

## 함께 보기

- [예제 대시보드](/ko/getting-started/example-dashboards/): Moonpool에 포함된 대시보드.
- [앱 필드](/ko/apps/fields/)
- [Windows에서 npm 개발 서버를 백그라운드에서 실행](/ko/guides/run-npm-dev-server-in-background-windows/)
- [Windows에서 Python 스크립트를 백그라운드에서 계속 실행](/ko/guides/keep-python-script-running-background-windows/)
