//! DeckCraft's settings, where its desktop binary keeps them on Linux
//! (`upstream/apps/deckcraft/src/main.rs`): `ui.json` and `prefs.json` in
//! `$XDG_CONFIG_HOME/deckcraft`. On Android, craft-android points `XDG_CONFIG_HOME` at the app's
//! private storage.

use std::path::PathBuf;

use deckcraft_ui_egui::SlideApp;
use deckcraft_ui_egui::theme::Brightness;

/// Runs without preferences (`DECKCRAFT_NO_PREFS`, as for the desktop binary) and tests neither
/// read nor write them.
fn enabled() -> bool {
    !cfg!(test) && std::env::var_os("DECKCRAFT_NO_PREFS").is_none()
}

/// DeckCraft's settings folder, unless preferences are off.
pub fn settings_dir() -> Option<PathBuf> {
    let config = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|p| !p.as_os_str().is_empty())
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
    enabled().then(|| config.join("deckcraft"))
}

/// Restore the saved interface state (recent presentations, panes, appearance) and preferences.
/// `false` when there was no saved interface state: a first start.
pub fn load(app: &mut SlideApp) -> bool {
    let Some(dir) = settings_dir() else { return false };
    let (ui, prefs) = deckcraft_ui_egui::settings::load(&dir);
    if let Some(prefs) = prefs {
        app.session.prefs = prefs;
    }
    match ui {
        Some(ui) => {
            app.ui = ui;
            true
        }
        None => false,
    }
}

pub fn save(app: &SlideApp) {
    if let Some(dir) = settings_dir() {
        deckcraft_ui_egui::settings::save(&dir, &app.ui, &app.session.prefs);
    }
}

/// DeckCraft's appearance is Light or Dark, with no "as the system" choice: a first start takes
/// the system's (Android's dark mode, also in DeX).
pub fn follow_system_theme(app: &mut SlideApp, system: Option<egui::Theme>) {
    if let Some(theme) = system {
        app.ui.brightness = brightness(theme);
    }
}

fn brightness(theme: egui::Theme) -> Brightness {
    match theme {
        egui::Theme::Dark => Brightness::Dark,
        egui::Theme::Light => Brightness::Light,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deckcraft_engine::Session;
    use deckcraft_ui_egui::Services;

    #[test]
    fn a_first_start_takes_the_systems_appearance() {
        let mut app = SlideApp::new(Session::new(), Services::default());
        assert!(!load(&mut app), "tests have no saved settings");
        follow_system_theme(&mut app, Some(egui::Theme::Dark));
        assert_eq!(app.ui.brightness, Brightness::Dark);
        follow_system_theme(&mut app, None);
        assert_eq!(app.ui.brightness, Brightness::Dark, "an unknown system appearance changes nothing");
        follow_system_theme(&mut app, Some(egui::Theme::Light));
        assert_eq!(app.ui.brightness, Brightness::Light);
    }
}
