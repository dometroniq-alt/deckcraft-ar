//! Animations tab thumbnails: add animation, trigger, the animation pane and the animation
//! painter. A shape on a slide with its numbered badge, moving or appearing as the film plays.

#![allow(unused_imports)]

use egui::{Align2, Color32, Pos2, Rect, Shape, Stroke, pos2, vec2};

use super::Entry;
use super::design::{DS_BLUE, DS_ORANGE, DS_TEAL};
use super::draw::{Dir, Draw, SIZE};

pub const ENTRIES: &[Entry] = &[
    Entry::new(
        "Add\nAnimation",
        "Gives the selected object one more animation on top of the ones it already has, instead of replacing them. Example: a box that already flies in gets a second effect that makes it pulse afterwards.",
        |d| d.an_add_animation(),
    ),
    Entry::new(
        "Trigger",
        "Makes the selected animation start when you click a particular object on the slide, instead of on the next click anywhere. Example: the picture appears only when the presenter clicks the «show» button.",
        |d| d.an_trigger(),
    ),
    Entry::new(
        "Animation\nPane",
        "Opens a side pane listing every animation on this slide in order, with its timing bar, so you can reorder, retime or remove them. Example: drag the title's animation above the picture's so it plays first.",
        |d| d.an_pane(),
    ),
    Entry::new(
        "Animation\nPainter",
        "Copies the animation of the selected object and puts the same one on the object you click next; double-click the button to paint several objects. Example: the fly-in of the first box is painted onto the other three.",
        |d| d.an_painter(),
    ),
];

