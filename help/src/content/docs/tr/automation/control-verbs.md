---
title: "Moonpool denetim kanalı ve komutları başvurusu"
description: "Moonpool denetim kanalının (adlandırılmış kanal veya Unix soketi) nasıl çalıştığı, protokolü ve çalışan uygulamanın yanıtladığı her komut; bağımsız değişkenler ve yanıtlarla."
---

## Dinlediği yer

Her Moonpool kopyasının kendi kanalı vardır; bu yüzden kurulu Moonpool ve taşınabilir kopyalar, birbirleri
adına yanıt vermeden yan yana çalışabilir. Windows'ta kurulu Moonpool `\\.\pipe\moonpool` adlı kanalı
dinler. Taşınabilir bir kopya klasöründen türetilen bir kimlik ekler: `\\.\pipe\moonpool-<id>`.

`<id>`, kopyanın `moonpool-config` klasör yolundan türetilen 8 onaltılık rakamdır; bu yüzden o klasör için
yeniden başlatmalarda ve güncellemelerde aynı kalır, klasörü taşırsanız değişir. Bir kopyanın `moonpool.exe`
dosyası, `moonpool.exe mcp` dahil, her zaman kendi kopyasının kanalını bulur.

Linux'ta bunun yerine `0600` kipinde bir Unix etki alanı soketini dinler:

| Durum | Soket yolu |
| --- | --- |
| Normal | Değişken ayarlıysa `$XDG_RUNTIME_DIR/moonpool.sock`, değilse Moonpool'un yapılandırma klasöründeki `moonpool.sock` |
| Taşınabilir mod | Taşınabilir kopyanın yapılandırma klasöründeki `moonpool.sock`; böylece taşınabilir bir kopya kurulu olanla asla çakışmaz |
| Yol bir soket için çok uzun (yaklaşık 100 karakter) | Yalnızca sizin açabileceğiniz bir dizinde `/tmp/moonpool-<uid>/moonpool.sock` (taşınabilir kopya için `moonpool-<id>.sock`) |

Bir çökmeden geride kalan soket dosyası algılanır ve bir sonraki başlangıçta değiştirilir. Üzerinde hâlâ bir şeyin
yanıt verdiği soket asla devralınmaz. Dosya, Moonpool normal şekilde çıktığında kaldırılır.

Kanal aynı zamanda [MCP sunucusunun](/tr/automation/mcp-setup/) Moonpool'un çalışıp çalışmadığını öğrenme
yoludur: bir `ping` yanıtlanırsa çalışıyordur, kanal veya soket yoksa çalışmıyordur. Aynı komutlara,
aşağıdaki tanılama komutları dışında [komut satırından](/tr/automation/command-line/) da ulaşılabilir.

## Protokol

Girişte satır başına bir JSON nesnesi, çıkışta bir JSON satırı, sırayla. Bir bağlantı birçok istek taşıyabilir.

```json title="request"
{"cmd": "restart", "args": ["my-app"]}
```

```jsonl title="replies"
{"ok": true, "result": "..."}
{"ok": false, "error": "unknown app id: ..."}
```

PowerShell'den bir istek ve yanıtı:

Taşınabilir bir kopya için `moonpool` yerine kendi kanal adını (`paths` komutunun gösterdiği
`moonpool-<id>`) kullanın.

```powershell frame="terminal"
$p = New-Object System.IO.Pipes.NamedPipeClientStream('.', 'moonpool', 'InOut')
$p.Connect(2000)
$w = New-Object System.IO.StreamWriter($p); $w.AutoFlush = $true
$r = New-Object System.IO.StreamReader($p)
$w.WriteLine('{"cmd":"ping"}')
$r.ReadLine()
```

```json title="reply"
{"ok":true,"result":"pong"}
```

- `args` bir dize listesidir ve atlanabilir. Diğer alanlar yok sayılır.
- `result` bir dize ya da null'dır. Yapılandırılmış veri döndüren komutlar bunu bir JSON dizesi olarak döndürür.
- Geçerli JSON olmayan bir satır `{"ok": false, "error": "bad request: ..."}` yanıtını alır.
- Bilinmeyen bir `cmd`, `unknown cmd: <name>` yanıtını alır.
- Pencere üzerinden giden bir komut (`launch`, `stop`, `restart`, `reload`,
  `refresh-icons`, `help`, `open-window`) eylem bittiğinde ya da 45 sn sonra bir zaman aşımı hatasıyla
  yanıtlanır. Merkez penceresinin arayüzü yüklenmemişse `frontend not
  loaded` ile hemen başarısız olur.
- Önceki bir Moonpool hâlâ çıkarken başlayan bir Moonpool, kanalı bağlamayı yaklaşık 8 saniye yeniden dener.
  Yine de başaramazsa bunu günlüğe yazar ve kanal olmadan çalışmaya devam eder.

## Komutlar

