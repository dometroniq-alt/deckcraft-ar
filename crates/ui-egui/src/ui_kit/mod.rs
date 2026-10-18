//! DeckCraft AR's UI kit: **one place** that maps every ribbon button and menu item to what the
//! user sees of it — a thumbnail (a drawn picture of what it does, played as a little film) and a
//! plain-words explanation with an example, in English and Arabic. The tooltips, the menu rows,
//! the Developer tab's gallery and the tests all read this registry. The module is the same one
//! WordCraft AR ships (`crates/ui-egui/src/ui_kit/` there); only [`draw::Palette::from_tokens`]
//! and the tab files are this app's.
//!
//! Keys: a ribbon button is keyed by its English tooltip text, the one string every widget already
//! carries; a menu item by its command id plus the parameter that picks its variant, see [`key`].

use serde_json::Value;

pub mod draw;
pub mod tooltip;
pub use draw::{Dir, Draw, Palette, SIZE};

pub mod animations;
pub mod design;
pub mod file;
pub mod home;
pub mod insert;
pub mod misc;
pub mod review;
pub mod slideshow;
pub mod transitions;
pub mod view;

/// One button or menu item as the user sees it: its key, what it does in one or two plain
/// sentences ending with an example (English source; the Arabic catalog translates it), and
/// how to draw it.
pub struct Entry {
    pub key: &'static str,
    pub explain: &'static str,
    pub draw: fn(&Draw),
}

impl Entry {
    pub const fn new(key: &'static str, explain: &'static str, draw: fn(&Draw)) -> Self {
        Self { key, explain, draw }
    }
}

/// Every entry of every tab, in ribbon order.
pub fn entries() -> impl Iterator<Item = &'static Entry> {
    [
        file::ENTRIES,
        home::ENTRIES,
        insert::ENTRIES,
        design::ENTRIES,
        transitions::ENTRIES,
        animations::ENTRIES,
        slideshow::ENTRIES,
        review::ENTRIES,
        view::ENTRIES,
        misc::ENTRIES,
    ]
    .into_iter()
    .flatten()
}

/// The entry for a key; a menu variant without a picture of its own shows the command's.
pub fn entry(key: &str) -> Option<&'static Entry> {
    entries().find(|e| e.key == key).or_else(|| {
        let id = key.split(':').next().unwrap_or(key);
        (id != key).then(|| entries().find(|e| e.key == id)).flatten()
    })
}

/// Parameters whose string value names a menu item's variant, in priority order.
pub const VARIANT_PARAMS: &[&str] = &[
    "name",
    "value",
    "kind",
    "preset",
    "mode",
    "scope",
    "direction",
    "position",
    "style",
    "format",
    "rule",
    "level",
    "layout",
    "type",
    "what",
    "op",
];

/// The key of a menu item: its command id plus `:variant` from the first string parameter in
/// [`VARIANT_PARAMS`] (or `:on`/`:off`/`:remove` for those flags), e.g. `home.paste:values`.
pub fn key(id: &str, params: &Value) -> String {
    let variant = VARIANT_PARAMS
        .iter()
        .find_map(|k| params.get(*k).and_then(Value::as_str))
        .map(str::to_string)
        .or_else(|| ["on", "off", "remove"].iter().find(|f| params.get(**f).and_then(Value::as_bool) == Some(true)).map(|f| f.to_string()));
    match variant {
        Some(v) => format!("{id}:{v}"),
        None => id.to_string(),
    }
}

/// Text that is part of a drawing (a thumbnail's sample words): never translated.
pub fn artwork(s: &str) -> String {
    crate::i18n::artwork(s)
}

/// An explanation in `lang`, English when the catalog has no translation.
pub fn explain(lang: crate::i18n::Lang, text: &str) -> &str {
    crate::i18n::explain(lang, text)
}

/// Draw the thumbnail for `key` scaled into `target` (a menu row's preview): the drawing is
/// made at full [`SIZE`] and its shapes are transformed down. False when there is no entry.
pub fn paint_scaled(p: &egui::Painter, target: egui::Rect, key: &str, t: &Palette) -> bool {
    let Some(e) = entry(key) else { return false };
    let scale = (target.width() / SIZE.x).min(target.height() / SIZE.y);
    let full = egui::Rect::from_min_size(egui::Pos2::ZERO, SIZE);
    let painter = p.with_clip_rect(egui::Rect::EVERYTHING);
    let layer = painter.layer_id();
    let start = p.ctx().graphics_mut(|g| g.entry(layer).next_idx());
    painter.rect_filled(full, 4.0 / scale, t.panel);
    (e.draw)(&Draw::new(&painter, full, t));
    let end = p.ctx().graphics_mut(|g| g.entry(layer).next_idx());
    let clip = target.intersect(p.clip_rect());
    p.ctx().graphics_mut(|g| {
        let list = g.entry(layer);
        list.transform_range(start, end, egui::emath::TSTransform::new(target.min.to_vec2(), scale));
        for i in start.0..end.0 {
            list.mutate_shape(egui::layers::ShapeIdx(i), |cs| cs.clip_rect = clip);
        }
    });
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_are_unique_and_explanations_are_sentences() {
        let mut seen = std::collections::HashSet::new();
        for e in entries() {
            assert!(seen.insert(e.key), "{}: listed twice", e.key);
            assert!(e.explain.ends_with('.') && e.explain.split_whitespace().count() >= 4, "{}: one full sentence", e.key);
        }
    }

    /// Every thumbnail draws, still and at a few points of its film, without a panic.
    #[test]
    fn every_thumbnail_draws_at_every_phase() {
        let ctx = egui::Context::default();
        let mut out = ctx.run_ui(Default::default(), |ui| {
            let t = Palette::get(ui.ctx());
            for e in entries() {
                for phase in [0.0, 0.25, 0.5, 0.75, 1.0] {
                    let r = egui::Rect::from_min_size(egui::Pos2::ZERO, SIZE);
                    (e.draw)(&Draw::animated(ui.painter(), r, &t, phase));
                }
            }
        });
        out.textures_delta.clear();
    }

    #[test]
    fn keys_pick_the_variant() {
        assert_eq!(key("edit.paste", &serde_json::json!({"mode": "values"})), "edit.paste:values");
        assert_eq!(key("edit.paste", &serde_json::json!({})), "edit.paste");
    }
}
