//! Craft Exchange (`.craftx`): the JSON hand-off format WordCraft AR, DeckCraft AR and GridCraft AR
//! share (`docs/ar-exchange.md`). It carries a document's content with its Arabic typography —
//! paragraph direction, alignment, kashida level, lists, tables — without going through Office
//! formats.
//!
//! [`write`] turns a presentation into one `slide` block per slide followed by that slide's body
//! in reading order; [`read`] builds a 16:9 presentation from any `.craftx` (one slide per
//! heading or `slide` block, bodies split into continuation slides when they would overflow).
//! Everything in the file is hostile input: every field has a default, every number is checked,
//! every count is capped.

use std::sync::Arc;

use deckcraft_model::table::{Cell as TCell, Row as TRow, Table};
use deckcraft_model::text::{Align, Bullet, ParaProps, Paragraph, Run, RunKind, RunProps, Spacing, TextBody};
use deckcraft_model::{ColorRef, Fill, LayoutType, PhType, Presentation, Rgba, Shape, ShapeKind, Xfrm, defaults, resolve};
use serde_json::{Map, Value, json};

use crate::{FormatError, Result};

/// The format version this module writes and reads.
pub const VERSION: u64 = 1;
/// The file extension.
pub const EXTENSION: &str = "craftx";
/// What this app writes into `from`.
pub const FROM: &str = "deckcraft-ar";

/// Blocks one file may carry (more are dropped).
const MAX_BLOCKS: usize = 100_000;
/// Rows and cells one table may carry.
const MAX_ROWS: usize = 10_000;
const MAX_COLS: usize = 1_000;
/// Runs one paragraph may carry.
const MAX_RUNS: usize = 10_000;
/// Items one list may carry.
const MAX_ITEMS: usize = 100_000;
/// Decoded bytes one image may carry.
const MAX_IMAGE: usize = 64 << 20;
/// Characters one text block may carry (longer ones are cut, never lost: the rest is dropped
/// after the slide shows what fits).
const MAX_CHARS: usize = 1 << 20;

/// Lines a 16:9 slide body holds at 20 pt (the spec's estimate).
pub const LINES_PER_BODY: usize = 11;
/// Characters per line of body text, Latin and Arabic (the spec's estimates).
pub const CHARS_PER_LINE: usize = 70;
pub const CHARS_PER_LINE_ARABIC: usize = 60;
/// Table rows a slide holds at most.
pub const ROWS_PER_SLIDE: usize = 8;

/// Body text size the overflow estimate assumes, in points.
const BODY_PT: f64 = 20.0;

// ---------------------------------------------------------------------------------------------
// The document model
// ---------------------------------------------------------------------------------------------

/// One run of text with its character formatting (all optional).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CRun {
    pub text: String,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underline: Option<bool>,
    pub size_pt: Option<f64>,
    /// `#RRGGBB`.
    pub color: Option<String>,
    pub font: Option<String>,
}

/// Paragraph alignment: `start` is the right edge of a bidi paragraph.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAlign {
    Start,
    Center,
    End,
    Justify,
    Distribute,
}

impl CAlign {
    fn name(self) -> &'static str {
        match self {
            CAlign::Start => "start",
            CAlign::Center => "center",
            CAlign::End => "end",
            CAlign::Justify => "justify",
            CAlign::Distribute => "distribute",
        }
    }
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "start" => CAlign::Start,
            "center" => CAlign::Center,
            "end" => CAlign::End,
            "justify" => CAlign::Justify,
            "distribute" => CAlign::Distribute,
            _ => return None,
        })
    }
}

/// Kashida (tatweel) stretch of a justified Arabic paragraph.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Kashida {
    #[default]
    None,
    Low,
    Medium,
    High,
}

impl Kashida {
    fn name(self) -> &'static str {
        match self {
            Kashida::None => "none",
            Kashida::Low => "low",
            Kashida::Medium => "medium",
            Kashida::High => "high",
        }
    }
    fn parse(s: &str) -> Self {
        match s {
            "low" => Kashida::Low,
            "medium" => Kashida::Medium,
            "high" => Kashida::High,
            _ => Kashida::None,
        }
    }
}

/// Paragraph formatting.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CPara {
    /// Right-to-left; `None` = the document's `rtl`.
    pub bidi: Option<bool>,
    pub align: Option<CAlign>,
    pub kashida: Kashida,
    pub space_before_pt: Option<f64>,
    pub space_after_pt: Option<f64>,
    /// Multiple of the line (1.15…).
    pub line_spacing: Option<f64>,
    pub indent_pt: Option<f64>,
}

/// Text with its paragraph formatting (a heading, a paragraph, a list item, a cell).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CText {
    pub runs: Vec<CRun>,
    pub para: CPara,
}

