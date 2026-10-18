//! Home tab thumbnails: the clipboard, slides (new, layout, section, reset), font and paragraph
//! formatting, the drawing group (shapes, arrange, styles, fill, outline, effects), editing
//! (find, replace, replace fonts) and the panes — plus Photo Album. Mostly pairs of little 16:9
//! slides: the left slide as it is, the right one after the button, with what changed in the
//! accent colour. The single-slide drawings are short films: a shape lands, a line grows, a pane
//! fills. This file also holds the slide-flavoured helpers the other tab files share
//! (`hm_slide`, `hm_pair`, `hm_title`, `hm_body`, `hm_strip`, `hm_clipboard`, `hm_ar`…).

#![allow(unused_imports)]

use egui::{Align2, Color32, Pos2, Rect, Shape, Stroke, pos2, vec2};

use super::Entry;
use super::draw::{Dir, Draw, SIZE};

pub const ENTRIES: &[Entry] = &[
    // ---- Photo Album ----
    Entry::new(
        "Photo\nAlbum",
        "Builds a new presentation from a set of pictures: pick the photos and each one gets its own slide, with a frame and a caption if you like. Example: twenty photos from a visit become a twenty-slide album in one go.",
        |d| d.hm_photo_album(),
    ),
    // ---- Clipboard ----
    Entry::new(
        "Paste",
        "Puts what you cut or copied onto the current slide: text where the caret is, or a shape or picture in the middle of the slide. Example: copy a logo from slide ١ and paste it onto slide ٢.",
        |d| d.hm_paste(),
    ),
    Entry::new(
        "Cut",
        "Takes the selected shape or text off the slide and keeps it on the clipboard, so you can paste it somewhere else. Example: cut a picture from one slide and paste it on the next.",
        |d| d.hm_clip(true),
    ),
    Entry::new(
        "Copy",
        "Keeps a copy of the selected shape or text on the clipboard to paste elsewhere; the original stays where it is. Example: copy a title box once and paste it on three slides.",
        |d| d.hm_clip(false),
    ),
    Entry::new(
        "Format",
        "Format Painter: copies the look of the selected shape or text (fill, outline, font, colour) and paints it onto the next thing you click. Example: click a blue rounded box, then click a plain box to make it blue and rounded too.",
        |d| d.hm_format_painter(),
    ),
    // ---- Slides ----
    Entry::new(
        "New\nSlide",
        "Adds a new empty slide after the current one, with the same layout: a title and a body ready to fill. Example: after the introduction slide, a new blank slide appears for the first topic.",
        |d| d.hm_new_slide(),
    ),
    Entry::new(
        "Layout",
        "Changes how the placeholders are arranged on the current slide: title only, title and content, two columns, comparison… Example: a slide with one text box becomes a slide with two side-by-side boxes.",
        |d| d.hm_layout(),
    ),
    Entry::new(
        "Section",
        "Groups the slides into named sections you can collapse, move and jump to in the slide list. Example: the slides about the budget get a section called «الميزانية».",
        |d| d.hm_section(),
    ),
    Entry::new(
        "Reset",
        "Puts the placeholders of the current slide back where the layout has them, with the layout's fonts and sizes, after you moved or restyled them. Example: a title dragged to the corner jumps back to the top.",
        |d| d.hm_reset(),
    ),
    // ---- Font ----
    Entry::new("Increase Font Size", "Makes the selected text one step bigger. Example: a title in ٢٨ points becomes ٣٢ points.", |d| {
        d.hm_font_size(true)
    }),
    Entry::new(
        "Decrease Font Size",
        "Makes the selected text one step smaller. Example: a body text in ٢٠ points becomes ١٨ points, so one more line fits.",
        |d| d.hm_font_size(false),
    ),
    Entry::new(
        "Clear All Formatting",
        "Removes bold, italic, colours, sizes and other looks from the selected text, leaving plain text in the slide's own font. Example: a red bold underlined line becomes an ordinary line.",
        |d| d.hm_clear_formatting(),
    ),
    Entry::new(
        "Bold",
        "Makes the selected text bold: thicker, darker letters that stand out. Example: the key word of a bullet point is made bold.",
        |d| d.hm_pair(|d, a| d.hm_words(a, false, d.grey()), |d, a| d.hm_words_bold(a)),
    ),
    Entry::new("Italic", "Slants the selected letters to the side, like handwriting. Example: a quotation on the slide is written slanted.", |d| {
        d.hm_pair(|d, a| d.hm_words(a, false, d.grey()), |d, a| d.hm_words(a, true, d.accent()))
    }),
    Entry::new("Underline", "Draws a line under the selected text. Example: the word «مهم» gets a line under it.", |d| {
        d.hm_pair(|d, a| d.hm_words(a, false, d.grey()), |d, a| d.hm_words_marked(a, 5.0))
    }),
    Entry::new(
        "Strikethrough",
        "Draws a line through the middle of the selected text, as if crossed out. Example: an old price is crossed out next to the new one.",
        |d| d.hm_pair(|d, a| d.hm_words(a, false, d.grey()), |d, a| d.hm_words_marked(a, 0.0)),
    ),
    Entry::new("Superscript", "Makes the selected text small and raised above the line. Example: the ٢ in م² is raised.", |d| d.hm_script(true)),
    Entry::new("Subscript", "Makes the selected text small and lowered below the line. Example: the ٢ in H₂O is lowered.", |d| d.hm_script(false)),
    Entry::new(
        "Character Spacing",
        "Widens or tightens the space between the letters of the selected text. Example: the letters of a title are spread out so it fills the slide's width.",
        |d| d.hm_char_spacing(),
    ),
    Entry::new(
        "Change Case",
        "Changes the capital letters of selected Latin text: all capitals, all small, first letter capital, or the reverse. Example: «word craft» becomes «WORD CRAFT».",
        |d| d.hm_change_case(),
    ),
    Entry::new(
        "Text Highlight Color",
        "Paints a bright colour behind the selected text, like a marker pen, so it stands out. Example: the deadline in a bullet point gets a yellow background.",
        |d| d.hm_highlight(),
    ),
    Entry::new("Font Color", "Colours the selected letters; the background stays as it is. Example: the warning line is written in red.", |d| {
        d.hm_font_color()
    }),
    // ---- Paragraph ----
    Entry::new(
        "Bullets",
        "Puts a dot in front of each selected paragraph, making a bulleted list; press again to remove the dots. Example: three plain lines become three bullet points.",
        |d| d.hm_list(&["•", "•", "•"]),
    ),
    Entry::new(
        "Numbering",
        "Numbers the selected paragraphs ١, ٢, ٣… in order; add a paragraph and the numbers follow. Example: the steps of a procedure are numbered.",
        |d| d.hm_list(&["١", "٢", "٣"]),
    ),
    Entry::new(
        "Decrease List Level",
        "Moves the selected list item one level out, back towards the margin, making it a main point again. Example: a sub-point under «المعدات» becomes a main point of its own.",
        |d| d.hm_list_level(false),
    ),
    Entry::new(
        "Increase List Level",
        "Moves the selected list item one level in, making it a sub-point of the item above it. Example: «الطابعة» becomes a sub-point under «المعدات».",
        |d| d.hm_list_level(true),
    ),
    Entry::new(
        "Line Spacing",
        "Sets the space between the lines of the selected text: single, one and a half, double, or an exact value. Example: a crowded bullet list gets more air between its lines.",
        |d| d.hm_line_spacing(),
    ),
    Entry::new(
        "Columns",
        "Splits the text of the selected box into two or more columns side by side. Example: a long list of names flows into two columns instead of one.",
        |d| d.hm_columns(),
    ),
    Entry::new(
        "Text Direction",
        "Turns the text in the selected box: horizontal, rotated ٩٠°, rotated ٢٧٠°, or stacked letter under letter. Example: a label beside a chart is turned to run from bottom to top.",
        |d| d.hm_text_direction(),
    ),
    Entry::new(
        "Align Text",
        "Places the text at the top, the middle or the bottom of its box. Example: a short title in a tall box is moved to the middle.",
        |d| d.hm_align_text(),
    ),
    Entry::new(
        "Convert to SmartArt",
        "Turns the selected bulleted list into a diagram: boxes, a process with arrows, a cycle, a hierarchy… Example: three bullet points become three connected boxes.",
        |d| d.hm_smartart_convert(),
    ),
    Entry::new(
        "Align Left",
        "Lines the selected text up along the left edge of its box. Example: an English address on the slide starts at the left edge.",
        |d| d.hm_align(0),
    ),
    Entry::new("Center", "Centres each line of the selected text in its box. Example: the title sits in the middle of the slide.", |d| d.hm_align(1)),
    Entry::new(
        "Align Right",
        "Lines the selected text up along the right edge of its box, as Arabic text normally is. Example: a paragraph that was centred moves to the right edge.",
        |d| d.hm_align(2),
    ),
    Entry::new(
        "Justify",
        "Stretches each line of the selected text to reach both edges of its box, by widening the spaces between words. Example: a paragraph gets straight edges on both sides.",
        |d| d.hm_align(3),
    ),
    // ---- Drawing ----
    Entry::new(
        "Shapes",
        "Opens a gallery of shapes (rectangles, circles, arrows, stars, callouts…) to draw on the slide: pick one, then drag on the slide. Example: a circle is drawn around the important number.",
        |d| d.hm_shapes(),
    ),
    Entry::new(
        "Arrange",
        "Orders, aligns, groups and rotates the selected shapes: bring to front, send to back, align tops, distribute evenly, group. Example: three boxes at different heights are lined up in a row.",
        |d| d.hm_arrange(),
    ),
    Entry::new(
        "Quick\nStyles",
        "Gives the selected shape a ready-made look in one click: fill, outline and effects that match the theme. Example: a plain box becomes a blue box with a soft shadow.",
        |d| d.hm_quick_styles(),
    ),
    Entry::new(
        "Shape Fill",
        "Colours the inside of the selected shape: a colour, a gradient, a picture or no fill at all. Example: a white box is filled with the theme's blue.",
        |d| d.hm_shape_fill(),
    ),
    Entry::new(
        "Shape Outline",
        "Sets the line around the selected shape: its colour, thickness and dashes, or no line. Example: a box gets a thick dark border.",
        |d| d.hm_shape_outline(),
    ),
    Entry::new(
        "Shape Effects",
        "Adds a shadow, glow, reflection, soft edge or 3-D look to the selected shape. Example: a box gets a shadow so it seems to float above the slide.",
        |d| d.hm_shape_effects(),
    ),
    Entry::new(
        "Text Box",
        "Draws a box on the slide to type free text anywhere, outside the layout's placeholders. Example: a note is added in the corner of a chart slide.",
        |d| d.hm_text_box(),
    ),
    // ---- Editing ----
    Entry::new(
        "Picture",
        "Places a picture file from your computer on the current slide; drag it to move it and pull its corners to resize it. Example: the unit's logo lands on the title slide.",
        |d| d.hm_picture(),
    ),
    Entry::new(
        "Find",
        "Searches all the slides for a word and highlights each place it appears. Example: find every «بغداد» in the presentation.",
        |d| d.hm_find(),
    ),
    Entry::new(
        "Replace",
        "Finds a word on the slides and swaps it for another, one at a time or all at once. Example: every «٢٠٢٥» becomes «٢٠٢٦».",
        |d| d.hm_replace(),
    ),
    Entry::new(
        "Replace Fonts…",
        "Swaps one font for another everywhere in the presentation. Example: every text set in one font is changed to the new font in a single step.",
        |d| d.hm_replace_fonts(),
    ),
    Entry::new(
        "Selection\nPane",
        "Opens a list of everything on the current slide, to select, rename, hide or reorder the shapes, even ones hidden behind others. Example: hide the big picture to reach the small text box behind it.",
        |d| d.hm_selection_pane(),
    ),
    Entry::new(
        "Design\nIdeas",
        "Suggests ready-made designs for the current slide's content in a side pane; click one to apply it. Example: a slide with a title and a picture is offered three polished arrangements.",
        |d| d.hm_design_ideas(),
    ),
    // ---- Send (Craft exchange) ----
    Entry::new(
        "Send to\nWordCraft",
        "Hands the presentation to WordCraft AR as a document: every slide becomes a heading with its text, lists, tables and pictures under it, keeping right-to-left direction, alignment and kashida; the file is also saved in the exchange folder. Example: a 12-slide deck becomes a document with 12 headings, right-to-left and kashida kept.",
        |d| d.hm_send_word(),
    ),
    Entry::new(
        "Send to\nGridCraft",
        "Hands the presentation to GridCraft AR as a workbook: each table lands on its own sheet and the slide text goes into a Text sheet, right-to-left direction and kashida kept per cell. Example: a slide with a 12-row duty roster becomes a sheet with 12 rows.",
        |d| d.hm_send_grid(),
    ),
    Entry::new(
        "Receive…",
        "Opens a Craft exchange file (.craftx) from WordCraft AR or GridCraft AR as a new presentation: each heading starts a slide, long text continues on «(2)», «(3)»… slides, tables get slides of their own, and direction, alignment and kashida stay as they were. Example: a 3-page duties document becomes 7 slides, every paragraph still right-to-left and justified with kashida.",
        |d| d.hm_receive_craft(),
    ),
];

