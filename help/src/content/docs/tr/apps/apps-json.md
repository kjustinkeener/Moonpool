---
title: "apps.json dosyasını düzenleme: konumu, yeniden yükleme ve kurtarma"
description: "Moonpool'un her yönetilen uygulama için okuduğu apps.json dosyasını bulun, uygulama düzenleyicide veya elle düzenleyin, yeniden yükleyin ve hatalı düzenlemeden kurtulun."
---

Moonpool'un yönettiği her uygulama `apps.json` içinde tek bir girdidir. Dosyayı uygulama düzenleyiciden
(Uygulama ekle ve Uygulamayı düzenle iletişim kutusu) ya da elle düzenleyebilirsiniz. İkisi de aynı dosyaya yazar.
Bazı araç sonuçları ve iletiler bu dosyaya manifest der.

## Yapılandırmanın bulunduğu yer

| Mod | Yapılandırma klasörü |
| --- | --- |
| Kurulu (Windows) | `%USERPROFILE%\.moonpool\moonpool-config\` |
| Taşınabilir | `moonpool.exe` dosyasının yanındaki `moonpool-config\` (`.moonpool\` klasörünün içinde) |
| Linux | `$XDG_CONFIG_HOME/Moonpool/`, yoksa `~/.config/Moonpool/` |

`apps.json` bu klasördedir; yanında şunlar bulunur:

| Öğe | Amaç |
| --- | --- |
| `apps.json.history\` | Son 10 geçerli `apps.json` dosyasının geri alma halkası. |
| `settings.json` | Uygulama ayarları. Bkz. [settings.json](/tr/data/settings-json/). |
| `cli-output\<id>\` | Uygulama başına oturum günlükleri. Bkz. [Günlükler](/tr/data/logs/). |
| `moonpool.log` | **Hata ayıklama bilgilerini bir dosyaya kaydet** açıkken tutulan hata ayıklama günlüğü. |
| `icons\` | İsteğe bağlı `<id>.png` (ayrıca `.ico`, `.svg`, `.jpg`, `.jpeg`, `.webp`) simge geçersiz kılmaları. |
| `state.json` | Birkaç saniyede bir yenilenen canlı durum anlık görüntüsü. |
| `dumps\` | `dump`, `read-config` ve `restore-config` komutlarının yazdığı dosyalar. |
| `mcp_seen.json` | Hangi uygulamalarda MCP yardımcısı görüldüğü. |
| `window-state.json` | Merkez penceresinin boyutu ve konumu. |
| `AI-README.md` | Yapay zekâ ajanları için kılavuz; her açılışta yeniden yazılır. |

Bunlardan hangilerinin yedekleneceği [Yedekleme ve kurtarma](/tr/data/backup-and-recovery/#yapılandırma-klasörü) sayfasındadır.

İlk çalıştırmada Moonpool `apps.json` dosyasına örnek girdiler yazar. Zaten var olan bir dosyanın
üzerine asla yazılmaz.

## Düzenleme

- **İletişim kutusu.** Kenar çubuğunun üstündeki **...** menüsünde **Uygulama ekle** seçeneğini kullanın. Bir
  uygulamayı değiştirmek için satırındaki kalemi kullanın ya da sağ tıklayıp **Düzenle** seçeneğini seçin.
  İletişim kutusu doğrular ve hemen kaydeder.
- **Elle.** Aynı menüdeki **apps.json dosyasını düzenle** dosyayı varsayılan düzenleyicinizde açar.
  Kaydedin, ardından menüden **Yeniden yükle** seçeneğini seçin (veya F5 ya da Ctrl+R tuşlarına basın).

Elle yapılan düzenlemeler yeniden yükleyene kadar algılanmaz. Yeniden yükleme yalnızca dosyayı okur; dosyayı
yeniden yazmaz.

İletişim kutusundan kaydetmek tüm dosyayı normalleştirilmiş, girintili bir biçimde yeniden yazar. Moonpool'un
bilmediği anahtarlar silinir ve JSON'da yorum yoktur; notlarınızı `note` alanında tutun.

## Biçim

Dosya, nesnelerden oluşan bir JSON dizisidir. Her girdide dört anahtar zorunludur: `id`, `name`,
`group`, `type`. Geri kalan her şey isteğe bağlıdır. Bkz. [Uygulama alanları](/tr/apps/fields/).

```json title="apps.json"
[
  { "id": "site", "name": "Site", "group": "Web apps", "type": "web",
    "cwd": "C:\\code\\site", "command": "npm run dev", "port": 5173,
    "url": "http://localhost:5173", "openBrowser": true }
]
```

Gruplar kenar çubuğunda dosyada ilk göründükleri sırayla yer alır.

## Yeniden yükleme ne yapar

Yeniden yükleme, Moonpool'un bellekteki listesini dosyanın içeriğiyle değiştirir. Başlat, Durdur ve Yeniden başlat
düğmelerine tıkladığınızda girdiyi o anda okur; bu yüzden düzenlenmiş bir `command`, `cwd`, `env` veya kill
ayarı, o uygulamayı bir sonraki başlatışınızda ya da yeniden başlatışınızda geçerli olur. Yeniden yükleme hiçbir şeyi
yeniden başlatmaz: zaten çalışan bir uygulama, başladığı ayarlarla çalışmaya devam eder.

## Doğrulama

Moonpool tüm dosyayı yüklenirken, her kaydetmede ve ajanın her yazışında doğrular.
Tek bir hatalı girdi tüm dosyanın reddedilmesine yol açar.

| Kural | Hata şunu içerir |
| --- | --- |
| Geçerli JSON değil, zorunlu bir anahtar eksik ya da bir değerin türü yanlış | JSON ayrıştırıcısının iletisi |
| `id` boş, `-` ile başlıyor veya harf, rakam, `.`, `_`, `-` dışında karakter içeriyor | `invalid id` |
| İki girdi aynı `id` değerini paylaşıyor | `duplicate app id` |
| `name` boş | `has an empty name` |
| `group` boş | `has an empty group` |
| `type` değeri `desktop`, `web`, `static` veya `cli` değil | `unknown type` |
| `port` değeri `0` (65535'ten büyük bir `port` ayrıştırılamaz) | `invalid port 0` |
| `url` içermeyen `static` girdi | `requires a url` |
| `command` içermeyen diğer her tür | `requires a command` |

Hatalar girdiyi sırasıyla belirtir, örneğin:

```text
apps.json entry 2 (site) requires a command
```

### id

`id`, girdinin kalıcı anahtarıdır. Günlük klasörünü ve simge dosyasını adlandırır; `moonpool.exe launch <id>`
komutuna ve ajanlara verdiğiniz değer de odur. Uygulama eklerken iletişim kutusu bunu addan türetir.
Adı küçük harfe çevirir, `a` ile `z` ve `0` ile `9` dışındaki her karakter dizisini tek bir `-` yapar ve
`-` karakterlerini iki uçtan keser. Sonuç boşsa `app` olur. `id` zaten alınmışsa `-2`, `-3` gibi
sonekler ekler. Sonrasında `id` değerini asla değiştirmez; yani bir uygulamayı yeniden adlandırmak `id` değerini korur. `Habit Tracker`
adı `habit-tracker` kimliğini alır.

## Dosya hatalıysa

- **Yeniden yüklemede**, doğrulamadan geçemeyen dosyaya dokunulmaz ve Moonpool son yüklenen listeyi tutar.
  Kenar çubuğunun üzerinde hatayı gösteren bir şerit ve dosyayı açan bir düğme belirir; liste kullanılabilir
  kalır ama soluklaşır. Bkz.
  [apps.json dosyasında hata olduğunda](/tr/using/hub-window/#appsjson-hata-içerdiğinde).
- **Başlangıçta**, bozuk bir dosya tutulacak liste olmadığı anlamına gelir; Moonpool hiç uygulama olmadan başlar
  ve şerit bunu söyler. Dosyayı düzeltip **Yeniden yükle** seçeneğini seçin ya da bir anlık görüntüyü geri yükleyin
  (aşağıda ya da `moonpool_restore_config` aracıyla).
- Her iki durumda da iletişim kutusundan kaydetme (ve yeniden adlandırma, silme, simge ayarlama) dosya yeniden
  yüklenene kadar reddedilir; böylece bozuk dosyanın üzerine asla yazılmaz. Dosyayı düzeltin ve
  **Yeniden yükle** seçeneğini seçin.
- **İletişim kutusundan, bir ajandan veya geri yüklemeden** gelen geçersiz bir değişiklik reddedilir ve diskteki
  dosya olduğu gibi kalır.

Moonpool, `apps.json` dosyasının son 10 iyi sürümünü `apps.json.history\` içinde tutar. Geri almanın
yolu [Yedekleme ve kurtarma](/tr/data/backup-and-recovery/#appsjson-dosyasını-geri-alma) sayfasındadır.
Belirtiler ve çözümler [Sorun giderme](/tr/support/troubleshooting/#appsjson-hata-içeriyor) sayfasındadır.

## Ajanlar

Bir yapay zekâ ajanı `apps.json` dosyasını dosyanın kendisi yerine Moonpool'un MCP araçlarıyla değiştirmelidir; böylece
eski veya geçersiz bir yazma reddedilir ve korumalı alandaki bir ajan hiçbir zaman özel bir kopyayı düzenlemez. Bkz.
[MCP araçları](/tr/automation/mcp-tools/#yapılandırma).