impl CText {
    pub fn plain(text: &str) -> Self {
        CText { runs: vec![CRun { text: text.to_string(), ..Default::default() }], para: CPara::default() }
    }
    /// The concatenated plain text.
    pub fn text(&self) -> String {
        self.runs.iter().map(|r| r.text.as_str()).collect()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CItem {
    pub text: CText,
    pub level: u8,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CCell {
    pub text: CText,
    pub align: Option<CAlign>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Block {
    Heading { level: u8, text: CText },
    Paragraph(CText),
    List { ordered: bool, items: Vec<CItem> },
    Table { rtl: Option<bool>, header: bool, widths: Vec<f64>, rows: Vec<Vec<CCell>> },
    Image { name: String, png: Vec<u8>, width_pt: Option<f64>, height_pt: Option<f64> },
    Slide { title: String, notes: String },
    PageBreak,
}

/// A whole `.craftx` document.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Doc {
    pub from: Option<String>,
    pub title: Option<String>,
    pub rtl: bool,
    pub font: Option<String>,
    pub blocks: Vec<Block>,
}

// ---------------------------------------------------------------------------------------------
// JSON ↔ model
// ---------------------------------------------------------------------------------------------

/// Does this look like a `.craftx` file (JSON with a `craftx` key near the start)?
pub fn sniff(bytes: &[u8]) -> bool {
    let head: &[u8] = bytes.get(..bytes.len().min(4096)).unwrap_or(bytes);
    let text = String::from_utf8_lossy(head);
    let t = text.trim_start_matches('\u{feff}').trim_start();
    t.starts_with('{') && t.contains("\"craftx\"")
}

fn finite(v: Option<&Value>) -> Option<f64> {
    v.and_then(Value::as_f64).filter(|f| f.is_finite())
}

fn pt(v: Option<&Value>) -> Option<f64> {
    finite(v).map(|f| f.clamp(-4032.0, 4032.0))
}

fn string(v: Option<&Value>) -> Option<String> {
    v.and_then(Value::as_str).map(|s| cap_chars(s, MAX_CHARS))
}

fn cap_chars(s: &str, max: usize) -> String {
    if s.chars().count() <= max { s.to_string() } else { s.chars().take(max).collect() }
}

fn run_of(v: &Value) -> Option<CRun> {
    let o = v.as_object()?;
    Some(CRun {
        text: string(o.get("text")).unwrap_or_default(),
        bold: o.get("bold").and_then(Value::as_bool),
        italic: o.get("italic").and_then(Value::as_bool),
        underline: o.get("underline").and_then(Value::as_bool),
        size_pt: finite(o.get("size_pt")).filter(|s| *s > 0.0).map(|s| s.clamp(1.0, 4000.0)),
        color: o.get("color").and_then(Value::as_str).and_then(Rgba::from_hex).map(|c| format!("#{}", c.hex())),
        font: string(o.get("font")).filter(|f| !f.is_empty() && f.chars().count() <= 128),
    })
}

fn para_of(v: Option<&Value>) -> CPara {
    let Some(o) = v.and_then(Value::as_object) else { return CPara::default() };
    CPara {
        bidi: o.get("bidi").and_then(Value::as_bool),
        align: o.get("align").and_then(Value::as_str).and_then(CAlign::parse),
        kashida: o.get("kashida").and_then(Value::as_str).map(Kashida::parse).unwrap_or_default(),
        space_before_pt: pt(o.get("space_before_pt")).filter(|v| *v >= 0.0),
        space_after_pt: pt(o.get("space_after_pt")).filter(|v| *v >= 0.0),
        line_spacing: finite(o.get("line_spacing")).filter(|v| *v > 0.0).map(|v| v.clamp(0.1, 10.0)),
        indent_pt: pt(o.get("indent_pt")),
    }
}

/// `runs` when present, else `text` as one run; the paragraph formatting from `para`.
fn text_of(o: &Map<String, Value>) -> CText {
    let runs: Vec<CRun> = match o.get("runs").and_then(Value::as_array) {
        Some(a) if !a.is_empty() => a.iter().take(MAX_RUNS).filter_map(run_of).collect(),
        _ => string(o.get("text")).map(|t| vec![CRun { text: t, ..Default::default() }]).unwrap_or_default(),
    };
    CText { runs, para: para_of(o.get("para")) }
}

fn level_of(v: Option<&Value>, default: u64) -> u8 {
    v.and_then(Value::as_u64).unwrap_or(default).min(9) as u8
}

fn block_of(v: &Value) -> Option<Block> {
    let o = v.as_object()?;
    let kind = o.get("kind").and_then(Value::as_str)?;
    Some(match kind {
        "heading" => Block::Heading { level: level_of(o.get("level"), 1).max(1), text: text_of(o) },
        "paragraph" => Block::Paragraph(text_of(o)),
        "list" => Block::List {
            ordered: o.get("ordered").and_then(Value::as_bool).unwrap_or(false),
            items: o
                .get("items")
                .and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .take(MAX_ITEMS)
                        .filter_map(|i| i.as_object().map(|io| CItem { text: text_of(io), level: level_of(io.get("level"), 0).min(8) }))
                        .collect()
                })
                .unwrap_or_default(),
        },
        "table" => {
            let rows: Vec<Vec<CCell>> = o
                .get("rows")
                .and_then(Value::as_array)
                .map(|a| {
                    a.iter()
                        .take(MAX_ROWS)
                        .filter_map(Value::as_array)
                        .map(|r| {
                            r.iter()
                                .take(MAX_COLS)
                                .map(|c| match c.as_object() {
                                    Some(co) => CCell { text: text_of(co), align: co.get("align").and_then(Value::as_str).and_then(CAlign::parse) },
                                    None => CCell { text: CText::plain(c.as_str().unwrap_or_default()), align: None },
                                })
                                .collect::<Vec<CCell>>()
                        })
                        .collect()
                })
                .unwrap_or_default();
            let widths = o
                .get("widths")
                .and_then(Value::as_array)
                .map(|a| a.iter().take(MAX_COLS).filter_map(|w| finite(Some(w))).filter(|w| *w > 0.0).collect())
                .unwrap_or_default();
            Block::Table {
                rtl: o.get("rtl").and_then(Value::as_bool),
                header: o.get("header").and_then(Value::as_bool).unwrap_or(false),
                widths,
                rows,
            }
        }
        "image" => {
            let png = o.get("png_base64").and_then(Value::as_str).and_then(base64_decode).filter(|b| b.len() <= MAX_IMAGE).unwrap_or_default();
            Block::Image {
                name: string(o.get("name")).filter(|n| !n.is_empty()).unwrap_or_else(|| "image.png".into()),
                png,
                width_pt: pt(o.get("width_pt")).filter(|v| *v > 0.0),
                height_pt: pt(o.get("height_pt")).filter(|v| *v > 0.0),
            }
        }
        "slide" => Block::Slide { title: string(o.get("title")).unwrap_or_default(), notes: string(o.get("notes")).unwrap_or_default() },
        "page_break" => Block::PageBreak,
        _ => return None,
    })
}

/// Parse a `.craftx` text into its document. Unknown blocks and fields are skipped; a file that
/// isn't JSON, or whose JSON has no `craftx` key, is refused.
pub fn parse(json: &str) -> Result<Doc> {
    let v: Value =
        serde_json::from_str(json.trim_start_matches('\u{feff}')).map_err(|e| FormatError::NotOurs(format!("not a Craft exchange file: {e}")))?;
    let o = v.as_object().ok_or_else(|| FormatError::NotOurs("not a Craft exchange file: no object".into()))?;
    if !o.contains_key("craftx") {
        return Err(FormatError::NotOurs("not a Craft exchange file: no `craftx` key".into()));
    }
    let blocks = o.get("blocks").and_then(Value::as_array).map(|a| a.iter().take(MAX_BLOCKS).filter_map(block_of).collect()).unwrap_or_default();
    Ok(Doc {
        from: string(o.get("from")),
        title: string(o.get("title")).filter(|t| !t.trim().is_empty()),
        rtl: o.get("rtl").and_then(Value::as_bool).unwrap_or(false),
        font: string(o.get("font")).filter(|f| !f.trim().is_empty() && f.chars().count() <= 128),
        blocks,
    })
}

fn run_json(r: &CRun) -> Value {
    let mut m = Map::new();
    m.insert("text".into(), json!(r.text));
    if r.bold == Some(true) {
        m.insert("bold".into(), json!(true));
    }
    if r.italic == Some(true) {
        m.insert("italic".into(), json!(true));
    }
    if r.underline == Some(true) {
        m.insert("underline".into(), json!(true));
    }
    if let Some(s) = r.size_pt {
        m.insert("size_pt".into(), json!(s));
    }
    if let Some(c) = &r.color {
        m.insert("color".into(), json!(c));
    }
    if let Some(f) = &r.font {
        m.insert("font".into(), json!(f));
    }
    Value::Object(m)
}

fn para_json(p: &CPara) -> Option<Value> {
    let mut m = Map::new();
    if let Some(b) = p.bidi {
        m.insert("bidi".into(), json!(b));
    }
    if let Some(a) = p.align {
        m.insert("align".into(), json!(a.name()));
    }
    if p.kashida != Kashida::None {
        m.insert("kashida".into(), json!(p.kashida.name()));
    }
    if let Some(v) = p.space_before_pt {
        m.insert("space_before_pt".into(), json!(v));
    }
    if let Some(v) = p.space_after_pt {
        m.insert("space_after_pt".into(), json!(v));
    }
    if let Some(v) = p.line_spacing {
        m.insert("line_spacing".into(), json!(v));
    }
    if let Some(v) = p.indent_pt {
        m.insert("indent_pt".into(), json!(v));
    }
    (!m.is_empty()).then_some(Value::Object(m))
}

fn text_json(m: &mut Map<String, Value>, t: &CText) {
    m.insert("runs".into(), Value::Array(t.runs.iter().map(run_json).collect()));
    m.insert("text".into(), json!(t.text()));
    if let Some(p) = para_json(&t.para) {
        m.insert("para".into(), p);
    }
}

fn block_json(b: &Block) -> Value {
    let mut m = Map::new();
    match b {
        Block::Heading { level, text } => {
            m.insert("kind".into(), json!("heading"));
            m.insert("level".into(), json!(level));
            text_json(&mut m, text);
        }
        Block::Paragraph(text) => {
            m.insert("kind".into(), json!("paragraph"));
            text_json(&mut m, text);
        }
        Block::List { ordered, items } => {
            m.insert("kind".into(), json!("list"));
            m.insert("ordered".into(), json!(ordered));
            let items: Vec<Value> = items
                .iter()
                .map(|i| {
                    let mut im = Map::new();
                    text_json(&mut im, &i.text);
                    im.insert("level".into(), json!(i.level));
                    Value::Object(im)
                })
                .collect();
            m.insert("items".into(), Value::Array(items));
        }
        Block::Table { rtl, header, widths, rows } => {
            m.insert("kind".into(), json!("table"));
            if let Some(r) = rtl {
                m.insert("rtl".into(), json!(r));
            }
            m.insert("header".into(), json!(header));
            if !widths.is_empty() {
                m.insert("widths".into(), json!(widths));
            }
            let rows: Vec<Value> = rows
                .iter()
                .map(|r| {
                    Value::Array(
                        r.iter()
                            .map(|c| {
                                let mut cm = Map::new();
                                text_json(&mut cm, &c.text);
                                if let Some(a) = c.align {
                                    cm.insert("align".into(), json!(a.name()));
                                }
                                Value::Object(cm)
                            })
                            .collect(),
                    )
                })
                .collect();
            m.insert("rows".into(), Value::Array(rows));
        }
        Block::Image { name, png, width_pt, height_pt } => {
            m.insert("kind".into(), json!("image"));
            m.insert("name".into(), json!(name));
            m.insert("png_base64".into(), json!(base64_encode(png)));
            if let Some(w) = width_pt {
                m.insert("width_pt".into(), json!(w));
            }
            if let Some(h) = height_pt {
                m.insert("height_pt".into(), json!(h));
            }
        }
        Block::Slide { title, notes } => {
            m.insert("kind".into(), json!("slide"));
            m.insert("title".into(), json!(title));
            if !notes.is_empty() {
                m.insert("notes".into(), json!(notes));
            }
        }
        Block::PageBreak => {
            m.insert("kind".into(), json!("page_break"));
        }
    }
    Value::Object(m)
}

/// The file's top level, in this order (a struct keeps it; a `Map` would sort `blocks` first and
/// push the `craftx` marker past where [`sniff`] looks).
#[derive(serde::Serialize)]
struct Head<'a> {
    craftx: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    from: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    rtl: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    font: Option<&'a str>,
    blocks: Vec<Value>,
}

