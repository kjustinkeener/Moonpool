---
title: "Bir betiği veya geliştirme sunucusunu Windows oturum açılışında otomatik başlatın"
description: "Moonpool'u Windows oturum açılışında Başlangıç klasörü kısayoluyla başlatın, ardından küçük bir PowerShell betiğiyle içinde bir geliştirme sunucusu veya betik çalıştırın. Bunu yapan bir ayar yoktur."
---

Windows'ta oturum açılışında bir şey başlatmanın iki olağan yolu vardır: Başlangıç
klasörünüzdeki bir kısayol (Win+R'ye basın, `shell:startup` yazın, Enter'a basın) ya da
"Oturum açılırken" tetikleyicili bir Görev Zamanlayıcı görevi. İkisi de bir program ya da
betik çalıştırır; bu doğrudan geliştirme sunucunuzun komutu olabilir, ancak o zaman hiçbir
şey onu izlemez, çıktısını göstermez ya da sizin için durdurmaz.

## Moonpool ne sunar

Moonpool'un oturum açılışında başlama ayarı yoktur ve `apps.json` içindeki bir girdinin,
Moonpool başladığında onu başlatan bir alanı yoktur (tam liste [Uygulama alanları](/tr/apps/fields/)
ve [settings.json](/tr/data/settings-json/) sayfalarındadır). Yapabileceğiniz şey, Moonpool'u
oturum açılışında kendiniz başlatmak, sonra istediğiniz uygulamaları bir betikle başlatmaktır;
bunun için [komut satırının](/tr/automation/command-line/) sunduğu aynı fiili kullanırsınız.

Önce uygulamayı her zamanki gibi kaydedin:

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173"
}
```

Ardından bunu `start-moonpool-apps.ps1` olarak kaydedin. Kurulu olduğunda program
`%USERPROFILE%\.moonpool\moonpool.exe` dosyasıdır; taşınabilir bir kopya için o kopyanın exe
dosyasının yolunu kullanın.

```powershell title="start-moonpool-apps.ps1"
$mp = "$env:USERPROFILE\.moonpool\moonpool.exe"
Start-Process $mp
Start-Sleep -Seconds 15
& $mp launch site
```

`launch` komutunun ona iletilebilmesi için Moonpool'un zaten çalışıyor olması gerekir; hiçbiri
yerleşik değilken çalıştırılırsa aynı komut yeni bir Moonpool başlatır ve fiil yerine
getirilmez. Gecikme ona başlaması için zaman tanır, bu yüzden yavaş bir makinede artırın.
Her uygulama için bir `& $mp launch <id>` satırı ekleyin.

Son olarak betiğin bir kısayolunu Başlangıç klasörüne, şu hedefle koyun:

```text title="Shortcut target"
powershell.exe -NoProfile -WindowStyle Hidden -File "C:\Users\you\start-moonpool-apps.ps1"
```

Ne olduğunu denetlemek için bir fiile `--ticket t1` ekleyin ve sonucu `state.json` içinden
okuyun ([Sonucu okuma](/tr/automation/command-line/#sonucu-okuma)).

## Uyarılar

- Bu şekilde başlatılan bir geliştirme sunucusu, diğerleri gibi Moonpool tarafından
  "yönetilir"; yani Durdur ve Çıkış onun üzerinde çalışır. Aynı uygulama zaten çalışıyorsa
  (örneğin elle başlatılmışsa), Moonpool onu çalışıyor ama yönetilmiyor olarak gösterir.
- Moonpool çıkan bir uygulamayı yeniden başlatmaz ve en son çıktığınızda hangi uygulamaların
  çalıştığını hatırlamaz.

## Ayrıca bakın

- [Komut satırı](/tr/automation/command-line/)
- [Sistem tepsisi, kapatma ve küçültme](/tr/using/tray-and-closing/)
- [Windows'ta bir npm geliştirme sunucusunu arka planda çalıştırın](/tr/guides/run-npm-dev-server-in-background-windows/)
