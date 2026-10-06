---
title: "Moonpool MCP araçları başvurusu: parametreler ve sonuçlar"
description: "Moonpool MCP sunucusunun ajanlara sunduğu her araç; parametreleri, ne döndürdüğü ve karşılaşabileceğiniz hata durumlarıyla birlikte."
---

Tüm araçlar metin döndürür; bir PNG görüntüsü döndüren `moonpool_screenshot` hariç. Bir başarısızlık, nedeni
metin olarak içeren ve hata olarak işaretlenmiş bir araç sonucu olarak gelir. Kurulum için bkz.
[MCP kurulumu](/tr/automation/mcp-setup/).

`app_id` alan araçlar, uygulamanın `apps.json` içindeki `id` değerini gerektirir. Yalnızca harf, rakam,
`.`, `_` ve `-` içermeli ve `-` ile başlamamalıdır; aksi halde çağrı "invalid
app_id" ile başarısız olur.

Merkez üzerinde işlem yapan araçların çoğu, merkez çalışmıyorken bu iletiyle başarısız olur.
`moonpool_bootup_launcher`, `moonpool_shutdown_launcher`, `moonpool_raise_launcher` ve
`moonpool_launcher_paths` bu durumu kendileri ele alır (satırlarına bakın). Taşınabilir bir kopya için
ileti kopyanın adını verir, örneğin `Moonpool (<folder>)`.

```text
Moonpool is not running - call moonpool_bootup_launcher first
```

Bir sonuç bekleyen çağrılar 45 saniye sonra zaman aşımına uğrar.

## Başlatıcı ve uygulamalar

Örnek `moonpool_list_apps` sonucu:

```text
site  [running] (managed by Moonpool)  Site
notes-app  [stopped]  [mcp: stopped]  Notes App
```

| Araç | Parametreler | Davranış |
| --- | --- | --- |
| `moonpool_list_apps` | yok | Uygulama başına bir satır: `id  [running]` veya `[stopped]`, uygulanabiliyorsa `(managed by Moonpool)`, bir MCP yardımcısı görülmüşse `[mcp: running]` veya `[mcp: stopped]`, ardından ad. Çalışan merkeze denetim kanalı (`list` komutu) üzerinden sorulur, yani canlıdır. Moonpool çalışmıyorsa eski bir liste göstermek yerine "Moonpool is not running" ile başarısız olur. Moonpool başladıktan hemen sonra, ilk durum denetiminden önce uygulamalar `[status pending]` gösterir. `apps.json` hatalıyken sonuç `apps.json has an error: <message>. This list is the last one that loaded; fix the file and call moonpool_reload_config.` ile başlar. Dosya Moonpool başladığında zaten bozuksa hiçbir uygulamanın yüklenmediğini söyler ve ayrıca `moonpool_restore_config` aracını önerir. |
| `moonpool_bootup_launcher` | yok | Moonpool'un kendisini başlatır ve denetim kanalının yanıt vermesi için en fazla 30 sn bekler. "Moonpool started" veya "Moonpool is already running" döndürür. Yeni süreç hemen çıkarsa (hâlâ kapanmakta olan bir Moonpool'a devretmişse) bir tane daha başlatır. Bir şey kanalı tutup yanıt vermiyorsa bir Moonpool sürecinin takılmış olabileceğini bildirir. |
| `moonpool_shutdown_launcher` | yok | Sistem tepsisi menüsündeki Çıkış ile aynıdır. Denetim kanalının kaybolması için en fazla 30 sn bekler. "Moonpool shut down" veya "Moonpool is not running" döndürür. |
| `moonpool_raise_launcher` | yok | Moonpool penceresini öne getirir. "window shown" döndürür. Moonpool çalışmıyorsa onu başlatır ve "Moonpool was not running; started it" döndürür. |
| `moonpool_start_app` | `app_id` (zorunlu) | Uygulamayı başlatır ve terminal sekmesini açar. Çalışır duruma geldiğinde "launched" ya da olmama nedenini döndürür (`unknown app id: <id>`, 25 sn sonra `did not reach running in time`). Yalnızca `url` içeren bir `static` girdi için sayfayı açar ve ayrıca "launched" döndürür. |
| `moonpool_stop_app` | `app_id` (zorunlu) | Uygulamayı durdurur. "stopped" ya da (15 sn sonra) `still running after stop` gibi bir hata döndürür. |
| `moonpool_restart_app` | `app_id` (zorunlu) | Durdurur, bağlantı noktasının ve sürecin serbest kalmasını bekler, başlatır. "restarted" döndürür. |
| `moonpool_app_output` | `app_id` (zorunlu), `tail_lines` (tam sayı, varsayılan 200, en az 1) | Uygulamanın geçerli Moonpool oturumuna ait terminal çıktısı, ANSI kodları kaldırılmış. Günlük `tail_lines` değerinden uzunsa metin, tam günlüğün yolunu veren bir satırla başlar. Uygulama hiç çalışmamışsa `no console output recorded for '<id>' (not launched this session)` ile başarısız olur. Günlük var ama boşsa `(no output recorded for '<id>')` döndürür. |
| `moonpool_stop_mcp_server` | `app_id` (zorunlu) | Uygulamanın bağlı MCP yardımcı sürecini sonlandırır ve uygulamayı çalışır halde bırakır. "stopped" döndürür. Uygulamada ne `processName` ne de `mcpProcessName` varsa hiçbir şey yapmaz. |
| `moonpool_refresh_app_icons` | yok | Her uygulama simgesini yeniden getirir. "icons refreshed" döndürür. |