/// The document as `.craftx` text.
pub fn to_json(doc: &Doc) -> Result<String> {
    let head = Head {
        craftx: VERSION,
        from: doc.from.as_deref(),
        title: doc.title.as_deref(),
        rtl: doc.rtl,
        font: doc.font.as_deref(),
        blocks: doc.blocks.iter().map(block_json).collect(),
    };
    serde_json::to_string_pretty(&head).map_err(|e| FormatError::Write(e.to_string()))
}

// ---------------------------------------------------------------------------------------------
// Deck → craftx
// ---------------------------------------------------------------------------------------------

/// Is this text mostly Arabic (or another right-to-left script)?
fn is_arabic(text: &str) -> bool {
    deckcraft_model::first_strong_is_rtl(text)
}

/// A hex colour for an explicit run fill.
fn run_color(ctx: &resolve::Ctx, r: &RunProps) -> Option<String> {
    match &r.fill {
        Some(Fill::Solid { color }) => Some(format!("#{}", ctx.color(color, None).hex())),
        _ => None,
    }
}

/// A run's craftx form: only what the run (or its paragraph's `defRPr`) sets explicitly, so a
/// document doesn't inherit slide-sized text.
fn crun(ctx: &resolve::Ctx, para: &Paragraph, run: &Run) -> Option<CRun> {
    let text = match run.kind {
        RunKind::Break => "\n".to_string(),
        _ => run.text.clone(),
    };
    if text.is_empty() {
        return None;
    }
    let mut props = run.props.clone();
    props.inherit(&para.def_props);
    let font =
        props.font.as_deref().map(|f| if f.starts_with('+') { resolve::font_family(ctx, &props) } else { f.to_string() }).filter(|f| !f.is_empty());
    Some(CRun {
        text,
        bold: props.bold.filter(|b| *b),
        italic: props.italic.filter(|b| *b),
        underline: props.underline.as_deref().is_some_and(|u| u != "none").then_some(true),
        size_pt: props.size.filter(|s| s.is_finite() && *s > 0.0),
        color: run_color(ctx, &props),
        font,
    })
}

/// The paragraph formatting craftx carries, from the resolved properties (direction and
/// alignment come down from the layout) and the paragraph's own spacing.
fn cpara(resolved: &ParaProps, own: &ParaProps, text: &str) -> CPara {
    let bidi = resolved.rtl.unwrap_or_else(|| is_arabic(text));
    let (align, kashida) = match resolved.align {
        None => (None, Kashida::None),
        Some(Align::Left) => (Some(if bidi { CAlign::End } else { CAlign::Start }), Kashida::None),
        Some(Align::Right) => (Some(if bidi { CAlign::Start } else { CAlign::End }), Kashida::None),
        Some(Align::Center) => (Some(CAlign::Center), Kashida::None),
        Some(Align::Justify) => (Some(CAlign::Justify), Kashida::None),
        Some(Align::JustLow) => (Some(CAlign::Justify), Kashida::Low),
        Some(Align::JustMedium) => (Some(CAlign::Justify), Kashida::Medium),
        Some(Align::JustHigh) => (Some(CAlign::Justify), Kashida::High),
        Some(Align::Distributed) => (Some(CAlign::Distribute), Kashida::None),
    };
    let pts = |s: &Option<Spacing>| match s {
        Some(Spacing::Pts(v)) if v.is_finite() => Some(*v),
        _ => None,
    };
    CPara {
        bidi: Some(bidi),
        align,
        kashida,
        space_before_pt: pts(&own.space_before),
        space_after_pt: pts(&own.space_after),
        line_spacing: match own.line_spacing {
            Some(Spacing::Pct(v)) if v.is_finite() && v > 0.0 => Some(v),
            _ => None,
        },
        indent_pt: own.indent.filter(|v| v.is_finite()),
    }
}

fn ctext(ctx: &resolve::Ctx, shape: &Shape, para: &Paragraph) -> CText {
    let runs: Vec<CRun> = para.runs.iter().filter_map(|r| crun(ctx, para, r)).collect();
    let text: String = runs.iter().map(|r| r.text.as_str()).collect();
    CText { runs, para: cpara(&resolve::para(ctx, shape, para), &para.props, &text) }
}

/// A cell's text: its paragraphs joined by newlines, the first one's formatting.
fn ccell(ctx: &resolve::Ctx, shape: &Shape, cell: &TCell) -> CCell {
    let mut runs: Vec<CRun> = vec![];
    for (i, p) in cell.text.paragraphs.iter().enumerate() {
        if i > 0 {
            runs.push(CRun { text: "\n".into(), ..Default::default() });
        }
        runs.extend(p.runs.iter().filter_map(|r| crun(ctx, p, r)));
    }
    let text: String = runs.iter().map(|r| r.text.as_str()).collect();
    let para = match cell.text.paragraphs.first() {
        Some(p) => cpara(&resolve::para(ctx, shape, p), &p.props, &text),
        None => CPara { bidi: Some(false), ..Default::default() },
    };
    let align = para.align;
    CCell { text: CText { runs, para }, align }
}

