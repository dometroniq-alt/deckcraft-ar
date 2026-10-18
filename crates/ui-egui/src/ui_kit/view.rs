//! View tab and Slide Master tab thumbnails: the three masters, grid options, notes, zoom,
//! fit to window, a new window, grayscale and dark mode; on the master tab, inserting and
//! renaming layouts, placeholders, themes and closing the master view. A master is drawn as a
//! big slide with dashed placeholders and the small layouts hanging under it.

#![allow(unused_imports)]

use egui::{Align2, Color32, Pos2, Rect, Shape, Stroke, pos2, vec2};

use super::Entry;
use super::design::{DS_BLUE, DS_ORANGE, DS_PURPLE, DS_TEAL};
use super::draw::{Dir, Draw, SIZE};

pub const ENTRIES: &[Entry] = &[
    // ---- View ----
    Entry::new(
        "Slide\nMaster",
        "Opens the master: the one slide behind all the others, whose placeholders, fonts and background every layout copies; change it here and every slide follows. Example: put the logo on the master and it appears on all 30 slides.",
        |d| d.vw_slide_master(),
    ),
    Entry::new(
        "Handout\nMaster",
        "Opens the page design used when you print several slides per sheet for the audience: how many slides per page, the header, footer, date and page number. Example: set three slides per page with lines beside them for notes.",
        |d| d.vw_handout_master(),
    ),
    Entry::new(
        "Notes\nMaster",
        "Opens the page design used when you print your speaker notes: the slide picture on top, the notes text under it, and the header and footer. Example: make the notes text larger so it is easy to read at the lectern.",
        |d| d.vw_notes_master(),
    ),
    Entry::new(
        "Grid\nOptions",
        "Opens a box to set the spacing of the grid and whether objects snap to it and to each other, and whether the gridlines and guides are drawn. Example: set a 1 cm grid so boxes line up when you drag them.",
        |d| d.vw_grid_options(),
    ),
    Entry::new(
        "Notes",
        "Shows or hides the notes box under the slide, where you type what you will say; the audience never sees it. Example: open it and write the figures you will quote for slide 5.",
        |d| d.vw_notes(),
    ),
    Entry::new(
        "Fit to\nWindow",
        "Sets the zoom so the whole slide fits in the window, after you zoomed in to work on a detail. Example: after working at 200%, press it to see the full slide again.",
        |d| d.vw_fit(),
    ),
    Entry::new(
        "New\nWindow",
        "Opens a second window on the same presentation, so you can look at two slides at once or keep one on another screen; edits show in both. Example: keep slide 2 open on the second monitor while you edit slide 9.",
        |d| d.vw_new_window(),
    ),
    Entry::new(
        "Grayscale",
        "Shows the slides in shades of grey, as a black-and-white printer would print them, so you can check that everything still reads; press again for colour. Example: the red and green bars of a chart look the same in grey, so you add labels.",
        |d| d.vw_grayscale(),
    ),
    Entry::new(
        "Dark\nMode",
        "Switches the program's own frame, ribbon and panes between light and dark; the slides keep their colours. Example: work at night with a dark frame around a white slide.",
        |d| d.vw_dark_mode(),
    ),
    // ---- Slide Master ----
    Entry::new(
        "Insert\nLayout",
        "Adds a new layout under the master, a pattern of placeholders that new slides can be made from. Example: add a «two pictures» layout to use on the product slides.",
        |d| d.vw_insert_layout(),
    ),
    Entry::new(
        "Rename",
        "Gives the selected layout a name of your own, the one shown in the Layout and New Slide menus. Example: call the layout «غلاف الوزارة» instead of «تخطيط ١».",
        |d| d.vw_rename(),
    ),
    Entry::new(
        "Insert\nPlaceholder",
        "Adds a placeholder box to the layout: a frame where every slide made from it will get content, text, a picture, a chart, a table or a video. Example: add a picture placeholder on the right so each product slide has a photo there.",
        |d| d.vw_insert_placeholder(),
    ),
    Entry::new(
        "Themes",
        "Applies one of the built-in themes, a complete look of colours, fonts and backgrounds, to the master and all its layouts. Example: pick the dark blue theme and every layout takes it.",
        |d| d.vw_themes(),
    ),
    Entry::new(
        "Close\nMaster View",
        "Leaves the master and goes back to the normal slides, which now show the changes you made to the master and layouts. Example: after adding the logo to the master, close it and see the logo on every slide.",
        |d| d.vw_close_master(),
    ),
    Entry::new(
        "master|Delete",
        "Removes the selected layout from the master; slides that used it move to the nearest remaining layout, so nothing is lost. Example: delete the unused «two pictures» layout to keep the Layout menu short.",
        |d| d.vw_delete_layout(),
    ),
    Entry::new(
        "view|Zoom",
        "Sets how large the slide is drawn in the window, from a far overview to a close-up for fine work; the slide itself keeps its size. Example: choose 200% to align two small boxes exactly, then Fit to Window.",
        |d| d.vw_zoom(),
    ),
];

