---
title: "Moonpool sorun giderme: sistem tepsisi, başlamayan uygulamalar, güncellemeler"
description: "Sık karşılaşılan Moonpool sorunlarını gördüğünüze göre giderin: eksik sistem tepsisi simgesi, başlamayan veya durmayan uygulamalar, yanlış durum noktaları, başarısız güncellemeler ve MCP hataları."
---

Belirtiyi bulun, sonra çözümü izleyin. Alıntılanan metin Moonpool'un gösterdiğidir. Tam bir
iletiyi aramak için bkz. [Hata iletileri açıklamalı](/tr/support/error-messages/).

## Sistem tepsisi simgesini göremiyorum

- **Windows.** Simge gizli simgeler alanında olabilir. Görev çubuğunun sağındaki **^** okuna
  tıklayın. Görünür kalması için simgeyi görev çubuğuna sürükleyin.
- **Standart GNOME'da Linux.** GNOME, AppIndicator eklentisi olmadan sistem tepsisi simgelerini
  göstermez. Bkz. [Linux](/tr/platforms/linux/#gnomeda-sistem-tepsisi).
- **Ayarlar.** **Tepside göster** kapalı olabilir. Merkez penceresini görev çubuğundan ya da
  Başlat Menüsü'nden açın ve [Ayarlar](/tr/using/settings/) içinde yeniden açın.

## Kurulum programı bir hata gösteriyor

| İleti | Ne yapmalı |
| --- | --- |
| `Install failed: <error>` | İki noktadan sonraki metin başarısız olan adı verir, örneğin `copy exe: ...`. Bir dosya kullanımdaysa `%USERPROFILE%\.moonpool` içinden çalışan her Moonpool'dan çıkın ve yeniden deneyin. |
| `target folder does not exist` | Taşınabilir kopya için seçtiğiniz klasör artık yok. Var olan bir klasör seçin. |
| `that folder already has a .moonpool with an exe of this name that isn't a portable Moonpool - pick an empty folder` | Boş bir klasör seçin ya da önce o `.moonpool` klasörünü kaldırın. |

## Kurulum programını çalıştırınca Windows bilgisayarınızı korudu uyarısı çıkıyor

Bu, `moonpool.exe` kod imzalı olmadığı için Windows SmartScreen'dir. **More info** (Daha fazla
bilgi), ardından **Run anyway** (Yine de çalıştır) seçeneğine tıklayın. Bkz.
[Windows bilgisayarınızı korudu](/tr/support/windows-protected-your-pc/).

## Windows'ta Moonpool penceresi boş ya da hiç açılmıyor

Microsoft Edge WebView2 Çalışma Zamanı eksik olabilir. Bkz.
[WebView2 çalışma zamanı eksik](/tr/support/webview2-runtime-missing/).

## Bir uygulama başlamıyor

1. Terminal sekmesini açmak ve çıktıyı okumak için uygulamanın adına tıklayın. Bir aracı aynı
   metni `moonpool_app_output` ile okuyabilir.
2. `cwd` değerini denetleyin. Eksik bir klasör ya da `./` olmayan göreli bir yol olağan
   nedendir. Bkz. [Yollar ve ortam](/tr/apps/paths-and-environment/).
3. `command` değerini denetleyin. Onu `cwd` içinde bir terminalde elle çalıştırın. Windows'ta
   iç içe çift tırnaktan kaçının; `cmd /c` onları bozar.
4. Ayarlar'da **Hata ayıklama bilgilerini bir dosyaya kaydet** seçeneğini açın ve yeniden
   başlatın. `moonpool.log` tam komutu ve klasörü kaydeder. Bkz. [Günlükler](/tr/data/logs/).

| İleti | Anlamı |
| --- | --- |
| `already running` | Moonpool bu uygulama için zaten bir terminal tutuyor. Önce durdurun ya da Yeniden başlat'ı kullanın. |
| `stopped during launch` | Başlatma hâlâ sürerken Durdur'a basıldı. |
| `did not reach running in time` | Bir betikten veya aracıdan: uygulama 25 saniye içinde Çalışıyor olarak okunmadı. `port` veya `processName` değerini ve çıktısını denetleyin. |

## Durum noktası yanlış

