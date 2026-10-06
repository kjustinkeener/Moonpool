---
title: "Linux에 Moonpool 설치하고 사용하기"
description: "Linux에 Moonpool을 설치하고, GNOME 트레이 문제를 해결하고, 업데이트 방식과 Windows 버전과 다른 기능을 확인합니다."
---

Moonpool은 WebKitGTK를 통해 Linux에서 실행됩니다. 주로 Windows에서 개발되므로 Linux는 지원되지만
검증은 덜 되어 있습니다. Linux에는 설치 카드나 포터블 모드 선택 기능이 없으며, "..." 메뉴에도
**Moonpool 설치…** 항목이 없습니다.

## 설치

프로젝트의 릴리스 페이지에서 패키지를 다운로드하세요.

| 패키지 | 업데이트 |
| --- | --- |
| AppImage | Moonpool이 스스로 업데이트 |
| `.deb` | 패키지 관리자 |
| RPM (배포판의 RPM 도구로 설치) | 패키지 관리자 |

```bash title="AppImage" frame="terminal"
chmod +x Moonpool_*.AppImage
./Moonpool_*.AppImage
```

```bash title=".deb" frame="terminal"
sudo apt install ./Moonpool_*_amd64.deb
```

```bash title="RPM" frame="terminal"
sudo dnf install ./Moonpool-*.x86_64.rpm
```

`.deb`는 런타임 의존성을 함께 설치합니다. AppImage에는 WebKitGTK와 AppIndicator 라이브러리가 있어야
하며, 예를 들어 Debian이나 Ubuntu에서는 다음과 같이 설치합니다.

```bash frame="terminal"
sudo apt-get install -y libwebkit2gtk-4.1-0 libayatana-appindicator3-1
```

Fedora나 Arch에서는 해당하는 패키지를 사용하세요.

```bash title="Fedora" frame="terminal"
sudo dnf install webkit2gtk4.1 libayatana-appindicator-gtk3
```

```bash title="Arch" frame="terminal"
sudo pacman -S webkit2gtk-4.1 libayatana-appindicator
```

## GNOME의 트레이

기본 GNOME은 트레이 아이콘을 표시하지 않으므로, AppIndicator 확장을 설치하고 활성화하기 전에는
Moonpool의 트레이 아이콘이 나타나지 않습니다.

```bash frame="terminal"
sudo apt-get install -y gnome-shell-extension-appindicator
gnome-extensions enable ubuntu-appindicators@ubuntu.com
```

그런 다음 로그아웃했다가 다시 로그인하세요. 허브 창과 내장 터미널은 이 확장 없이도 동작합니다. KDE,
Cinnamon, XFCE, MATE는 별도 설정 없이 트레이를 표시합니다.

## 업데이트

AppImage만 스스로 업데이트됩니다. GitHub 릴리스의 `linux-update.json`을 읽고, minisign 서명을 검증하고,
AppImage 파일을 그 자리에서 교체하므로 쓰기 가능한 폴더에 두세요. `.deb`와 RPM으로 설치한 경우 Moonpool이
덮어쓰지 않습니다. 업데이트 확인은 새 버전을 알릴 수 있지만 Moonpool에서 설치를 시도하면 패키지 관리자를
사용하라는 메시지와 함께 실패합니다. [업데이트](/ko/data/updating/)를 참고하세요.

## 설정 위치

```text
~/.config/Moonpool/              (or $XDG_CONFIG_HOME/Moonpool/)
~/.config/Moonpool/apps.json
~/.config/Moonpool/dashboards/examples/   (example dashboards)
```

`apps.json`은 처음 실행할 때 예제로 채워집니다. [설정 개요](/ko/apps/apps-json/)를 참고하세요.

## Windows와의 차이점

- 실행 명령은 `$SHELL -c <command>`를 거쳐 실행되므로(`SHELL`이 설정되지 않았으면 `/bin/sh`), 사용하는 셸이 이해하는 문법을 쓰세요.
- 중지는 프로세스 그룹을 종료한 다음 `killMode`로 고른 추가 정리를 수행합니다. `killMode: "port"`로 포트를 해제할 때는 `lsof`를 사용하며 없으면 `fuser`로 되돌아갑니다. 배포판에 `lsof`가 없다면 설치하세요. [중지와 다시 시작](/ko/apps/stop-and-restart/)을 참고하세요.
- `desktop` 앱의 `processName`은 15자 이하여야 합니다. Linux는 프로세스 이름을 15자로 잘라내므로 더 긴 이름은 실행 중으로 감지되지 않으며 이름으로 중지할 수도 없습니다. `web` 앱은 포트로 판단하므로 영향을 받지 않습니다.
- 아이콘은 앱의 `src-tauri/icons/`, `public/favicon.*`, `icon.png` 또는 실시간 파비콘에서 찾습니다. 바이너리에서 아이콘을 추출하는 기능은 Windows 전용입니다.
- 열기 버튼은 파일을 선택하지 않고 해당 파일이 들어 있는 폴더를 엽니다.
- 설정 파일은 기본 텍스트 편집기에서 열립니다(`text/plain` 연결에서 확인합니다).
- Windows 설치 프로그램, 바로 가기, 앱 추가/제거 항목은 적용되지 않습니다.

## 함께 보기

- [Windows](/ko/platforms/windows/#플랫폼별-차이): 플랫폼별 차이를 정리한 표.
- [업데이트](/ko/data/updating/#linux)
