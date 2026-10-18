//! DeckCraft AR — interface translation. Every string the interface shows goes through
//! [`tl!`] (or [`t`]); the catalogs are TSV files (`<context>\t<English>\t<translation>`) parsed
//! once per language ([`catalog`]). Command ids, file formats and the presentation's own text
//! are never translated. The module is the one WordCraft AR uses, cut down to what DeckCraft
//! needs; a new language is one `.tsv` file and one line in [`LANGUAGES`].
//!
//! The interface language is a preference (`UiState::language`, `None` = follow the system),
//! applied each frame with [`set_current`]. The lookup is a thread-local read, cheap enough for
//! every label of every frame.

use std::cell::Cell;
use std::sync::OnceLock;

pub mod catalog;
use catalog::Catalog;

/// Translate an interface string in the current language (a `&'static str` or `&str` in,
/// a `&str` out; English when the catalog has no entry).
#[macro_export]
macro_rules! tl {
    ($s:expr) => {
        $crate::i18n::t($s)
    };
}

/// One interface language.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Lang(usize);

pub struct LangInfo {
    pub code: &'static str,
    pub name: &'static str,
    /// The catalog's TSV source (empty for English).
    pub source: &'static str,
    /// Right-to-left interface.
    pub rtl: bool,
    catalog: OnceLock<Catalog>,
}

/// The languages the interface ships, English first.
pub static LANGUAGES: [LangInfo; 2] = [
    LangInfo { code: "en", name: "English", source: "", rtl: false, catalog: OnceLock::new() },
    LangInfo { code: "ar", name: "العربية", source: include_str!("ar.tsv"), rtl: true, catalog: OnceLock::new() },
];

impl Lang {
    pub const EN: Lang = Lang(0);
    pub const AR: Lang = Lang(1);

    pub fn all() -> impl Iterator<Item = Lang> {
        (0..LANGUAGES.len()).map(Lang)
    }
    pub fn info(self) -> &'static LangInfo {
        LANGUAGES.get(self.0).unwrap_or(&LANGUAGES[0])
    }
    pub fn code(self) -> &'static str {
        self.info().code
    }
    pub fn name(self) -> &'static str {
        self.info().name
    }
    /// The interface runs right to left in this language.
    pub fn is_rtl(self) -> bool {
        self.info().rtl
    }
    pub fn is_arabic(self) -> bool {
        self.code() == "ar"
    }
    /// A language by its code (`ar`, `ar-IQ`, `en-US`).
    pub fn from_code(code: &str) -> Option<Lang> {
        let base = code.trim().split(['-', '_']).next()?.to_ascii_lowercase();
        (0..LANGUAGES.len()).map(Lang).find(|l| l.code() == base)
    }
    pub fn catalog(self) -> &'static Catalog {
        self.info().catalog.get_or_init(|| Catalog::parse_with(self.info().source, self.is_rtl()).0)
    }
    /// The system's language when the interface ships it, else English.
    pub fn system() -> Lang {
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::env::var("LANG")
                .ok()
                .and_then(|v| Lang::from_code(&v))
                .or_else(|| sys_locale::get_locale().and_then(|tag| Lang::from_code(&tag)))
                .unwrap_or(Lang::EN)
        }
        #[cfg(target_arch = "wasm32")]
        {
            Lang::EN
        }
    }
}

thread_local! {
    static CURRENT: Cell<Lang> = const { Cell::new(Lang(0)) };
}

/// The language the interface is drawn in this frame (per thread).
pub fn current() -> Lang {
    CURRENT.with(|c| c.get())
}

/// Set the language for the frames to come (the app calls it once per frame from its prefs).
pub fn set_current(lang: Lang) {
    CURRENT.with(|c| c.set(lang));
}

/// `s` in the current language; strings without a translation come back unchanged.
pub fn t(s: &str) -> &str {
    tr(current(), s)
}

