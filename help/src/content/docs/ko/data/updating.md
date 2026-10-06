---
title: "Moonpool 업데이트와 실패한 업데이트 해결"
description: "Moonpool이 업데이트를 확인, 다운로드, 적용하는 방식, 업데이트 배너의 동작, 포터블과 Linux 복사본의 업데이트, 실패 시 대처 방법을 안내합니다."
---

Moonpool은 스스로 업데이트합니다. 따로 내려받을 설치 프로그램도, 클릭해서 진행할 마법사도 없습니다.

## 업데이트가 도착하는 방식

Moonpool은 프로젝트의 GitHub Releases에서 `update.json`(Linux는 `linux-update.json`)을 가져와 버전을 비교하고, 엄격하게 더 새로운 버전만 제안합니다. 확인 시점은 다음과 같습니다.

- 시작할 때. 단, [설정](/ko/using/settings/)에서 **시작할 때 업데이트 확인**을 끈 경우는 제외합니다.
- 정보 창에서 **업데이트 확인**을 누를 때마다. 이 버튼은 더 새로운 버전이 있으면 바로 설치하고 Moonpool을 다시 시작합니다. 그렇지 않으면 최신 버전이라고 알리거나 오류를 표시합니다.

정보 창은 이름 아래에 실행 중인 버전을 보여 줍니다.

![정보 창의 위쪽: 로고, 이름(1), 그 아래의 버전 줄](../../../../assets/screenshots/about-header.png)

1. 이름입니다. 그 아래 줄은 버전과 빌드 날짜입니다.

모든 다운로드는 적용하기 전에 Moonpool의 minisign 서명 키로 검증되므로, 변조되었거나 손상된 다운로드는 거부됩니다. Moonpool은 이전 버전을 설치하지 않습니다.

## 업데이트 배너

시작할 때 발견된 업데이트는 허브의 빈 화면에 배너로 표시됩니다.

```text
Moonpool {version}을(를) 사용할 수 있습니다 (현재 버전 {current}).
```

배너는 앱 탭이 하나도 열려 있지 않고 CLI 창이 펼쳐져 있을 때만 표시됩니다. 창이 접혀 있으면 필터 상자 옆의 화살표가 대신 깜박입니다. 탭이 열려 있으면 아무 표시도 없습니다. 배너를 보려면 모든 탭을 닫거나(그리고 창을 펼치거나) 정보 창에서 **업데이트 확인**을 사용하세요.

**다운로드 후 설치**를 클릭하면 Moonpool이 스스로 교체하고 다시 실행됩니다. x를 눌러 배너를 닫을 수도 있습니다.

## 포터블 복사본

포터블 복사본은 자신의 `.moonpool\` 폴더에 있는 `moonpool.exe`를 같은 방식으로 업데이트합니다. 각 복사본은 따로 확인하고 업데이트합니다. 폴더에 쓸 수 있어야 하므로 읽기 전용 USB 메모리나 공유 폴더에 있는 복사본은 스스로 업데이트할 수 없습니다. 이 경우 더 새로운 `moonpool.exe`를 직접 덮어 복사하세요.

## Linux

AppImage만 스스로 업데이트합니다. AppImage 파일을 제자리에서 교체하므로 쓸 수 있는 폴더에 두세요. `.deb` 또는 RPM 설치는 패키지 관리자가 업데이트합니다. Moonpool에서 설치하면 다음 오류로 실패합니다.

```text
automatic updates are available for the AppImage only; update the .deb or RPM with your package manager
```

[Linux](/ko/platforms/linux/#업데이트)를 참고하세요.

## 업데이트가 실패할 때

배너에 이유가 표시되고 버튼이 다시 활성화되므로 재시도할 수 있습니다.

```text
업데이트 실패: <error>
```

| 오류에 포함된 문구 | 가능한 원인 | 대처 방법 |
| --- | --- | --- |
| `download failed` | 연결 없음, 프록시, 또는 GitHub의 요청 제한 | 잠시 기다렸다가 재시도하거나 직접 업데이트하세요. |
| `signature verification FAILED - refusing to install` | 다운로드가 손상되었거나 변경됨 | 재시도하세요. 계속 실패하면 Releases 페이지에서 직접 업데이트하세요. |
| `rename self aside` 또는 `write new exe` | 폴더가 읽기 전용이거나 백신이 파일을 잡고 있음 | 폴더에 쓸 수 있게 하거나, 백신에서 `moonpool.exe`를 허용한 뒤 재시도하세요. |
| `refusing to install ... not newer than current` | 제안된 버전이 더 새롭지 않음 | 할 일이 없습니다. |

### 직접 업데이트하기

Moonpool을 종료하고, 프로젝트의 [Releases 페이지](https://github.com/kjustinkeener/Moonpool/releases)에서 `moonpool.exe`를 내려받아 기존 파일 위에 복사하세요. 설치형은 `%USERPROFILE%\.moonpool\moonpool.exe`, 포터블 복사본은 `.moonpool\` 폴더 안의 파일입니다. 설정 폴더는 건드리지 않습니다. Linux에서는 AppImage를 교체하거나 패키지 관리자를 사용하세요.

## 도움말도 함께 업데이트

이 도움말은 Moonpool 안에 포함되어 있으므로, 프로그램을 업데이트할 때마다 일치하는 도움말도 함께 업데이트됩니다. 오프라인 사본은 항상 실행 중인 버전과 일치합니다.

## 관련 항목

- [새로운 기능](/ko/getting-started/whats-new/)
- [설정 창](/ko/using/settings/)
