//! DexDeckCraft in a desktop window, to iterate without a device:
//! `cargo run -p dexdeckcraft --features desktop -- [--sample] [talk.pptx…]`. No system pickers,
//! clipboard pictures or sound here (the Android build provides them).

fn main() {
    let mut launch = dexdeckcraft::Launch::default();
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--sample" => launch.sample = true,
            _ => launch.files.push(arg),
        }
    }
    dexdeckcraft::run(dexdeckcraft::native_options, |cc| {
        log::info!("dexdeckcraft: {}", dexdeckcraft::renderer_line(cc));
        let services = deckcraft_ui_egui::Services {
            read: Some(Box::new(|p: &str| std::fs::read(p).map_err(|e| e.to_string()))),
            write: Some(Box::new(|p: &str, b: &[u8]| std::fs::write(p, b).map_err(|e| e.to_string()))),
            ..Default::default()
        };
        let app = dexdeckcraft::DeckApp::new(cc, &launch, services);
        Box::new(
            craft_android::Shell::new(app)
                .on_focus_lost(|a| a.focus_lost())
                .title(dexdeckcraft::DeckApp::title)
                .on_theme_change(dexdeckcraft::on_theme_change),
        )
    });
}
