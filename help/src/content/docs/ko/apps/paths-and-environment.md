---
title: "앱에서 경로, MP_HOME 토큰, 환경 변수 사용하기"
description: "앱 항목에서 {MP_HOME}, {MP_DATA} 토큰과 상대 경로 ./ 를 사용하고, 어떤 필드가 이를 확장하는지, env와 작업 폴더는 어떻게 설정하는지 알아봅니다."
---

## 토큰

| 토큰 | 확장되는 값 |
| --- | --- |
| `{MP_HOME}` | 포터블: `moonpool.exe`가 있는 폴더(`.moonpool\` 폴더). Windows 설치형: `%USERPROFILE%\.moonpool`. Linux: `$XDG_CONFIG_HOME/Moonpool`, 없으면 `~/.config/Moonpool`이며 `{MP_DATA}`와 같은 폴더입니다. |
| `{MP_DATA}` | `apps.json`이 있는 설정 폴더. |

해석할 수 없는 토큰은 적힌 그대로 남습니다.

## 확장되는 필드

| 필드 | 토큰 | 앞의 `./` 또는 `.\` |
| --- | --- | --- |
| `cwd` | 예 | 예, `{MP_HOME}`을 기준으로 함 |
| `command` | 예 | 아니요 |
| `stopCommand` | 예 | 아니요 (`cwd`에서 실행되며 `cwd`는 기준이 정해져 있음) |
| `url` | 예 | 아니요 |
| `icon` | 예 | 예, `{MP_HOME}`을 기준으로 함 |
| `env` 값, `processName`, `note` | 아니요 | 아니요 |

`./` 없는 상대 경로(예: `apps\tool`)는 그대로 두며 Moonpool 자체의 작업 폴더를 기준으로 해석되는데,
원하는 동작인 경우는 드뭅니다. `./`나 토큰을 사용하는 것이 좋습니다.

```text
./apps/notes                       anchored to {MP_HOME}
{MP_HOME}\apps\notes\notes.exe     token
{MP_DATA}\dumps                    token
apps\tool                          left alone, resolves against Moonpool's working folder
```

```json title="apps.json"
{ "id": "notes", "name": "Notes", "group": "Desktop apps", "type": "desktop",
  "cwd": "./apps/notes",
  "command": "{MP_HOME}\\apps\\notes\\notes.exe",
  "processName": "notes" }
```

두 형식 모두 포터블 폴더를 옮겨도 계속 동작합니다. `C:\tools\notes` 같은 고정 경로는 함께
이동하지 않습니다. 포터블 모드에서 앱 편집 대화 상자는 절대 경로인 `cwd`와 `url` 값에 "이동 불가"
배지를 표시합니다. [포터블 모드](/ko/data/portable-mode/)를 참고하세요.

## 환경 변수

`env`는 문자열 객체입니다. 대화 상자에서는 한 줄에 `KEY=VALUE` 하나씩 편집합니다. 각 줄을 첫 번째
`=`에서 나누고 양쪽을 다듬으며, `=`가 없는 줄은 무시합니다.

대화 상자에서는 다음과 같이 입력합니다.

```text
PORT=8091
NODE_ENV=development
```

`apps.json`에서는 항목의 `env` 키로 입력합니다.

```json title="apps.json (one entry)"
{ "id": "habits", "name": "Habits", "group": "Web apps", "type": "web", "command": "python app.py",
  "env": { "PORT": "8091", "NODE_ENV": "development" } }
```

- 실행된 명령은 Moonpool의 환경 변수에 `env`를 더한 값을 상속합니다. `env`의 항목이 우선합니다.
- `env`는 `stopCommand`에도 적용됩니다.
- 값은 적힌 그대로 사용됩니다. Moonpool은 `{MP_HOME}` 확장이나 `%VAR%` 확장을 하지 않습니다.
- Moonpool은 `WEBVIEW2_USER_DATA_FOLDER`를 통해 자체 WebView2가 비공개 프로필 폴더를 사용하도록
  지정합니다. 실행된 앱은 이를 상속하지 않습니다. Moonpool을 시작하기 전에 직접 이 변수를 설정해
  두었다면 그 값을 받고, 그렇지 않으면 설정되지 않은 상태입니다. `env` 항목으로 재정의할 수도 있습니다.

## 작업 폴더

명령과 `stopCommand`는 `cwd`에서 실행됩니다. `cwd`를 생략하면 명령은 Moonpool 자체의 작업 폴더에서
실행되므로, 상대 경로를 사용하는 것에는 `cwd`를 설정하세요.