/// Append the blocks of `shapes` (groups flattened) to `out`.
fn shape_blocks(p: &Presentation, ctx: &resolve::Ctx, shapes: &[Shape], out: &mut Vec<Block>, depth: usize) {
    if depth > 32 {
        return;
    }
    for sh in shapes {
        if sh.ph_type().is_some_and(PhType::is_title) {
            continue;
        }
        match &sh.kind {
            ShapeKind::Group { children, .. } => shape_blocks(p, ctx, children, out, depth + 1),
            ShapeKind::Table(t) => {
                let rows: Vec<Vec<CCell>> = t.rows.iter().map(|r| r.cells.iter().map(|c| ccell(ctx, sh, c)).collect()).collect();
                let rtl = rows.first().and_then(|r| r.first()).and_then(|c| c.text.para.bidi);
                let total: f64 = t.cols.iter().filter(|w| w.is_finite()).sum();
                let widths = if total > 0.0 {
                    t.cols.iter().map(|w| if w.is_finite() { (w / total * 1000.0).round() / 1000.0 } else { 0.0 }).collect()
                } else {
                    vec![]
                };
                out.push(Block::Table { rtl, header: t.first_row, widths, rows });
            }
            ShapeKind::Picture { fill } => {
                let Some(m) = p.media(fill.media) else { continue };
                let png = m.content_type.starts_with("image/png") || m.data.starts_with(b"\x89PNG\r\n\x1a\n");
                if !png || m.data.is_empty() {
                    continue;
                }
                let x = resolve::xfrm(ctx, sh);
                let name = if m.name.is_empty() { "picture.png".to_string() } else { m.name.clone() };
                out.push(Block::Image {
                    name,
                    png: m.data.as_ref().clone(),
                    width_pt: (x.w.is_finite() && x.w > 0.0).then_some(x.w),
                    height_pt: (x.h.is_finite() && x.h > 0.0).then_some(x.h),
                });
            }
            _ => {
                let Some(t) = &sh.text else { continue };
                let mut list: Option<(bool, Vec<CItem>)> = None;
                for para in &t.paragraphs {
                    if para.is_empty() {
                        continue;
                    }
                    let resolved = resolve::para(ctx, sh, para);
                    let bullet = match &resolved.bullet {
                        Some(Bullet::None) | None => None,
                        Some(Bullet::AutoNum { .. }) => Some(true),
                        Some(_) => Some(false),
                    };
                    let text = ctext(ctx, sh, para);
                    match bullet {
                        Some(ordered) => match &mut list {
                            Some((o, items)) if *o == ordered => items.push(CItem { text, level: para.level.min(8) }),
                            _ => {
                                if let Some((o, items)) = list.take() {
                                    out.push(Block::List { ordered: o, items });
                                }
                                list = Some((ordered, vec![CItem { text, level: para.level.min(8) }]));
                            }
                        },
                        None => {
                            if let Some((o, items)) = list.take() {
                                out.push(Block::List { ordered: o, items });
                            }
                            out.push(Block::Paragraph(text));
                        }
                    }
                }
                if let Some((o, items)) = list.take() {
                    out.push(Block::List { ordered: o, items });
                }
            }
        }
    }
}

/// The presentation as a craftx document: one `slide` block per slide (title and speaker notes),
/// then the slide's body in reading order.
pub fn to_doc(deck: &Presentation, title: &str) -> Doc {
    let mut blocks = vec![];
    for slide in &deck.slides {
        blocks.push(Block::Slide { title: slide.title(), notes: slide.notes.text().trim().to_string() });
        if let Some(ctx) = resolve::Ctx::for_slide(deck, slide) {
            shape_blocks(deck, &ctx, &slide.shapes, &mut blocks, 0);
        }
    }
    let (mut rtl_n, mut n) = (0usize, 0usize);
    for b in &blocks {
        let texts: Vec<&CText> = match b {
            Block::Paragraph(t) | Block::Heading { text: t, .. } => vec![t],
            Block::List { items, .. } => items.iter().map(|i| &i.text).collect(),
            _ => vec![],
        };
        for t in texts {
            n += 1;
            if t.para.bidi == Some(true) {
                rtl_n += 1;
            }
        }
    }
    let font = deck.masters.first().map(|m| m.theme.fonts.minor.latin.clone()).filter(|f| !f.is_empty() && !f.starts_with('+'));
    Doc { from: Some(FROM.into()), title: Some(title.to_string()), rtl: n > 0 && rtl_n * 2 > n, font, blocks }
}

/// The presentation as `.craftx` text.
pub fn write(deck: &Presentation, title: &str) -> Result<String> {
    to_json(&to_doc(deck, title))
}

// ---------------------------------------------------------------------------------------------
// craftx → Deck
// ---------------------------------------------------------------------------------------------

/// Lines a text of `chars` characters takes on a slide (at least one).
fn lines_for(text: &str) -> usize {
    let n = text.chars().filter(|c| *c != '\n').count();
    let per = if is_arabic(text) { CHARS_PER_LINE_ARABIC } else { CHARS_PER_LINE };
    n.div_ceil(per).max(1) + text.matches('\n').count()
}

/// How a craftx paragraph lands in a placeholder.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ParaKind {
    /// A plain paragraph: no bullet.
    Plain,
    /// A list item at its level with the layout's bullet.
    Bullet,
    /// A numbered list item.
    Numbered,
    /// A heading inside a body: bold, no bullet.
    Bold,
}

fn model_align(p: &CPara, bidi: bool) -> Option<Align> {
    let justified = match p.kashida {
        Kashida::None => None,
        Kashida::Low => Some(Align::JustLow),
        Kashida::Medium => Some(Align::JustMedium),
        Kashida::High => Some(Align::JustHigh),
    };
    match p.align {
        Some(CAlign::Start) => Some(if bidi { Align::Right } else { Align::Left }),
        Some(CAlign::End) => Some(if bidi { Align::Left } else { Align::Right }),
        Some(CAlign::Center) => Some(Align::Center),
        Some(CAlign::Justify) => Some(justified.unwrap_or(Align::Justify)),
        Some(CAlign::Distribute) => Some(Align::Distributed),
        None => justified,
    }
}

fn model_run(r: &CRun) -> Run {
    let props = RunProps {
        bold: r.bold.filter(|b| *b),
        italic: r.italic.filter(|b| *b),
        underline: (r.underline == Some(true)).then(|| "sng".to_string()),
        size: r.size_pt,
        fill: r.color.as_deref().and_then(Rgba::from_hex).map(|c| Fill::solid(ColorRef::rgb(c))),
        font: r.font.clone(),
        ..Default::default()
    };
    Run::with(r.text.replace(['\r', '\n', '\u{b}'], " "), props)
}

