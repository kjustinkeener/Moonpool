---
title: "Moonpool sürüm notları ve son değişiklikler"
description: "Son Moonpool sürümlerinde nelerin değiştiğini, çalıştırmak için gereksinimleri ve GitHub'daki tam sürüm notlarını nerede bulacağınızı görün."
---

Her sürümün tam notları projenin
[Sürümler sayfasında](https://github.com/kjustinkeener/Moonpool/releases) bulunur. Bu yardım
Moonpool'un içinde gelir, bu yüzden her zaman çalıştırdığınız sürümü anlatır. Moonpool
kendini günceller; bkz. [Güncelleme](/tr/data/updating/).

## 0.3.18

- **Güncelleme artık "Erişim engellendi" hatasıyla başarısız olmuyor**; önceki güncellemeden
  önce başlatılmış bir Moonpool MCP sunucusu hâlâ çalışıyor olsa bile.

## 0.3.17

- **14 dilde yardım.** Yardım, Moonpool'un dilinde açılır: İngilizce, Almanca, İspanyolca,
  Fransızca, İtalyanca, Felemenkçe, Lehçe, Brezilya Portekizcesi, Rusça, Türkçe, Japonca, Korece
  ve Basitleştirilmiş ile Geleneksel Çince.
- **Yeni kılavuzlar ve destek sayfaları:** geliştirme sunucuları, bağlantı noktaları, oturum
  açılışında başlatma, MCP ajanları, Python betikleri ve sık görülen hata iletileri.
- **`mcpProcessName`.** Bir uygulamanın MCP sunucusunun işlem adı için joker karakterli desen;
  farklı adla çalışan sunucular içindir. Bkz. [mcpProcessName](/tr/apps/fields/#mcpprocessname).
- **macOS artık desteklenmiyor.** macOS sürümü yok. Windows ve Linux için bir şey değişmiyor.

## 0.3.16

- **Aynı anda birden çok Moonpool.** Kurulu Moonpool ve istediğiniz sayıda taşınabilir kopya,
  klasör başına bir tane olmak üzere yan yana çalışabilir; her birinin kendi uygulamaları,
  sistem tepsisi simgesi ve denetim kanalı vardır. Bkz.
  [Taşınabilir mod](/tr/data/portable-mode/#aynı-anda-birkaç-kopya).
- **Tema tarayıcısı.** Her biri kendi renkleriyle önizlenen 68 tema. Bkz.
  [Temalar, dil ve saydamlık](/tr/using/themes-and-language/).
- **Çalıştırılabilir örnekler.** Yeni bir `apps.json`, hepsi olduğu gibi çalışan örnek
  uygulamalar içerir. Örnek panolar artık Moonpool ile güncellenen, uygulamaya ait bir
  `dashboards/examples` klasöründe durur. Bkz. [Örnek panolar](/tr/getting-started/example-dashboards/).
- **apps.json hataları gösterilir.** Kenar çubuğunun üstündeki bir şerit hatayı gösterir ve
  başarısız bir Yeniden yükle, son yüklenen listeyi korur. Bkz.
  [apps.json hata içerdiğinde](/tr/using/hub-window/#appsjson-hata-içerdiğinde).
- **Linux'ta denetim kanalı**, bir Unix soketi üzerinden, ayrıca `list` fiili. Bkz.
  [Denetim fiilleri](/tr/automation/control-verbs/).
- Hakkında penceresi ve uygulama düzenleyicisi tema ve dil değişikliklerini canlı olarak
  izler. **Moonpool'u kur...** menü öğesi Windows dışında gizlenir.

## 0.3.15

- Yeniden başlatılan bir uygulama önceki çıktısını, tarihli bir "yeniden başlatıldı"
  ayracıyla korur. Bkz. [Terminal sekmeleri](/tr/using/terminal-tabs/#yeniden-başlatma).
- Her uygulamanın kendi `cli-output` klasörü vardır, bu yüzden günlük temizleme başka bir
  uygulamanın günlüklerine asla dokunmaz.
- `killMode` ve `stopCommand` uygulama düzenleyicisinde yer alır. Bkz.
  [Durdurma ve yeniden başlatma](/tr/apps/stop-and-restart/).
- Başlatılan uygulamalar artık Moonpool'un kendi WebView2 profilini devralmaz.

## 0.3.14

- Oturum günlükleri, uygulama başına bir boyut sınırıyla oturumlar arasında saklanabilir. Bkz.
  [Günlükler](/tr/data/logs/).
- Ayarlar'da günlük klasörleri için Aç ve Kopyala düğmeleri.
- Yardım penceresinin başlık çubuğunda düzeltmeler.

## Gereksinimler

- WebView2 bulunan Windows 10 veya 11 (bkz. [Windows](/tr/platforms/windows/)).
- WebKitGTK 4.1 ve bir AppIndicator kitaplığı bulunan Linux (bkz. [Linux](/tr/platforms/linux/)).
