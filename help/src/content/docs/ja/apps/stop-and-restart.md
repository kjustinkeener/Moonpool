---
title: "開発サーバーと、そこから起動されたものすべてを停止する"
description: "killMode と stopCommand で、停止と再起動がアプリとその子プロセスをきれいに終了するようにします。種類ごとの既定値と Windows 上の Docker も説明します。"
---

停止は、常に最初に次のことを行います。Moonpool は、そのアプリのために起動したターミナルを、そこから起動されたものすべてを含めて終了します。多くのアプリでは、それだけで十分です。

そのターミナルより長く生き残るアプリもあります (デスクトップのウィンドウが、起動した開発サーバーから切り離される場合や、サーバーのサブプロセスがポートを握り続ける場合)。**`killMode`** は、そのあとに実行される追加の手順を 1 つ選びます。

| `killMode` | 停止時の追加の手順 | 読み取る項目 | 既定になる type |
| --- | --- | --- | --- |
| `processName` | その名前のプロセスをすべて強制終了します。Windows では子プロセスも終了します (`taskkill /IM <name>.exe /T /F`)。それ以外では `pkill -KILL -x <name>` で、大文字小文字を区別する名前の完全一致のみで、子プロセスは含まれません。 | `processName` | `desktop` |
| `port` | `port` で待ち受けているプロセスを強制終了します。 | `port` | `web` |
| `command` | `cwd` で `stopCommand` を実行し、その完了を待ちます。 | `stopCommand`、`cwd`、`env` | なし |
| `none` | 何もしません。 | なし | `static`、`cli` |

`killMode` を省略すると、そのアプリの type の既定値になります。停止しても何かが動き続ける場合にだけ設定してください。

![アプリを編集するダイアログの killMode の選択。「既定 (種類に応じる)」に設定され、ヒントの行に type ごとの既定の動作が一覧されている](../../../../assets/screenshots/edit-app-killmode.png)

1. `killMode` の選択。「既定 (種類に応じる)」は、キーを省略するのと同じです。

- モードが必要とするフィールドが空の場合 (たとえば `port` がない `port` モード)、追加の手順は省略されます。エラーにはなりません。
- `killMode` は `type` と独立しています。`port` は `cli` アプリでも、`processName` は `web` アプリでも使えます。
- 空文字列や認識されない値では、追加の処理は何も行われません。type の既定値には戻りません。

デスクトップアプリでは、`processName` モードは次と同等のことを行います。

```powershell frame="terminal"
taskkill /IM notes-app.exe /T /F
```

## 複数の Moonpool や、自分で起動したプロセス

`processName` と `port` は、プロセスを誰が起動したかを知りません。`processName` はその名前のプロセスをすべて終了し、`port` は、別の Moonpool のコピーが起動したもの (インストール版とポータブル版のコピーは独立して動作します。[ポータブルモード](/ja/data/portable-mode/#複数のコピーを同時に)を参照) や、自分で起動したものも含め、そのポートで待ち受けているものを終了します。これらのモードは、そのように衝突しないアプリにだけ使ってください。つまり、マシン上のほかのものが使わない名前やポートです。2 つのコピーが同じアプリを登録している場合や、手作業でも実行する場合は、`killMode` に `none` を指定するか、自分のインスタンスだけを止める `command` を指定してください。

## stopCommand

`killMode` が `command` のときだけ使われます。Windows では `cmd /c`、それ以外では `$SHELL -c` を通して、`cwd` で、`env` を加えて実行されます。`{MP_HOME}` と `{MP_DATA}` が使えます。Moonpool はこれが完了するのを待ってから次に進むので、再起動が、これの実行中に再び起動することはありません。終了コードは無視されます。60 秒たってもまだ実行中なら、Moonpool はそれと子プロセスを終了して、続行します。

## 再起動

再起動は、停止に続けて、同じ `command` を起動することです。Moonpool は、再起動の前に、古いインスタンスが停止と判定される (つまりポートが解放される) まで、最大 4 秒待ちます。`url` だけの `static` エントリーには、停止するものがありません。再起動は、単にページを再び開きます。

## Windows 上の Docker アプリ

`none` か、`docker compose stop app` のような実際の停止コマンドを指定した `command` を使ってください。`port` は使わないでください。

Docker Desktop は、すべてのコンテナーのポートを、共有される 1 つのバックグラウンドプロセスを通して公開します。Windows では、「そのポートで待ち受けているもの」はその共有プロセスなので、`port` モードは Docker Desktop を強制終了し、このアプリだけでなくすべてのコンテナーを止めてしまいます。念のため、Moonpool は、Docker Desktop のバックエンド、プロキシ、サービスのプロセス、`dockerd`、`vpnkit`、WSL ホストのプロセス、`svchost` などの中核的なシステムプロセスという、共有される Windows のプロセスの固定リストを、ポート経由では終了しません。ただし、これは適切なモードを選ぶことの代わりにはなりません。

`command` がすでにコンテナーを再作成する場合 (`docker compose up -d --build`) は、`none` が正解です。再起動は、それを再び実行するだけです。

[ポートを使っているプロセスを見つけて終了する](/ja/guides/find-and-kill-process-using-port-windows/)と、[EADDRINUSE と「Port 5173 is in use」を解決する](/ja/support/port-already-in-use/)も参照してください。

## 例

ときどき node プロセスがポートを握ったままにする開発サーバー (これは `web` の既定で、ここでは明示的に示しています)。

```json title="apps.json"
{ "id": "site", "name": "Site", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
  "killMode": "port" }
```

Docker Compose アプリ。

```json title="apps.json"
{ "id": "api", "name": "API", "group": "Web apps", "type": "web",
  "cwd": "C:\\code\\api", "command": "docker compose up -d --build", "port": 8080,
  "killMode": "command", "stopCommand": "docker compose stop app" }
```
