---
title: "Moonpool'u yedekleme, apps.json dosyasını geri alma ve kurulumu kurtarma"
description: "Nelerin yedekleneceğini öğrenin, hatalı bir apps.json dosyasını geri alın, örnek uygulamalara sıfırlayın, kurulu bir yapıyı taşınabilir kopyaya taşıyın ve kaldırmanın neleri sildiğini görün."
---

Moonpool'un tuttuğu her şey iki yerdedir: yapılandırma klasörü ve panolar klasörü.
Her mod için yollar [Yapılandırmanın bulunduğu yer](/tr/apps/apps-json/#yapılandırmanın-bulunduğu-yer) bölümündedir.

## Yapılandırma klasörü

```text
moonpool-config\
  apps.json            your apps                              back up
  apps.json.history\   the last 10 good apps.json files       back up (optional)
  settings.json        app settings                           back up
  icons\               icon overrides, <id>.png and so on     back up
  cli-output\<id>\     session logs                           disposable
  moonpool.log         debug log                              disposable
  state.json           live status snapshot                   disposable
  dumps\               files written by dump and read-config  disposable
  mcp_seen.json        which apps had an MCP helper           disposable
  window-state.json    hub window size and position           disposable
  AI-README.md         rewritten at every launch              disposable
  webview\             the window's browser profile (Windows) disposable
```

Panolar klasörü `{MP_HOME}\dashboards` yoludur: kurulu sürümde `%USERPROFILE%\.moonpool\dashboards`,
taşınabilir sürümde `<your .moonpool folder>\dashboards` ve Linux'ta yapılandırma klasörünün içindeki
`dashboards/`. İçindeki kendinize ait her şeyi yedekleyin. İçindeki `examples` klasörü Moonpool'a aittir ve
güncellemede yeniden yazılır.

Tema, kopyalayabileceğiniz bir dosyada değil, pencerenin tarayıcı depolamasında tutulur. Bir yedekle birlikte
taşınmaz; geri yüklemeden sonra yeniden seçin.

## Yedekleme

1. Moonpool'dan çıkın; böylece hiçbir dosya yarım yazılmış kalmaz.
2. Yapılandırma klasöründen `apps.json`, `settings.json` ve `icons\` öğelerini, `dashboards\` içinden de kendi
   dosyalarınızı kopyalayın.

```powershell frame="terminal"
$cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
Copy-Item "$cfg\apps.json", "$cfg\settings.json" D:\backup\
Copy-Item "$cfg\icons" D:\backup\ -Recurse
```

Geri yüklemek için Moonpool'dan çıkın, dosyaları geri kopyalayın ve başlatın.

## apps.json dosyasını geri alma

Her başarılı kaydetme, ajan yazması ve geri yükleme ile içeriği değişmiş bulan her Yeniden yükle işlemi,
doğrulanmış `apps.json` dosyasını `apps.json.history\` içine kopyalar ve en yeni 10 tanesini tutar. Her dosya,
alındığı zamanla adlandırılır, örneğin `1767225600000.json`. `apps.json.bak` yoktur.

- **Elle.** Bir anlık görüntüyü `apps.json` üzerine kopyalayın, ardından **Yeniden yükle** seçeneğini seçin.

  ```powershell frame="terminal"
  $cfg = "$env:USERPROFILE\.moonpool\moonpool-config"
  Copy-Item "$cfg\apps.json.history\<snapshot>" "$cfg\apps.json"
  ```

- **Bir betikten.** `moonpool.exe restore-config` anlık görüntüleri listeler;
  `moonpool.exe restore-config 1` en yenisini geri yükler. Bkz.
  [Komut satırı](/tr/automation/command-line/).
- **Bir ajandan.** `moonpool_restore_config`. Bkz. [MCP araçları](/tr/automation/mcp-tools/#yapılandırma).

Hiçbir şey otomatik olarak geri yüklenmez.

## Bozuk dosya

- **apps.json.** Moonpool bozuk bir dosyanın üzerine asla yazmaz. Bkz.
  [Dosya hatalıysa](/tr/apps/apps-json/#dosya-hatalıysa).
- **settings.json.** Düzeltin ya da tüm ayarları sıfırlamak için silin, ardından Moonpool'u yeniden başlatın. Bkz.
  [settings.json](/tr/data/settings-json/#okuma-ve-onarım).

## Örneklere sıfırlama

Moonpool örnek uygulamalarını yalnızca `apps.json` yokken yazar. Baştan başlamak için Moonpool'dan çıkın (ya da
çalışır halde bırakın), `apps.json` dosyasını yeniden adlandırın veya silin, ardından Moonpool'u başlatın ya da
**Yeniden yükle** seçeneğini seçin. Örneklerle birlikte yeni bir `apps.json` yazılır.

## Kurulu sürümden taşınabilire

Yeni bir taşınabilir kopya örnek uygulamalarla başlar. Kendi uygulamalarınızı aktarmak için bkz.
[Taşınabilir mod](/tr/data/portable-mode/#yükleyiciden-taşınabilir-seçme). İsterseniz `icons\` ve
`settings.json` öğelerini de aynı şekilde kopyalayın.

## Kaldırma

Kurulu Moonpool'u kaldırmak, yapılandırma klasörü ve panolar dahil tüm `%USERPROFILE%\.moonpool` klasörünü
siler. Önce yedek alın. Bkz. [Kaldırma](/tr/getting-started/install/#kaldırma). Taşınabilir bir kopya,
`.moonpool\` klasörü silinerek kaldırılır.
