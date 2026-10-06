---
title: "Yaygın uygulama kurulumları için kopyala-yapıştır apps.json örnekleri"
description: "Geliştirme sunucusu, masaüstü uygulaması, statik sayfa, CLI aracı, Docker Compose ve taşınabilir uygulama için kopyalayıp uyarlayabileceğiniz eksiksiz, geçerli apps.json girdileri."
---

Her parça tek bir girdidir. Bunları `apps.json` dosyasının en üstteki dizisinin içine, virgülle ayırarak
koyun. Kimlikleri, adları ve yolları kendi kurulumunuza göre değiştirin.

## Web geliştirme sunucusu

5173 numaralı bağlantı noktası yanıt verdiği sürece Çalışıyor görünür. Yanıt verince tarayıcı açılır. Durdur
ayrıca bağlantı noktasını da serbest bırakır; bu, `web` için varsayılandır.

```json title="apps.json"
{
  "id": "site",
  "name": "Site",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\site",
  "command": "npm run dev",
  "port": 5173,
  "url": "http://localhost:5173",
  "openBrowser": true
}
```

## Bağlantı noktasını ortamdan okuyan web uygulaması

```json title="apps.json"
{
  "id": "habit-tracker",
  "name": "Habit Tracker",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\habits",
  "command": "python app.py",
  "port": 8091,
  "url": "http://127.0.0.1:8091",
  "openBrowser": true,
  "env": { "PORT": "8091" },
  "note": "Moved off 8000 to avoid a clash"
}
```

## Masaüstü uygulaması

`notes-app` adlı bir süreç var olduğu sürece Çalışıyor görünür. Durdur, bu süreci adıyla sonlandırır.

```json title="apps.json"
{
  "id": "notes-app",
  "name": "Notes App",
  "group": "Desktop apps",
  "type": "desktop",
  "cwd": "C:\\code\\notes-app",
  "command": "npm run tauri dev",
  "processName": "notes-app"
}
```

## Zaten barındırılan statik sayfa

Terminal yok. Başlat sayfayı açar.

```json title="apps.json"
{
  "id": "team-board",
  "name": "Team board",
  "group": "Docs",
  "type": "static",
  "url": "https://example.com/board"
}
```

## Bir komutla sunulan statik klasör

Moonpool sunucuyu bir terminalde çalıştırır, bağlantı noktasından izler ve yanıt verince sayfayı açar.

```json title="apps.json"
{
  "id": "docs-site",
  "name": "Docs",
  "group": "Docs",
  "type": "static",
  "cwd": "C:\\code\\docs\\public",
  "command": "python -m http.server 8090",
  "port": 8090,
  "url": "http://localhost:8090",
  "openBrowser": true,
  "killMode": "port"
}
```

## CLI aracı

Bir terminal sekmesinde çalışır. `-NoExit`, betik bittikten sonra kabuğu açık tutar.

```json title="apps.json"
{
  "id": "backup",
  "name": "Backup script",
  "group": "CLI tools",
  "type": "cli",
  "cwd": "C:\\code\\scripts",
  "command": "pwsh -NoLogo -NoProfile -NoExit -Command .\\backup.ps1 -Verbose"
}
```

## Docker Compose

`command` kapsayıcıyı zaten yeniden oluşturup çıkar; bu yüzden Çalışıyor durumu bağlantı noktasından gelir. Durdur,
bağlantı noktasının sahibini sonlandırmak yerine `stopCommand` komutunu çalıştırır; Windows'ta o sahip
Docker Desktop olurdu. Bkz. [Durdurma ve yeniden başlatma](/tr/apps/stop-and-restart/#windowsta-docker-uygulamaları).

```json title="apps.json"
{
  "id": "api",
  "name": "API",
  "group": "Web apps",
  "type": "web",
  "cwd": "C:\\code\\api",
  "command": "docker compose up -d --build",
  "port": 8080,
  "url": "http://localhost:8080",
  "killMode": "command",
  "stopCommand": "docker compose stop app"
}
```

Uygulamayı durdurduğunuzda kapsayıcının çalışır durumda kalmasını tercih ederseniz
`"killMode": "none"` kullanın ve `stopCommand` satırını kaldırın.

## Taşınabilir uygulama

Yollar taşınabilir klasöre sabitlenir; böylece klasör taşındıktan sonra da girdi çalışmaya devam eder.

```json title="apps.json"
{
  "id": "notes",
  "name": "Notes",
  "group": "Desktop apps",
  "type": "desktop",
  "cwd": "./apps/notes",
  "command": "{MP_HOME}\\apps\\notes\\notes.exe",
  "processName": "notes",
  "icon": "{MP_HOME}\\icons\\notes.png"
}
```

## Ayrıca bakın

- [Örnek panolar](/tr/getting-started/example-dashboards/): Moonpool ile birlikte gelen panolar.
- [Uygulama alanları](/tr/apps/fields/)
- [Windows'ta bir npm geliştirme sunucusunu arka planda çalıştırma](/tr/guides/run-npm-dev-server-in-background-windows/)
- [Windows'ta bir Python betiğini arka planda çalışır tutma](/tr/guides/keep-python-script-running-background-windows/)
