---
title: "Kenar çubuğunu okuyun: durum noktaları, gruplar, filtre ve satır menüsü"
description: "Her kenar çubuğu satırının neyi gösterdiğini, durum noktalarının ve grupların nasıl çalıştığını, uygulamaların nasıl filtrelendiğini, satır bağlam menüsünü ve yeniden boyutlandırmayı öğrenin."
---

Kenar çubuğu, `apps.json` içindeki her uygulamayı, her uygulamanın `group` alanına göre gruplanmış olarak listeler. Bkz. [Uygulama alanları](/tr/apps/fields/).

## Satırlar

Her satır bir durum noktası, uygulama simgesi (simge yoksa bir tür glifi), ad, ayarlıysa bağlantı noktası (`:3000`) ve denetimleri gösterir.

| Nokta | Anlamı |
| --- | --- |
| Sabit | çalışıyor |
| Yanıp sönen | başlatılıyor: Moonpool uygulamayı başlattı ama henüz ayakta olduğu algılanmadı |
| Gri | durduruldu |

Sözcük için noktanın üzerine gelin.

![İki çalışan web uygulamasının çevrelendiği kenar çubuğu: yanan noktalar ve Durdur düğmeleri](../../../../assets/screenshots/sidebar-running-narrow.png)

1. İki çalışan uygulama. Noktaları yanıyor ve Başlat'ın yerini Durdur (kare) alıyor.

| Denetim | Ne yapar |
| --- | --- |
| Kalem | Uygulamayı düzenler. |
| Yeniden başlat | Durdurur ve yeniden başlatır. Durmuş bir uygulamada onu yalnızca başlatır. |
| Başlat (oynat) | Uygulamayı başlatır ve terminal sekmesini açar. Uygulama durmuşken gösterilir. |
| Durdur (kare) | Uygulamayı durdurur. Çalışırken veya başlatılırken gösterilir. |

![Yakınlaştırılmış tek bir çalışan satır: durum noktası, tür simgesi, ad ve bağlantı noktası, ardından düzenle, yeniden başlat ve durdur düğmeleri](../../../../assets/screenshots/sidebar-row-controls.png)

1. Durum noktası (çalışırken yanar).
2. Tür simgesi.
3. Düzenle (kalem).
4. Yeniden başlat.
5. Durdur (çalışırken Başlat yerine gösterilir).

Bir başlatma veya durdurma sürerken denetimlerin yerini bir döndürücü alır (`Çalışılıyor...`).

Bir uygulamanın **adına** tıklamak terminal sekmesini açar veya odaklar ve hiçbir şeyi başlatmaz. Durmuş bir uygulamanın sekmesi bu oturumdaki günlüğü gösterir. Başlatmak için Başlat veya Yeniden başlat'ı kullanın. Yalnızca `url` içeren ve `command` içermeyen bir `static` uygulamanın terminali yoktur: Başlat, URL'yi tarayıcınızda açar.

### İpucu

Adın üzerine gelmek, varsa uygulamanın `note` değerini, yoksa adını gösterir. `note` değerini düzenleyicide veya `apps.json` içinde ayarlayın.

### MCP alt satırı

Bir yapay zeka istemcisi bir uygulamanın kendi MCP araçlarını kullandığında, uygulamanın altında
soluk bir `MCP sunucusu` alt satırı belirir. İstemci bağlıyken noktası yanar ve ipucu "MCP
istemcisi bağlı" der. Bir durdur düğmesi o süreci sonlandırır.

Satır süreci `processName` artı `mcp` bağımsız değişkeniyle ya da ayarlıysa uygulamanın
`mcpProcessName` deseniyle bulur. Bkz. [alanlar](/tr/apps/fields/#mcpprocessname).

Bu satırları Ayarlar'da **MCP süreçlerini göster** ile gizleyin. Bkz.
[MCP kurulumu](/tr/automation/mcp-setup/#kendi-mcp-sunucusu-olan-uygulamalar).

## Gruplar

![Beş grup başlığının çevrelendiği, her birinin sağında uygulama sayısı olan boşta kenar çubuğu](../../../../assets/screenshots/sidebar-groups-narrow.png)

- Bir grup başlığına tıklayarak onu daraltın veya genişletin. Yanındaki sayı gösterilen uygulama sayısıdır. Daraltılmış gruplar hatırlanır.
- Bir grubun içinde en son başlatılan uygulama en üsttedir. Hiç başlatılmamış uygulamalar `apps.json` sırasını korur. Yeni başlatılan bir uygulama parlar ve en üste çıkar.

## Filtre kutusu

Listeyi daraltmak için **Uygulamaları filtrele...** kutusuna yazın. Uygulama adıyla ve grup adıyla, büyük/küçük harfi yok sayarak eşleşir. Hiçbir şey eşleşmezse liste şunu gösterir:

```text
“<text>” ile eşleşen uygulama yok.
```

Uygulama metnin çevresinde kıvrık tırnaklar gösterir; burada ve aşağıdaki Sil isteminde.

## Sağ tıklama menüsü

Bir satıra sağ tıklayın:

| Öğe | Ne yapar |
| --- | --- |
| Düzenle | Uygulama düzenleyicisini açar. |
| Yeniden adlandır | Adı bir düzenleme alanına çevirir. **Enter** veya başka yere tıklamak kaydeder, **Esc** iptal eder. Boş ya da değişmemiş bir ad yok sayılır. |
| Simge seç... | Simge olarak kullanılacak bir görsel dosyası seçin (png, jpg, jpeg, gif, svg, webp, ico). |
| Sil | `“<name>” silinsin mi?` diye sorar ve girdiyi `apps.json` dosyasından kaldırır. Moonpool uygulamayı çalıştırıyorsa önce durdurulur. |

**Esc** menüyü bir şey yapmadan kapatır.

## Yeniden boyutlandırma

Kenar çubuğu ile CLI paneli arasındaki ayırıcıyı sürükleyin. Genişlik 180 ile 620 px arasında sınırlıdır (varsayılan 280) ve hatırlanır. CLI paneli daraltılmışken ayırıcı kilitlidir. Bkz. [Terminal sekmeleri](/tr/using/terminal-tabs/).
