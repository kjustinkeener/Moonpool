// Single source of truth for the help site's languages.
//
// The app's locale ids (src/lib/locales/*.ts, LOCALES in src/lib/i18n.svelte.ts) map to a
// help directory by ONE rule: lowercase the id ("pt-BR" -> "pt-br", "zh-Hans" -> "zh-hans").
// English is the root locale and has no prefix. The app side applies the same rule in
// `helpUrl()` (src/lib/api.ts). `lang` is the BCP-47 tag written to <html lang> and hreflang.
//
//   app locale  help dir   lang      URL
//   en          (root)     en        /
//   de          de         de        /de/
//   es          es         es        /es/
//   fr          fr         fr        /fr/
//   it          it         it        /it/
//   ja          ja         ja        /ja/
//   ko          ko         ko        /ko/
//   nl          nl         nl        /nl/
//   pl          pl         pl        /pl/
//   pt-BR       pt-br      pt-BR     /pt-br/
//   ru          ru         ru        /ru/
//   tr          tr         tr        /tr/
//   zh-Hans     zh-hans    zh-Hans   /zh-hans/
//   zh-Hant     zh-hant    zh-Hant   /zh-hant/
//
// Sidebar `translations` use the BCP-47 tag; Starlight's `locales` keys are the directory names.

/** [app locale id, endonym]. Order matches LOCALES in src/lib/i18n.svelte.ts. */
const NON_ENGLISH = [
	['de', 'Deutsch'],
	['es', 'Español'],
	['fr', 'Français'],
	['it', 'Italiano'],
	['ja', '日本語'],
	['ko', '한국어'],
	['nl', 'Nederlands'],
	['pl', 'Polski'],
	['pt-BR', 'Português (Brasil)'],
	['ru', 'Русский'],
	['tr', 'Türkçe'],
	['zh-Hans', '简体中文'],
	['zh-Hant', '繁體中文'],
];

/** Starlight `locales` option: root = English, every other key is the lowercase directory. */
export const starlightLocales = {
	root: { label: 'English', lang: 'en' },
	...Object.fromEntries(NON_ENGLISH.map(([id, label]) => [id.toLowerCase(), { label, lang: id }])),
};

