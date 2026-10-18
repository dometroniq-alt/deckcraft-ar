//! DeckCraft AR — Home › Send: hand the presentation (or the current slide) to WordCraft AR or
//! GridCraft AR as a Craft exchange file (`.craftx`, `docs/ar-exchange.md`), and receive one back
//! as a new presentation.

use deckcraft_format::craftx;
use serde_json::{Value, json};

use super::*;
use crate::craft_launch::{self, Launched, Sibling};
use crate::{EngineError, Result, Session};

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(
            noundo "send.toWordCraft",
            "Send to WordCraft AR",
            ["Home", "Send"],
            None,
            "{scope?: presentation|slide} → {path, launched, message}: writes <temp>/craft-exchange/<title>-<HHMMSS>.craftx and starts WordCraft AR on it",
            has_doc,
            to_wordcraft
        ),
        cmd!(
            noundo "send.toGridCraft",
            "Send to GridCraft AR",
            ["Home", "Send"],
            None,
            "{scope?: presentation|slide} → {path, launched, message}: writes <temp>/craft-exchange/<title>-<HHMMSS>.craftx and starts GridCraft AR on it",
            has_doc,
            to_gridcraft
        ),
        cmd!(
            noundo "send.importCraftx",
            "Receive from WordCraft / GridCraft",
            ["Home", "Send"],
            None,
            "{path | data: base64, name?} → {document, slides}: opens a .craftx as a new presentation",
            always,
            import_craftx
        ),
    ]
}

/// The folder the exchange files go to: `craft-exchange` under the OS temp dir.
pub fn exchange_dir() -> std::path::PathBuf {
    std::env::temp_dir().join("craft-exchange")
}

/// `HHMMSS` of the local clock (UTC on platforms without a clock offset); `000000` where there
/// is no clock at all (the browser).
fn stamp() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0) % 86_400;
        format!("{:02}{:02}{:02}", secs / 3600, (secs / 60) % 60, secs % 60)
    }
    #[cfg(target_arch = "wasm32")]
    {
        "000000".to_string()
    }
}