## Yapılandırma

Bunlar `apps.json` dosyasını merkez üzerinden okur ve değiştirir, diskteki dosya üzerinden asla. Bir yazma, son
okumadan gelen belirteci taşımalıdır; eski bir belirteç reddedilir ve yeni dosya herhangi bir şey yazılmadan önce
doğrulanır. Merkez üzerinden gitmek önemlidir, çünkü korumalı bir ana bilgisayardaki ajana gerçek yapılandırma
klasörü yerine onun özel bir kopyası gösterilebilir.

| Araç | Parametreler | Davranış |
| --- | --- | --- |
| `moonpool_read_config` | yok | `manifest_text` (dosyanın tam içeriği), `token`, `valid`, `error` (geçerliyken null) ve `path` içeren JSON metni. Dosya yoksa veya boşsa `token` değeri `none` olur. |
| `moonpool_write_config` | `manifest` (zorunlu, tam yeni `apps.json` metni), `expected_token` (zorunlu, son okumadan) | Manifesti doğrular ve `apps.json` dosyasını değiştirir, ardından yükler. `apps.json updated; new version token <token>` döndürür. Eski bir belirteç `stale token: apps.json changed since it was read ...` ile başarısız olur. Geçersiz bir manifest `rejected invalid manifest: ...` ile başarısız olur. Her iki durumda da dosyaya dokunulmaz. Boş bir `expected_token` reddedilir. |
| `moonpool_restore_config` | `snapshot` (isteğe bağlı) | Değer verilmezse kayıtlı anlık görüntüleri en yeniden başlayarak listeleyen JSON metni (`index`, `filename`, `millis`, `app_count`, `valid`). Bir dizin (1 = en yeni) veya dosya adıyla, o anlık görüntüyü doğrular ve geri yükler. `restored <file> (<n> apps); new version token <token>` döndürür. Belirteç gerekmez: geri yükleme geçerli dosyanın üzerine bilerek yazar. |
| `moonpool_reload_config` | yok | `apps.json` dosyasını yeniden okur. "apps.json reloaded" döndürür. Dosya ayrıştırılamıyor veya doğrulanamıyorsa `apps.json has an error: ...` ile başarısız olur ve Moonpool yüklenen son listeyi tutar. |
| `moonpool_launcher_paths` | yok | Merkezin yapılandırma klasörünü, `apps.json`, `state.json`, günlük, dumps klasörü, icons klasörü, taşınabilir bayrağı ve exe yolunu, ardından MCP sürecinin yapılandırma klasörünü, `apps.json`, `state.json`, dumps klasörü, taşınabilir bayrağı ve exe yolunu listeler (günlük veya icons yok). Merkez çalışmıyorsa onun yarısı `merkez paths unavailable: ...` olarak görünür ve MCP yarısı yine de gösterilir. Bir düzenleme etkili olmadığında kullanın. |

## İleri düzey: test araçları

`moonpool_screenshot` yalnızca Windows'ta çalışır; Linux ve macOS'ta "screenshot is not
supported on this platform" ile başarısız olur. `moonpool_window_state` ve `moonpool_reset_mcp_seen` her platformda
çalışır.

`window` değeri `main`, `settings`, `about`, `installer`, `editor`, `help` veya `themes` olabilir ve varsayılanı
`main` değeridir. Bilinmeyen bir ad `unknown window '<name>'` ile başarısız olur.

| Araç | Parametreler | Davranış |
| --- | --- | --- |
| `moonpool_screenshot` | `window` (isteğe bağlı) | O Moonpool penceresinin kendi içeriğini, uzun kenarı en fazla 320 piksel olacak şekilde satır içi bir PNG olarak yakalar. Boyut MCP'den artırılamaz. Pencere görünmüyorsa `window '<name>' is not open` ile başarısız olur. Başka hiçbir uygulamayı yakalayamaz. |
| `moonpool_window_state` | `window` (isteğe bağlı) | JSON metni: pencere açık değilse `{"open":false}`, aksi halde `open`, `visible`, `minimized`, `maximized`, `x`, `y`, `width`, `height`. Testler için tasarlanmıştır. |
| `moonpool_reset_mcp_seen` | `app_id` (isteğe bağlı) | Yalnızca test için. Bir uygulama için ya da atlanırsa her uygulama için hatırlanan "bir MCP yardımcısı görüldü" kaydını temizler; böylece kenar çubuğundaki MCP alt satırı, bir yardımcı görülene kadar yeniden gizlenir. |

## Ayrıca bakın

- [MCP kurulumu](/tr/automation/mcp-setup/)
- [Komut satırı](/tr/automation/command-line/)
