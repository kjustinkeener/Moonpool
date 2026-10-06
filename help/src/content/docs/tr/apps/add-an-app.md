---
title: "Moonpool'a uygulama veya geliştirme sunucusu ekleme"
description: "Yerel bir uygulamayı veya geliştirme sunucusunu başlatma komutu, çalışma klasörü ve ortamla kaydedin; Moonpool sizin için başlatsın, durdursun ve izlesin."
---

Moonpool'daki her uygulama; bir başlatma komutu, bir çalışma klasörü ve isteğe bağlı bir
ortam içeren tek bir girdidir. Moonpool komutu kendi yönettiği terminalde çalıştırır.

## Uygulama ekleme

1. Kenar çubuğunun üstündeki **...** menüsünü açın ve **Uygulama ekle** seçeneğini seçin.
2. Bir **ad** girin ve bir **grup** seçin.
3. **Türü** seçin: `web` (bir bağlantı noktasında çalışan sunucu), `desktop` (yerel uygulama), `static` (bir sayfa) veya `cli` (bir komut).
4. **command** değerini ve çalışacağı **cwd** klasörünü ayarlayın.
5. Türün gerektirdiği alanları doldurun: web için **port** ve **url**, desktop için **processName**,
   static için **url**. Yalnızca bir `url` içeren `static` uygulama için **command** veya **cwd** gerekmez.
6. Kaydedin. Uygulama kenar çubuğunda görünür. Başlatmak için **Başlat** denetimini kullanın.

![Uygulama düzenleyicide tür seçimi (1) ve port alanı (2); aralarında cwd ve command alanları var](../../../../assets/screenshots/edit-app-type-and-port.png)

1. **type** seçimi; ipucu satırı bu türün nasıl çalıştığını anlatır.
2. `web` uygulamalarının kullandığı **port** alanı.

Sonuç, `apps.json` içinde tek bir girdidir, örneğin:

```json title="apps.json"
{
  "id": "my-api",
  "name": "My API",
  "group": "Dev",
  "type": "web",
  "command": "npm run dev",
  "cwd": "C:\\code\\my-api",
  "port": 3000,
  "url": "http://localhost:3000"
}
```

Bir uygulamanın adına tıklamak yalnızca terminal sekmesini açar; bkz. [Uygulama durumları](/tr/support/glossary/#uygulama-durumları).

## Uygulama düzenleyici

- **Grup.** Listeden bir grup seçin ya da **+ Yeni grup...** seçeneğini seçip bir ad yazın.
  **listeye dön** listeye geri götürür. Boş bırakılan grup `Apps` olarak kaydedilir.
- **Soluk alanlar** seçili tür tarafından kullanılmaz. Yine de kaydedilirler.
- **Adsız kaydetmek** `ad zorunludur.` iletisini gösterir.
- **Esc** tuşuna basmak ya da düzenleyiciyi kaydedilmemiş değişikliklerle kapatmak "Değişiklikleriniz atılsın mı?" sorusunu getirir.
- Bir uygulamayı sonradan değiştirmek için satırındaki kalemi kullanın ya da satıra sağ tıklayıp **Düzenle** seçeneğini seçin.

## Elle düzenleme

Aynı menüden **apps.json dosyasını düzenle** seçeneğini seçin, dosyayı kaydedin, ardından **Yeniden yükle**
seçeneğini seçin. Biçim, doğrulama kuralları ve kurtarma seçenekleri
[Yapılandırmaya genel bakış](/tr/apps/apps-json/) sayfasındadır.

## Sonraki adımlar

- [Uygulama alanları](/tr/apps/fields/): her anahtar ve ne işe yaradığı.
- [Uygulama türleri](/tr/apps/types/): her türün nasıl başlatıldığı ve Çalışıyor durumunu nasıl gösterdiği.
- [Durdurma ve yeniden başlatma](/tr/apps/stop-and-restart/): Durdur bir şeyi çalışır halde bıraktığında nelerin ayarlanacağı ve Docker uygulamalarının neden dikkat istediği.
- [Yollar ve ortam](/tr/apps/paths-and-environment/): `{MP_HOME}`, `./` yolları ve `env`.
- [Örnekler](/tr/apps/examples/): kopyalayabileceğiniz eksiksiz girdiler.
- [Nasıl yapılır kılavuzları](/tr/guides/run-npm-dev-server-in-background-windows/): arka planda geliştirme sunucuları, Python betikleri, bağlantı noktaları.
- [Taşınabilir mod](/tr/data/portable-mode/)
- [Güncelleme](/tr/data/updating/)
