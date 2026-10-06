---
title: "Moonpool temasını, dilini ve saydamlığını değiştirin"
description: "Bir renk teması ve arayüz dili seçin, arka plan saydamlığını ve arayüz ölçeğini ayarlayın; hepsinin açık olan her Moonpool penceresine anında uygulandığını görün."
---

Tema, dil ve saydamlık [Ayarlar penceresinde](/tr/using/settings/) ayarlanır. Üçü de açık olan
her Moonpool penceresine anında uygulanır.

![Ayarlar'ın üstündeki Dil (1) ve Tema (2) seçicileri](../../../../assets/screenshots/settings-language-theme.png)

1. Dil seçici.
2. Tema düğmesi. Geçerli temanın adını gösterir ve tema tarayıcısını açar.

## Temalar

Tema tarayıcısı kendi penceresidir. Her tema için, o temanın kendi renkleriyle çizilmiş (metin,
panel, giriş, düğme, durum noktaları, gösterge gradyanı ve 16 renkli terminal takımı) bir
önizleme kartı vardır; kartlar Temel, Neon, Sıcak, Soğuk, Yeşiller, Nötr, Açık, Pembemsi,
Canlı, Açık pastel ve Pastel olarak gruplanır. Uygulamak için bir karta tıklayın: açık olan
her Moonpool penceresi aynı anda değişir ve seçim kaydedilir. Karşılaştırabilmeniz için pencere
açık kalır; kapatmak için Esc'ye basın.

68 tema artı Otomatik vardır ve yaklaşık yarısı açıktır. Tema adları özel adlardır ve
çevrilmez; yalnızca Otomatik (sistem), Koyu ve Açık çevrilir.

**Otomatik (sistem)**, işletim sisteminin açık ya da koyu tercihini izler ve işletim sistemi
değiştirdiğinde canlı olarak geçiş yapar. Diğer her seçim sabittir. Terminalin 16 ANSI rengi de
temayı izler.

Önceki bir sürümden bir tema kaydettiyseniz korunur. Moonpool'un artık tanımadığı kayıtlı bir
ad Otomatik'e döner. Bazı etiketler öncekinden farklıdır (örneğin Matrix artık Terminal,
Nord Arctic, Dracula Nocturne, Gruvbox Retro ve Solarized Solar olarak etiketlenir); kayıtlı
seçimin kendisi değişmez.

Tema, `settings.json` içinde değil, web görünümünün `localStorage` alanında saklanır. Depolama
kullanılamıyorsa Otomatik'e döner.

```text
localStorage key: moonpool.theme
```

## Diller

Otomatik, işletim sistemi dilini izler. Aksi halde her biri kendi dilinde gösterilen 14
dilden birini seçin:

```text
English, Deutsch, Español, Français, Italiano, 日本語, 한국어, Nederlands, Polski,
Português (Brasil), Русский, Türkçe, 简体中文, 繁體中文
```

Seçici merkeze ve diğer pencerelere anında uygulanır. Seçim `settings.json` içinde `locale`
olarak kaydedilir.

## Saydamlık

**Arka plan saydamlığı**, pencere arka planını 0% (opak) ile 90% arasında saydam yapar.

- İşaretçiyi bir pencerenin üzerine getirmek onu hemen tam opak yapar. İşaretçi ayrılınca yaklaşık 2 saniyede ayarınıza geri solar.
- Terminaller kendi tonlarını eklemek yerine aynı tonu izler.
- Her pencere (merkez, Ayarlar, Hakkında, uygulama düzenleyicisi ve tema tarayıcısı) ayarı kendisi uygular ve Ayarlar, kaydırıcıyı sürüklerken diğerlerini canlı olarak günceller.

## Arayüz ölçeği

Tüm arayüzü Ctrl + fare tekerleğiyle yakınlaştırın. Klavyeyle yakınlaştırma yoktur. Bkz.
[Kısayollar ve yakınlaştırma](/tr/using/keyboard-shortcuts/).

## Ayrıca bakın

- [Ayarlar penceresi](/tr/using/settings/)
- [Kısayollar ve yakınlaştırma](/tr/using/keyboard-shortcuts/)
