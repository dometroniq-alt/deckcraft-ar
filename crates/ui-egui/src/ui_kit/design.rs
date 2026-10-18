//! Design tab thumbnails: themes, colours, fonts, effects, background styles, slide size and the
//! Format Background pane. Also the slide kit the other DeckCraft tabs draw with (`ds_*`): a
//! landscape 16:9 slide with a title and body, a before/after pair of slides, a strip of mini
//! slides, theme tiles, gradients and the little numbered badges of animations.

#![allow(unused_imports)]

use egui::{Align2, Color32, Pos2, Rect, Shape, Stroke, pos2, vec2};

use super::Entry;
use super::draw::{Dir, Draw, SIZE};

pub const ENTRIES: &[Entry] = &[
    Entry::new(
        "Browse for Themes…",
        "Opens a file box so you can pick a theme saved on your computer (or take the look of another presentation) and apply its colours, fonts and backgrounds to every slide. Example: choose the ministry's theme file and all the slides take its blue title bars.",
        |d| d.ds_browse_themes(),
    ),
    Entry::new(
        "Colors",
        "Changes the set of colours the whole presentation uses: titles, shapes, charts and links all switch together, while the layout stays the same. Example: pick the green set and every blue heading and shape turns green.",
        |d| d.ds_colors(),
    ),
    Entry::new(
        "Fonts",
        "Changes the pair of fonts the presentation uses, one for headings and one for body text, on every slide at once. Example: choose a bolder heading font and all the slide titles change together.",
        |d| d.ds_fonts(),
    ),
    Entry::new(
        "Effects",
        "Changes how the theme's shapes look: flat, with soft shadows, with glows or with bevelled edges, on every slide at once. Example: the plain boxes of a diagram get a gentle shadow under each one.",
        |d| d.ds_effects(),
    ),
    Entry::new(
        "Background\nStyles",
        "Gives the slides a background from the theme: light, tinted, or dark shades, plain or with a soft gradient. Example: a white slide gets a light-to-dark blue gradient behind its text.",
        |d| d.ds_background_styles(),
    ),
    Entry::new(
        "Slide\nSize",
        "Changes the shape of every slide: the older square-ish 4:3 for old projectors, or the wide 16:9 for modern screens and TVs. Example: switch a 4:3 deck to 16:9 so it fills the meeting-room screen.",
        |d| d.ds_slide_size(),
    ),
    Entry::new(
        "Format\nBackground",
        "Opens a side pane where you set the background of this slide yourself: a solid colour, a gradient, a picture or a pattern, and whether to apply it to all slides. Example: put a faint picture of the building behind the title slide.",
        |d| d.ds_format_background(),
    ),
];

/// Theme sample colours (our own): title, accent one, accent two, accent three.
pub(super) const DS_TEAL: Color32 = Color32::from_rgb(38, 140, 140);
pub(super) const DS_ORANGE: Color32 = Color32::from_rgb(230, 140, 50);
pub(super) const DS_PURPLE: Color32 = Color32::from_rgb(120, 80, 170);
pub(super) const DS_BLUE: Color32 = Color32::from_rgb(60, 110, 200);
pub(super) const DS_GREEN: Color32 = Color32::from_rgb(70, 150, 90);

