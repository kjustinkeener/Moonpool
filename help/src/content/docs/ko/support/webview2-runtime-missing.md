---
title: "WebView2 런타임 없음: Windows에서 Moonpool 창이 비어 있거나 열리지 않을 때"
description: "Windows에서 Moonpool 창이 열리지 않거나 비어 있다면 Microsoft Edge WebView2 런타임이 없을 수 있습니다. 확인하고 설치하는 방법입니다."
---

Windows에서 Moonpool 창이 열리지 않거나, 열려도 비어 있다면 Microsoft Edge WebView2 런타임이 없는 것이 원인일 가능성이 높습니다. Moonpool은 Tauri 앱이며, 창은 WebView2가 그리는 웹 페이지입니다.

WebView2는 Windows 11과 최신 Windows 10에 포함되어 있으므로 대부분의 PC에는 이미 있습니다. 오래되었거나 기능이 제거된 Windows 10, 또는 삭제한 PC에서는 없을 수 있습니다. Moonpool의 소스에는 이 경우를 위한 전용 메시지가 없으므로 인용할 오류 문구가 없습니다. 증상은 창이 나타나지 않거나 비어 있는 것입니다.

## 설치되어 있는지 확인

PowerShell에서 레지스트리의 런타임 버전을 찾아보세요(첫 번째 경로는 시스템 전체 설치, 두 번째는 사용자별 설치입니다).

```powershell frame="terminal"
Get-ItemProperty "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
Get-ItemProperty "HKCU:\Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
```

`120.0.2210.91` 같은 버전 번호가 나오면 설치된 것입니다. 두 경로 모두 오류가 나오면 설치되지 않은 것입니다.

## 설치하기

Microsoft의 WebView2 페이지("WebView2 Runtime download"로 검색)에서 **Evergreen** WebView2 런타임을 내려받아 설치 프로그램을 실행한 뒤 Moonpool을 다시 시작하세요. Evergreen 런타임은 스스로 업데이트됩니다.

## 설치되어 있는데도 창이 비어 있다면

- 트레이에서 모든 Moonpool을 종료하거나(또는 작업 관리자에서 `moonpool.exe`를 끝내고) 다시 시작하세요.
- 가능하면 [설정](/ko/using/settings/)에서 **디버그 정보를 파일에 기록**을 켜고 `moonpool.log`를 확인하세요. [로그](/ko/data/logs/)를 참고하세요.
- 창이 열리지만 화면 밖에 있다면 [창 문제](/ko/support/troubleshooting/#창-문제)를 참고하세요.

## 관련 항목

- [Windows](/ko/platforms/windows/#실행하기-전에)
- [설치](/ko/getting-started/install/)
- [문제 해결](/ko/support/troubleshooting/)
