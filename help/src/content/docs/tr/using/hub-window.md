---
title: "Moonpool merkez penceresinde yolunuzu bulun"
description: "Moonpool merkezinin turu: kenar çubuğu, terminal sekmeleri, durum çubuğu, menü, apps.json hata şeridi ve boyutunu ile konumunu nasıl hatırladığı."
---

![Üç uygulamanın çalıştığı merkez penceresi: iki çalışan web uygulaması satırı (1), sekme şeridi (2), etkin uygulamanın canlı çıktısı (3) ve durum çubuğu (4)](../../../../assets/screenshots/hub-window.png)

1. Çalışan uygulamalardan ikisi: yanan bir durum noktası ve oynat yerine bir durdur düğmesi.
2. Sekme şeridi; açılan her uygulama için bir sekme, etkin sekme vurgulu.
3. Etkin uygulamanın canlı çıktısı.
4. CPU ve bellek durum çubuğu.

## Düzen

| Alan | İçeriği |
| --- | --- |
| Başlık çubuğu | Simge durumuna küçült, ekranı kapla ve kapat. |
| Kenar çubuğu | Filtre kutusu, **...** menüsü ve `group` alanına göre gruplanmış uygulamalarınız. Bkz. [Kenar çubuğu ve menüler](/tr/using/sidebar-and-menus/). |
| CLI paneli | Açılan her uygulama için bir terminal sekmesi. Bkz. [Terminal sekmeleri](/tr/using/terminal-tabs/). |
| Durum çubuğu | Alt kenar boyunca canlı CPU ve bellek. |

Kenar çubuğunu yeniden boyutlandırmak için kenar çubuğu ile CLI paneli arasındaki ayırıcıyı sürükleyin.

## Durum çubuğu

![Durum çubuğu: solda çekirdek başına CPU çubukları, sağda bellek çubuğu](../../../../assets/screenshots/status-bar.png)

Durum çubuğu her CPU çekirdeği için ince bir çubuk gösterir (üzerine gelince "Çekirdek başına
CPU kullanımı"), ardından `used/total GB` etiketli bir bellek çubuğu gelir. Ayarlar'da
**CPU/bellek durum çubuğunu göster** ile kapatın (`showStatusbar`; bkz. [Ayarlar penceresi](/tr/using/settings/)). Değişiklik hemen uygulanır.

## ... menüsü

Filtre kutusunun solundaki **...** düğmesi menüyü açar.

![Kenar çubuğunun üstünde ... menü düğmesi (1) ve Uygulamaları filtrele kutusu (2)](../../../../assets/screenshots/sidebar-filter-and-menu.png)

1. **...** menü düğmesi.
2. **Uygulamaları filtrele...** kutusu.

| Öğe | Ne yapar |
| --- | --- |
| Uygulama ekle | Uygulama düzenleyicisini açar. Bkz. [Uygulama ekleme](/tr/apps/add-an-app/). |
| apps.json dosyasını düzenle | `apps.json` dosyasını elle düzenlemek için varsayılan düzenleyicinizde açar. |
| Yeniden yükle | `apps.json` dosyasını diskten yeniden okur (ayrıca F5, bkz. [Kısayollar ve yakınlaştırma](/tr/using/keyboard-shortcuts/)). |
| Ayarlar | Ayarlar penceresini açar. |
| Yardım | Bu yardımı açar. |
| Hakkında | Sürümü ve güncelleme denetimini içeren Hakkında penceresini açar. |
| Moonpool'u kur… | Yalnızca Windows. Uygulamayı kurmak ya da taşınabilir bir kopya yapmak için kurulum penceresini açar. Bkz. [Kurulum](/tr/getting-started/install/) ve [Taşınabilir mod](/tr/data/portable-mode/). |

### Bağlantı noktası çakışması uyarısı

`apps.json` içindeki iki uygulama aynı `port` değerini kullanıyorsa menünün altında bir uyarı satırı belirir, örneğin:

```text
bağlantı noktası 3000: App A / App B
```

Tam cümle için üzerine gelin. Çakışmayı `apps.json` içinde veya uygulama düzenleyicisinde giderin; hiçbir bağlantı noktası paylaşılmadığında satır kaybolur.

## apps.json hata içerdiğinde

Yeniden yükle (veya F5), `apps.json` dosyasının artık ayrıştırılmadığını ya da doğrulanmadığını
bulursa Moonpool zaten sahip olduğu listeyi korur. Kenar çubuğunun üstündeki bir şerit
"apps.json dosyasında hata var; en son yüklenen liste gösteriliyor." der ve ardından hatayı
gösterir (tam metin için üzerine gelin). Aşağıdaki liste soluktur ama yine de çalışır, bu
yüzden uygulamaları her zamanki gibi başlatıp durdurabilirsiniz. Şeritteki **apps.json
dosyasını düzenle** dosyayı açar; düzeltin ve **Yeniden yükle** seçin, şerit kaybolur.

Dosya yeniden yüklenene kadar Moonpool, uygulama düzenleyicisinden, yeniden adlandırmadan,
silmeden veya simge ayarlamadan gelen değişiklikleri kaydetmez; böylece bozuk bir dosyanın
üzerine asla yazılmaz.

Dosya Moonpool başladığında zaten bozuksa korunacak önceki bir liste yoktur: şerit hiçbir
uygulamanın yüklenmediğini söyler ve kenar çubuğu boştur. Dosyayı düzeltip yeniden yükleyin ya
da yakın zamanlı iyi bir kopyaya geri dönün (bkz.
[Dosya bozuksa](/tr/apps/apps-json/#dosya-hatalıysa)).

## Boş ekran

Hiçbir sekme açık değilken CLI paneli "Başlatmak için soldaki listeden bir uygulama seçin."
gösterir. Ayrıca yalnızca hiçbir sekme açık değilken görünen iki şeyi barındırır:

- Başlangıçta daha yeni bir sürüm bulunduğunda **güncelleme şeridi**. Bkz.
  [Güncelleme](/tr/data/updating/).
- **İstemi kopyala**; uygulamalarınızı kurmayı bir yapay zeka aracısına devreden hazır bir
  istem. Bkz. [Yapay zeka aracıları: hızlı başlangıç](/tr/automation/quick-start/#i̇stemi-kopyala).

Sistem tepsisi simgesi, kapatma, küçültme, Çıkış ve her zaman üstte seçeneği
[Sistem tepsisi, kapatma ve küçültme](/tr/using/tray-and-closing/) sayfasındadır.

## Boyut, konum ve ekranı kaplama durumu

Moonpool, merkez penceresinin boyutunu, konumunu ve ekranı kaplama durumunu çalışmalar arasında
hatırlar. İlk çalıştırma, Windows'un seçtiği konumda 1200x780 boyutunda açılır.

Kayıtlı konum artık bağlı hiçbir ekranda değilse (örneğin çıkarılmış bir monitör), konum yok
sayılır ve kayıtlı boyut varsayılan konumda kullanılır. Dosya, yapılandırma klasöründeki
`window-state.json` dosyasıdır (bkz.
[Yapılandırma nerede durur](/tr/apps/apps-json/#yapılandırmanın-bulunduğu-yer)).

Kenar çubuğu genişliği ve CLI panelinin daraltılıp daraltılmadığı da hatırlanır.
