---
title: "Moonpool terminal sekmelerini kullanın: açın, kapatın, kopyalayın, yeniden başlatın"
description: "Merkezdeki uygulama başına terminal sekmeleriyle çalışın: sekmeleri açın ve kapatın, paneli daraltın, kopyalayıp yapıştırın, bir oturumu yeniden başlatın ve oturum günlüklerini bulun."
---

Her uygulama, CLI panelinde kendi terminal sekmesinde çalışır.

## Sekmeler

![Metrics Dashboard sekmesinin etkin (çerçeveli) olduğu ve altında canlı günlüğünün göründüğü sekme şeridi; her sekmede bir nokta ve bir x vardır](../../../../assets/screenshots/hub-terminal-tab.png)

- Bir uygulamayı başlatmak ya da kenar çubuğunda adına tıklamak sekmesini açar. Bir ada tıklamak hiçbir şeyi başlatmaz; bkz. [Uygulama durumları](/tr/support/glossary/#uygulama-durumları).
- Sekmedeki bir nokta, uygulama çalışırken yanar.
- Bir sekmedeki **x** sekmeyi kapatır. Uygulamayı durdurmaz. Sekmeyi yeniden açmak için adına yeniden tıklayın; bu oturumun günlüğünü gösterir.

## Paneli daraltma

Sekme şeridinin en sağındaki **x** ("CLI panelini gizle"), CLI panelini daraltır ve pencereyi
yalnızca kenar çubuğuna küçültür. Terminaller çalışmaya devam eder ve geri kaydırma
geçmişlerini korur.

Paneli önceki genişliğinde geri getirmek için filtre kutusunun yanında bir ok belirir. Bir
güncelleme beklerken ok yanıp söner, çünkü güncelleme şeridi panelin içindedir.

## Kopyalama ve yapıştırma

| Eylem | Sonuç |
| --- | --- |
| Fareyle metin seçme | Bırakınca panoya kopyalanır, sonra seçim temizlenir. |
| Orta tıklama | Panoyu terminale yapıştırır. |
| **Tümünü kopyala** düğmesi (sağ üstte, üzerine gelince görünür) | Tüm geri kaydırma geçmişini metin olarak kopyalar. |

## Geri kaydırma geçmişi

Her terminal 10.000 satır tutar.

## Bir süreç bittiğinde

Süreç çıktığında terminal şunu yazdırır:

```text
[process exited]
```

Sekme, çıktısı bozulmadan açık kalır. `[process exited]` satırı sizin dilinizde gösterilir.

## Yeniden başlatma

**Yeniden başlat** (veya durmuş bir uygulamada Başlat) aynı sekmede yeni bir çalıştırma
başlatır. Sekme yeniden kurulur ve bu oturumun önceki çıktısı oturum günlüğünden içine yeniden
oynatılır.

Uygulama bu oturumda daha önce de çalışmışsa Moonpool önce oturum günlüğüne soluk bir ayraç
yazar; böylece eski çıktı ile yeni çalıştırma arasında görünür:

```text
---------- restarted 2026-10-05 09:14:02 ----------
```

Yeni çalıştırma ekranı temizleyerek başlıyorsa önceki çıktı silinmek yerine geri kaydırma
geçmişine itilir.

## Oturum günlükleri

Bir uygulamanın yazdırdığı her şey ayrıca `cli-output\` altında bir günlük dosyasına yazılır;
her uygulama için her Moonpool oturumunda bir dosya. Konum, saklama ve **Uygulama çıktı
günlüklerini oturumlar arasında sakla** ayarı [Günlükler](/tr/data/logs/) sayfasındadır.
