//! Hand-rolled window size/position persistence for the main window.
//!
//! Replaces `tauri-plugin-window-state`: the plugin only let us override the state
//! *filename*, not its directory, and always `create_dir_all`ed `%APPDATA%\<id>` even
//! in portable mode. Rolling our own lets the state live next to the rest of the
//! config (`moonpool_dir`) in both modes - `%APPDATA%\Moonpool\window-state.json` when
//! installed, `{MP_DATA}\window-state.json` in a portable bundle - so nothing leaks
//! outside a portable folder and there's no stray AppData directory.
//!
//! Scope is deliberately small: physical size + position + a maximized flag for the
//! `main` window. A missing or corrupt file falls back to the config default size/pos
//! (1200x780, OS-centered) by simply doing nothing on restore.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::{Manager, PhysicalPosition, PhysicalSize, WebviewWindow};

/// A real window is never smaller than this; used to reject degenerate saved state
/// (the borderless minimize sliver) and to validate a file before restoring it.
const MIN_DIM: u32 = 300;

#[derive(Clone, Serialize, Deserialize, Default)]
struct WinState {
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    #[serde(default)]
    maximized: bool,
}

/// `<moonpool_dir>/window-state.json`, mode-aware via `moonpool_dir`.
fn state_file(app: &tauri::AppHandle) -> Option<PathBuf> {
    crate::moonpool_dir(app).map(|d| d.join("window-state.json"))
}

/// Read the saved state, or None if absent / unreadable / corrupt (so callers fall
/// back to the window's config defaults).
fn load(path: &PathBuf) -> Option<WinState> {
    let text = std::fs::read_to_string(path).ok()?;
    serde_json::from_str::<WinState>(&text).ok()
}

/// Whether the saved rect overlaps any currently-connected monitor, so we don't
/// restore the window onto a display that has since been unplugged (stranding it
/// off-screen). If we can't enumerate monitors, assume it's fine.
fn on_screen(window: &WebviewWindow, s: &WinState) -> bool {
    let monitors = match window.available_monitors() {
        Ok(m) if !m.is_empty() => m,
        _ => return true,
    };
    let (l, t) = (s.x, s.y);
    let (r, b) = (s.x + s.width as i32, s.y + s.height as i32);
    monitors.iter().any(|m| {
        let mp = m.position();
        let ms = m.size();
        let (ml, mt) = (mp.x, mp.y);
        let (mr, mb) = (mp.x + ms.width as i32, mp.y + ms.height as i32);
        // Standard AABB overlap test.
        l < mr && r > ml && t < mb && b > mt
    })
}

/// Restore the main window's geometry from disk. No-op (keep config defaults) when
/// there's no file, it's corrupt, or the saved size is degenerate. Call in `setup`
/// before the poller starts.
pub fn restore(window: &WebviewWindow) {
    let Some(path) = state_file(window.app_handle()) else {
        return;
    };
    let Some(s) = load(&path) else {
        return; // absent or corrupt -> config default size/pos
    };
    if s.width < MIN_DIM || s.height < MIN_DIM {
        return;
    }
    let _ = window.set_size(PhysicalSize::new(s.width, s.height));
    // Only reposition if the saved spot is still on a live monitor.
    if on_screen(window, &s) {
        let _ = window.set_position(PhysicalPosition::new(s.x, s.y));
    }
    if s.maximized {
        let _ = window.maximize();
    }
}

/// Persist the main window's geometry. Callers already gate this behind the
/// not-minimized / above-floor guard, so `save` trusts the current geometry as sane.
/// When the window is maximized we keep the previously-saved *normal* bounds and only
/// flip the flag, so a later unmaximize restores a reasonable size.
pub fn save(window: &WebviewWindow) {
    let Some(path) = state_file(window.app_handle()) else {
        return;
    };
    let maximized = window.is_maximized().unwrap_or(false);
    let mut s = load(&path).unwrap_or_default();
    if maximized {
        s.maximized = true;
    } else {
        if let (Ok(sz), Ok(pos)) = (window.inner_size(), window.outer_position()) {
            s.width = sz.width;
            s.height = sz.height;
            s.x = pos.x;
            s.y = pos.y;
        }
        s.maximized = false;
    }
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(&s) {
        let _ = std::fs::write(&path, json);
    }
}
