//! DexDeckCraft: DeckCraft's desktop interface on Android, for Samsung DeX and Android's desktop
//! windowing. The Android app (`android/`) is a `GameActivity` (craft-android's `CraftActivity`)
//! that loads this library and calls `android_main`; egui draws the whole window with wgpu
//! (Vulkan or GLES), or with OpenGL ES through glow when wgpu can't start.
//!
//! The window is DeckCraft's own interface ([`SlideApp`]), started as its desktop binary starts
//! it (`upstream/apps/deckcraft/src/main.rs`), in the activity's one window: Presenter View and
//! View › New Window, which the desktop opens as native windows, are egui windows inside it.

pub mod prefs;
pub mod tabs;

#[cfg(target_os = "android")]
pub(crate) mod android;

use deckcraft_engine::Session;
use deckcraft_ui_egui::{Services, SlideApp};

/// What to start with.
#[derive(Clone, Debug, Default)]
pub struct Launch {
    /// Presentations to open (paths). A PowerPoint Show (`.ppsx`) given last starts its show.
    pub files: Vec<String>,
    /// Open DeckCraft's sample presentation.
    pub sample: bool,
}

/// What DeckCraft's Open pickers offer for a `purpose` (`open`, `picture`, `audio`, `video`,
/// `theme`), as its desktop binary's file dialogs do: (name, extensions).
pub fn open_filters(purpose: &str) -> (&'static str, &'static [&'static str]) {
    match purpose {
        "picture" => ("Pictures", &["png", "jpg", "jpeg", "gif", "webp", "bmp", "tif", "tiff"]),
        "audio" => ("Audio", &["wav", "mp3", "m4a", "m4b", "aac", "flac", "ogg", "oga", "opus", "aif", "aiff", "aifc", "caf", "wma", "weba", "mka"]),
        "video" => ("Video", &["mp4", "m4v", "mov", "webm", "mkv", "wmv"]),
        "theme" => ("Themes and presentations", &["thmx", "pptx", "potx", "ppsx"]),
        _ => ("Presentations", &["deckcraft", "pptx", "potx", "ppsx", "txt", "md"]),
    }
}

/// The extension a Save picker offers for a suggested name (`Talk.pdf` → `pdf`); a name without
/// one is a DeckCraft presentation.
pub fn save_extension(suggested: &str) -> String {
    std::path::Path::new(suggested)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .map(str::to_ascii_lowercase)
        .filter(|e| !e.is_empty())
        .unwrap_or_else(|| "deckcraft".to_owned())
}

/// DeckCraft's interface as an eframe app: what the desktop binary's `App` does, minus the
/// control channel, the Apple events, the self-update and the window decorations.
pub struct DeckApp {
    pub slide: SlideApp,
    /// Where File › Print's PDF goes instead of the PDF viewer the desktop opens (`None`: as
    /// the desktop). The link is `file://…pdf`; see [`DeckApp::ui`].
    print: Option<PrintFn>,
}

/// Takes the PDF File › Print composed (its path) and the presentation's title.
type PrintFn = Box<dyn FnMut(&std::path::Path, &str)>;

impl DeckApp {
    /// DeckCraft as its desktop binary starts it, with the host's `services` and the
    /// presentation tabs below the ribbon ([`tabs`]).
    pub fn new(cc: &eframe::CreationContext<'_>, launch: &Launch, mut services: Services) -> DeckApp {
        deckcraft_ui_egui::install_ime_pass_hooks(&cc.egui_ctx);
        services.below_ribbon = Some(Box::new(tabs::show));
        let mut slide = SlideApp::new(Session::new(), services);
        if !prefs::load(&mut slide) {
            // The first start: the interface takes the system's light or dark appearance. Later
            // ones keep what the title bar's Light/Dark button left.
            prefs::follow_system_theme(&mut slide, cc.egui_ctx.system_theme());
        }
        // AutoRecover: reopen what a previous run left unsaved, then keep it current. On Android
        // it is what keeps the work when the system closes the app (DeX's X can't be
        // intercepted, and a background app can be killed).
        if std::env::var_os("DECKCRAFT_NO_RECOVERY").is_none() {
            slide.session.recovery_dir = deckcraft_engine::recovery::default_dir();
            if let Ok(v) = slide.session.execute("file.recovery.open", &serde_json::json!({}))
                && let Some(n) = v.get("opened").and_then(|o| o.as_array()).map(Vec::len).filter(|n| *n > 0)
            {
                slide.set_status(format!("Recovered {n} unsaved presentation{} from the last session", if n == 1 { "" } else { "s" }));
            }
        }
        if launch.sample
            && let Err(e) = deckcraft_engine::sample::open_sample(&mut slide.session)
        {
            log::error!("dexdeckcraft: sample: {e}");
        }
        let mut deck = DeckApp { slide, print: None };
        deck.open(&launch.files);
        deck
    }

    /// Send File › Print's PDF to `print` (the path of the PDF and the presentation's title).
    pub fn with_print(mut self, print: impl FnMut(&std::path::Path, &str) + 'static) -> DeckApp {
        self.print = Some(Box::new(print));
        self
    }

    /// The window title DeX shows in its taskbar: the presentation's, as DeckCraft's title bar.
    pub fn title(&self) -> String {
        match self.slide.session.active() {
            Some(d) => format!("{}{} — DexDeckCraft", d.title(), if d.is_dirty() { " — Edited" } else { "" }),
            None => "DexDeckCraft".to_owned(),
        }
    }

    /// Open presentations as the system hands them over ("Open with", shared files, the command
    /// line): the last one starts its show if it is a PowerPoint Show. One that can't be opened
    /// says why in a toast, which also shows on the start screen.
    pub fn open(&mut self, paths: &[String]) {
        for (path, error) in self.slide.open_from_system(paths) {
            log::error!("dexdeckcraft: {path}: {error}");
            self.slide.notify(error);
        }
    }

    /// The preferences, written now: Android may end the process without calling `on_exit`.
    pub fn save_prefs(&self) {
        prefs::save(&self.slide);
    }

    /// The window lost focus: DeX's close button (which the app can't intercept), the home
    /// screen, another app. The preferences and an AutoRecover copy of every presentation with
    /// unsaved changes are written now instead of at DeckCraft's next AutoRecover minute, so the
    /// next start reopens them.
    pub fn focus_lost(&mut self) {
        self.save_prefs();
        let session = &self.slide.session;
        if let Some(dir) = &session.recovery_dir
            && session.documents().iter().any(|d| d.is_dirty())
            && let Err(e) = deckcraft_engine::recovery::save(session, dir)
        {
            log::warn!("dexdeckcraft: AutoRecover: {e}");
        }
    }
}

/// The path of a `file://` link to a PDF: what DeckCraft's File › Print opens (it prints
/// through a PDF, which the desktop hands to the system's PDF viewer).
pub fn printed_pdf(url: &str) -> Option<std::path::PathBuf> {
    let path = url.strip_prefix("file://")?;
    let path = format!("/{}", path.trim_start_matches('/'));
    path.to_ascii_lowercase().ends_with(".pdf").then(|| std::path::PathBuf::from(path))
}

impl eframe::App for DeckApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(target_os = "android")]
        {
            let opened = craft_android::android::events::opened();
            if !opened.is_empty() {
                self.open(&opened);
            }
        }
        // (The close — Back, through craft-android's shell — asks about unsaved changes inside.)
        self.slide.logic(ctx);
    }

    fn raw_input_hook(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        // (craft-android's Shell has already made Ctrl+C, X and V copy, cut and paste with the
        // system clipboard.) DeckCraft opens or inserts the files dropped on the window itself.
        self.slide.raw_input_hook(ctx, raw);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.slide.ui(ui);
        // File › Print: the PDF DeckCraft composed goes to the host's print dialog instead of
        // being opened as a link.
        let Some(print) = self.print.as_mut() else { return };
        let printed = ui.ctx().output_mut(|o| {
            let mut printed = Vec::new();
            o.commands.retain(|c| match c {
                egui::OutputCommand::OpenUrl(open) => match printed_pdf(&open.url) {
                    Some(path) => {
                        printed.push(path);
                        false
                    }
                    None => true,
                },
                _ => true,
            });
            printed
        });
        let title = self.slide.session.active().map(|d| d.title()).unwrap_or_else(|| "DeckCraft".to_owned());
        for path in printed {
            print(&path, &title);
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save_prefs();
        self.slide.cleanup_print_previews();
    }
}

