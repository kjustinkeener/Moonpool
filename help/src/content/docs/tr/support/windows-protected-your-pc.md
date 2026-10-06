---
title: "Windows bilgisayarınızı korudu: Moonpool kurulum programını yine de çalıştırın (SmartScreen)"
description: "moonpool.exe dosyasını çalıştırdığınızda Windows SmartScreen \"Windows protected your PC\" gösterir. Neden çıkar, More info ardından Run anyway nasıl seçilir ve önce nelere bakılmalı."
---

İndirdiğiniz `moonpool.exe` dosyasını çalıştırdığınızda Windows, **Windows protected your PC**
(Türkçe Windows'ta: "Windows bilgisayarınızı korudu") başlıklı mavi bir kutu gösterebilir;
kutuda şu satır yer alır: "Microsoft Defender SmartScreen prevented an unrecognized app from
starting. Running this app might put your PC at risk." (Microsoft Defender SmartScreen,
tanınmayan bir uygulamanın başlatılmasını engelledi. Bu uygulamayı çalıştırmak bilgisayarınızı
riske atabilir.)

## Neden çıkar

SmartScreen, yeni olan ya da birçok bilgisayarda çalıştığını görmediği programlar hakkında
uyarır. `moonpool.exe` kod imzalı değildir; bu yüzden Windows'un güvenebileceği bir yayımcı
yoktur ve uyarıyı ilk seferde gösterebilir. Bu bir itibar denetimidir, dosyanın kötü amaçlı
olduğu yönünde bir tespit değildir.

## Ne yapmalı

1. Kutuda **More info** (Daha fazla bilgi) seçeneğine tıklayın. Yayımcı "Unknown publisher"
   (Bilinmeyen yayımcı) olarak görünür.
2. **Run anyway** (Yine de çalıştır) seçeneğine tıklayın. Kurulum kartı açılır. Bkz. [Kurulum](/tr/getting-started/install/).

Önce dikkatli olmak istiyorsanız yalnızca Moonpool'un resmi sitesinden veya GitHub
sürümlerinden indirin ve dosya adının `moonpool.exe` olduğunu denetleyin.

## Run anyway düğmesi yoksa

Bazı yönetilen bilgisayarlarda yönetici bu seçeneği kapatır ve **Run anyway** görünmez.
Yöneticinize danışın ya da yönettiğiniz bir bilgisayar kullanın. İndirilmiş bir zip içinden
gelen dosya da bir engel taşıyabilir: dosyaya sağ tıklayın, **Properties** (Özellikler) seçin,
görünüyorsa **Unblock** (Engellemeyi kaldır) kutusunu işaretleyin, ardından **OK** deyin ve
yeniden çalıştırın.

## Antivirüs uyarıları

Profilinize kendini kopyalayan ve güncellemede kendinin yerine geçen yeni, imzasız bir exe,
antivirüs yazılımını da tetikleyebilir. Sizinki `moonpool.exe` dosyasını engeller veya
karantinaya alırsa `.moonpool` klasörü için izin verin. Bkz. [Windows](/tr/platforms/windows/#çalıştırmadan-önce).

## Ayrıca bakın

- [Kurulum](/tr/getting-started/install/)
- [Windows](/tr/platforms/windows/)
- [Kurulum programı bir hata gösteriyor](/tr/support/troubleshooting/#kurulum-programı-bir-hata-gösteriyor)
