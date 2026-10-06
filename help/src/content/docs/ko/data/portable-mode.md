---
title: "USB 메모리나 동기화 폴더에서 Moonpool 실행하기"
description: "Moonpool과 모든 데이터를 이동 가능한 폴더 하나에 보관해 USB 메모리에 들고 다니거나 동기화하고, 여러 복사본을 나란히 실행하는 방법입니다."
---

포터블 모드는 Moonpool과 Moonpool이 기록하는 모든 것을 `.moonpool\` 폴더 하나에 보관하므로, USB 메모리에 담아 다니거나 동기화 폴더에 두고 어느 PC에서든 실행할 수 있습니다.

## 동작 방식

포터블로 설치하면 Moonpool은 선택한 위치 안에 `.moonpool\` 폴더를 만듭니다. 이 폴더에는 프로그램, 설정, 도움말 콘텐츠가 들어 있습니다. Windows AppData에는 아무것도 기록되지 않으므로, 폴더를 옮기거나 복사하면 설정 전체가 함께 이동합니다.

```text
<chosen location>\.moonpool\
```

## 설치형과 다른 점

| | 설치형 | 포터블 |
| --- | --- | --- |
| 프로그램 | `%USERPROFILE%\.moonpool\moonpool.exe` | `<chosen location>\.moonpool\moonpool.exe` |
| 설정 폴더 | `%USERPROFILE%\.moonpool\moonpool-config\` | `<chosen location>\.moonpool\moonpool-config\` |
| 창 브라우저 프로필, 창 크기와 위치 | 설정 폴더 안 | 설정 폴더 안이므로 함께 이동 |
| 시작 메뉴, 바탕 화면 바로 가기, 프로그램 추가/제거 항목 | 있음 | 없음 |
| 업데이트 | 자체 exe를 교체 | 동일하며 `.moonpool\` 폴더 안에서 수행. [업데이트](/ko/data/updating/#포터블-복사본)를 참고하세요. |
| 제거 | 프로그램 추가/제거 또는 `--uninstall` | 폴더 삭제 |

어느 모드도 Windows AppData에 기록하지 않습니다.

### 동기화 폴더

포터블 복사본을 동기화 폴더(OneDrive, Dropbox 등)에 둘 수는 있지만, 한 번에 한 PC에서만 실행하세요. Moonpool은 몇 초마다 `state.json`을 쓰고 앱이 실행되는 동안 로그를 기록하므로, 두 PC가 같은 폴더를 실행하면 같은 파일을 두고 충돌하며, 동기화 충돌로 `apps.json`이 손상될 수 있습니다. 다른 PC에서 시작하기 전에 한쪽 PC에서 종료하세요.

## 여러 복사본을 동시에

폴더당 하나의 Moonpool이 실행됩니다. 설치된 Moonpool과 각자의 폴더에 있는 여러 개의 포터블 복사본을 동시에 실행할 수 있으며, 각각은 자체 앱, 트레이 아이콘, 창, 설정, 로그, [제어 채널](/ko/automation/control-verbs/)을 가진 완전히 별개의 복사본입니다.

- 트레이 툴팁과 작업 표시줄 이름으로 어느 복사본인지 알 수 있습니다. 설치된 복사본은 `Moonpool`, 포터블 복사본은 `Moonpool (<folder>)`이며, `<folder>`는 선택한 폴더(`.moonpool\`이 들어 있는 폴더)입니다.
- 같은 복사본을 한 번 더 시작하면 새 창을 여는 대신 기존 창을 다시 보여 줍니다. 다른 복사본을 시작하면 그 복사본이 열립니다.
- AI 에이전트에 복사본을 둘 이상 제공하려면 각각을 고유한 이름으로 등록하세요. [MCP 설정](/ko/automation/mcp-setup/#moonpool이-둘-이상일-때)을 참고하세요.
- 포터블 폴더를 옮기거나 이름을 바꾸면 새로운 식별자(새 제어 채널 이름)를 갖게 됩니다. 옮기기 전에 종료하세요.
- 복사본끼리는 서로의 앱을 알지 못합니다. 두 복사본이 같은 포트에서 같은 서버를 시작하면 여전히 충돌하며, 프로세스 이름이나 포트로 동작하는 중지는 다른 복사본이 시작한 것을 종료시킬 수 있습니다. [중지와 다시 시작](/ko/apps/stop-and-restart/#여러-moonpool-또는-직접-실행한-프로세스)을 참고하세요.

## 앱도 함께 이동하게 하기

앱의 경로에 `{MP_HOME}` 토큰을 사용하면, 한 컴퓨터의 고정된 위치가 아니라 포터블 폴더 안을 가리킵니다. 포터블 복사본에서 `{MP_HOME}`은 `moonpool.exe`가 들어 있는 폴더, 즉 선택한 폴더가 아니라 `.moonpool\` 폴더 자체입니다.

```json title="apps.json"
{ "cwd": "{MP_HOME}/my-app" }
```

여기서 `{MP_HOME}/my-app`은 `<chosen location>\.moonpool\my-app`입니다. `./`로 시작하는 경로도 같은 방식으로 기준이 정해집니다. 토큰과 `./` 경로는 설치된 Moonpool에서도 동작합니다. 경로가 해석되는 방식은 [경로와 환경](/ko/apps/paths-and-environment/)을 참고하세요.

## 설치 프로그램에서 포터블 선택하기

포터블 모드는 설치 카드에서 설정하며, 설치 카드에는 **Moonpool 설치** 옆에 **포터블로 설치**가 있습니다.

![설치 카드: 포터블로 설치 링크가 주 버튼인 Moonpool 설치 아래에 있음](../../../../assets/screenshots/installer-window.png)

폴더를 선택하면 Moonpool이 그곳에 `.moonpool\` 폴더를 만들고 자신을 복사한 뒤, 새 복사본을 새 설정으로 시작합니다.

이 카드는 설치형과 포터블 모드 모두에서 "..." 메뉴의 **Moonpool 설치…**로도 열 수 있습니다. 여기서 **포터블로 설치**를 사용하면 실행 중인 Moonpool이 종료되고 그 자리에서 새 포터블 복사본이 시작됩니다. 처음 시작한 Moonpool은 그 자리에 그대로 남으므로 나중에 다시 시작할 수 있습니다.

포터블 복사본은 새로 시작하며 기존 앱을 복사하지 않습니다. 앱을 옮기려면 포터블 복사본을 종료하고 `apps.json`을 직접 복사하세요.

| | 경로 |
| --- | --- |
| 원본(설치형) | `%USERPROFILE%\.moonpool\moonpool-config\apps.json` |
| 대상(포터블) | `<chosen location>\.moonpool\moonpool-config\apps.json` |

절대 경로를 사용하는 항목은 같은 PC에서는 계속 동작하지만 함께 이동하지 않습니다. 앱 편집 대화상자는 이런 항목에 "이동 불가"를 표시합니다.

## Moonpool이 포터블임을 아는 방법

`moonpool.exe` 옆에 `moonpool.portable`이라는 파일이 있는 동안 그 복사본은 포터블입니다. 다른 표시는 없으며 Windows에 등록되는 것도 없습니다.

포터블 복사본을 제거하려면 종료한 뒤 `.moonpool\` 폴더를 삭제하세요. `--uninstall`은 설치된 Moonpool만 제거하며 포터블 복사본은 제거하지 않습니다.
