---
title: "Moonpool hata iletileri açıklamalı: already running, requires a command ve daha fazlası"
description: "already running, requires a command, stale token ve Update failed gibi Moonpool hata iletilerinin tam metnini arayın; her birinin ne anlama geldiğini ve çözümünü görün."
---

Gördüğünüz iletiyi sayfa aramasına yapıştırın ya da tablolara göz atın. İletiler Moonpool'un
gösterdiği biçimde alıntılanmıştır. `<angle brackets>` içindeki metin bir değerle (bir
uygulama kimliği, bir yol veya sistemden gelen bir hata) değiştirilir. Hata iletisi olmayan
belirtiler [Sorun giderme ve SSS](/tr/support/troubleshooting/) sayfasındadır.

## Bir uygulamayı başlatma ve durdurma

| İleti | Anlamı ve çözümü |
| --- | --- |
| `already running` | Moonpool bu uygulama için zaten bir terminal tutuyor. Önce durdurun ya da Yeniden başlat'ı kullanın. |
| `stopped during launch` | Başlatma hâlâ sürerken Durdur'a basıldı. Yeniden başlatın. |
| `app has no launch command` | Girdinin `command` değeri yok. Uygulama düzenleyicisinde veya `apps.json` içinde bir tane ekleyin. Yalnızca `url` içeren bir `static` girdi bunsuz olabilir. |
| `unknown app: <id>` | Bu `id` ile yüklenmiş bir uygulama yok. Kimliği denetleyin, `apps.json` dosyasını elle düzenlediyseniz Yeniden yükle'yi kullanın. |
| `unknown app id: <id>` | Aynı sorun, bir betiğe veya aracıya bildirilmiş hali. Uygulamaları `moonpool_list_apps` ile listeleyin. |
| `did not reach running in time` | Bir betikten veya aracıdan: uygulama 25 saniye içinde Çalışıyor olarak okunmadı. `port` veya `processName` değerini denetleyin ve çıktıyı okuyun. Bkz. [Durum noktası yanlış](/tr/support/troubleshooting/#durum-noktası-yanlış). |
| `still running after stop` | 15 saniye sonra uygulama hâlâ Çalışıyor olarak okunuyor. `killMode` ayarlayın. Bkz. [Durdurma ve yeniden başlatma](/tr/apps/stop-and-restart/). |
| `refusing to open non-web url: <url>` | `url`, `http://`, `https://`, `mailto:` veya `file://` değil. `url` değerini düzeltin. |
| `[process exited]` | Hata değildir: uygulamanın komutu bitti. Terminal sekmesinde gösterilir. |

## apps.json doğrulaması

Moonpool, bir kuralı bozan `apps.json` dosyasını reddeder ve son yüklenen listeyi korur.
`<n>`, girdinin dosyadaki konumudur ve 1'den başlayarak sayılır.

| İleti | Çözüm |
| --- | --- |
| `apps.json entry <n> has invalid id "<id>"; use letters, digits, '.', '_', and '-' without a leading '-'` | `id` değerini yeniden adlandırın. |
| `duplicate app id "<id>"` | İki girdi aynı `id` değerini paylaşıyor. Her birini benzersiz yapın. |
| `apps.json entry <n> (<id>) has an empty name` | `name` alanını doldurun. |
| `apps.json entry <n> (<id>) has an empty group` | `group` alanını doldurun. |
| `apps.json entry <n> (<id>) has unknown type "<type>"` | `type`, `web`, `desktop`, `static` veya `cli` olmalıdır. |
| `apps.json entry <n> (<id>) has invalid port 0` | `port` 1 ile 65535 arasında olmalıdır. |
| `apps.json entry <n> (<id>) requires a url` | Bir `static` girdinin `url` değeri olmalıdır. |
| `apps.json entry <n> (<id>) requires a command` | Diğer her türün bir `command` değeri olmalıdır. |

Uygulama düzenleyicisinde ad olmadan kaydetmek `name is required.` (ad zorunludur) iletisini
gösterir. Şerit metni, "apps.json dosyasında hata var; en son yüklenen liste gösteriliyor."
veya "apps.json dosyasında hata var, bu yüzden hiçbir uygulama yüklenmedi.", ve nasıl
kurtarılacağı [apps.json hata içeriyor](/tr/support/troubleshooting/#appsjson-hata-içeriyor)
başlığı altındadır. Şerit kaydetmenin duraklatıldığını söylüyorsa ileti şununla biter:
`Repair apps.json and reload it before saving from Moonpool`. Kuralların tam listesi
[Doğrulama](/tr/apps/apps-json/#doğrulama) bölümündedir.

## Ayarlar, güncellemeler ve kurulum programı

| İleti | Anlamı ve çözümü |
| --- | --- |
| `... Repair settings.json and restart Moonpool before changing settings` | `settings.json` bozuk. Düzeltin ya da silin ve yeniden başlatın. Bkz. [settings.json](/tr/data/settings-json/#okuma-ve-onarım). |
| `Update failed: <error>` | Bir güncellemenin indirilmesi veya kurulumu başarısız oldu. Bkz. [Bir güncelleme başarısız olduğunda](/tr/data/updating/#bir-güncelleme-başarısız-olduğunda). |
| `Update check failed: <error>` | Hakkında penceresindeki güncelleme denetimi başarısız oldu. İki noktadan sonraki metin nedenini söyler. Daha sonra yeniden deneyin. |
| `Install failed: <error>` | Kurulum programı, iki noktadan sonra adı geçen adımda durdu, örneğin `copy exe: ...`. `%USERPROFILE%\.moonpool` içinden çalışan her Moonpool'dan çıkın ve yeniden deneyin. |
| `target folder does not exist` | Taşınabilir kopya için seçilen klasör artık yok. Var olan birini seçin. |

## MCP ve betikler

| İleti | Anlamı ve çözümü |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Moonpool'u başlatın ya da aracının bu aracı çağırmasına izin verin. Taşınabilir bir kopya için ileti kopyayı adıyla belirtir. |
| `frontend not loaded` | Merkez penceresi yüklenmeyi henüz bitirmedi. Bekleyip yeniden deneyin. |
| `invalid app_id: use only letters, digits, '.', '_', '-' (no leading '-')` | Aracı, MCP sunucusunun kabul etmeyeceği bir kimlik verdi. `moonpool_list_apps` çıktısındaki kimliği kullanın. |
| `stale token: apps.json changed since it was read ...` | `apps.json` dosyasını yeniden okuyun, düzenlemeyi yeniden uygulayın, sonra yazın. |
| `rejected invalid manifest: ...` | Yeni `apps.json` doğrulamadan geçemedi (yukarıya bakın). Dosya değiştirilmedi. |
| `no console output recorded for '<id>' (not launched this session)` | `moonpool_app_output`, Moonpool başladığından beri çalışmamış bir uygulama için istendi. |

Daha fazlası [MCP araçları](/tr/automation/mcp-tools/) ve
[MCP kurulumu](/tr/automation/mcp-setup/#araçlar-çalışmıyorsa) sayfalarındadır.

## Diğer programlardan gelen hatalar

- [`Error: listen EADDRINUSE: address already in use :::3000` ve `Port 5173 is in use`](/tr/support/port-already-in-use/)
- [`Windows protected your PC`](/tr/support/windows-protected-your-pc/)
- [WebView2 çalışma zamanı eksik](/tr/support/webview2-runtime-missing/)
