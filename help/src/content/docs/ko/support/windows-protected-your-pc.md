---
title: "Windows protected your PC: Moonpool 설치 프로그램 실행하기(SmartScreen)"
description: "moonpool.exe를 실행할 때 Windows SmartScreen이 Windows protected your PC를 표시하는 이유와, More info 후 Run anyway를 선택하는 방법입니다."
---

내려받은 `moonpool.exe`를 실행하면 Windows가 **Windows protected your PC**라는 제목의 파란 상자를 표시할 수 있으며, "Microsoft Defender SmartScreen prevented an unrecognized app from starting. Running this app might put your PC at risk."라는 문구가 함께 나옵니다. 한국어 Windows에서는 같은 내용이 한국어로 표시됩니다. 이 페이지에서는 영어 문구를 그대로 인용하고 괄호 안에 한국어 풀이를 덧붙였습니다.

## 나타나는 이유

SmartScreen은 새 프로그램이나 많은 PC에서 실행된 적이 없는 프로그램에 대해 경고합니다. `moonpool.exe`에는 코드 서명이 없어 Windows가 신뢰할 게시자를 알 수 없으므로 처음 실행할 때 경고를 표시할 수 있습니다. 이는 평판 확인이며, 파일이 악성이라는 판정이 아닙니다.

## 해야 할 일

1. 상자에서 **More info**(추가 정보)를 클릭합니다. 게시자는 "Unknown publisher"(알 수 없는 게시자)로 표시됩니다.
2. **Run anyway**(실행)를 클릭합니다. 설치 카드가 열립니다. [설치](/ko/getting-started/install/)를 참고하세요.

먼저 신중하게 확인하고 싶다면, Moonpool의 공식 사이트나 GitHub 릴리스에서만 내려받고 파일 이름이 `moonpool.exe`인지 확인하세요.

## Run anyway 버튼이 없다면

일부 관리되는 PC에서는 관리자가 이 옵션을 꺼 두어 **Run anyway**가 보이지 않습니다. 관리자에게 문의하거나 직접 관리하는 PC를 사용하세요. 내려받은 zip에 들어 있던 파일은 차단 표시가 붙어 있을 수도 있습니다. 파일을 마우스 오른쪽 버튼으로 클릭해 **Properties**(속성)를 선택하고, **Unblock**(차단 해제)이 보이면 체크한 다음 **OK**(확인)를 누르고 다시 실행하세요.

## 백신 경고

자신을 사용자 프로필에 복사하고 업데이트할 때 스스로 교체하는 서명되지 않은 새 exe는 백신 소프트웨어에서도 경고를 일으킬 수 있습니다. 사용 중인 백신이 `moonpool.exe`를 차단하거나 격리하면 `.moonpool` 폴더에 대해 허용하세요. [Windows](/ko/platforms/windows/#실행하기-전에)를 참고하세요.

## 관련 항목

- [설치](/ko/getting-started/install/)
- [Windows](/ko/platforms/windows/)
- [설치 프로그램에 오류가 표시됩니다](/ko/support/troubleshooting/#설치-프로그램에-오류가-표시됩니다)