/// A model paragraph for a craftx text: direction, alignment and kashida kept, the kind's bullet.
fn model_paragraph(t: &CText, doc_rtl: bool, kind: ParaKind, level: u8) -> Paragraph {
    let bidi = t.para.bidi.unwrap_or(doc_rtl);
    let mut props = ParaProps {
        rtl: if bidi || t.para.bidi.is_some() { Some(bidi) } else { None },
        align: model_align(&t.para, bidi),
        space_before: t.para.space_before_pt.map(Spacing::Pts),
        space_after: t.para.space_after_pt.map(Spacing::Pts),
        line_spacing: t.para.line_spacing.map(Spacing::Pct),
        indent: t.para.indent_pt,
        ..Default::default()
    };
    match kind {
        ParaKind::Plain | ParaKind::Bold => {
            props.bullet = Some(Bullet::None);
            props.margin_left = Some(0.0);
            if props.indent.is_none() {
                props.indent = Some(0.0);
            }
        }
        ParaKind::Numbered => props.bullet = Some(Bullet::AutoNum { scheme: "arabicPeriod".into(), start_at: 1 }),
        ParaKind::Bullet => {}
    }
    let mut runs: Vec<Run> = t.runs.iter().filter(|r| !r.text.is_empty()).map(model_run).collect();
    if kind == ParaKind::Bold {
        for r in &mut runs {
            r.props.bold = Some(true);
        }
    }
    let mut para = Paragraph { level: level.min(8), props, runs, ..Default::default() };
    para.def_props.size = Some(BODY_PT);
    para.normalize();
    para
}

/// One thing a slide body holds, with the lines it takes.
enum Piece {
    Para(Paragraph, usize),
    Table { rtl: bool, header: bool, widths: Vec<f64>, rows: Vec<Vec<CCell>> },
    Image { name: String, png: Vec<u8>, width_pt: Option<f64>, height_pt: Option<f64> },
}

/// A slide in the making: its title, the pieces of its body, its notes.
struct Building {
    title: String,
    notes: String,
    pieces: Vec<Piece>,
    /// Started by a `slide` block (then headings of level ≥ 2 are body text).
    from_slide: bool,
}

/// The reader's state and output.
struct Builder<'a> {
    p: &'a mut Presentation,
    doc_rtl: bool,
    made: usize,
    /// The deck can take no more slides (its ids are used up).
    full: bool,
}

/// `(x, y, w, h)` of the body area of the Title and Content layout, else the spec's proportions.
fn body_area(p: &Presentation) -> Xfrm {
    let from_layout = defaults::layout_of_kind(p, LayoutType::TitleAndContent)
        .and_then(|id| p.layout(id))
        .and_then(|(_, l)| l.shapes.iter().find(|s| s.ph_type().is_some_and(|k| matches!(k, PhType::Obj | PhType::Body))).and_then(|s| s.xfrm));
    from_layout.unwrap_or_else(|| {
        let (sx, sy) = (p.slide_size.width / 960.0, p.slide_size.height / 540.0);
        Xfrm::new(60.0 * sx, 144.0 * sy, 840.0 * sx, 343.0 * sy)
    })
}

/// Width and height of a PNG from its header, when the bytes are one.
pub fn png_size(bytes: &[u8]) -> Option<(u32, u32)> {
    if !bytes.starts_with(b"\x89PNG\r\n\x1a\n") || bytes.get(12..16)? != b"IHDR" {
        return None;
    }
    let be = |at: usize| bytes.get(at..at + 4).and_then(|b| <[u8; 4]>::try_from(b).ok()).map(u32::from_be_bytes);
    let (w, h) = (be(16)?, be(20)?);
    (w > 0 && h > 0).then_some((w, h))
}

