//! Slide Show and Record tab thumbnails: playing the show from the start or from here, the
//! presenter view, hiding a slide, rehearsing timings, recording, custom shows and the set-up
//! box; on the Record tab, the camera cameo, recording from the beginning or from this slide,
//! and Learn More. A screen above a strip of mini slides, the one the show starts from lit.

#![allow(unused_imports)]

use egui::{Align2, Color32, Pos2, Rect, Shape, Stroke, pos2, vec2};

use super::Entry;
use super::design::{DS_BLUE, DS_ORANGE, DS_TEAL};
use super::draw::{Dir, Draw, SIZE};

pub const ENTRIES: &[Entry] = &[
    // ---- Slide Show ----
    Entry::new(
        "Play from\nStart",
        "Starts the slide show full screen from the first slide, whatever slide you are looking at now; press Escape to come back. Example: you are editing slide 7, press it and the show opens on slide 1.",
        |d| d.ss_play(0, false),
    ),
    Entry::new(
        "Play from\nCurrent Slide",
        "Starts the slide show full screen from the slide you are on, so you can check it without going through the earlier ones. Example: on slide 7, press it and the show opens on slide 7.",
        |d| d.ss_play(2, false),
    ),
    Entry::new(
        "Presenter\nView",
        "When the show plays, your screen shows the current slide, the next one, your notes and a clock, while the audience sees only the slide. Example: read your notes under slide 3 while the projector shows the slide alone.",
        |d| d.ss_presenter(),
    ),
    Entry::new(
        "Hide\nSlide",
        "Keeps this slide in the file but skips it when the show plays; press again to show it. Example: hide the budget slide for the public talk and unhide it for the committee.",
        |d| d.ss_hide(),
    ),
    Entry::new(
        "Rehearse\nTimings",
        "Plays the show with a stopwatch and remembers how long you stay on each slide, so later the show can advance by itself at the same pace. Example: you spend 40 seconds on slide 2 during the rehearsal, and in the real show it moves on after 40 seconds.",
        |d| d.ss_rehearse(),
    ),
    Entry::new(
        "Record",
        "Records your voice and your timings while you go through the slides, so the show can play itself with narration; this needs sound capture and is not available yet in this version. Example: record the talk once and send the deck to colleagues who missed it.",
        |d| d.ss_record(),
    ),
    Entry::new(
        "Custom\nShow",
        "Lets you pick some of the slides, in any order, and save them under a name as a shorter show to play for a particular audience. Example: a «managers» show that plays slides 1, 4 and 9 only.",
        |d| d.ss_custom_show(),
    ),
    Entry::new(
        "Set Up\nSlide Show",
        "Opens a box with the rules of the show: which slides play, whether it loops until Escape, whether to use the saved timings and narration, and the pen colour. Example: make the show loop for a stand at an exhibition.",
        |d| d.ss_setup(),
    ),
    // ---- Record ----
    Entry::new(
        "Cameo",
        "Puts a live window of your camera on the slide so the audience sees you beside your content; camera capture is not available yet in this version. Example: a round video of the speaker in the corner of each slide.",
        |d| d.ss_cameo(),
    ),
    Entry::new(
        "From\nBeginning",
        "Starts a recording run of the show from the first slide, keeping how long you stay on each slide. Example: press it, talk through all the slides, and the timings are saved with the deck.",
        |d| d.ss_play(0, true),
    ),
    Entry::new(
        "From\nCurrent Slide",
        "Starts a recording run from the slide you are on, keeping the timings from there; the earlier slides keep theirs. Example: redo the timing of slides 5 to 8 without starting over.",
        |d| d.ss_play(2, true),
    ),
    Entry::new(
        "Learn\nMore",
        "Opens DeckCraft's web page in your browser, with the guide and the news about recording and the other features. Example: press it to read how narration will work when it arrives.",
        |d| d.ss_learn_more(),
    ),
];

