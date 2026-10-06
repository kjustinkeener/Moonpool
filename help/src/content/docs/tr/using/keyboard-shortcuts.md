---
title: "Moonpool klavye kısayolları, fare kısayolları ve yakınlaştırma"
description: "Moonpool merkezindeki tüm klavye ve fare kısayollarını ve metnin rahat okunması için arayüzü nasıl yakınlaştırıp uzaklaştıracağınızı görün."
---

## Klavye

| Tuşlar | Nerede | Ne yapar |
| --- | --- | --- |
| F5, Ctrl+R, Cmd+R | Merkez | `apps.json` dosyasını diskten yeniden yükler; menüdeki **Yeniden yükle** ile aynıdır. Sayfanın kendisi yenilenmez. |
| Esc | Bağlam menüsü | Kapatır. |
| Esc | Bir uygulamayı yeniden adlandırırken | Yeniden adlandırmayı iptal eder. |
| Esc | Ayarlar, Hakkında ve uygulama düzenleyicisi pencereleri | Pencereyi kapatır (düzenleyici değişiklikleri atmadan önce sorar). |
| Enter | Bir uygulamayı yeniden adlandırırken | Yeni adı kaydeder. |

## Fare

| Eylem | Nerede | Ne yapar |
| --- | --- | --- |
| Ctrl + tekerlek | Merkez | Arayüzü yakınlaştırır. |
| Metin seçme | Terminal | Seçimi kopyalar ve temizler. |
| Orta tıklama | Terminal | Yapıştırır. |
| Sağ tıklama | Kenar çubuğu satırı | Satır menüsünü açar. Bkz. [Kenar çubuğu ve menüler](/tr/using/sidebar-and-menus/). |

## Yakınlaştırma

Yakınlaştırmak için Ctrl tuşunu basılı tutup merkez üzerinde tekerleği çevirin. Yukarı çevirmek yakınlaştırır, aşağı çevirmek uzaklaştırır; her tekerlek olayında yaklaşık yüzde 10'luk adımlarla.

```text
Ctrl + wheel up      zoom in
Ctrl + wheel down    zoom out
```

- Aralık 0,5x ile 3x arasındadır.
- Pencere aynı çarpanla yeniden boyutlanır, böylece düzen 2x'te de 1x'teki kadar sıkı kalır. Sınıra ulaşıldığında pencere büyümeyi bırakır.
- Çarpan `settings.json` içinde `uiScale` olarak kaydedilir ve bir sonraki başlangıçta uygulanır. Kayıtlı pencere boyutu zaten yakınlaştırılmış boyuttur, bu yüzden yeniden ölçeklenmez. Bkz. [settings.json](/tr/data/settings-json/).

- Yakınlaştırma yalnızca merkez penceresine uygulanır. Ayarlar, Hakkında, uygulama
  düzenleyicisi ve Yardım kendi boyutlarını korur.

`uiScale` için bir Ayarlar denetimi ve sıfırlama tuşu yoktur. Normal boyuta dönmek için aynı
sayıda çentik geri çevirin ya da Moonpool'dan çıkın, `settings.json` içinde `uiScale` değerini
`1` yapın (veya anahtarı kaldırın) ve yeniden başlatın.

## Ayrıca bakın

- [Temalar, dil ve saydamlık](/tr/using/themes-and-language/)
- [Kenar çubuğu ve menüler](/tr/using/sidebar-and-menus/)