/// `n` in Arabic-Indic digits.
fn arabic_digits(n: u32) -> String {
    n.to_string().chars().map(|c| c.to_digit(10).and_then(|d| char::from_u32(0x0660 + d)).unwrap_or(c)).collect()
}

impl Draw<'_> {
    /// A master slide at `x, y` of `w × h`: dashed title and body placeholders; returns its
    /// content area.
    pub(super) fn vw_master(&self, x: f32, y: f32, w: f32, h: f32, col: Color32) -> Rect {
        let a = self.ds_slide(x, y, w, h);
        let title = Rect::from_min_max(a.min, pos2(a.max.x, a.min.y + a.height() * 0.3));
        self.dashed_rect(title, col);
        let body = Rect::from_min_max(pos2(a.min.x, a.min.y + a.height() * 0.4), a.max);
        self.dashed_rect(body, col);
        a
    }
    /// The master tree: the master at the top, `n` small layouts under it joined by lines;
    /// returns the layouts' outer rects.
    pub(super) fn vw_tree(&self, n: usize, col: Color32) -> Vec<Rect> {
        let m = self.ds_slide_rect(56.0, 6.0, 64.0, 36.0);
        self.vw_master(56.0, 6.0, 64.0, 36.0, col);
        let n = n.clamp(1, 4);
        let (w, h, gap) = (34.0, 20.0, 8.0);
        let total = n as f32 * (w + gap) - gap;
        let x0 = (SIZE.x - total) / 2.0;
        let y = 70.0;
        self.line(m.center_bottom(), pos2(m.center_bottom().x, m.max.y + 10.0), self.grey(), 1.0);
        (0..n)
            .map(|i| {
                let x = x0 + i as f32 * (w + gap);
                let r = self.ds_slide_rect(x, y, w, h);
                self.line(pos2(m.center_bottom().x, m.max.y + 10.0), pos2(r.center().x, m.max.y + 10.0), self.grey(), 1.0);
                self.line(pos2(r.center().x, m.max.y + 10.0), r.center_top(), self.grey(), 1.0);
                let a = self.ds_slide(x, y, w, h);
                let title = Rect::from_min_max(a.min, pos2(a.max.x, a.min.y + 5.0));
                self.dashed_rect(title, col);
                match i % 3 {
                    0 => self.dashed_rect(Rect::from_min_max(pos2(a.min.x, a.min.y + 8.0), a.max), col),
                    1 => {
                        self.dashed_rect(Rect::from_min_max(pos2(a.min.x, a.min.y + 8.0), pos2(a.center().x - 1.0, a.max.y)), col);
                        self.dashed_rect(Rect::from_min_max(pos2(a.center().x + 1.0, a.min.y + 8.0), a.max), col);
                    }
                    _ => {}
                }
                r
            })
            .collect()
    }
    /// A window frame (title bar with three dots) at `r`; returns the area inside it.
    pub(super) fn vw_window(&self, r: Rect, dark: bool) -> Rect {
        let (chrome, bar) = if dark { (Color32::from_gray(45), Color32::from_gray(30)) } else { (self.t.panel, self.t.hover) };
        self.p.rect_filled(r, 3.0, chrome);
        self.p.rect_stroke(r, 3.0, self.stroke(if dark { Color32::from_gray(80) } else { self.t.border_strong }), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(r.min, vec2(r.width(), 9.0)), 3.0, bar);
        for k in 0..3 {
            self.p.circle_filled(r.min + vec2(5.0 + k as f32 * 5.0, 4.5), 1.5, if dark { Color32::from_gray(140) } else { self.grey() });
        }
        Rect::from_min_max(r.min + vec2(4.0, 12.0), r.max - vec2(4.0, 4.0))
    }
    /// A slide in greys: title, lines and three grey bars.
    pub(super) fn vw_bars(&self, a: Rect, cols: &[Color32]) {
        let bw = a.width() * 0.2;
        for (k, c) in cols.iter().enumerate() {
            let h = [14.0, 20.0, 10.0].get(k % 3).copied().unwrap_or(14.0);
            let x = a.min.x + 2.0 + k as f32 * (bw + 3.0);
            self.p.rect_filled(Rect::from_min_max(pos2(x, a.max.y - h), pos2(x + bw, a.max.y)), 1.0, *c);
        }
    }

    // ---- View ----

    fn vw_slide_master(&self) {
        // A mark put on the master lands on every layout.
        let layouts = self.vw_tree(3, self.accent());
        let m = self.ds_slide_rect(56.0, 6.0, 64.0, 36.0);
        let t = self.reveal(0.1, 0.3);
        if t > 0.0 {
            self.p.circle_filled(m.min + vec2(8.0, 8.0), 4.0, DS_ORANGE);
        }
        for (i, r) in layouts.iter().enumerate() {
            let t = self.reveal(0.4 + i as f32 * 0.15, 0.5 + i as f32 * 0.15);
            if t > 0.0 {
                self.p.circle_filled(r.min + vec2(5.0, 5.0), 2.5, DS_ORANGE);
            }
        }
    }

    fn vw_handout_master(&self) {
        // A portrait page with three slides down it and note lines beside each.
        let a = self.page(52.0, 4.0, 72.0, 96.0);
        self.line(pos2(a.max.x, a.min.y), pos2(a.max.x - 24.0, a.min.y), self.ds_grey(), 1.5);
        self.line(pos2(a.min.x, a.max.y), pos2(a.min.x + 10.0, a.max.y), self.ds_grey(), 1.5);
        for i in 0..3 {
            let t = self.reveal(0.1 + i as f32 * 0.2, 0.25 + i as f32 * 0.2);
            if t <= 0.0 {
                continue;
            }
            let y = a.min.y + 6.0 + i as f32 * 25.0;
            let s = Rect::from_min_size(pos2(a.max.x - 32.0, y), vec2(32.0, 18.0));
            self.p.rect_filled(s, 1.0, Color32::WHITE);
            self.p.rect_stroke(s, 1.0, Stroke::new(1.0, self.accent()), egui::StrokeKind::Inside);
            self.line(pos2(s.max.x - 3.0, s.min.y + 4.0), pos2(s.max.x - 14.0, s.min.y + 4.0), self.ds_ink(), 1.5);
            self.line(pos2(s.max.x - 3.0, s.min.y + 9.0), pos2(s.min.x + 3.0, s.min.y + 9.0), self.ds_grey(), 1.0);
            for k in 0..3 {
                let ly = y + 4.0 + k as f32 * 5.0;
                self.line(pos2(a.min.x, ly), pos2(s.min.x - 4.0, ly), self.ds_grey(), 1.0);
            }
        }
    }

    fn vw_notes_master(&self) {
        // A portrait page: the slide picture on top, the notes text below, header and footer.
        let a = self.page(52.0, 4.0, 72.0, 96.0);
        self.line(pos2(a.max.x, a.min.y), pos2(a.max.x - 24.0, a.min.y), self.ds_grey(), 1.5);
        self.line(pos2(a.min.x, a.max.y), pos2(a.min.x + 10.0, a.max.y), self.ds_grey(), 1.5);
        let s = Rect::from_min_size(pos2(a.min.x + 6.0, a.min.y + 5.0), vec2(a.width() - 12.0, 26.0));
        self.p.rect_filled(s, 1.0, Color32::WHITE);
        self.p.rect_stroke(s, 1.0, Stroke::new(1.0, self.accent()), egui::StrokeKind::Inside);
        self.ds_text(s.shrink2(vec2(3.0, 3.0)), 2);
        let notes = Rect::from_min_max(pos2(a.min.x + 2.0, a.min.y + 36.0), pos2(a.max.x - 2.0, a.max.y - 6.0));
        self.dashed_rect(notes, self.accent());
        let t = self.reveal(0.2, 0.8);
        let n = (6.0 * t).ceil().max(1.0) as usize;
        let clipped =
            self.p.with_clip_rect(Rect::from_min_max(notes.min, pos2(notes.max.x, notes.min.y + 2.0 + n as f32 * 7.0)).intersect(self.p.clip_rect()));
        let d = self.with_painter(&clipped);
        let w = notes.width() - 6.0;
        d.text_lines(notes.shrink2(vec2(3.0, 0.0)), true, &[w, w * 0.9, w * 0.95, w * 0.85, w * 0.9, w * 0.5], 7.0, self.ds_grey());
    }

    fn vw_grid_options(&self) {
        let outer = self.ds_slide_rect(6.0, 14.0, 100.0, 56.0);
        let a = self.ds_slide(6.0, 14.0, 100.0, 56.0);
        // The grid dots come up, then a box snaps onto them.
        let t = self.reveal(0.1, 0.4);
        if t > 0.0 {
            for i in 1..10 {
                for j in 1..6 {
                    let p = pos2(outer.min.x + i as f32 * 10.0, outer.min.y + j as f32 * 9.3);
                    self.p.circle_filled(p, 0.8, self.ds_grey().linear_multiply(t));
                }
            }
        }
        let snap = self.reveal(0.5, 0.8);
        let loose = Rect::from_min_size(pos2(outer.min.x + 33.0, outer.min.y + 23.0), vec2(30.0, 18.0));
        let tight = Rect::from_min_size(pos2(outer.min.x + 30.0, outer.min.y + 18.6), vec2(30.0, 18.0));
        let r = Rect::from_min_max(loose.min.lerp(tight.min, snap), loose.max.lerp(tight.max, snap));
        self.shape(r);
        self.ds_title(a, self.ds_ink());
        // The box: spacing and the snap checkbox.
        let dlg = Rect::from_min_size(self.at(112.0, 14.0), vec2(60.0, 56.0));
        self.p.rect_filled(dlg, 3.0, self.t.menu);
        self.p.rect_stroke(dlg, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(dlg.min, vec2(dlg.width(), 11.0)), 3.0, self.t.hover);
        self.ds_label(pos2(dlg.max.x - 4.0, dlg.min.y + 5.5), "الشبكة", 6.5, self.ink(), Align2::RIGHT_CENTER);
        let f = Rect::from_min_size(pos2(dlg.min.x + 5.0, dlg.min.y + 17.0), vec2(26.0, 10.0));
        self.p.rect_filled(f, 1.0, self.t.input);
        self.p.rect_stroke(f, 1.0, self.stroke(self.t.input_border), egui::StrokeKind::Inside);
        self.ds_label(pos2(f.max.x - 2.0, f.center().y), "١ سم", 6.0, self.ink(), Align2::RIGHT_CENTER);
        self.line(pos2(dlg.max.x - 5.0, f.center().y), pos2(f.max.x + 4.0, f.center().y), self.grey(), 1.5);
        for (k, on) in [(0, true), (1, snap > 0.5 || self.phase >= 1.0)] {
            let y = dlg.min.y + 36.0 + k as f32 * 11.0;
            let cb = Rect::from_min_size(pos2(dlg.max.x - 12.0, y - 3.5), vec2(7.0, 7.0));
            self.p.rect_stroke(cb, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
            if on {
                self.line(cb.min + vec2(1.5, 3.5), cb.min + vec2(3.0, 5.5), self.green(), 1.5);
                self.line(cb.min + vec2(3.0, 5.5), cb.max - vec2(1.0, 5.0), self.green(), 1.5);
            }
            self.line(pos2(cb.min.x - 3.0, y), pos2(dlg.min.x + 5.0, y), self.grey(), 1.5);
        }
    }

    fn vw_notes(&self) {
        // The notes box slides up under the slide.
        let t = self.reveal(0.2, 0.6);
        let nh = 26.0 * t;
        let sh = 92.0 - nh - 4.0;
        let sw = (sh * 16.0 / 9.0).min(150.0);
        let a = self.ds_slide((SIZE.x - sw) / 2.0, 4.0, sw, sh);
        self.ds_text(a, 3);
        if t > 0.0 {
            let r = Rect::from_min_size(self.at((SIZE.x - sw) / 2.0, 8.0 + sh), vec2(sw, nh));
            self.p.rect_filled(r, 2.0, self.t.menu);
            self.p.rect_stroke(r, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
            let clipped = self.p.with_clip_rect(r.intersect(self.p.clip_rect()));
            let d = self.with_painter(&clipped);
            let area = r.shrink2(vec2(6.0, 2.0));
            d.text_lines(area, true, &[area.width() * 0.9, area.width() * 0.7, area.width() * 0.4], 7.0, self.grey());
            d.ds_label(pos2(area.max.x, area.min.y - 2.0), "ملاحظات", 5.5, self.accent(), Align2::RIGHT_TOP);
        }
    }

    fn vw_fit(&self) {
        // A window; the slide, too big for it, shrinks until it fits.
        let win = Rect::from_min_size(self.at(10.0, 6.0), vec2(156.0, 92.0));
        let inner = self.vw_window(win, false);
        let t = self.reveal(0.2, 0.7);
        let big = Rect::from_center_size(inner.center(), vec2(200.0, 112.0));
        let fit_w = inner.width() - 8.0;
        let fit = Rect::from_center_size(inner.center(), vec2(fit_w, fit_w * 9.0 / 16.0));
        let r = Rect::from_min_max(big.min.lerp(fit.min, t), big.max.lerp(fit.max, t));
        let clip = self.p.with_clip_rect(inner.intersect(self.p.clip_rect()));
        let d = self.with_painter(&clip);
        let a = d.ds_slide(r.min.x - self.r.min.x, r.min.y - self.r.min.y, r.width(), r.height());
        d.ds_text(a, 3);
        if t > 0.6 {
            self.measure_h(pos2(inner.min.x + 2.0, inner.max.y - 3.0), pos2(inner.max.x - 2.0, inner.max.y - 3.0));
        }
    }

    fn vw_new_window(&self) {
        // The second window slides out from behind the first, showing another slide.
        let t = self.reveal(0.15, 0.6);
        let first = Rect::from_min_size(self.at(6.0, 10.0), vec2(96.0, 66.0));
        let second = Rect::from_min_size(self.at(6.0 + 68.0 * t, 10.0 + 18.0 * t), vec2(96.0, 66.0));
        let a = self.vw_window(first, false);
        let s = self.ds_slide(a.min.x - self.r.min.x, a.min.y - self.r.min.y, a.width(), a.height());
        self.ds_text(s, 3);
        self.badge(pos2(s.min.x + 8.0, s.max.y - 5.0), "٢", self.grey());
        let b = self.vw_window(second, false);
        let s2 = self.ds_slide(b.min.x - self.r.min.x, b.min.y - self.r.min.y, b.width(), b.height());
        self.ds_title(s2, DS_BLUE);
        self.picture(Rect::from_min_max(pos2(s2.min.x, s2.min.y + 9.0), s2.max));
        self.badge(pos2(s2.min.x + 8.0, s2.max.y - 5.0), "٩", self.accent());
    }

    fn vw_grayscale(&self) {
        let cols = [self.red(), DS_TEAL, DS_ORANGE];
        let greys = [Color32::from_gray(110), Color32::from_gray(140), Color32::from_gray(190)];
        self.ds_pair(
            |d, a| {
                d.ds_title(a, DS_BLUE);
                d.vw_bars(Rect::from_min_max(pos2(a.min.x, a.min.y + 8.0), pos2(a.max.x - 30.0, a.max.y)), &cols);
                d.picture(Rect::from_min_size(pos2(a.max.x - 26.0, a.min.y + 10.0), vec2(26.0, 18.0)));
            },
            |d, a| {
                d.ds_title(a, Color32::from_gray(60));
                d.vw_bars(Rect::from_min_max(pos2(a.min.x, a.min.y + 8.0), pos2(a.max.x - 30.0, a.max.y)), &greys);
                let pic = Rect::from_min_size(pos2(a.max.x - 26.0, a.min.y + 10.0), vec2(26.0, 18.0));
                d.p.rect_filled(pic, 1.0, Color32::from_gray(225));
                d.p.rect_stroke(pic, 1.0, d.stroke(d.t.border_strong), egui::StrokeKind::Inside);
                let base = pic.max.y - 2.0;
                d.p.add(Shape::convex_polygon(
                    vec![pos2(pic.min.x + 2.0, base), pos2(pic.min.x + 9.0, pic.min.y + 8.0), pos2(pic.min.x + 16.0, base)],
                    Color32::from_gray(150),
                    Stroke::NONE,
                ));
                d.p.add(Shape::convex_polygon(
                    vec![pos2(pic.min.x + 10.0, base), pos2(pic.min.x + 18.0, pic.min.y + 5.0), pos2(pic.max.x - 2.0, base)],
                    Color32::from_gray(120),
                    Stroke::NONE,
                ));
                d.p.circle_filled(pos2(pic.min.x + 6.0, pic.min.y + 5.0), 1.8, Color32::from_gray(200));
            },
        );
    }

    fn vw_dark_mode(&self) {
        // The window's frame goes dark; the slide stays white. Drawn as two windows.
        let t = self.reveal(0.2, 0.6);
        let left = Rect::from_min_size(self.at(4.0, 10.0), vec2(78.0, 84.0));
        let a = self.vw_window(left, false);
        let s = self.ds_slide(a.min.x - self.r.min.x + 2.0, a.min.y - self.r.min.y + 10.0, a.width() - 4.0, (a.width() - 4.0) * 9.0 / 16.0);
        self.ds_text(s, 3);
        self.p.rect_filled(Rect::from_min_size(a.min, vec2(a.width(), 7.0)), 1.0, self.t.hover);
        let right = Rect::from_min_size(self.at(94.0, 10.0), vec2(78.0, 84.0));
        let dark = t > 0.5 || self.phase >= 1.0;
        let b = self.vw_window(right, dark);
        let s2 = self.ds_slide(b.min.x - self.r.min.x + 2.0, b.min.y - self.r.min.y + 10.0, b.width() - 4.0, (b.width() - 4.0) * 9.0 / 16.0);
        self.ds_text(s2, 3);
        self.p.rect_filled(Rect::from_min_size(b.min, vec2(b.width(), 7.0)), 1.0, if dark { Color32::from_gray(60) } else { self.t.hover });
        self.arrow_right(self.at(84.0, 52.0), self.at(92.0, 52.0));
        // A moon on the dark side, a sun on the light.
        self.p.circle_filled(self.at(16.0, 86.0), 4.0, DS_ORANGE);
        let moon = self.at(160.0, 86.0);
        self.p.circle_filled(moon, 4.5, if dark { Color32::from_gray(230) } else { self.grey() });
        self.p.circle_filled(moon + vec2(2.5, -1.5), 3.8, if dark { Color32::from_gray(45) } else { self.t.panel });
    }

    // ---- Slide Master ----

    fn vw_delete_layout(&self) {
        // Three layouts under the master; the middle one fades and the last slides up into its place.
        let layouts = self.vw_tree(3, self.ds_grey());
        let t = self.reveal(0.3, 0.7);
        if let (Some(gone), Some(last)) = (layouts.get(1), layouts.get(2)) {
            if t < 1.0 {
                self.p.rect_filled(gone.expand(1.0), 1.0, self.t.panel.linear_multiply(t));
                self.p.rect_stroke(*gone, 1.0, Stroke::new(1.5, self.red().linear_multiply(1.0 - t)), egui::StrokeKind::Inside);
                let c = gone.center();
                self.line(c + vec2(-4.0, -4.0), c + vec2(4.0, 4.0), self.red().linear_multiply(1.0 - t), 1.5);
                self.line(c + vec2(-4.0, 4.0), c + vec2(4.0, -4.0), self.red().linear_multiply(1.0 - t), 1.5);
            }
            if t > 0.0 {
                let moved = last.translate((gone.min - last.min) * t);
                self.p.rect_filled(moved.expand(1.0), 1.0, self.t.panel);
                let a = self.ds_slide(moved.min.x - self.r.min.x, moved.min.y - self.r.min.y, moved.width(), moved.height());
                self.ds_text(a, 2);
            }
        }
    }

    fn vw_zoom(&self) {
        // A window whose slide grows from an overview to a close-up, with a percentage.
        let win = Rect::from_min_size(self.at(10.0, 6.0), vec2(156.0, 92.0));
        let inner = self.vw_window(win, false);
        let t = self.reveal(0.2, 0.7);
        let small = Rect::from_center_size(inner.center(), vec2(80.0, 45.0));
        let big = Rect::from_center_size(inner.center() + vec2(20.0, 10.0), vec2(220.0, 124.0));
        let r = Rect::from_min_max(small.min.lerp(big.min, t), small.max.lerp(big.max, t));
        let clip = self.p.with_clip_rect(inner.intersect(self.p.clip_rect()));
        let d = self.with_painter(&clip);
        let a = d.ds_slide(r.min.x - self.r.min.x, r.min.y - self.r.min.y, r.width(), r.height());
        d.ds_text(a, 3);
        let pct = 100.0 + 100.0 * t;
        let label = format!("{}٪", arabic_digits(pct as u32));
        self.ds_chip(pos2(inner.max.x - 22.0, inner.max.y - 9.0), &label, 34.0);
    }

    fn vw_insert_layout(&self) {
        let layouts = self.vw_tree(3, self.ds_grey());
        // The third layout is the new one: it appears with a plus.
        let t = self.reveal(0.3, 0.6);
        if let Some(r) = layouts.get(2) {
            if t < 1.0 {
                self.p.rect_filled(r.expand(1.0), 1.0, Color32::WHITE.linear_multiply(1.0 - t));
                self.p.rect_filled(r.expand(1.0), 1.0, self.t.panel.linear_multiply(1.0 - t));
            }
            if t > 0.0 {
                self.p.rect_stroke(*r, 1.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
                self.p.circle_filled(r.right_top(), 5.0, self.accent());
                self.line(r.right_top() + vec2(-3.0, 0.0), r.right_top() + vec2(3.0, 0.0), Color32::WHITE, 1.5);
                self.line(r.right_top() + vec2(0.0, -3.0), r.right_top() + vec2(0.0, 3.0), Color32::WHITE, 1.5);
            }
        }
    }

    fn vw_rename(&self) {
        // A layout with its name under it, and the box typing the new name.
        let r = self.ds_slide_rect(8.0, 26.0, 48.0, 27.0);
        let a = self.ds_slide(8.0, 26.0, 48.0, 27.0);
        self.dashed_rect(Rect::from_min_max(a.min, pos2(a.max.x, a.min.y + 6.0)), self.ds_grey());
        self.dashed_rect(Rect::from_min_max(pos2(a.min.x, a.min.y + 9.0), a.max), self.ds_grey());
        let t = self.reveal(0.3, 0.8);
        let typed = t > 0.5 || self.phase >= 1.0;
        self.ds_label(
            pos2(r.center().x, r.max.y + 8.0),
            if typed { "غلاف الوزارة" } else { "تخطيط ١" },
            6.5,
            if typed { self.accent() } else { self.grey() },
            Align2::CENTER_CENTER,
        );
        let dlg = Rect::from_min_size(self.at(66.0, 24.0), vec2(104.0, 52.0));
        self.p.rect_filled(dlg, 3.0, self.t.menu);
        self.p.rect_stroke(dlg, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(dlg.min, vec2(dlg.width(), 12.0)), 3.0, self.t.hover);
        self.ds_label(pos2(dlg.max.x - 5.0, dlg.min.y + 6.0), "إعادة تسمية", 7.0, self.ink(), Align2::RIGHT_CENTER);
        let f = Rect::from_min_size(pos2(dlg.min.x + 8.0, dlg.min.y + 20.0), vec2(dlg.width() - 16.0, 12.0));
        self.p.rect_filled(f, 1.0, self.t.input);
        self.p.rect_stroke(f, 1.0, self.stroke(self.accent()), egui::StrokeKind::Inside);
        let name = "غلاف الوزارة";
        let n = name.chars().count();
        let shown: String = name.chars().take(((n as f32) * t).round() as usize).collect();
        let shown = if self.phase >= 1.0 { name.to_string() } else { shown };
        self.ds_label(pos2(f.max.x - 3.0, f.center().y), &shown, 7.0, self.ink(), Align2::RIGHT_CENTER);
        let caret_x = f.max.x - 3.0 - shown.chars().count() as f32 * 3.6;
        self.caret(pos2(caret_x.max(f.min.x + 2.0), f.min.y + 2.0), 8.0);
        self.ds_chip(pos2(dlg.center().x, dlg.max.y - 10.0), "موافق", 36.0);
    }

    fn vw_insert_placeholder(&self) {
        let a = self.vw_master(24.0, 16.0, 128.0, 72.0, self.ds_grey());
        // Body placeholder shrinks to the left half; a picture placeholder lands on the right.
        let body = Rect::from_min_max(pos2(a.min.x, a.min.y + a.height() * 0.4), a.max);
        self.p.rect_filled(body.expand(1.0), 0.0, Color32::WHITE);
        self.dashed_rect(Rect::from_min_max(body.min, pos2(body.center().x - 3.0, body.max.y)), self.ds_grey());
        let t = self.reveal(0.3, 0.7);
        let full = Rect::from_min_max(pos2(body.center().x + 3.0, body.min.y), body.max);
        let r = Rect::from_center_size(full.center(), full.size() * t.max(0.05));
        if t > 0.0 {
            self.dashed_rect(r, self.accent());
            if t > 0.6 {
                // A little picture glyph in the middle: a frame with a hill and sun.
                let g = Rect::from_center_size(r.center(), vec2(16.0, 12.0));
                self.picture(g);
            }
        }
    }

    fn vw_themes(&self) {
        // Three theme tiles; the lit one moves, and the master below takes its look.
        let t = self.reveal(0.3, 0.6);
        let pick = if t > 0.5 || self.phase >= 1.0 { 1 } else { 0 };
        let tiles = [
            (Color32::WHITE, Color32::from_gray(60), [DS_BLUE, DS_ORANGE, DS_TEAL]),
            (DS_BLUE.lerp_to_gamma(Color32::BLACK, 0.5), Color32::WHITE, [DS_ORANGE, DS_TEAL, DS_PURPLE]),
            (DS_TEAL.lerp_to_gamma(Color32::WHITE, 0.8), DS_TEAL, [DS_TEAL, DS_ORANGE, DS_BLUE]),
        ];
        for (i, (bg, fg, cols)) in tiles.iter().enumerate() {
            let x = 22.0 + i as f32 * 46.0;
            let r = self.ds_theme_tile(x, 8.0, 40.0, 26.0, *bg, *fg, cols);
            if i == pick {
                self.p.rect_stroke(r.expand(1.5), 3.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Outside);
            }
        }
        let (bg, fg, cols) = tiles.get(pick).copied().unwrap_or((Color32::WHITE, Color32::from_gray(60), [DS_BLUE, DS_ORANGE, DS_TEAL]));
        let outer = self.ds_slide_rect(40.0, 44.0, 96.0, 54.0);
        let a = self.ds_slide(40.0, 44.0, 96.0, 54.0);
        self.p.rect_filled(outer.shrink(1.0), 1.0, bg);
        self.dashed_rect(Rect::from_min_max(a.min, pos2(a.max.x, a.min.y + 14.0)), fg);
        self.line(pos2(a.max.x - 4.0, a.min.y + 7.0), pos2(a.max.x - 40.0, a.min.y + 7.0), fg, 3.0);
        for (k, c) in cols.iter().enumerate() {
            let w = (a.width() - 8.0) / 3.0;
            let x = a.min.x + 2.0 + k as f32 * (w + 2.0);
            self.p.rect_filled(Rect::from_min_max(pos2(x, a.min.y + 20.0), pos2(x + w, a.max.y)), 1.0, c.linear_multiply(0.8));
        }
    }

    fn vw_close_master(&self) {
        self.ds_pair(
            |d, a| {
                // The master view: the master with dashed placeholders, two layouts under it.
                d.dashed_rect(Rect::from_min_max(a.min, pos2(a.max.x, a.min.y + 7.0)), d.accent());
                d.dashed_rect(Rect::from_min_max(pos2(a.min.x, a.min.y + 10.0), pos2(a.max.x, a.max.y - 10.0)), d.accent());
                for k in 0..2 {
                    let r = Rect::from_min_size(pos2(a.max.x - 14.0 - k as f32 * 18.0, a.max.y - 8.0), vec2(14.0, 8.0));
                    d.p.rect_stroke(r, 1.0, d.stroke(d.t.border_strong), egui::StrokeKind::Inside);
                }
                d.p.circle_filled(a.min + vec2(4.0, 3.0), 2.5, DS_ORANGE);
            },
            |d, a| {
                // The normal view: a slide list on the right, the slide with the logo it got.
                let list = Rect::from_min_max(pos2(a.max.x - 12.0, a.min.y), a.max);
                for k in 0..3 {
                    let r = Rect::from_min_size(pos2(list.min.x, list.min.y + k as f32 * 11.0), vec2(12.0, 8.0));
                    d.p.rect_stroke(
                        r,
                        1.0,
                        if k == 0 { Stroke::new(1.0, d.accent()) } else { d.stroke(d.t.border_strong) },
                        egui::StrokeKind::Inside,
                    );
                    d.p.circle_filled(r.min + vec2(2.0, 2.0), 1.0, DS_ORANGE);
                }
                let s = Rect::from_min_max(a.min, pos2(list.min.x - 4.0, a.max.y));
                d.ds_text(s, 3);
                d.p.circle_filled(s.min + vec2(4.0, 3.0), 2.5, DS_ORANGE);
            },
        );
        self.cross(self.at(88.0, 22.0));
    }
}