Moonpool Çalışıyor durumuna önce `port`, sonra `processName`, sonra kendi terminalinin hâlâ
canlı olup olmadığına göre karar verir. Bkz. [Çalışıyor durumuna nasıl karar verilir](/tr/apps/types/#çalışıyor-durumu-nasıl-belirlenir).

- **Hiç sabitlenmiyor.** Bir `web` uygulamasının `port` değeri yanıt vermiyor ya da bir
  `desktop` uygulamasının `processName` değeri eşleşmiyor. Linux'ta `processName` 15 karakter
  veya daha kısa olmalıdır.
- **Başlatmadan hemen sonra griye dönüyor.** Bir `cli` uygulaması komutu çıkınca Çalışıyor
  olmaktan çıkar. Açık kalmasını istiyorsanız `-NoExit` ile bir kabuk kullanın.
- **Bir `static` uygulama hiç Çalışıyor göstermez.** Yalnızca `url` içeren bir girdi için
  bu beklenen davranıştır.
- **Siz başlatmadığınız halde Çalışıyor gösteriyor.** Başka bir şey o bağlantı noktasını ya da
  süreç adını kullanıyor. Moonpool onu çalışıyor ama "Moonpool tarafından yönetilmiyor"
  olarak gösterir.

## Error: listen EADDRINUSE veya "Port 5173 is in use"

Başka bir şey zaten sunucunuzun istediği bağlantı noktasını dinliyor. Onu bulup sonlandırın ya
da Durdur'un onu serbest bırakması için uygulamada `port` ayarlayın. Bkz.
[EADDRINUSE ve "Port 5173 is in use" hatalarını düzeltin](/tr/support/port-already-in-use/) ve
[Bir bağlantı noktasını kullanan süreci bulun ve sonlandırın](/tr/guides/find-and-kill-process-using-port-windows/).

## İki uygulama aynı bağlantı noktasını kullanıyor

**...** menüsünün altında bir uyarı satırı görünür, örneğin `bağlantı noktası 3000: App A / App B`.
Uygulamalardan birinin `port` değerini (ve `PORT` okuyorsa `env` değerini) değiştirin. Bkz.
[Bağlantı noktası çakışması uyarısı](/tr/using/hub-window/#bağlantı-noktası-çakışması-uyarısı).

## Uygulama Durdur'dan sonra çalışmaya devam ediyor

Bir betikten veya aracıdan hata `still running after stop` olur (15 saniye sonra).

- Uygulama terminalinden uzun yaşıyor. `killMode` değerini `port` veya `processName` yapın.
  Bkz. [Durdurma ve yeniden başlatma](/tr/apps/stop-and-restart/).
- Windows'ta bir Docker uygulaması: `docker compose stop app` gibi bir `stopCommand` ile
  `killMode` `command` kullanın. Asla `port` değil.

## apps.json hata içeriyor

Kenar çubuğu bir şerit gösterir: "apps.json dosyasında hata var; en son yüklenen liste
gösteriliyor." ya da başlangıçta "apps.json dosyasında hata var, bu yüzden hiçbir uygulama
yüklenmedi." Dosya yeniden yüklenene kadar Moonpool'dan kaydetme duraklatılır.

Tipik hatalar:

```text
apps.json entry 2 (site) requires a command
apps.json entry 3 has invalid id "my app"; use letters, digits, '.', '_', and '-' without a leading '-'
duplicate app id "site"
apps.json entry 4 (api) has invalid port 0
```

1. Şeritte **apps.json dosyasını düzenle** seçeneğini seçin, girdiyi düzeltin, kaydedin, sonra
   **Yeniden yükle** (F5) deyin.
2. Ya da yakın zamanlı iyi bir kopyaya geri dönün. Bkz.
   [Yedekleme ve kurtarma](/tr/data/backup-and-recovery/#appsjson-dosyasını-geri-alma).

Kuralların tam listesi [Doğrulama](/tr/apps/apps-json/#doğrulama) bölümündedir.

Bir ayar değiştirilemiyorsa ve ileti `Repair settings.json and restart Moonpool before
changing settings` ile bitiyorsa, yapılandırma klasöründeki `settings.json` dosyasını düzeltin
ya da silin ve Moonpool'u yeniden başlatın. Silmek her ayarı varsayılanına sıfırlar.

## Düzenlemem etkili olmadı

- Elle yapılan düzenlemeler **Yeniden yükle** (veya F5) gerektirir. Moonpool dosyayı izlemez.
- Yeniden yükleme çalışan uygulamaları yeniden başlatmaz. Değişen bir `command`, `cwd` veya
  `env` değerini kullanmak için uygulamayı yeniden başlatın.
- Bir aracı farklı bir `apps.json` dosyasını düzenliyor olabilir. Ondan `moonpool_launcher_paths`
  çağırmasını isteyin ve merkezin klasörünü kendisininkiyle karşılaştırın. Moonpool'un birden
  çok kopyası varsa hangi kopyayı düzenlediğinizi denetleyin.

## Örnek uygulamalar eksik

Örnekler yalnızca hiç `apps.json` yokken yazılır. Onları geri getirmek için bkz.
[Örneklere sıfırlama](/tr/data/backup-and-recovery/#örneklere-sıfırlama) ya da girdileri
[Örnek panolar](/tr/getting-started/example-dashboards/#örnek-uygulamalar-yalnızca-ilk-çalıştırmada-görünür)
sayfasından kopyalayın.

## Bir güncelleme başarısız oldu

Şerit `Update failed: <error>` gösterir. Bkz.
[Bir güncelleme başarısız olduğunda](/tr/data/updating/#bir-güncelleme-başarısız-olduğunda).

## Bir web bağlantısı açılmıyor

`refusing to open non-web url: <url>`, `url` değerinin `http://`, `https://`, `mailto:` veya
`file://` olmadığı anlamına gelir. `url` değerini düzeltin.

## MCP ve betik hataları

| İleti | Ne yapmalı |
| --- | --- |
| `Moonpool is not running - call moonpool_bootup_launcher first` | Moonpool'u başlatın ya da aracının `moonpool_bootup_launcher` çağırmasına izin verin. |
| `frontend not loaded` | Merkez penceresi yüklenmeyi henüz bitirmedi. Biraz bekleyip yeniden deneyin. |
| `stale token: ...` | `apps.json`, aracı onu okuduktan sonra değişti. Yeniden okuyun, sonra yazın. |
| `rejected invalid manifest: ...` | Yeni `apps.json` doğrulamadan geçemedi. Dosya değiştirilmedi. |
| `... A Moonpool process may be hung ...` | Bir şey denetim kanalını yanıt vermeden tutuyor. Moonpool'dan sistem tepsisinden çıkın ya da süreci sonlandırın ve yeniden başlatın. |

Daha fazlası [MCP kurulumu](/tr/automation/mcp-setup/#notlar) ve
[MCP araçları](/tr/automation/mcp-tools/) sayfalarındadır.

## Pencere sorunları

- **Ekran dışında.** Moonpool, bağlı hiçbir ekranda olmayan kayıtlı bir konumu yok sayar.
  Pencere yine de kayıpsa Moonpool'dan çıkın ve yapılandırma klasöründeki `window-state.json`
  dosyasını silin.
- **Yakınlaştırma çok büyük ya da çok küçük takılı kaldı.** Merkez penceresi üzerinde Ctrl +
  tekerlek onu değiştirir. Bkz. [Kısayollar ve yakınlaştırma](/tr/using/keyboard-shortcuts/#yakınlaştırma).
- **Ayarlar merkez penceresinin arkasında açılıyor.** Ayarlar'da **Her zaman üstte**
  seçeneğini kapatın ya da açın. Bu her Moonpool penceresi için geçerlidir, böylece hepsi aynı
  katmanda kalır.

## Günlükler nerede?

Bkz. [Günlükler](/tr/data/logs/).

## Yedekleme, sıfırlama veya kaldırma

Bkz. [Yedekleme ve kurtarma](/tr/data/backup-and-recovery/) ve
[Kaldırma](/tr/getting-started/install/#kaldırma).

## SSS

**Pencereyi kapatmak uygulamalarımı durdurur mu?**
Varsayılan olarak kapatmak Moonpool'dan çıkar ve Windows'ta çıkmak, başlattığı uygulamaları
durdurur. Pencereyi kapattığınızda Moonpool'un çalışmaya devam etmesi için **Kapatınca tepsiye
gizle** seçeneğini açın. Bkz. [Sistem tepsisi, kapatma ve küçültme](/tr/using/tray-and-closing/).

**Moonpool'u iki kez çalıştırabilir miyim?**
Klasör başına bir tane. Aynı kopyayı yeniden başlatmak penceresini geri getirir. Kurulu kopya
ve taşınabilir kopyalar yan yana çalışabilir. Bkz.
[Taşınabilir mod](/tr/data/portable-mode/#aynı-anda-birkaç-kopya).

**Moonpool eve telefon eder mi?**
Yalnızca güncellemeleri denetlemek için: başlangıçta (**Başlangıçta güncellemeleri denetle**
açıksa) ve **Güncellemeleri denetle** düğmesine bastığınızda GitHub'dan sürüm dosyasını
(`update.json`) alır. Her indirme, kullanılmadan önce Moonpool'un imza anahtarına karşı
doğrulanır.

**Komutlarımı hangi kabuk çalıştırır?**
Windows'ta `cmd /c`, Linux ve macOS'ta `$SHELL -c`.

**Gizli bilgileri nereye koymalıyım?**
`env` değerleri `apps.json` içinde düz metin olarak saklanır. Uygulamanızın kendi okuduğu bir
dosyayı ya da kullanıcı ortamınızda önceden ayarlı bir değişkeni tercih edin; başlatılan
uygulamalar bunu devralır.

**Denetim kanalı korumalı mı?**
Oturum açma ya da belirteç yoktur. Sizin adınıza çalışan herhangi bir süreç ona komut
gönderebilir. Linux ve macOS'ta soket yalnızca kullanıcınız tarafından okunabilir. Bkz.
[Güvenlik özellikleri](/tr/automation/overview/#güvenlik-özellikleri).
