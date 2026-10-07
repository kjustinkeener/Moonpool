---
title: "Moonpool 릴리스 노트와 최근 변경 사항"
description: "최근 Moonpool 릴리스에서 달라진 점, 실행에 필요한 요구 사항, GitHub의 전체 릴리스 노트를 찾을 수 있는 위치를 확인합니다."
---

모든 릴리스의 전체 노트는 프로젝트의 [릴리스 페이지](https://github.com/kjustinkeener/Moonpool/releases)에
있습니다. 이 도움말은 Moonpool 안에 포함되어 있으므로 항상 실행 중인 버전을 설명합니다. Moonpool은
스스로 업데이트됩니다. [업데이트](/ko/data/updating/)를 참고하세요.

## 0.3.16

- **여러 Moonpool을 동시에 실행.** 설치된 Moonpool과 여러 포터블 복사본을 폴더당 하나씩 나란히
  실행할 수 있으며, 각각 자체 앱, 트레이 아이콘, 제어 채널을 가집니다.
  [포터블 모드](/ko/data/portable-mode/#여러-복사본을-동시에)를 참고하세요.
- **테마 브라우저.** 테마 68개를 각각의 색상으로 미리 볼 수 있습니다.
  [테마, 언어, 투명도](/ko/using/themes-and-language/)를 참고하세요.
- **바로 실행되는 예제.** 새로 만든 `apps.json`에는 모두 그대로 실행되는 예제 앱이 들어 있습니다.
  예제 대시보드는 이제 Moonpool과 함께 업데이트되는 앱 전용 `dashboards/examples` 폴더에 있습니다.
  [예제 대시보드](/ko/getting-started/example-dashboards/)를 참고하세요.
- **apps.json 오류 표시.** 사이드바 위의 배너에 오류가 표시되며, 새로 고침에 실패하면 마지막으로
  불러온 목록이 유지됩니다.
  [apps.json에 오류가 있을 때](/ko/using/hub-window/#appsjson에-오류가-있을-때)를 참고하세요.
- **Linux의 제어 채널.** Unix 소켓을 통해 사용하며 `list` 동사가 추가되었습니다.
  [제어 동사](/ko/automation/control-verbs/)를 참고하세요.
- 정보 창과 앱 편집기가 테마와 언어 변경을 실시간으로 따릅니다. **Moonpool 설치…** 메뉴
  항목은 Windows가 아닌 환경에서 숨겨집니다.

## 0.3.15

- 다시 시작한 앱은 이전 출력을 유지하며, 날짜가 표시된 "restarted" 구분선이 추가됩니다.
  [터미널 탭](/ko/using/terminal-tabs/#다시-시작)을 참고하세요.
- 앱마다 자체 `cli-output` 폴더가 있으므로 로그 정리가 다른 앱의 로그를 건드리지 않습니다.
- `killMode`와 `stopCommand`를 앱 편집기에서 설정할 수 있습니다.
  [중지와 다시 시작](/ko/apps/stop-and-restart/)을 참고하세요.
- 실행된 앱이 더 이상 Moonpool 자체의 WebView2 프로필을 상속하지 않습니다.

## 0.3.14

- 세션 로그를 세션 사이에도 보관할 수 있으며, 앱별 용량 상한이 있습니다.
  [로그](/ko/data/logs/)를 참고하세요.
- 설정에 로그 폴더용 열기 및 복사 버튼이 추가되었습니다.
- 도움말 창 제목 표시줄을 수정했습니다.

## 요구 사항

- WebView2가 설치된 Windows 10 또는 11 ([Windows](/ko/platforms/windows/) 참고).
- WebKitGTK 4.1과 AppIndicator 라이브러리가 있는 Linux ([Linux](/ko/platforms/linux/) 참고).