/** Sidebar group labels, per language tag. Short labels. */
const GROUPS = {
	'Getting started': {
		de: 'Erste Schritte', es: 'Primeros pasos', fr: 'Prise en main', it: 'Per iniziare', ja: 'はじめに',
		ko: '시작하기', nl: 'Aan de slag', pl: 'Pierwsze kroki', 'pt-BR': 'Primeiros passos', ru: 'Начало работы',
		tr: 'Başlarken', 'zh-Hans': '入门', 'zh-Hant': '入門',
	},
	'How-to guides': {
		de: 'Anleitungen', es: 'Guías prácticas', fr: 'Guides pratiques', it: 'Guide pratiche', ja: 'ハウツーガイド',
		ko: '방법 안내', nl: 'Handleidingen', pl: 'Poradniki', 'pt-BR': 'Guias práticos', ru: 'Практические руководства',
		tr: 'Nasıl yapılır', 'zh-Hans': '操作指南', 'zh-Hant': '操作指南',
	},
	'Using Moonpool': {
		de: 'Moonpool verwenden', es: 'Usar Moonpool', fr: 'Utiliser Moonpool', it: 'Usare Moonpool', ja: 'Moonpool の使い方',
		ko: 'Moonpool 사용', nl: 'Moonpool gebruiken', pl: 'Korzystanie z Moonpool', 'pt-BR': 'Usando o Moonpool', ru: 'Работа с Moonpool',
		tr: 'Moonpool kullanımı', 'zh-Hans': '使用 Moonpool', 'zh-Hant': '使用 Moonpool',
	},
	'Configuring apps': {
		de: 'Apps konfigurieren', es: 'Configurar aplicaciones', fr: 'Configurer les applications', it: 'Configurare le app', ja: 'アプリの設定',
		ko: '앱 구성', nl: 'Apps configureren', pl: 'Konfiguracja aplikacji', 'pt-BR': 'Configurando apps', ru: 'Настройка приложений',
		tr: 'Uygulamaları yapılandırma', 'zh-Hans': '配置应用', 'zh-Hant': '設定應用程式',
	},
	'Data, updates and recovery': {
		de: 'Daten, Updates und Wiederherstellung', es: 'Datos, actualizaciones y recuperación', fr: 'Données, mises à jour et récupération',
		it: 'Dati, aggiornamenti e ripristino', ja: 'データ、更新、復旧', ko: '데이터, 업데이트 및 복구', nl: 'Gegevens, updates en herstel',
		pl: 'Dane, aktualizacje i odzyskiwanie', 'pt-BR': 'Dados, atualizações e recuperação', ru: 'Данные, обновления и восстановление',
		tr: 'Veriler, güncellemeler ve kurtarma', 'zh-Hans': '数据、更新与恢复', 'zh-Hant': '資料、更新與復原',
	},
	'Automation and AI agents': {
		de: 'Automatisierung und KI-Agenten', es: 'Automatización y agentes de IA', fr: 'Automatisation et agents IA',
		it: 'Automazione e agenti IA', ja: '自動化と AI エージェント', ko: '자동화와 AI 에이전트', nl: 'Automatisering en AI-agents',
		pl: 'Automatyzacja i agenci AI', 'pt-BR': 'Automação e agentes de IA', ru: 'Автоматизация и ИИ-агенты',
		tr: 'Otomasyon ve yapay zeka ajanları', 'zh-Hans': '自动化与 AI 代理', 'zh-Hant': '自動化與 AI 代理',
	},
	'Platform notes': {
		de: 'Hinweise zu Plattformen', es: 'Notas por plataforma', fr: 'Notes par plateforme', it: 'Note sulle piattaforme', ja: 'プラットフォーム別の注意点',
		ko: '플랫폼별 참고', nl: 'Platformnotities', pl: 'Uwagi o platformach', 'pt-BR': 'Notas de plataforma', ru: 'Заметки о платформах',
		tr: 'Platform notları', 'zh-Hans': '平台说明', 'zh-Hant': '平台說明',
	},
	Support: {
		de: 'Support', es: 'Soporte', fr: 'Assistance', it: 'Supporto', ja: 'サポート', ko: '지원', nl: 'Ondersteuning', pl: 'Pomoc',
		'pt-BR': 'Suporte', ru: 'Поддержка', tr: 'Destek', 'zh-Hans': '支持', 'zh-Hant': '支援',
	},
};

/** A sidebar group: English label plus its translations. */
export const group = (label, items) => ({ label, translations: GROUPS[label], items });

/**
 * A sidebar page link. `de` is the German label; other languages fall back to the English
 * label until their translator adds an entry (pass more tags via `extra`).
 */
export const page = (slug, label, de, extra = {}) => ({
	label,
	slug,
	translations: { ...(de ? { de } : {}), ...extra },
});

/** Strings in the web-only Starlight overrides (src/components/*.astro), per language tag. */
export const WEB_STRINGS = {
	en: { download: 'Download', footer: 'Moonpool is free software from' },
	de: { download: 'Download', footer: 'Moonpool ist freie Software von' },
	es: { download: 'Descargar', footer: 'Moonpool es software gratuito de' },
	fr: { download: 'Télécharger', footer: 'Moonpool est un logiciel gratuit de' },
	it: { download: 'Scarica', footer: 'Moonpool è software gratuito di' },
	ja: { download: 'ダウンロード', footer: 'Moonpool は次の会社が提供する無料ソフトウェアです:' },
	ko: { download: '다운로드', footer: 'Moonpool은 다음에서 제공하는 무료 소프트웨어입니다:' },
	nl: { download: 'Downloaden', footer: 'Moonpool is gratis software van' },
	pl: { download: 'Pobierz', footer: 'Moonpool to darmowe oprogramowanie od' },
	'pt-BR': { download: 'Baixar', footer: 'O Moonpool é um software gratuito da' },
	ru: { download: 'Скачать', footer: 'Moonpool: бесплатное программное обеспечение от' },
	tr: { download: 'İndir', footer: 'Moonpool, şu kaynaktan ücretsiz bir yazılımdır:' },
	'zh-Hans': { download: '下载', footer: 'Moonpool 是来自以下团队的免费软件：' },
	'zh-Hant': { download: '下載', footer: 'Moonpool 是來自以下團隊的免費軟體：' },
};
