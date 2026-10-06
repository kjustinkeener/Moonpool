---
title: "Windows에서 Moonpool 사용하기"
description: "Windows는 Moonpool의 주 플랫폼입니다. 설치 방법과 Windows와 Linux의 차이를 정리한 표로 어떤 동작을 기대할 수 있는지 알아봅니다."
---

Windows는 Moonpool의 주 플랫폼입니다. [설치](/ko/getting-started/install/)의 설명대로 설치하세요.

## 실행하기 전에

- **SmartScreen.** `moonpool.exe`는 코드 서명이 되어 있지 않아 처음 실행할 때 Windows가 "Windows
  protected your PC"(PC 보호) 화면을 표시할 수 있습니다. **More info**(추가 정보)를 선택한 다음
  **Run anyway**(실행)를 선택하세요.
- **백신 프로그램.** 서명되지 않은 새 exe가 자기 자신을 복사하고 업데이트할 때 스스로를 교체하면
  백신 프로그램이 이를 의심할 수 있습니다. 사용 중인 백신 프로그램이 `moonpool.exe`를 차단하거나
  격리하면 `.moonpool` 폴더에 대해 허용하세요.
- **WebView2.** Moonpool의 창은 Windows 11과 최신 Windows 10에 포함된 Microsoft Edge WebView2를
  사용합니다. 창이 빈 채로 남아 있거나 열리지 않으면 Microsoft의 Evergreen WebView2 런타임을
  설치하세요.

자세한 내용: [Windows가 PC를 보호했습니다](/ko/support/windows-protected-your-pc/),
[WebView2 런타임 없음](/ko/support/webview2-runtime-missing/),
[Windows 로그인 시 스크립트나 개발 서버 자동 시작](/ko/guides/start-app-at-windows-login/).

## 트레이

Windows 11에서는 새 트레이 아이콘이 숨겨진 아이콘 영역으로 들어가는 경우가 많습니다. 작업 표시줄
오른쪽의 **^** 화살표를 클릭해서 찾고, 작업 표시줄로 끌어다 놓으면 계속 보입니다.

## 명령

- 명령은 `cmd /c`를 거쳐 실행됩니다. `command`에서는 중첩된 큰따옴표를 피하세요. `cmd /c`가 따옴표를
  망가뜨립니다. 스크립트 실행 후 셸을 열어 두려면
  `pwsh -NoLogo -NoProfile -NoExit -Command <script and args>`를 사용하고, 스크립트 부분에는
  따옴표를 쓰지 마세요.
- `processName`은 `.exe`가 있든 없든, 대소문자를 구분하지 않고 일치합니다.
- 중지하면 Moonpool이 시작한 전체 프로세스 트리가, 그 트리에서 분리된 프로세스를 포함해 종료됩니다.
- Docker Desktop 앱에는 `killMode` `none` 또는 `command`가 필요하며 `port`는 절대 사용하면 안 됩니다.
  [Windows의 Docker 앱](/ko/apps/stop-and-restart/#windows의-docker-앱)을 참고하세요.

## 플랫폼별 차이

| | Windows | Linux |
| --- | --- | --- |
| 설치 | 스스로 설치하는 `moonpool.exe` 또는 포터블 | AppImage, `.deb`, RPM. 설치 카드 없음 |
| 자체 업데이트 | 예, 설치형과 포터블 모두 | AppImage만 |
| 설정 폴더 | `%USERPROFILE%\.moonpool\moonpool-config\` | `~/.config/Moonpool/` |
| 명령용 셸 | `cmd /c` | `$SHELL -c` |
| `processName` | 길이 제한 없음, `.exe` 선택, 대소문자 구분 안 함 | 15자 이하, 대소문자 정확히 일치 |
| `processName`으로 중지 | 프로세스와 자식 프로세스를 종료 | 정확히 그 이름의 프로세스만 종료 |
| 제어 채널 | 명명된 파이프 | Unix 소켓 |
| 창 스크린샷(테스트용) | 예 | 아니요 |
| 프로그램 파일에서 아이콘 추출 | 예 | 아니요 |
| 트레이 | 별도 설정 없이 동작 | AppIndicator 필요. 기본 GNOME은 확장이 필요 |

Linux에 관한 자세한 내용은 [Linux](/ko/platforms/linux/) 페이지에 있습니다.
