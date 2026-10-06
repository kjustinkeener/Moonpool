---
title: "Moonpool'u USB bellekten veya eşitlenen bir klasörden çalıştırma"
description: "Moonpool'u ve tüm verilerini taşınabilir tek bir klasörde tutun; USB bellekte taşıyın ya da eşitleyin ve birkaç kopyayı yan yana çalıştırın."
---

Taşınabilir mod, Moonpool'u ve yazdığı her şeyi tek bir `.moonpool\` klasörünün içinde tutar; böylece
onu bir USB bellekte taşıyabilir ya da eşitlenen bir klasöre bırakıp herhangi bir bilgisayarda çalıştırabilirsiniz.

## Nasıl çalışır

Taşınabilir olarak kurduğunuzda Moonpool, seçtiğiniz konumun içinde bir `.moonpool\` klasörü oluşturur.
Bu klasör programı, yapılandırmanızı ve yardım içeriğini barındırır. Windows AppData'ya hiçbir şey yazılmaz;
dolayısıyla klasörü taşımak ya da kopyalamak tüm kurulumunuzu onunla birlikte taşır.

```text
<chosen location>\.moonpool\
```

## Kurulu sürümden farkları

| | Kurulu | Taşınabilir |
| --- | --- | --- |
| Program | `%USERPROFILE%\.moonpool\moonpool.exe` | `<chosen location>\.moonpool\moonpool.exe` |
| Yapılandırma klasörü | `%USERPROFILE%\.moonpool\moonpool-config\` | `<chosen location>\.moonpool\moonpool-config\` |
| Pencere tarayıcı profili, pencere boyutu ve konumu | Yapılandırma klasöründe | Yapılandırma klasöründe; yani onlar da taşınır |
| Başlat Menüsü, masaüstü kısayolu, Add/Remove girdisi | Evet | Yok |
| Güncellemeler | Kendi exe'sini değiştirir | Aynısı, `.moonpool\` klasörünün içinde. Bkz. [Güncelleme](/tr/data/updating/#taşınabilir-kopyalar). |
| Kaldırma | Add/Remove Programs veya `--uninstall` | Klasörü silin |

Hiçbir mod Windows AppData'ya yazmaz.

### Eşitlenen klasörler

Taşınabilir bir kopyayı eşitlenen bir klasörde (OneDrive, Dropbox ve benzerleri) tutabilirsiniz, ancak aynı anda
yalnızca tek bir bilgisayarda çalıştırın. Moonpool `state.json` dosyasını birkaç saniyede bir yazar ve uygulamalar
çalıştıkça günlük tutar; bu yüzden aynı klasörü çalıştıran iki bilgisayar aynı dosyalar için çekişir ve bir
eşitleme çakışması `apps.json` dosyasını bozuk bırakabilir. Başka bir bilgisayarda başlatmadan önce birinde
çıkın.

## Aynı anda birkaç kopya

Klasör başına bir Moonpool çalışır. Kurulu Moonpool ve her biri kendi klasöründe olan istediğiniz sayıda taşınabilir
kopya aynı anda çalışabilir ve her biri tamamen ayrıdır: kendi uygulamaları, sistem tepsisi simgesi, penceresi,
ayarları, günlükleri ve [denetim kanalı](/tr/automation/control-verbs/) vardır.

- Sistem tepsisi ipucu ve görev çubuğu adı hangi kopyanın hangisi olduğunu söyler: kurulu olan için `Moonpool`,
  taşınabilir olan için `Moonpool (<folder>)`; burada `<folder>` seçtiğiniz klasördür (`.moonpool\` klasörünü
  içeren klasör).
- Aynı kopyayı ikinci kez başlatmak, bir tane daha açmak yerine penceresini geri getirir. Farklı bir kopyayı
  başlatmak o kopyayı açar.
- Bir yapay zekâ ajanına birden fazla kopya vermek için her birini kendi adıyla kaydedin; bkz.
  [MCP kurulumu](/tr/automation/mcp-setup/#birden-fazla-moonpool).
- Taşınabilir bir klasörü taşımak veya yeniden adlandırmak ona yeni bir kimlik verir (yeni bir denetim kanalı
  adı). Taşımadan önce çıkın.
- Kopyalar birbirlerinin uygulamalarını bilmez. Aynı sunucuyu aynı bağlantı noktasında başlatan iki kopya yine
  çakışır ve süreç adına veya bağlantı noktasına göre çalışan bir Durdur, başka bir kopyanın başlattığı bir şeyi
  sonlandırabilir; bkz.
  [Durdurma ve yeniden başlatma](/tr/apps/stop-and-restart/#birkaç-moonpool-veya-kendi-süreçleriniz).

## Uygulamalarınızı da taşınabilir yapma

Bir uygulamanın yolunda `{MP_HOME}` belirtecini kullanın; böylece tek bir makinedeki sabit bir konuma değil,
taşınabilir klasörün içine işaret eder. Taşınabilir bir kopyada `{MP_HOME}`, `moonpool.exe` dosyasını
içeren klasördür; yani seçtiğiniz klasör değil, `.moonpool\` klasörünün kendisidir:

```json title="apps.json"
{ "cwd": "{MP_HOME}/my-app" }
```

Burada `{MP_HOME}/my-app`, `<chosen location>\.moonpool\my-app` yoludur. `./` ile başlayan bir yol da aynı şekilde
sabitlenir. Belirteçler ve `./` yolları kurulu bir Moonpool'da da çalışır.
Yolların nasıl çözümlendiği için bkz. [Yollar ve ortam](/tr/apps/paths-and-environment/).

## Yükleyiciden taşınabilir seçme

Taşınabilir mod, **Moonpool'u kur** düğmesinin yanında **Taşınabilir olarak kur** seçeneğini sunan yükleyici
kartından ayarlanır.

![Kurulum kartı: Taşınabilir olarak kur bağlantısı, ana Moonpool'u kur düğmesinin altında yer alır](../../../../assets/screenshots/installer-window.png)

Bir klasör seçin; Moonpool orada `.moonpool\` klasörünü oluşturur, kendini içine kopyalar ve yeni kopyayı yeni bir
yapılandırmayla başlatır.

Kart ayrıca "..." menüsünde, hem kurulu hem taşınabilir modda **Moonpool'u kur…** olarak bulunur. Oradan
**Taşınabilir olarak kur** seçeneğini kullanmak, çalışan Moonpool'un çıkmasına ve yeni taşınabilir kopyanın onun
yerine başlamasına yol açar. Başlattığınız Moonpool olduğu yerde bırakılır; böylece sonrasında yeniden
başlatabilirsiniz.

Taşınabilir bir kopya temiz başlar ve mevcut uygulamalarınızı kopyalamaz. Onları aktarmak için taşınabilir
kopyadan çıkın ve `apps.json` dosyasını elle kopyalayın:

| | Yol |
| --- | --- |
| Kaynak (kurulu) | `%USERPROFILE%\.moonpool\moonpool-config\apps.json` |
| Hedef (taşınabilir) | `<chosen location>\.moonpool\moonpool-config\apps.json` |

Mutlak yollu girdiler aynı bilgisayarda yine çalışır ama taşınmazlar. Uygulamayı düzenle iletişim kutusu bunları
"taşınabilir değil" olarak işaretler.

## Moonpool taşınabilir olduğunu nasıl bilir

Bir kopya, `moonpool.exe` dosyasının yanında `moonpool.portable` adlı bir dosya bulunduğu sürece taşınabilirdir.
Başka hiçbir şey onu işaretlemez ve Windows'a hiçbir şey kaydedilmez.

Taşınabilir bir kopyayı kaldırmak için çıkın ve `.moonpool\` klasörünü silin. `--uninstall` yalnızca
kurulu Moonpool'u kaldırır, taşınabilir bir kopyayı asla.
