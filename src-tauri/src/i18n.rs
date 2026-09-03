//! The *small* half of the localization story.
//!
//! Every string the user reads lives in the webview catalog
//! (`src/lib/locales/*.ts`) with exactly one exception: the tray menu. The tray
//! is built in `setup()`, before any window exists, and it stays up while every
//! window is closed, so its labels can never come from JS. Those few strings are
//! mirrored here.
//!
//! **The rule for adding to this file: don't, unless the webview genuinely
//! cannot render the string.** Native menus, native dialogs and OS notifications
//! qualify. Anything drawn inside a window does not. A second catalog is a second
//! place for translations to rot, so it stays as close to empty as possible.

/// Tray menu labels for one locale.
pub struct TrayStrings {
    pub show: &'static str,
    pub quit: &'static str,
}

/// Tray labels for a resolved locale tag. Unknown or partial tags fall back to
/// English rather than failing, so a settings.json from a newer build (or a
/// hand-edited one) can never leave the tray blank.
///
/// Matching is on the tag as written by the frontend, which has already done the
/// OS-language resolution and normalized the casing - see `resolveLocale` in
/// `src/lib/i18n.svelte.ts`. The bare-language prefix arm catches regional tags
/// we don't enumerate ("pt-PT" -> the "pt" arm's strings).
pub fn tray_strings(locale: &str) -> TrayStrings {
    let base = locale.split('-').next().unwrap_or("en");
    match (locale, base) {
        (_, "es") => TrayStrings {
            show: "Mostrar Moonpool",
            quit: "Salir",
        },
        (_, "fr") => TrayStrings {
            show: "Afficher Moonpool",
            quit: "Quitter",
        },
        (_, "de") => TrayStrings {
            show: "Moonpool anzeigen",
            quit: "Beenden",
        },
        (_, "pt") => TrayStrings {
            show: "Mostrar o Moonpool",
            quit: "Sair",
        },
        (_, "ja") => TrayStrings {
            show: "Moonpool を表示",
            quit: "終了",
        },
        (_, "zh") => TrayStrings {
            show: "显示 Moonpool",
            quit: "退出",
        },
        _ => TrayStrings {
            show: "Show Moonpool",
            quit: "Quit",
        },
    }
}