/// A light tint of a colour for shading and translucent marks.
fn tint(c: Color32) -> Color32 {
    c.linear_multiply(0.3)
}

impl Draw<'_> {
    // ---- the slide kit ----

    /// A slide (white, grey border) at `x, y` of `w × h`; returns its content area.
    pub(super) fn hm_slide(&self, x: f32, y: f32, w: f32, h: f32) -> Rect {
        let r = Rect::from_min_size(self.at(x, y), vec2(w, h));
        self.p.rect_filled(r, 1.5, Color32::WHITE);
        self.p.rect_stroke(r, 1.5, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        r.shrink2(vec2(6.0, 5.0))
    }
    /// One centred 16:9 slide; returns its content area.
    pub(super) fn hm_slide_center(&self) -> Rect {
        self.hm_slide(20.0, 14.0, 136.0, 76.0)
    }
    /// Two slides with an arrow between them: `before(area)` then `after(area)`. In motion the
    /// arrow grows first and the "after" slide is then wiped in from the left.
    pub(super) fn hm_pair(&self, before: impl FnOnce(&Draw<'_>, Rect), after: impl FnOnce(&Draw<'_>, Rect)) {
        let b = self.hm_slide(6.0, 32.0, 70.0, 40.0);
        before(self, b);
        let arrow = self.reveal(0.05, 0.3);
        if arrow > 0.0 {
            self.arrow_right(self.at(80.0, 52.0), self.at(80.0 + 16.0 * arrow, 52.0));
        }
        let outer = Rect::from_min_size(self.at(100.0, 32.0), vec2(70.0, 40.0));
        let wipe = self.reveal(0.3, 0.75);
        if wipe <= 0.0 {
            self.hm_slide(100.0, 32.0, 70.0, 40.0);
            return;
        }
        let clip = Rect::from_min_max(outer.min - vec2(2.0, 2.0), pos2(outer.min.x + outer.width() * wipe + 2.0, outer.max.y + 2.0));
        let clipped = self.p.with_clip_rect(clip.intersect(self.p.clip_rect()));
        let d = self.with_painter(&clipped);
        let a = d.hm_slide(100.0, 32.0, 70.0, 40.0);
        after(&d, a);
        if wipe < 1.0 {
            let rest =
                self.p.with_clip_rect(Rect::from_min_max(pos2(clip.max.x, clip.min.y), outer.max + vec2(2.0, 2.0)).intersect(self.p.clip_rect()));
            self.with_painter(&rest).hm_slide(100.0, 32.0, 70.0, 40.0);
        }
    }
    /// A slide title: a thick line along the top of `a`, right-aligned (Arabic).
    pub(super) fn hm_title(&self, a: Rect, c: Color32) {
        let y = a.min.y + 3.0;
        let w = a.width() * 0.55;
        self.line(pos2(a.max.x - w, y), pos2(a.max.x, y), c, if a.height() > 50.0 { 3.5 } else { 2.5 });
    }
    /// `n` grey body lines under the title, right-aligned, `pitch` apart; returns their area.
    pub(super) fn hm_body(&self, a: Rect, n: usize, pitch: f32, c: Color32) -> Rect {
        let area = Rect::from_min_max(pos2(a.min.x, a.min.y + 7.0), a.max);
        let full = area.width();
        let widths: Vec<f32> = (0..n).map(|i| full * [0.9, 0.7, 0.82, 0.6, 0.76].get(i % 5).copied().unwrap_or(0.8)).collect();
        self.text_lines(area, true, &widths, pitch, c);
        area
    }
    /// A slide with a title and `n` body lines (the ordinary "slide with content").
    pub(super) fn hm_title_body(&self, a: Rect, n: usize) {
        self.hm_title(a, self.ink());
        let pitch = if a.height() > 50.0 { 9.0 } else { 7.0 };
        self.hm_body(a, n, pitch, self.grey());
    }
    /// A vertical strip of `n` slide thumbnails (the slide list) at `x, y`, each `w` wide;
    /// returns their outer rects.
    pub(super) fn hm_strip(&self, x: f32, y: f32, w: f32, n: usize) -> Vec<Rect> {
        let h = w * 9.0 / 16.0;
        (0..n.min(6))
            .map(|i| {
                let yy = y + i as f32 * (h + 6.0);
                let a = self.hm_slide(x, yy, w, h);
                self.hm_title(a, self.ink());
                self.text_lines(Rect::from_min_max(pos2(a.min.x, a.min.y + 5.0), a.max), true, &[a.width() * 0.8, a.width() * 0.6], 4.5, self.grey());
                Rect::from_min_size(self.at(x, yy), vec2(w, h))
            })
            .collect()
    }
    /// A sample string as written: [`Draw::label`] puts every string in the order the renderer
    /// draws it right (`i18n::artwork`), so nothing is turned here any more.
    pub(super) fn hm_ar(s: &str) -> String {
        s.to_string()
    }
    /// A clipboard (a board with a clip) centred at `c`, holding a small accent shape, faded by
    /// `alpha`.
    pub(super) fn hm_clipboard(&self, c: Pos2, alpha: f32) {
        let board = Rect::from_center_size(c, vec2(26.0, 32.0));
        self.p.rect_filled(board, 2.0, self.t.hover.linear_multiply(alpha.max(0.3)));
        self.p.rect_stroke(board, 2.0, self.stroke(self.t.border_strong.linear_multiply(alpha.max(0.3))), egui::StrokeKind::Inside);
        let clip = Rect::from_center_size(pos2(c.x, board.min.y), vec2(10.0, 5.0));
        self.p.rect_filled(clip, 1.0, self.t.text.linear_multiply(alpha.max(0.3)));
        let s = Rect::from_center_size(c + vec2(0.0, 3.0), vec2(16.0, 12.0));
        self.p.rect_filled(s, 2.0, self.t.accent.linear_multiply(0.35 * alpha));
        self.p.rect_stroke(s, 2.0, Stroke::new(1.2, self.t.accent.linear_multiply(alpha)), egui::StrokeKind::Inside);
    }
    /// A dashed placeholder box (an empty layout slot).
    pub(super) fn hm_placeholder(&self, r: Rect) {
        self.dashed_rect(r, self.t.border_strong);
    }
    /// A plain outlined shape (white inside, grey line) in `r`.
    pub(super) fn hm_shape_plain(&self, r: Rect) {
        self.p.rect_filled(r, 3.0, Color32::WHITE);
        self.p.rect_stroke(r, 3.0, Stroke::new(1.2, self.t.border_strong), egui::StrokeKind::Inside);
    }
    /// An ellipse stroke/fill in `r`.
    pub(super) fn hm_ellipse(&self, r: Rect, fill: Color32, stroke: Stroke) {
        let c = r.center();
        let n = 32;
        let pts: Vec<Pos2> = (0..n)
            .map(|i| {
                let a = i as f32 / n as f32 * std::f32::consts::TAU;
                pos2(c.x + r.width() * 0.5 * a.cos(), c.y + r.height() * 0.5 * a.sin())
            })
            .collect();
        self.p.add(Shape::convex_polygon(pts, fill, stroke));
    }
    /// A triangle (point up) in `r`.
    pub(super) fn hm_triangle(&self, r: Rect, fill: Color32, stroke: Stroke) {
        self.p.add(Shape::convex_polygon(vec![r.center_top(), r.right_bottom(), r.left_bottom()], fill, stroke));
    }
    /// A block arrow pointing right in `r`.
    pub(super) fn hm_block_arrow(&self, r: Rect, fill: Color32, stroke: Stroke) {
        let (x0, x1, y0, y1) = (r.min.x, r.max.x, r.min.y, r.max.y);
        let (cy, h) = (r.center().y, r.height());
        let xm = x1 - r.width() * 0.4;
        let pts = vec![
            pos2(x0, cy - h * 0.25),
            pos2(xm, cy - h * 0.25),
            pos2(xm, y0),
            pos2(x1, cy),
            pos2(xm, y1),
            pos2(xm, cy + h * 0.25),
            pos2(x0, cy + h * 0.25),
        ];
        self.p.add(Shape::convex_polygon(vec![pos2(xm, y0), pos2(x1, cy), pos2(xm, y1)], fill, Stroke::NONE));
        self.p.rect_filled(Rect::from_min_max(pos2(x0, cy - h * 0.25), pos2(xm, cy + h * 0.25)), 0.0, fill);
        self.p.add(Shape::closed_line(pts, stroke));
    }
    /// `r` scaled about its centre by `k` (0 → a point, 1 → itself).
    pub(super) fn hm_grow(r: Rect, k: f32) -> Rect {
        Rect::from_center_size(r.center(), r.size() * k.clamp(0.0, 1.0))
    }
    /// A brush: a handle from `a` to the tip near `b`, with a wide accent tip.
    pub(super) fn hm_brush(&self, a: Pos2, b: Pos2) {
        let dir = (b - a).normalized();
        let mid = b - dir * 8.0;
        self.line(a, mid, self.ink(), 2.5);
        self.line(mid, b, self.accent(), 5.0);
    }

    /// The rows of "words" the Font pictures share: block widths, right to left, 4 apart.
    const HM_WORDS: [&'static [f32]; 3] = [&[12.0, 8.0, 15.0, 9.0], &[10.0, 14.0, 8.0], &[13.0, 10.0]];

    /// Three rows of "words" (blocks) in a small slide; slanted parallelograms when `italic`.
    pub(super) fn hm_words(&self, a: Rect, italic: bool, c: Color32) {
        self.hm_title(a, self.ink());
        for (r, ws) in Self::HM_WORDS.iter().enumerate() {
            let y = a.min.y + 12.0 + r as f32 * 9.0;
            let mut x = a.max.x;
            for w in ws.iter() {
                let (x0, x1) = (x - w, x);
                if italic {
                    let pts = vec![pos2(x0 + 1.5, y - 2.5), pos2(x1 + 1.5, y - 2.5), pos2(x1 - 1.5, y + 2.5), pos2(x0 - 1.5, y + 2.5)];
                    self.p.add(Shape::convex_polygon(pts, c, Stroke::NONE));
                } else {
                    self.p.rect_filled(Rect::from_min_size(pos2(x0, y - 2.5), vec2(*w, 5.0)), 1.0, c);
                }
                x = x0 - 4.0;
            }
        }
    }
    /// The words, thicker and in the accent (bold).
    pub(super) fn hm_words_bold(&self, a: Rect) {
        self.hm_title(a, self.ink());
        for (r, ws) in Self::HM_WORDS.iter().enumerate() {
            let y = a.min.y + 12.0 + r as f32 * 9.0;
            let mut x = a.max.x;
            for w in ws.iter() {
                self.p.rect_filled(Rect::from_min_size(pos2(x - w - 0.5, y - 3.5), vec2(*w + 1.0, 7.0)), 1.0, self.accent());
                x -= w + 4.0;
            }
        }
    }
    /// The words with an accent line `dy` below their middle (strike at 0, underline at 5).
    pub(super) fn hm_words_marked(&self, a: Rect, dy: f32) {
        self.hm_words(a, false, self.grey());
        for (r, ws) in Self::HM_WORDS.iter().enumerate() {
            let y = a.min.y + 12.0 + r as f32 * 9.0 + dy;
            let w: f32 = ws.iter().sum::<f32>() + 4.0 * (ws.len().saturating_sub(1)) as f32;
            self.line(pos2(a.max.x - w, y), pos2(a.max.x, y), self.accent(), 1.5);
        }
    }

    // ---- Photo Album ----

    /// A fan of photos on the left; on the right a column of slides, each taking one photo.
    pub(super) fn hm_photo_album(&self) {
        for (i, (x, y)) in [(8.0, 28.0), (16.0, 36.0), (24.0, 44.0)].iter().enumerate() {
            let r = Rect::from_min_size(self.at(*x, *y), vec2(34.0, 24.0));
            self.picture(r);
            if i == 2 {
                self.handles(r);
            }
        }
        let go = self.reveal(0.0, 0.25);
        if go > 0.0 {
            self.arrow_right(self.at(70.0, 52.0), self.at(70.0 + 18.0 * go, 52.0));
        }
        for i in 0..3 {
            let y = 6.0 + i as f32 * 33.0;
            let a = self.hm_slide(104.0, y, 56.0, 30.0);
            let k = self.reveal(0.3 + 0.12 * i as f32, 0.55 + 0.12 * i as f32);
            let pic = Rect::from_min_max(a.min, pos2(a.max.x, a.max.y - 4.0));
            if k > 0.0 {
                self.picture(Self::hm_grow(pic, 0.3 + 0.7 * k));
            } else {
                self.hm_placeholder(pic);
            }
            self.line(pos2(a.center().x - 8.0, a.max.y - 0.5), pos2(a.center().x + 8.0, a.max.y - 0.5), self.grey(), 1.5);
        }
    }

    // ---- clipboard ----

    /// The clipboard's shape lands on the slide: the arrow grows, then the shape fades in.
    pub(super) fn hm_paste(&self) {
        self.hm_clipboard(self.at(30.0, 52.0), 1.0);
        let go = self.reveal(0.0, 0.25);
        if go > 0.0 {
            self.arrow_right(self.at(52.0, 52.0), self.at(52.0 + 20.0 * go, 52.0));
        }
        let a = self.hm_slide(82.0, 26.0, 88.0, 50.0);
        self.hm_title(a, self.ink());
        self.text_lines(Rect::from_min_max(pos2(a.min.x, a.min.y + 7.0), a.max), true, &[a.width() * 0.6, a.width() * 0.45], 7.0, self.grey());
        let k = self.reveal(0.25, 0.65);
        if k > 0.0 {
            let r = Rect::from_min_size(pos2(a.min.x + 2.0, a.max.y - 20.0), vec2(30.0, 18.0));
            let r = Self::hm_grow(r, 0.4 + 0.6 * k);
            self.shape(r);
            if k >= 1.0 {
                self.handles(r);
            }
        }
    }
    /// Cut (`remove`) or Copy: a selected shape on the left slide; on the right it is gone
    /// (cut) or still there (copy), and the clipboard holding it fades in above.
    pub(super) fn hm_clip(&self, remove: bool) {
        let shape_in = |d: &Draw<'_>, a: Rect, alpha: f32| {
            d.hm_title(a, d.ink());
            d.text_lines(Rect::from_min_max(pos2(a.min.x, a.min.y + 7.0), a.max), true, &[a.width() * 0.5, a.width() * 0.4], 7.0, d.grey());
            let r = Rect::from_min_size(pos2(a.min.x + 2.0, a.max.y - 16.0), vec2(26.0, 14.0));
            if alpha > 0.0 {
                d.p.rect_filled(r, 3.0, d.t.accent.linear_multiply(0.35 * alpha));
                d.p.rect_stroke(r, 3.0, Stroke::new(1.5, d.t.accent.linear_multiply(alpha)), egui::StrokeKind::Inside);
            }
            r
        };
        let b = self.hm_slide(6.0, 32.0, 70.0, 40.0);
        let r = shape_in(self, b, 1.0);
        self.handles(r);
        let go = self.reveal(0.0, 0.2);
        if go > 0.0 {
            self.arrow_right(self.at(80.0, 52.0), self.at(80.0 + 16.0 * go, 52.0));
        }
        let k = self.reveal(0.2, 0.6);
        let a = self.hm_slide(100.0, 32.0, 70.0, 40.0);
        shape_in(self, a, if remove { 1.0 - k } else { 1.0 });
        self.hm_clipboard(self.at(152.0, 16.0), k.max(0.15));
    }
    /// A blue rounded box on the left and a plain box; on the right both look the same.
    pub(super) fn hm_format_painter(&self) {
        self.hm_pair(
            |d, a| {
                d.hm_title(a, d.ink());
                d.shape(Rect::from_min_size(pos2(a.min.x + 2.0, a.min.y + 10.0), vec2(26.0, 16.0)));
                d.hm_shape_plain(Rect::from_min_size(pos2(a.max.x - 28.0, a.min.y + 10.0), vec2(26.0, 16.0)));
            },
            |d, a| {
                d.hm_title(a, d.ink());
                d.shape(Rect::from_min_size(pos2(a.min.x + 2.0, a.min.y + 10.0), vec2(26.0, 16.0)));
                d.shape(Rect::from_min_size(pos2(a.max.x - 28.0, a.min.y + 10.0), vec2(26.0, 16.0)));
            },
        );
        self.hm_brush(self.at(78.0, 24.0), self.at(96.0, 42.0));
    }

    // ---- slides ----

    /// The slide list on the left gains a third slide (it grows in); on the right the new slide
    /// shows its empty placeholders.
    pub(super) fn hm_new_slide(&self) {
        let strip = self.hm_strip(6.0, 8.0, 36.0, 2);
        let k = self.reveal(0.1, 0.5);
        if let Some(last) = strip.last() {
            let y = last.max.y + 6.0;
            let h = (36.0 * 9.0 / 16.0) * k;
            if h > 1.0 {
                let r = Rect::from_min_size(pos2(last.min.x, y), vec2(36.0, h));
                self.p.rect_filled(r, 1.5, Color32::WHITE);
                self.p.rect_stroke(r, 1.5, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
            }
        }
        let a = self.hm_slide(52.0, 14.0, 116.0, 65.0);
        let f = self.reveal(0.4, 0.8);
        if f > 0.0 {
            let c = self.t.border_strong.linear_multiply(f);
            self.dashed_rect(Rect::from_min_size(a.min, vec2(a.width(), 12.0)), c);
            self.dashed_rect(Rect::from_min_max(pos2(a.min.x, a.min.y + 18.0), a.max), c);
            self.label(pos2(a.max.x - 4.0, a.min.y + 6.0), "عنوان", 7.0, self.grey().linear_multiply(f), Align2::RIGHT_CENTER);
            self.label(pos2(a.max.x - 4.0, a.min.y + 26.0), "نص", 7.0, self.grey().linear_multiply(f), Align2::RIGHT_CENTER);
        }
        let badge = self.at(160.0, 14.0);
        self.p.circle_filled(badge, 7.0, self.accent());
        self.line(badge + vec2(-3.5, 0.0), badge + vec2(3.5, 0.0), Color32::WHITE, 1.8);
        self.line(badge + vec2(0.0, -3.5), badge + vec2(0.0, 3.5), Color32::WHITE, 1.8);
    }
    /// Title and one body slot become title and two side-by-side slots.
    pub(super) fn hm_layout(&self) {
        self.hm_pair(
            |d, a| {
                d.hm_title(a, d.ink());
                d.hm_placeholder(Rect::from_min_max(pos2(a.min.x, a.min.y + 8.0), a.max));
            },
            |d, a| {
                d.hm_title(a, d.ink());
                let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 8.0), a.max);
                let half = body.width() / 2.0 - 2.0;
                d.dashed_rect(Rect::from_min_size(body.min, vec2(half, body.height())), d.accent());
                d.dashed_rect(Rect::from_min_size(pos2(body.max.x - half, body.min.y), vec2(half, body.height())), d.accent());
            },
        );
    }
    /// A column of slide thumbnails; a section bar slides in above the last one, which moves
    /// down to make room.
    pub(super) fn hm_section(&self) {
        let k = self.reveal(0.15, 0.6);
        let w = 46.0;
        let h = w * 9.0 / 16.0;
        let x = (SIZE.x - w) / 2.0;
        for i in 0..3 {
            let shift = if i == 2 { 12.0 * k } else { 0.0 };
            let a = self.hm_slide(x, 6.0 + i as f32 * (h + 6.0) + shift, w, h);
            self.hm_title(a, self.ink());
            self.text_lines(Rect::from_min_max(pos2(a.min.x, a.min.y + 5.0), a.max), true, &[a.width() * 0.8, a.width() * 0.6], 4.5, self.grey());
        }
        if k > 0.0 {
            let y = 6.0 + 2.0 * (h + 6.0) - 4.0;
            let bar = Rect::from_min_size(self.at(x - 10.0, y), vec2((w + 20.0) * k, 10.0));
            self.p.rect_filled(bar, 2.0, self.accent());
            if k > 0.6 {
                self.label(pos2(bar.max.x - 4.0, bar.center().y), "قسم", 6.5, Color32::WHITE, Align2::RIGHT_CENTER);
                let c = pos2(bar.min.x + 5.0, bar.center().y);
                self.line(c + vec2(-2.5, -1.5), c + vec2(0.0, 1.5), Color32::WHITE, 1.3);
                self.line(c + vec2(0.0, 1.5), c + vec2(2.5, -1.5), Color32::WHITE, 1.3);
            }
        }
    }
    /// Placeholders dragged out of place jump back to the layout's positions.
    pub(super) fn hm_reset(&self) {
        self.hm_pair(
            |d, a| {
                let title = Rect::from_min_size(pos2(a.min.x + 4.0, a.min.y + 10.0), vec2(a.width() * 0.5, 8.0));
                d.hm_placeholder(title);
                d.line(pos2(title.max.x - 2.0, title.center().y), pos2(title.min.x + 6.0, title.center().y), d.ink(), 2.5);
                let body = Rect::from_min_size(pos2(a.max.x - 24.0, a.max.y - 20.0), vec2(22.0, 18.0));
                d.hm_placeholder(body);
                d.text_lines(body.shrink(2.0), true, &[16.0, 12.0, 14.0], 5.0, d.grey());
            },
            |d, a| {
                let title = Rect::from_min_size(a.min, vec2(a.width(), 8.0));
                d.dashed_rect(title, d.accent());
                d.hm_title(a.translate(vec2(0.0, 1.0)), d.ink());
                let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 11.0), a.max);
                d.dashed_rect(body, d.accent());
                d.text_lines(body.shrink(2.0), true, &[body.width() * 0.8, body.width() * 0.6, body.width() * 0.7], 6.0, d.grey());
            },
        );
    }

    // ---- font ----

    /// A letter grows (or shrinks) on the slide.
    pub(super) fn hm_font_size(&self, bigger: bool) {
        let (s0, s1) = if bigger { (12.0, 22.0) } else { (22.0, 12.0) };
        self.hm_pair(
            |d, a| {
                d.hm_title(a, d.ink());
                d.glyph(pos2(a.center().x, a.center().y + 5.0), "أ", s0, d.grey());
            },
            |d, a| {
                d.hm_title(a, d.ink());
                d.glyph(pos2(a.center().x, a.center().y + 5.0), "أ", s1, d.accent());
                let c = a.center() + vec2(22.0, 5.0);
                if bigger {
                    d.arrow_up(c + vec2(0.0, 8.0), c + vec2(0.0, -8.0));
                } else {
                    d.arrow_down(c + vec2(0.0, -8.0), c + vec2(0.0, 8.0));
                }
            },
        );
    }
    /// Red, thick, underlined lines become plain grey ones.
    pub(super) fn hm_clear_formatting(&self) {
        self.hm_pair(
            |d, a| {
                d.hm_title(a, d.accent());
                let full = a.width();
                let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 7.0), a.max);
                d.text_lines(body, true, &[full * 0.9], 7.0, d.red());
                d.line(pos2(a.max.x - full * 0.9, body.min.y + 7.0), pos2(a.max.x, body.min.y + 7.0), d.red(), 1.0);
                d.line(pos2(a.max.x - full * 0.7, body.min.y + 11.0), pos2(a.max.x, body.min.y + 11.0), d.ink(), 3.0);
                d.line(pos2(a.max.x - full * 0.8, body.min.y + 18.0), pos2(a.max.x, body.min.y + 18.0), d.green(), 1.5);
            },
            |d, a| d.hm_title_body(a, 3),
        );
    }
    /// `X2` on one baseline, then the 2 small and raised (`up`) or lowered.
    pub(super) fn hm_script(&self, up: bool) {
        self.hm_pair(
            |d, a| {
                d.hm_title(a, d.ink());
                let c = a.center() + vec2(0.0, 5.0);
                d.glyph(c + vec2(-5.0, 0.0), "X", 18.0, d.ink());
                d.glyph(c + vec2(8.0, 0.0), "2", 18.0, d.grey());
            },
            |d, a| {
                d.hm_title(a, d.ink());
                let c = a.center() + vec2(0.0, 5.0);
                d.glyph(c + vec2(-5.0, 0.0), "X", 18.0, d.ink());
                let dy = if up { -7.0 } else { 6.0 };
                d.glyph(c + vec2(7.0, dy), "2", 10.0, d.accent());
            },
        );
    }
    /// Letter blocks tight together, then spread apart with a measure under them.
    pub(super) fn hm_char_spacing(&self) {
        let letters = |d: &Draw<'_>, a: Rect, gap: f32, c: Color32| {
            let n = 5;
            let w = 6.0;
            let total = n as f32 * w + (n - 1) as f32 * gap;
            let x0 = a.center().x - total / 2.0;
            for i in 0..n {
                let x = x0 + i as f32 * (w + gap);
                d.p.rect_filled(Rect::from_min_size(pos2(x, a.min.y + 12.0), vec2(w, 9.0)), 1.0, c);
            }
            Rect::from_min_size(pos2(x0, a.min.y + 12.0), vec2(total, 9.0))
        };
        self.hm_pair(
            |d, a| {
                d.hm_title(a, d.ink());
                letters(d, a, 1.5, d.grey());
            },
            |d, a| {
                d.hm_title(a, d.ink());
                let r = letters(d, a, 6.0, d.accent());
                d.measure_h(pos2(r.min.x, r.max.y + 6.0), pos2(r.max.x, r.max.y + 6.0));
            },
        );
    }
    /// «word craft» becomes «WORD CRAFT» in the title.
    pub(super) fn hm_change_case(&self) {
        self.hm_pair(
            |d, a| {
                d.label(pos2(a.center().x, a.min.y + 6.0), "word craft", 8.0, d.grey(), Align2::CENTER_CENTER);
                d.hm_body(a.translate(vec2(0.0, 6.0)), 2, 7.0, d.grey());
            },
            |d, a| {
                d.label(pos2(a.center().x, a.min.y + 6.0), "WORD CRAFT", 8.0, d.accent(), Align2::CENTER_CENTER);
                d.hm_body(a.translate(vec2(0.0, 6.0)), 2, 7.0, d.grey());
            },
        );
    }
    /// A yellow band appears behind one body line.
    pub(super) fn hm_highlight(&self) {
        self.hm_pair(
            |d, a| d.hm_title_body(a, 3),
            |d, a| {
                d.hm_title(a, d.ink());
                let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 7.0), a.max);
                let y = body.min.y + 4.0 + 7.0;
                d.highlight(Rect::from_min_max(pos2(a.max.x - a.width() * 0.7 - 2.0, y - 3.5), pos2(a.max.x + 2.0, y + 3.5)));
                d.hm_body(a, 3, 7.0, d.grey());
            },
        );
    }
    /// One body line turns red.
    pub(super) fn hm_font_color(&self) {
        self.hm_pair(
            |d, a| d.hm_title_body(a, 3),
            |d, a| {
                d.hm_title(a, d.ink());
                let body = d.hm_body(a, 3, 7.0, d.grey());
                let y = body.min.y + 4.0 + 7.0;
                d.line(pos2(a.max.x - a.width() * 0.7, y), pos2(a.max.x, y), d.red(), 1.5);
            },
        );
    }

    // ---- paragraph ----

    /// Plain lines become a list with `markers`.
    pub(super) fn hm_list(&self, markers: &[&str]) {
        self.hm_pair(
            |d, a| d.hm_title_body(a, 3),
            |d, a| {
                d.hm_title(a, d.ink());
                let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 6.0), a.max);
                for (i, m) in markers.iter().enumerate() {
                    let y = body.min.y + 4.0 + i as f32 * 8.0;
                    d.label(pos2(body.max.x, y), m, 7.0, d.accent(), Align2::RIGHT_CENTER);
                    let w = body.width() * [0.7, 0.55, 0.62].get(i % 3).copied().unwrap_or(0.6);
                    d.line(pos2(body.max.x - 9.0 - w, y), pos2(body.max.x - 9.0, y), d.grey(), 1.5);
                }
            },
        );
    }
    /// A bulleted list whose middle item steps in (`inc`) or back out.
    pub(super) fn hm_list_level(&self, inc: bool) {
        let list = |d: &Draw<'_>, a: Rect, indent: f32| {
            d.hm_title(a, d.ink());
            let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 6.0), a.max);
            for i in 0..3 {
                let y = body.min.y + 4.0 + i as f32 * 8.0;
                let off = if i == 1 { indent } else { 0.0 };
                let c = if i == 1 && indent > 0.0 { d.accent() } else { d.grey() };
                let right = body.max.x - off;
                d.label(pos2(right, y), if off > 0.0 { "–" } else { "•" }, 7.0, if i == 1 { d.accent() } else { d.ink() }, Align2::RIGHT_CENTER);
                let w = body.width() * [0.7, 0.5, 0.62].get(i % 3).copied().unwrap_or(0.6);
                d.line(pos2(right - 9.0 - w, y), pos2(right - 9.0, y), c, 1.5);
            }
        };
        if inc {
            self.hm_pair(|d, a| list(d, a, 0.0), |d, a| list(d, a, 10.0));
        } else {
            self.hm_pair(|d, a| list(d, a, 10.0), |d, a| list(d, a, 0.0));
        }
    }
    /// Four lines close together, then farther apart, with a measure beside them.
    pub(super) fn hm_line_spacing(&self) {
        self.hm_pair(
            |d, a| {
                d.hm_title(a, d.ink());
                d.hm_body(a, 4, 5.0, d.grey());
            },
            |d, a| {
                d.hm_title(a, d.ink());
                let body = d.hm_body(a, 4, 8.0, d.accent());
                d.measure_v(pos2(a.min.x + 3.0, body.min.y + 1.0), pos2(a.min.x + 3.0, body.min.y + 4.0 + 3.0 * 8.0 + 3.0));
            },
        );
    }
    /// One wide paragraph becomes two columns.
    pub(super) fn hm_columns(&self) {
        self.hm_pair(
            |d, a| d.hm_title_body(a, 4),
            |d, a| {
                d.hm_title(a, d.ink());
                let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 7.0), a.max);
                let half = body.width() / 2.0 - 3.0;
                let right = Rect::from_min_size(pos2(body.max.x - half, body.min.y), vec2(half, body.height()));
                let left = Rect::from_min_size(body.min, vec2(half, body.height()));
                d.text_lines(right, true, &[half, half * 0.9, half * 0.95, half * 0.7], 5.5, d.accent());
                d.text_lines(left, true, &[half, half * 0.85, half * 0.95, half * 0.5], 5.5, d.accent());
            },
        );
    }
    /// Horizontal lines become vertical ones, with a turning arrow.
    pub(super) fn hm_text_direction(&self) {
        self.hm_pair(
            |d, a| d.hm_title_body(a, 3),
            |d, a| {
                d.hm_title(a, d.ink());
                let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 7.0), a.max);
                for (i, f) in [0.9, 0.7, 0.82].iter().enumerate() {
                    let x = body.max.x - 4.0 - i as f32 * 7.0;
                    d.line(pos2(x, body.min.y + 1.0), pos2(x, body.min.y + 1.0 + body.height() * f), d.accent(), 1.5);
                }
                let c = pos2(body.min.x + 12.0, body.center().y);
                let pts: Vec<Pos2> = (0..=10).map(|i| c + 8.0 * vec2((i as f32 * 0.15 - 1.2).cos(), (i as f32 * 0.15 - 1.2).sin())).collect();
                d.p.add(Shape::line(pts, Stroke::new(1.5, d.accent())));
                let tip = c + 8.0 * vec2(0.3f32.cos(), 0.3f32.sin());
                d.line(tip, tip + vec2(-4.0, -1.0), d.accent(), 1.5);
                d.line(tip, tip + vec2(-0.5, -4.5), d.accent(), 1.5);
            },
        );
    }
    /// Text at the top of its box moves to the middle.
    pub(super) fn hm_align_text(&self) {
        self.hm_pair(
            |d, a| {
                d.hm_title(a, d.ink());
                let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 7.0), a.max);
                d.hm_placeholder(body);
                d.text_lines(body.shrink(2.0), true, &[body.width() * 0.7, body.width() * 0.5], 5.0, d.grey());
            },
            |d, a| {
                d.hm_title(a, d.ink());
                let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 7.0), a.max);
                d.hm_placeholder(body);
                let mid = Rect::from_min_max(pos2(body.min.x + 2.0, body.center().y - 6.5), pos2(body.max.x - 2.0, body.max.y));
                d.text_lines(mid, true, &[body.width() * 0.7, body.width() * 0.5], 5.0, d.accent());
                d.measure_v(pos2(body.min.x + 5.0, body.min.y + 1.5), pos2(body.min.x + 5.0, body.max.y - 1.5));
            },
        );
    }
    /// Three bullet points become three connected boxes.
    pub(super) fn hm_smartart_convert(&self) {
        self.hm_pair(
            |d, a| {
                d.hm_title(a, d.ink());
                let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 6.0), a.max);
                for i in 0..3 {
                    let y = body.min.y + 4.0 + i as f32 * 8.0;
                    d.label(pos2(body.max.x, y), "•", 7.0, d.ink(), Align2::RIGHT_CENTER);
                    let w = body.width() * [0.7, 0.55, 0.62].get(i % 3).copied().unwrap_or(0.6);
                    d.line(pos2(body.max.x - 9.0 - w, y), pos2(body.max.x - 9.0, y), d.grey(), 1.5);
                }
            },
            |d, a| {
                d.hm_title(a, d.ink());
                let y = a.min.y + 19.0;
                let w = 15.0;
                let gap = (a.width() - 3.0 * w) / 2.0;
                for i in 0..3 {
                    let x = a.max.x - w - i as f32 * (w + gap);
                    let r = Rect::from_min_size(pos2(x, y - 7.0), vec2(w, 14.0));
                    d.shape(r);
                    d.line(pos2(r.min.x + 3.0, y), pos2(r.max.x - 3.0, y), Color32::WHITE, 1.5);
                    if i < 2 {
                        d.arrow_left(pos2(x - 1.0, y), pos2(x - gap + 1.0, y));
                    }
                }
            },
        );
    }
    /// Right-aligned lines move to the left (0), the centre (1), the right from the left (2)
    /// or stretch to both edges (3).
    pub(super) fn hm_align(&self, how: u8) {
        let lines = |d: &Draw<'_>, a: Rect, how: u8, c: Color32| {
            d.hm_title(a, d.ink());
            let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 7.0), a.max);
            let full = body.width();
            for (i, f) in [1.0, 0.8, 0.9, 0.6].iter().enumerate() {
                let y = body.min.y + 4.0 + i as f32 * 6.0;
                let w = if how == 3 && i < 3 { full } else { full * f };
                let (x0, x1) = match how {
                    0 => (body.min.x, body.min.x + w),
                    1 => (body.center().x - w / 2.0, body.center().x + w / 2.0),
                    _ => (body.max.x - w, body.max.x),
                };
                d.line(pos2(x0, y), pos2(x1, y), c, 1.5);
            }
        };
        let before = if how == 2 { 0 } else { 2 };
        self.hm_pair(|d, a| lines(d, a, before, d.grey()), |d, a| lines(d, a, how, d.accent()));
    }

    // ---- drawing ----

    /// A tray of shapes above the slide; a circle drops onto the slide and grows.
    pub(super) fn hm_shapes(&self) {
        let tray = Rect::from_min_size(self.at(40.0, 2.0), vec2(96.0, 18.0));
        self.p.rect_filled(tray, 3.0, self.t.menu);
        self.p.rect_stroke(tray, 3.0, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        let s = Stroke::new(1.2, self.ink());
        let cell = |i: usize| Rect::from_center_size(pos2(tray.min.x + 12.0 + i as f32 * 18.0, tray.center().y), vec2(11.0, 11.0));
        self.p.rect_stroke(cell(0), 1.0, s, egui::StrokeKind::Inside);
        self.p.rect_filled(cell(1).expand(1.5), 0.0, self.t.checked);
        self.hm_ellipse(cell(1), Color32::TRANSPARENT, Stroke::new(1.2, self.accent()));
        self.hm_triangle(cell(2), Color32::TRANSPARENT, s);
        self.hm_block_arrow(cell(3), Color32::TRANSPARENT, s);
        let st = cell(4);
        self.p.add(Shape::closed_line(
            (0..5)
                .map(|i| {
                    st.center()
                        + 6.0
                            * vec2(
                                (i as f32 * 2.0 * std::f32::consts::TAU / 5.0 - std::f32::consts::FRAC_PI_2).cos(),
                                (i as f32 * 2.0 * std::f32::consts::TAU / 5.0 - std::f32::consts::FRAC_PI_2).sin(),
                            )
                })
                .collect(),
            s,
        ));
        let a = self.hm_slide(20.0, 26.0, 136.0, 76.0);
        self.hm_title(a, self.ink());
        self.text_lines(Rect::from_min_max(pos2(a.min.x, a.min.y + 8.0), a.max), true, &[a.width() * 0.6, a.width() * 0.5], 8.0, self.grey());
        let k = self.reveal(0.2, 0.65);
        if k > 0.0 {
            let r = Self::hm_grow(Rect::from_min_size(pos2(a.min.x + 6.0, a.min.y + 22.0), vec2(40.0, 28.0)), 0.2 + 0.8 * k);
            self.hm_ellipse(r, tint(self.accent()), Stroke::new(1.5, self.accent()));
            if k >= 1.0 {
                self.handles(r);
            }
        }
    }
    /// Three boxes at odd heights line up along a guide.
    pub(super) fn hm_arrange(&self) {
        self.hm_pair(
            |d, a| {
                for (i, dy) in [4.0, 14.0, 9.0].iter().enumerate() {
                    let r = Rect::from_min_size(pos2(a.min.x + 2.0 + i as f32 * 20.0, a.min.y + dy), vec2(16.0, 12.0));
                    d.hm_shape_plain(r);
                }
            },
            |d, a| {
                let y = a.min.y + 10.0;
                d.line(pos2(a.min.x, y - 1.5), pos2(a.max.x, y - 1.5), d.accent().linear_multiply(0.5), 1.0);
                for i in 0..3 {
                    let r = Rect::from_min_size(pos2(a.min.x + 2.0 + i as f32 * 20.0, y), vec2(16.0, 12.0));
                    d.shape(r);
                }
            },
        );
    }
    /// A plain box becomes a filled box with a shadow and a white line of text.
    pub(super) fn hm_quick_styles(&self) {
        self.hm_pair(
            |d, a| {
                d.hm_title(a, d.ink());
                let r = Rect::from_center_size(a.center() + vec2(0.0, 5.0), vec2(34.0, 18.0));
                d.hm_shape_plain(r);
                d.line(pos2(r.center().x - 8.0, r.center().y), pos2(r.center().x + 8.0, r.center().y), d.grey(), 1.5);
            },
            |d, a| {
                d.hm_title(a, d.ink());
                let r = Rect::from_center_size(a.center() + vec2(0.0, 5.0), vec2(34.0, 18.0));
                d.p.rect_filled(r.translate(vec2(2.0, 2.0)), 3.0, d.grey().linear_multiply(0.5));
                d.p.rect_filled(r, 3.0, d.accent());
                d.line(pos2(r.center().x - 8.0, r.center().y), pos2(r.center().x + 8.0, r.center().y), Color32::WHITE, 1.5);
            },
        );
    }
    /// An outlined shape fills with colour.
    pub(super) fn hm_shape_fill(&self) {
        self.hm_pair(
            |d, a| {
                d.hm_title(a, d.ink());
                d.hm_shape_plain(Rect::from_center_size(a.center() + vec2(0.0, 5.0), vec2(34.0, 18.0)));
            },
            |d, a| {
                d.hm_title(a, d.ink());
                let r = Rect::from_center_size(a.center() + vec2(0.0, 5.0), vec2(34.0, 18.0));
                d.p.rect_filled(r, 3.0, d.accent());
                d.p.rect_stroke(r, 3.0, Stroke::new(1.2, d.t.border_strong), egui::StrokeKind::Inside);
                // A paint bucket's drop.
                d.p.circle_filled(pos2(r.max.x + 6.0, r.min.y - 2.0), 3.0, d.accent());
                d.hm_triangle(Rect::from_center_size(pos2(r.max.x + 6.0, r.min.y - 6.0), vec2(5.0, 5.0)), d.accent(), Stroke::NONE);
            },
        );
    }
    /// A thin grey line around a shape becomes a thick accent one.
    pub(super) fn hm_shape_outline(&self) {
        self.hm_pair(
            |d, a| {
                d.hm_title(a, d.ink());
                let r = Rect::from_center_size(a.center() + vec2(0.0, 5.0), vec2(34.0, 18.0));
                d.p.rect_filled(r, 3.0, tint(d.accent()));
                d.p.rect_stroke(r, 3.0, Stroke::new(1.0, d.t.border_strong), egui::StrokeKind::Inside);
            },
            |d, a| {
                d.hm_title(a, d.ink());
                let r = Rect::from_center_size(a.center() + vec2(0.0, 5.0), vec2(34.0, 18.0));
                d.p.rect_filled(r, 3.0, tint(d.accent()));
                d.p.rect_stroke(r, 3.0, Stroke::new(3.0, d.accent()), egui::StrokeKind::Inside);
            },
        );
    }
    /// A flat shape gets a shadow and a glow.
    pub(super) fn hm_shape_effects(&self) {
        self.hm_pair(
            |d, a| {
                d.hm_title(a, d.ink());
                d.shape(Rect::from_center_size(a.center() + vec2(0.0, 5.0), vec2(34.0, 18.0)));
            },
            |d, a| {
                d.hm_title(a, d.ink());
                let r = Rect::from_center_size(a.center() + vec2(0.0, 5.0), vec2(34.0, 18.0));
                d.p.rect_filled(r.expand(4.0), 6.0, d.accent().linear_multiply(0.12));
                d.p.rect_filled(r.expand(2.0), 5.0, d.accent().linear_multiply(0.18));
                d.p.rect_filled(r.translate(vec2(3.0, 3.0)), 4.0, d.grey().linear_multiply(0.55));
                d.shape(r);
            },
        );
    }
    /// A text box is dragged out on the slide (a crosshair at its corner) and fills with text.
    pub(super) fn hm_text_box(&self) {
        let a = self.hm_slide_center();
        self.hm_title(a, self.ink());
        self.text_lines(Rect::from_min_max(pos2(a.min.x, a.min.y + 8.0), a.max), true, &[a.width() * 0.6, a.width() * 0.5], 8.0, self.grey());
        let k = self.reveal(0.1, 0.5);
        let full = Rect::from_min_size(pos2(a.min.x + 4.0, a.min.y + 30.0), vec2(56.0, 26.0));
        if k > 0.0 {
            let r = Rect::from_min_size(full.min, full.size() * (0.15 + 0.85 * k));
            self.p.rect_stroke(r, 0.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
            let c = r.max;
            self.line(c + vec2(-5.0, 0.0), c + vec2(5.0, 0.0), self.ink(), 1.2);
            self.line(c + vec2(0.0, -5.0), c + vec2(0.0, 5.0), self.ink(), 1.2);
        }
        let t = self.reveal(0.55, 0.9);
        if t > 0.0 {
            let inner = full.shrink(3.0);
            self.text_lines(inner, true, &[inner.width() * 0.9 * t, inner.width() * 0.7 * t, inner.width() * 0.5 * t], 7.0, self.accent());
            if t >= 1.0 {
                self.caret(pos2(inner.max.x - inner.width() * 0.5 - 2.0, inner.min.y + 4.0 + 14.0 - 4.0), 8.0);
            }
        }
    }

    // ---- editing ----

    /// A picture grows onto the slide beside the text.
    pub(super) fn hm_picture(&self) {
        let a = self.hm_slide_center();
        self.hm_title(a, self.ink());
        let text = Rect::from_min_max(pos2(a.max.x - a.width() * 0.45, a.min.y + 8.0), a.max);
        self.text_lines(text, true, &[text.width() * 0.95, text.width() * 0.8, text.width() * 0.9, text.width() * 0.6], 8.0, self.grey());
        let r = Rect::from_min_max(pos2(a.min.x, a.min.y + 12.0), pos2(a.min.x + a.width() * 0.48, a.max.y - 2.0));
        let k = self.reveal(0.1, 0.6);
        if k > 0.0 {
            let r = Self::hm_grow(r, 0.2 + 0.8 * k);
            self.picture(r);
            if k >= 1.0 {
                self.handles(r);
            }
        }
    }
    /// A magnifier slides along the body lines and a word lights up yellow.
    pub(super) fn hm_find(&self) {
        let a = self.hm_slide_center();
        self.hm_title(a, self.ink());
        let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 8.0), a.max);
        let y = body.min.y + 4.0 + 9.0;
        let word = Rect::from_min_max(pos2(a.max.x - 60.0, y - 4.0), pos2(a.max.x - 42.0, y + 4.0));
        let k = self.reveal(0.1, 0.6);
        if k >= 1.0 {
            self.highlight(word);
        }
        self.hm_body(a, 4, 9.0, self.grey());
        self.p.rect_filled(word.shrink2(vec2(0.0, 2.5)), 1.0, if k >= 1.0 { self.ink() } else { self.grey() });
        let from = pos2(a.max.x - 10.0, y - 6.0);
        let to = pos2(word.center().x - 2.0, y - 6.0);
        self.magnifier(from + (to - from) * k);
    }
    /// A grey word in the text becomes an accent one.
    pub(super) fn hm_replace(&self) {
        let word_at = |d: &Draw<'_>, a: Rect, c: Color32, lit: bool| {
            d.hm_title(a, d.ink());
            let body = Rect::from_min_max(pos2(a.min.x, a.min.y + 7.0), a.max);
            let y = body.min.y + 4.0 + 7.0;
            let word = Rect::from_min_max(pos2(a.max.x - 36.0, y - 3.0), pos2(a.max.x - 22.0, y + 3.0));
            if lit {
                d.highlight(word.expand(1.5));
            }
            d.hm_body(a, 3, 7.0, d.grey());
            d.p.rect_filled(word, 1.0, c);
        };
        self.hm_pair(|d, a| word_at(d, a, d.ink(), true), |d, a| word_at(d, a, d.accent(), false));
        self.chip(self.at(88.0, 16.0), "أ ↔ ب", 40.0);
    }
    /// The Replace Fonts box: the old font's sample above the new one's.
    pub(super) fn hm_replace_fonts(&self) {
        let inner = self.dialog("Aa → Aa", 2);
        let f0 = Rect::from_min_size(pos2(inner.min.x + 34.0, inner.min.y), vec2(inner.width() - 34.0, 10.0));
        let f1 = f0.translate(vec2(0.0, 14.0));
        self.label(pos2(f0.min.x + 4.0, f0.center().y), "Aa", 8.0, self.grey(), Align2::LEFT_CENTER);
        let k = self.reveal(0.2, 0.6);
        if k > 0.0 {
            self.label(pos2(f1.min.x + 4.0, f1.center().y), "Aa", 8.5, self.accent().linear_multiply(k), Align2::LEFT_CENTER);
        }
        self.arrow_down(pos2(f0.max.x - 8.0, f0.max.y - 1.0), pos2(f0.max.x - 8.0, f1.min.y + 1.0));
        self.button(pos2(inner.max.x - 14.0, inner.max.y - 8.0), "OK", k >= 1.0);
    }
    /// The slide on the left with three shapes; a pane on the right lists them, the selected
    /// one lit.
    pub(super) fn hm_selection_pane(&self) {
        let a = self.hm_slide(4.0, 22.0, 100.0, 56.0);
        self.hm_title(a, self.ink());
        let pic = Rect::from_min_size(pos2(a.min.x + 2.0, a.min.y + 10.0), vec2(30.0, 22.0));
        self.picture(pic);
        let box1 = Rect::from_min_size(pos2(a.max.x - 36.0, a.min.y + 10.0), vec2(34.0, 12.0));
        self.text_box(box1);
        let box2 = Rect::from_min_size(pos2(a.max.x - 36.0, a.min.y + 26.0), vec2(24.0, 10.0));
        self.shape(box2);
        let k = self.reveal(0.2, 0.5);
        if k >= 1.0 {
            self.handles(box2);
        }
        let pane = self.pane(true, 60.0, 0);
        for i in 0..3 {
            let y = pane.min.y + 20.0 + i as f32 * 14.0;
            let row = Rect::from_min_max(pos2(pane.min.x + 3.0, y - 6.0), pos2(pane.max.x - 3.0, y + 6.0));
            if i == 2 && k > 0.0 {
                self.p.rect_filled(row, 2.0, self.t.checked.linear_multiply(k));
            }
            let icon = Rect::from_center_size(pos2(row.min.x + 7.0, y), vec2(8.0, 6.0));
            match i {
                0 => {
                    self.p.rect_stroke(icon, 1.0, self.stroke(self.ink()), egui::StrokeKind::Inside);
                }
                1 => {
                    self.p.rect_filled(icon, 0.0, Color32::from_rgb(96, 150, 96));
                }
                _ => {
                    self.p.rect_filled(icon, 2.0, self.accent());
                }
            }
            self.line(pos2(row.min.x + 15.0, y), pos2(row.max.x - 12.0, y), self.grey(), 1.5);
            let eye = pos2(row.max.x - 6.0, y);
            self.hm_ellipse(Rect::from_center_size(eye, vec2(7.0, 4.0)), Color32::TRANSPARENT, Stroke::new(1.0, self.ink()));
            self.p.circle_filled(eye, 1.2, self.ink());
        }
    }
    /// The slide on the left; a pane on the right offers three arrangements, one chosen.
    pub(super) fn hm_design_ideas(&self) {
        let a = self.hm_slide(4.0, 22.0, 100.0, 56.0);
        self.hm_title(a, self.ink());
        self.picture(Rect::from_min_size(pos2(a.min.x + 2.0, a.min.y + 10.0), vec2(40.0, 28.0)));
        self.text_lines(Rect::from_min_max(pos2(a.max.x - 44.0, a.min.y + 10.0), a.max), true, &[40.0, 30.0, 36.0], 7.0, self.grey());
        let pane = self.pane(true, 64.0, 0);
        for i in 0..3 {
            let k = self.reveal(0.1 + 0.2 * i as f32, 0.3 + 0.2 * i as f32);
            if k <= 0.0 {
                continue;
            }
            let r = Rect::from_min_size(pos2(pane.min.x + 8.0, pane.min.y + 17.0 + i as f32 * 27.0), vec2(48.0, 24.0));
            let alpha = k;
            self.p.rect_filled(r, 1.0, Color32::WHITE.linear_multiply(alpha));
            let inner = r.shrink(3.0);
            match i {
                0 => {
                    self.p.rect_filled(
                        Rect::from_min_size(inner.min, vec2(inner.width() * 0.45, inner.height())),
                        0.0,
                        self.accent().linear_multiply(0.5 * alpha),
                    );
                    self.text_lines(
                        Rect::from_min_max(pos2(inner.max.x - inner.width() * 0.45, inner.min.y - 2.0), inner.max),
                        true,
                        &[16.0, 12.0],
                        5.0,
                        self.grey().linear_multiply(alpha),
                    );
                }
                1 => {
                    self.p.rect_filled(
                        Rect::from_min_size(inner.min, vec2(inner.width(), inner.height() * 0.5)),
                        0.0,
                        self.accent().linear_multiply(0.5 * alpha),
                    );
                    self.text_lines(
                        Rect::from_min_max(pos2(inner.min.x, inner.center().y), inner.max),
                        true,
                        &[20.0],
                        5.0,
                        self.grey().linear_multiply(alpha),
                    );
                }
                _ => {
                    self.p.rect_filled(inner, 0.0, self.accent().linear_multiply(0.25 * alpha));
                    self.line(
                        pos2(inner.max.x - 2.0, inner.center().y),
                        pos2(inner.max.x - 18.0, inner.center().y),
                        self.ink().linear_multiply(alpha),
                        2.0,
                    );
                }
            }
            let chosen = i == 1 && self.reveal(0.75, 0.95) > 0.0;
            self.p.rect_stroke(
                r,
                1.0,
                Stroke::new(if chosen { 1.8 } else { 1.0 }, if chosen { self.accent() } else { self.t.border_strong.linear_multiply(alpha) }),
                egui::StrokeKind::Inside,
            );
        }
    }

    // ---- Send (Craft exchange) ----

    /// A tall page at `x, y` of `w × h` (white, grey border); returns its text area.
    fn hm_page(&self, x: f32, y: f32, w: f32, h: f32) -> Rect {
        let r = Rect::from_min_size(self.at(x, y), vec2(w, h));
        self.p.rect_filled(r, 1.5, Color32::WHITE);
        self.p.rect_stroke(r, 1.5, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        r.shrink2(vec2(6.0, 6.0))
    }
    /// A slide's lines flow into a tall page: the arrow grows, then the page fills from the top
    /// with a heading (the slide's title) and the body lines, right-to-left.
    pub(super) fn hm_send_word(&self) {
        let a = self.hm_slide(6.0, 30.0, 70.0, 40.0);
        self.hm_title(a, self.ink());
        self.hm_body(a, 3, 7.0, self.grey());
        let go = self.reveal(0.0, 0.25);
        if go > 0.0 {
            self.arrow_right(self.at(80.0, 52.0), self.at(80.0 + 16.0 * go, 52.0));
        }
        let page = self.hm_page(104.0, 8.0, 62.0, 88.0);
        let k = self.reveal(0.25, 0.8);
        let n = (1.0 + 4.0 * k).floor() as usize;
        if n >= 1 {
            let y = page.min.y + 4.0;
            let w = page.width() * 0.6;
            self.line(pos2(page.max.x - w, y), pos2(page.max.x, y), self.accent(), 3.0);
        }
        if n >= 2 {
            let area = Rect::from_min_max(pos2(page.min.x, page.min.y + 10.0), page.max);
            let widths: Vec<f32> = [1.0, 0.92, 0.96, 0.6].iter().take(n - 1).map(|f| area.width() * f).collect();
            self.text_lines(area, true, &widths, 10.0, self.grey());
        }
    }
    /// A slide's table drops into a grid: the table slides down and right into a sheet whose
    /// cells light up as it lands.
    pub(super) fn hm_send_grid(&self) {
        let a = self.hm_slide(6.0, 30.0, 70.0, 40.0);
        self.hm_title(a, self.ink());
        let tbl = Rect::from_min_max(pos2(a.min.x + 2.0, a.min.y + 9.0), pos2(a.max.x - 2.0, a.max.y - 1.0));
        self.table(tbl, 3, 3);
        let go = self.reveal(0.0, 0.25);
        if go > 0.0 {
            self.arrow_right(self.at(80.0, 52.0), self.at(80.0 + 16.0 * go, 52.0));
        }
        let sheet = Rect::from_min_size(self.at(102.0, 14.0), vec2(66.0, 76.0));
        self.p.rect_filled(sheet, 1.5, Color32::WHITE);
        self.p.rect_stroke(sheet, 1.5, self.stroke(self.t.border_strong), egui::StrokeKind::Inside);
        self.grid(sheet.shrink(3.0), 7, 3, self.t.border);
        let k = self.reveal(0.25, 0.8);
        if k > 0.0 {
            let target = Rect::from_min_size(sheet.min + vec2(3.0, 3.0), vec2(sheet.width() - 6.0, (sheet.height() - 6.0) * 3.0 / 7.0));
            let from = tbl.translate(vec2(0.0, -20.0));
            let r = Rect::from_min_max(from.min + (target.min - from.min) * k, from.max + (target.max - from.max) * k);
            self.table(r, 3, 3);
            if k >= 1.0 {
                self.p.rect_stroke(r, 0.0, Stroke::new(1.5, self.accent()), egui::StrokeKind::Inside);
            }
        }
    }
    /// A page's paragraphs fly into slides: the heading becomes a slide title and the lines
    /// settle as its body; a long page spills onto a second slide.
    pub(super) fn hm_receive_craft(&self) {
        let page = self.hm_page(6.0, 8.0, 62.0, 88.0);
        let hy = page.min.y + 4.0;
        let hw = page.width() * 0.6;
        self.line(pos2(page.max.x - hw, hy), pos2(page.max.x, hy), self.ink(), 3.0);
        let area = Rect::from_min_max(pos2(page.min.x, page.min.y + 10.0), page.max);
        let widths = [1.0, 0.92, 0.96, 0.9, 0.94, 0.6];
        let k = self.reveal(0.2, 0.8);
        let go = self.reveal(0.0, 0.2);
        if go > 0.0 {
            self.arrow_right(self.at(72.0, 52.0), self.at(72.0 + 16.0 * go, 52.0));
        }
        let s1 = self.hm_slide(96.0, 10.0, 72.0, 40.0);
        let s2 = self.hm_slide(96.0, 56.0, 72.0, 40.0);
        self.hm_title(s1, self.accent());
        self.hm_title(s2, self.accent());
        for (i, f) in widths.iter().enumerate() {
            let y0 = area.min.y + 4.0 + i as f32 * 10.0;
            let w0 = area.width() * f;
            let (slide, row) = if i < 3 { (s1, i) } else { (s2, i - 3) };
            let y1 = slide.min.y + 10.0 + row as f32 * 7.0;
            let w1 = slide.width() * 0.8 * f;
            let y = y0 + (y1 - y0) * k;
            let x1 = area.max.x + (slide.max.x - area.max.x) * k;
            let w = w0 + (w1 - w0) * k;
            let c = if k >= 1.0 { self.grey() } else { self.accent() };
            self.line(pos2(x1 - w, y), pos2(x1, y), c, 1.5);
        }
        if k >= 1.0 {
            self.label(pos2(s2.min.x, s2.min.y + 3.0), "(2)", 6.0, self.grey(), Align2::LEFT_CENTER);
        }
    }
}