/// `s` in `lang`.
pub fn tr(lang: Lang, s: &str) -> &str {
    if lang == Lang::EN {
        return s;
    }
    lang.catalog().plain(s).unwrap_or(s)
}

/// `s` in the current language as worded for `context` (one of `catalog::CONTEXTS`).
pub fn tc<'a>(context: &str, s: &'a str) -> &'a str {
    let lang = current();
    if lang == Lang::EN {
        return s;
    }
    lang.catalog().in_context(context, s).unwrap_or(s)
}

/// Whether `lang` has a translation of `s`.
pub fn has(lang: Lang, s: &str) -> bool {
    lang != Lang::EN && lang.catalog().plain(s).is_some()
}

/// Fill `{name}` placeholders of a (translated) template, then put the runs of a right-to-left
/// string in the order egui draws them correctly.
pub fn fmt(template: &str, args: &[(&str, &str)]) -> String {
    let out = fill(template, args);
    if current().is_rtl() { rtl_display(&out) } else { out }
}

/// A visual-help explanation ([`crate::ui_kit`]) in `lang`: translated where the catalog has
/// it, English otherwise — in logical order, to be drawn with [`crate::rtl_text::paragraph`],
/// which wraps it and puts each line in the renderer's order.
pub fn explain(lang: Lang, s: &str) -> &str {
    if lang == Lang::EN {
        return s;
    }
    lang.catalog().logical(s).unwrap_or(s)
}

/// Text that is part of a drawing (a thumbnail's sample words): never translated, but put in
/// the order the renderer draws right-to-left text correctly ([`rtl_display`]).
pub fn artwork(s: &str) -> String {
    rtl_display(s)
}

/// `s` with its runs in the order egui draws a right-to-left label correctly: egui lays a
/// label's runs out left to right in logical order and shapes each on its own, so "الصفحة 1 من
/// 2" would show its words right to left but the sequence *word, 1, word, 2* left to right.
/// UAX #9's visual run order, applied to the runs of one line, gives a string egui draws as the
/// bidi algorithm would. Strings with no right-to-left character come back unchanged.
pub fn rtl_display(s: &str) -> String {
    if !s.chars().any(|c| is_rtl_char(c) || is_arabic_digit(c)) {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    for (i, line) in s.split('\n').enumerate() {
        if i > 0 {
            out.push('\n');
        }
        out.push_str(&rtl_display_line(line));
    }
    out
}

fn rtl_display_line(line: &str) -> String {
    use unicode_bidi::{BidiInfo, Level};
    if line.is_empty() {
        return String::new();
    }
    let info = BidiInfo::new(line, Some(Level::rtl()));
    let Some(para) = info.paragraphs.first() else { return line.to_string() };
    let (levels, runs) = info.visual_runs(para, para.range.clone());
    let mut out = String::with_capacity(line.len());
    for r in runs {
        let Some(piece) = line.get(r.clone()) else { continue };
        let ltr = levels.get(r.start).is_some_and(|l| l.is_ltr());
        if ltr {
            if piece.chars().all(|c| is_arabic_digit(c) || c == '٫' || c == '٬') {
                out.extend(piece.chars().rev());
            } else {
                out.push_str(piece);
            }
        } else {
            out.push_str(&rtl_run_for_faces(piece));
        }
    }
    out
}

/// One right-to-left bidi run as egui will draw it. The Arabic interface face has the Arabic
/// letters, the space and a few marks (`.,:!-«»`), so a bracket, a quote, a question mark or a
/// slash in Arabic text comes from the Latin face: epaint then cuts the run into one font run
/// per face and lays those out left to right, undoing the right-to-left order of the pieces.
/// Emitting the pieces in reverse order (the Arabic ones unchanged: the shaper reverses them)
/// restores it; a bracket drawn by the Latin face is mirrored by hand, as the shaper would
/// have in a right-to-left run.
fn rtl_run_for_faces(run: &str) -> String {
    let mut pieces: Vec<(bool, String)> = Vec::new();
    for c in run.chars() {
        let arabic = in_arabic_face(c);
        match pieces.last_mut() {
            Some((a, s)) if *a == arabic => s.push(c),
            _ => pieces.push((arabic, c.to_string())),
        }
    }
    if pieces.len() < 2 {
        return run.to_string();
    }
    let mut out = String::with_capacity(run.len());
    for (arabic, s) in pieces.iter().rev() {
        if *arabic {
            out.push_str(s);
        } else {
            out.extend(s.chars().rev().map(mirror));
        }
    }
    out
}

/// A character the bundled Arabic interface face draws (so it stays in the Arabic run).
fn in_arabic_face(c: char) -> bool {
    is_rtl_char(c) || is_arabic_digit(c) || matches!(c, ' ' | '.' | ',' | ':' | '!' | '-' | '«' | '»' | '\u{a0}' | '\u{2010}')
}

/// The mirrored form of a paired bracket (what the shaper draws for it in right-to-left text).
fn mirror(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        other => other,
    }
}

