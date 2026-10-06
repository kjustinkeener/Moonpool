---
title: "Moonpool ayarlarını değiştirin: Ayarlar ve Hakkında içindeki her seçenek"
description: "Moonpool Ayarlar ve Hakkında pencerelerindeki denetimlerin tam listesi, her birinin yazdığı settings.json anahtarı ve bir ayarın nasıl sıfırlanacağı."
---

**Ayarlar**'ı merkezin "..." menüsünden açın. Değişiklikler siz yaptıkça kaydedilir. Escape
pencereyi kapatır. Bu sayfa ayarların tam listesidir. Her biri gösterilen anahtar altında
`settings.json` içinde kaydedilir; dosyanın kendisi
[settings.json](/tr/data/settings-json/) sayfasında anlatılır.

![Ayarlar penceresi: sol sütunda açma/kapama düğmeleri ve kaydırıcılar, sağda günlük seçenekleri](../../../../assets/screenshots/settings-window.png)

## Bir denetimi sıfırlama

Yalnızca o ayarı varsayılanına sıfırlamak için herhangi bir onay kutusuna, kaydırıcıya ya da
sayı alanına sağ tıklayın. Her denetimin üzerine gelince çıkan ipucu bunu söyler. Dil ve Tema
seçicilerinde sıfırlama yoktur.

## Sol sütun

| Denetim | Anahtar | Varsayılan | Ne yapar |
| --- | --- | --- | --- |
| Dil | `locale` | Otomatik (sistem) | Moonpool'un kendi metinlerinin dili. Anında uygulanır. Bkz. [Temalar, dil ve saydamlık](/tr/using/themes-and-language/). |
| Tema | yok (tarayıcı depolaması) | Otomatik (sistem) | Renk teması. Düğme, her temanın önizlemesini içeren bir tema tarayıcısı açar; birine tıklamak onu anında uygular. Bkz. [Temalar, dil ve saydamlık](/tr/using/themes-and-language/). |
| Kapatınca tepsiye gizle | `closeToTray` | kapalı | Açık: pencereyi kapatmak Moonpool'u sistem tepsisine gizler. Kapalı: kapatmak çıkar. |
| Küçültünce tepsiye gizle | `minimizeToTray` | açık | Açık: küçültmek Moonpool'u sistem tepsisine gizler ve görev çubuğundan kalkar. Kapalı: görev çubuğuna küçültür. |
| Her zaman üstte | `alwaysOnTop` | kapalı | Her Moonpool penceresini diğer pencerelerin üstünde tutar. |
| Tepside göster | `showInTray` | açık | Sistem tepsisi simgesini görünür tutar. |
| Görev çubuğunda göster | `showInTaskbar` | açık | Görev çubuğu düğmesini görünür tutar. |
| CPU/bellek durum çubuğunu göster | `showStatusbar` | açık | Merkezin altında canlı CPU ve bellek çubuğu. |
| MCP süreçlerini göster | `showMcpProcesses` | açık | Bir uygulamanın MCP araçları kullanılırken, MCP sürecini kenar çubuğunda MCP alt satırı olarak gösterir. |
| Arka plan saydamlığı | `transparency` | %0 | 5'er adımla 0'dan 90'a kaydırıcı. Bkz. [Temalar, dil ve saydamlık](/tr/using/themes-and-language/#saydamlık). |
| Başlangıçta güncellemeleri denetle | `checkOnStartup` | açık | Açılışta GitHub'da daha yeni bir sürüm olup olmadığına bakar ve bulunursa bir şerit gösterir. Bkz. [Güncelleme](/tr/data/updating/). |

### Sistem tepsisi ve görev çubuğu kilitlenmesi

**Tepside göster** ve **Görev çubuğunda göster** seçeneklerinden en az biri açık kalmalıdır;
aksi halde gizli bir pencerenin geri dönüş yolu olmaz. Yalnızca biri açıkken, diğerini yeniden
açana kadar onay kutusu devre dışıdır.

## Sağ sütun: günlükler

| Denetim | Anahtar | Varsayılan | Ne yapar |
| --- | --- | --- | --- |
| Uygulama çıktı günlüklerini oturumlar arasında sakla | `cliLogging` | kapalı | Çalışan oturumun terminal çıktısı kendi sekmeleri için her zaman saklanır. Açık: eski oturumların günlükleri `cli-output\` altında diskte kalır ve saklama ayarıyla sınırlanır. Kapalı: o uygulama bir sonraki başlatıldığında silinir. |
| Uygulama başına günlük saklama | `logRetentionMb` | 10 MB | Her uygulamanın toplam günlükleri için üst sınır. En az 1. Yukarıdaki düğme kapalıyken devre dışıdır. Geçerli oturumun günlüğü sınıra dahil edilir ama sınır tarafından asla kesilmez veya silinmez. |
| Hata ayıklama bilgilerini bir dosyaya kaydet | `debugLogging` | kapalı | `apps.json` yüklemelerini, başlatmaları ve hataları `moonpool.log` dosyasına kaydeder. |

Her günlük grubunun altında, konumu gösteren bir yol alanı ve iki düğme bulunur:

- **Aç** klasörü dosya yöneticisinde açar (`cli-output\` için **CLI günlük klasörünü aç**, `moonpool.log` için **Günlüğü aç**).
- **Kopyala** yolu panoya koyar (**CLI günlük klasörünün yolunu kopyala**, **Günlük dosyasının yolunu kopyala**).

Günlük dosyası biçimleri, yeniden başlatma ayracı ve saklama kuralları
[Günlükler](/tr/data/logs/) sayfasındadır.

Bir onay kutusu kaydedilemezse pencerenin üstündeki kırmızı bir ileti bunu söyler ve onay kutusu
eski haline döner.

## Hakkında penceresi

**Hakkında**'yı "..." menüsünden açın.

![Sürüm satırı, bağlantılar, Güncellemeleri denetle ve Kapat düğmeleri bulunan Hakkında penceresi](../../../../assets/screenshots/about-window.png)

Şunları gösterir:

- Sürüm ve derleme tarihi.
- Proje sitesine, GitHub deposuna ve iletişim adresine bağlantılar.
- **Güncellemeleri denetle**. Daha yeni bir sürüm varsa indirir, doğrular ve kurar, sonra Moonpool'u yeniden başlatır. Yoksa en son sürümü kullandığınızı ya da denetim başarısız olduysa hatayı bildirir.
- Moonpool'un üzerine kurulduğu kitaplıkların ve yazarın künyesi.

Escape pencereyi kapatır. Hakkında penceresi tema, saydamlık ve dil ayarlarını canlı olarak izler.
