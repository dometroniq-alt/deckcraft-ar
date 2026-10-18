//! Review tab thumbnails: spelling, thesaurus, accessibility, translate, language, compare
//! (accept all, end review) and comments (new, show). A slide with the mark the tool makes —
//! a red squiggle, a warning, a speech bubble — and the pane or box that comes with it.

#![allow(unused_imports)]

use egui::{Align2, Color32, Pos2, Rect, Shape, Stroke, pos2, vec2};

use super::Entry;
use super::design::{DS_BLUE, DS_ORANGE, DS_TEAL};
use super::draw::{Dir, Draw, SIZE};

pub const ENTRIES: &[Entry] = &[
    Entry::new(
        "Spelling",
        "Checks the words on every slide against the dictionary and stops at each one it does not know, offering corrections to accept or ignore. Example: «الاجتمااع» is flagged and the box offers «الاجتماع».",
        |d| d.rv_spelling(),
    ),
    Entry::new(
        "Thesaurus",
        "Opens a side pane with words that mean the same as the selected word, so you can swap it for a better one with a click. Example: select «كبير» and the pane offers «ضخم», «واسع», «عظيم».",
        |d| d.rv_thesaurus(),
    ),
    Entry::new(
        "Check\nAccessibility",
        "Looks through the deck for things people with weak sight or a screen reader would struggle with: pictures without a description, faint text, unnamed objects, and lists them to fix. Example: it reports the photo on slide 2 has no description.",
        |d| d.rv_accessibility(),
    ),
    Entry::new(
        "Translate",
        "Translates the selected text into another language, showing the result beside the original; this needs a language model that is not bundled, so it is not available yet. Example: a slide title in Arabic shown in English next to it.",
        |d| d.rv_translate(),
    ),
    Entry::new(
        "Language",
        "Sets which language the selected text is in, so the spelling check uses the right dictionary and the text runs the right way. Example: mark an English quotation as English so Arabic spelling does not flag every word.",
        |d| d.rv_language(),
    ),
    Entry::new(
        "Compare",
        "Opens another copy of this presentation and shows every difference between the two, slide by slide, as changes you can accept or reject. Example: see which boxes your colleague moved in the copy she sent back.",
        |d| d.rv_compare(),
    ),
    Entry::new(
        "Accept\nAll",
        "Takes every remaining change from the compared copy into this presentation at once, instead of accepting them one by one. Example: all 12 edits from the reviewed copy go in with one click.",
        |d| d.rv_accept_all(),
    ),
    Entry::new(
        "End\nReview",
        "Closes the comparison: the changes you accepted stay, the rest are dropped, and the revisions pane goes away. Example: after accepting the three edits you wanted, end the review and go on editing.",
        |d| d.rv_end_review(),
    ),
    Entry::new(
        "New\nComment",
        "Attaches a note to the slide, or to the selected object, for a colleague to read; it shows in the comments pane and never prints or plays in the show. Example: write «check this figure» on the chart for the finance clerk.",
        |d| d.rv_new_comment(),
    ),
    Entry::new(
        "Show\nComments",
        "Opens or closes the comments pane beside the slide, where every note on this slide is listed with its author and can be answered or removed. Example: open it to read what the manager wrote about slide 4.",
        |d| d.rv_show_comments(),
    ),
    Entry::new(
        "comment|Delete",
        "Removes the selected comment from the slide, with every reply under it; the slide itself is not changed. Example: once the figure is corrected, delete the «check this figure» note.",
        |d| d.rv_delete_comment(),
    ),
];

