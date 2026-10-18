//! Insert tab thumbnails (and the Draw tab's five): what lands on the slide when you press the
//! button — a table, a picture, a chart, an icon, a link, a comment, a date, a video — drawn in
//! the accent colour on a little 16:9 slide, each a short film. The Draw tab's inking entries
//! show an ink stroke being selected, erased or turned into a shape, text or an equation. The
//! slide kit itself (`hm_slide`, `hm_pair`, `hm_title`…) lives in `home.rs`.

#![allow(unused_imports)]

use egui::{Align2, Color32, Pos2, Rect, Shape, Stroke, pos2, vec2};

use super::Entry;
use super::draw::{Dir, Draw, SIZE};

pub const ENTRIES: &[Entry] = &[
    // ---- Insert ----
    Entry::new(
        "Table",
        "Puts a table on the slide: drag over the grid to choose how many rows and columns, and type in the cells. Example: a ٣ × ٣ table for the names, the dates and the amounts.",
        |d| d.in_table(),
    ),
    Entry::new(
        "Pictures",
        "Places a picture file from your computer on the slide; drag it to move it and pull its corners to resize it. Example: a photo of the device is placed beside its description.",
        |d| d.in_pictures(),
    ),
    Entry::new(
        "Screenshot",
        "Takes a picture of another open window, or of a part of the screen you drag over, and puts it on the slide. Example: a snapshot of the budget sheet lands on the slide.",
        |d| d.in_screenshot(),
    ),
    Entry::new(
        "Icons",
        "Opens a gallery of simple symbols (people, tools, arrows, buildings…) to put on the slide as shapes you can colour and resize. Example: a gear icon next to the word «الصيانة».",
        |d| d.in_icons(),
    ),
    Entry::new(
        "SmartArt",
        "Puts a ready-made diagram on the slide (a list, a process, a cycle, a hierarchy…) and you type the words into its boxes. Example: a tree of the unit's sections and who reports to whom.",
        |d| d.in_smartart(),
    ),
    Entry::new(
        "Chart",
        "Puts a chart (bars, lines, a pie…) on the slide, with a small table where you type the numbers it shows. Example: a bar chart of the monthly totals.",
        |d| d.in_chart(),
    ),
    Entry::new(
        "Zoom",
        "Puts a live thumbnail of another slide or section on this slide; clicking it during the show jumps there and zooms in. Example: a summary slide with a thumbnail of each section.",
        |d| d.in_zoom(),
    ),
    Entry::new(
        "Link",
        "Turns the selected text or shape into a link: clicking it during the show opens a web page, a file or another slide. Example: the word «الموقع» opens the unit's web page.",
        |d| d.in_link(),
    ),
    Entry::new(
        "Action",
        "Makes the selected shape do something when clicked or pointed at during the show: jump to a slide, open a file, play a sound. Example: a «التالي» button jumps to the next slide.",
        |d| d.in_action(),
    ),
    Entry::new(
        "Comment",
        "Attaches a note to the slide for the people you work with; it shows in a side pane, not in the show. Example: «please check this number» is pinned next to the table.",
        |d| d.in_comment(),
    ),
    Entry::new(
        "Text\nBox",
        "Draws a box on the slide to type free text anywhere, outside the layout's placeholders. Example: a source line is added under a chart.",
        |d| d.in_text_box(),
    ),
    Entry::new(
        "Header &\nFooter",
        "Puts the date, the slide number and a line of footer text at the bottom of every slide (or only this one). Example: every slide shows the unit's name and its number at the bottom.",
        |d| d.in_header_footer(),
    ),
    Entry::new(
        "WordArt",
        "Puts decorative text on the slide: big letters with an outline, a shadow, a gradient or a 3-D look. Example: the word «مرحباً» in large outlined letters on the opening slide.",
        |d| d.in_wordart(),
    ),
    Entry::new(
        "Date &\nTime",
        "Inserts today's date (and the time if you like) where the caret is, in the format you choose, and can keep it updated automatically. Example: ٢٠٢٦/١٠/١٨ is written at the caret.",
        |d| d.in_datetime(),
    ),
    Entry::new(
        "Slide\nNumber",
        "Inserts the number of the current slide where the caret is; it changes by itself when the slides are reordered. Example: the third slide shows ٣.",
        |d| d.in_slide_number(),
    ),
    Entry::new(
        "Equation",
        "Puts a mathematical formula on the slide, with fractions, powers, roots and symbols laid out properly. Example: a fraction with a + b above the line and ٢ below it.",
        |d| d.in_equation(),
    ),
    Entry::new(
        "Symbol",
        "Opens a table of characters that are not on the keyboard (©, ±, →, Ω…) to insert one where the caret is. Example: the degree sign in ٢٥°.",
        |d| d.in_symbol(),
    ),
    Entry::new(
        "Video",
        "Puts a video file on the slide, with play controls; it plays during the show. Example: a short clip of the procedure plays on the training slide.",
        |d| d.in_video(),
    ),
    Entry::new(
        "Audio",
        "Puts a sound file on the slide as a small speaker icon; it plays during the show, when clicked or automatically. Example: a recorded explanation plays when the slide opens.",
        |d| d.in_audio(),
    ),
    // ---- Draw ----
    Entry::new(
        "Select",
        "Lasso Select: drag a loop around the ink strokes you want, to move, resize or delete them together. Example: circle three strokes of a sketch to move them aside.",
        |d| d.in_lasso(),
    ),
    Entry::new("Eraser", "Rubs out ink strokes: drag over a stroke and it disappears. Example: a wrong line in a sketch is wiped away.", |d| {
        d.in_eraser()
    }),
    Entry::new(
        "Ink to\nShape",
        "Turns a shape you drew by hand into a clean one: a wobbly circle becomes a perfect circle, a rough box a straight rectangle. Example: a hand-drawn ring around a number becomes a neat ellipse.",
        |d| d.in_ink_to_shape(),
    ),
    Entry::new("Ink to\nText", "Turns handwriting into typed text in a text box. Example: a word written with the pen becomes a typed word.", |d| {
        d.in_ink_to_text()
    }),
    Entry::new(
        "Ink to\nMath",
        "Turns a formula written by hand into a proper typed equation. Example: a handwritten x² + y² becomes a clean equation.",
        |d| d.in_ink_to_math(),
    ),
];