impl Draw<'_> {
    // ---- the slide kit ----

    /// A landscape slide at `x, y` of `w × h` (white, grey border); returns its content area.
    pub(super) fn ds_slide(&self, x: f32, y: f32, w: f32, h: f32) -> Rect {
        let r = Rect::from_min_size(self.at(x, y), vec2(w, h));
        self.p.rect_filled(r, 2.0, Color32::WHITE);
        self.p.rect_stroke(r, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        r.shrink2(vec2(w * 0.06, h * 0.1))
    }
    /// The outer rect of a slide drawn with [`Draw::ds_slide`] at the same place.
    pub(super) fn ds_slide_rect(&self, x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect::from_min_size(self.at(x, y), vec2(w, h))
    }
    /// One centred 16:9 slide; returns its content area.
    pub(super) fn ds_slide_center(&self) -> Rect {
        self.ds_slide(24.0, 16.0, 128.0, 72.0)
    }
    /// A title line (thick, in `c`) at the top of a slide area, right-aligned (Arabic).
    pub(super) fn ds_title(&self, area: Rect, c: Color32) {
        let y = area.min.y + 3.0;
        let w = area.width() * 0.55;
        self.line(pos2(area.max.x - w, y), pos2(area.max.x, y), c, 3.0);
    }
    /// `n` body lines under the title, right-aligned.
    pub(super) fn ds_body(&self, area: Rect, n: usize, c: Color32) {
        let body = Rect::from_min_max(pos2(area.min.x, area.min.y + 8.0), area.max);
        let full = body.width();
        let widths: Vec<f32> = (0..n).map(|i| full * [0.9, 0.75, 0.85, 0.6, 0.8].get(i % 5).copied().unwrap_or(0.8)).collect();
        let pitch = ((body.height() - 4.0) / n.max(1) as f32).clamp(5.0, 10.0);
        self.text_lines(body, true, &widths, pitch, c);
    }
    /// A title and `n` body lines: the usual slide.
    pub(super) fn ds_text(&self, area: Rect, n: usize) {
        self.ds_title(area, self.ds_ink());
        self.ds_body(area, n, self.ds_grey());
    }
    /// Text on a slide: slides are white in both themes, so their ink is fixed, not the
    /// theme's.
    pub(super) fn ds_ink(&self) -> Color32 {
        Color32::from_gray(45)
    }
    /// Ordinary (body) text on a slide.
    pub(super) fn ds_grey(&self) -> Color32 {
        Color32::from_gray(150)
    }
    /// Two small slides with an arrow between them: `before(area)` then `after(area)`. In
    /// motion the arrow grows first and the "after" slide is wiped in from the left.
    pub(super) fn ds_pair(&self, before: impl FnOnce(&Draw<'_>, Rect), after: impl FnOnce(&Draw<'_>, Rect)) {
        let (w, h, y) = (70.0, 40.0, 32.0);
        let b = self.ds_slide(6.0, y, w, h);
        before(self, b);
        let arrow = self.reveal(0.05, 0.3);
        if arrow > 0.0 {
            self.arrow_right(self.at(80.0, 52.0), self.at(80.0 + 16.0 * arrow, 52.0));
        }
        let outer = self.ds_slide_rect(100.0, y, w, h);
        let wipe = self.reveal(0.3, 0.75);
        if wipe <= 0.0 {
            self.ds_slide(100.0, y, w, h);
            return;
        }
        let clip = Rect::from_min_max(outer.min - vec2(2.0, 2.0), pos2(outer.min.x + outer.width() * wipe + 2.0, outer.max.y + 2.0));
        let clipped = self.p.with_clip_rect(clip.intersect(self.p.clip_rect()));
        let d = self.with_painter(&clipped);
        let a = d.ds_slide(100.0, y, w, h);
        after(&d, a);
        if wipe < 1.0 {
            let rest =
                self.p.with_clip_rect(Rect::from_min_max(pos2(clip.max.x, clip.min.y), outer.max + vec2(2.0, 2.0)).intersect(self.p.clip_rect()));
            self.with_painter(&rest).ds_slide(100.0, y, w, h);
        }
    }
    /// A strip of `n` mini slides centred at `y`, the `active` one framed in the accent colour;
    /// returns their outer rects.
    pub(super) fn ds_strip(&self, y: f32, n: usize, active: Option<usize>) -> Vec<Rect> {
        let n = n.clamp(1, 6);
        let (w, h, gap) = (26.0, 15.0, 6.0);
        let total = n as f32 * (w + gap) - gap;
        let x0 = (SIZE.x - total) / 2.0;
        (0..n)
            .map(|i| {
                let x = x0 + i as f32 * (w + gap);
                let r = self.ds_slide_rect(x, y, w, h);
                self.p.rect_filled(r, 1.0, Color32::WHITE);
                let on = active == Some(i);
                let stroke = if on { Stroke::new(1.5, self.accent()) } else { self.stroke(self.t.border_strong) };
                self.p.rect_stroke(r, 1.0, stroke, egui::StrokeKind::Inside);
                self.line(pos2(r.max.x - 4.0, r.min.y + 4.0), pos2(r.max.x - 14.0, r.min.y + 4.0), self.ds_ink(), 1.5);
                self.line(pos2(r.max.x - 4.0, r.min.y + 8.0), pos2(r.min.x + 4.0, r.min.y + 8.0), self.ds_grey(), 1.0);
                self.line(pos2(r.max.x - 4.0, r.min.y + 11.0), pos2(r.min.x + 8.0, r.min.y + 11.0), self.ds_grey(), 1.0);
                r
            })
            .collect()
    }
    /// A theme tile at `x, y`: a background, two letters in the title colour and a strip of
    /// the theme's colours along the bottom.
    pub(super) fn ds_theme_tile(&self, x: f32, y: f32, w: f32, h: f32, bg: Color32, fg: Color32, cols: &[Color32]) -> Rect {
        let r = Rect::from_min_size(self.at(x, y), vec2(w, h));
        self.p.rect_filled(r, 2.0, bg);
        self.p.rect_stroke(r, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.ds_label(pos2(r.max.x - 4.0, r.min.y + h * 0.4), "أب", h * 0.42, fg, Align2::RIGHT_CENTER);
        let n = cols.len().max(1) as f32;
        let sw = (w - 8.0) / n;
        for (k, c) in cols.iter().enumerate() {
            self.p.rect_filled(Rect::from_min_size(pos2(r.min.x + 4.0 + k as f32 * sw, r.max.y - 7.0), vec2(sw - 1.5, 4.0)), 0.0, *c);
        }
        r
    }
    /// A vertical gradient from `top` to `bottom` filling `r`, drawn as bands.
    pub(super) fn ds_gradient(&self, r: Rect, top: Color32, bottom: Color32) {
        let bands = 12;
        let bh = r.height() / bands as f32;
        for i in 0..bands {
            let t = i as f32 / (bands - 1) as f32;
            let c = top.lerp_to_gamma(bottom, t);
            let y0 = r.min.y + i as f32 * bh;
            self.p.rect_filled(Rect::from_min_max(pos2(r.min.x, y0), pos2(r.max.x, (y0 + bh + 0.5).min(r.max.y))), 0.0, c);
        }
    }
    /// A small numbered badge (an animation's order) centred at `c`.
    pub(super) fn ds_num(&self, c: Pos2, s: &str) {
        self.p.circle_filled(c, 5.5, self.accent());
        self.ds_label(c, s, 7.0, Color32::WHITE, Align2::CENTER_CENTER);
    }
    /// A sample string as written: [`Draw::label`] puts every string in the order the renderer
    /// draws it right (`i18n::artwork`), so nothing is turned here any more.
    pub(super) fn ds_ar(&self, s: &str) -> String {
        s.to_string()
    }
    /// Text set in the thumbnail's font, Arabic ordered for the renderer (see [`Draw::ds_ar`]).
    pub(super) fn ds_label(&self, pos: Pos2, s: &str, size: f32, c: Color32, anchor: Align2) {
        self.label(pos, &self.ds_ar(s), size, c, anchor);
    }
    /// A labelled chip centred at `c`, `w` wide, in the theme's menu colours (readable in the
    /// light and the dark theme alike).
    pub(super) fn ds_chip(&self, c: Pos2, text: &str, w: f32) {
        let r = Rect::from_center_size(c, vec2(w, 14.0));
        self.p.rect_filled(r, 3.0, self.t.menu);
        self.p.rect_stroke(r, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.ds_label(c, text, 7.5, self.t.text, Align2::CENTER_CENTER);
    }
    /// A folder (a tab on a box) centred at `c`.
    pub(super) fn ds_folder(&self, c: Pos2) {
        let body = Rect::from_center_size(c + vec2(0.0, 2.0), vec2(26.0, 18.0));
        let tab = Rect::from_min_size(pos2(body.min.x, body.min.y - 4.0), vec2(11.0, 5.0));
        self.p.rect_filled(tab, 1.0, DS_ORANGE);
        self.p.rect_filled(body, 2.0, DS_ORANGE.lerp_to_gamma(Color32::WHITE, 0.25));
        self.p.rect_stroke(body, 2.0, Stroke::new(1.0, DS_ORANGE), egui::StrokeKind::Inside);
    }
    /// A slide made over in a theme: a coloured title band and accent shapes.
    pub(super) fn ds_themed(&self, outer: Rect, area: Rect, title: Color32, accent: Color32) {
        let band = Rect::from_min_max(outer.min, pos2(outer.max.x, outer.min.y + outer.height() * 0.3));
        self.p.rect_filled(band, 2.0, title);
        self.line(pos2(band.max.x - 6.0, band.center().y), pos2(band.max.x - band.width() * 0.5, band.center().y), Color32::WHITE, 2.5);
        let y = band.max.y + 6.0;
        for k in 0..3 {
            let x = area.max.x - 4.0 - k as f32 * (area.width() * 0.3);
            self.p.rect_filled(
                Rect::from_min_size(pos2(x - area.width() * 0.26, y), vec2(area.width() * 0.26, area.max.y - y - 2.0)),
                1.0,
                accent.linear_multiply(if k == 1 { 1.0 } else { 0.55 }),
            );
        }
    }

    // ---- entries ----

    fn ds_browse_themes(&self) {
        self.ds_pair(
            |d, a| {
                d.ds_text(a, 3);
            },
            |d, a| {
                let outer = d.ds_slide_rect(100.0, 32.0, 70.0, 40.0);
                d.ds_themed(outer, a, DS_BLUE, DS_ORANGE);
            },
        );
        // The theme comes out of a folder above the arrow.
        let lift = self.reveal(0.0, 0.3);
        self.ds_folder(self.at(88.0, 18.0));
        let tile = Rect::from_center_size(self.at(88.0, 10.0 - 6.0 * lift), vec2(14.0, 9.0));
        self.p.rect_filled(tile, 1.0, DS_BLUE);
        self.p.rect_filled(Rect::from_min_size(pos2(tile.min.x + 2.0, tile.max.y - 3.0), vec2(4.0, 2.0)), 0.0, DS_ORANGE);
        self.p.rect_filled(Rect::from_min_size(pos2(tile.min.x + 7.0, tile.max.y - 3.0), vec2(4.0, 2.0)), 0.0, Color32::WHITE);
    }

    fn ds_colors(&self) {
        let greys = [Color32::from_gray(120), Color32::from_gray(160), Color32::from_gray(200)];
        let cols = [DS_GREEN, DS_GREEN.linear_multiply(0.7), DS_GREEN.lerp_to_gamma(Color32::WHITE, 0.4)];
        self.ds_pair(
            |d, a| {
                d.ds_title(a, Color32::from_gray(90));
                d.ds_color_bars(a, &greys);
            },
            |d, a| {
                d.ds_title(a, DS_GREEN);
                d.ds_color_bars(a, &cols);
            },
        );
        // The palette picked, above the arrow.
        let sw = [DS_GREEN, DS_GREEN.linear_multiply(0.7), DS_ORANGE, DS_BLUE];
        for (k, c) in sw.iter().enumerate() {
            let r = Rect::from_min_size(self.at(70.0 + k as f32 * 9.0, 12.0), vec2(8.0, 10.0));
            self.p.rect_filled(r, 1.0, *c);
        }
        self.p.rect_stroke(
            Rect::from_min_size(self.at(69.0, 11.0), vec2(37.0, 12.0)),
            1.0,
            self.stroke(self.t.border_strong),
            egui::StrokeKind::Inside,
        );
    }
    /// Three bars (a chart or a diagram) in `cols` under a slide's title.
    fn ds_color_bars(&self, a: Rect, cols: &[Color32]) {
        let bw = a.width() * 0.22;
        for (k, c) in cols.iter().enumerate() {
            let h = [16.0, 22.0, 12.0].get(k % 3).copied().unwrap_or(16.0);
            let x = a.max.x - (k as f32 + 1.0) * (bw + 3.0);
            self.p.rect_filled(Rect::from_min_max(pos2(x, a.max.y - h), pos2(x + bw, a.max.y)), 1.0, *c);
        }
    }

    fn ds_fonts(&self) {
        self.ds_pair(
            |d, a| {
                d.ds_label(pos2(a.max.x, a.min.y + 8.0), "عنوان", 9.0, d.ds_ink(), Align2::RIGHT_CENTER);
                d.ds_body(Rect::from_min_max(pos2(a.min.x, a.min.y + 8.0), a.max), 3, d.ds_grey());
            },
            |d, a| {
                d.ds_label(pos2(a.max.x, a.min.y + 8.0), "عنوان", 11.5, DS_PURPLE, Align2::RIGHT_CENTER);
                let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 10.0), a.max);
                let widths = [body.width() * 0.8, body.width() * 0.65, body.width() * 0.75];
                d.text_lines(body, true, &widths, 7.0, d.ds_grey());
            },
        );
        // The pair of fonts, as chips above the arrow.
        self.ds_chip(self.at(70.0, 16.0), "أ", 16.0);
        self.ds_label(self.at(88.0, 16.0), "+", 9.0, self.grey(), Align2::CENTER_CENTER);
        self.ds_chip(self.at(106.0, 16.0), "ب", 16.0);
    }

    fn ds_effects(&self) {
        self.ds_pair(
            |d, a| {
                d.ds_title(a, d.ds_ink());
                let s = Rect::from_min_size(pos2(a.max.x - 28.0, a.min.y + 11.0), vec2(26.0, 16.0));
                d.p.rect_filled(s, 2.0, DS_BLUE);
                let s2 = Rect::from_min_size(pos2(a.min.x + 2.0, a.min.y + 11.0), vec2(26.0, 16.0));
                d.p.rect_filled(s2, 2.0, DS_ORANGE);
            },
            |d, a| {
                d.ds_title(a, d.ds_ink());
                for (x, c) in [(a.max.x - 28.0, DS_BLUE), (a.min.x + 2.0, DS_ORANGE)] {
                    let s = Rect::from_min_size(pos2(x, a.min.y + 11.0), vec2(26.0, 16.0));
                    // Shadow, then a glow ring, then the shape with a lighter top (a bevel).
                    d.p.rect_filled(s.translate(vec2(2.0, 2.5)), 2.0, Color32::from_black_alpha(70));
                    d.p.rect_stroke(s.expand(1.5), 3.0, Stroke::new(1.5, c.linear_multiply(0.35)), egui::StrokeKind::Outside);
                    d.p.rect_filled(s, 2.0, c);
                    d.p.rect_filled(Rect::from_min_size(s.min + vec2(2.0, 2.0), vec2(s.width() - 4.0, 4.0)), 1.0, Color32::from_white_alpha(90));
                }
            },
        );
    }

    fn ds_background_styles(&self) {
        self.ds_pair(
            |d, a| {
                d.ds_text(a, 3);
            },
            |d, a| {
                let outer = d.ds_slide_rect(100.0, 32.0, 70.0, 40.0).shrink(1.0);
                d.ds_gradient(outer, DS_BLUE.lerp_to_gamma(Color32::WHITE, 0.75), DS_BLUE);
                d.ds_title(a, Color32::WHITE);
                d.ds_body(a, 3, Color32::from_white_alpha(200));
            },
        );
        // The style swatches above: light, tinted, dark.
        let shades = [
            Color32::WHITE,
            DS_BLUE.lerp_to_gamma(Color32::WHITE, 0.7),
            DS_BLUE.lerp_to_gamma(Color32::WHITE, 0.3),
            DS_BLUE.lerp_to_gamma(Color32::BLACK, 0.3),
        ];
        for (k, c) in shades.iter().enumerate() {
            let r = Rect::from_min_size(self.at(64.0 + k as f32 * 13.0, 10.0), vec2(11.0, 8.0));
            self.p.rect_filled(r, 1.0, *c);
            let on = k == 2;
            self.p.rect_stroke(
                r,
                1.0,
                if on { Stroke::new(1.5, self.accent()) } else { self.stroke(self.t.border_strong) },
                egui::StrokeKind::Inside,
            );
        }
    }

    fn ds_slide_size(&self) {
        // 4:3 on the left, 16:9 on the right, each with its ratio badge.
        let t = self.reveal(0.2, 0.7);
        let b = self.ds_slide(10.0, 30.0, 60.0, 45.0);
        self.ds_text(b, 3);
        self.badge(self.at(40.0, 18.0), "٤:٣", self.grey());
        self.arrow_right(self.at(76.0, 52.0), self.at(90.0, 52.0));
        let w = 60.0 + 16.0 * t;
        let h = w * 9.0 / 16.0;
        let a = self.ds_slide(96.0, 52.0 - h / 2.0, w, h);
        self.ds_text(a, 3);
        self.badge(self.at(96.0 + w / 2.0, 18.0), "١٦:٩", self.accent());
        self.measure_h(self.at(96.0, 52.0 + h / 2.0 + 8.0), self.at(96.0 + w, 52.0 + h / 2.0 + 8.0));
    }

    fn ds_format_background(&self) {
        // The slide takes the colour the pane picks, as the film plays.
        let fill = self.reveal(0.3, 0.8);
        let outer = self.ds_slide_rect(6.0, 20.0, 104.0, 60.0);
        let a = self.ds_slide(6.0, 20.0, 104.0, 60.0);
        let tint = Color32::WHITE.lerp_to_gamma(DS_TEAL.lerp_to_gamma(Color32::WHITE, 0.55), fill);
        self.p.rect_filled(outer.shrink(1.0), 1.0, tint);
        self.ds_text(a, 3);
        let pane = self.pane(true, 56.0, 0);
        // Pane rows: fill choices with radio dots, a swatch and an "apply to all" button.
        let x = pane.min.x + 6.0;
        for (k, on) in [(0, true), (1, false), (2, false)] {
            let y = pane.min.y + 22.0 + k as f32 * 11.0;
            self.p.circle_stroke(pos2(x + 3.0, y), 3.0, self.stroke(self.t.border_strong));
            if on {
                self.p.circle_filled(pos2(x + 3.0, y), 1.8, self.accent());
            }
            self.line(pos2(x + 10.0, y), pos2(pane.max.x - 6.0, y), self.grey(), 1.5);
        }
        let sw = Rect::from_min_size(pos2(x, pane.min.y + 58.0), vec2(pane.width() - 12.0, 10.0));
        self.p.rect_filled(sw, 1.0, DS_TEAL.lerp_to_gamma(Color32::WHITE, 0.55));
        self.p.rect_stroke(sw, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_stroke(
            Rect::from_min_size(pos2(x, pane.max.y - 18.0), vec2(pane.width() - 12.0, 12.0)),
            2.0,
            self.stroke(self.t.border_strong),
            egui::StrokeKind::Inside,
        );
        self.line(pos2(x + 6.0, pane.max.y - 12.0), pos2(pane.max.x - 12.0, pane.max.y - 12.0), self.ink(), 1.5);
    }
}
