//! The Android side: the entry point and DeckCraft's [`Services`] on craft-android (the Storage
//! Access Framework's pickers, the clipboard, printing, sound). Everything else (the JNI bridge,
//! the environment, the activity's input, the lifecycle) is craft-android's.

mod audio;

use std::future::Future;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc::{Sender, channel};

use craft_android::android::{self as platform, AndroidApp, clipboard::Clip, pickers::PickKind};
use deckcraft_ui_egui::Services;

use crate::{DeckApp, Launch};

/// Entry point, called by android-activity on the activity's own thread.
#[allow(unsafe_code)] // `no_mangle` is how android-activity's glue finds the entry point
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    let config = platform::Config { log_tag: "dexdeckcraft", asset_dirs: Vec::new() };
    let Some(started) = platform::start(&app, &config) else { return };
    platform::diag(&format!("DexDeckCraft {} (DeckCraft {})", env!("CARGO_PKG_VERSION"), crate::deckcraft_version()));
    let print_dir = Arc::new(started.data_dir.join("cache/print"));
    let launch = Launch::default();
    crate::run(
        |renderer| platform::native_options(&app, crate::native_options(renderer)),
        |cc| {
            platform::events::set_context(cc.egui_ctx.clone());
            platform::diag(&crate::renderer_line(cc));
            // One window: Presenter View and View › New Window are egui windows inside it.
            cc.egui_ctx.set_embed_viewports(true);
            let print_dir = print_dir.clone();
            let deck = DeckApp::new(cc, &launch, services()).with_print(move |pdf, title| print(pdf, title, &print_dir));
            // craft-android's shell: the activity's mouse, keyboard and insets, Back as the close
            // button (DeckCraft asks about unsaved presentations), the title for DeX's taskbar.
            Box::new(craft_android::Shell::new(deck).on_focus_lost(|d| d.focus_lost()).title(DeckApp::title))
        },
    );
    // The window closed (DeckCraft allowed it, after asking about unsaved changes).
    platform::finish();
}

/// File › Print: DeckCraft composed the PDF (layout, slides, options); Android's print dialog
/// takes it from there (printer, copies, two-sided, Save as PDF).
fn print(pdf: &std::path::Path, title: &str, dir: &std::path::Path) {
    let sent = std::fs::read(pdf).map_err(|e| e.to_string()).and_then(|bytes| platform::print::print_pdf(&bytes, title, dir));
    if let Err(e) = sent {
        log::error!("dexdeckcraft: print: {e}");
        platform::toast(&format!("Couldn't print: {e}"));
    }
}

/// Wait for a picker's answer on a worker thread, then hand it to `then`. The picker itself must
/// be started on the interface thread, which can't wait: the activity's pause waits for it.
fn when_picked(picked: platform::pickers::Picked, then: impl FnOnce(Vec<PathBuf>) + Send + 'static) {
    let spawned = std::thread::Builder::new().name("picker".into()).spawn(move || then(block_on(picked)));
    if let Err(e) = spawned {
        log::error!("dexdeckcraft: picker thread: {e}");
    }
}

/// craft-android's picker futures block inside `poll` until the answer comes, so one poll is
/// the whole wait.
fn block_on<T>(future: impl Future<Output = T>) -> T {
    let mut future = std::pin::pin!(future);
    let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
    loop {
        if let std::task::Poll::Ready(value) = future.as_mut().poll(&mut cx) {
            return value;
        }
        std::thread::yield_now();
    }
}

/// A picker's answer for DeckCraft's `Services::picked_later`: the chosen path, or an empty one
/// for a dismissed picker, so DeckCraft forgets what asked for it.
fn answer(picked: platform::pickers::Picked, later: Sender<String>) {
    when_picked(picked, move |paths| {
        let path = paths.first().and_then(|p| p.to_str()).unwrap_or_default().to_owned();
        let _ = later.send(path);
        platform::events::wake();
    });
}

fn services() -> Services {
    // The pickers are other activities: they answer after the frame that asked (the fork's
    // `picked_later`), and DeckCraft then runs what asked for them again with the path.
    let (later, picked_later) = channel();
    let save_later = later.clone();
    Services {
        // File › Open, Insert › Pictures, Audio and Video, Browse for Themes…: the system picker.
        pick_open: Some(Box::new(move |purpose: &str| {
            let (name, extensions) = crate::open_filters(purpose);
            let filters = [(name.to_owned(), extensions.iter().map(|e| (*e).to_owned()).collect())];
            answer(platform::pickers::pick(PickKind::OpenFile, None, &filters, None), later.clone());
            None
        })),
        // Save As and the exports: the system's Save picker, offering the suggested name's type.
        pick_save: Some(Box::new(move |suggested: &str| {
            let extension = crate::save_extension(suggested);
            let name = std::path::Path::new(suggested).file_name().and_then(std::ffi::OsStr::to_str).unwrap_or("Presentation.deckcraft");
            let filters = [(extension.to_uppercase(), vec![extension])];
            answer(platform::pickers::pick(PickKind::Save, None, &filters, Some(name)), save_later.clone());
            None
        })),
        picked_later: Some(picked_later),
        read: Some(Box::new(|p: &str| std::fs::read(p).map_err(|e| e.to_string()))),
        write: Some(Box::new(|p: &str, b: &[u8]| std::fs::write(p, b).map_err(|e| e.to_string()))),
        // A copied picture, as the PNG DeckCraft pastes.
        clipboard_image: Some(Box::new(|| match platform::clipboard::get()? {
            Clip::Image { width, height, rgba } => {
                let picture = image::RgbaImage::from_raw(u32::try_from(width).ok()?, u32::try_from(height).ok()?, rgba)?;
                let mut png = Vec::new();
                picture.write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png).ok()?;
                Some(png)
            }
            _ => None,
        })),
        clipboard_html: Some(Box::new(|| match platform::clipboard::get()? {
            Clip::Html { html, .. } => Some(html),
            _ => None,
        })),
        clipboard_set_html: Some(Box::new(|html: &str, text: &str| {
            match platform::clipboard::set(Clip::Html { html: html.to_owned(), text: text.to_owned() }) {
                Ok(()) => true,
                Err(e) => {
                    log::warn!("dexdeckcraft: clipboard: {e}");
                    false
                }
            }
        })),
        // Media is mixed and clocked by the output device; `DECKCRAFT_NO_AUDIO` plays silently.
        audio_out: std::env::var_os("DECKCRAFT_NO_AUDIO")
            .is_none()
            .then(|| Box::new(audio::CpalOut::default()) as Box<dyn deckcraft_media::AudioOut>),
        // The activity has one window on one display: the show plays in it, with Presenter View
        // (when asked for) inside the same window.
        displays: Some(Box::new(|| Some(1))),
        ..Default::default()
    }
}