impl Builder<'_> {
    fn title_text(&self, title: &str) -> TextBody {
        let mut tb = TextBody::from_text(title);
        let rtl = self.doc_rtl || is_arabic(title);
        for para in &mut tb.paragraphs {
            if rtl {
                para.props.rtl = Some(true);
                para.props.align = Some(Align::Right);
            }
        }
        tb
    }

    fn notes_text(&self, notes: &str) -> TextBody {
        if notes.trim().is_empty() {
            return TextBody::default();
        }
        let mut tb = TextBody::from_text(notes);
        if self.doc_rtl || is_arabic(notes) {
            for para in &mut tb.paragraphs {
                para.props.rtl = Some(true);
                para.props.align = Some(Align::Right);
            }
        }
        tb
    }

    /// A slide of `layout` titled `title`; `None` once the deck can't grow.
    fn new_slide(&mut self, layout: LayoutType, title: &str, notes: &str) -> Option<deckcraft_model::Slide> {
        if self.full {
            return None;
        }
        let lay = defaults::layout_of_kind(self.p, layout).or_else(|| defaults::layout_of_kind(self.p, LayoutType::TitleAndContent))?;
        let Some(mut s) = defaults::new_slide(self.p, lay) else {
            self.full = true;
            return None;
        };
        let tt = self.title_text(title);
        if let Some(t) = s.shapes.iter_mut().find(|sh| sh.ph_type().is_some_and(PhType::is_title)) {
            t.text = Some(tt);
        }
        s.notes = self.notes_text(notes);
        Some(s)
    }

    fn push(&mut self, s: deckcraft_model::Slide) {
        self.p.slides.push(Arc::new(s));
        self.made += 1;
    }

    /// A text slide holding `paras`.
    fn text_slide(&mut self, title: &str, notes: &str, paras: Vec<Paragraph>) {
        let Some(mut s) = self.new_slide(LayoutType::TitleAndContent, title, notes) else { return };
        if let Some(b) = s.shapes.iter_mut().find(|sh| sh.ph_type().is_some_and(|k| matches!(k, PhType::Obj | PhType::Body))) {
            b.text = Some(TextBody { paragraphs: paras, ..Default::default() });
        }
        self.push(s);
    }

    /// A Title Only slide with a table of `rows` filling the body area.
    fn table_slide(&mut self, title: &str, notes: &str, rtl: bool, header: bool, widths: &[f64], rows: &[Vec<CCell>]) {
        let Some(mut s) = self.new_slide(LayoutType::TitleOnly, title, notes) else { return };
        let area = body_area(self.p);
        let ncols = rows.iter().map(Vec::len).max().unwrap_or(0).clamp(1, MAX_COLS);
        let nrows = rows.len().max(1);
        let total: f64 = widths.iter().take(ncols).sum();
        let cols: Vec<f64> = if widths.len() >= ncols && total > 0.0 {
            widths.iter().take(ncols).map(|w| (w / total * area.w).max(1.0)).collect()
        } else {
            vec![(area.w / ncols as f64).max(1.0); ncols]
        };
        let row_h = (area.h / nrows as f64).clamp(1.0, 4032.0);
        let mut table = Table { cols, rows: vec![], first_row: header, ..Default::default() };
        for row in rows {
            let mut cells: Vec<TCell> = vec![];
            for c in 0..ncols {
                let mut cell = TCell::new("");
                if let Some(cc) = row.get(c) {
                    let mut t = cc.text.clone();
                    if t.para.bidi.is_none() {
                        t.para.bidi = Some(rtl);
                    }
                    if cc.align.is_some() {
                        t.para.align = cc.align;
                    }
                    let mut para = model_paragraph(&t, rtl, ParaKind::Plain, 0);
                    para.def_props.size = None;
                    cell.text = TextBody { paragraphs: vec![para], ..Default::default() };
                }
                cells.push(cell);
            }
            table.rows.push(TRow { height: row_h, cells });
        }
        let Some(id) = self.p.alloc_id() else {
            self.full = true;
            return;
        };
        let shape = Shape {
            id: deckcraft_model::ShapeId(id),
            name: "Table".into(),
            xfrm: Some(Xfrm::new(area.x, area.y, area.w, row_h * nrows as f64)),
            kind: ShapeKind::Table(table),
            ..Default::default()
        };
        s.shapes.push(shape);
        self.push(s);
    }

    /// A Title Only slide with the picture fitted into the body area.
    fn image_slide(&mut self, title: &str, notes: &str, name: &str, png: Vec<u8>, width_pt: Option<f64>, height_pt: Option<f64>) {
        let Some(mut s) = self.new_slide(LayoutType::TitleOnly, title, notes) else { return };
        let area = body_area(self.p);
        let (iw, ih) = match (width_pt, height_pt, png_size(&png)) {
            (Some(w), Some(h), _) => (w, h),
            (_, _, Some((w, h))) => (f64::from(w), f64::from(h)),
            _ => (area.w, area.h),
        };
        let scale = (area.w / iw.max(1e-9)).min(area.h / ih.max(1e-9));
        let (w, h) = if scale.is_finite() && scale > 0.0 { (iw * scale, ih * scale) } else { (area.w, area.h) };
        let rect = Xfrm::new(area.x + (area.w - w) / 2.0, area.y + (area.h - h) / 2.0, w, h);
        let (Some(media), Some(id)) = (self.p.add_media(name, "image/png", png), self.p.alloc_id()) else {
            self.full = true;
            return;
        };
        let shape = Shape {
            id: deckcraft_model::ShapeId(id),
            name: "Picture".into(),
            xfrm: Some(rect),
            kind: ShapeKind::Picture { fill: deckcraft_model::style::PictureFill { media, ..Default::default() } },
            descr: name.to_string(),
            ..Default::default()
        };
        s.shapes.push(shape);
        self.push(s);
    }

    /// Lay a finished slide out: its pieces in order, continued on "<title> (2)", "(3)"… when
    /// they overflow.
    fn flush(&mut self, b: Building) {
        let Building { title, notes, pieces, .. } = b;
        let mut part = 0usize;
        let next_title = |part: &mut usize| {
            *part += 1;
            if *part == 1 { title.clone() } else { format!("{title} ({part})") }
        };
        let mut paras: Vec<Paragraph> = vec![];
        let mut lines = 0usize;
        let mut first_notes = Some(notes);
        if pieces.is_empty() {
            let t = next_title(&mut part);
            self.text_slide(&t, first_notes.take().as_deref().unwrap_or(""), vec![]);
            return;
        }
        for piece in pieces {
            match piece {
                Piece::Para(p, n) => {
                    if !paras.is_empty() && lines + n > LINES_PER_BODY {
                        let t = next_title(&mut part);
                        self.text_slide(&t, first_notes.take().as_deref().unwrap_or(""), std::mem::take(&mut paras));
                        lines = 0;
                    }
                    lines += n;
                    paras.push(p);
                }
                Piece::Table { rtl, header, widths, rows } => {
                    if !paras.is_empty() {
                        let t = next_title(&mut part);
                        self.text_slide(&t, first_notes.take().as_deref().unwrap_or(""), std::mem::take(&mut paras));
                        lines = 0;
                    }
                    if rows.is_empty() {
                        let t = next_title(&mut part);
                        self.table_slide(&t, first_notes.take().as_deref().unwrap_or(""), rtl, header, &widths, &[vec![CCell::default()]]);
                        continue;
                    }
                    for chunk in rows.chunks(ROWS_PER_SLIDE) {
                        let t = next_title(&mut part);
                        self.table_slide(&t, first_notes.take().as_deref().unwrap_or(""), rtl, header, &widths, chunk);
                    }
                }
                Piece::Image { name, png, width_pt, height_pt } => {
                    if !paras.is_empty() {
                        let t = next_title(&mut part);
                        self.text_slide(&t, first_notes.take().as_deref().unwrap_or(""), std::mem::take(&mut paras));
                        lines = 0;
                    }
                    let t = next_title(&mut part);
                    self.image_slide(&t, first_notes.take().as_deref().unwrap_or(""), &name, png, width_pt, height_pt);
                }
            }
        }
        if !paras.is_empty() {
            let t = next_title(&mut part);
            self.text_slide(&t, first_notes.take().as_deref().unwrap_or(""), paras);
        }
    }
}

fn piece_para(t: &CText, doc_rtl: bool, kind: ParaKind, level: u8) -> Piece {
    let text = t.text();
    Piece::Para(model_paragraph(t, doc_rtl, kind, level), lines_for(&text))
}

/// Append the document's slides to `p`; the number of slides made.
pub fn add_slides(p: &mut Presentation, doc: &Doc) -> usize {
    let default_title = doc.title.clone().unwrap_or_else(|| "Presentation".to_string());
    let mut b = Builder { p, doc_rtl: doc.rtl, made: 0, full: false };
    let mut cur: Option<Building> = None;
    for block in &doc.blocks {
        if b.full {
            break;
        }
        match block {
            Block::Slide { title, notes } => {
                if let Some(done) = cur.take() {
                    b.flush(done);
                }
                let title = if title.trim().is_empty() { default_title.clone() } else { title.clone() };
                cur = Some(Building { title, notes: notes.clone(), pieces: vec![], from_slide: true });
            }
            Block::Heading { level, text } => {
                let in_body = cur.as_ref().is_some_and(|c| c.from_slide) && *level >= 2;
                if in_body {
                    if let Some(c) = cur.as_mut() {
                        c.pieces.push(piece_para(text, doc.rtl, ParaKind::Bold, 0));
                    }
                } else {
                    if let Some(done) = cur.take() {
                        b.flush(done);
                    }
                    let title = text.text().replace(['\r', '\n'], " ");
                    let title = if title.trim().is_empty() { default_title.clone() } else { title };
                    cur = Some(Building { title, notes: String::new(), pieces: vec![], from_slide: false });
                }
            }
            Block::PageBreak => {}
            other => {
                let c = cur.get_or_insert_with(|| Building { title: default_title.clone(), notes: String::new(), pieces: vec![], from_slide: false });
                match other {
                    Block::Paragraph(t) => c.pieces.push(piece_para(t, doc.rtl, ParaKind::Plain, 0)),
                    Block::List { ordered, items } => {
                        let kind = if *ordered { ParaKind::Numbered } else { ParaKind::Bullet };
                        for i in items {
                            c.pieces.push(piece_para(&i.text, doc.rtl, kind, i.level));
                        }
                    }
                    Block::Table { rtl, header, widths, rows } => {
                        c.pieces.push(Piece::Table { rtl: rtl.unwrap_or(doc.rtl), header: *header, widths: widths.clone(), rows: rows.clone() });
                    }
                    Block::Image { name, png, width_pt, height_pt } => {
                        if !png.is_empty() {
                            c.pieces.push(Piece::Image { name: name.clone(), png: png.clone(), width_pt: *width_pt, height_pt: *height_pt });
                        }
                    }
                    Block::Slide { .. } | Block::Heading { .. } | Block::PageBreak => {}
                }
            }
        }
    }
    if let Some(done) = cur.take() {
        b.flush(done);
    }
    b.made
}

