---
title: "Windows 로그인 시 스크립트나 개발 서버 자동으로 시작하기"
description: "시작 프로그램 폴더의 바로 가기로 로그인 시 Moonpool을 시작하고, 작은 PowerShell 스크립트로 개발 서버나 스크립트를 실행합니다. 전용 설정은 없습니다."
---

Windows에서 로그인할 때 무언가를 시작하는 일반적인 방법은 두 가지입니다. 시작 프로그램 폴더의
바로 가기(Win+R을 누르고 `shell:startup`을 입력한 뒤 Enter), 또는 "로그온할 때" 트리거를 사용하는
작업 스케줄러 작업입니다. 어느 쪽이든 프로그램이나 스크립트를 실행하므로 개발 서버의 명령을 직접
지정할 수도 있지만, 그러면 그것을 추적하거나, 출력을 보여 주거나, 중지해 주는 것이 없습니다.

## Moonpool이 제공하는 것

Moonpool에는 로그인 시 시작 설정이 없으며, `apps.json`의 항목에도 Moonpool이 시작될 때 앱을
실행하는 필드가 없습니다(전체 목록은 [앱 필드](/ko/apps/fields/)와
[settings.json](/ko/data/settings-json/)에 있습니다). 할 수 있는 일은 로그인 시 Moonpool을 직접
시작한 다음, [명령줄](/ko/automation/command-line/)이 제공하는 것과 같은 동사로 스크립트가 원하는
앱을 실행하게 하는 것입니다.

먼저 평소처럼 앱을 등록합니다.

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

그런 다음 이 내용을 `start-moonpool-apps.ps1`로 저장합니다. 설치형이면 프로그램은
`%USERPROFILE%\.moonpool\moonpool.exe`이고, 포터블 복사본이라면 해당 복사본의 exe 경로를 사용하세요.

```powershell title="start-moonpool-apps.ps1"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
Start-Process $mp
Start-Sleep -Seconds 15
& $mp launch site
```

`launch`가 Moonpool에 전달되려면 Moonpool이 이미 실행 중이어야 합니다. 상주하는 것이 없는 상태에서
실행하면 같은 명령이 새 Moonpool을 시작하고 동사는 수행되지 않습니다. 대기 시간은 Moonpool이 시작할
시간을 주기 위한 것이므로 느린 컴퓨터에서는 늘리세요. 앱마다 `& $mp launch <id>` 줄을 하나씩
추가합니다.

마지막으로 스크립트의 바로 가기를 시작 프로그램 폴더에 넣고 대상을 다음과 같이 지정합니다.

```text title="Shortcut target"
powershell.exe -NoProfile -WindowStyle Hidden -File "C:\Users\you\start-moonpool-apps.ps1"
```

무슨 일이 일어났는지 확인하려면 동사에 `--ticket t1`을 추가하고 `state.json`에서 결과를 읽으세요
([결과 읽기](/ko/automation/command-line/#결과-확인하기)).

## 유의 사항

- 이 방식으로 시작한 개발 서버도 다른 앱처럼 Moonpool이 "관리"하므로 중지와 종료가 적용됩니다.
  같은 앱이 이미 실행 중이면(예를 들어 직접 시작한 경우) Moonpool은 이를 실행 중이지만 관리되지
  않는 것으로 표시합니다.
- Moonpool은 종료된 앱을 다시 실행하지 않으며, 마지막으로 종료했을 때 어떤 앱이 실행 중이었는지도
  기억하지 않습니다.

## 함께 보기

- [명령줄](/ko/automation/command-line/)
- [트레이, 닫기, 최소화](/ko/using/tray-and-closing/)
- [Windows에서 npm 개발 서버를 백그라운드에서 실행](/ko/guides/run-npm-dev-server-in-background-windows/)
