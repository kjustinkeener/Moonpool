---
title: "Uygulamalarda yolları, MP_HOME belirteçlerini ve ortam değişkenlerini kullanma"
description: "Uygulama girdilerinde {MP_HOME} ve {MP_DATA} belirteçlerini ve göreli ./ yollarını kullanın, hangi alanların bunları genişlettiğini görün; env ve çalışma klasörünü ayarlayın."
---

## Belirteçler

| Belirteç | Şuna genişler |
| --- | --- |
| `{MP_HOME}` | Taşınabilir: `moonpool.exe` dosyasını içeren klasör (`.moonpool\` klasörü). Windows'ta kurulu: `%USERPROFILE%\.moonpool`. Linux: `$XDG_CONFIG_HOME/Moonpool`, yoksa `~/.config/Moonpool`; `{MP_DATA}` ile aynı klasör. |
| `{MP_DATA}` | `apps.json` dosyasını içeren yapılandırma klasörü. |

Çözümlenemeyen bir belirteç yazıldığı gibi bırakılır.

## Hangi alanlar genişletilir

| Alan | Belirteçler | Başta `./` veya `.\` |
| --- | --- | --- |
| `cwd` | evet | evet, `{MP_HOME}` konumuna sabitlenir |
| `command` | evet | hayır |
| `stopCommand` | evet | hayır (sabitlenmiş olan `cwd` içinde çalışır) |
| `url` | evet | hayır |
| `icon` | evet | evet, `{MP_HOME}` konumuna sabitlenir |
| `env` değerleri, `processName`, `note` | hayır | hayır |

`./` içermeyen göreli bir yol (örneğin `apps\tool`) olduğu gibi bırakılır ve Moonpool'un kendi çalışma
klasörüne göre çözümlenir; bu da nadiren istediğiniz şeydir. `./` ya da bir belirteç tercih edin.

```text
./apps/notes                       anchored to {MP_HOME}
{MP_HOME}\apps\notes\notes.exe     token
{MP_DATA}\dumps                    token
apps\tool                          left alone, resolves against Moonpool's working folder
```

```json title="apps.json"
{ "id": "notes", "name": "Notes", "group": "Desktop apps", "type": "desktop",
  "cwd": "./apps/notes",
  "command": "{MP_HOME}\\apps\\notes\\notes.exe",
  "processName": "notes" }
```

Taşınabilir klasörü taşıdığınızda iki biçim de çalışmaya devam eder. `C:\tools\notes` gibi sabit bir yol
taşınmaz. Taşınabilir modda Uygulamayı düzenle iletişim kutusu, mutlak `cwd` ve `url` değerlerini
"taşınabilir değil" rozetiyle işaretler. Bkz. [Taşınabilir mod](/tr/data/portable-mode/).

## Ortam

`env`, dizelerden oluşan bir nesnedir. İletişim kutusu bunu satır başına bir `KEY=VALUE` olarak düzenler;
her satırı ilk `=` işaretinde böler, iki yanı kırpar ve `=` içermeyen satırları yok sayar.

İletişim kutusunda:

```text
PORT=8091
NODE_ENV=development
```

`apps.json` içinde, girdinin `env` anahtarı olarak:

```json title="apps.json (one entry)"
{ "id": "habits", "name": "Habits", "group": "Web apps", "type": "web", "command": "python app.py",
  "env": { "PORT": "8091", "NODE_ENV": "development" } }
```

- Başlatılan komut, Moonpool'un ortamını ve `env` değerlerini devralır. `env` içindeki girdiler önceliklidir.
- `env` ayrıca `stopCommand` komutuna da uygulanır.
- Değerler yazıldığı gibi kullanılır: Moonpool `{MP_HOME}` genişletmesi veya `%VAR%` genişletmesi yapmaz.
- Moonpool kendi WebView2 bileşenini `WEBVIEW2_USER_DATA_FOLDER` ile özel bir profil klasörüne yönlendirir.
  Başlatılan uygulamalar bunu devralmaz. Moonpool'u başlatmadan önce bu değişkeni kendiniz ayarladıysanız
  onlar sizin değerinizi alır; aksi halde tanımsızdır. Bir `env` girdisi yine de bunu geçersiz kılabilir.

## Çalışma klasörü

Komut ve `stopCommand`, `cwd` içinde çalışır. `cwd` belirtilmezse komut Moonpool'un kendi çalışma
klasöründe çalışır; bu yüzden göreli yollar kullanan her şey için `cwd` ayarlayın.