impl Draw<'_> {
    /// A red squiggle under a word from `a` to `b`.
    pub(super) fn rv_squiggle(&self, a: Pos2, b: Pos2, col: Color32) {
        let n = ((b.x - a.x) / 3.0).max(1.0) as usize;
        let pts: Vec<Pos2> = (0..=n).map(|i| pos2(a.x + i as f32 * 3.0, a.y + if i % 2 == 0 { 0.0 } else { 2.0 })).collect();
        self.p.add(Shape::line(pts, Stroke::new(1.2, col)));
    }
    /// A speech bubble (a comment) with its tail at `tip`, `w × h`, with `rows` text lines.
    pub(super) fn rv_bubble(&self, tip: Pos2, w: f32, h: f32, rows: usize, fill: Color32) {
        let body = Rect::from_min_size(pos2(tip.x - w + 8.0, tip.y - h - 6.0), vec2(w, h));
        self.p.rect_filled(body, 3.0, fill);
        self.p.rect_stroke(body, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let tail = vec![pos2(tip.x - 2.0, body.max.y - 1.0), tip, pos2(tip.x - 8.0, body.max.y - 1.0)];
        self.p.add(Shape::convex_polygon(tail, fill, Stroke::NONE));
        self.line(pos2(tip.x - 2.0, body.max.y), tip, self.t.border_strong, 1.0);
        self.line(tip, pos2(tip.x - 8.0, body.max.y), self.t.border_strong, 1.0);
        let area = Rect::from_min_max(body.min + vec2(4.0, 2.0), body.max - vec2(4.0, 2.0));
        let widths: Vec<f32> = (0..rows).map(|i| area.width() * if i + 1 == rows { 0.5 } else { 0.9 }).collect();
        self.text_lines(area, true, &widths, 6.0, self.ds_grey());
    }
    /// A comment row in a pane: an author circle, a name line and text lines.
    pub(super) fn rv_comment_row(&self, pane: Rect, y: f32, col: Color32) {
        self.p.circle_filled(pos2(pane.max.x - 10.0, y + 4.0), 4.0, col);
        self.line(pos2(pane.max.x - 17.0, y + 2.0), pos2(pane.max.x - 36.0, y + 2.0), self.ink(), 2.0);
        self.line(pos2(pane.max.x - 17.0, y + 7.0), pos2(pane.min.x + 6.0, y + 7.0), self.grey(), 1.2);
        self.line(pos2(pane.max.x - 6.0, y + 12.0), pos2(pane.min.x + 14.0, y + 12.0), self.grey(), 1.2);
    }

    fn rv_delete_comment(&self) {
        // A slide with a comment bubble that fades out and shrinks to nothing.
        let a = self.ds_slide(6.0, 20.0, 104.0, 60.0);
        self.ds_text(a, 3);
        let t = self.reveal(0.3, 0.8);
        if t < 1.0 {
            let k = 1.0 - t;
            let tip = pos2(a.max.x - 14.0, a.min.y + 30.0);
            let fill = DS_ORANGE.linear_multiply(0.25 + 0.75 * k);
            self.rv_bubble(tip, 40.0 * k.max(0.2), 22.0 * k.max(0.2), 2, fill);
        }
        let pane = Rect::from_min_size(self.at(118.0, 14.0), vec2(52.0, 72.0));
        self.p.rect_filled(pane, 3.0, self.t.menu);
        self.p.rect_stroke(pane, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        if t < 1.0 {
            self.rv_comment_row(pane, pane.min.y + 8.0, DS_ORANGE.linear_multiply(1.0 - t));
        }
        self.rv_comment_row(pane, pane.min.y + 28.0 - 20.0 * t, DS_BLUE);
        if t >= 1.0 {
            self.ds_label(pos2(pane.center().x, pane.max.y - 10.0), "١", 7.0, self.grey(), Align2::CENTER_CENTER);
        }
    }

    fn rv_spelling(&self) {
        let a = self.ds_slide(6.0, 20.0, 104.0, 60.0);
        self.ds_title(a, self.ds_ink());
        let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 8.0), a.max);
        self.text_lines(body, true, &[body.width() * 0.9, body.width() * 0.4, body.width() * 0.85], 9.0, self.ds_grey());
        // The flagged word on the second line, with its squiggle, then the corrected word.
        // The still shows the flagged word; the film then turns it green when the fix goes in.
        let fixed = if self.phase >= 1.0 { 0.0 } else { self.reveal(0.6, 0.8) };
        let wy = body.min.y + 4.0 + 9.0;
        let wx = body.max.x - body.width() * 0.4 - 4.0;
        let w = Rect::from_min_size(pos2(wx - 22.0, wy - 4.0), vec2(22.0, 7.0));
        self.p.rect_filled(w, 1.0, self.red().linear_multiply(0.15 * (1.0 - fixed)));
        self.line(pos2(w.min.x + 2.0, wy), pos2(w.max.x - 2.0, wy), self.ds_grey().lerp_to_gamma(self.green(), fixed), 1.5);
        if fixed < 1.0 {
            self.rv_squiggle(pos2(w.min.x + 2.0, wy + 3.0), pos2(w.max.x - 2.0, wy + 3.0), self.red().linear_multiply(1.0 - fixed));
        }
        // The pane: the word in red, the suggestions, one lit.
        let pane = self.pane(true, 58.0, 0);
        self.ds_label(pos2(pane.max.x - 6.0, pane.min.y + 22.0), "الاجتمااع", 6.5, self.red(), Align2::RIGHT_CENTER);
        let row = Rect::from_min_size(pos2(pane.min.x + 5.0, pane.min.y + 30.0), vec2(pane.width() - 10.0, 11.0));
        self.p.rect_filled(row, 2.0, self.t.checked);
        self.ds_label(pos2(pane.max.x - 7.0, row.center().y), "الاجتماع", 6.5, self.ink(), Align2::RIGHT_CENTER);
        self.line(pos2(pane.max.x - 7.0, pane.min.y + 50.0), pos2(pane.max.x - 36.0, pane.min.y + 50.0), self.grey(), 1.5);
        self.line(pos2(pane.max.x - 7.0, pane.min.y + 60.0), pos2(pane.max.x - 30.0, pane.min.y + 60.0), self.grey(), 1.5);
        self.ds_chip(pos2(pane.center().x, pane.max.y - 12.0), "تغيير", 36.0);
    }

    fn rv_thesaurus(&self) {
        let a = self.ds_slide(6.0, 20.0, 104.0, 60.0);
        self.ds_title(a, self.ds_ink());
        let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 8.0), a.max);
        self.text_lines(body, true, &[body.width() * 0.9, body.width() * 0.3, body.width() * 0.85], 9.0, self.ds_grey());
        let wy = body.min.y + 4.0 + 9.0;
        let wx = body.max.x - body.width() * 0.3 - 4.0;
        let w = Rect::from_min_size(pos2(wx - 24.0, wy - 5.0), vec2(24.0, 10.0));
        self.selection(w);
        self.ds_label(pos2(w.max.x - 2.0, wy), "كبير", 7.0, self.ds_ink(), Align2::RIGHT_CENTER);
        let pane = self.pane(true, 58.0, 0);
        self.ds_label(pos2(pane.max.x - 6.0, pane.min.y + 22.0), "كبير", 7.0, self.accent(), Align2::RIGHT_CENTER);
        // The synonyms appear one after another.
        for (i, s) in ["ضخم", "واسع", "عظيم"].iter().enumerate() {
            let t = self.reveal(0.2 + i as f32 * 0.2, 0.35 + i as f32 * 0.2);
            if t > 0.0 {
                let y = pane.min.y + 36.0 + i as f32 * 13.0;
                self.ds_label(pos2(pane.max.x - 10.0, y), s, 6.5, self.ink().linear_multiply(t.max(0.3)), Align2::RIGHT_CENTER);
                self.p.circle_filled(pos2(pane.max.x - 5.0, y), 1.2, self.grey());
            }
        }
    }

    fn rv_accessibility(&self) {
        let a = self.ds_slide(6.0, 20.0, 104.0, 60.0);
        self.ds_title(a, self.ds_ink());
        let pic = Rect::from_min_size(pos2(a.min.x + 2.0, a.min.y + 11.0), vec2(40.0, 26.0));
        self.picture(pic);
        self.ds_body(Rect::from_min_max(pos2(a.min.x + 46.0, a.min.y), a.max), 3, self.ds_grey());
        // The picture gets its warning when the check reaches it.
        let t = self.reveal(0.3, 0.5);
        if t > 0.0 {
            self.warning(pic.right_top() + vec2(-4.0, 4.0));
        }
        let pane = self.pane(true, 58.0, 0);
        for (i, warn) in [(0, true), (1, false), (2, true)] {
            let y = pane.min.y + 24.0 + i as f32 * 16.0;
            let shown = self.reveal(0.1 + i as f32 * 0.15, 0.2 + i as f32 * 0.15);
            if shown <= 0.0 {
                continue;
            }
            if warn {
                self.warning(pos2(pane.max.x - 10.0, y));
            } else {
                self.p.circle_filled(pos2(pane.max.x - 10.0, y), 5.0, self.green());
                self.line(pos2(pane.max.x - 12.5, y), pos2(pane.max.x - 10.5, y + 2.0), Color32::WHITE, 1.5);
                self.line(pos2(pane.max.x - 10.5, y + 2.0), pos2(pane.max.x - 7.0, y - 2.5), Color32::WHITE, 1.5);
            }
            self.line(pos2(pane.max.x - 20.0, y - 2.0), pos2(pane.min.x + 6.0, y - 2.0), self.ink(), 1.5);
            self.line(pos2(pane.max.x - 20.0, y + 3.0), pos2(pane.min.x + 14.0, y + 3.0), self.grey(), 1.2);
        }
    }

    fn rv_translate(&self) {
        self.ds_pair(
            |d, a| {
                d.ds_label(pos2(a.max.x, a.min.y + 8.0), "مرحبا", 10.0, d.ds_ink(), Align2::RIGHT_CENTER);
                d.ds_body(Rect::from_min_max(pos2(a.min.x, a.min.y + 10.0), a.max), 2, d.ds_grey());
            },
            |d, a| {
                d.ds_label(pos2(a.min.x, a.min.y + 8.0), "Hello", 10.0, DS_BLUE, Align2::LEFT_CENTER);
                let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 10.0), a.max);
                d.text_lines(body, false, &[body.width() * 0.8, body.width() * 0.5], 10.0, d.ds_grey());
            },
        );
        // Two speech bubbles above the arrow: the languages.
        self.rv_bubble(self.at(78.0, 26.0), 20.0, 12.0, 1, self.t.menu);
        self.rv_bubble(self.at(104.0, 26.0), 20.0, 12.0, 1, DS_BLUE.lerp_to_gamma(Color32::WHITE, 0.7));
    }

    fn rv_language(&self) {
        let inner = self.dialog("اللغة", 0);
        // A list of languages; the lit row moves to the chosen one as the film plays.
        let t = self.reveal(0.3, 0.6);
        let chosen = if t > 0.5 || self.phase >= 1.0 { 1 } else { 0 };
        for (i, name) in ["العربية", "English", "Français"].iter().enumerate() {
            let y = inner.min.y + 4.0 + i as f32 * 12.0;
            let row = Rect::from_min_size(pos2(inner.min.x + 2.0, y - 1.0), vec2(inner.width() - 4.0, 11.0));
            if i == chosen {
                self.p.rect_filled(row, 2.0, self.t.checked);
            }
            let anchor = if i == 0 { Align2::RIGHT_CENTER } else { Align2::LEFT_CENTER };
            let x = if i == 0 { row.max.x - 4.0 } else { row.min.x + 4.0 };
            self.ds_label(pos2(x, row.center().y), name, 7.0, self.ink(), anchor);
        }
        self.ds_chip(pos2(inner.center().x, inner.max.y - 6.0), "موافق", 36.0);
        // The checked letters on a slide corner behind the box: "abc" with a tick.
        self.check(self.at(150.0, 16.0));
    }

    fn rv_compare(&self) {
        // Two copies of the deck meet in the middle; the differences light up.
        let b = self.ds_slide(6.0, 26.0, 60.0, 34.0);
        self.ds_title(b, self.ds_ink());
        let box_b = Rect::from_min_size(pos2(b.max.x - 20.0, b.min.y + 10.0), vec2(18.0, 12.0));
        self.p.rect_filled(box_b, 1.0, DS_BLUE);
        self.ds_body(Rect::from_min_max(b.min, pos2(b.max.x - 24.0, b.max.y)), 2, self.ds_grey());
        let a = self.ds_slide(110.0, 26.0, 60.0, 34.0);
        self.ds_title(a, self.ds_ink());
        let box_a = Rect::from_min_size(pos2(a.min.x + 2.0, a.min.y + 12.0), vec2(18.0, 12.0));
        self.p.rect_filled(box_a, 1.0, DS_BLUE);
        self.ds_body(Rect::from_min_max(pos2(a.min.x + 24.0, a.min.y), a.max), 2, self.ds_grey());
        let t = self.reveal(0.2, 0.6);
        if t > 0.0 {
            self.p.rect_stroke(box_b.expand(2.0), 1.0, Stroke::new(1.5, DS_ORANGE), egui::StrokeKind::Outside);
            self.p.rect_stroke(box_a.expand(2.0), 1.0, Stroke::new(1.5, DS_ORANGE), egui::StrokeKind::Outside);
            // The move, as a dashed arc over the top.
            let (p0, p1) = (box_b.center_top(), box_a.center_top());
            let n = 10;
            for i in 0..n {
                let f0 = i as f32 / n as f32;
                let f1 = f0 + 0.5 / n as f32;
                let lift = |f: f32| vec2(0.0, -14.0 * (f * std::f32::consts::PI).sin());
                self.line(p0.lerp(p1, f0) + lift(f0), p0.lerp(p1, f1) + lift(f1), DS_ORANGE, 1.2);
            }
        }
        self.ds_label(self.at(36.0, 18.0), "الأصل", 6.5, self.grey(), Align2::CENTER_CENTER);
        self.ds_label(self.at(140.0, 18.0), "النسخة", 6.5, self.grey(), Align2::CENTER_CENTER);
        // Below: accept / reject.
        self.check(self.at(70.0, 82.0));
        self.cross(self.at(106.0, 82.0));
    }

    fn rv_accept_all(&self) {
        self.ds_pair(
            |d, a| {
                d.ds_title(a, d.ds_ink());
                let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 8.0), a.max);
                let w = body.width();
                d.text_lines(body, true, &[w * 0.9, w * 0.7, w * 0.8], 7.0, d.ds_grey());
                for row in [0, 2] {
                    let y = body.min.y + 4.0 + row as f32 * 7.0;
                    d.line(pos2(body.max.x, y), pos2(body.max.x - w * if row == 0 { 0.9 } else { 0.8 }, y), DS_ORANGE, 1.5);
                    d.line(pos2(a.min.x - 3.0, y - 3.0), pos2(a.min.x - 3.0, y + 3.0), DS_ORANGE, 2.0);
                }
            },
            |d, a| {
                d.ds_text(a, 3);
            },
        );
        self.check(self.at(88.0, 24.0));
        self.ds_label(self.at(88.0, 86.0), "الكل", 7.0, self.accent(), Align2::CENTER_CENTER);
    }

    fn rv_end_review(&self) {
        // The revisions pane slides out to the right and the slide has the room again.
        let t = self.reveal(0.2, 0.7);
        let pane_w = 56.0;
        let clip = self.p.with_clip_rect(Rect::from_min_size(self.at(0.0, 0.0), SIZE).intersect(self.p.clip_rect()));
        let d = self.with_painter(&clip);
        let slide_w = 104.0 + 60.0 * t;
        let a = d.ds_slide(6.0, 52.0 - slide_w * 9.0 / 32.0, slide_w, slide_w * 9.0 / 16.0);
        d.ds_text(a, 3);
        let x = SIZE.x - pane_w - 4.0 + (pane_w + 8.0) * t;
        let r = Rect::from_min_size(d.at(x, 4.0), vec2(pane_w, SIZE.y - 8.0));
        d.p.rect_filled(r, 2.0, d.t.panel);
        d.p.rect_stroke(r, 2.0, d.stroke(d.t.border_strong), egui::StrokeKind::Inside);
        d.p.rect_filled(Rect::from_min_size(r.min, vec2(pane_w, 12.0)), 2.0, d.t.hover);
        d.line(pos2(r.max.x - 4.0, r.min.y + 6.0), pos2(r.max.x - pane_w * 0.6, r.min.y + 6.0), d.t.text, 2.0);
        for i in 0..3 {
            let y = r.min.y + 24.0 + i as f32 * 14.0;
            d.line(pos2(r.max.x - 6.0, y), pos2(r.min.x + 6.0, y), DS_ORANGE, 1.5);
        }
        if t > 0.0 {
            self.cross(self.at(150.0, 20.0));
        }
    }

    fn rv_new_comment(&self) {
        let a = self.ds_slide_center();
        self.ds_title(a, self.ds_ink());
        let chart = Rect::from_min_size(pos2(a.min.x + 2.0, a.min.y + 12.0), vec2(50.0, 32.0));
        self.chart(chart);
        self.ds_body(Rect::from_min_max(pos2(a.min.x + 56.0, a.min.y), a.max), 3, self.ds_grey());
        // The bubble pops out of the chart.
        let t = self.reveal(0.2, 0.5);
        if t > 0.0 {
            let tip = chart.right_top() + vec2(-6.0, 6.0);
            self.rv_bubble(tip, 44.0 * t.max(0.4), 22.0 * t.max(0.4), 2, Color32::from_rgb(255, 246, 200));
        }
        self.pointer(chart.right_top() + vec2(-2.0, 10.0));
    }

    fn rv_show_comments(&self) {
        let a = self.ds_slide(6.0, 20.0, 104.0, 60.0);
        self.ds_text(a, 3);
        let marker = a.right_top() + vec2(2.0, -2.0);
        self.p.circle_filled(marker, 5.0, Color32::from_rgb(255, 246, 200));
        self.p.circle_stroke(marker, 5.0, self.stroke(self.t.border_strong));
        // The pane slides in from the right.
        let t = self.reveal(0.1, 0.5);
        let w = 58.0;
        let clip = self.p.with_clip_rect(Rect::from_min_size(self.at(0.0, 0.0), SIZE).intersect(self.p.clip_rect()));
        let d = self.with_painter(&clip);
        let x = SIZE.x - w - 4.0 + (w + 8.0) * (1.0 - t);
        let r = Rect::from_min_size(d.at(x, 4.0), vec2(w, SIZE.y - 8.0));
        d.p.rect_filled(r, 2.0, d.t.panel);
        d.p.rect_stroke(r, 2.0, d.stroke(d.t.border_strong), egui::StrokeKind::Inside);
        d.p.rect_filled(Rect::from_min_size(r.min, vec2(w, 12.0)), 2.0, d.t.hover);
        d.line(pos2(r.max.x - 4.0, r.min.y + 6.0), pos2(r.max.x - w * 0.6, r.min.y + 6.0), d.t.text, 2.0);
        d.rv_comment_row(r, r.min.y + 18.0, DS_TEAL);
        d.rv_comment_row(r, r.min.y + 40.0, DS_ORANGE);
        d.rv_comment_row(r, r.min.y + 62.0, DS_BLUE);
    }
}
