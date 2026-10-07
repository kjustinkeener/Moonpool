---
title: "Moonpool sözlüğü: uygulamalar, durumlar, dosyalar ve ayarlar"
description: "Moonpool yardımının parçaları, uygulama durumları, dosyaları ve ayarları için kullandığı sözcüklerin sade tanımları; belgelerin geri kalanını izlemenize yardımcı olur."
---

## Uygulamalar

| Terim | Anlamı |
| --- | --- |
| uygulama (app) | Moonpool'un yönettiği tek bir şey: bir geliştirme sunucusu, bir masaüstü uygulaması, bir sayfa veya bir komut. |
| girdi (entry) | Bir uygulamanın `apps.json` içindeki kaydı. Yalnızca JSON'dan söz ederken kullanılır. |
| uygulama satırı (app row) | Bir uygulamanın kenar çubuğundaki satırı; durum noktası ve denetimleriyle birlikte. |
| grup (group) | Bir uygulamanın altında listelendiği kenar çubuğu başlığı; `group` alanından gelir. |
| tür (type) | `web`, `desktop`, `static` veya `cli`. Hangi alanların önemli olduğunu belirler. Bkz. [Uygulama türleri](/tr/apps/types/). |
| kimlik (id) | Bir uygulamanın kalıcı anahtarı; dosya adlarında, komutlarda ve aracı araçlarında kullanılır. Bkz. [Kimlik](/tr/apps/apps-json/#id). |

## Uygulama durumları

| Durum | Anlamı |
| --- | --- |
| başlatılıyor (starting) | Moonpool uygulamayı başlattı ama henüz ayakta olduğunu görmedi. Yanıp sönen nokta. |
| Çalışıyor (Running) | `port` yanıt veriyor, `processName` mevcut ya da ikisi de ayarlı değilse Moonpool'un başlattığı terminal hâlâ canlı. Sabit nokta. Bkz. [Çalışıyor durumuna nasıl karar verilir](/tr/apps/types/#çalışıyor-durumu-nasıl-belirlenir). |
| durduruldu (stopped) | Yukarıdakilerin hiçbiri. Gri nokta. |
| yönetilen (managed) | Moonpool onu bu oturumda başlattı. Yönetilmeyen çalışan bir uygulama başka bir yolla başlatılmıştır ve Çıkış ona dokunmaz. |

Bir terminal sekmesi ile çalışan bir uygulama ayrı şeylerdir. Bir uygulamanın adına tıklamak
yalnızca terminal sekmesini açar; uygulamayı asla başlatmaz. Bir sekmeyi kapatmak uygulamayı
asla durdurmaz.

## Pencereler ve parçalar

| Terim | Anlamı |
| --- | --- |
| merkez (hub) | Yerleşik Moonpool süreci ve ana penceresi. Araç adları ona "launcher" (başlatıcı) der. |
| merkez penceresi (hub window) | Ana pencere: solda kenar çubuğu, sağda CLI paneli. |
| sistem tepsisi (tray) | Sistem tepsisi simgesi ve menüsü (**Moonpool'u göster**, **Çıkış**). |
| kenar çubuğu (sidebar) | Merkez penceresinin sol tarafı: filtre kutusu, **...** menüsü ve uygulama satırları. |
| CLI paneli (CLI pane) | Merkez penceresinin sağ tarafı; terminal sekmelerini barındırır. |
| terminal sekmesi (terminal tab) | CLI panelinde bir uygulamanın terminali. |
| MCP alt satırı (MCP sub-row) | Bir uygulamanın altında, kendi `<exe> mcp` yardımcı sürecini gösteren soluk bir satır. |
| uygulama düzenleyicisi (app editor) | Uygulama ekle ve Uygulamayı düzenle iletişim kutusu. |

## Dosyalar ve klasörler

| Terim | Anlamı |
| --- | --- |
| yapılandırma klasörü (config folder) | `apps.json` ve Moonpool'un diğer dosyalarını barındıran klasör. `{MP_DATA}` belirteci. Bkz. [Yapılandırma nerede durur](/tr/apps/apps-json/#yapılandırmanın-bulunduğu-yer). |
| `{MP_HOME}` | Moonpool klasörü: kuruluyken `%USERPROFILE%\.moonpool`, taşınabilir bir kopyanın `.moonpool\` klasörü, Linux'ta yapılandırma klasörü. |
| oturum (session) | Merkezin başlangıçtan Çıkış'a kadar tek bir çalışması. |
| oturum günlüğü (session log) | Bir uygulamanın bir oturum boyunca yazdırdığı her şeyi tutan dosya; `cli-output\` altında. Bkz. [Günlükler](/tr/data/logs/). |
| `moonpool.log` | Moonpool'un kendi hata ayıklama günlüğü; yalnızca **Hata ayıklama bilgilerini bir dosyaya kaydet** açıkken yazılır. |
| döküm (dump) | `dump` fiiliyle oluşturulan, bir oturum günlüğünün düz metin kopyası. |
| anlık görüntü (snapshot) | İyi bir `apps.json` dosyasının `apps.json.history\` içindeki kopyası. Bkz. [Yedekleme ve kurtarma](/tr/data/backup-and-recovery/). |

## Modlar

| Terim | Anlamı |
| --- | --- |
| kurulu (installed) | `%USERPROFILE%\.moonpool` içindeki, Başlat Menüsü kısayolu ve Program Ekle/Kaldır girdisi olan bir Moonpool. Yalnızca Windows. |
| taşınabilir (portable) | Seçtiğiniz bir `.moonpool\` klasöründeki, bir `moonpool.portable` dosyasıyla işaretli bir Moonpool. Bkz. [Taşınabilir mod](/tr/data/portable-mode/). |
| kopya (copy) | Kurulu ya da taşınabilir tek bir Moonpool klasörü. Her kopya kendi başına çalışır. |

## Durdurma ve otomasyon

| Terim | Anlamı |
| --- | --- |
| `killMode` | Durdur'un uygulamanın terminalini sonlandırdıktan sonra attığı ek adım. Bkz. [Durdurma ve yeniden başlatma](/tr/apps/stop-and-restart/). |
| `stopCommand` | `killMode` değeri `command` olduğunda Durdur'un çalıştırdığı komut. |
| `processName` | Moonpool'un izlediği ve `processName` modunda sonlandırdığı süreç adı. |
| denetim kanalı (control channel) | Merkezin yanıt verdiği adlandırılmış kanal (Windows) veya Unix soketi (Linux). Bkz. [Denetim fiilleri](/tr/automation/control-verbs/). |
| fiil (verb) | Komut satırında veya denetim kanalında verilen `launch` ya da `reload` gibi bir komut sözcüğü. |
| bilet (ticket) | Bir komutun sonucunu `state.json` içinden okumak için `--ticket` ile eklediğiniz anahtar. |
| belirteç (token) | Bir yapılandırma yazımının taşıması gereken `apps.json` sürüm damgası. |
| MCP yardımcısı (shim) | Bir yapay zeka ana bilgisayarının, bir uygulamanın kendi araçlarına ulaşmak için başlattığı `<exe> mcp` süreci. |
