---
title: "Moonpool oturum günlüklerini ve hata ayıklama günlüğünü bulma ve yönetme"
description: "Her uygulamanın oturum günlüğünü, Moonpool'un kendi hata ayıklama günlüğünü, dökümleri ve kaydırma geçmişini bulun; her birinin ne kadar süre tutulduğunu ve nasıl açılıp kopyalanacağını görün."
---

Moonpool dört tür çıktı tutar:

| Tür | Yer | Saklanma |
| --- | --- | --- |
| Oturum günlüğü | Yapılandırma klasöründe `cli-output\<id>\<session-start-ms>.log` | Bu oturum her zaman; eski oturumlar aşağıdaki saklama kurallarına göre |
| `moonpool.log` | Yapılandırma klasörü | Yalnızca **Hata ayıklama bilgilerini bir dosyaya kaydet** açıkken yazılır |
| Döküm | İstediğiniz yer ya da oturum günlüğünün kendi yolu | Siz silene kadar |
| Kaydırma geçmişi | Terminal sekmesinde | 10.000 satır, Moonpool çıkana kadar |

Yapılandırma klasörü [Yapılandırmanın bulunduğu yer](/tr/apps/apps-json/#yapılandırmanın-bulunduğu-yer) bölümünde listelenir.

## Oturum günlükleri

Bir uygulamanın terminalinde yazdırdığı her şey ayrıca bir günlük dosyasına yazılır:

```text
<config folder>\cli-output\<id>\<session-start-ms>.log
```

- Uygulama başına ve Moonpool oturumu başına bir dosya. Sayı, o Moonpool sürecinin başladığı zamandır.
- Bir uygulamayı durdurup yeniden başlatmak aynı dosyaya eklemeyi sürdürür. Soluk bir ayırıcı satır her yeni
  çalıştırmanın nerede başladığını işaretler ve aynı işaret terminal sekmesinde de görünür:

  ```text title="1767225600000.log"
  Local:   http://localhost:5173/
  ---------- restarted 2026-10-05 09:14:02 ----------
  Local:   http://localhost:5173/
  ```

- Bir `id` içindeki harf, rakam, `-` ve `_` dışındaki karakterler klasör adında `_` olur. Yani `.`, `_` olur.
  İngilizce dışı harfler korunur.
- Dosya, renk kodları dahil ham terminal çıktısını tutar. Düz metin için döküm kullanın.

Bir uygulamanın sekmesini yeniden açmak bu oturumun günlüğünü yeniden oynatır; böylece önceki çıktısını görürsünüz.

## Saklama

Saklama yalnızca önceki Moonpool oturumlarının günlüklerini ilgilendirir. Bir uygulamayı başlattığınızda, yalnızca o
uygulamanın klasörü için, en eskiden başlayarak çalışır.

| **Uygulama çıktı günlüklerini oturumlar arasında sakla** (`cliLogging`) | Önceki oturumların günlüklerine ne olur |
| --- | --- |
| kapalı (varsayılan) | Uygulamanın bir sonraki başlatılışında silinir. |
| açık | Klasörün toplam boyutu **Uygulama başına günlük saklama** (`logRetentionMb`, varsayılan 10 MB) değerini aşana kadar saklanır, sonra en eskiler silinir. |

Geçerli oturumun dosyası bu toplama dahildir ama asla silinmez veya kesilmez. Yani çok büyük tek bir geçerli günlük
daha eski her günlüğü dışarı itebilir.

![Ayarlar'ın Günlük bölümü: günlükleri sakla onay kutusu, uygulama başına MB cinsinden saklama boyutu ve hata ayıklama günlüğü onay kutusu; her birinin yanında bir klasör yolu satırı](../../../../assets/screenshots/settings-logging-section.png)

1. **Uygulama çıktı günlüklerini oturumlar arasında sakla** ayarı `cliLogging` değeridir. Altındaki **Uygulama başına günlük saklama** ise `logRetentionMb` değeridir.

## moonpool.log

**Hata ayıklama bilgilerini bir dosyaya kaydet** (`debugLogging`) açıkken Moonpool, yapılandırma klasöründeki
`moonpool.log` dosyasına zaman damgalı satırlar ekler: `apps.json` yüklemeleri, başlatmalar (komut ve klasörle),
denetim komutları ve hatalar. Bir sorunu yeniden üretmeden önce açın.

## Aç ve Kopyala

[Ayarlar](/tr/using/settings/#sağ-sütun-günlükler) içinde, her günlük grubunun altında:

- **CLI günlük klasörünü aç** ve **Günlüğü aç** klasörü dosya yöneticinizde açar.
- **CLI günlük klasörünün yolunu kopyala** ve **Günlük dosyasının yolunu kopyala** yolu panoya koyar.

Bir terminal sekmesinde **Tümünü kopyala**, kaydırma geçmişinin tamamını metin olarak kopyalar.

## Dökümler

`dump` komutu, bir betikten oturum günlüğü almanızı sağlar:

```powershell frame="terminal"
& "$env:USERPROFILE\.moonpool\moonpool.exe" dump my-app C:\temp\my-app.log
```

Bir çıktı yoluyla, renk kodları kaldırılmış düz metin bir kopya yazar. Yol verilmezse oturum günlüğünün kendi
yolunu bildirir. Bir ajan aynı metni, zaten temizlenmiş olarak `moonpool_app_output` aracından alır.
Bkz. [Komut satırı](/tr/automation/command-line/).