/// A file stem from a title: no path separators or characters Windows refuses, never empty.
pub fn safe_stem(title: &str) -> String {
    let s: String = title
        .chars()
        .map(|c| if c.is_control() || matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|') { '_' } else { c })
        .take(80)
        .collect();
    let s = s.trim().trim_end_matches('.').to_string();
    if s.is_empty() { "Presentation".into() } else { s }
}

/// The user-facing line after a send: the sibling opened the file, or where it was saved.
pub fn message(app: Sibling, launched: &Launched, path: &str) -> String {
    match launched {
        Launched::Started(_) => format!("Sent to {}", app.name()),
        Launched::NotFound => format!("Saved {path} — open it in {} (File › Open)", app.name()),
    }
}

fn send(s: &mut Session, p: &Value, app: Sibling) -> Result<Value> {
    let cmd = match app {
        Sibling::WordCraft => "send.toWordCraft",
        Sibling::GridCraft => "send.toGridCraft",
    };
    let scope = str_param(p, "scope").unwrap_or("presentation");
    let st = s.doc()?;
    let title = st.title();
    let json = match scope {
        "presentation" => craftx::write(&st.doc, &title),
        "slide" => {
            let mut one = (*st.doc).clone();
            let slide = one.slides.get(st.selection.slide).cloned().ok_or_else(|| bad(cmd, "no current slide"))?;
            one.slides = vec![slide];
            craftx::write(&one, &title)
        }
        other => return Err(bad(cmd, format!("scope “{other}” isn't presentation or slide"))),
    }
    .map_err(|e| EngineError::Other(e.to_string()))?;
    let dir = exchange_dir();
    crate::io::create_dir_all(&dir).map_err(EngineError::Other)?;
    let path = dir.join(format!("{}-{}.{}", safe_stem(&title), stamp(), craftx::EXTENSION));
    let path_str = path.to_string_lossy().to_string();
    crate::io::write_file(&path_str, json.as_bytes()).map_err(EngineError::Other)?;
    // Tests never start other programs (`launch` also refuses under its own `cfg!(test)`).
    let launched = if cfg!(test) { Launched::NotFound } else { craft_launch::launch(app, &path) };
    let text = message(app, &launched, &path_str);
    s.ui_requests.push(crate::UiRequest::Message { text: text.clone() });
    Ok(json!({
        "path": path_str,
        "launched": matches!(launched, Launched::Started(_)),
        "app": app.name(),
        "message": text,
        "bytes": json.len(),
    }))
}

fn to_wordcraft(s: &mut Session, p: &Value) -> Result<Value> {
    send(s, p, Sibling::WordCraft)
}

fn to_gridcraft(s: &mut Session, p: &Value) -> Result<Value> {
    send(s, p, Sibling::GridCraft)
}

/// `send.importCraftx {path}` or `{data: base64, name?}`: the file as a new presentation.
fn import_craftx(s: &mut Session, p: &Value) -> Result<Value> {
    let (name, bytes) = match (str_param(p, "path"), str_param(p, "data")) {
        (Some(path), _) => (path.to_string(), crate::io::read_file(path).map_err(EngineError::Other)?),
        (None, Some(data)) => (
            str_param(p, "name").unwrap_or("Received.craftx").to_string(),
            base64_decode(data).ok_or_else(|| bad("send.importCraftx", "invalid base64 `data`"))?,
        ),
        (None, None) => return Err(bad("send.importCraftx", "missing `path` (or `data`)")),
    };
    let text = String::from_utf8_lossy(&bytes);
    let doc = craftx::read(&text).map_err(|e| EngineError::Other(format!("{name}: {e}")))?;
    let slides = doc.slides.len();
    let i = super::file::add_opened(s, &name, None, doc);
    Ok(json!({"document": i, "slides": slides}))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

    use serde_json::json;

    use super::*;
    use crate::Session;

    const AR: &str = "نص عربي تجريبي يوضح كيف ينتقل المحتوى بين البرامج الثلاثة ";

    fn arabic(chars: usize) -> String {
        let mut s = String::new();
        while s.chars().count() < chars {
            s.push_str(AR);
        }
        s.chars().take(chars).collect()
    }

    /// The brief's fixture: a Heading 1, three Arabic justified paragraphs (kashida low, medium,
    /// high; the middle one 500 characters, so it overflows), a 12-row right-to-left table and a
    /// bulleted list.
    fn fixture() -> String {
        let para = |text: String, k: &str| json!({"kind": "paragraph", "text": text, "para": {"bidi": true, "align": "justify", "kashida": k}});
        let rows: Vec<Value> = (0..12).map(|r| json!([{"text": format!("البند {r}")}, {"text": format!("القيمة {r}")}])).collect();
        json!({
            "craftx": 1, "from": "wordcraft-ar", "title": "واجبات", "rtl": true,
            "blocks": [
                {"kind": "heading", "level": 1, "text": "X"},
                para(arabic(150), "low"),
                para(arabic(500), "medium"),
                para(arabic(100), "high"),
                {"kind": "table", "rtl": true, "header": true, "rows": rows},
                {"kind": "list", "ordered": false, "items": [{"text": "الأول", "level": 0}, {"text": "الثاني", "level": 1}]},
            ]
        })
        .to_string()
    }

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("deckcraft-craftx-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn craftx_import_builds_slides_with_continuations_and_rtl_kashida() {
        use deckcraft_model::text::Align;
        let mut s = Session::new();
        let r = s.execute("send.importCraftx", &json!({"data": base64_encode(fixture().as_bytes()), "name": "duties.craftx"})).unwrap();
        assert!(r["slides"].as_u64().unwrap() >= 4, "{r}");
        let doc = s.doc().unwrap().doc.clone();
        let titles: Vec<String> = doc.slides.iter().map(|sl| sl.title()).collect();
        assert_eq!(&titles[..2], &["X".to_string(), "X (2)".to_string()]);
        let mut aligns = vec![];
        let mut table_rows = vec![];
        for sl in &doc.slides {
            for sh in &sl.shapes {
                if sh.ph_type().is_some_and(deckcraft_model::PhType::is_title) {
                    continue;
                }
                for para in sh.text.iter().flat_map(|t| &t.paragraphs).filter(|q| !q.is_empty()) {
                    assert_eq!(para.props.rtl, Some(true), "{}", para.text());
                    aligns.push(para.props.align);
                }
                if let deckcraft_model::ShapeKind::Table(t) = &sh.kind {
                    table_rows.push(t.rows.len());
                    for row in &t.rows {
                        for c in &row.cells {
                            assert_eq!(c.text.paragraphs[0].props.rtl, Some(true));
                        }
                    }
                }
            }
        }
        assert_eq!(&aligns[..3], &[Some(Align::JustLow), Some(Align::JustMedium), Some(Align::JustHigh)]);
        assert_eq!(table_rows, [8, 4]);
        assert!(doc.validate().is_empty());
    }

    /// Saved to PPTX, the right-to-left kashida paragraphs come out as `rtl="1"` and
    /// `algn="justLow"` in the first slide.
    #[test]
    fn craftx_slides_save_to_pptx_with_rtl_and_justlow() {
        let doc = craftx::read(&fixture()).unwrap();
        let bytes = deckcraft_pptx::export(&doc).unwrap();
        let mut z = zip::ZipArchive::new(std::io::Cursor::new(bytes)).unwrap();
        let mut xml = String::new();
        std::io::Read::read_to_string(&mut z.by_name("ppt/slides/slide1.xml").unwrap(), &mut xml).unwrap();
        assert!(xml.contains("rtl=\"1\""), "{xml}");
        assert!(xml.contains("algn=\"justLow\""), "{xml}");
    }

    #[test]
    fn craftx_write_of_read_keeps_texts_order_bidi_and_kashida() {
        let p = craftx::read(&fixture()).unwrap();
        let back = craftx::parse(&craftx::write(&p, "واجبات").unwrap()).unwrap();
        let mut texts = vec![];
        for b in &back.blocks {
            match b {
                craftx::Block::Paragraph(t) => texts.push((t.text(), t.para.bidi, t.para.kashida)),
                craftx::Block::List { items, .. } => texts.extend(items.iter().map(|i| (i.text.text(), i.text.para.bidi, i.text.para.kashida))),
                _ => {}
            }
        }
        let want = [
            (arabic(150), craftx::Kashida::Low),
            (arabic(500), craftx::Kashida::Medium),
            (arabic(100), craftx::Kashida::High),
            ("الأول".to_string(), craftx::Kashida::None),
            ("الثاني".to_string(), craftx::Kashida::None),
        ];
        assert_eq!(texts.len(), want.len(), "{texts:?}");
        for ((text, bidi, k), (wt, wk)) in texts.iter().zip(want.iter()) {
            assert_eq!(text, wt);
            assert_eq!(*bidi, Some(true));
            assert_eq!(k, wk);
        }
        assert!(back.rtl);
        let rows: usize = back.blocks.iter().filter_map(|b| if let craftx::Block::Table { rows, .. } = b { Some(rows.len()) } else { None }).sum();
        assert_eq!(rows, 12);
    }

    #[test]
    fn craftx_hostile_json_never_panics() {
        let hostile = r#"{"craftx":1,"blocks":[{"kind":"table","rows":[[]]},{"kind":"heading","level":99},{"kind":"image","png_base64":"@@"},{"kind":"paragraph","text":"x","para":{"size_pt":1e9}}]}"#;
        let mut s = Session::new();
        let r = s.execute("send.importCraftx", &json!({"data": base64_encode(hostile.as_bytes())})).unwrap();
        assert!(r["slides"].as_u64().unwrap() >= 1);
        assert!(s.doc().unwrap().doc.validate().is_empty());
        assert!(s.execute("send.importCraftx", &json!({"data": "@@"})).is_err());
        assert!(s.execute("send.importCraftx", &json!({})).is_err());
        assert!(s.execute("send.importCraftx", &json!({"data": base64_encode(b"not json")})).is_err());
        assert!(s.execute("send.importCraftx", &json!({"path": "/nonexistent/x.craftx"})).is_err());
    }

    /// `send.toWordCraft` writes `<temp>/craft-exchange/<title>-<HHMMSS>.craftx` (nothing is
    /// started under test) and says where; `scope: slide` sends the current slide alone.
    #[test]
    fn send_writes_the_exchange_file_and_says_where() {
        let mut s = Session::new();
        crate::sample::open_sample(&mut s).unwrap();
        let n = s.doc().unwrap().doc.slides.len();
        let r = s.execute("send.toWordCraft", &json!({})).unwrap();
        let path = r["path"].as_str().unwrap().to_string();
        assert!(path.ends_with(".craftx") && path.contains("craft-exchange"), "{path}");
        assert_eq!(r["launched"], false);
        assert!(r["message"].as_str().unwrap().contains("WordCraft AR (File › Open)"), "{r}");
        let text = std::fs::read_to_string(&path).unwrap();
        let doc = craftx::parse(&text).unwrap();
        assert_eq!(doc.blocks.iter().filter(|b| matches!(b, craftx::Block::Slide { .. })).count(), n);
        assert_eq!(doc.from.as_deref(), Some("deckcraft-ar"));
        let _ = std::fs::remove_file(&path);
        let r = s.execute("send.toGridCraft", &json!({"scope": "slide"})).unwrap();
        let path = r["path"].as_str().unwrap().to_string();
        let doc = craftx::parse(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(doc.blocks.iter().filter(|b| matches!(b, craftx::Block::Slide { .. })).count(), 1);
        assert!(r["message"].as_str().unwrap().contains("GridCraft AR"));
        let _ = std::fs::remove_file(&path);
        assert!(s.execute("send.toWordCraft", &json!({"scope": "everything"})).is_err());
        assert!(matches!(s.ui_requests.last(), Some(crate::UiRequest::Message { .. })));
    }

    #[test]
    fn send_is_disabled_without_a_presentation() {
        let mut s = Session::new();
        assert!(s.execute("send.toWordCraft", &json!({})).is_err());
        assert!(s.execute("send.toGridCraft", &json!({})).is_err());
    }

    /// `.craftx` goes through the file commands too: `file.open`, `file.saveAs` and `file.export`.
    #[test]
    fn craftx_opens_and_saves_through_the_file_commands() {
        let dir = scratch("file");
        let src = dir.join("in.craftx");
        std::fs::write(&src, fixture()).unwrap();
        let mut s = Session::new();
        let r = s.execute("file.open", &json!({"path": src.to_str().unwrap()})).unwrap();
        assert!(r["slides"].as_u64().unwrap() >= 4, "{r}");
        assert!(s.doc().unwrap().path.is_none(), "a received file isn't the presentation's own file");
        let out = dir.join("out.craftx");
        s.execute("file.saveAs", &json!({"path": out.to_str().unwrap()})).unwrap();
        assert!(craftx::sniff(&std::fs::read(&out).unwrap()));
        let out2 = dir.join("out2.json");
        s.execute("file.export", &json!({"path": out2.to_str().unwrap(), "format": "craftx"})).unwrap();
        assert!(craftx::parse(&std::fs::read_to_string(&out2).unwrap()).is_ok());
        assert_eq!(crate::cmd::file::writable_format_for_path("a.craftx"), Some("craftx"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn stems_and_messages_are_safe() {
        assert_eq!(safe_stem("a/b:c*d?e\"f<g>h|i"), "a_b_c_d_e_f_g_h_i");
        assert_eq!(safe_stem("   "), "Presentation");
        assert_eq!(safe_stem("name..."), "name");
        assert_eq!(message(Sibling::WordCraft, &Launched::Started("x".into()), "/p"), "Sent to WordCraft AR");
        assert_eq!(message(Sibling::GridCraft, &Launched::NotFound, "/p"), "Saved /p — open it in GridCraft AR (File › Open)");
        assert_eq!(stamp().len(), 6);
    }
}
