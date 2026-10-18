//! DeckCraft AR — the single switch for a right-to-left interface: when the interface language
//! is Arabic, rows run from the right, as PowerPoint's Arabic interface does — the ribbon's tabs
//! start at the right edge, its groups run from the right (Clipboard at the far right), a small
//! button's icon stands at its right end, the Quick Access Toolbar sits at the right of the title
//! bar, the status bar's zoom at the left, the slide thumbnails at the right and a task pane at
//! the left. Everything that mirrors asks here; nothing else checks the language for layout.
//! (The slides' own text direction is the paragraphs' `rtl`, and is not this.)
//!
//! When the interface is not Arabic every helper is exactly egui's left-to-right default (the
//! `Ui` helpers reproduce `ui.horizontal`, `ui.horizontal_top`, `ui.horizontal_centered` and
//! `ui.vertical` to the pixel), so every other language is laid out as before.

use egui::{Align, Align2, InnerResponse, Layout, Pos2, Rect, Ui, pos2, vec2};

use crate::i18n;

/// Whether the interface is laid out right-to-left this frame.
pub fn on() -> bool {
    i18n::current().is_rtl()
}

/// A row layout: right-to-left when [`on`], else left-to-right.
pub fn row(valign: Align) -> Layout {
    if on() { Layout::right_to_left(valign) } else { Layout::left_to_right(valign) }
}

/// The opposite row (for a cluster that sits at the end of a bar): from the right in a
/// left-to-right interface, from the left when [`on`].
pub fn row_from_end(valign: Align) -> Layout {
    if on() { Layout::left_to_right(valign) } else { Layout::right_to_left(valign) }
}

/// A column whose items are aligned to the start edge (`ui.vertical`'s layout mirrored).
pub fn column() -> Layout {
    Layout::top_down(align_start())
}

/// The start-edge alignment: RIGHT when [`on`], else LEFT.
pub fn align_start() -> Align {
    if on() { Align::RIGHT } else { Align::LEFT }
}

/// The end-edge alignment: LEFT when [`on`], else RIGHT.
pub fn align_end() -> Align {
    if on() { Align::LEFT } else { Align::RIGHT }
}

/// The x of a point `inset` in from the start edge of `r`.
pub fn start_x(r: Rect, inset: f32) -> f32 {
    if on() { r.max.x - inset } else { r.min.x + inset }
}

/// The x of a point `inset` in from the end edge of `r`.
pub fn end_x(r: Rect, inset: f32) -> f32 {
    if on() { r.min.x + inset } else { r.max.x - inset }
}

/// Text anchor at the start edge, vertically centred.
pub fn start_center() -> Align2 {
    if on() { Align2::RIGHT_CENTER } else { Align2::LEFT_CENTER }
}

/// Text anchor at the end edge, vertically centred.
pub fn end_center() -> Align2 {
    if on() { Align2::LEFT_CENTER } else { Align2::RIGHT_CENTER }
}

/// Text anchor at the start edge, at the top.
pub fn start_top() -> Align2 {
    if on() { Align2::RIGHT_TOP } else { Align2::LEFT_TOP }
}

/// `rect` mirrored horizontally within `within` when [`on`]; `rect` itself otherwise.
pub fn mirror_x(rect: Rect, within: Rect) -> Rect {
    if !on() {
        return rect;
    }
    let left = within.min.x + (within.max.x - rect.max.x);
    let right = within.max.x - (rect.min.x - within.min.x);
    Rect::from_min_max(pos2(left, rect.min.y), pos2(right, rect.max.y))
}

/// A rectangle `w` wide and `h` high, `inset` in from the start edge of `within` (at its top).
pub fn start_rect(within: Rect, inset: f32, w: f32, h: f32) -> Rect {
    let r = Rect::from_min_size(pos2(within.min.x + inset, within.min.y), vec2(w, h));
    mirror_x(r, within)
}

/// Where the next item of a row goes: its left edge in a left-to-right row, its right edge when
/// the row runs right-to-left (egui's cursor rect keeps that side as its `max.x`).
pub fn cursor_x(ui: &Ui) -> f32 {
    let c = ui.cursor();
    if ui.layout().main_dir() == egui::Direction::RightToLeft { c.max.x } else { c.min.x }
}

/// The top corner where the next item of a row goes ([`cursor_x`] at the cursor's top).
pub fn cursor_start(ui: &Ui) -> Pos2 {
    pos2(cursor_x(ui), ui.cursor().min.y)
}

/// The direction a row inside `ui` runs: right-to-left when [`on`], or when egui itself would
/// prefer it there (a column aligned to the right), else left-to-right — `ui.horizontal`'s rule
/// plus the Arabic interface.
fn row_in(ui: &Ui, valign: Align) -> Layout {
    if on() || ui.layout().prefer_right_to_left() { Layout::right_to_left(valign) } else { Layout::left_to_right(valign) }
}