/// An Arabic-Indic or extended Arabic-Indic digit.
pub fn is_arabic_digit(c: char) -> bool {
    matches!(c as u32, 0x0660..=0x0669 | 0x06F0..=0x06F9)
}

/// A character of a right-to-left script (Arabic, Hebrew, Syriac, Thaana).
pub fn is_rtl_char(c: char) -> bool {
    matches!(c as u32, 0x0590..=0x08FF | 0xFB1D..=0xFDFF | 0xFE70..=0xFEFF)
}

/// Fill `{name}` placeholders of `template` from `args`; unknown names stay as written.
fn fill(template: &str, args: &[(&str, &str)]) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some((before, after_open)) = rest.split_once('{') {
        out.push_str(before);
        let Some((name, after_close)) = after_open.split_once('}') else {
            out.push('{');
            out.push_str(after_open);
            return out;
        };
        match args.iter().find(|(key, _)| *key == name) {
            Some((_, value)) => out.push_str(value),
            None => {
                out.push('{');
                out.push_str(name);
                out.push('}');
            }
        }
        rest = after_close;
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_is_the_identity_and_arabic_looks_up_the_catalog() {
        set_current(Lang::EN);
        assert_eq!(t("Home"), "Home");
        set_current(Lang::AR);
        assert_ne!(t("Home"), "Home", "the Arabic catalog translates the Home tab");
        assert_eq!(t("no such string"), "no such string");
        set_current(Lang::EN);
    }

    #[test]
    fn the_catalog_is_well_formed() {
        for lang in Lang::all().filter(|l| *l != Lang::EN) {
            let (_, errors) = Catalog::parse_with(lang.info().source, lang.is_rtl());
            assert!(errors.is_empty(), "{}: {errors:#?}", lang.code());
        }
    }

    #[test]
    fn placeholders_fill_and_rtl_runs_reorder() {
        assert_eq!(fill("{n} of {m}", &[("n", "1"), ("m", "2")]), "1 of 2");
        assert_eq!(rtl_display("Home"), "Home");
        // One Arabic word, a number, another word: the number stays between them visually.
        let s = rtl_display("الصفحة 1 من 2");
        assert!(s.contains("الصفحة") && s.contains("من"));
    }

    #[test]
    fn mixed_text_is_put_in_the_renderers_order() {
        assert_eq!(rtl_display("١٢٥"), "٥٢١");
        assert_eq!(rtl_display("يكتب (أو الخلايا) كجدول"), " كجدول(أو الخلايا)يكتب ");
        let s = rtl_display("انسخ A1:A3 ثم");
        assert!(s.trim().starts_with("ثم") && s.trim().ends_with("انسخ"), "{s}");
    }

    #[test]
    fn language_codes_resolve() {
        assert_eq!(Lang::from_code("ar-IQ"), Some(Lang::AR));
        assert_eq!(Lang::from_code("en_US.UTF-8"), Some(Lang::EN));
        assert_eq!(Lang::from_code("xx"), None);
    }
}