/// The window's options, shared by Android and the desktop binary.
pub fn native_options(renderer: eframe::Renderer) -> eframe::NativeOptions {
    eframe::NativeOptions {
        renderer,
        viewport: egui::ViewportBuilder::default()
            .with_title("DexDeckCraft")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([640.0, 420.0])
            .with_app_id("io.github.dexdeckcraft")
            .with_drag_and_drop(true),
        ..Default::default()
    }
}

/// Run the window with wgpu, then with glow if wgpu can't start (old or unusual GPU drivers).
/// `create` builds the app once a renderer has started.
pub fn run(
    mut options: impl FnMut(eframe::Renderer) -> eframe::NativeOptions,
    mut create: impl FnMut(&eframe::CreationContext<'_>) -> Box<dyn eframe::App>,
) {
    let mut last_error = None;
    for renderer in [eframe::Renderer::Wgpu, eframe::Renderer::Glow] {
        let result = eframe::run_native("DexDeckCraft", options(renderer), Box::new(|cc| Ok(create(cc))));
        match result {
            Ok(()) => return,
            Err(e) => {
                log::error!("dexdeckcraft: the {renderer} renderer could not start: {e}");
                #[cfg(target_os = "android")]
                craft_android::android::diag(&format!("the {renderer} renderer could not start: {e}"));
                last_error = Some(e);
            }
        }
    }
    if let Some(e) = last_error {
        log::error!("dexdeckcraft: no renderer could start: {e}");
    }
}

/// The DeckCraft release `upstream/` is at (its workspace version).
pub fn deckcraft_version() -> &'static str {
    const MANIFEST: &str = include_str!("../../../upstream/Cargo.toml");
    MANIFEST.lines().find_map(|l| l.strip_prefix("version = \"").and_then(|v| v.strip_suffix('"'))).unwrap_or("unknown")
}

