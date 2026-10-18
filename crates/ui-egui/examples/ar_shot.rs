//! DeckCraft AR — offscreen screenshots for review: the app in a language on a ribbon tab, or a
//! contact sheet of the UI kit's thumbnails (wgpu, no window).
//!
//! `cargo run -p deckcraft-ui-egui --example ar_shot -- app out.png [ar|en] [Home]`
//! `cargo run -p deckcraft-ui-egui --example ar_shot -- thumbs out.png [filter]`
//! `KIT_PHASE=0.5` renders every film at that point instead of its still frame.

use deckcraft_ui_egui::ui_kit;
use deckcraft_ui_egui::{Services, SlideApp};
use egui::{Rect, Sense, Stroke, vec2};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let what = args.get(1).cloned().unwrap_or_else(|| "app".into());
    let out = args.get(2).cloned().unwrap_or_else(|| "shot.png".into());
    let third = args.get(3).cloned().unwrap_or_default();
    let fourth = args.get(4).cloned().unwrap_or_else(|| "Home".into());
    if what == "thumbs" {
        thumbs(&out, &third);
    } else {
        app(&out, if third.is_empty() { "ar" } else { &third }, &fourth);
    }
}

fn app(out: &str, lang: &str, tab: &str) {
    let mut frame = 0u32;
    let size = vec2(std::env::var("SHOT_W").ok().and_then(|v| v.parse().ok()).unwrap_or(1600.0), 700.0);
    let (lang, tab) = (lang.to_string(), tab.to_string());
    let mut harness = egui_kittest::Harness::builder().with_size(size).with_pixels_per_point(1.5).wgpu().build_ui_state(
        move |ui, app: &mut SlideApp| {
            frame += 1;
            if frame == 1 {
                deckcraft_ui_egui::theme::install_fonts(ui.ctx());
                let _ = app.run("app.language", serde_json::json!({"code": lang}));
                app.ui.tab = tab.clone();
                app.ui.ribbon_collapsed = false;
                if let Ok(search) = std::env::var("KIT_SEARCH") {
                    let _ = app.run("app.uiKit", serde_json::json!({"search": search}));
                }
                return;
            }
            app.logic(ui.ctx());
            app.ui(ui);
        },
        sample_app(),
    );
    for _ in 0..4 {
        harness.step();
    }
    save(&mut harness, out);
}

fn sample_app() -> SlideApp {
    let mut session = deckcraft_engine::Session::with_new();
    if let Err(e) = deckcraft_engine::sample::open_sample(&mut session) {
        eprintln!("sample: {e}");
    }
    SlideApp::new(session, Services::default())
}

fn thumbs(out: &str, filter: &str) {
    let cols = 12usize;
    let cell = vec2(ui_kit::SIZE.x + 12.0, ui_kit::SIZE.y + 30.0);
    let n = ui_kit::entries().filter(|e| filter.is_empty() || e.key.contains(filter)).count();
    let (w, h) = (cols as f32 * cell.x + 16.0, (n as f32 / cols as f32).ceil() * cell.y + 16.0);
    let filter = filter.to_string();
    let mut frame = 0u32;
    let mut harness = egui_kittest::Harness::builder().with_size(vec2(w, h)).with_pixels_per_point(2.0).wgpu().build_ui(move |ui| {
        frame += 1;
        if frame == 1 {
            deckcraft_ui_egui::theme::install_fonts(ui.ctx());
            ui.ctx().set_theme(egui::Theme::Light);
            return;
        }
        let t = ui_kit::Palette::get(ui.ctx());
        ui.painter().rect_filled(ui.max_rect(), 0.0, t.menu);
        let phase: f32 = std::env::var("KIT_PHASE").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0);
        for (i, e) in ui_kit::entries().filter(|e| filter.is_empty() || e.key.contains(filter.as_str())).enumerate() {
            let (col, row) = (i % cols, i / cols);
            let origin = ui.max_rect().min + vec2(8.0 + col as f32 * cell.x, 8.0 + row as f32 * cell.y);
            let r = Rect::from_min_size(origin, ui_kit::SIZE);
            ui.painter().rect_filled(r, 4.0, t.panel);
            ui.painter().rect_stroke(r, 4.0, Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
            (e.draw)(&ui_kit::Draw::animated(ui.painter(), r, &t, phase));
            let key = e.key.replace('\n', " ");
            ui.painter().text(origin + vec2(2.0, ui_kit::SIZE.y + 14.0), egui::Align2::LEFT_CENTER, key, egui::FontId::proportional(9.5), t.text);
            let _ = ui.allocate_rect(r, Sense::hover());
        }
    });
    harness.input_mut().max_texture_side = Some(8192);
    for _ in 0..3 {
        harness.step();
    }
    save(&mut harness, out);
}

fn save<S>(harness: &mut egui_kittest::Harness<'_, S>, out: &str) {
    match harness.render() {
        Ok(img) => match img.save(out) {
            Ok(()) => println!("{out}"),
            Err(e) => eprintln!("saving {out}: {e}"),
        },
        Err(e) => eprintln!("render failed: {e}"),
    }
}
