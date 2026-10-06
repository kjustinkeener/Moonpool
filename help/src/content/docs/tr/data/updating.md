---
title: "Moonpool'u güncelleme ve başarısız bir güncellemeyi düzeltme"
description: "Moonpool'un güncellemeleri nasıl denetlediğini, indirdiğini ve uyguladığını, güncelleme şeridinin ne yaptığını, taşınabilir ve Linux kopyalarının nasıl güncellendiğini ve hata durumunda neler yapılacağını görün."
---

Moonpool kendini günceller. İndirilecek ayrı bir yükleyici ve tıklanacak bir sihirbaz yoktur.

## Güncellemeler nasıl gelir

Moonpool, projenin GitHub Releases sayfasından `update.json` (Linux'ta `linux-update.json`) dosyasını getirir,
sürümleri karşılaştırır ve yalnızca kesin olarak daha yeni olanı sunar. Şu durumlarda denetler:

- başlangıçta, [Ayarlar](/tr/using/settings/) içinde **Başlangıçta güncellemeleri denetle** kapalı değilse;
- Hakkında penceresinde **Güncellemeleri denetle** düğmesine her bastığınızda. Bu düğme daha yeni bir sürümü
  hemen kurar ve Moonpool'u yeniden başlatır. Aksi halde en son sürümü kullandığınızı söyler ya da hatayı gösterir.

Hakkında penceresi, çalıştırdığınız sürümü adın altında gösterir:

![Hakkında penceresinin üst kısmı: logo, ad (1) ve altındaki sürüm satırı](../../../../assets/screenshots/about-header.png)

1. Ad. Altındaki satır sürüm ve derleme tarihidir.

Her indirme, uygulanmadan önce Moonpool'un minisign imzalama anahtarına karşı doğrulanır; böylece kurcalanmış ya da
bozulmuş bir indirme reddedilir. Moonpool eski bir sürümü asla kurmaz.

## Güncelleme şeridi

Başlangıçta bulunan bir güncelleme, merkezin boş ekranında bir şerit olarak görünür:

```text
Moonpool {version} yayınlandı (sizdeki sürüm: {current}).
```

Şerit yalnızca hiçbir uygulama sekmesi açık değilken ve CLI paneli genişletilmişken görünür. Panel
daraltılmışsa, filtre kutusunun yanındaki ok onun yerine atar. Bir sekme açıkken hiçbir işaret yoktur. Şeridi
görmek için tüm sekmeleri kapatın (ve paneli genişletin) ya da Hakkında penceresindeki **Güncellemeleri
denetle** seçeneğini kullanın.

**İndir ve kur** düğmesine tıklayın; Moonpool kendini değiştirir ve yeniden başlar. Ya da şeridi x ile kapatın.

## Taşınabilir kopyalar

Taşınabilir bir kopya, kendi `.moonpool\` klasöründeki `moonpool.exe` dosyasını aynı şekilde günceller.
Her kopya kendi başına denetler ve güncellenir. Klasör yazılabilir olmalıdır; bu yüzden salt okunur bir bellekte
ya da paylaşımda duran bir kopya kendini güncelleyemez; daha yeni bir `moonpool.exe` dosyasını onun üzerine
elle kopyalayın.

## Linux

Yalnızca AppImage kendini günceller. AppImage dosyasını yerinde değiştirir; bu yüzden onu yazabileceğiniz bir
klasörde tutun. Bir `.deb` veya RPM kurulumu paket yöneticinizle güncellenir:
Moonpool'dan kurmaya çalışmak şu hatayla başarısız olur

```text
automatic updates are available for the AppImage only; update the .deb or RPM with your package manager
```

Bkz. [Linux](/tr/platforms/linux/#güncellemeler).

## Bir güncelleme başarısız olduğunda

Şerit nedeni gösterir ve düğme yeniden denemeniz için tekrar kullanılabilir olur:

```text
Güncelleme başarısız: <error>
```

| Hata şunu içerir | Olası neden | Ne yapmalı |
| --- | --- | --- |
| `download failed` | Bağlantı yok, bir proxy ya da GitMerkezin istekleri sınırlaması | Bekleyip yeniden deneyin ya da elle güncelleyin. |
| `signature verification FAILED - refusing to install` | İndirme bozuk veya değiştirilmiş | Yeniden deneyin. Başarısızlık sürerse Releases sayfasından elle güncelleyin. |
| `rename self aside` veya `write new exe` | Klasör salt okunur ya da bir antivirüs dosyayı tutuyor | Klasörü yazılabilir yapın ya da antivirüsünüzde `moonpool.exe` dosyasına izin verin, ardından yeniden deneyin. |
| `refusing to install ... not newer than current` | Sunulan sürüm daha yeni değil | Yapılacak bir şey yok. |

### Elle güncelleme

Moonpool'dan çıkın, `moonpool.exe` dosyasını projenin
[Releases sayfasından](https://github.com/kjustinkeener/Moonpool/releases) indirin ve eskisinin üzerine
kopyalayın: kurulu sürümde `%USERPROFILE%\.moonpool\moonpool.exe`, taşınabilir kopyada `.moonpool\`
klasörünüzdeki dosya. Yapılandırma klasörünüze dokunulmaz. Linux'ta AppImage dosyasını değiştirin ya da paket
yöneticinizi kullanın.

## Yardım da güncellenir

Bu yardım Moonpool'un içinde gelir; bu yüzden her program güncellemesi eşleşen yardımı da getirir.
Çevrimdışı kopya her zaman çalıştırdığınız sürümle eşleşir.

## Ayrıca bakın

- [Yenilikler](/tr/getting-started/whats-new/)
- [Ayarlar penceresi](/tr/using/settings/)
