---
title: "Moonpool'u sistem tepsisinde tutun: kapatma, küçültme ve çıkış davranışı"
description: "Sistem tepsisi simgesinin, kapat düğmesinin, küçültmenin ve Çıkış'ın ne yaptığını denetleyin, pencereyi her zaman üstte tutun ve hem tepsiyi hem görev çubuğunu gizlemekten kaçının."
---

## Sistem tepsisi simgesi

| Eylem | Sonuç |
| --- | --- |
| Sol tıklama | Merkez penceresini gösterir (küçültülmüşse veya gizliyse geri yükler). |
| Sağ tıklama | Yalnızca **Moonpool'u göster** ve **Çıkış** içeren menü (sizin dilinizde). |

Birden çok Moonpool kopyası çalışırken her birinin kendi sistem tepsisi simgesi vardır. İpucu,
hangi kopya olduğunu söyler. Bkz. [Taşınabilir mod](/tr/data/portable-mode/#aynı-anda-birkaç-kopya).

## Çıkış

**Çıkış**, Moonpool'dan çıkar ve Windows'ta Moonpool'un başlattığı her uygulamayı, alt süreçleri
dahil durdurur. Moonpool onları görmeden önce zaten çalışan uygulamalara ("Moonpool tarafından
yönetilmiyor" ile çalışıyor gösterilenler) dokunulmaz. Linux ve macOS'ta çıkmak, başlatılan
uygulamaları güvenilir biçimde durdurmaz.

## Kapatma ve küçültme

Kapat düğmesi varsayılan olarak Moonpool'dan çıkar (`closeToTray` değeri `false`). Ayarlar'da
**Kapatınca tepsiye gizle** seçeneğini açarsanız kapatmak pencereyi bunun yerine sistem
tepsisine gizler. Moonpool çalışmaya devam eder ve sistem tepsisi simgesi ya da **Moonpool'u
göster** onu geri getirir.

**Küçültünce tepsiye gizle** (`minimizeToTray`, varsayılan açık), pencere küçültüldüğünde onu
sistem tepsisine gizler ve görev çubuğundan kalkar. Her zamanki gibi görev çubuğuna küçültmek
için kapatın.

![Ayarlar: Kapatınca tepsiye gizle ve Küçültünce tepsiye gizle (1) ile Arka plan saydamlığı kaydırıcısı (2)](../../../../assets/screenshots/settings-tray-and-transparency.png)

1. **Kapatınca tepsiye gizle** ve **Küçültünce tepsiye gizle**.
2. **Arka plan saydamlığı**. Bkz. [Temalar, dil ve saydamlık](/tr/using/themes-and-language/#saydamlık).

## Sistem tepsisi ve görev çubuğu kilitlenmesi

**Tepside göster** ve **Görev çubuğunda göster**, sistem tepsisi simgesinin ve görev çubuğu
düğmesinin görünür olup olmadığını denetler. En az biri açık kalmalıdır; aksi halde gizli bir
pencerenin geri dönüş yolu olmaz. Yalnızca biri açıkken, diğerini yeniden açana kadar onay
kutusu devre dışıdır.

## Her zaman üstte

Ayarlar'daki **Her zaman üstte**, her Moonpool penceresini (merkez, Ayarlar, Hakkında,
uygulama düzenleyicisi, tema tarayıcısı, kurulum programı ve Yardım) diğer pencerelerin üstünde
tutar. Varsayılan olarak kapalıdır.

## Ayrıca bakın

- [Windows'ta bir npm geliştirme sunucusunu arka planda çalıştırın](/tr/guides/run-npm-dev-server-in-background-windows/)
- [Bir betiği veya geliştirme sunucusunu Windows oturum açılışında otomatik başlatın](/tr/guides/start-app-at-windows-login/)
- [Ayarlar penceresi](/tr/using/settings/)
- [Merkez penceresi](/tr/using/hub-window/)