impl Draw<'_> {
    /// A screen (a dark frame with a slide in it) at `x, y` of `w × h`; returns the slide's
    /// content area.
    pub(super) fn ss_screen(&self, x: f32, y: f32, w: f32, h: f32) -> Rect {
        let outer = Rect::from_min_size(self.at(x, y), vec2(w, h));
        self.p.rect_filled(outer, 2.0, Color32::from_gray(40));
        self.ds_slide(x + 4.0, y + 3.0, w - 8.0, h - 6.0)
    }
    /// A red recording dot with its ring, centred at `c`, blinking with the film.
    pub(super) fn ss_rec_dot(&self, c: Pos2) {
        let on = self.phase >= 1.0 || (self.phase * 4.0).fract() < 0.6;
        self.p.circle_stroke(c, 5.0, Stroke::new(1.5, self.red()));
        if on {
            self.p.circle_filled(c, 3.0, self.red());
        }
    }
    /// A microphone centred at `c`.
    pub(super) fn ss_mic(&self, c: Pos2, col: Color32) {
        self.p.rect_filled(Rect::from_center_size(c - vec2(0.0, 4.0), vec2(8.0, 14.0)), 4.0, col);
        let pts: Vec<Pos2> = (0..=8)
            .map(|i| {
                let a = i as f32 / 8.0 * std::f32::consts::PI;
                c + vec2(7.0 * a.cos(), 7.0 * a.sin())
            })
            .collect();
        self.p.add(Shape::line(pts, Stroke::new(1.5, col)));
        self.line(c + vec2(0.0, 7.0), c + vec2(0.0, 12.0), col, 1.5);
        self.line(c + vec2(-5.0, 12.0), c + vec2(5.0, 12.0), col, 1.5);
    }
    /// A clock face centred at `c`, its hand turned by `turn` (0 → 1 is one round).
    pub(super) fn ss_clock(&self, c: Pos2, r: f32, turn: f32) {
        self.p.circle_filled(c, r, Color32::WHITE);
        self.p.circle_stroke(c, r, Stroke::new(1.5, self.ds_ink()));
        for i in 0..4 {
            let a = i as f32 * std::f32::consts::FRAC_PI_2;
            let d = vec2(a.cos(), a.sin());
            self.line(c + d * (r - 3.0), c + d * (r - 1.0), self.ds_ink(), 1.0);
        }
        let a = turn * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
        self.line(c, c + vec2(a.cos(), a.sin()) * (r - 3.0), self.red(), 1.5);
    }
    /// A camera centred at `c`.
    pub(super) fn ss_camera(&self, c: Pos2, col: Color32) {
        let body = Rect::from_center_size(c, vec2(18.0, 12.0));
        self.p.rect_filled(body, 2.0, col);
        let pts = vec![
            body.right_top() + vec2(0.0, 2.0),
            body.right_top() + vec2(7.0, -1.0),
            body.right_bottom() + vec2(7.0, 1.0),
            body.right_bottom() - vec2(0.0, 2.0),
        ];
        self.p.add(Shape::convex_polygon(pts, col, Stroke::NONE));
        self.p.circle_filled(body.center(), 2.5, Color32::WHITE);
    }
    /// A person (head and shoulders) in a round frame centred at `c`.
    pub(super) fn ss_person(&self, c: Pos2, r: f32) {
        self.p.circle_filled(c, r, DS_TEAL.lerp_to_gamma(Color32::WHITE, 0.6));
        let clip = self.p.with_clip_rect(Rect::from_center_size(c, vec2(r * 2.0, r * 2.0)).intersect(self.p.clip_rect()));
        clip.circle_filled(c - vec2(0.0, r * 0.3), r * 0.35, DS_TEAL);
        clip.circle_filled(c + vec2(0.0, r * 0.9), r * 0.7, DS_TEAL);
        self.p.circle_stroke(c, r, Stroke::new(1.5, Color32::WHITE));
    }

    /// The show starting from slide `from` of five: the screen grows out of that mini slide;
    /// `rec` adds the recording dot.
    fn ss_play(&self, from: usize, rec: bool) {
        let strip = self.ds_strip(82.0, 5, Some(from));
        let grow = self.reveal(0.1, 0.6);
        let full = Rect::from_min_size(self.at(30.0, 6.0), vec2(116.0, 66.0));
        let start = strip.get(from).copied().unwrap_or(full);
        let r = Rect::from_min_max(start.min.lerp(full.min, grow), start.max.lerp(full.max, grow));
        if grow > 0.05 {
            let a = self.ss_screen(r.min.x - self.r.min.x, r.min.y - self.r.min.y, r.width(), r.height());
            if grow > 0.6 {
                self.ds_text(a, 3);
                let n = ["١", "٢", "٣", "٤", "٥"].get(from).copied().unwrap_or("١");
                self.badge(pos2(a.min.x + 10.0, a.max.y - 6.0), n, self.accent());
                if rec {
                    self.ss_rec_dot(pos2(a.max.x - 6.0, a.max.y - 6.0));
                } else {
                    self.tr_play(a.center() + vec2(2.0, 2.0), 7.0, self.accent());
                }
            }
        }
    }

    fn ss_presenter(&self) {
        // The presenter's screen: current slide big, next slide small, notes, a clock.
        let frame = Rect::from_min_size(self.at(4.0, 6.0), vec2(168.0, 92.0));
        self.p.rect_filled(frame, 3.0, Color32::from_gray(50));
        let cur = self.ds_slide(10.0, 14.0, 96.0, 54.0);
        self.ds_text(cur, 3);
        let next = self.ds_slide(114.0, 14.0, 52.0, 29.0);
        self.ds_title(next, DS_BLUE);
        self.picture(Rect::from_min_max(pos2(next.min.x, next.min.y + 7.0), next.max));
        self.ds_label(pos2(frame.max.x - 6.0, 50.0 + self.r.min.y), "التالية", 6.5, Color32::from_gray(210), Align2::RIGHT_CENTER);
        // Notes under the current slide.
        let notes = Rect::from_min_max(self.at(10.0, 74.0), self.at(106.0, 94.0));
        self.text_lines(notes, true, &[88.0, 70.0, 80.0], 6.0, Color32::from_gray(200));
        // The clock, ticking.
        let turn = if self.phase >= 1.0 { 0.25 } else { self.phase };
        self.ss_clock(self.at(140.0, 76.0), 10.0, turn);
    }

    fn ss_hide(&self) {
        let a = self.ds_slide(40.0, 8.0, 96.0, 54.0);
        self.ds_text(a, 3);
        let strip = self.ds_strip(76.0, 5, Some(2));
        // The third mini slide fades and gets a slash: skipped in the show.
        let t = self.reveal(0.2, 0.6);
        if let Some(r) = strip.get(2) {
            self.p.rect_filled(r.shrink(1.0), 1.0, Color32::from_white_alpha((160.0 * t) as u8));
            if t > 0.0 {
                self.p.rect_stroke(*r, 1.0, Stroke::new(1.5, self.grey()), egui::StrokeKind::Inside);
                self.line(r.left_bottom() + vec2(2.0, -2.0), r.right_top() + vec2(-2.0, 2.0), self.red(), 2.0 * t.max(0.3));
            }
            self.badge(pos2(r.center().x, r.min.y - 8.0), "٣", if t > 0.5 { self.grey() } else { self.accent() });
        }
        if t > 0.5 {
            self.p.rect_filled(a.expand2(vec2(6.0, 5.0)), 2.0, Color32::from_white_alpha(120));
            self.line(a.left_bottom(), a.right_top(), self.red(), 2.0);
        }
    }

    fn ss_rehearse(&self) {
        let a = self.ds_slide(30.0, 6.0, 116.0, 62.0);
        self.ds_text(a, 3);
        let turn = if self.phase >= 1.0 { 0.6 } else { self.phase };
        self.ss_clock(pos2(a.max.x - 8.0, a.max.y - 8.0), 11.0, turn);
        // Under each mini slide, the seconds it took, written in as the clock goes round.
        let strip = self.ds_strip(80.0, 4, Some(1));
        for (i, r) in strip.iter().enumerate() {
            let shown = self.reveal(0.2 + i as f32 * 0.2, 0.3 + i as f32 * 0.2);
            if shown > 0.0 {
                let secs = ["١٥", "٤٠", "٢٠", "٣٠"].get(i).copied().unwrap_or("١٠");
                self.ds_label(pos2(r.center().x, r.max.y + 6.0), secs, 6.5, self.accent(), Align2::CENTER_CENTER);
            }
        }
    }

    fn ss_record(&self) {
        let a = self.ds_slide(30.0, 8.0, 116.0, 64.0);
        self.ds_text(a, 3);
        self.ss_mic(self.at(88.0, 86.0), self.ink());
        self.ss_rec_dot(pos2(a.max.x - 6.0, a.max.y - 6.0));
        // Sound waves beside the microphone, growing as the film plays.
        let t = self.reveal(0.1, 0.9);
        for (k, h) in [4.0, 8.0, 12.0, 7.0, 10.0].iter().enumerate() {
            let x = self.at(104.0 + k as f32 * 5.0, 86.0).x;
            let hh = h * t.max(0.15);
            self.line(pos2(x, self.at(0.0, 86.0).y - hh / 2.0), pos2(x, self.at(0.0, 86.0).y + hh / 2.0), self.red(), 2.0);
        }
    }

    fn ss_custom_show(&self) {
        let inner = self.dialog(&self.ds_ar("عرض مخصص"), 0);
        // A list of all the slides with ticks on some, and the chosen ones on the right.
        let picked = [true, false, false, true, true];
        let t = self.reveal(0.2, 0.8);
        let mut shown = 0usize;
        for (i, on) in picked.iter().enumerate() {
            let y = inner.min.y + 4.0 + i as f32 * 10.0;
            let x = inner.max.x - 4.0;
            let n = ["١", "٢", "٣", "٤", "٥"].get(i).copied().unwrap_or("١");
            self.ds_label(pos2(x, y), n, 7.0, self.ink(), Align2::RIGHT_CENTER);
            let r = Rect::from_min_size(pos2(inner.max.x - 22.0, y - 4.0), vec2(8.0, 8.0));
            self.p.rect_stroke(r, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
            self.line(pos2(inner.max.x - 46.0, y), pos2(inner.max.x - 26.0, y), self.grey(), 1.5);
            if *on {
                let ticked = (shown as f32 + 1.0) / 3.0 <= t + 0.001 || self.phase >= 1.0;
                if ticked {
                    self.line(r.min + vec2(1.5, 4.0), r.min + vec2(3.5, 6.5), self.green(), 1.5);
                    self.line(r.min + vec2(3.5, 6.5), r.max - vec2(1.5, 5.5), self.green(), 1.5);
                    let mini = Rect::from_min_size(pos2(inner.min.x + 2.0, inner.min.y + 2.0 + shown as f32 * 14.0), vec2(22.0, 12.0));
                    self.p.rect_filled(mini, 1.0, Color32::WHITE);
                    self.p.rect_stroke(mini, 1.0, Stroke::new(1.0, self.accent()), egui::StrokeKind::Inside);
                    self.ds_label(mini.center(), n, 6.5, self.accent(), Align2::CENTER_CENTER);
                }
                shown += 1;
            }
        }
        self.arrow_left(pos2(inner.min.x + 46.0, inner.center().y), pos2(inner.min.x + 30.0, inner.center().y));
    }

    fn ss_setup(&self) {
        let inner = self.dialog(&self.ds_ar("إعداد العرض"), 0);
        // Radio choices on the right, a monitor on the left; the loop choice gets ticked.
        let t = self.reveal(0.3, 0.6);
        for (i, on) in [(0, true), (1, false), (2, t > 0.5 || self.phase >= 1.0)] {
            let y = inner.min.y + 6.0 + i as f32 * 12.0;
            let c = pos2(inner.max.x - 6.0, y);
            if i < 2 {
                self.p.circle_stroke(c, 3.5, self.stroke(self.t.border_strong));
                if on {
                    self.p.circle_filled(c, 2.0, self.accent());
                }
            } else {
                let r = Rect::from_center_size(c, vec2(7.0, 7.0));
                self.p.rect_stroke(r, 1.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
                if on {
                    self.line(r.min + vec2(1.5, 3.5), r.min + vec2(3.0, 5.5), self.green(), 1.5);
                    self.line(r.min + vec2(3.0, 5.5), r.max - vec2(1.0, 5.0), self.green(), 1.5);
                }
            }
            self.line(pos2(inner.max.x - 44.0, y), pos2(inner.max.x - 14.0, y), self.grey(), 1.5);
        }
        // A monitor with a tiny slide.
        let mon = Rect::from_min_size(pos2(inner.min.x + 4.0, inner.min.y + 6.0), vec2(40.0, 24.0));
        self.p.rect_filled(mon, 2.0, Color32::from_gray(40));
        self.p.rect_filled(mon.shrink(3.0), 1.0, Color32::WHITE);
        self.line(mon.center_bottom(), mon.center_bottom() + vec2(0.0, 5.0), self.ink(), 2.0);
        self.line(mon.center_bottom() + vec2(-8.0, 5.0), mon.center_bottom() + vec2(8.0, 5.0), self.ink(), 2.0);
        self.ds_title(mon.shrink(5.0), self.accent());
    }

    fn ss_cameo(&self) {
        let a = self.ds_slide_center();
        self.ds_text(Rect::from_min_max(pos2(a.min.x + 36.0, a.min.y), a.max), 3);
        // The camera bubble lands in the corner of the slide.
        let t = self.reveal(0.2, 0.6);
        let c = pos2(a.min.x + 16.0, a.max.y - 16.0 - 30.0 * (1.0 - t));
        self.ss_person(c, 14.0 * t.max(0.3));
        self.ss_camera(self.at(26.0, 14.0), self.ink());
    }

    fn ss_learn_more(&self) {
        // A question mark on the slide, and the arrow out to a browser window.
        let a = self.ds_slide(6.0, 26.0, 70.0, 40.0);
        self.ds_text(Rect::from_min_max(pos2(a.min.x + 20.0, a.min.y), a.max), 2);
        self.p.circle_filled(pos2(a.min.x + 10.0, a.center().y + 4.0), 9.0, self.accent());
        self.glyph(pos2(a.min.x + 10.0, a.center().y + 4.0), "؟", 12.0, Color32::WHITE);
        let t = self.reveal(0.1, 0.4);
        self.arrow_right(self.at(80.0, 46.0), self.at(80.0 + 14.0 * t, 46.0));
        let win = Rect::from_min_size(self.at(100.0, 20.0), vec2(70.0, 54.0));
        let open = self.reveal(0.3, 0.7);
        if open > 0.0 {
            self.p.rect_filled(win, 3.0, self.t.menu);
            self.p.rect_stroke(win, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
            let bar = Rect::from_min_size(win.min, vec2(win.width(), 10.0));
            self.p.rect_filled(bar, 3.0, self.t.hover);
            let addr = Rect::from_min_size(win.min + vec2(14.0, 2.0), vec2(win.width() - 18.0, 6.0));
            self.p.rect_filled(addr, 2.0, self.t.input);
            for k in 0..3 {
                self.p.circle_filled(win.min + vec2(4.0 + k as f32 * 4.0, 5.0), 1.3, self.grey());
            }
            let body = Rect::from_min_max(win.min + vec2(6.0, 14.0), win.max - vec2(6.0, 4.0));
            let clipped = self
                .p
                .with_clip_rect(Rect::from_min_max(body.min, pos2(body.max.x, body.min.y + body.height() * open)).intersect(self.p.clip_rect()));
            let d = self.with_painter(&clipped);
            d.heading_then_text(body, Dir::Rtl, 3, self.accent());
        }
    }
}
