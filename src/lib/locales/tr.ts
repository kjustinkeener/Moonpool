// Türkçe. Keys absent here fall back to English at runtime.
import type { PartialDict } from "../i18n.svelte";

export const tr: PartialDict = {
  "common.close": "Kapat",
  "common.cancel": "İptal",
  "common.save": "Kaydet",
  "common.delete": "Sil",
  "common.edit": "Düzenle",
  "common.rename": "Yeniden adlandır",
  "common.reload": "Yeniden yükle",
  "common.settings": "Ayarlar",
  "common.about": "Hakkında",
  "common.tryAgain": "Yeniden dene",
  "common.dismiss": "Yoksay",
  "common.copied": "Kopyalandı!",
  "common.autoSystem": "Otomatik (sistem)",

  "titlebar.minimize": "Simge durumuna küçült",
  "titlebar.maximize": "Ekranı kapla",
  "titlebar.restore": "Geri yükle",

  "sidebar.menu": "Menü",
  "sidebar.addApp": "Uygulama ekle",
  "sidebar.editJson": "apps.json dosyasını düzenle",
  "sidebar.installMoonpool": "Moonpool'u kur…",
  "sidebar.filterPlaceholder": "Uygulamaları filtrele...",
  "sidebar.filterLabel": "Uygulamaları filtrele",
  "sidebar.showCli": "CLI panelini göster",
  "sidebar.updateOpenToInstall": "Güncelleme var, kurmak için açın",
  "sidebar.updateShowCli": "Güncelleme var, CLI panelini gösterin",
  "sidebar.statusRunning": "çalışıyor",
  "sidebar.statusStarting": "başlatılıyor...",
  "sidebar.statusStopped": "durduruldu",
  "sidebar.working": "Çalışılıyor...",
  "sidebar.restart": "Yeniden başlat",
  "sidebar.stop": "Durdur",
  "sidebar.launch": "Başlat",
  "sidebar.setIcon": "Simge seç...",
  "sidebar.portConflict": "{names} aynı {port} bağlantı noktasını kullanıyor",
  "sidebar.portConflictBadge": "bağlantı noktası {port}: {names}",
  "sidebar.noMatch": "“{filter}” ile eşleşen uygulama yok.",

  "app.dragToResize": "Yeniden boyutlandırmak için sürükleyin",
  "app.closeTab": "Sekmeyi kapat",
  "app.hideCli": "CLI panelini gizle",
  "app.pickApp": "Başlatmak için soldaki listeden bir uygulama seçin.",
  "app.updateAvailable": "Moonpool {version} yayınlandı (sizdeki sürüm: {current}).",
  "app.downloading": "{version} indiriliyor…",
  "app.installing": "Kuruluyor…",
  "app.downloadInstall": "İndir ve kur",
  "app.updateInstalled": "Güncelleme kuruldu. Yeniden başlatılıyor…",
  "app.updateFailed": "Güncelleme başarısız: {error}",
  "app.confirmDelete": "“{name}” silinsin mi?",
  "app.iconDialogTitle": "{name} için simge",
  "app.imagesFilter": "Görseller",
  "app.newHere":
    "Yeni mi başladınız? Uygulamalarınızı ayarlaması için bunu bir yapay zeka aracısına verin:",
  "app.copyPrompt": "İstemi kopyala",
  "app.editFile": "Dosyayı düzenle",
  "app.orUseHint":
    "Ya da yukarıdaki {add} düğmesini kullanın veya {file} dosyasını doğrudan düzenlemek için {edit} seçeneğine tıklayın.",

  "about.tagline":
    "Yerel uygulamalarınız ve geliştirme sunucularınız için, her uygulamaya gömülü bir terminal sunan sistem tepsisi başlatma merkezi.",
  "about.version": "sürüm {version}",
  "about.checkUpdates": "Güncellemeleri denetle",
  "about.checking": "Güncellemeler denetleniyor...",
  "about.upToDate": "En son sürümü kullanıyorsunuz.",
  "about.updateDownloading": "{version} güncellemesi mevcut - indiriliyor...",
  "about.updateInstalled": "Güncelleme kuruldu. Yeniden başlatılıyor...",
  "about.checkFailed": "Güncelleme denetimi başarısız: {error}",
  "about.builtWith": "Şunlarla geliştirildi",
  "about.byLine": "MIT License · geliştiren: {author}",

  "settings.title": "Ayarlar",
  "settings.theme": "Tema",
  "settings.language": "Dil",
  "settings.languageHint": "Moonpool'un kendi metinlerinin gösterileceği dil.",
  "settings.closeToTray": "Kapatınca tepsiye gizle",
  "settings.closeToTrayHint":
    "Pencereyi kapatmak Moonpool'u sistem tepsisine gizler (görev çubuğundan kalkar). Kapalıyken: kapatmak uygulamadan çıkar.",
  "settings.minimizeToTray": "Küçültünce tepsiye gizle",
  "settings.minimizeToTrayHint":
    "Küçültmek Moonpool'u sistem tepsisine gizler (görev çubuğundan kalkar). Kapalıyken: görev çubuğuna küçültür.",
  "settings.alwaysOnTop": "Her zaman üstte",
  "settings.alwaysOnTopHint":
    "Moonpool'u ve Ayarlar/Hakkında pencerelerini diğer pencerelerin üstünde tutar.",
  "settings.transparency": "Arka plan saydamlığı",
  "settings.transparencyHint": "Pencere arka planı saydam olur. %0 tamamen opaktır.",
  "settings.checkOnStartup": "Başlangıçta güncellemeleri denetle",
  "settings.checkOnStartupHint":
    "Açılışta GitHub'da sessizce yeni bir sürüm olup olmadığına bakar ve varsa bir bildirim şeridi gösterir.",
  "settings.debugLogging": "Hata ayıklama bilgilerini bir dosyaya kaydet",
  "settings.debugLoggingHint":
    "Manifest yüklemelerini, başlatmaları ve hataları {file} dosyasına kaydeder.",
  "settings.openLog": "Günlüğü aç",

  "theme.dark": "Koyu",
  "theme.light": "Açık",

  "installer.tagline":
    "Yerel uygulamalarınız ve geliştirme sunucularınız için bir tepsi başlatıcısı.",
  "installer.pickFolder": "Taşınabilir Moonpool için bir klasör seçin",
  "installer.poke": "dürtmek için sol tıklayın, sıfırlamak için sağ tıklayın",
  "installer.installed": "Kuruldu, Moonpool başlatılıyor…",
  "installer.portableDone": "Taşınabilir mod, Moonpool başlatılıyor…",
  "installer.failed": "Kurulum başarısız: {error}",
  "installer.alreadyInstalledTitle": "Bu kopya bu bilgisayarda zaten kurulu.",
  "installer.alreadyInstalled": "Zaten kurulu",
  "installer.installing": "Kuruluyor…",
  "installer.install": "Moonpool'u kur",
  "installer.desktopShortcut": "Masaüstüne kısayol ekle",
  "installer.portableHint":
    "Moonpool'u seçtiğiniz bir klasörden çalıştırın (USB bellek, zip) ve istediğiniz yere taşıyın. Veriler exe dosyasının yanında kalır.",
  "installer.installPortable": "Taşınabilir olarak kur",
  "installer.installPath": "Kurulum yolu: {dir}",

  "editor.addApp": "Uygulama ekle",
  "editor.editApp": "Uygulamayı düzenle",
  "editor.backToList": "listeye dön",
  "editor.nameHint": "Kenar çubuğunda görünen ad. Zorunlu.",
  "editor.namePlaceholder": "Uygulamam",
  "editor.groupHint":
    "Bu uygulamanın kenar çubuğunda altında listeleneceği başlık. Mevcut bir grubu seçin ya da yeni bir tane eklemek için '+ Yeni grup' seçeneğini kullanın.",
  "editor.newGroupPlaceholder": "Yeni grup adı",
  "editor.pickExistingGroup": "Mevcut bir grup seçin",
  "editor.newGroupOption": "+ Yeni grup...",
  "editor.typeHint":
    "Moonpool'un uygulamayı nasıl çalıştırıp izleyeceği. web = bir bağlantı noktasında çalışan geliştirme sunucusu. desktop = süreç adına göre izlenen yerel uygulama. static = yalnızca bir URL açar. cli = terminalde bir komut çalıştırır.",
  "editor.hintWeb":
    "Terminalde bir geliştirme sunucusu çalıştırır; bağlantı noktası yanıt verdiğinde Çalışıyor olarak gösterir ve sunucu ayağa kalkar kalkmaz tarayıcıyı açar.",
  "editor.hintDesktop":
    "Yerel bir uygulama başlatır; processName adında bir süreç bulunduğunda Çalışıyor olarak gösterir.",
  "editor.hintStatic": "Yalnızca url adresini tarayıcıda açar; terminal ya da komut yoktur.",
  "editor.hintCli": "Bir komut çalıştırır ve cwd içinde etkileşimli bir kabuğu açık tutar.",
  "editor.portableWarn":
    "Mutlak yol - bu klasörle birlikte taşınmaz. Taşınabilir kalması için {MP_HOME}\\... ya da ./ ile başlayan bir yol kullanın.",
  "editor.notPortable": "taşınabilir değil",
  "editor.cwdHint":
    "Komutun çalıştırılacağı çalışma dizini; genellikle uygulamanın proje klasörüdür. Mutlak bir yol kullanın ya da taşınabilir kalması için {MP_HOME}\\... / ./ yazın.",
  "editor.commandHint":
    "Uygulamayı başlatmak için gömülü terminalde çalıştırılan komut, örneğin 'npm run dev' ya da 'python app.py'. Yalnızca statik URL girdisi için boş bırakın.",
  "editor.portHint":
    "Uygulamanın dinlediği yerel TCP bağlantı noktası. Bu bağlantı noktası yanıt verdiğinde Moonpool uygulamayı Çalışıyor olarak gösterir, Durdur ile de bu noktayı serbest bırakır. Web uygulamalarında kullanılır.",
  "editor.processNameHint":
    "Masaüstü uygulamaları için: Çalışıyor durumunu algılamak ve uygulamayı durdurmak için kullanılan süreç/çalıştırılabilir dosya adı (.exe olmadan). Linux'ta en fazla 15 karakter olabilir.",
  "editor.urlHint":
    "Açılacak URL: bir web uygulaması için http://localhost:<port>, statik bir sayfa için file:///path/to/index.html. Taşınabilir kalması için file:///{MP_HOME}/... kullanın.",
  "editor.openBrowser": "tarayıcıyı aç",
  "editor.openBrowserHint":
    "Uygulama erişilebilir hale geldiğinde URL'yi varsayılan tarayıcınızda otomatik olarak açar.",
  "editor.envLabel": "env (her satıra bir KEY=VALUE)",
  "editor.envHint":
    "Komuta aktarılan ortam değişkenleri; her satıra bir KEY=VALUE (örneğin PORT=3000).",
  "editor.noteHint":
    "Kenar çubuğunda bu uygulamanın üzerine gelindiğinde ipucu olarak gösterilen isteğe bağlı metin.",
  "editor.notePlaceholder": "isteğe bağlı ipucu",
  "editor.phFolder": "uygulama klasörünün yolu",
  "editor.phLaunchCommand": "uygulamanın başlatma komutu",
  "editor.phCommand": "çalıştırılacak komut",
  "editor.discardChanges": "Değişiklikleriniz atılsın mı?",
  "editor.nameRequired": "ad zorunludur.",

  "term.processExited": "[süreç sona erdi]",
};