/// The renderer that started, for the diagnostics report.
pub fn renderer_line(cc: &eframe::CreationContext<'_>) -> String {
    match &cc.wgpu_render_state {
        Some(state) => {
            let adapter = state.adapter.get_info();
            format!("renderer wgpu, {} ({:?}, {:?})", adapter.name, adapter.device_type, adapter.backend)
        }
        None if cc.gl.is_some() => "renderer glow (OpenGL ES)".to_owned(),
        None => "renderer unknown".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upstream_has_a_version() {
        assert!(deckcraft_version().split('.').count() == 3, "{}", deckcraft_version());
    }

    #[test]
    fn the_suggested_name_gives_the_save_type() {
        assert_eq!(save_extension("Talk.deckcraft"), "deckcraft");
        assert_eq!(save_extension("Talk.PPTX"), "pptx");
        assert_eq!(save_extension("Talk.pdf"), "pdf");
        assert_eq!(save_extension("Slide 1.png"), "png");
        assert_eq!(save_extension("Talk"), "deckcraft");
    }

    #[test]
    fn each_purpose_has_its_file_types() {
        assert!(open_filters("open").1.contains(&"pptx"));
        assert!(open_filters("open").1.contains(&"deckcraft"));
        assert!(open_filters("picture").1.contains(&"png"));
        assert!(open_filters("audio").1.contains(&"mp3"));
        assert!(open_filters("video").1.contains(&"mp4"));
        assert!(open_filters("theme").1.contains(&"thmx"));
        assert_eq!(open_filters("anything else").0, "Presentations");
    }

    #[test]
    fn only_a_pdf_file_link_is_a_print_job() {
        assert_eq!(
            printed_pdf("file:///data/user/0/app/cache/p/Talk (print).pdf").as_deref(),
            Some(std::path::Path::new("/data/user/0/app/cache/p/Talk (print).pdf"))
        );
        assert_eq!(printed_pdf("file:////tmp/x/Talk.PDF").as_deref(), Some(std::path::Path::new("/tmp/x/Talk.PDF")));
        assert_eq!(printed_pdf("https://example.com/a.pdf"), None);
        assert_eq!(printed_pdf("file:///tmp/notes.txt"), None);
        assert_eq!(printed_pdf("mailto:a@example.com"), None);
    }

    /// What the window does when it loses focus: the unsaved presentation is in the recovery
    /// folder at once, and a new session reopens it.
    #[test]
    fn losing_focus_writes_a_recovery_copy_at_once() {
        let dir = std::env::temp_dir().join(format!("dexdeckcraft-recovery-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let mut slide = SlideApp::new(Session::with_new(), Services::default());
        slide.session.recovery_dir = Some(dir.clone());
        slide.run("slide.new", serde_json::json!({})).unwrap();
        assert!(slide.session.active().unwrap().is_dirty());
        let mut deck = DeckApp { slide, print: None };
        // (Preferences are off in tests: see `prefs`.)
        deck.focus_lost();
        assert_eq!(deckcraft_engine::recovery::list(&dir).len(), 1);
        let mut next = Session::new();
        next.recovery_dir = Some(dir.clone());
        let opened = next.execute("file.recovery.open", &serde_json::json!({})).unwrap();
        assert_eq!(opened["opened"].as_array().map(Vec::len), Some(1));
        assert_eq!(next.active().unwrap().doc.slides.len(), deck.slide.session.active().unwrap().doc.slides.len());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
