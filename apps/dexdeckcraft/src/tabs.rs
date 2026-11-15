//! The presentation tabs below DeckCraft's ribbon: craft-android's tab strip
//! ([`craft_android::tabs`]) drawn through the fork's `Services::below_ribbon`, one tab per open
//! presentation, since the activity's one window shows them all. What a tab asks for runs as
//! DeckCraft's own commands, so closing a tab with unsaved changes asks Save / Don't Save /
//! Cancel exactly as File › Close does.

use craft_android::tabs::{Tab, TabAction, TabStrip};
use deckcraft_ui_egui::SlideApp;
use serde_json::{Value, json};

/// Draw the tabs and run what was clicked. No tabs in a window with no presentation open.
pub fn show(app: &mut SlideApp, ui: &mut egui::Ui) {
    let docs = app.session.documents();
    if docs.is_empty() {
        return;
    }
    let active = app.session.active().map(|d| d.uid);
    let ids: Vec<u64> = docs.iter().map(|d| d.uid).collect();
    let tabs: Vec<Tab> = docs.iter().map(|d| Tab::new(d.uid, d.title(), d.is_dirty(), active == Some(d.uid))).collect();
    let out = TabStrip::new(&tabs).new_tooltip("New presentation").close_tooltip("Close").show_panel(ui);
    let Some(action) = out.action else { return };
    for (command, params) in commands(action, &ids, active) {
        if let Err(e) = app.run(command, params) {
            log::warn!("dexdeckcraft: tab {action:?}: {command}: {e}");
            break;
        }
    }
}

/// The DeckCraft commands for a tab's action, given the open presentations' ids in order and the
/// shown one. Closing another presentation's tab shows it first, so Close (and its Save prompt)
/// is about that one. An id that is no longer open does nothing.
pub fn commands(action: TabAction, ids: &[u64], active: Option<u64>) -> Vec<(&'static str, Value)> {
    let switch = |id: u64| ids.iter().position(|i| *i == id).map(|index| ("window.next", json!({"index": index})));
    match action {
        TabAction::Activate(id) => switch(id).into_iter().collect(),
        TabAction::Close(id) if active == Some(id) => vec![("app.closePresentation", json!({}))],
        TabAction::Close(id) => switch(id).into_iter().chain([("app.closePresentation", json!({}))]).collect(),
        TabAction::New => vec![("file.new", json!({}))],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use deckcraft_engine::Session;
    use deckcraft_ui_egui::Services;

    #[test]
    fn each_action_is_a_deckcraft_command() {
        let ids = [7, 9];
        assert_eq!(commands(TabAction::Activate(9), &ids, Some(7)), [("window.next", json!({"index": 1}))]);
        assert_eq!(commands(TabAction::Close(7), &ids, Some(7)), [("app.closePresentation", json!({}))]);
        assert_eq!(commands(TabAction::Close(9), &ids, Some(7)), [("window.next", json!({"index": 1})), ("app.closePresentation", json!({}))]);
        assert_eq!(commands(TabAction::New, &ids, None), [("file.new", json!({}))]);
        assert!(commands(TabAction::Activate(3), &ids, Some(7)).is_empty(), "a presentation that is gone");
    }

    fn run(app: &mut SlideApp, action: TabAction) {
        let ids: Vec<u64> = app.session.documents().iter().map(|d| d.uid).collect();
        let active = app.session.active().map(|d| d.uid);
        for (command, params) in commands(action, &ids, active) {
            app.run(command, params).unwrap();
        }
    }

    /// The commands do what the tabs promise on DeckCraft itself: switch, new, and close a
    /// presentation without changes at once while one with changes asks first.
    #[test]
    fn the_commands_switch_open_and_close_presentations() {
        let mut app = SlideApp::new(Session::with_new(), Services::default());
        let first = app.session.active().unwrap().uid;
        app.run("slide.new", json!({})).unwrap();
        run(&mut app, TabAction::New);
        let second = app.session.active().unwrap().uid;
        assert_ne!(first, second);
        assert_eq!(app.session.documents().len(), 2);
        run(&mut app, TabAction::Activate(first));
        assert_eq!(app.session.active().unwrap().uid, first);
        // The untouched second presentation closes from its tab without a question.
        run(&mut app, TabAction::Close(second));
        assert_eq!(app.session.documents().len(), 1);
        assert_eq!(app.session.active().unwrap().uid, first);
        // The first one has unsaved changes: closing it asks, and leaves it open meanwhile.
        run(&mut app, TabAction::Close(first));
        assert_eq!(app.session.documents().len(), 1, "a presentation with changes is not closed without asking");
        assert!(app.dialog.is_some(), "the Save / Don't Save / Cancel prompt is up");
    }
}
