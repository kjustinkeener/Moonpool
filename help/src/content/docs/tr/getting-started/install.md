---
title: "Moonpool'u Windows veya Linux'a kurun"
description: "Moonpool'u birkaç tıklamayla kurun, kurulu ya da taşınabilir modu seçin, Moonpool'u kur menü öğesini sonradan kullanın ve işiniz bitince temiz biçimde kaldırın."
---

Bu sayfa Windows içindir. Windows'ta Moonpool kendi kurulum programıdır: indirilen dosya tek
bir `moonpool.exe` dosyasıdır. Linux'ta kurulum kartı ya da taşınabilir seçici yoktur; bkz.
[Linux](/tr/platforms/linux/).

## Kurulu mod

İndirdiğiniz `moonpool.exe` dosyasını çalıştırın. İlk açılışta kurulum kartını gösterir.
Üç denetimi vardır: **Moonpool'u kur** düğmesi, bir **Masaüstüne kısayol ekle** onay kutusu
(varsayılan olarak açık) ve bir **Taşınabilir olarak kur** bağlantısı.

Kurulum, Moonpool'u kullanıcı profilinizin altındaki `.moonpool\` klasörüne kopyalar, bir
Başlat Menüsü kısayolu ekler (kutu işaretliyse masaüstüne de) ve Program Ekle/Kaldır'a bir
giriş kaydeder. Ardından kurulan kopyayı başlatır ve kapanır. İndirdiğiniz dosya olduğu yerde
kalır; silebilirsiniz. Bundan sonra Moonpool'u diğer uygulamalar gibi kısayoldan başlatın.

![Kurulum kartı: Moonpool'u kur düğmesi, masaüstü kısayolu onay kutusu, Taşınabilir olarak kur bağlantısı ve kurulum yolu](../../../../assets/screenshots/installer-window.png)

Moonpool'un ihtiyaç duyduğu her şey bu tek klasörün altında durur: program, yapılandırmanız
ve birlikte gelen yardım.

```text title="Installed layout"
%USERPROFILE%\.moonpool\
```

## Menüden Moonpool'u kur...

Windows'ta "..." menüsünde her iki modda da **Moonpool'u kur…** bulunur. Aynı kurulum kartını
açar. Taşınabilir bir kopyadan düzgün biçimde kurabilirsiniz. Kurulu bir kopyada **Moonpool'u
kur** devre dışıdır ("Zaten kurulu") ve **Taşınabilir olarak kur** kullanılabilir kalır.

## Kaldırma

Windows Program Ekle/Kaldır'ı (Yüklü uygulamalar) kullanın ya da kurulu kopyayı
`--uninstall` ile çalıştırın. PATH içinde değildir, bu yüzden tam yolunu verin:

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" --uninstall
```

Bu, Başlat Menüsü ve masaüstü kısayollarını, kayıt defteri girdisini ve tüm
`%USERPROFILE%\.moonpool` klasörünü, **yapılandırmanız dahil** (`apps.json`, ayarlar ve
günlükler) kaldırır. Yapılandırmanızı korumak istiyorsanız önce bu klasörü yedekleyin:

```text
%USERPROFILE%\.moonpool\moonpool-config
```

Çalışan her Moonpool kaldırma işleminin parçası olarak durdurulur.

## Taşınabilir mod

USB bellek ya da taşınabilir bir klasör mü tercih edersiniz? Kurulum kartında **Taşınabilir
olarak kur** seçeneğine tıklayın ve bir klasör seçin. Bkz. [Taşınabilir mod](/tr/data/portable-mode/).

## Sonraki adım

- [Windows bilgisayarınızı korudu](/tr/support/windows-protected-your-pc/): SmartScreen kurulum programını engellerse.
- [WebView2 çalışma zamanı eksik](/tr/support/webview2-runtime-missing/): pencere boş kalırsa.
- [İlk uygulamanız](/tr/getting-started/first-app/)
