---
title: "Windows'ta bir Python betiğini arka planda çalışır tutun"
description: "Windows'ta uzun ömürlü bir Python betiğini veya küçük bir web uygulamasını arka planda çalıştırın, çıktısını görün ve temiz biçimde durdurun: pythonw ve Moonpool ile."
---

Konsol penceresinden çalıştırılan bir Python betiği, o pencereyi kapattığınızda durur.
Olağan Windows çözümleri `pythonw.exe` (konsol penceresiz aynı yorumlayıcı; çıktı hiçbir yere
gitmez), betiği ayrık başlatmak için `Start-Process pythonw -ArgumentList worker.py` ya da
oturum açılışında veya bir zamanlayıcıyla çalışması gereken şeyler için zamanlanmış bir
görevdir. Hepsinde, kaldırmak istediğinizde süreci Görev Yöneticisi'nde bulmak size kalır.

## Moonpool yöntemi

Moonpool komutu kendi terminal sekmesinde çalıştırır; böylece kendi konsol pencereniz olmadan
çıktıyı ve bir Durdur düğmesini elinizde tutarsınız. Siz durdurana kadar çalışan bir betik için
`cli` türünde bir uygulama kullanın. `-u`, Python'un çıktıyı hemen boşaltmasını sağlar; böylece
sekme çıktıyı canlı gösterir:

```json title="apps.json"
{
  "id": "worker",
  "name": "Queue worker",
  "group": "Scripts",
  "type": "cli",
  "cwd": "C:\\code\\worker",
  "command": ".venv\\Scripts\\python.exe -u worker.py"
}
```

Başlatın ve çıktısını izlemek için uygulamanın adına tıklayın. Bir `cli` uygulaması komutu
çalıştığı sürece Çalışıyor sayılır; betik bittiğinde griye döner ve sekmede
`[process exited]` kalır. **Durdur**, betiği ve başlattığı her şeyi sonlandırır. Sanal
ortamın `python.exe` dosyasını yoluyla kullanmak, etkinleştirme adımına gerek bırakmaz.

Betik HTTP sunuyorsa (Flask, FastAPI, `python -m http.server`), Çalışıyor durumu
bağlantı noktasını izlesin diye onu bir `web` uygulaması yapın:

```json title="apps.json"
{
  "id": "docs-api",
  "name": "Docs API",
  "group": "Scripts",
  "type": "web",
  "cwd": "C:\\code\\docs-api",
  "command": ".venv\\Scripts\\python.exe -u app.py",
  "port": 8091,
  "url": "http://127.0.0.1:8091",
  "env": { "PORT": "8091" }
}
```

## Sınırlar

- Moonpool'u çalışır durumda tutun. Pencereyi kapatmak varsayılan olarak Moonpool'dan çıkar
  ve Windows'ta çıkmak, başlattığı her uygulamayı durdurur. Pencereyi bunun yerine gizlemek
  için **Kapatınca tepsiye gizle** seçeneğini açın; bkz.
  [Sistem tepsisi, kapatma ve küçültme](/tr/using/tray-and-closing/).
- Moonpool çöken bir betiği yeniden başlatmaz ve onu Windows oturum açılışında kendiliğinden
  başlatmaz. Bkz. [Bir betiği veya geliştirme sunucusunu Windows oturum açılışında otomatik başlatın](/tr/guides/start-app-at-windows-login/).
- `command` içinde iç içe çift tırnaktan kaçının: `cmd /c` sarmalayıcısı onları bozar.

## Ayrıca bakın

- [Uygulama türleri](/tr/apps/types/#cli): `cli` ve `web` uygulamaları nasıl izlenir.
- [Durdurma ve yeniden başlatma](/tr/apps/stop-and-restart/)
- [Örnekler](/tr/apps/examples/)
- [Günlükler](/tr/data/logs/): oturum çıktısının nerede tutulduğu.