/// A new 16:9 presentation from the default template holding the document's slides. A document
/// without content gives one empty slide titled by its title.
pub fn read(json: &str) -> Result<Presentation> {
    let doc = parse(json)?;
    let mut p = defaults::blank_presentation(defaults::WIDE, Default::default(), false);
    if let Some(font) = &doc.font
        && let Some(m) = p.masters.first_mut()
    {
        let m = Arc::make_mut(m);
        m.theme.fonts.minor.latin = font.clone();
        m.theme.fonts.minor.cs = font.clone();
    }
    let made = add_slides(&mut p, &doc);
    if made == 0 {
        let mut b = Builder { p: &mut p, doc_rtl: doc.rtl, made: 0, full: false };
        b.flush(Building {
            title: doc.title.clone().unwrap_or_else(|| "Presentation".into()),
            notes: String::new(),
            pieces: vec![],
            from_slide: false,
        });
    }
    crate::repair(&mut p);
    Ok(p)
}

// ---------------------------------------------------------------------------------------------
// Base64 (standard alphabet; the engine's helper lives above this layer)
// ---------------------------------------------------------------------------------------------

pub fn base64_encode(data: &[u8]) -> String {
    const A: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b = [chunk.first().copied().unwrap_or(0), chunk.get(1).copied().unwrap_or(0), chunk.get(2).copied().unwrap_or(0)];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        for (i, shift) in [18u32, 12, 6, 0].iter().enumerate() {
            if i <= chunk.len() {
                out.push(A.get(((n >> shift) & 63) as usize).copied().unwrap_or(b'A') as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

pub fn base64_decode(s: &str) -> Option<Vec<u8>> {
    let s = s.split_once("base64,").map(|(_, b)| b).unwrap_or(s);
    let mut out = Vec::with_capacity(s.len() / 4 * 3);
    let mut acc = 0u32;
    let mut bits = 0u32;
    for c in s.bytes() {
        let v = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' | b'-' => 62,
            b'/' | b'_' => 63,
            b'=' | b'\n' | b'\r' | b' ' | b'\t' => continue,
            _ => return None,
        };
        acc = (acc << 6) | u32::from(v);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push(((acc >> bits) & 0xff) as u8);
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

    use super::*;

    const AR: &str = "هذا نص عربي للتجربة يوضّح كيف تُنقل الفقرات بين البرامج ";

    fn arabic(chars: usize) -> String {
        let mut s = String::new();
        while s.chars().count() < chars {
            s.push_str(AR);
        }
        s.chars().take(chars).collect()
    }

    fn para(text: &str, kashida: &str) -> Value {
        json!({"kind": "paragraph", "text": text, "para": {"bidi": true, "align": "justify", "kashida": kashida}})
    }

    fn sample() -> String {
        let rows: Vec<Value> =
            (0..12).map(|r| json!([{"text": format!("خلية {r}")}, {"text": format!("قيمة {r}"), "para": {"bidi": true}}])).collect();
        json!({
            "craftx": 1,
            "from": "wordcraft-ar",
            "title": "واجبات المستوصف",
            "rtl": true,
            "blocks": [
                {"kind": "heading", "level": 1, "text": "X"},
                para(&arabic(150), "low"),
                para(&arabic(500), "medium"),
                para(&arabic(100), "high"),
                {"kind": "table", "rtl": true, "header": true, "rows": rows},
                {"kind": "list", "ordered": false, "items": [{"text": "أول", "level": 0}, {"text": "ثانٍ", "level": 1}]},
            ]
        })
        .to_string()
    }

    fn body_paras(p: &Presentation, i: usize) -> Vec<Paragraph> {
        p.slides[i]
            .shapes
            .iter()
            .filter(|s| s.ph_type().is_some_and(|k| matches!(k, PhType::Obj | PhType::Body)))
            .flat_map(|s| s.text.clone().unwrap().paragraphs)
            .collect()
    }

    #[test]
    fn a_word_document_becomes_slides_with_continuations_and_kashida() {
        let p = read(&sample()).unwrap();
        let titles: Vec<String> = p.slides.iter().map(|s| s.title()).collect();
        assert_eq!(titles[0], "X");
        assert_eq!(titles[1], "X (2)");
        assert!(titles.len() >= 4, "{titles:?}");
        // 150 Arabic chars are 3 lines, 500 are 9: the second paragraph overflows to "X (2)",
        // where the third (100 chars, 2 lines) still fits beside it.
        let first = body_paras(&p, 0);
        assert_eq!(first.len(), 1, "{first:?}");
        assert_eq!(first[0].props.align, Some(Align::JustLow));
        assert_eq!(first[0].props.rtl, Some(true));
        let second = body_paras(&p, 1);
        assert_eq!(second.len(), 2);
        assert_eq!(second[0].props.align, Some(Align::JustMedium));
        assert_eq!(second.last().unwrap().props.align, Some(Align::JustHigh));
        // Table slides hold at most 8 rows each, every cell paragraph right-to-left.
        let tables: Vec<&Table> =
            p.slides.iter().flat_map(|s| s.shapes.iter()).filter_map(|sh| if let ShapeKind::Table(t) = &sh.kind { Some(t) } else { None }).collect();
        assert_eq!(tables.len(), 2);
        assert_eq!(tables[0].rows.len(), 8);
        assert_eq!(tables[1].rows.len(), 4);
        assert!(tables[0].first_row);
        for t in &tables {
            for r in &t.rows {
                for c in &r.cells {
                    assert_eq!(c.text.paragraphs[0].props.rtl, Some(true));
                }
            }
        }
        // The list follows the table on its own slide, bulleted at its levels.
        let last = body_paras(&p, p.slides.len() - 1);
        assert_eq!(last.len(), 2);
        assert_eq!(last[0].level, 0);
        assert_eq!(last[1].level, 1);
        assert!(last.iter().all(|q| q.props.rtl == Some(true) && q.props.bullet.is_none()));
        // Every paragraph of every slide is right-to-left.
        for (i, s) in p.slides.iter().enumerate() {
            for sh in &s.shapes {
                for q in sh.text.iter().flat_map(|t| &t.paragraphs).filter(|q| !q.is_empty()) {
                    assert_eq!(q.props.rtl, Some(true), "slide {i}: {}", q.text());
                }
            }
        }
        assert!(p.validate().is_empty());
    }

    #[test]
    fn write_of_read_keeps_texts_order_bidi_and_kashida() {
        let p = read(&sample()).unwrap();
        let out = write(&p, "X").unwrap();
        let doc = parse(&out).unwrap();
        assert_eq!(doc.from.as_deref(), Some(FROM));
        assert!(doc.rtl);
        let texts: Vec<(String, Option<bool>, Kashida)> = doc
            .blocks
            .iter()
            .flat_map(|b| match b {
                Block::Paragraph(t) => vec![(t.text(), t.para.bidi, t.para.kashida)],
                Block::List { items, .. } => items.iter().map(|i| (i.text.text(), i.text.para.bidi, i.text.para.kashida)).collect(),
                _ => vec![],
            })
            .collect();
        assert_eq!(texts.len(), 5, "{texts:?}");
        assert_eq!(texts[0].0, arabic(150));
        assert_eq!(texts[1].0, arabic(500));
        assert_eq!(texts[2].0, arabic(100));
        assert_eq!(texts[3].0, "أول");
        assert_eq!(texts[4].0, "ثانٍ");
        assert_eq!([texts[0].2, texts[1].2, texts[2].2], [Kashida::Low, Kashida::Medium, Kashida::High]);
        assert!(texts.iter().all(|t| t.1 == Some(true)));
        // Slides come out in order, the table with its rows and header.
        let slides: Vec<&str> =
            doc.blocks.iter().filter_map(|b| if let Block::Slide { title, .. } = b { Some(title.as_str()) } else { None }).collect();
        assert_eq!(slides[..2], ["X", "X (2)"]);
        let rows: usize = doc.blocks.iter().filter_map(|b| if let Block::Table { rows, .. } = b { Some(rows.len()) } else { None }).sum();
        assert_eq!(rows, 12);
        assert!(doc.blocks.iter().any(|b| matches!(b, Block::Table { header: true, rtl: Some(true), .. })));
        // And it reads again.
        assert!(read(&out).unwrap().slides.len() >= 4);
    }

    #[test]
    fn hostile_json_never_panics() {
        let hostile = r#"{"craftx":1,"blocks":[{"kind":"table","rows":[[]]},{"kind":"heading","level":99},{"kind":"image","png_base64":"@@"},{"kind":"paragraph","text":"x","para":{"size_pt":1e9}}]}"#;
        let p = read(hostile).unwrap();
        assert!(!p.slides.is_empty());
        assert!(p.validate().is_empty());
        for bad in [
            "",
            "null",
            "[]",
            "{}",
            "{\"craftx\":1}",
            "{\"craftx\":\"x\",\"blocks\":5}",
            "{\"craftx\":1,\"blocks\":[1,\"a\",null,{\"kind\":5},{\"kind\":\"list\",\"items\":[1]}]}",
            "{\"craftx\":1,\"rtl\":true,\"blocks\":[{\"kind\":\"image\"},{\"kind\":\"slide\"},{\"kind\":\"table\",\"rows\":[1,[\"a\",2]],\"widths\":[\"x\",-1]}]}",
        ] {
            match read(bad) {
                Ok(p) => assert!(p.validate().is_empty(), "{bad}"),
                Err(e) => assert!(matches!(e, FormatError::NotOurs(_)), "{bad}: {e}"),
            }
        }
        assert!(matches!(read("[]"), Err(FormatError::NotOurs(_))));
        assert!(matches!(read("{}"), Err(FormatError::NotOurs(_))));
        assert!(read("{\"craftx\":1}").is_ok());
    }

    #[test]
    fn a_document_without_headings_is_titled_and_numbered() {
        let paras: Vec<Value> = (0..30).map(|i| json!({"kind": "paragraph", "text": format!("Line {i}")})).collect();
        let p = read(&json!({"craftx": 1, "title": "Notes", "blocks": paras}).to_string()).unwrap();
        let titles: Vec<String> = p.slides.iter().map(|s| s.title()).collect();
        assert_eq!(titles, ["Notes", "Notes (2)", "Notes (3)"]);
        assert_eq!(body_paras(&p, 0).len(), 11);
        assert!(body_paras(&p, 0).iter().all(|q| q.props.bullet == Some(Bullet::None) && q.props.rtl.is_none()));
        assert_eq!(body_paras(&p, 0)[0].props.align, None);
    }

    #[test]
    fn alignment_swaps_for_bidi_and_pictures_fit_the_body() {
        // A 1×1 PNG.
        let png: Vec<u8> = vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0, 0, 0, 0x0D, b'I', b'H', b'D', b'R', 0, 0, 0, 2, 0, 0, 0, 1, 8, 6, 0, 0, 0, 0, 0, 0, 0,
        ];
        assert_eq!(png_size(&png), Some((2, 1)));
        let doc = json!({"craftx": 1, "rtl": true, "blocks": [
            {"kind": "heading", "level": 1, "text": "عنوان"},
            {"kind": "paragraph", "text": "يمين", "para": {"align": "start"}},
            {"kind": "paragraph", "text": "left", "para": {"bidi": false, "align": "start"}},
            {"kind": "paragraph", "text": "وسط", "para": {"align": "center"}},
            {"kind": "paragraph", "text": "موزّع", "para": {"align": "distribute"}},
            {"kind": "image", "name": "p.png", "png_base64": base64_encode(&png)},
            {"kind": "page_break"},
        ]});
        let p = read(&doc.to_string()).unwrap();
        let paras = body_paras(&p, 0);
        assert_eq!(paras[0].props.align, Some(Align::Right));
        assert_eq!(paras[1].props.align, Some(Align::Left));
        assert_eq!(paras[1].props.rtl, Some(false));
        assert_eq!(paras[2].props.align, Some(Align::Center));
        assert_eq!(paras[3].props.align, Some(Align::Distributed));
        let title = &p.slides[0].shapes[0];
        assert_eq!(title.text.as_ref().unwrap().paragraphs[0].props.rtl, Some(true));
        let pic = p.slides[1].shapes.iter().find(|s| matches!(s.kind, ShapeKind::Picture { .. })).expect("picture slide");
        let x = pic.xfrm.unwrap();
        let area = body_area(&p);
        assert!(x.w <= area.w + 0.5 && (x.h - area.h).abs() < 0.5 && (x.w / x.h - 2.0).abs() < 0.01, "{x:?} fits {area:?}");
        assert_eq!(p.slides[1].title(), "عنوان (2)");
        // Written back, the alignments and the picture survive.
        let back = parse(&write(&p, "t").unwrap()).unwrap();
        let aligns: Vec<Option<CAlign>> =
            back.blocks.iter().filter_map(|b| if let Block::Paragraph(t) = b { Some(t.para.align) } else { None }).collect();
        assert_eq!(aligns, [Some(CAlign::Start), Some(CAlign::Start), Some(CAlign::Center), Some(CAlign::Distribute)]);
        assert!(back.blocks.iter().any(|b| matches!(b, Block::Image { png: bytes, .. } if *bytes == png)));
        assert_eq!(base64_decode(&base64_encode(&png)).unwrap(), png);
    }

    #[test]
    fn slide_blocks_keep_their_notes_and_subheadings_in_the_body() {
        let doc = json!({"craftx": 1, "blocks": [
            {"kind": "slide", "title": "One", "notes": "Say hello"},
            {"kind": "heading", "level": 2, "text": "Sub"},
            {"kind": "paragraph", "text": "Body"},
            {"kind": "slide", "title": "Two"},
            {"kind": "list", "ordered": true, "items": [{"text": "a"}, {"text": "b"}]},
        ]});
        let p = read(&doc.to_string()).unwrap();
        assert_eq!(p.slides.len(), 2);
        assert_eq!(p.slides[0].notes.text(), "Say hello");
        let paras = body_paras(&p, 0);
        assert_eq!(paras.len(), 2);
        assert_eq!(paras[0].runs[0].props.bold, Some(true));
        assert_eq!(paras[0].text(), "Sub");
        let two = body_paras(&p, 1);
        assert!(matches!(two[0].props.bullet, Some(Bullet::AutoNum { .. })));
        let back = to_doc(&p, "deck");
        assert!(matches!(&back.blocks[0], Block::Slide { title, notes } if title == "One" && notes == "Say hello"));
        assert!(back.blocks.iter().any(|b| matches!(b, Block::List { ordered: true, items } if items.len() == 2)));
    }

    #[test]
    fn sniff_knows_its_own_files() {
        assert!(sniff(br#"{"craftx":1,"blocks":[]}"#));
        assert!(sniff("\u{feff}{ \"craftx\": 1 }".as_bytes()));
        assert!(!sniff(b"PK\x03\x04"));
        assert!(!sniff(b"{\"format\":\"deckcraft\"}"));
    }
}
