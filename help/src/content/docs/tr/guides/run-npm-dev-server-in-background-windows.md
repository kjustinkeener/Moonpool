---
title: "Windows'ta bir npm geliştirme sunucusunu terminal penceresi olmadan arka planda çalıştırın"
description: "npm run dev, Vite veya başka bir geliştirme sunucusunu Windows'ta bakmanız gereken bir konsol penceresi olmadan çalışır tutun; sistem tepsisinden başlatın, durdurun ve çıktısını okuyun."
---

`npm run dev` ile başlatılan bir geliştirme sunucusu onu başlatan terminalde çalışır, bu yüzden
o pencereyi kapatmak sunucuyu bitirir. Çalışır tutmanın düz Windows yolu gizli bir süreçtir,
örneğin PowerShell'de `Start-Process npm.cmd -ArgumentList "run","dev" -WindowStyle Hidden`;
ancak o zaman okuyacak bir çıktınız olmaz ve durdurmak doğru `node.exe` dosyasını aramak
demektir (bkz.
[Bir bağlantı noktasını kullanan süreci bulun ve sonlandırın](/tr/guides/find-and-kill-process-using-port-windows/)).

## Moonpool yöntemi

Moonpool komutu merkez penceresinin içindeki kendi gömülü terminal sekmesinde çalıştırır,
yani açık tutmanız gereken ayrı bir konsol penceresi yoktur. Merkez penceresini sistem
tepsisine gizleyin, sunucu çalışmaya devam eder. Uygulamayı bir kez ekleyin:

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173",
  "openBrowser": true
}
```

Uygulamanın **Başlat** denetimine tıklayın. `port` yanıt verdiğinde durum noktası sabitlenir
ve `openBrowser` sayesinde tarayıcı `url` adresinde açılır. Çıktısını kendi sekmesinde
okumak için uygulamanın adına tıklayın. **Durdur**, terminali ve başlattığı her şeyi
sonlandırır ve bağlantı noktasını serbest bırakır (`web` için `killMode` `port` varsayılandır).

## Pencereyi kapatınca çalışır tutun

Varsayılan olarak kapat düğmesi Moonpool'dan çıkar ve Windows'ta çıkmak, başlattığı her
uygulamayı durdurur. [Ayarlar](/tr/using/settings/) içinde **Kapatınca tepsiye gizle**
seçeneğini açın; pencereyi kapatmak yalnızca gizler. Sistem tepsisi simgesi (veya
**Moonpool'u göster**) onu geri getirir. Ayrıntılar
[Sistem tepsisi, kapatma ve küçültme](/tr/using/tray-and-closing/) sayfasındadır.

## Bağlantı noktasını öngörülebilir tutun

Moonpool Çalışıyor durumuna `port` üzerinden karar verir. Vite kendi bağlantı noktası doluysa
bir sonraki boş olana geçer; bu da Moonpool'un yanlış olanı izlemesine yol açar. Vite'ın bunun
yerine çıkması için `--strictPort` verin ve `port` değerini buna uyacak şekilde ayarlayın:

```text title="command"
npm run dev -- --port 5173 --strictPort
```

Bağlantı noktası zaten doluysa
[EADDRINUSE ve "Port 5173 is in use" hatalarını düzeltin](/tr/support/port-already-in-use/)
sayfasına bakın.

## Sınırlar

- Moonpool çöken bir sunucuyu yeniden başlatmaz. Uygulamayı durmuş olarak gösterir ve sekme
  `[process exited]` yazdırır.
- Moonpool Windows oturum açılışında kendiliğinden başlamaz. Bkz.
  [Bir betiği veya geliştirme sunucusunu Windows oturum açılışında otomatik başlatın](/tr/guides/start-app-at-windows-login/).

## Ayrıca bakın

- [Uygulama alanları](/tr/apps/fields/): `port`, `openBrowser`, `killMode`.
- [Uygulama türleri](/tr/apps/types/): `web` için Çalışıyor durumuna nasıl karar verilir.
- [Durdurma ve yeniden başlatma](/tr/apps/stop-and-restart/)
- [Örnekler](/tr/apps/examples/#web-geliştirme-sunucusu)
