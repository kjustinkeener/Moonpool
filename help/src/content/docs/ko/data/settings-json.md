---
title: "settings.json 이해하기와 손상된 파일 복구"
description: "Moonpool settings.json의 구조, Moonpool이 대신 기록하는 키, 파일이 손상되었을 때 읽고 복구하는 방법을 설명합니다."
---

앱 전체 설정은 설정 폴더의 `settings.json`에 있습니다([설정이 있는 위치](/ko/apps/apps-json/#설정-위치) 참고). 설정은 모든 설정을 JSON 키와 기본값과 함께 보여 주는 [설정 창](/ko/using/settings/)에서 변경하세요. 로그와 보관 규칙은 [로그](/ko/data/logs/) 페이지에 있습니다.

## 구조

JSON 객체 하나입니다. 생략한 키는 기본값을 사용합니다.

```json title="settings.json"
{
  "closeToTray": true,
  "transparency": 20,
  "debugLogging": true,
  "cliLogging": true,
  "logRetentionMb": 25
}
```

| 키 | 기본값 |
| --- | --- |
| `closeToTray` | `false` |
| `minimizeToTray` | `true` |
| `showInTray` | `true` |
| `showInTaskbar` | `true` |
| `alwaysOnTop` | `false` |
| `transparency` | `0`(0에서 90) |
| `showStatusbar` | `true` |
| `showMcpProcesses` | `true` |
| `checkOnStartup` | `true` |
| `locale` | `"auto"` |
| `debugLogging` | `false` |
| `cliLogging` | `false` |
| `logRetentionMb` | `10`(최소 1) |

## 자동으로 기록되는 키

Moonpool은 UI 확대/축소 배율(`uiScale`, 0.5에서 3.0)과 확정된 언어(`localeResolved`)도 이 파일에 저장합니다. 둘 다 직접 설정할 필요는 없습니다. 테마는 여기에 없으며 웹뷰 저장소에 보관됩니다([테마, 언어, 투명도](/ko/using/themes-and-language/) 참고).

## 읽기와 복구

Moonpool은 시작할 때 파일을 읽습니다. 실행 중에 한 수정은 반영되지 않으므로 먼저 종료하세요.

파일 형식이 잘못되면 Moonpool은 기본값으로 시작하고 설정 변경을 거부합니다. 오류 메시지는 `Repair settings.json and restart Moonpool before changing settings`로 끝납니다. 파일을 고치거나, 삭제해서 모든 설정을 초기화한 다음 Moonpool을 다시 시작하세요.
