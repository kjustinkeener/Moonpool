---
title: "WebView2 çalışma zamanı eksik: Windows'ta boş veya açılmayan Moonpool penceresini düzeltin"
description: "Windows'ta Moonpool'un penceresi hiç açılmıyor ya da boş kalıyorsa Microsoft Edge WebView2 Çalışma Zamanı eksik olabilir. Nasıl denetlenir ve nasıl kurulur."
---

Windows'ta Moonpool'un penceresi hiç açılmıyor ya da açılıp boş kalıyorsa olası neden eksik
bir Microsoft Edge WebView2 Çalışma Zamanı'dır. Moonpool bir Tauri uygulamasıdır ve
pencereleri WebView2 tarafından çizilen web sayfalarıdır.

WebView2, Windows 11 ve güncel Windows 10 ile birlikte gelir; bu yüzden çoğu bilgisayarda zaten
vardır. Eski ya da kırpılmış bir Windows 10'da veya kaldırılmış olduğu bir bilgisayarda eksik
olabilir. Moonpool'un kendi kaynak kodu bu durum için ayrılmış bir ileti göstermez; bu yüzden
burada alıntılanan bir hata metni yoktur: belirti, pencerenin görünmemesi ya da boş olmasıdır.

## Kurulu olup olmadığını denetleyin

PowerShell'de çalışma zamanının sürümünü kayıt defterinde arayın (ilk yol sistem genelindeki
kurulum, ikincisi kullanıcıya özel olandır):

```powershell frame="terminal"
Get-ItemProperty "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
Get-ItemProperty "HKCU:\Software\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" -Name pv
```

`120.0.2210.91` gibi bir sürüm numarası kurulu olduğu anlamına gelir. İkisi için de hata
alırsanız kurulu değildir.

## Kurun

Microsoft'un WebView2 sayfasından **Evergreen** WebView2 Çalışma Zamanı'nı indirin ("WebView2
Runtime download" diye aratın), kurulum programını çalıştırın, ardından Moonpool'u yeniden
başlatın. Evergreen çalışma zamanı kendini günceller.

## Kuruluysa ve pencere hâlâ boşsa

- Her Moonpool'dan sistem tepsisinden çıkın (ya da Görev Yöneticisi'nde `moonpool.exe` dosyasını
  sonlandırın) ve yeniden başlatın.
- Ulaşabiliyorsanız [Ayarlar](/tr/using/settings/) içinde **Hata ayıklama bilgilerini bir
  dosyaya kaydet** seçeneğini açın ve `moonpool.log` dosyasına bakın. Bkz. [Günlükler](/tr/data/logs/).
- Pencere açılıyor ama ekran dışındaysa bkz.
  [Pencere sorunları](/tr/support/troubleshooting/#pencere-sorunları).

## Ayrıca bakın

- [Windows](/tr/platforms/windows/#çalıştırmadan-önce)
- [Kurulum](/tr/getting-started/install/)
- [Sorun giderme](/tr/support/troubleshooting/)
