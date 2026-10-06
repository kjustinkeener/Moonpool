---
title: "AI 에이전트로 Moonpool 설정 및 제어하기: 빠른 시작"
description: "AI 에이전트나 스크립트로 Moonpool을 설정하고 제어하는 세 가지 방법과 에이전트에 맞는 선택 기준, 각 방법의 같은 동작 예시를 소개합니다."
---

방법은 세 가지입니다. 에이전트가 할 수 있는 일에 맞춰 고르세요.

| 원하는 것 | 사용할 방법 | 시작 위치 |
| --- | --- | --- |
| 에이전트가 앱을 찾아 한 번 추가하게 하기 | 허브의 빈 화면에 있는 **프롬프트 복사** | 아래 |
| 에이전트가 도구 호출로 앱을 시작, 중지하고 출력을 읽게 하기 | MCP 서버 `moonpool.exe mcp` | [MCP 설정](/ko/automation/mcp-setup/) |
| 스크립트 또는 MCP를 쓰지 않는 에이전트 | 명령줄 동사 | [명령줄](/ko/automation/command-line/) |

## 프롬프트 복사

열려 있는 탭이 없으면 CLI 창에 미리 만들어 둔 프롬프트("처음이신가요? 이 프롬프트를 AI 에이전트에게 전달해 앱을 설정해 보세요")가 표시됩니다. **프롬프트 복사**를 누르면 클립보드에 복사됩니다. 이를 에이전트에 붙여 넣으세요. 이 프롬프트는 에이전트가 설정 폴더의 `AI-README.md`와 `apps.json`을 참고하여 앱을 찾아 등록하도록 안내합니다. 작업이 끝나면 **새로 고침**을 선택하세요.

Moonpool은 실행할 때마다 `apps.json` 옆의 `AI-README.md`를 다시 작성하므로, 항상 사용 중인 버전과 일치합니다. 이 파일에 직접 수정한 내용을 보관하지 마세요.

## 같은 동작, 세 가지 방법

| 동작 | 명령줄 | 제어 채널 동사 | MCP 도구 |
| --- | --- | --- | --- |
| 앱 시작 | `moonpool.exe launch <id>` | `launch` | `moonpool_start_app` |
| 앱 중지 | `moonpool.exe stop <id>` | `stop` | `moonpool_stop_app` |
| 앱 다시 시작 | `moonpool.exe restart <id>` | `restart` | `moonpool_restart_app` |
| 앱 출력 읽기 | `moonpool.exe dump <id> [out-path]` | `dump` | `moonpool_app_output` |
| 앱 목록과 상태 보기 | `state.json` 읽기 | `list` | `moonpool_list_apps` |
| `apps.json` 다시 읽기 | `moonpool.exe reload` | `reload` | `moonpool_reload_config` |
| `apps.json` 읽기 | `moonpool.exe read-config` | `read-config` | `moonpool_read_config` |
| `apps.json` 교체 | `moonpool.exe write-config <file> [token]` | `write-config` | `moonpool_write_config` |
| `apps.json` 되돌리기 | `moonpool.exe restore-config [n]` | `restore-config` | `moonpool_restore_config` |
| 창 표시 | `moonpool.exe show` | `show` | `moonpool_raise_launcher` |
| Moonpool 시작 | `moonpool.exe` | 없음 | `moonpool_bootup_launcher` |
| Moonpool 종료 | `moonpool.exe quit` | `quit` | `moonpool_shutdown_launcher` |
| 사용 중인 폴더 표시 | `moonpool.exe paths` | `paths` | `moonpool_launcher_paths` |

명령줄은 아무것도 출력하지 않습니다. 결과는 `--ticket`으로 확인하세요([결과 확인하기](/ko/automation/command-line/#결과-확인하기) 참고). 제어 채널과 MCP는 바로 응답합니다.

## 에이전트의 도구가 실패할 때

- `Moonpool is not running - call moonpool_bootup_launcher first`: Moonpool을 시작하거나, 에이전트가 해당 도구를 호출하게 하세요.
- 수정이 "반영되지 않는" 경우: 에이전트에게 `moonpool_launcher_paths`를 요청하세요. 허브와 MCP의 폴더가 다르면 에이전트가 다른 `apps.json`을 읽고 있는 것입니다. [샌드박스 호스트](/ko/automation/mcp-setup/#샌드박스-호스트)를 참고하세요.
- Moonpool 복사본이 여러 개인 경우: 각각을 서로 다른 이름으로 등록하세요. [Moonpool이 둘 이상일 때](/ko/automation/mcp-setup/#moonpool이-둘-이상일-때)를 참고하세요.

Claude Code, Codex, Cursor의 실제 예시는 [AI 에이전트에게 로컬 앱을 시작하고 중지하는 MCP 서버 제공하기](/ko/guides/mcp-server-for-ai-agent-to-start-stop-local-apps/)에 있습니다.

더 많은 증상은 [문제 해결](/ko/support/troubleshooting/#mcp와-스크립트-오류)에서 확인하세요.