/// `ui.horizontal`, mirrored when [`on`].
pub fn horizontal<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let size = vec2(ui.available_size_before_wrap().x, ui.spacing().interact_size.y);
    let layout = row_in(ui, Align::Center);
    ui.allocate_ui_with_layout(size, layout, add)
}

/// `ui.horizontal_top`, mirrored when [`on`].
pub fn horizontal_top<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let size = ui.available_size_before_wrap();
    let layout = row_in(ui, Align::Min);
    ui.allocate_ui_with_layout(size, layout, add)
}

/// `ui.horizontal_centered`, mirrored when [`on`].
pub fn horizontal_centered<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let size = ui.available_size_before_wrap();
    let layout = row_in(ui, Align::Center);
    ui.allocate_ui_with_layout(size, layout, add)
}

/// `ui.vertical`, its items aligned to the start edge when [`on`].
pub fn vertical<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let layout = column();
    ui.with_layout(layout, add)
}

/// A menu item's button: its text at the start edge. Menu popups keep egui's left-to-right
/// column, so a right-to-left item puts its text at the right end of the button through the
/// button's own parts instead of the column's alignment.
pub fn menu_item<'a>(text: &'a str) -> egui::Button<'a> {
    if on() { egui::Button::new("").right_text(text) } else { egui::Button::new(text) }
}

/// A side panel at the start edge of the window (left; right when [`on`]).
pub fn start_panel(id: impl Into<egui::Id>) -> egui::Panel {
    if on() { egui::Panel::right(id) } else { egui::Panel::left(id) }
}

