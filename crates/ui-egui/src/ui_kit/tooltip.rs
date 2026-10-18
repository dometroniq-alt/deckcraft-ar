//! The visual-help tooltip and the gallery menu row: what the ribbon widgets call instead of
//! `on_hover_text` so a button with an entry shows its film and its explanation (in the
//! interface language, and in Arabic as well when the interface is another language).

use egui::{Align2, CornerRadius, Rect, Response, Sense, Stroke, Ui, pos2, vec2};

use super::{Palette, SIZE, entry, paint_scaled};
use crate::i18n::{self, Lang};

/// One loop of a thumbnail's film.
pub const FILM_SECONDS: f64 = 2.4;

/// Where the app leaves its "explain buttons in Arabic too" preference each frame
/// (`egui::Context` temp data, a `bool`; on when it was never set).
pub fn bilingual_id() -> egui::Id {
    egui::Id::new("deckcraft_ar_bilingual")
}

/// Whether tooltips add their Arabic line under the interface language's (the preference the
/// app stored this frame under [`bilingual_id`]).
pub fn bilingual(ctx: &egui::Context) -> bool {
    ctx.data(|d| d.get_temp::<bool>(bilingual_id())).unwrap_or(true)
}

/// The phase (0 → 1, looping every [`FILM_SECONDS`]) of a film that started at `since`, or 1
/// (still) when animations are off. Asks for the next frame while the film plays.
pub fn film_phase(ui: &Ui, since: f64) -> f32 {
    if ui.ctx().global_style().animation_time <= 0.0 {
        return 1.0;
    }
    let now = ui.input(|i| i.time);
    ui.ctx().request_repaint_after(std::time::Duration::from_millis(33));
    (((now - since) / FILM_SECONDS) % 1.0) as f32
}

/// A hover tooltip for a control keyed by `key` (a button's English tip, a menu item's key):
/// the translated tip, then — when the kit has an entry — the film, the explanation and its
/// Arabic. Without an entry it is the plain tooltip every widget had before.
pub fn tooltip(_ui: &Ui, resp: Response, key: &str, label: &str) -> Response {
    let lang = i18n::current();
    let shown = i18n::tr(lang, label).to_string();
    if entry(key).is_none() {
        return resp.on_hover_text(shown);
    }
    body(resp, key, Some(&shown))
}

/// The visual help alone (film, explanation, Arabic) for a ribbon button whose caller adds
/// its own name-and-shortcut tooltip after it; without an entry the response comes back
/// untouched, so nothing is shown twice.
pub fn help(resp: Response, key: &str) -> Response {
    if entry(key).is_none() {
        return resp;
    }
    body(resp, key, None)
}

fn body(resp: Response, key: &str, title: Option<&str>) -> Response {
    let lang = i18n::current();
    let Some(e) = entry(key) else { return resp };
    let key = key.to_string();
    let title = title.map(str::to_string);
    resp.on_hover_ui(|ui| {
        ui.set_max_width(SIZE.x + 16.0);
        if let Some(title) = &title {
            ui.label(egui::RichText::new(title).font(crate::theme::bold(12.0)));
        }
        let t = Palette::get(ui.ctx());
        let (r, _) = ui.allocate_exact_size(SIZE, Sense::hover());
        ui.painter().rect_filled(r, 4.0, t.panel);
        ui.painter().rect_stroke(r, 4.0, Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
        // (since, last frame shown): a tooltip that was away for a moment starts its film over.
        let since_id = egui::Id::new(("ui_kit_since", &key));
        let now = ui.input(|i| i.time);
        let since = ui.data_mut(|d| {
            let (since, last) = d.get_temp_mut_or_insert_with(since_id, || (now, now));
            if now - *last > 0.25 {
                *since = now;
            }
            *last = now;
            *since
        });
        let phase = film_phase(ui, since);
        (e.draw)(&super::Draw::animated(ui.painter(), r, &t, phase));
        ui.add_space(4.0);
        let font = crate::theme::font(11.5);
        crate::rtl_text::paragraph(ui, super::explain(lang, e.explain), font.clone(), t.text, SIZE.x);
        if lang != Lang::AR && bilingual(ui.ctx()) {
            let arabic = super::explain(Lang::AR, e.explain);
            if arabic != e.explain {
                ui.add_space(2.0);
                crate::rtl_text::paragraph(ui, arabic, font, t.text_dim, SIZE.x);
            }
        }
    })
}

/// A menu row with a preview of what it does: the thumbnail scaled small at the start edge,
/// the text after it. Returns the row's response (clicked = run the item).
pub fn gallery_item(ui: &mut Ui, key: &str, text: &str, enabled: bool) -> Response {
    const PREVIEW: egui::Vec2 = egui::Vec2::new(96.0, 56.0);
    let t = Palette::get(ui.ctx());
    let font = egui::FontId::proportional(13.0);
    let text_w = ui.ctx().fonts_mut(|f| f.layout_no_wrap(text.to_string(), font.clone(), t.text).size().x);
    let w = (PREVIEW.x + 12.0 + text_w + 16.0).max(240.0);
    let (r, resp) = ui.allocate_exact_size(vec2(w, PREVIEW.y + 8.0), if enabled { Sense::click() } else { Sense::hover() });
    if !ui.is_rect_visible(r) {
        return resp;
    }
    if enabled && (resp.hovered() || resp.is_pointer_button_down_on()) {
        ui.painter().rect_filled(r, CornerRadius::same(4), if resp.is_pointer_button_down_on() { t.checked } else { t.hover });
    }
    let rtl = crate::rtl::on();
    let preview = if rtl {
        Rect::from_min_size(pos2(r.max.x - 4.0 - PREVIEW.x, r.min.y + 4.0), PREVIEW)
    } else {
        Rect::from_min_size(pos2(r.min.x + 4.0, r.min.y + 4.0), PREVIEW)
    };
    paint_scaled(ui.painter(), preview, key, &t);
    ui.painter().rect_stroke(preview, CornerRadius::same(3), Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
    let color = if enabled { t.text } else { t.text_dim };
    if rtl {
        ui.painter().text(pos2(preview.min.x - 8.0, r.center().y), Align2::RIGHT_CENTER, text, font, color);
    } else {
        ui.painter().text(pos2(preview.max.x + 8.0, r.center().y), Align2::LEFT_CENTER, text, font, color);
    }
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, text));
    help(resp, key)
}