impl Draw<'_> {
    // ---- helpers ----

    /// A tray (a little gallery) of `w × h` at `x, y`.
    pub(super) fn in_tray(&self, x: f32, y: f32, w: f32, h: f32) -> Rect {
        let r = Rect::from_min_size(self.at(x, y), vec2(w, h));
        self.p.rect_filled(r, 3.0, self.t.menu);
        self.p.rect_stroke(r, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        r
    }
    /// A wobbly hand-drawn stroke from `a` to `b` (a sine wobble across the line) in `c`;
    /// only the first `until` share of it when below 1.
    pub(super) fn in_ink(&self, a: Pos2, b: Pos2, amp: f32, c: Color32, until: f32) {
        let n = 24;
        let last = ((n as f32) * until.clamp(0.0, 1.0)) as usize;
        if last < 1 {
            return;
        }
        let dir = b - a;
        let normal = vec2(-dir.y, dir.x).normalized();
        let pts: Vec<Pos2> = (0..=last)
            .map(|i| a + dir * (i as f32 / n as f32) + normal * (amp * (i as f32 * 1.1).sin() + amp * 0.4 * (i as f32 * 2.7).cos()))
            .collect();
        self.p.add(Shape::line(pts, Stroke::new(2.0, c)));
    }
    /// A wobbly hand-drawn ring around `c` with radius `r`, `share` of the way round.
    pub(super) fn in_ink_ring(&self, c: Pos2, r: f32, col: Color32, share: f32) {
        let n = 28;
        let last = ((n as f32) * share.clamp(0.0, 1.0)) as usize;
        if last < 2 {
            return;
        }
        let pts: Vec<Pos2> = (0..=last)
            .map(|i| {
                let a = i as f32 / n as f32 * std::f32::consts::TAU - 1.0;
                let rr = r + 1.6 * (i as f32 * 1.3).sin() + 0.8 * (i as f32 * 3.1).cos();
                pos2(c.x + rr * 1.25 * a.cos(), c.y + rr * a.sin())
            })
            .collect();
        self.p.add(Shape::line(pts, Stroke::new(2.0, col)));
    }
    /// A dashed loop (a lasso) around `c`, radii `rx, ry`, `share` of the way round.
    pub(super) fn in_lasso_loop(&self, c: Pos2, rx: f32, ry: f32, share: f32) {
        let n = 36;
        let last = ((n as f32) * share.clamp(0.0, 1.0)) as usize;
        for i in 0..last {
            if i % 2 == 1 {
                continue;
            }
            let a0 = i as f32 / n as f32 * std::f32::consts::TAU;
            let a1 = (i + 1) as f32 / n as f32 * std::f32::consts::TAU;
            self.line(pos2(c.x + rx * a0.cos(), c.y + ry * a0.sin()), pos2(c.x + rx * a1.cos(), c.y + ry * a1.sin()), self.accent(), 1.2);
        }
    }
    /// A small calendar icon centred at `c`.
    pub(super) fn in_calendar(&self, c: Pos2) {
        let r = Rect::from_center_size(c, vec2(16.0, 15.0));
        self.p.rect_filled(r, 2.0, Color32::WHITE);
        self.p.rect_stroke(r, 2.0, self.stroke(self.ink()), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(r.min, vec2(r.width(), 4.0)), 2.0, self.accent());
        for i in 0..3 {
            for j in 0..3 {
                self.p.circle_filled(pos2(r.min.x + 3.5 + j as f32 * 4.5, r.min.y + 7.5 + i as f32 * 3.0), 0.9, self.grey());
            }
        }
    }
    /// A folder icon centred at `c`.
    pub(super) fn in_folder(&self, c: Pos2) {
        let body = Rect::from_center_size(c + vec2(0.0, 2.0), vec2(30.0, 20.0));
        let tab = Rect::from_min_size(body.min - vec2(0.0, 4.0), vec2(12.0, 6.0));
        let col = Color32::from_rgb(240, 190, 60);
        self.p.rect_filled(tab, 2.0, col);
        self.p.rect_filled(body, 2.0, col);
        self.p.rect_filled(Rect::from_min_max(pos2(body.min.x, body.min.y + 4.0), body.max), 2.0, Color32::from_rgb(250, 210, 90));
    }
    /// A chain link (two overlapping rounded rings) centred at `c`.
    pub(super) fn in_chain(&self, c: Pos2) {
        let s = Stroke::new(1.8, self.accent());
        self.p.rect_stroke(Rect::from_center_size(c + vec2(-4.0, 0.0), vec2(11.0, 6.0)), 3.0, s, egui::StrokeKind::Middle);
        self.p.rect_stroke(Rect::from_center_size(c + vec2(4.0, 0.0), vec2(11.0, 6.0)), 3.0, s, egui::StrokeKind::Middle);
    }

    // ---- Insert ----

    /// A grid picker on the left (the chosen cells lit), an arrow, and a table on the slide.
    pub(super) fn in_table(&self) {
        let tray = self.in_tray(4.0, 24.0, 52.0, 44.0);
        let g = tray.shrink(5.0);
        self.grid(g, 5, 6, self.t.border);
        let k = self.reveal(0.0, 0.35);
        let sel = Rect::from_min_max(g.min, pos2(g.min.x + g.width() * (1.0 + 2.0 * k) / 6.0, g.min.y + g.height() * (1.0 + 2.0 * k) / 5.0));
        self.p.rect_filled(sel, 0.0, self.t.selection);
        self.p.rect_stroke(sel, 0.0, Stroke::new(1.2, self.accent()), egui::StrokeKind::Inside);
        self.pointer(sel.max - vec2(2.0, 2.0));
        let go = self.reveal(0.3, 0.5);
        if go > 0.0 {
            self.arrow_right(self.at(60.0, 46.0), self.at(60.0 + 14.0 * go, 46.0));
        }
        let a = self.hm_slide(78.0, 22.0, 92.0, 52.0);
        self.hm_title(a, self.ink());
        let t = self.reveal(0.5, 0.85);
        if t > 0.0 {
            let r = Rect::from_min_max(pos2(a.min.x, a.min.y + 9.0), a.max);
            self.table(Draw::hm_grow(r, 0.3 + 0.7 * t), 3, 3);
        }
    }
    /// A picture file comes out of a folder onto the slide.
    pub(super) fn in_pictures(&self) {
        self.in_folder(self.at(30.0, 52.0));
        let small = Rect::from_center_size(self.at(30.0, 52.0), vec2(22.0, 16.0));
        self.picture(small);
        let go = self.reveal(0.0, 0.25);
        if go > 0.0 {
            self.arrow_right(self.at(52.0, 52.0), self.at(52.0 + 20.0 * go, 52.0));
        }
        let a = self.hm_slide(82.0, 26.0, 88.0, 50.0);
        self.hm_title(a, self.ink());
        let text = Rect::from_min_max(pos2(a.max.x - 32.0, a.min.y + 8.0), a.max);
        self.text_lines(text, true, &[30.0, 22.0, 26.0], 7.0, self.grey());
        let k = self.reveal(0.25, 0.65);
        if k > 0.0 {
            let r = Draw::hm_grow(Rect::from_min_size(pos2(a.min.x + 1.0, a.min.y + 9.0), vec2(38.0, 26.0)), 0.3 + 0.7 * k);
            self.picture(r);
            if k >= 1.0 {
                self.handles(r);
            }
        }
    }
    /// Another window on the left, a dashed region dragged over it; the region lands on the
    /// slide as a picture.
    pub(super) fn in_screenshot(&self) {
        let win = Rect::from_min_size(self.at(4.0, 18.0), vec2(68.0, 50.0));
        self.p.rect_filled(win, 2.0, Color32::WHITE);
        self.p.rect_stroke(win, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.p.rect_filled(Rect::from_min_size(win.min, vec2(win.width(), 8.0)), 2.0, self.t.hover);
        for i in 0..3 {
            self.p.circle_filled(pos2(win.min.x + 5.0 + i as f32 * 5.0, win.min.y + 4.0), 1.5, self.grey());
        }
        let content = Rect::from_min_max(pos2(win.min.x + 5.0, win.min.y + 12.0), win.max - vec2(5.0, 4.0));
        self.table(Rect::from_min_size(content.min, vec2(content.width(), 20.0)), 3, 3);
        self.text_lines(Rect::from_min_max(pos2(content.min.x, content.min.y + 22.0), content.max), false, &[40.0, 30.0], 6.0, self.grey());
        let k = self.reveal(0.0, 0.3);
        let region = Rect::from_min_size(content.min - vec2(2.0, 2.0), vec2((content.width() + 4.0) * k.max(0.1), 24.0 * k.max(0.1)));
        self.dashed_rect(region, self.accent());
        self.pointer(region.max);
        let go = self.reveal(0.3, 0.5);
        if go > 0.0 {
            self.arrow_right(self.at(76.0, 46.0), self.at(76.0 + 14.0 * go, 46.0));
        }
        let a = self.hm_slide(94.0, 22.0, 78.0, 44.0);
        self.hm_title(a, self.ink());
        let t = self.reveal(0.5, 0.85);
        if t > 0.0 {
            let r = Draw::hm_grow(Rect::from_min_max(pos2(a.min.x, a.min.y + 9.0), a.max), 0.3 + 0.7 * t);
            self.p.rect_filled(r, 1.0, Color32::WHITE);
            self.table(r.shrink(2.0), 3, 3);
            self.p.rect_stroke(r, 1.0, Stroke::new(1.2, self.accent()), egui::StrokeKind::Inside);
        }
    }
    /// A gallery of icons; the gear flies onto the slide.
    pub(super) fn in_icons(&self) {
        let tray = self.in_tray(4.0, 10.0, 56.0, 84.0);
        let cells: Vec<Pos2> = (0..6).map(|i| pos2(tray.min.x + 14.0 + (i % 2) as f32 * 28.0, tray.min.y + 14.0 + (i / 2) as f32 * 28.0)).collect();
        for (i, c) in cells.iter().enumerate() {
            match i {
                0 => {
                    self.p.circle_stroke(*c, 7.0, Stroke::new(1.8, self.ink()));
                }
                1 => {
                    self.p.rect_filled(Rect::from_center_size(*c, vec2(22.0, 22.0)), 3.0, self.t.checked);
                    self.gear(*c, 5.0);
                }
                2 => self.magnifier(*c - vec2(3.0, 3.0)),
                3 => self.speaker(*c),
                4 => {
                    self.p.circle_filled(*c - vec2(0.0, 4.0), 3.5, self.ink());
                    self.p.add(Shape::convex_polygon(
                        vec![*c + vec2(-7.0, 8.0), *c + vec2(7.0, 8.0), *c + vec2(0.0, -1.0)],
                        self.ink(),
                        Stroke::NONE,
                    ));
                }
                _ => {
                    self.p.rect_stroke(Rect::from_center_size(*c, vec2(14.0, 12.0)), 1.0, Stroke::new(1.8, self.ink()), egui::StrokeKind::Inside);
                    self.line(*c + vec2(-7.0, -2.0), *c + vec2(7.0, -2.0), self.ink(), 1.8);
                }
            }
        }
        let a = self.hm_slide(68.0, 22.0, 102.0, 58.0);
        self.hm_title(a, self.ink());
        self.text_lines(Rect::from_min_max(pos2(a.max.x - 50.0, a.min.y + 10.0), a.max), true, &[46.0, 36.0, 40.0], 8.0, self.grey());
        let k = self.reveal(0.1, 0.6);
        if k > 0.0 {
            let from = cells.get(1).copied().unwrap_or(tray.center());
            let to = pos2(a.min.x + 18.0, a.min.y + 28.0);
            let c = from + (to - from) * k;
            let s = 5.0 + 6.0 * k;
            self.p.circle_stroke(c, s, Stroke::new(2.0, self.accent()));
            for i in 0..8 {
                let ang = i as f32 * std::f32::consts::FRAC_PI_4;
                let dir = vec2(ang.cos(), ang.sin());
                self.line(c + dir * (s + 1.5), c + dir * (s + 5.0), self.accent(), 2.0);
            }
        }
    }
    /// A gallery of three diagram kinds; the hierarchy lands on the slide.
    pub(super) fn in_smartart(&self) {
        let tray = self.in_tray(4.0, 14.0, 46.0, 76.0);
        let s = Stroke::new(1.2, self.ink());
        // process
        let y0 = tray.min.y + 12.0;
        for i in 0..3 {
            let r = Rect::from_min_size(pos2(tray.min.x + 5.0 + i as f32 * 12.0, y0 - 4.0), vec2(9.0, 8.0));
            self.p.rect_stroke(r, 1.0, s, egui::StrokeKind::Inside);
        }
        // cycle
        let c = pos2(tray.center().x, tray.min.y + 36.0);
        for i in 0..3 {
            let ang = i as f32 * std::f32::consts::TAU / 3.0 - std::f32::consts::FRAC_PI_2;
            self.p.circle_stroke(c + 8.0 * vec2(ang.cos(), ang.sin()), 3.5, s);
        }
        // hierarchy (chosen)
        let hy = tray.min.y + 60.0;
        self.p.rect_filled(Rect::from_center_size(pos2(tray.center().x, hy), vec2(36.0, 22.0)), 3.0, self.t.checked);
        self.p.rect_stroke(
            Rect::from_center_size(pos2(tray.center().x, hy - 6.0), vec2(10.0, 7.0)),
            1.0,
            Stroke::new(1.2, self.accent()),
            egui::StrokeKind::Inside,
        );
        for dx in [-9.0, 9.0] {
            self.p.rect_stroke(
                Rect::from_center_size(pos2(tray.center().x + dx, hy + 6.0), vec2(10.0, 7.0)),
                1.0,
                Stroke::new(1.2, self.accent()),
                egui::StrokeKind::Inside,
            );
            self.line(pos2(tray.center().x, hy - 2.5), pos2(tray.center().x + dx, hy + 2.5), self.accent(), 1.0);
        }
        let a = self.hm_slide(58.0, 20.0, 112.0, 63.0);
        self.hm_title(a, self.ink());
        let k = self.reveal(0.15, 0.6);
        if k > 0.0 {
            let cx = a.center().x;
            let top = pos2(cx, a.min.y + 18.0);
            let scale = 0.4 + 0.6 * k;
            let bw = 24.0 * scale;
            let bh = 11.0 * scale;
            self.shape(Rect::from_center_size(top, vec2(bw, bh)));
            for dx in [-30.0 * scale, 0.0, 30.0 * scale] {
                let b = pos2(cx + dx, top.y + 24.0 * scale);
                self.line(pos2(cx, top.y + bh / 2.0), pos2(b.x, b.y - bh / 2.0), self.accent(), 1.2);
                self.shape(Rect::from_center_size(b, vec2(bw, bh)));
            }
        }
    }
    /// A bar chart grows on the slide.
    pub(super) fn in_chart(&self) {
        let a = self.hm_slide_center();
        self.hm_title(a, self.ink());
        let r = Rect::from_min_max(pos2(a.min.x + 2.0, a.min.y + 10.0), pos2(a.max.x - 2.0, a.max.y));
        self.p.rect_filled(r, 1.0, Color32::WHITE);
        self.p.rect_stroke(r, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.line(pos2(r.min.x + 4.0, r.max.y - 4.0), pos2(r.max.x - 4.0, r.max.y - 4.0), self.grey(), 1.0);
        let k = self.reveal(0.1, 0.7);
        let n = 5;
        let bw = (r.width() - 8.0) / n as f32;
        for (i, h) in [0.5, 0.8, 0.35, 0.95, 0.6].iter().enumerate() {
            let x = r.min.x + 4.0 + i as f32 * bw + 3.0;
            let hh = (r.height() - 8.0) * h * k;
            if hh <= 0.0 {
                continue;
            }
            let top = r.max.y - 4.0 - hh;
            self.p.rect_filled(
                Rect::from_min_max(pos2(x, top), pos2(x + bw - 6.0, r.max.y - 4.0)),
                1.0,
                if i % 2 == 0 { self.t.accent } else { self.t.accent.linear_multiply(0.5) },
            );
        }
        if k >= 1.0 {
            self.label(pos2(r.max.x - 4.0, r.min.y + 6.0), "١٢٣", 6.5, self.grey(), Align2::RIGHT_CENTER);
        }
    }
    /// A thumbnail of another slide sits on this slide and zooms up to fill it.
    pub(super) fn in_zoom(&self) {
        let a = self.hm_slide_center();
        self.hm_title(a, self.ink());
        self.text_lines(Rect::from_min_max(pos2(a.max.x - 48.0, a.min.y + 10.0), a.max), true, &[44.0, 34.0, 40.0], 8.0, self.grey());
        let small = Rect::from_min_size(pos2(a.min.x + 2.0, a.min.y + 12.0), vec2(48.0, 27.0));
        let big = Rect::from_min_size(self.at(20.0, 14.0), vec2(136.0, 76.0));
        let k = self.reveal(0.3, 0.8);
        let r = Rect::from_min_max(small.min + (big.min - small.min) * k, small.max + (big.max - small.max) * k);
        self.p.rect_filled(r, 1.5, Color32::WHITE);
        let inner = r.shrink2(vec2(5.0, 4.0));
        self.hm_title(inner, self.accent());
        self.picture(Rect::from_min_max(pos2(inner.min.x, inner.min.y + 7.0), pos2(inner.min.x + inner.width() * 0.45, inner.max.y)));
        self.text_lines(
            Rect::from_min_max(pos2(inner.max.x - inner.width() * 0.45, inner.min.y + 7.0), inner.max),
            true,
            &[inner.width() * 0.4, inner.width() * 0.3],
            6.0,
            self.grey(),
        );
        self.p.rect_stroke(r, 1.5, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
        if k <= 0.0 {
            self.magnifier(r.max - vec2(4.0, 4.0));
        }
    }
    /// A word becomes a link and an arrow leads to a little globe.
    pub(super) fn in_link(&self) {
        let a = self.hm_slide_center();
        self.hm_title(a, self.ink());
        let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 8.0), a.max);
        self.hm_body(a, 3, 9.0, self.grey());
        let y = body.min.y + 4.0 + 9.0;
        let word = Rect::from_min_max(pos2(a.max.x - 50.0, y - 2.5), pos2(a.max.x - 32.0, y + 2.5));
        let k = self.reveal(0.1, 0.4);
        let c = if k > 0.0 { self.accent() } else { self.grey() };
        self.p.rect_filled(word, 1.0, c);
        if k > 0.0 {
            self.line(pos2(word.min.x, y + 4.0), pos2(word.min.x + word.width() * k, y + 4.0), self.accent(), 1.2);
            self.in_chain(pos2(word.min.x - 10.0, y));
        }
        let globe = pos2(a.min.x + 20.0, a.max.y - 14.0);
        let g = self.reveal(0.4, 0.8);
        if g > 0.0 {
            let from = pos2(word.min.x - 18.0, y + 2.0);
            self.arrow_down(from, from + (globe + vec2(0.0, -12.0) - from) * g);
        }
        self.p.circle_filled(globe, 10.0, Color32::from_rgb(214, 232, 246));
        self.p.circle_stroke(globe, 10.0, Stroke::new(1.2, self.ink()));
        self.hm_ellipse(Rect::from_center_size(globe, vec2(9.0, 20.0)), Color32::TRANSPARENT, Stroke::new(1.0, self.ink()));
        self.line(globe + vec2(-10.0, 0.0), globe + vec2(10.0, 0.0), self.ink(), 1.0);
    }
    /// A button shape on the left slide is clicked and the show jumps to the slide on the right.
    pub(super) fn in_action(&self) {
        let b = self.hm_slide(6.0, 32.0, 70.0, 40.0);
        self.hm_title(b, self.ink());
        let btn = Rect::from_min_size(pos2(b.min.x + 2.0, b.max.y - 14.0), vec2(26.0, 12.0));
        self.shape(btn);
        self.label(btn.center(), "التالي", 6.5, Color32::WHITE, Align2::CENTER_CENTER);
        let k = self.reveal(0.0, 0.3);
        self.pointer(btn.center() + vec2(4.0, 2.0 - 6.0 * (1.0 - k)));
        let go = self.reveal(0.3, 0.55);
        if go > 0.0 {
            self.arrow_right(self.at(80.0, 52.0), self.at(80.0 + 16.0 * go, 52.0));
        }
        let t = self.reveal(0.55, 0.9);
        let a = self.hm_slide(100.0, 32.0, 70.0, 40.0);
        if t > 0.0 {
            self.hm_title(a, self.accent().linear_multiply(t));
            self.hm_body(a, 3, 7.0, self.grey().linear_multiply(t));
        }
        self.label(self.at(138.0, 24.0), "٣", 8.0, self.accent(), Align2::CENTER_CENTER);
        self.label(self.at(41.0, 24.0), "٢", 8.0, self.grey(), Align2::CENTER_CENTER);
    }
    /// A comment balloon pops out beside a line of the slide, and the pane on the right shows it.
    pub(super) fn in_comment(&self) {
        let a = self.hm_slide(4.0, 22.0, 100.0, 56.0);
        self.hm_title_body(a, 3);
        let k = self.reveal(0.1, 0.45);
        let anchor = pos2(a.max.x - 8.0, a.min.y + 22.0);
        if k > 0.0 {
            let r = Rect::from_center_size(anchor + vec2(0.0, -10.0 * k), vec2(16.0 * k, 11.0 * k));
            self.p.rect_filled(r, 2.0, self.accent());
            self.p.add(Shape::convex_polygon(
                vec![pos2(r.center().x - 3.0, r.max.y), pos2(r.center().x + 3.0, r.max.y), pos2(r.center().x, r.max.y + 4.0 * k)],
                self.accent(),
                Stroke::NONE,
            ));
            self.line(pos2(r.min.x + 3.0, r.center().y - 1.5), pos2(r.max.x - 3.0, r.center().y - 1.5), Color32::WHITE, 1.0);
            self.line(pos2(r.min.x + 3.0, r.center().y + 1.5), pos2(r.max.x - 6.0, r.center().y + 1.5), Color32::WHITE, 1.0);
        }
        let pane = self.pane(true, 60.0, 0);
        let t = self.reveal(0.45, 0.8);
        if t > 0.0 {
            let card = Rect::from_min_size(pos2(pane.min.x + 5.0, pane.min.y + 18.0), vec2(50.0, 32.0));
            self.p.rect_filled(card, 2.0, Color32::WHITE.linear_multiply(t));
            self.p.rect_stroke(card, 2.0, self.stroke(self.t.border.linear_multiply(t)), egui::StrokeKind::Inside);
            self.p.circle_filled(pos2(card.min.x + 7.0, card.min.y + 7.0), 4.0, self.accent().linear_multiply(t));
            self.line(pos2(card.min.x + 14.0, card.min.y + 7.0), pos2(card.max.x - 4.0, card.min.y + 7.0), self.ink().linear_multiply(t), 1.5);
            self.text_lines(
                Rect::from_min_max(pos2(card.min.x + 4.0, card.min.y + 12.0), card.max),
                true,
                &[40.0 * t, 28.0 * t],
                6.0,
                self.grey().linear_multiply(t),
            );
        }
    }
    /// A text box under a chart, typed in.
    pub(super) fn in_text_box(&self) {
        let a = self.hm_slide_center();
        self.hm_title(a, self.ink());
        self.chart(Rect::from_min_max(pos2(a.min.x + 10.0, a.min.y + 9.0), pos2(a.max.x - 10.0, a.max.y - 16.0)));
        let k = self.reveal(0.1, 0.45);
        let full = Rect::from_min_max(pos2(a.min.x + 10.0, a.max.y - 13.0), pos2(a.max.x - 10.0, a.max.y));
        if k > 0.0 {
            let r = Rect::from_min_size(
                pos2(full.max.x - full.width() * (0.15 + 0.85 * k), full.min.y),
                vec2(full.width() * (0.15 + 0.85 * k), full.height() * (0.3 + 0.7 * k)),
            );
            self.p.rect_stroke(r, 0.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
            let c = r.left_bottom();
            self.line(c + vec2(-5.0, 0.0), c + vec2(5.0, 0.0), self.ink(), 1.2);
            self.line(c + vec2(0.0, -5.0), c + vec2(0.0, 5.0), self.ink(), 1.2);
        }
        let t = self.reveal(0.5, 0.9);
        if t > 0.0 {
            let inner = full.shrink2(vec2(4.0, 2.0));
            self.line(pos2(inner.max.x - inner.width() * 0.7 * t, inner.center().y), pos2(inner.max.x, inner.center().y), self.accent(), 1.5);
            if t >= 1.0 {
                self.caret(pos2(inner.max.x - inner.width() * 0.7 - 2.0, inner.center().y - 4.0), 8.0);
            }
        }
    }
    /// The footer strip fills in: a date, a footer line and the slide number.
    pub(super) fn in_header_footer(&self) {
        let a = self.hm_slide_center();
        self.hm_title_body(a, 3);
        let foot = Rect::from_min_max(pos2(a.min.x, a.max.y - 8.0), a.max);
        self.dashed_rect(foot, self.t.border_strong);
        let k = self.reveal(0.1, 0.6);
        if k > 0.0 {
            let c = self.accent().linear_multiply(k);
            self.label(pos2(foot.min.x + 3.0, foot.center().y), &Draw::hm_ar("٢٠٢٦/١٠/١٨"), 5.5, c, Align2::LEFT_CENTER);
            self.line(pos2(foot.center().x - 14.0, foot.center().y), pos2(foot.center().x + 14.0, foot.center().y), c, 1.5);
            self.label(pos2(foot.max.x - 3.0, foot.center().y), "٣", 6.5, c, Align2::RIGHT_CENTER);
        }
    }
    /// A big word with an outline and a shadow grows onto the slide.
    pub(super) fn in_wordart(&self) {
        let a = self.hm_slide_center();
        self.text_lines(Rect::from_min_max(pos2(a.min.x, a.max.y - 16.0), a.max), true, &[a.width() * 0.5, a.width() * 0.35], 7.0, self.grey());
        let k = self.reveal(0.1, 0.6);
        let size = 10.0 + 16.0 * k;
        let c = pos2(a.center().x, a.min.y + 22.0);
        self.glyph(c + vec2(2.5, 2.5), "مرحباً", size, self.grey().linear_multiply(0.5));
        for off in [vec2(-1.2, 0.0), vec2(1.2, 0.0), vec2(0.0, -1.2), vec2(0.0, 1.2)] {
            self.glyph(c + off, "مرحباً", size, self.accent());
        }
        self.glyph(c, "مرحباً", size, Color32::WHITE);
    }
    /// A calendar beside a text box where today's date appears.
    pub(super) fn in_datetime(&self) {
        let a = self.hm_slide_center();
        self.hm_title_body(a, 2);
        self.in_calendar(pos2(a.min.x + 14.0, a.max.y - 14.0));
        let field = Rect::from_min_size(pos2(a.min.x + 28.0, a.max.y - 20.0), vec2(50.0, 12.0));
        self.p.rect_stroke(field, 0.0, Stroke::new(1.2, self.accent()), egui::StrokeKind::Inside);
        let k = self.reveal(0.2, 0.6);
        if k > 0.0 {
            self.label(
                pos2(field.max.x - 3.0, field.center().y),
                &Draw::hm_ar("٢٠٢٦/١٠/١٨"),
                7.0,
                self.accent().linear_multiply(k),
                Align2::RIGHT_CENTER,
            );
        } else {
            self.caret(pos2(field.max.x - 3.0, field.min.y + 2.0), 8.0);
        }
        let go = self.reveal(0.0, 0.3);
        if go > 0.0 {
            self.arrow_right(pos2(a.min.x + 22.0, field.center().y), pos2(a.min.x + 22.0 + 5.0 * go, field.center().y));
        }
    }
    /// The slide list on the left shows which slide this is; its number lands in the corner.
    pub(super) fn in_slide_number(&self) {
        let strip = self.hm_strip(4.0, 8.0, 30.0, 3);
        if let Some(r) = strip.get(2) {
            self.p.rect_stroke(*r, 1.5, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
        }
        for (i, r) in strip.iter().enumerate() {
            let digit = ["١", "٢", "٣"].get(i).copied().unwrap_or("٠");
            self.label(pos2(r.max.x + 5.0, r.center().y), digit, 6.5, if i == 2 { self.accent() } else { self.grey() }, Align2::CENTER_CENTER);
        }
        let a = self.hm_slide(50.0, 18.0, 118.0, 66.0);
        self.hm_title_body(a, 3);
        let k = self.reveal(0.15, 0.6);
        if k > 0.0 {
            let c = pos2(a.max.x - 5.0, a.max.y - 3.0 + 10.0 * (1.0 - k));
            self.label(c, "٣", 8.0, self.accent().linear_multiply(k), Align2::CENTER_CENTER);
        }
    }
    /// An equation box: x = a fraction, its parts appearing one by one.
    pub(super) fn in_equation(&self) {
        let a = self.hm_slide_center();
        self.hm_title(a, self.ink());
        let r = Rect::from_center_size(pos2(a.center().x, a.center().y + 6.0), vec2(70.0, 32.0));
        self.dashed_rect(r, self.accent());
        let c = r.center();
        self.label(c + vec2(-18.0, 0.0), "x =", 11.0, self.ink(), Align2::CENTER_CENTER);
        let k1 = self.reveal(0.1, 0.4);
        if k1 > 0.0 {
            self.label(c + vec2(8.0, -8.0), "a + b", 9.0, self.accent().linear_multiply(k1), Align2::CENTER_CENTER);
        }
        let k2 = self.reveal(0.35, 0.6);
        if k2 > 0.0 {
            self.line(c + vec2(-6.0, 0.0), c + vec2(-6.0 + 28.0 * k2, 0.0), self.ink(), 1.5);
        }
        let k3 = self.reveal(0.55, 0.85);
        if k3 > 0.0 {
            self.label(c + vec2(8.0, 8.0), "٢", 9.0, self.accent().linear_multiply(k3), Align2::CENTER_CENTER);
        }
    }
    /// A table of symbols; the chosen one goes to the end of a line on the slide.
    pub(super) fn in_symbol(&self) {
        let tray = self.in_tray(4.0, 20.0, 60.0, 50.0);
        let g = tray.shrink(4.0);
        self.grid(g, 3, 4, self.t.border);
        let syms = ["©", "±", "→", "Ω", "°", "√", "≠", "∞", "€", "µ", "½", "¶"];
        for (i, s) in syms.iter().enumerate() {
            let cell = self.cell(g, 3, 4, i / 4, i % 4);
            if i == 4 {
                self.p.rect_filled(cell.shrink(0.5), 0.0, self.t.checked);
            }
            self.label(cell.center(), s, 7.5, if i == 4 { self.accent() } else { self.ink() }, Align2::CENTER_CENTER);
        }
        let go = self.reveal(0.1, 0.35);
        if go > 0.0 {
            self.arrow_right(self.at(68.0, 46.0), self.at(68.0 + 14.0 * go, 46.0));
        }
        let a = self.hm_slide(86.0, 22.0, 84.0, 48.0);
        self.hm_title(a, self.ink());
        let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 8.0), a.max);
        let y = body.min.y + 4.0;
        self.label(pos2(body.max.x, y), "٢٥", 8.0, self.ink(), Align2::RIGHT_CENTER);
        let k = self.reveal(0.4, 0.7);
        if k > 0.0 {
            self.label(pos2(body.max.x - 12.0, y - 2.0), "°", 9.0, self.accent().linear_multiply(k), Align2::RIGHT_CENTER);
        } else {
            self.caret(pos2(body.max.x - 12.0, y - 4.0), 8.0);
        }
        self.text_lines(
            Rect::from_min_max(pos2(body.min.x, body.min.y + 7.0), body.max),
            true,
            &[body.width() * 0.7, body.width() * 0.5],
            7.0,
            self.grey(),
        );
    }
    /// A video frame on the slide: a play button and a progress bar that fills.
    pub(super) fn in_video(&self) {
        let a = self.hm_slide_center();
        self.hm_title(a, self.ink());
        let r = Rect::from_min_max(pos2(a.min.x + 14.0, a.min.y + 9.0), pos2(a.max.x - 14.0, a.max.y));
        self.p.rect_filled(r, 2.0, Color32::from_rgb(40, 44, 52));
        self.picture(Rect::from_min_max(pos2(r.min.x + 10.0, r.min.y + 4.0), pos2(r.max.x - 10.0, r.max.y - 12.0)));
        let c = r.center() - vec2(0.0, 4.0);
        self.p.circle_filled(c, 9.0, Color32::WHITE.linear_multiply(0.85));
        self.p.add(Shape::convex_polygon(vec![c + vec2(-3.0, -4.5), c + vec2(5.0, 0.0), c + vec2(-3.0, 4.5)], self.accent(), Stroke::NONE));
        let bar = Rect::from_min_max(pos2(r.min.x + 6.0, r.max.y - 7.0), pos2(r.max.x - 6.0, r.max.y - 4.0));
        self.p.rect_filled(bar, 1.5, Color32::from_gray(110));
        let k = self.reveal(0.0, 1.0);
        self.p.rect_filled(Rect::from_min_size(bar.min, vec2(bar.width() * k, bar.height())), 1.5, self.accent());
        self.p.circle_filled(pos2(bar.min.x + bar.width() * k, bar.center().y), 2.5, Color32::WHITE);
    }
    /// A speaker on the slide with sound waves and a little play bar.
    pub(super) fn in_audio(&self) {
        let a = self.hm_slide_center();
        self.hm_title_body(a, 2);
        let c = pos2(a.center().x, a.center().y + 8.0);
        self.p.circle_filled(c, 16.0, self.t.hover);
        self.speaker(c - vec2(2.0, 0.0));
        let k = self.reveal(0.0, 1.0);
        let bar = Rect::from_min_max(pos2(c.x - 30.0, c.y + 24.0), pos2(c.x + 30.0, c.y + 27.0));
        self.p.rect_filled(bar, 1.5, self.t.border_strong);
        self.p.rect_filled(Rect::from_min_size(bar.min, vec2(bar.width() * k, bar.height())), 1.5, self.accent());
        let tri = pos2(bar.min.x - 8.0, bar.center().y);
        self.p.add(Shape::convex_polygon(vec![tri + vec2(-2.5, -3.5), tri + vec2(3.0, 0.0), tri + vec2(-2.5, 3.5)], self.ink(), Stroke::NONE));
        // Sound bars beside the speaker rise and fall with the film.
        for i in 0..3 {
            let h = 4.0 + 6.0 * ((self.phase * std::f32::consts::TAU * 2.0 + i as f32).sin() * 0.5 + 0.5);
            let x = c.x + 20.0 + i as f32 * 4.0;
            self.line(pos2(x, c.y - h), pos2(x, c.y + h), self.accent(), 2.0);
        }
    }

    // ---- Draw ----

    /// A dashed loop closes around two of three ink strokes, then handles appear.
    pub(super) fn in_lasso(&self) {
        let a = self.hm_slide_center();
        self.hm_title(a, self.ink());
        let c = pos2(a.min.x + 42.0, a.center().y + 8.0);
        self.in_ink(c + vec2(-16.0, -8.0), c + vec2(16.0, -4.0), 1.5, self.red(), 1.0);
        self.in_ink(c + vec2(-14.0, 6.0), c + vec2(14.0, 10.0), 1.5, self.accent(), 1.0);
        self.in_ink(pos2(a.max.x - 36.0, a.max.y - 14.0), pos2(a.max.x - 6.0, a.max.y - 10.0), 1.5, self.green(), 1.0);
        let k = self.reveal(0.05, 0.55);
        self.in_lasso_loop(c, 26.0, 18.0, k);
        let ang = k * std::f32::consts::TAU;
        self.pointer(pos2(c.x + 26.0 * ang.cos(), c.y + 18.0 * ang.sin()));
        if self.reveal(0.6, 0.8) > 0.0 {
            let r = Rect::from_center_size(c + vec2(0.0, 1.0), vec2(40.0, 24.0));
            self.p.rect_stroke(r, 0.0, Stroke::new(1.0, self.accent()), egui::StrokeKind::Inside);
            self.handles(r);
        }
    }
    /// An eraser moves along an ink stroke and the part behind it is gone.
    pub(super) fn in_eraser(&self) {
        let a = self.hm_slide_center();
        self.hm_title(a, self.ink());
        let (s, e) = (pos2(a.min.x + 10.0, a.center().y + 10.0), pos2(a.max.x - 10.0, a.center().y + 2.0));
        self.in_ink(s, e, 2.0, self.red(), 1.0);
        let k = self.reveal(0.05, 0.8);
        // The erased part: painted over in white along the stroke.
        if k > 0.0 {
            let gone = Rect::from_min_max(pos2(s.x - 4.0, s.y - 14.0), pos2(s.x + (e.x - s.x) * k, e.y + 14.0));
            self.p.rect_filled(gone, 0.0, Color32::WHITE);
        }
        let c = s + (e - s) * k;
        let pts = vec![c + vec2(-9.0, 2.0), c + vec2(-1.0, -7.0), c + vec2(9.0, 3.0), c + vec2(1.0, 12.0)];
        self.p.add(Shape::convex_polygon(pts.clone(), Color32::from_rgb(240, 160, 170), Stroke::NONE));
        self.p.add(Shape::closed_line(pts, Stroke::new(1.2, self.ink())));
        self.line(c + vec2(-5.0, -2.0), c + vec2(5.0, 8.0), self.ink(), 1.2);
    }
    /// A wobbly ring becomes a clean ellipse.
    pub(super) fn in_ink_to_shape(&self) {
        self.hm_pair(
            |d, a| {
                d.hm_title_body(a, 2);
                d.in_ink_ring(a.center() + vec2(0.0, 6.0), 9.0, d.red(), 1.0);
            },
            |d, a| {
                d.hm_title_body(a, 2);
                d.hm_ellipse(
                    Rect::from_center_size(a.center() + vec2(0.0, 6.0), vec2(24.0, 18.0)),
                    Color32::TRANSPARENT,
                    Stroke::new(2.0, d.accent()),
                );
            },
        );
    }
    /// Handwritten squiggles become typed lines.
    pub(super) fn in_ink_to_text(&self) {
        self.hm_pair(
            |d, a| {
                d.hm_title(a, d.ink());
                for (i, w) in [44.0, 34.0, 40.0].iter().enumerate() {
                    let y = a.min.y + 13.0 + i as f32 * 8.0;
                    d.in_ink(pos2(a.max.x - w, y), pos2(a.max.x, y), 2.0, d.red(), 1.0);
                }
            },
            |d, a| {
                d.hm_title(a, d.ink());
                let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 6.0), a.max);
                d.p.rect_stroke(
                    Rect::from_min_max(pos2(a.max.x - 50.0, body.min.y), pos2(a.max.x + 1.0, body.min.y + 25.0)),
                    0.0,
                    Stroke::new(1.0, d.accent()),
                    egui::StrokeKind::Inside,
                );
                d.text_lines(body.shrink2(vec2(1.0, 0.0)), true, &[42.0, 32.0, 38.0], 8.0, d.accent());
            },
        );
    }
    /// A handwritten formula becomes a typed equation.
    pub(super) fn in_ink_to_math(&self) {
        self.hm_pair(
            |d, a| {
                d.hm_title(a, d.ink());
                let c = a.center() + vec2(0.0, 6.0);
                d.in_ink(c + vec2(-22.0, 2.0), c + vec2(-8.0, -3.0), 2.5, d.red(), 1.0);
                d.in_ink(c + vec2(-4.0, -4.0), c + vec2(-4.0, 4.0), 1.0, d.red(), 1.0);
                d.in_ink(c + vec2(-8.0, 0.0), c + vec2(0.0, 0.0), 0.8, d.red(), 1.0);
                d.in_ink(c + vec2(4.0, 3.0), c + vec2(20.0, -2.0), 2.5, d.red(), 1.0);
            },
            |d, a| {
                d.hm_title(a, d.ink());
                let r = Rect::from_center_size(a.center() + vec2(0.0, 6.0), vec2(46.0, 16.0));
                d.dashed_rect(r, d.accent());
                d.label(r.center(), "x² + y²", 9.0, d.accent(), Align2::CENTER_CENTER);
            },
        );
    }
}
