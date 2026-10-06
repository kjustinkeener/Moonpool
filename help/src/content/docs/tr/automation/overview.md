---
title: "Moonpool'u betikler ve yapay zekâ ajanlarıyla otomatikleştirme"
description: "Çalışan bir Moonpool'u betiklerden ve yapay zekâ ajanlarından yönetmenin üç yolu (MCP, komut satırı ve denetim komutları), aralarındaki ilişki ve her birinin neyi değiştirebildiği."
---

Moonpool, penceresine dokunmadan yönetilebilir. Hepsi aynı yerleşik Moonpool tarafından (burada merkez olarak
adlandırılan sistem tepsisi örneği) sunulan üç yüzey vardır.

Her Moonpool kopyası kendi merkezidir: kurulu olan ve her taşınabilir kopya bağımsız çalışır ve her birinin kendi
denetim kanalı vardır. Bir yüzey her zaman kullandığı `moonpool.exe` dosyasının ait olduğu kopyaya ulaşır.
Bkz. [Taşınabilir mod](/tr/data/portable-mode/#aynı-anda-birkaç-kopya).

| Yüzey | Nedir | Başvuru |
| --- | --- | --- |
| MCP sunucusu | `moonpool.exe mcp`, bir yapay zekâ ana bilgisayarının başlattığı stdio [MCP](https://modelcontextprotocol.io) sunucusu. | [MCP kurulumu](/tr/automation/mcp-setup/), [MCP araçları](/tr/automation/mcp-tools/) |
| Komut satırı | `moonpool.exe <verb> [args]`. Aynı kopyanın ikinci bir çalıştırması komutu denetim kanalı üzerinden merkezina iletir ve çıkar. | [Komut satırı](/tr/automation/command-line/) |
| Denetim kanalı | Windows'ta adlandırılmış bir kanal, `\\.\pipe\moonpool` (taşınabilir kopya için `\\.\pipe\moonpool-<id>`), Linux ve macOS'ta bir Unix soketi; satır başına bir JSON isteğiyle konuşur. | [Denetim komutları](/tr/automation/control-verbs/) |

## Birbirleriyle ilişkileri

- Merkez her şeye sahiptir: uygulamaları başlatma, oturum günlükleri, `apps.json`.
- MCP sunucusu merkezin bir istemcisidir, ikinci bir kopyası değildir. Araç çağrılarının çoğu denetim kanalı
  üzerinden merkeze iletilir ve yanıt araç sonucu olarak geri gelir. İstisnalar: `moonpool_bootup_launcher`
  `moonpool.exe` dosyasını kendisi başlatır; `moonpool_app_output` ve yapılandırma araçları merkezden bir dosya
  yazmasını ister ve sonra onu okur; `moonpool_launcher_paths`, MCP sürecinin kendi yollarını merkezinkilere ekler.
- Bir merkezin çalışıp çalışmadığına bir süreç aranarak değil, o kanala ping atılarak karar verilir. Yanıt veren
  bir merkez çalışıyordur; eksik bir kanal veya soket çalışmadığı anlamına gelir.
- Her yüzey pencereyle aynı işleyicileri çalıştırır; bu yüzden bir komut, karşılık gelen tıklamanın yaptığını yapar.
- Çalışan bir merkez yoksa, `moonpool_list_apps` dahil, onun üzerinde işlem yapan araçlar
  "Moonpool is not running" ile reddeder. Eski liste yoktur. `moonpool_bootup_launcher` onu başlatır.
  Bir şey kanalı tutuyor ama birkaç saniye içinde yanıt vermiyorsa hata, bir Moonpool sürecinin takılmış
  olabileceğini söyler.
- MCP sunucusu artık denetim kanalından önceki bir merkez sürümünü yönetmeye geri dönmez. O kopyayı güncelleyin
  ya da çıkıp yeniden başlatın.

## Neler değişiklik yapabilir

| Değiştirebildiği | Yüzeyler |
| --- | --- |
| Bir uygulamayı başlatma, durdurma veya yeniden başlatma | MCP, komut satırı, kanal |
| `apps.json` dosyasını yeniden yazma | MCP (`moonpool_write_config`, `moonpool_restore_config`), komut satırı, kanal |
| Moonpool'dan çıkma | MCP (`moonpool_shutdown_launcher`), komut satırı (`quit`), kanal |
| Bir uygulamanın MCP yardımcı sürecini sonlandırma | MCP (`moonpool_stop_mcp_server`), kanal (`stop-mcp`) |
| `apps.json` dosyasını yeniden yükleme, simgeleri yeniden getirme, pencereyi gösterme | MCP (`moonpool_reload_config`, `moonpool_refresh_app_icons`, `moonpool_raise_launcher`), komut satırı (`reload`, `refresh-icons`, `show`), kanal |
| Bir pencere veya terminal sekmesi açma | kanal (`open-window`) |
| Hatırlanan MCP yardımcısı görülme kayıtlarını temizleme | MCP (`moonpool_reset_mcp_seen`), kanal (`reset-mcp-seen`) |

Salt okunur araçlar: `moonpool_list_apps`, `moonpool_app_output`, `moonpool_read_config`,
`moonpool_launcher_paths`, `moonpool_window_state`, `moonpool_screenshot`.

## Güvenlik özellikleri

- **Yapılandırma yazmaları korumalıdır.** Bir yazma, son okumadan gelen sürüm belirtecini taşımalıdır; eski bir
  belirteç reddedilir ve yeni `apps.json` herhangi bir şey yazılmadan önce doğrulanır. Reddedilen bir yazma
  `apps.json` dosyasına dokunmaz. Bkz. [MCP araçları](/tr/automation/mcp-tools/#yapılandırma).
- **Uygulama kimlikleri kısıtlıdır.** MCP sunucusu yalnızca harf, rakam, `.`, `_` ve `-` kabul eder ve başta
  asla `-` kabul etmez; böylece bir kimlik komut satırı bayrağı olarak okunamaz.
- **Ekran görüntüleri yalnızca Moonpool'a aittir.** `moonpool_screenshot`, Moonpool'un kendi altı penceresinden
  birini (`main`, `settings`, `about`, `installer`, `editor`, `help`, `themes`) yakalar; ekranı veya başka bir
  uygulamayı asla yakalamaz. PNG bellekte oluşturulur ve satır içi döndürülür; Moonpool onu bir dosyaya kaydetmez.
- **Kanalda kimlik doğrulama yoktur.** Moonpool denetim kanalına veya sokete bir oturum açma ya da belirteç
  eklemez. Onu açabilen herhangi bir süreç komut gönderebilir. Linux ve macOS'ta soket dosyası `0600` kipiyle
  oluşturulur; yani yalnızca kendi kullanıcınız açabilir.
- **Korumalı alandaki ana bilgisayarlar algılanır.** MCP sunucusu paketlenmiş (Store/MSIX) bir korumalı alanın
  içinde çalıştığını, yani Moonpool'un dosyalarının özel bir kopyasını gördüğünü anlarsa, dosya okuyan veya yazan
  araçlar (`moonpool_app_output`, `moonpool_read_config`, `moonpool_write_config`, `moonpool_restore_config`)
  eski veri yerine nedenini açıklayan bir hata döndürür. Yalnızca denetim kanalını kullanan araçlar engellenmez.
  Bkz. [MCP kurulumu](/tr/automation/mcp-setup/#korumalı-alandaki-ana-bilgisayarlar).

## Platform

Denetim kanalı her platformda vardır: Windows'ta adlandırılmış bir kanal, Linux ve macOS'ta bir Unix soketi
(konumu [Denetim komutları](/tr/automation/control-verbs/#dinlediği-yer) sayfasında). Yalnızca
`screenshot` (ve dolayısıyla `moonpool_screenshot`) Windows'a özgüdür; Linux ve macOS'ta
"not supported on this platform" döndürür. Komut satırı komutları her platformda çalışır.

## Ayrıca bakın

- [Yapay zekâ ajanları: hızlı başlangıç](/tr/automation/quick-start/)
- [MCP kurulumu](/tr/automation/mcp-setup/)
