---
title: "Windows 또는 Linux에 Moonpool 설치하기"
description: "몇 번의 클릭으로 Moonpool을 설치하고, 설치형과 포터블 모드 중에서 선택하고, 나중에 설치 메뉴를 사용하고, 깔끔하게 제거하는 방법을 안내합니다."
---

이 페이지는 Windows용입니다. Windows에서 Moonpool은 자체 설치 프로그램이며, 다운로드 파일은
`moonpool.exe` 하나입니다. Linux에는 설치 카드나 포터블 선택 기능이 없습니다. [Linux](/ko/platforms/linux/)를
참고하세요.

## 설치형 모드

다운로드한 `moonpool.exe`를 실행합니다. 처음 실행하면 설치 카드가 표시됩니다. 설치 카드에는
컨트롤이 세 개 있습니다. **Moonpool 설치** 버튼, **바탕 화면에 바로 가기 만들기** 확인란(기본값은
켜짐), **포터블로 설치** 링크입니다.

설치하면 Moonpool이 사용자 프로필의 `.moonpool\` 아래로 복사되고, 시작 메뉴 바로 가기(확인란을
선택했다면 바탕 화면 바로 가기도)가 추가되며, 앱 추가/제거 항목이 등록됩니다. 그런 다음 설치된
복사본을 시작하고 설치 프로그램은 닫힙니다. 다운로드한 파일은 원래 위치에 남아 있으므로 삭제해도
됩니다. 이후에는 다른 앱처럼 바로 가기에서 Moonpool을 실행하면 됩니다.

![설치 카드: Moonpool 설치 버튼, 바탕 화면 바로 가기 확인란, 포터블로 설치 링크, 설치 경로](../../../../assets/screenshots/installer-window.png)

Moonpool에 필요한 모든 것은 이 폴더 하나 아래에 있습니다. 프로그램, 설정, 함께 제공되는 도움말입니다.

```text title="Installed layout"
%USERPROFILE%\.moonpool\
```

## 메뉴에서 Moonpool 설치...

Windows에서는 두 모드 모두 "..." 메뉴에 **Moonpool 설치…**가 있습니다. 같은 설치 카드가 열립니다.
포터블 복사본에서는 정식으로 설치할 수 있습니다. 설치된 복사본에서는 **Moonpool 설치**가 비활성화되고
("설치됨") **포터블로 설치**는 계속 사용할 수 있습니다.

## 제거

Windows의 앱 추가/제거(설치된 앱)를 사용하거나, 설치된 복사본을 `--uninstall`로 실행합니다.
PATH에 등록되어 있지 않으므로 전체 경로를 지정하세요.

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" --uninstall
```

이렇게 하면 시작 메뉴와 바탕 화면 바로 가기, 레지스트리 항목, `%USERPROFILE%\.moonpool` 폴더 전체가
삭제되며, **설정도 함께 삭제됩니다**(`apps.json`, 설정, 로그). 설정을 보관하려면 먼저 이 폴더를
백업하세요.

```text
%USERPROFILE%\.moonpool\moonpool-config
```

실행 중인 Moonpool은 제거 과정에서 중지됩니다.

## 포터블 모드

USB 메모리나 옮길 수 있는 폴더를 선호하시나요? 설치 카드에서 **포터블로 설치**를 클릭하고 폴더를
선택하세요. [포터블 모드](/ko/data/portable-mode/)를 참고하세요.

## 다음 단계

- [Windows가 PC를 보호했습니다](/ko/support/windows-protected-your-pc/): SmartScreen이 설치 프로그램을 차단하는 경우.
- [WebView2 런타임 없음](/ko/support/webview2-runtime-missing/): 창이 빈 채로 남아 있는 경우.
- [첫 번째 앱](/ko/getting-started/first-app/)
