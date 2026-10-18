//! Transitions tab thumbnails: preview, effect options and apply-to-all. Transitions are drawn
//! as one slide giving way to the next, so the films here move: a slide pushes in, a wipe runs
//! in the chosen direction, a mark copies from slide to slide.

#![allow(unused_imports)]

use egui::{Align2, Color32, Pos2, Rect, Shape, Stroke, pos2, vec2};

use super::Entry;
use super::design::{DS_BLUE, DS_ORANGE};
use super::draw::{Dir, Draw, SIZE};

pub const ENTRIES: &[Entry] = &[
    Entry::new(
        "Preview",
        "Plays this slide's transition (or, on the Animations tab, its animations) right here on the canvas, so you can see it without starting the whole show. Example: press it after choosing Push to watch the slide slide in.",
        |d| d.tr_preview(),
    ),
    Entry::new(
        "Effect\nOptions",
        "Sets the details of the chosen transition: which direction it comes from, whether it runs smoothly or in pieces. Example: a Wipe from the left becomes a Wipe from the top.",
        |d| d.tr_effect_options(),
    ),
    Entry::new(
        "Apply\nTo All",
        "Copies this slide's transition, its sound and its timing to every slide of the presentation, so the whole show moves the same way. Example: set Fade on slide 1, press it, and slides 2 to 20 fade too.",
        |d| d.tr_apply_to_all(),
    ),
];

impl Draw<'_> {
    /// The slide-in film: inside the frame at `x, y`, slide A is pushed out to the left by
    /// slide B coming in from the right, by `t` (0 = only A, 1 = only B).
    pub(super) fn tr_push(&self, x: f32, y: f32, w: f32, h: f32, t: f32) {
        let outer = self.ds_slide_rect(x, y, w, h);
        let clip = self.p.with_clip_rect(outer.intersect(self.p.clip_rect()));
        let d = self.with_painter(&clip);
        let shift = w * t;
        // Slide A: a title and body.
        let a = d.ds_slide(x - shift, y, w, h);
        d.ds_text(a, 3);
        // Slide B: a picture slide, in another colour so the eye tells the two apart.
        let b = d.ds_slide(x + w - shift, y, w, h);
        d.ds_title(b, DS_BLUE);
        d.picture(Rect::from_min_max(pos2(b.min.x, b.min.y + 9.0), b.max));
        self.p.rect_stroke(outer, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
    }
    /// A play triangle centred at `c`.
    pub(super) fn tr_play(&self, c: Pos2, r: f32, col: Color32) {
        let pts = vec![c + vec2(-r * 0.8, -r), c + vec2(r, 0.0), c + vec2(-r * 0.8, r)];
        self.p.add(Shape::convex_polygon(pts, col, Stroke::NONE));
    }
    /// A transition mark (a small accent star of four points) centred at `c`.
    pub(super) fn tr_mark(&self, c: Pos2, col: Color32) {
        for (dx, dy) in [(1.0, 0.0), (0.0, 1.0)] {
            self.line(c - vec2(dx * 4.0, dy * 4.0), c + vec2(dx * 4.0, dy * 4.0), col, 1.5);
        }
        for (dx, dy) in [(1.0, 1.0), (1.0, -1.0)] {
            self.line(c - vec2(dx * 2.6, dy * 2.6), c + vec2(dx * 2.6, dy * 2.6), col, 1.5);
        }
    }

    fn tr_preview(&self) {
        // The push runs across the middle of the film; before it starts the play button pulses.
        let t = self.reveal(0.25, 0.8);
        self.tr_push(24.0, 20.0, 128.0, 72.0, t);
        let c = self.at(88.0, 56.0);
        self.p.circle_filled(c, 12.0, Color32::from_black_alpha(120));
        self.tr_play(c + vec2(1.5, 0.0), 6.0, Color32::WHITE);
    }

    fn tr_effect_options(&self) {
        // Two mini slides, each a wipe from another side; the second one is the choice made.
        let t = self.reveal(0.2, 0.7);
        for (k, from_top) in [(0usize, false), (1, true)] {
            let x = 10.0 + k as f32 * 86.0;
            let outer = self.ds_slide_rect(x, 26.0, 70.0, 40.0);
            let a = self.ds_slide(x, 26.0, 70.0, 40.0);
            self.ds_text(a, 3);
            // The wipe: the new slide (tinted) covers the old from the left or from the top.
            let cover = if from_top {
                Rect::from_min_max(outer.min, pos2(outer.max.x, outer.min.y + outer.height() * t))
            } else {
                Rect::from_min_max(outer.min, pos2(outer.min.x + outer.width() * t, outer.max.y))
            };
            self.p.rect_filled(cover.shrink(1.0), 1.0, DS_BLUE.lerp_to_gamma(Color32::WHITE, 0.6));
            self.p.rect_stroke(outer, 2.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
            if from_top {
                self.arrow_down(pos2(outer.center().x, outer.min.y - 12.0), pos2(outer.center().x, outer.min.y - 3.0));
            } else {
                self.arrow_right(pos2(outer.min.x - 12.0, outer.center().y), pos2(outer.min.x - 3.0, outer.center().y));
            }
            // The chosen option is ticked below its slide.
            let label = if from_top { "من الأعلى" } else { "من اليسار" };
            self.ds_chip(pos2(outer.center().x, outer.max.y + 14.0), label, 52.0);
            if k == 1 {
                self.p.rect_stroke(
                    Rect::from_center_size(pos2(outer.center().x, outer.max.y + 14.0), vec2(52.0, 14.0)),
                    3.0,
                    Stroke::new(1.5, self.accent()),
                    egui::StrokeKind::Inside,
                );
            }
        }
    }

    fn tr_apply_to_all(&self) {
        let outer = self.ds_slide_rect(40.0, 8.0, 96.0, 54.0);
        let a = self.ds_slide(40.0, 8.0, 96.0, 54.0);
        self.ds_text(a, 3);
        self.tr_mark(outer.min + vec2(8.0, 8.0), self.accent());
        let strip = self.ds_strip(76.0, 5, None);
        // The mark lands on each mini slide in turn.
        for (i, r) in strip.iter().enumerate() {
            let t = self.reveal(0.1 + i as f32 * 0.15, 0.2 + i as f32 * 0.15);
            if t > 0.0 {
                self.tr_mark(r.min + vec2(5.0, 5.0), self.accent().linear_multiply(t.max(0.2)));
            }
        }
        let t = self.reveal(0.0, 0.1);
        if t > 0.0 {
            self.arrow_down(pos2(outer.center().x, outer.max.y + 2.0), pos2(outer.center().x, outer.max.y + 10.0));
        }
    }
}
