---
title: "Moonpool'u komut satırından denetleme"
description: "Çalışan bir Moonpool'u terminalden veya betikten moonpool.exe komutlarıyla yönetin, bir komutu bilet anahtarıyla etiketleyin ve sonucu state.json dosyasından okuyun."
---

Aynı Moonpool zaten çalışırken bir `moonpool.exe` dosyasını yeniden çalıştırmak ikinci bir pencere açmaz.
İkinci süreç bağımsız değişkenlerini [denetim kanalı](/tr/automation/control-verbs/) üzerinden çalışan sürece
iletir ve çıkar. Moonpool zaten çalışıyor olmalıdır: hiçbiri yerleşik değilse aynı komut yeni bir Moonpool
başlatır ve komut çalıştırılmaz.

"Aynı Moonpool", aynı klasör demektir. Kurulu Moonpool ve her taşınabilir kopya kendi başına çalışır;
yani bir komut, çalıştırdığınız `moonpool.exe` dosyasının ait olduğu kopyaya ulaşır, asla bir başkasına ulaşmaz.
Bkz. [Taşınabilir mod](/tr/data/portable-mode/#aynı-anda-birkaç-kopya).

Kastettiğiniz kopyanın yolunu kullanın. Kurulu olan için:

```powershell frame="terminal"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
```

Birkaç kopya çalışırken `Get-Process moonpool` hepsini listeler; bu yüzden ilkini almak yerine `Path`
değerine göre seçin. Ayrıca MCP ana bilgisayarlarının başlattığı boşta `moonpool.exe mcp` yardımcılarını da
listeler; dolayısıyla bir `moonpool` süreci bir merkezin çalıştığını kanıtlamaz. Bunun yerine denetim
kanalına `ping` ile sorun ([Denetim komutları](/tr/automation/control-verbs/)).

## Komutlar

Komut büyük/küçük harfe duyarlı değildir. `<id>`, `apps.json` içindeki bir uygulamanın `id` değeridir.

| Komut | Etki |
| --- | --- |
| `moonpool.exe` | Komut yok: pencereyi öne getirir. |
| `moonpool.exe show` | Pencereyi öne getirir. |
| `moonpool.exe launch <id>` | Uygulamayı başlatır ve terminal sekmesini açar. |
| `moonpool.exe stop <id>` | Uygulamayı durdurur. |
| `moonpool.exe restart <id>` | Durdurur, bağlantı noktasının ve sürecin serbest kalmasını bekler, başlatır. |
| `moonpool.exe reload` | `apps.json` dosyasını yeniden okur. |
| `moonpool.exe refresh-icons` | Her simgeyi yeniden getirir. |
| `moonpool.exe help` | Yardım penceresini açar. |
| `moonpool.exe quit` | Moonpool'dan çıkar, sistem tepsisi menüsündeki seçenekle aynıdır. |
| `moonpool.exe dump <id> [out-path]` | `out-path` olmadan, uygulamanın bu oturuma ait günlüğünün yolunu bildirir. Verilirse günlüğü ANSI kodları kaldırılmış düz metin olarak oraya kopyalar. |
| `moonpool.exe paths` | Çalışan Moonpool'un kullandığı yapılandırma klasörünü, `apps.json`, `state.json`, günlük, dumps klasörü, icons klasörü, taşınabilir bayrağı ve exe yolunu bildirir. |
| `moonpool.exe read-config` | Yapılandırma klasöründe `dumps\read-config.json` dosyasını yazar; içinde `token`, `valid`, `error`, `path` ve `manifest_text` (`apps.json` dosyasının tam içeriği) bulunur. |
| `moonpool.exe write-config <file> [token]` | Manifest geçerliyse ve `token` verilmişse `apps.json` hâlâ onunla eşleşiyorsa, `apps.json` dosyasını `<file>` içindeki manifestle değiştirir. |
| `moonpool.exe restore-config [index or filename]` | Bağımsız değişken yoksa anlık görüntü listesini `dumps\restore-config.json` dosyasına yazar. Bir bağımsız değişkenle, o anlık görüntü geçerliyse geri yükler. |

```powershell frame="terminal"
& $mp restart my-app
```

```powershell frame="terminal"
& $mp dump my-app C:\temp\my-app.log
```

Bilinmeyen bir komut yok sayılır. Programın kendi başlangıç bağımsız değişkenleri de vardır:
`moonpool.exe mcp` ([MCP kurulumu](/tr/automation/mcp-setup/)), `--uninstall` (Add/Remove Programs tarafından
kullanılır) ve `--wait-pid <pid>` (Moonpool kendini yeniden başlatırken kullanılır). Bunlar yalnızca ilk
bağımsız değişken olarak dikkate alınır; bu yüzden `--uninstall` gibi bir uygulama kimliği bunları tetikleyemez.

## Sonucu okuma

Komut satırı hiçbir şey yazdırmaz; bu yüzden bir komutu `--ticket <key>` ile etiketleyin (herhangi bir benzersiz
anahtar, herhangi bir konumda) ve sonucu yapılandırma klasöründeki `state.json` dosyasından okuyun. Bu klasör,
kurulu sürümde `%USERPROFILE%\.moonpool\moonpool-config\`, taşınabilir kopyada
`<your .moonpool folder>\moonpool-config\` ve Linux'ta `~/.config/Moonpool/` olur (bkz.
[Yapılandırmaya genel bakış](/tr/apps/apps-json/#yapılandırmanın-bulunduğu-yer)). `show` ve `quit` bilet yazmaz.

```powershell frame="terminal"
& $mp launch my-app --ticket t1
```

`state.json` içinde `apps`, `statuses` (uygulama başına `id`, `running`, `managed`, `mcpRunning`, `mcpSeen`)
ve `tickets` bulunur. Çalışan Moonpool onu birkaç saniyede bir ve her komuttan sonra yeniden yazar, çıkarken
silmez; yani geride kalan bir dosya Moonpool'un çalıştığı anlamına gelmez. Çalışıp çalışmadığını sormak veya
canlı uygulama listesini almak için denetim kanalının `ping` ve `list` komutlarını
([Denetim komutları](/tr/automation/control-verbs/)) ya da MCP araçlarını kullanın. Biletinizi `status`
değeri `pending` olmayana kadar yoklayın:

| `status` | Anlamı |
| --- | --- |
| `pending` | Alındı; Moonpool hâlâ üzerinde işlem yapıyor. |
| `ok` | Bitti. `dump`, `read-config`, `write-config`, `restore-config` ve `paths` için `detail` yolu, belirteci veya raporu içerir. |
| `error` | Başarısız; `detail` nedenini söyler, örneğin `unknown app id: x`, `did not reach running in time`, `unknown command`. |

Her bilet `{ ticket, action, arg, status, detail, ts }` biçimindedir; `ts` Unix milisaniyesidir:

```json title="state.json (tickets entry)"
{
  "ticket": "t1",
  "action": "launch",
  "arg": "my-app",
  "status": "error",
  "detail": "did not reach running in time",
  "ts": 1767225600000
}
```

Tamamlanan biletler 24 saat sonra silinir; tamamlanan biletler en az 5 dakikalık olduğunda liste 50 girdiye doğru
kırpılır.

MCP destekleyen bir ajan yoklamayı atlayabilir: bkz. [MCP kurulumu](/tr/automation/mcp-setup/).

## Ayrıca bakın

- [Yapay zekâ ajanları: hızlı başlangıç](/tr/automation/quick-start/)
- [Denetim komutları](/tr/automation/control-verbs/)