impl Draw<'_> {
    /// An accent shape at `r` with its order badge at the top-left corner.
    pub(super) fn an_shape(&self, r: Rect, col: Color32, num: Option<&str>) {
        self.p.rect_filled(r, 2.0, col.linear_multiply(0.35));
        self.p.rect_stroke(r, 2.0, Stroke::new(1.5, col), egui::StrokeKind::Inside);
        if let Some(n) = num {
            self.ds_num(r.min + vec2(-2.0, -2.0), n);
        }
    }
    /// A brush: a handle from `a` to the tip near `b`, with a wide accent tip.
    pub(super) fn an_brush(&self, a: Pos2, b: Pos2) {
        let dir = (b - a).normalized();
        let mid = b - dir * 8.0;
        self.line(a, mid, self.ds_ink(), 2.5);
        self.line(mid, b, self.accent(), 5.0);
    }

    fn an_add_animation(&self) {
        let a = self.ds_slide_center();
        self.ds_title(a, self.ds_ink());
        // The box flies in (its first animation), then pulses (the one being added).
        let fly = self.reveal(0.05, 0.4);
        let pulse = self.reveal(0.5, 0.8);
        let grow = if self.phase >= 1.0 { 0.0 } else { (pulse * std::f32::consts::PI).sin() * 3.0 };
        let base = Rect::from_min_size(pos2(a.max.x - 48.0, a.min.y + 18.0), vec2(44.0, 26.0));
        let r = base.translate(vec2(-(1.0 - fly) * 60.0, 0.0)).expand(grow);
        self.an_shape(r, DS_BLUE, Some("١"));
        if pulse > 0.0 {
            self.ds_num(r.min + vec2(10.0, -2.0), "٢");
        }
        self.ds_body(Rect::from_min_max(pos2(a.min.x, a.min.y + 10.0), pos2(a.max.x - 54.0, a.max.y)), 3, self.ds_grey());
    }

    fn an_trigger(&self) {
        let a = self.ds_slide_center();
        self.ds_title(a, self.ds_ink());
        // A button on the slide; the pointer clicks it and the picture appears.
        let btn = Rect::from_min_size(pos2(a.min.x + 2.0, a.max.y - 16.0), vec2(34.0, 13.0));
        self.p.rect_filled(btn, 3.0, DS_ORANGE);
        self.ds_label(btn.center(), "اعرض", 7.0, Color32::WHITE, Align2::CENTER_CENTER);
        let click = self.reveal(0.1, 0.35);
        let show = self.reveal(0.4, 0.8);
        let pic = Rect::from_min_size(pos2(a.max.x - 60.0, a.min.y + 10.0), vec2(58.0, 34.0));
        if show > 0.0 {
            let clip = Rect::from_min_max(pic.min, pos2(pic.max.x, pic.min.y + pic.height() * show));
            let clipped = self.p.with_clip_rect(clip.intersect(self.p.clip_rect()));
            let d = self.with_painter(&clipped);
            d.picture(pic);
        } else {
            self.dashed_rect(pic, self.t.border_strong);
        }
        self.ds_num(pic.min + vec2(-2.0, -2.0), "١");
        // A dotted link from the badge to the button: "starts on click of this".
        let (p0, p1) = (pic.left_bottom() + vec2(0.0, 2.0), btn.right_top() + vec2(-2.0, 0.0));
        let n = 6;
        for i in 0..n {
            let t0 = i as f32 / n as f32;
            let t1 = t0 + 0.5 / n as f32;
            self.line(p0.lerp(p1, t0), p0.lerp(p1, t1), self.accent(), 1.2);
        }
        self.pointer(btn.center() + vec2(4.0 * (1.0 - click) + 2.0, 6.0 - 3.0 * click));
    }

    fn an_pane(&self) {
        let a = self.ds_slide(6.0, 20.0, 100.0, 58.0);
        self.ds_title(a, self.ds_ink());
        self.an_shape(Rect::from_min_size(pos2(a.max.x - 40.0, a.min.y + 12.0), vec2(36.0, 20.0)), DS_BLUE, Some("١"));
        self.an_shape(Rect::from_min_size(pos2(a.min.x + 2.0, a.min.y + 20.0), vec2(30.0, 18.0)), DS_ORANGE, Some("٢"));
        self.ds_num(pos2(a.max.x - 2.0, a.min.y + 3.0), "٣");
        let pane = self.pane(true, 60.0, 0);
        // Three rows: badge, name line, timing bar; the bars grow as the film plays.
        let t = self.reveal(0.2, 0.8);
        for (i, (col, from, len)) in [(DS_BLUE, 0.0, 0.5), (DS_ORANGE, 0.35, 0.4), (self.accent(), 0.6, 0.4)].iter().enumerate() {
            let y = pane.min.y + 24.0 + i as f32 * 20.0;
            self.ds_num(pos2(pane.min.x + 9.0, y), ["١", "٢", "٣"].get(i).copied().unwrap_or("١"));
            self.line(pos2(pane.min.x + 17.0, y), pos2(pane.max.x - 6.0, y), self.grey(), 1.5);
            let track = Rect::from_min_size(pos2(pane.min.x + 17.0, y + 5.0), vec2(pane.width() - 23.0, 5.0));
            self.p.rect_filled(track, 1.0, self.t.hover);
            let x0 = track.min.x + track.width() * from;
            let w = (track.width() * len * t).max(0.0);
            self.p.rect_filled(Rect::from_min_size(pos2(x0, track.min.y), vec2(w, track.height())), 1.0, *col);
        }
    }

    fn an_painter(&self) {
        self.ds_pair(
            |d, a| {
                d.ds_title(a, d.ds_ink());
                d.an_shape(Rect::from_min_size(pos2(a.max.x - 28.0, a.min.y + 12.0), vec2(26.0, 16.0)), DS_BLUE, Some("١"));
                d.an_shape(Rect::from_min_size(pos2(a.min.x + 2.0, a.min.y + 12.0), vec2(26.0, 16.0)), DS_TEAL, None);
            },
            |d, a| {
                d.ds_title(a, d.ds_ink());
                d.an_shape(Rect::from_min_size(pos2(a.max.x - 28.0, a.min.y + 12.0), vec2(26.0, 16.0)), DS_BLUE, Some("١"));
                let r = Rect::from_min_size(pos2(a.min.x + 2.0, a.min.y + 12.0), vec2(26.0, 16.0));
                d.an_shape(r, DS_TEAL, Some("١"));
                d.an_brush(r.center() + vec2(14.0, 14.0), r.center() + vec2(3.0, 2.0));
            },
        );
    }
}
