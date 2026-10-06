---
title: "Moonpool'u Windows'ta kullanın"
description: "Windows, Moonpool'un ana platformudur: nasıl kurulur ve ne bekleyeceğinizi bilmeniz için Windows ile Linux arasında nelerin farklı olduğunu gösteren bir tablo."
---

Windows, Moonpool'un ana platformudur. [Kurulum](/tr/getting-started/install/) sayfasında
anlatıldığı gibi kurun.

## Çalıştırmadan önce

- **SmartScreen.** `moonpool.exe` kod imzalı değildir; bu yüzden Windows ilk seferde "Windows
  protected your PC" (Türkçe: "Windows bilgisayarınızı korudu") gösterebilir. **More info**
  (Daha fazla bilgi), ardından **Run anyway** (Yine de çalıştır) seçeneğini seçin.
- **Antivirüs.** Kendini kopyalayan ve güncellemede kendinin yerine geçen yeni, imzasız bir
  exe antivirüsü tetikleyebilir. Sizinki `moonpool.exe` dosyasını engeller veya karantinaya
  alırsa `.moonpool` klasörü için izin verin.
- **WebView2.** Moonpool'un pencereleri Windows 11 ve güncel Windows 10 ile gelen Microsoft
  Edge WebView2'yi kullanır. Pencere boş kalırsa ya da hiç açılmazsa Microsoft'tan Evergreen
  WebView2 Çalışma Zamanı'nı kurun.

Daha fazlası: [Windows bilgisayarınızı korudu](/tr/support/windows-protected-your-pc/),
[WebView2 çalışma zamanı eksik](/tr/support/webview2-runtime-missing/) ve
[Bir betiği veya geliştirme sunucusunu Windows oturum açılışında otomatik başlatın](/tr/guides/start-app-at-windows-login/).

## Sistem tepsisi

Windows 11'de yeni bir sistem tepsisi simgesi çoğunlukla gizli simgeler alanına gider. Bulmak
için görev çubuğunun sağındaki **^** okuna tıklayın ve görünür kalması için simgeyi görev
çubuğuna sürükleyin.

## Komutlar

- Komutlar `cmd /c` üzerinden çalışır. `command` içinde iç içe çift tırnaktan kaçının;
  `cmd /c` onları bozar. Bir kabuğu açık bırakması gereken bir betik için betik kısmının
  çevresinde tırnak olmadan `pwsh -NoLogo -NoProfile -NoExit -Command <script and args>`
  kullanın.
- `processName`, `.exe` ile ya da `.exe` olmadan, büyük/küçük harf yok sayılarak eşleşir.
- Durdur, Moonpool'un başlattığı tüm süreç ağacını, ondan ayrılmış süreçler dahil sonlandırır.
- Docker Desktop uygulamaları `killMode` için `none` veya `command` gerektirir, asla `port`
  değil. Bkz. [Windows'ta Docker uygulamaları](/tr/apps/stop-and-restart/#windowsta-docker-uygulamaları).

## Platforma göre farklar

| | Windows | Linux |
| --- | --- | --- |
| Kurulum | Kendi kendini kuran `moonpool.exe` veya taşınabilir | AppImage, `.deb` veya RPM; kurulum kartı yok |
| Kendini güncelleme | Evet, kurulu ve taşınabilir | Yalnızca AppImage |
| Yapılandırma klasörü | `%USERPROFILE%\.moonpool\moonpool-config\` | `~/.config/Moonpool/` |
| Komutlar için kabuk | `cmd /c` | `$SHELL -c` |
| `processName` | Herhangi bir uzunluk, `.exe` isteğe bağlı, büyük/küçük harf duyarsız | 15 karakter veya daha kısa, tam harf düzeni |
| `processName` ile durdurma | Süreci ve alt süreçlerini sonlandırır | Yalnızca tam o ada sahip süreçleri sonlandırır |
| Denetim kanalı | Adlandırılmış kanal (named pipe) | Unix soketi |
| Pencere ekran görüntüleri (test) | Evet | Hayır |
| Program dosyasından simgeler | Evet | Hayır |
| Sistem tepsisi | Kutudan çıktığı gibi çalışır | AppIndicator gerekir; standart GNOME bir eklenti gerektirir |

Linux ayrıntıları [Linux](/tr/platforms/linux/) sayfasındadır.