/// A side panel at the end edge of the window (right; left when [`on`]).
pub fn end_panel(id: impl Into<egui::Id>) -> egui::Panel {
    if on() { egui::Panel::left(id) } else { egui::Panel::right(id) }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use serde_json::json;

    use crate::i18n::{Lang, set_current};
    use crate::{Services, SlideApp};

    /// Every text drawn in one frame of the whole app at 2560×900, with the top-left corner of each.
    fn drawn_texts(ctx: &egui::Context, app: &mut SlideApp) -> Vec<(String, egui::Pos2)> {
        fn texts(shape: &egui::Shape, out: &mut Vec<(String, egui::Pos2)>) {
            match shape {
                egui::Shape::Text(t) => out.push((t.galley.text().to_string(), t.pos)),
                egui::Shape::Vec(v) => v.iter().for_each(|s| texts(s, out)),
                _ => {}
            }
        }
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(2560.0, 900.0));
        let mut drawn = Vec::new();
        // Fonts install over the first frames; the ribbon's right-to-left strip settles in one.
        for n in 0..5 {
            let input = egui::RawInput { screen_rect: Some(screen), time: Some(f64::from(n) * 0.1), ..Default::default() };
            let mut out = ctx.run_ui(input, |ui| {
                app.logic(ui.ctx());
                app.ui(ui);
            });
            drawn.clear();
            for s in &out.shapes {
                texts(&s.shape, &mut drawn);
            }
            out.textures_delta.clear();
        }
        drawn
    }

    fn x_of(drawn: &[(String, egui::Pos2)], text: &str) -> f32 {
        let hits: Vec<f32> = drawn.iter().filter(|(t, _)| t == text).map(|(_, p)| p.x).collect();
        assert!(!hits.is_empty(), "{text:?} was not drawn; drawn: {:?}", drawn.iter().map(|(t, _)| t).collect::<Vec<_>>());
        hits.into_iter().fold(f32::NAN, f32::min)
    }

    fn app_in(code: &str) -> (egui::Context, SlideApp) {
        let ctx = egui::Context::default();
        let mut app = SlideApp::new(deckcraft_engine::Session::with_new(), Services::default());
        app.run("app.language", json!({"code": code})).unwrap();
        app.ui.tab = "Home".into();
        app.ui.ribbon_collapsed = false;
        (ctx, app)
    }

    /// Arabic interface: the tab strip starts at the right (Home right of Insert) and the ribbon's
    /// groups run right-to-left (Paste right of the Font group's buttons), as in PowerPoint's
    /// Arabic interface.
    #[test]
    fn arabic_interface_is_laid_out_right_to_left() {
        let (ctx, mut app) = app_in("ar");
        let drawn = drawn_texts(&ctx, &mut app);
        assert!(super::on(), "the interface language is Arabic while the app is drawn");
        let home_label = crate::i18n::tr(Lang::AR, "Home");
        let insert_label = crate::i18n::tr(Lang::AR, "Insert");
        let view_label = crate::i18n::tr(Lang::AR, "View");
        assert_ne!(home_label, "Home", "the catalog translates the Home tab");
        let (home, insert, view) = (x_of(&drawn, home_label), x_of(&drawn, insert_label), x_of(&drawn, view_label));
        assert!(home > insert, "Home tab at x={home} must be right of Insert at x={insert}");
        assert!(home > view, "Home tab at x={home} must be right of View at x={view}");
        assert!(home > 2560.0 / 2.0, "Home tab at x={home} starts at the right edge");
        // The first group (Clipboard) starts at the ribbon's right edge: well into the right half.
        let paste = x_of(&drawn, crate::i18n::tr(Lang::AR, "Paste"));
        assert!(paste > 2560.0 / 2.0, "Paste at x={paste} sits at the far right");
        set_current(Lang::EN);
        assert!(!super::on());
    }

    /// English (and every other language): left-to-right, unchanged.
    #[test]
    fn english_interface_stays_left_to_right() {
        let (ctx, mut app) = app_in("en");
        let drawn = drawn_texts(&ctx, &mut app);
        assert!(!super::on());
        let (home, insert, view) = (x_of(&drawn, "Home"), x_of(&drawn, "Insert"), x_of(&drawn, "View"));
        assert!(home < insert, "Home tab at x={home} must be left of Insert at x={insert}");
        assert!(home < view, "Home tab at x={home} must be left of View at x={view}");
        assert!(home < 2560.0 / 2.0, "Home tab at x={home} starts at the left edge");
        let paste = x_of(&drawn, "Paste");
        assert!(paste < 2560.0 / 2.0, "Paste at x={paste} sits at the far left");
        set_current(Lang::EN);
    }

    /// The helpers are egui's own layouts in a left-to-right interface and their mirror images
    /// in Arabic.
    #[test]
    fn helpers_follow_the_interface_language() {
        set_current(Lang::EN);
        assert_eq!(super::row(egui::Align::Min), egui::Layout::left_to_right(egui::Align::Min));
        assert_eq!(super::row_from_end(egui::Align::Center), egui::Layout::right_to_left(egui::Align::Center));
        assert_eq!(super::column(), egui::Layout::top_down(egui::Align::Min));
        assert_eq!(super::align_start(), egui::Align::LEFT);
        assert_eq!(super::align_end(), egui::Align::RIGHT);
        let within = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(100.0, 10.0));
        let r = egui::Rect::from_min_max(egui::pos2(10.0, 0.0), egui::pos2(30.0, 10.0));
        assert_eq!(super::mirror_x(r, within), r);
        assert_eq!(super::start_rect(within, 10.0, 20.0, 10.0), r);
        assert_eq!(super::start_x(r, 4.0), 14.0);
        assert_eq!(super::end_x(r, 4.0), 26.0);
        assert_eq!(super::start_center(), egui::Align2::LEFT_CENTER);
        assert_eq!(super::end_center(), egui::Align2::RIGHT_CENTER);
        set_current(Lang::AR);
        assert!(super::on());
        assert_eq!(super::row(egui::Align::Min), egui::Layout::right_to_left(egui::Align::Min));
        assert_eq!(super::row_from_end(egui::Align::Center), egui::Layout::left_to_right(egui::Align::Center));
        assert_eq!(super::column(), egui::Layout::top_down(egui::Align::Max));
        assert_eq!(super::align_start(), egui::Align::RIGHT);
        assert_eq!(super::align_end(), egui::Align::LEFT);
        assert_eq!(super::start_center(), egui::Align2::RIGHT_CENTER);
        assert_eq!(super::end_center(), egui::Align2::LEFT_CENTER);
        let mirrored = egui::Rect::from_min_max(egui::pos2(70.0, 0.0), egui::pos2(90.0, 10.0));
        assert_eq!(super::mirror_x(r, within), mirrored);
        assert_eq!(super::start_rect(within, 10.0, 20.0, 10.0), mirrored);
        assert_eq!(super::start_x(r, 4.0), 26.0);
        assert_eq!(super::end_x(r, 4.0), 14.0);
        set_current(Lang::EN);
    }

    /// The row helpers lay a left-to-right interface out exactly as egui's own do.
    #[test]
    fn row_helpers_match_egui_in_english() {
        set_current(Lang::EN);
        let ctx = egui::Context::default();
        let mut ours = Vec::new();
        let mut eguis = Vec::new();
        let mut out = ctx.run_ui(Default::default(), |ui| {
            super::horizontal(ui, |ui| {
                ours.push(ui.label("a").rect);
                ours.push(ui.button("b").rect);
            });
            super::horizontal_top(ui, |ui| {
                ours.push(ui.label("c").rect);
            });
            super::vertical(ui, |ui| {
                ours.push(ui.label("d").rect);
            });
        });
        out.textures_delta.clear();
        let mut out = ctx.run_ui(Default::default(), |ui| {
            ui.horizontal(|ui| {
                eguis.push(ui.label("a").rect);
                eguis.push(ui.button("b").rect);
            });
            ui.horizontal_top(|ui| {
                eguis.push(ui.label("c").rect);
            });
            ui.vertical(|ui| {
                eguis.push(ui.label("d").rect);
            });
        });
        out.textures_delta.clear();
        assert_eq!(ours, eguis);
    }
}
