---
title: "Moonpool에 포함된 예제 대시보드 사용해 보기"
description: "프로그램에 포함된 오프라인 예제 대시보드를 열고, 저장 위치와 예제 앱이 이를 참조하는 방식, 기존 설정에 추가하는 방법을 알아봅니다."
---

Moonpool은 프로그램 안에 독립적으로 동작하는 대시보드 모음을 포함하고 있습니다. 서버도 CDN도
없이 완전히 오프라인으로 실행됩니다.

| 대시보드 | 설명 |
| --- | --- |
| CSV explorer | CSV 또는 TSV 파일을 끌어다 놓으면 열을 분석하고 데이터를 표시합니다. |
| JSON explorer | JSON(배열, 중첩 객체, 맵)을 끌어다 놓습니다. |
| Excel explorer | `.xlsx` 또는 `.xls` 파일을 끌어다 놓으면 오프라인으로 파싱합니다. |
| Moonpool Docs | 오프라인 Markdown 문서 브라우저입니다. |

## 저장 위치

시작할 때 Moonpool은 대시보드를 `{MP_HOME}\dashboards\examples`에 기록합니다.

| 모드 | 폴더 |
| --- | --- |
| 설치형 (Windows) | `%USERPROFILE%\.moonpool\dashboards\examples` |
| 포터블 | `<your .moonpool folder, the one holding moonpool.exe>\dashboards\examples` |
| Linux | `~/.config/Moonpool/dashboards/examples` (또는 `$XDG_CONFIG_HOME/Moonpool/dashboards/examples`) |

`examples` 폴더는 Moonpool이 관리합니다. Moonpool이 업데이트될 때마다 교체되므로 이곳에서 한
수정은 사라집니다. 대시보드를 수정하려면 해당 폴더와 공용 `_lib` 폴더를 `dashboards`로 복사한 뒤
앱이 그 복사본을 가리키도록 설정하세요. Moonpool은 `dashboards`의 다른 내용은 절대 변경하지
않습니다.

0.3.16 이전 버전은 예제를 `dashboards`에 바로 기록했습니다. 그 복사본은 그대로 남아 있고 더 이상
업데이트를 받지 않으며, 이를 가리키는 앱은 계속 동작합니다. 업데이트된 버전을 사용하려면
`url`을 아래의 `dashboards/examples/...` 경로로 바꾸세요.

## 앱이 참조하는 방식

각 항목은 `static` 앱이며, `url`은 `{MP_HOME}`을 기준으로 한 `file:///` URL입니다.

```text
file:///{MP_HOME}/dashboards/examples/csv/index.html
```

`{MP_HOME}`은 설치 폴더로, 포터블 모드에서는 번들 폴더로 해석되므로 번들을 옮겨도 항목이 계속
동작합니다. `file://` URL은 허용됩니다. [경로와 환경 변수](/ko/apps/paths-and-environment/)를
참고하세요.

## 예제 앱은 처음 실행할 때만 나타납니다

예제 항목은 설정 파일이 아직 없을 때만 `apps.json`에 기록됩니다. 이미 `apps.json`이 있다면
대시보드 항목을 직접 추가하세요("..." 메뉴에서 **apps.json 편집**을 선택한 뒤 **새로 고침**). 아래
네 항목을 최상위 배열 안에, 다른 항목과 쉼표로 구분해서 추가합니다.

```jsonc title="apps.json (excerpt)"
{
  "id": "csv-explorer",
  "name": "Sample CSV Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html",
  "openBrowser": true
},
{
  "id": "json-explorer",
  "name": "Sample JSON Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/json/index.html",
  "openBrowser": true
},
{
  "id": "xlsx-explorer",
  "name": "Sample Excel Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/xlsx/index.html",
  "openBrowser": true
},
{
  "id": "docs-browser",
  "name": "Moonpool Docs",
  "group": "Docs",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/docs/index.html",
  "openBrowser": true
}
```

필드의 의미는 [앱 필드](/ko/apps/fields/)에 있습니다.

## 함께 보기

- [예제](/ko/apps/examples/): 복사해서 쓸 수 있는 더 완성된 항목입니다.
- [앱 유형](/ko/apps/types/#static)
