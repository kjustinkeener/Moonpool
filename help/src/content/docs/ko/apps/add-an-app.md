---
title: "Moonpool에 앱 또는 개발 서버 추가하기"
description: "실행 명령, 작업 폴더, 환경 변수와 함께 로컬 앱이나 개발 서버를 등록해 Moonpool이 대신 시작, 중지하고 감시하도록 설정합니다."
---

Moonpool의 각 앱은 실행 명령, 작업 폴더, 선택적인 환경 변수로 이루어진 항목 하나입니다.
Moonpool은 명령을 자체 관리 터미널에서 실행합니다.

## 앱 추가

1. 사이드바 상단의 **...** 메뉴를 열고 **앱 추가**를 선택합니다.
2. **name**을 입력하고 **group**을 고릅니다.
3. **type**을 고릅니다. `web`(포트에서 대기하는 서버), `desktop`(네이티브 앱), `static`(페이지), `cli`(명령) 중 하나입니다.
4. **command**와 명령을 실행할 **cwd**를 설정합니다.
5. 유형에 필요한 항목을 채웁니다. web은 **port**와 **url**, desktop은 **processName**,
   static은 **url**입니다. `url`만 있는 `static` 앱에는 **command**나 **cwd**가 필요하지 않습니다.
6. 저장합니다. 앱이 사이드바에 나타납니다. 앱의 **실행** 컨트롤로 시작하세요.

![앱 편집기의 type 선택(1)과 port 필드(2). 그 사이에 cwd와 command가 있습니다](../../../../assets/screenshots/edit-app-type-and-port.png)

1. **type** 선택. 힌트에 해당 유형의 실행 방식이 설명됩니다.
2. **port** 필드. `web` 앱에서 사용합니다.

결과는 `apps.json`의 항목 하나이며, 예를 들면 다음과 같습니다.

```json title="apps.json"
{
  "id": "my-api",
  "name": "My API",
  "group": "Dev",
  "type": "web",
  "command": "npm run dev",
  "cwd": "C:\\code\\my-api",
  "port": 3000,
  "url": "http://localhost:3000"
}
```

앱 이름을 클릭하면 터미널 탭만 열립니다. [앱 상태](/ko/support/glossary/#앱-상태)를 참고하세요.

## 앱 편집기

- **Group.** 목록에서 그룹을 고르거나 **+ 새 그룹...**을 선택해 이름을 입력합니다.
  **목록으로 돌아가기**를 누르면 목록으로 돌아갑니다. 비워 둔 그룹은 `Apps`로 저장됩니다.
- **흐리게 표시된 필드**는 선택한 유형에서 사용하지 않습니다. 그래도 저장은 됩니다.
- **이름 없이 저장**하면 `name은 필수입니다.`가 표시됩니다.
- 저장하지 않은 변경 사항이 있을 때 **Esc**를 누르거나 편집기를 닫으면 "변경 사항을 취소할까요?"라고 묻습니다.
- 나중에 앱을 바꾸려면 행의 연필을 사용하거나, 행을 오른쪽 클릭해서 **편집**을 선택합니다.

## 직접 편집

같은 메뉴에서 **apps.json 편집**을 선택하고, 파일을 저장한 다음 **새로 고침**을 선택합니다. 형식,
유효성 검사 규칙, 복구 방법은 [설정 개요](/ko/apps/apps-json/)에 있습니다.

## 다음으로 볼 내용

- [앱 필드](/ko/apps/fields/): 모든 키와 그 역할.
- [앱 유형](/ko/apps/types/): 유형별 실행 방식과 실행 중 표시 방식.
- [중지와 다시 시작](/ko/apps/stop-and-restart/): 중지해도 무언가 계속 실행될 때 설정할 내용과 Docker 앱에 주의가 필요한 이유.
- [경로와 환경 변수](/ko/apps/paths-and-environment/): `{MP_HOME}`, `./` 경로, `env`.
- [예제](/ko/apps/examples/): 복사해서 쓸 수 있는 완성된 항목.
- [사용 가이드](/ko/guides/run-npm-dev-server-in-background-windows/): 백그라운드 개발 서버, Python 스크립트, 포트.
- [포터블 모드](/ko/data/portable-mode/)
- [업데이트](/ko/data/updating/)