| Komut | Bağımsız değişkenler | Sonuç |
| --- | --- | --- |
| `ping` | yok | `pong`. Yalnızca kanal. |
| `list` | yok | Çalışan merkezin belleğinden okunan `{"apps": [...], "statuses": [...]}` JSON dizesi; `state.json` ile aynı `apps` ve `statuses` biçimi. Uygulamalar kayıtlıyken ilk durum denetimi henüz çalışmamışsa `"statusNotReady": true` ekler. `apps.json` yüklenemezken `"manifestError": "<message>"` ekler (uygulamalar o zaman yüklenen son listedir) ve başlangıçtan beri hiçbir liste yüklenmediyse `"manifestLoaded": false` ekler. Yalnızca kanal. |
| `show` | yok | null. Pencereyi öne getirir. |
| `quit` | yok | null. Moonpool'dan çıkar. |
| `launch` | `<id>` | Başarıda null; yalnızca `url` içeren bir `static` girdi için `opened`. Hatalar: `unknown app id: <id>`, `did not reach running in time`. |
| `stop` | `<id>` | Başarıda null; yalnızca `url` içeren bir `static` girdi için `stopped`. Hata: `still running after stop`. |
| `restart` | `<id>` | `launch` ile aynı sonuçlar ve hatalar. |
| `reload` | yok | Başarıda null. |
| `refresh-icons` | yok | Başarıda null. |
| `help` | yok | null. Yardım penceresini açar. |
| `dump` | `<id>` [`out-path`] | Uygulamanın oturum günlüğünün yolu ya da `out-path` konumundaki düz metin kopyanın yolu. |
| `paths` | yok | Merkezin kullandığı klasörlerin ve exe'nin çok satırlı raporu. |
| `read-config` | yok | `token`, `valid`, `error`, `path`, `manifest_text` içeren `dumps\read-config.json` dosyasının yolu. |
| `write-config` | `<source-file>` [`token`] | Yeni sürüm belirteci. Hatalar: `stale token: ...`, `rejected invalid manifest: ...`, `cannot read source ...`. |
| `restore-config` | [`index` veya `filename`] | Bağımsız değişken yoksa: `dumps\restore-config.json` dosyasının yolu (`count`, `snapshots`). Bir bağımsız değişkenle: `restored <file> (<n> apps); new version token <token>`. |
| `argv` | komut satırı bağımsız değişkenleri | Hemen null. Bunları, bu kopyanın ikinci bir `moonpool.exe <args>` çalıştırması gibi tam olarak çalıştırır; `--ticket` dahil. İkinci başlatma, çıkmadan önce bağımsız değişkenlerini bu şekilde devreder. |

Örnek alışverişler:

```jsonl title="request, reply"
{"cmd": "launch", "args": ["nope"]}
{"ok": false, "error": "unknown app id: nope"}

{"cmd": "restore-config", "args": ["1"]}
{"ok": true, "result": "restored 1767225600000.json (6 apps); new version token <token>"}
```

`write-config` ve `restore-config` yeni manifesti hemen yükler, `apps.json.history\` içine bir anlık görüntü
kaydeder ve pencereyi yeniler.

## Tanılama komutları (test)

Yalnızca kanal: komut satırı bunları kabul etmez. Hepsi Windows ve Linux'ta çalışır; yalnızca
`screenshot` Windows'a özgüdür ve başka yerlerde `screenshot is not supported on this
platform (Windows only)` yanıtını verir.

| Komut | Bağımsız değişkenler | Sonuç |
| --- | --- | --- |
| `screenshot` | [`window`] [`max_dim`] | Yalnızca Windows. O Moonpool penceresinin PNG'sinin Base64 hali (varsayılan `main`). İsteğe bağlı `max_dim`, uzun kenarı piksel olarak sınırlar (320-2400 aralığına sıkıştırılır, varsayılan 320; MCP aracı her zaman varsayılanı kullanır). Tam sayı olmayan bir `max_dim` hatadır. İzin verilen pencereler: `main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`. Hatalar: `unknown window '<name>'`, `window '<name>' is not open`. Diske yazılmaz. |
| `open-window` | `<kind>` [`<id>`] | null. Bir pencereyi menü öğesinin açtığı şekilde açar. `kind`: `settings`, `about`, `installer`, `help`, `themes`, `editor` (isteğe bağlı `<id>` o uygulamanın Uygulamayı düzenle iletişim kutusunu açar, yoksa Uygulama ekle açılır), `terminal` (`<id>` zorunlu: o uygulamanın terminal sekmesini seçer ve CLI paneli görünsün diye merkezi genişletir; uygulamayı başlatmaz), `cli` (yalnızca merkez'ı genişletir). Hatalar: `unknown window kind '<kind>'`, `terminal needs an app id`, `unknown app id: <id>`. `launch` gibi merkez penceresi üzerinden yanıtlanır. |
| `window-state` | [`window`] | JSON dizesi: `{"open":false}` ya da `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. |
| `stop-mcp` | `<id>` | `stopped`. Uygulamanın `<processName> mcp` yardımcısını sonlandırır, uygulamayı değil. Hatalar: `missing app id`, `unknown app id: <id>`. |
| `reset-mcp-seen` | [`<id>`] | `<id>: cleared` ya da `<id>: was not marked seen`; kimlik yoksa `cleared <n> entries`. Hatırlanan MCP yardımcısı görülme kayıtlarını temizler. |

Komut satırının `--ticket` ve `state.json` sonuç kayıtları diğer kanala aittir; bkz.
[Komut satırı](/tr/automation/command-line/#sonucu-okuma). Kanal istekleri yanıtlarını yanıtın içinde alır.

## Ayrıca bakın

- [Komut satırı](/tr/automation/command-line/)
- [Yapay zekâ ajanları: hızlı başlangıç](/tr/automation/quick-start/#aynı-eylem-üç-yolla)
