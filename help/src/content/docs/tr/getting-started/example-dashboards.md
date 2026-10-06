---
title: "Moonpool ile birlikte gelen örnek panoları deneyin"
description: "Birlikte gelen çevrimdışı örnek panoları açın, nerede bulunduklarını ve örnek uygulamaların onlara nasıl başvurduğunu görün, mevcut bir yapılandırmaya ekleyin."
---

Moonpool, programın içinde kendi kendine yeten bir pano takımıyla gelir. Tamamen çevrimdışı
çalışırlar; sunucu ya da CDN gerekmez.

| Pano | Nedir |
| --- | --- |
| CSV explorer | Bir CSV veya TSV dosyası bırakın; sütunları profiller ve verileri çizer. |
| JSON explorer | JSON bırakın (diziler, iç içe nesneler veya eşlemeler). |
| Excel explorer | Bir `.xlsx` veya `.xls` dosyası bırakın, çevrimdışı ayrıştırılır. |
| Moonpool Docs | Çevrimdışı bir Markdown belge tarayıcısı. |

## Nerede bulunurlar

Moonpool başlangıçta panoları `{MP_HOME}\dashboards\examples` konumuna yazar:

| Mod | Klasör |
| --- | --- |
| Kurulu (Windows) | `%USERPROFILE%\.moonpool\dashboards\examples` |
| Taşınabilir | `<your .moonpool folder, the one holding moonpool.exe>\dashboards\examples` |
| Linux | `~/.config/Moonpool/dashboards/examples` (veya `$XDG_CONFIG_HOME/Moonpool/dashboards/examples`) |

`examples` klasörü Moonpool'a aittir: Moonpool her güncellendiğinde değiştirilir, bu yüzden
orada yaptığınız düzenlemeler kaybolur. Bir panoyu özelleştirmek için klasörünü ve ortak
`_lib` klasörünü `dashboards` içine kopyalayın ve uygulamanızı kopyaya yönlendirin. Moonpool
`dashboards` içindeki başka hiçbir şeyi değiştirmez.

0.3.16 öncesi sürümler örnekleri doğrudan `dashboards` içine yazıyordu. Bu kopyalar olduğu
yerde kalır ve artık güncelleme almaz; onlara işaret eden uygulamalar çalışmaya devam eder.
Güncel sürümleri almak için `url` değerlerini aşağıdaki `dashboards/examples/...` yoluna
değiştirin.

## Uygulamalar onlara nasıl başvurur

Her biri, `{MP_HOME}` üzerine sabitlenmiş bir `file:///` URL'si olan `static` türünde bir
uygulamadır:

```text
file:///{MP_HOME}/dashboards/examples/csv/index.html
```

`{MP_HOME}` kurulum klasörüne, taşınabilir modda ise paket klasörüne çözülür; böylece giriş
taşınmış bir paketten de çalışır. `file://` URL'lerine izin verilir. Bkz.
[Yollar ve ortam](/tr/apps/paths-and-environment/).

## Örnek uygulamalar yalnızca ilk çalıştırmada görünür

Örnek girdiler `apps.json` dosyasına yalnızca henüz yapılandırma dosyası yokken yazılır.
Zaten bir `apps.json` dosyanız varsa pano girdilerini kendiniz ekleyin ("..." menüsünde
**apps.json dosyasını düzenle**, ardından **Yeniden yükle**). Şu dördünü en üstteki dizinin
içine, diğer girdilerinizden virgülle ayırarak ekleyin:

```jsonc title="apps.json (excerpt)"
{
  "id": "csv-explorer",
  "name": "Sample CSV Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/csv/index.html",
  "openBrowser": true
},
{
  "id": "json-explorer",
  "name": "Sample JSON Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/json/index.html",
  "openBrowser": true
},
{
  "id": "xlsx-explorer",
  "name": "Sample Excel Explorer",
  "group": "Dashboards",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/xlsx/index.html",
  "openBrowser": true
},
{
  "id": "docs-browser",
  "name": "Moonpool Docs",
  "group": "Docs",
  "type": "static",
  "url": "file:///{MP_HOME}/dashboards/examples/docs/index.html",
  "openBrowser": true
}
```

Alanların anlamları [Uygulama alanları](/tr/apps/fields/) sayfasındadır.

## Ayrıca bakın

- [Örnekler](/tr/apps/examples/): kopyalayabileceğiniz daha eksiksiz girdiler.
- [Uygulama türleri](/tr/apps/types/#static)
