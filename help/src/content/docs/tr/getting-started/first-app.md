---
title: "Moonpool'da ilk uygulamanızı ekleyin ve çalıştırın"
description: "İlk açılıştan kendi çalışan uygulamanıza birkaç dakikada ulaşın: ekleyin, başlatın, durdurun ve merkez penceresini ile bu yardımı daha sonra yeniden bulun."
---

## 1. Moonpool'u başlatın

Windows'ta `moonpool.exe` dosyasını çalıştırın ve **Moonpool'u kur** düğmesine tıklayın (bkz.
[Kurulum](/tr/getting-started/install/)). Linux'ta AppImage'ı ya da kurulu paketi başlatın.

Moonpool ilk çalıştığında kenar çubuğunu örnek uygulamalarla doldurur (Windows'ta Notepad, bir
kabuk, küçük bir web sunucusu ve birlikte gelen panolar). Oldukları gibi çalışırlar (web
sunucusu için Python gerekir); böylece onları deneyebilir, sonra düzenleyebilir veya
silebilirsiniz. Ayrıca sistem tepsisine bir simge koyar. Windows'ta simgeyi görmüyorsanız
görev çubuğunun sağındaki **^** okuna tıklayın.

## 2. Uygulamanızı ekleyin

1. Kenar çubuğunun üstündeki **...** menüsünü açın ve **Uygulama ekle** seçeneğini seçin.
2. Bir **name** girin. Grup `Web apps` olarak başlar; olduğu gibi bırakın ya da başka birini seçin.
3. Bir geliştirme sunucusu için **type** değerini `web` olarak bırakın.
4. **cwd** değerini proje klasörünüze, **command** değerini ise başlatmak için yazdığınız
   komuta ayarlayın, örneğin `npm run dev`.
5. **port** değerini uygulamanın dinlediği bağlantı noktasına, **url** değerini ise açılacak
   sayfaya ayarlayın.
6. Kaydedin.

Her alanın ayrıntıları [Uygulama ekleme](/tr/apps/add-an-app/) sayfasındadır.

## 3. Başlatın

Uygulamanın **Başlat** düğmesine (satırındaki oynat simgesi) tıklayın. Terminal sekmesi
açılır ve çıktıyı gösterir. Durum noktası uygulama başlarken yanıp söner, bağlantı noktası
yanıt verdiğinde sabitlenir. **openBrowser** açıksa sayfa açılır.

Uygulamanın adına tıklamak yalnızca terminal sekmesini açar. Uygulamayı asla başlatmaz.

## 4. Durdurun

Satırdaki **Durdur** düğmesine (kare) tıklayın. Nokta griye döner.

Durdur'dan sonra bir şey çalışmaya devam ediyorsa [Durdurma ve yeniden başlatma](/tr/apps/stop-and-restart/) sayfasına bakın.

## Bir aracıya yaptırın

Hiçbir sekme açık değilken CLI paneli bir **İstemi kopyala** düğmesi gösterir. İstemi bir
yapay zeka aracısına yapıştırın; uygulamalarınızı bulur ve ekler. Bkz.
[Yapay zeka aracıları: hızlı başlangıç](/tr/automation/quick-start/).

## Merkez penceresini daha sonra bulma

- Merkez penceresini göstermek için sistem tepsisi simgesine sol tıklayın. Sağ tıklayınca
  **Moonpool'u göster** ve **Çıkış** içeren bir menü açılır.
- Varsayılan olarak pencereyi kapatmak Moonpool'dan çıkar. Pencereyi bunun yerine sistem
  tepsisine gizleyip çalışır durumda tutmak için Ayarlar'da **Kapatınca tepsiye gizle**
  seçeneğini açın. Bkz.
  [Sistem tepsisi, kapatma ve küçültme](/tr/using/tray-and-closing/).

## Yardım alma

Kenar çubuğunun üstündeki **...** menüsündeki **Yardım**, bu yardımı kendi penceresinde açar.
Çevrimdışı çalışır ve her zaman çalıştırdığınız sürümle eşleşir.

![Solda bölüm gezintisi çerçevelenmiş, sağda bir sayfa görünen Yardım penceresi](../../../../assets/screenshots/help-window.png)

## Sonraki adım

- [Uygulama ekleme](/tr/apps/add-an-app/)
- [Sorun giderme](/tr/support/troubleshooting/)
