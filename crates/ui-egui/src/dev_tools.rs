//! DeckCraft AR — built-in developer systems, reachable from View › DeckCraft AR (ported from
//! GridCraft AR's and WordCraft AR's `dev_tools.rs`):
//!
//! - **UI Kit**: every entry of the registry ([`crate::ui_kit`]) with its thumbnail (drawn
//!   still), its key, its explanation in English and in Arabic, filterable — the place to check
//!   that a new button looks right and reads right before it ships.
//! - **Icons**: the whole icon set ([`Icon::ALL`]) at the ribbon's two sizes, 16 and 30 pt, so
//!   weights can be compared side by side.
//! - **Self-Test**: runs every registered engine command with empty params on a scratch
//!   presentation (never the open one) and lists what each answered and how long it took — the
//!   engine's `every_command_survives_empty_and_hostile_params` test, inside the app, on this
//!   machine.
//! - **Checklist**: what is still missing — ribbon controls without a UI-kit entry, explanations
//!   without an Arabic translation — the same checks the tests make, readable without a build.
//!
//! The dialog is `uiKit` (`app.uiKit {tab?}`; `app.selfTest` opens it on the Self-Test tab).

use egui::{Rect, Sense, Stroke, Ui, vec2};
use serde_json::{Value as Json, json};

use crate::SlideApp;
use crate::dialogs::Dialog;
use crate::i18n::Lang;
use crate::icons::{self, Icon};
use crate::theme::{self, Tokens};

/// The dialog's tabs, in order.
pub const TABS: [&str; 4] = ["UI Kit", "Icons", "Self-Test", "Checklist"];

/// Every ribbon control (tab function › key) with its icon, generated from the ribbon inventory
/// (one row per button or menu item; a button is keyed by its English label or tip, the one
/// string every widget hands `ui_kit::tooltip`, a menu item by `ui_kit::key(id, params)`).
/// Regenerate it when the ribbon gains a control.
pub const INVENTORY: &[(&str, &str, Option<Icon>)] = &[
    ("photo_album_button", "Photo\nAlbum", Some(Icon::Picture)),
    ("clipboard_group", "Paste", Some(Icon::Paste)),
    ("clipboard_group", "Format", Some(Icon::FormatPainter)),
    ("slides_group", "New\nSlide", Some(Icon::NewSlide)),
    ("slides_group", "Layout", Some(Icon::Layout)),
    ("slides_group", "Section", Some(Icon::Section)),
    ("font_group", "Character Spacing", Some(Icon::CharSpacing)),
    ("font_group", "Change Case", Some(Icon::ChangeCase)),
    ("font_group", "Text Highlight Color", Some(Icon::Highlight)),
    ("font_group", "Font Color", Some(Icon::FontColor)),
    ("paragraph_group", "Line Spacing", Some(Icon::LineSpacing)),
    ("paragraph_group", "Columns", Some(Icon::Columns)),
    ("paragraph_group", "Text Direction", Some(Icon::TextDirection)),
    ("paragraph_group", "Align Text", Some(Icon::AlignText)),
    ("paragraph_group", "Convert to SmartArt", Some(Icon::SmartArt)),
    ("drawing_group", "Shapes", Some(Icon::Shapes)),
    ("drawing_group", "Arrange", Some(Icon::Arrange)),
    ("drawing_group", "Quick\nStyles", Some(Icon::QuickStyles)),
    ("drawing_group", "Shape Fill", Some(Icon::ShapeFill)),
    ("drawing_group", "Shape Outline", Some(Icon::ShapeOutline)),
    ("drawing_group", "Shape Effects", Some(Icon::ShapeEffects)),
    ("home", "Replace Fonts…", Some(Icon::Fonts)),
    ("insert", "Table", Some(Icon::Table)),
    ("insert", "Screenshot", Some(Icon::Screenshot)),
    ("insert", "Icons", Some(Icon::Icons)),
    ("insert", "Chart", Some(Icon::Chart)),
    ("insert", "Zoom", Some(Icon::Zoom)),
    ("insert", "Action", Some(Icon::Action)),
    ("insert", "WordArt", Some(Icon::WordArt)),
    ("draw", "Select", Some(Icon::LassoSelect)),
    ("draw", "Eraser", Some(Icon::Eraser)),
    ("draw", "Ink to\nShape", Some(Icon::InkToShape)),
    ("draw", "Ink to\nText", Some(Icon::InkToText)),
    ("draw", "Ink to\nMath", Some(Icon::InkToMath)),
    ("design", "Browse for Themes…", Some(Icon::Themes)),
    ("design", "Colors", Some(Icon::Colors)),
    ("design", "Fonts", Some(Icon::Fonts)),
    ("design", "Effects", Some(Icon::ShapeEffects)),
    ("design", "Background\nStyles", Some(Icon::BackgroundStyles)),
    ("design", "Slide\nSize", Some(Icon::SlideSize)),
    ("transitions", "Preview", Some(Icon::Preview)),
    ("transitions", "Effect\nOptions", Some(Icon::EffectOptions)),
    ("animations", "Add\nAnimation", Some(Icon::AddAnimation)),
    ("animations", "Trigger", Some(Icon::Trigger)),
    ("animations", "Animation\nPainter", Some(Icon::AnimationPainter)),
    ("slide_show", "Play from\nStart", Some(Icon::PlayFromStart)),
    ("slide_show", "Play from\nCurrent Slide", Some(Icon::PlayFromCurrent)),
    ("slide_show", "Presenter\nView", Some(Icon::PresenterView)),
    ("slide_show", "Hide\nSlide", Some(Icon::HideSlide)),
    ("slide_show", "Rehearse\nTimings", Some(Icon::RehearseTimings)),
    ("slide_show", "Record", Some(Icon::Record)),
    ("record", "Cameo", Some(Icon::Cameo)),
    ("record", "From\nBeginning", Some(Icon::PlayFromStart)),
    ("record", "From\nCurrent Slide", Some(Icon::PlayFromCurrent)),
    ("record", "Learn\nMore", Some(Icon::Help)),
    ("review", "Translate", Some(Icon::Translate)),
    ("review", "Accept\nAll", Some(Icon::Check)),
    ("review", "End\nReview", Some(Icon::Close)),
    ("review", "Show\nComments", Some(Icon::ShowComments)),
    ("view", "Slide\nMaster", Some(Icon::SlideMaster)),
    ("view", "Handout\nMaster", Some(Icon::HandoutMaster)),
    ("view", "Notes\nMaster", Some(Icon::NotesMaster)),
    ("view", "Grid\nOptions", Some(Icon::Gridlines)),
    ("view", "Notes", Some(Icon::Notes)),
    ("view", "Zoom", Some(Icon::ZoomGlass)),
    ("view", "Fit to\nWindow", Some(Icon::FitToWindow)),
    ("view", "New\nWindow", Some(Icon::NewWindow)),
    ("view", "Grayscale", Some(Icon::Grayscale)),
    ("view", "Dark\nMode", Some(Icon::Palette)),
    ("shape_format", "Edit\nShape", Some(Icon::EditShape)),
    ("shape_format", "Merge\nShapes", Some(Icon::MergeShapes)),
    ("shape_format", "Shape\nStyles", Some(Icon::QuickStyles)),
    ("shape_format", "Text Fill", Some(Icon::TextFill)),
    ("shape_format", "Text Outline", Some(Icon::TextOutline)),
    ("shape_format", "Text Effects", Some(Icon::TextEffects)),
    ("arrange_group", "Bring\nForward", Some(Icon::BringForward)),
    ("arrange_group", "Send\nBackward", Some(Icon::SendBackward)),
    ("arrange_group", "Align", Some(Icon::AlignObjects)),
    ("arrange_group", "Group", Some(Icon::Group)),
    ("arrange_group", "Rotate", Some(Icon::Rotate)),
    ("picture_format", "Remove\nBackground", Some(Icon::RemoveBackground)),
    ("picture_format", "Corrections", Some(Icon::Corrections)),
    ("picture_format", "Color", Some(Icon::ColorAdjust)),
    ("picture_format", "Artistic\nEffects", Some(Icon::ArtisticEffects)),
    ("picture_format", "Transparency", Some(Icon::Transparency)),
    ("picture_format", "Compress Pictures", Some(Icon::CompressPictures)),
    ("picture_format", "Picture Border", Some(Icon::QuickStyles)),
    ("picture_format", "Crop", Some(Icon::Crop)),
    ("table_design", "Shading", Some(Icon::Shading)),
    ("table_design", "Borders", Some(Icon::Borders)),
    ("table_layout", "Delete", Some(Icon::DeleteTable)),
    ("chart_design", "Change\nChart Type", Some(Icon::ChangeChartType)),
    ("smartart_design", "Text\nPane", Some(Icon::SmartArt)),
    ("smartart_design", "Add Shape", Some(Icon::Plus)),
    ("smartart_design", "Promote", Some(Icon::DecreaseIndent)),
    ("smartart_design", "Demote", Some(Icon::IncreaseIndent)),
    ("smartart_design", "Insert Picture", Some(Icon::Picture)),
    ("smartart_design", "Layouts", Some(Icon::Layout)),
    ("smartart_design", "Convert", Some(Icon::Layout)),
    ("smartart_design", "Change Colors", Some(Icon::ChangeChartType)),
    ("smartart_design", "SmartArt Styles", Some(Icon::SmartArt)),
    ("playback", "Volume", Some(Icon::Volume)),
    ("slide_master", "Insert\nPlaceholder", Some(Icon::Table)),
    ("slide_master", "Themes", Some(Icon::Themes)),
    ("slide_master", "Close\nMaster View", Some(Icon::Close)),
    ("clipboard_group", "Cut", Some(Icon::Cut)),
    ("clipboard_group", "Copy", Some(Icon::Copy)),
    ("slides_group", "Reset", Some(Icon::ResetSlide)),
    ("font_group", "Increase Font Size", Some(Icon::FontGrow)),
    ("font_group", "Decrease Font Size", Some(Icon::FontShrink)),
    ("font_group", "Clear All Formatting", Some(Icon::ClearFormatting)),
    ("font_group", "Bold", Some(Icon::Bold)),
    ("font_group", "Italic", Some(Icon::Italic)),
    ("font_group", "Underline", Some(Icon::Underline)),
    ("font_group", "Strikethrough", Some(Icon::Strikethrough)),
    ("font_group", "Superscript", Some(Icon::Superscript)),
    ("font_group", "Subscript", Some(Icon::Subscript)),
    ("paragraph_group", "Decrease List Level", Some(Icon::DecreaseIndent)),
    ("paragraph_group", "Increase List Level", Some(Icon::IncreaseIndent)),
    ("paragraph_group", "Align Left", Some(Icon::AlignLeft)),
    ("paragraph_group", "Center", Some(Icon::AlignCenter)),
    ("paragraph_group", "Align Right", Some(Icon::AlignRight)),
    ("paragraph_group", "Justify", Some(Icon::Justify)),
    ("drawing_group", "Text Box", Some(Icon::TextBox)),
    ("home", "Picture", Some(Icon::Picture)),
    ("home", "Find", Some(Icon::Search)),
    ("home", "Replace", Some(Icon::Spelling)),
    ("home", "Selection\nPane", Some(Icon::SelectionPane)),
    ("home", "Design\nIdeas", Some(Icon::DesignIdeas)),
    ("insert", "Pictures", Some(Icon::Picture)),
    ("insert", "SmartArt", Some(Icon::SmartArt)),
    ("insert", "Link", Some(Icon::Link)),
    ("insert", "Comment", Some(Icon::Comment)),
    ("insert", "Text\nBox", Some(Icon::TextBox)),
    ("insert", "Header &\nFooter", Some(Icon::HeaderFooter)),
    ("insert", "Date &\nTime", Some(Icon::DateTime)),
    ("insert", "Slide\nNumber", Some(Icon::SlideNumber)),
    ("insert", "Equation", Some(Icon::Equation)),
    ("insert", "Symbol", Some(Icon::Symbol)),
    ("insert", "Video", Some(Icon::Video)),
    ("insert", "Audio", Some(Icon::Audio)),
    ("design", "Format\nBackground", Some(Icon::FormatBackground)),
    ("transitions", "Apply\nTo All", Some(Icon::ApplyToAll)),
    ("animations", "Animation\nPane", Some(Icon::AnimationPane)),
    ("slide_show", "Custom\nShow", Some(Icon::CustomShow)),
    ("slide_show", "Set Up\nSlide Show", Some(Icon::SetUpShow)),
    ("review", "Spelling", Some(Icon::Spelling)),
    ("review", "Thesaurus", Some(Icon::Thesaurus)),
    ("review", "Check\nAccessibility", Some(Icon::Accessibility)),
    ("review", "Language", Some(Icon::Language)),
    ("review", "Compare", Some(Icon::Compare)),
    ("review", "New\nComment", Some(Icon::NewComment)),
    ("review", "Delete", Some(Icon::DeleteComment)),
    ("shape_format", "Alt\nText", Some(Icon::AltText)),
    ("shape_format", "Format\nPane", Some(Icon::FormatBackground)),
    ("picture_format", "Change Picture", Some(Icon::ChangePicture)),
    ("picture_format", "Reset Picture", Some(Icon::ResetPicture)),
    ("table_layout", "Insert\nAbove", Some(Icon::InsertAbove)),
    ("table_layout", "Insert\nBelow", Some(Icon::InsertBelow)),
    ("table_layout", "Insert\nLeft", Some(Icon::InsertLeft)),
    ("table_layout", "Insert\nRight", Some(Icon::InsertRight)),
    ("table_layout", "Merge\nCells", Some(Icon::MergeCells)),
    ("table_layout", "Split\nCells", Some(Icon::SplitCells)),
    ("table_layout", "Distribute Rows", Some(Icon::DistributeV)),
    ("table_layout", "Distribute Columns", Some(Icon::DistributeH)),
    ("chart_design", "Edit\nData", Some(Icon::EditData)),
    ("playback", "Trim", Some(Icon::TrimMedia)),
    ("slide_master", "Insert\nLayout", Some(Icon::NewSlide)),
    ("slide_master", "Delete", Some(Icon::DeleteSlide)),
    ("slide_master", "Rename", Some(Icon::Layout)),
    ("slide_master", "Background\nStyles", Some(Icon::FormatBackground)),
    ("menus", "file.new", None),
    ("menus", "app.openDialog", None),
    ("menus", "app.closePresentation", None),
    ("menus", "file.save", None),
    ("menus", "app.saveAsDialog", None),
    ("menus", "app.exportDialog", None),
    ("menus", "app.print", None),
    ("menus", "file.properties", None),
    ("menus", "edit.undo", None),
    ("menus", "edit.redo", None),
    ("menus", "edit.cut", None),
    ("menus", "edit.copy", None),
    ("menus", "edit.paste", None),
    ("menus", "edit.pasteText", None),
    ("menus", "app.pasteSpecial", None),
    ("menus", "edit.selectAll", None),
    ("menus", "edit.duplicate", None),
    ("menus", "slide.delete", None),
    ("menus", "app.find", None),
    ("menus", "app.replace", None),
    ("menus", "view.normal", None),
    ("menus", "view.sorter", None),
    ("menus", "view.notesPage", None),
    ("menus", "view.outline", None),
    ("menus", "view.reading", None),
    ("menus", "show.start", None),
    ("menus", "view.slideMaster", None),
    ("menus", "view.handoutMaster", None),
    ("menus", "view.notesMaster", None),
    ("menus", "view.ruler", None),
    ("menus", "view.gridlines", None),
    ("menus", "view.guides", None),
    ("menus", "view.gridOptions", None),
    ("menus", "view.zoomDialog", None),
    ("menus", "view.zoomIn", None),
    ("menus", "view.zoomOut", None),
    ("menus", "view.fit", None),
    ("menus", "view.newWindow", None),
    ("menus", "slide.new", None),
    ("menus", "slide.duplicate", None),
    ("menus", "section.add", None),
    ("menus", "insert.textBox", None),
    ("menus", "insert.wordArt", None),
    ("menus", "insert.dateTime", None),
    ("menus", "insert.slideNumber", None),
    ("menus", "insert.table", None),
    ("menus", "insert.chart", None),
    ("menus", "app.insertPictureDialog", None),
    ("menus", "insert.photoAlbum", None),
    ("menus", "insert.photoAlbumEdit", None),
    ("menus", "app.insertAudioDialog", None),
    ("menus", "app.insertVideoDialog", None),
    ("menus", "format.fontDialog", None),
    ("menus", "format.paragraphDialog", None),
    ("menus", "format.bold", None),
    ("menus", "format.italic", None),
    ("menus", "format.underline", None),
    ("menus", "format.alignLeft", None),
    ("menus", "format.alignCenter", None),
    ("menus", "format.alignRight", None),
    ("menus", "format.justify", None),
    ("menus", "shape.sizePosition", None),
    ("menus", "format.rtl", None),
    ("menus", "arrange.bringToFront", None),
    ("menus", "arrange.sendToBack", None),
    ("menus", "arrange.bringForward", None),
    ("menus", "arrange.sendBackward", None),
    ("menus", "arrange.group", None),
    ("menus", "arrange.ungroup", None),
    ("menus", "arrange.regroup", None),
    ("menus", "arrange.rotateLeft", None),
    ("menus", "arrange.rotateRight", None),
    ("menus", "arrange.flipHorizontal", None),
    ("menus", "arrange.flipVertical", None),
    ("menus", "show.fromStart", None),
    ("menus", "show.fromCurrent", None),
    ("menus", "slide.hide", None),
    ("menus", "show.setup", None),
    ("menus", "app.about", None),
    ("menus", "app.palette", None),
];

/// One command's run in the self-test.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Run {
    pub id: String,
    /// What the command answered: a tick with a glimpse of its result, the dialog it asked
    /// for, or its error message.
    pub answer: String,
    pub ok: bool,
    pub millis: f64,
}

/// The self-test's result.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Report {
    pub runs: Vec<Run>,
    pub slides: usize,
}

impl Report {
    pub fn failed(&self) -> usize {
        self.runs.iter().filter(|r| !r.ok).count()
    }
    pub fn total_ms(&self) -> f64 {
        self.runs.iter().map(|r| r.millis).sum()
    }
}

/// Commands the self-test leaves out: they touch files on disk or close the scratch presentation
/// (the same ones the engine's fuzz test skips).
pub const SKIPPED: &[&str] = &["file.close", "file.open", "file.save", "file.saveAs", "file.export", "file.saveTemplate"];

/// The scratch presentation the self-test works on: the sample deck, in a session of its own,
/// with its first shape selected so shape commands have something to act on.
pub fn scratch_session() -> deckcraft_engine::Session {
    let mut s = deckcraft_engine::Session::new();
    if deckcraft_engine::sample::open_sample(&mut s).is_err() {
        let _ = s.execute("file.new", &json!({}));
    }
    let first = s.active().and_then(|d| d.current_slide()).and_then(|sl| sl.shapes.first()).map(|sh| sh.id.0);
    if let Some(id) = first {
        let _ = s.execute("edit.select", &json!({"ids": [id]}));
    }
    s
}

/// Runs every registered engine command with empty params on a scratch presentation and reports
/// what each answered and how long it took. Dialogs the commands ask for are noted, not opened.
pub fn self_test() -> Report {
    let mut s = scratch_session();
    let mut runs = Vec::new();
    for spec in deckcraft_engine::command_specs() {
        if SKIPPED.contains(&spec.id) {
            continue;
        }
        let start = crate::now_ms();
        let result = s.execute(spec.id, &json!({}));
        let millis = crate::now_ms() - start;
        let requests = std::mem::take(&mut s.ui_requests);
        let dialog = requests.iter().find_map(|q| match q {
            deckcraft_engine::UiRequest::Dialog { id, .. } => Some(id.clone()),
            _ => None,
        });
        let (ok, answer) = match (result, dialog) {
            (Ok(_), Some(name)) => (true, format!("opens the {name} dialog")),
            (Ok(r), None) => (true, glimpse(&r)),
            (Err(e), _) => (false, e.to_string()),
        };
        runs.push(Run { id: spec.id.to_string(), answer, ok, millis });
    }
    let slides = s.active().map(|d| d.doc.slides.len()).unwrap_or(0);
    Report { runs, slides }
}

/// A short look at a command's result for the table (`✓` alone for an empty one).
fn glimpse(r: &Json) -> String {
    let text = match r {
        Json::Null => String::new(),
        Json::Object(m) if m.is_empty() => String::new(),
        other => other.to_string(),
    };
    if text.is_empty() {
        return "✓".into();
    }
    let short: String = text.chars().take(72).collect();
    if short.chars().count() < text.chars().count() { format!("✓ {short}…") } else { format!("✓ {short}") }
}

/// Ribbon controls (tab › key) of the inventory that have no UI-kit entry yet.
pub fn unexplained() -> Vec<String> {
    INVENTORY
        .iter()
        .filter(|(_, key, _)| crate::ui_kit::entry(key).is_none())
        .map(|(tab, key, _)| format!("{tab} › {}", key.replace('\n', " ")))
        .collect()
}

/// UI-kit entries whose explanation the Arabic catalog doesn't translate.
pub fn untranslated() -> Vec<&'static str> {
    crate::ui_kit::entries().filter(|e| crate::ui_kit::explain(Lang::AR, e.explain) == e.explain).map(|e| e.key).collect()
}

/// The icon the inventory gives a UI-kit key (a menu item has none).
fn icon_for(key: &str) -> Option<Icon> {
    INVENTORY.iter().find(|(_, k, _)| *k == key).and_then(|(_, _, i)| *i)
}

/// The dialog body: the `tab` field is the open tab, `search` the filter, `params.report` the
/// last self-test report. `close` is set when the Close button is clicked.
pub fn body(app: &mut SlideApp, ui: &mut Ui, d: &mut Dialog, close: &mut bool) {
    let tab = d.fields.get("tab").cloned().filter(|t| TABS.contains(&t.as_str())).unwrap_or_else(|| TABS[0].to_string());
    d.fields.insert("tab".into(), tab.clone());
    let mut search = d.fields.get("search").cloned().unwrap_or_default();
    ui.set_width(760.0);
    ui.horizontal(|ui| {
        for name in TABS {
            if ui.selectable_label(tab == name, tl!(name)).clicked() {
                d.fields.insert("tab".into(), name.to_string());
            }
        }
        ui.add_space(12.0);
        if tab == "UI Kit" || tab == "Icons" {
            ui.label(tl!("Filter:"));
            ui.add(egui::TextEdit::singleline(&mut search).desired_width(180.0));
        }
    });
    d.fields.insert("search".into(), search.clone());
    ui.separator();
    let t = Tokens::get(ui.ctx());
    egui::ScrollArea::vertical().max_height(480.0).show(ui, |ui| match tab.as_str() {
        "Icons" => icon_sheet(ui, &t, &search),
        "Self-Test" => self_test_ui(ui, d),
        "Checklist" => checklist(ui, &t),
        _ => kit_gallery(ui, &t, &search),
    });
    ui.separator();
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(tl!("Add a control: one Entry in ui_kit/<tab>.rs, one row in the inventory, one Arabic line in i18n/ar.tsv."))
                .small()
                .weak(),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(tl!("Close")).clicked() {
                *close = true;
            }
        });
    });
    let _ = app;
}

fn matches(filter: &str, hay: &[&str]) -> bool {
    let f = filter.trim().to_lowercase();
    f.is_empty() || hay.iter().any(|h| h.to_lowercase().contains(&f))
}

fn count_line(ui: &mut Ui, shown: usize, total: usize, what: &str) {
    ui.label(egui::RichText::new(format!("{shown} / {total} {}", tl!(what))).small().weak());
}

/// Every registry entry: thumbnail, icon, key, explanation, Arabic.
fn kit_gallery(ui: &mut Ui, t: &Tokens, filter: &str) {
    let lang = crate::i18n::current();
    let palette = crate::ui_kit::Palette::get(ui.ctx());
    let mut shown = 0;
    for e in crate::ui_kit::entries() {
        let arabic = crate::ui_kit::explain(Lang::AR, e.explain);
        if !matches(filter, &[e.key, e.explain, arabic]) {
            continue;
        }
        shown += 1;
        ui.horizontal_top(|ui| {
            let (r, _) = ui.allocate_exact_size(crate::ui_kit::SIZE, Sense::hover());
            ui.painter().rect_filled(r, 4.0, palette.panel);
            ui.painter().rect_stroke(r, 4.0, Stroke::new(1.0, t.separator), egui::StrokeKind::Inside);
            (e.draw)(&crate::ui_kit::Draw::new(ui.painter(), r, &palette));
            ui.add_space(8.0);
            let (ir, _) = ui.allocate_exact_size(vec2(32.0, 32.0), Sense::hover());
            if let Some(icon) = icon_for(e.key) {
                icons::paint(ui.painter(), Rect::from_center_size(ir.center(), vec2(30.0, 30.0)), icon, t.text, false);
            }
            ui.add_space(8.0);
            ui.vertical(|ui| {
                ui.set_max_width(440.0);
                ui.label(egui::RichText::new(crate::ui_kit::artwork(e.key)).font(theme::bold(12.0)));
                crate::rtl_text::paragraph(ui, crate::ui_kit::explain(lang, e.explain), theme::font(11.5), t.text, 440.0);
                if arabic != e.explain && lang != Lang::AR {
                    crate::rtl_text::paragraph(ui, arabic, theme::font(11.5), t.text_dim, 440.0);
                } else if arabic == e.explain {
                    ui.label(egui::RichText::new(tl!("No Arabic translation yet")).small().color(icons::RED));
                }
            });
        });
        ui.add_space(6.0);
    }
    count_line(ui, shown, crate::ui_kit::entries().count(), "entries");
}

/// The icon set at 16 and 30 pt, six per row.
fn icon_sheet(ui: &mut Ui, t: &Tokens, filter: &str) {
    let named: Vec<(&str, Icon)> = Icon::ALL.iter().map(|i| (i.name(), *i)).filter(|(n, _)| matches(filter, &[n])).collect();
    for row in named.chunks(6) {
        ui.horizontal(|ui| {
            for (name, icon) in row {
                let (r, resp) = ui.allocate_exact_size(vec2(122.0, 44.0), Sense::hover());
                if resp.hovered() {
                    ui.painter().rect_filled(r, 3.0, t.hover);
                }
                let small = Rect::from_center_size(r.min + vec2(14.0, 22.0), vec2(16.0, 16.0));
                let big = Rect::from_center_size(r.min + vec2(44.0, 22.0), vec2(30.0, 30.0));
                icons::paint(ui.painter(), small, *icon, t.text, false);
                icons::paint(ui.painter(), big, *icon, t.text, false);
                ui.painter().text(
                    r.min + vec2(66.0, 22.0),
                    egui::Align2::LEFT_CENTER,
                    crate::ui_kit::artwork(name),
                    egui::FontId::proportional(9.5),
                    t.text,
                );
            }
        });
    }
    count_line(ui, named.len(), Icon::ALL.len(), "icons");
}

fn self_test_ui(ui: &mut Ui, d: &mut Dialog) {
    ui.label(tl!("Runs every engine command with empty parameters on a scratch copy of the sample presentation (your presentation is not touched) and lists what each one answered."));
    if ui.button(tl!("Run the self-test")).clicked() {
        let report = serde_json::to_value(self_test()).unwrap_or(Json::Null);
        if !d.params.is_object() {
            d.params = json!({});
        }
        if let Some(o) = d.params.as_object_mut() {
            o.insert("report".into(), report);
        }
    }
    let Some(rep) = d.params.get("report").and_then(|r| serde_json::from_value::<Report>(r.clone()).ok()) else { return };
    let failed = rep.failed();
    ui.label(
        egui::RichText::new(format!(
            "{} {}; {:.0} ms; {} {}; {} {}",
            rep.runs.len(),
            tl!("commands ran"),
            rep.total_ms(),
            failed,
            tl!("answered with an error (a feature that needs a selection or a parameter)"),
            rep.slides,
            tl!("slide(s) in the scratch presentation")
        ))
        .font(theme::bold(12.0)),
    );
    egui::Grid::new("deckcraft_ar_self_test").striped(true).min_col_width(60.0).show(ui, |ui| {
        for h in ["Command", "ms", "Answer"] {
            ui.label(egui::RichText::new(tl!(h)).font(theme::bold(11.5)));
        }
        ui.end_row();
        for r in &rep.runs {
            ui.label(crate::ui_kit::artwork(&r.id));
            ui.label(format!("{:.1}", r.millis));
            let color = if r.ok { icons::GREEN } else { icons::ORANGE };
            ui.add(egui::Label::new(egui::RichText::new(crate::ui_kit::artwork(&r.answer)).color(color)).truncate());
            ui.end_row();
        }
    });
}

fn checklist(ui: &mut Ui, t: &Tokens) {
    let missing = unexplained();
    let untranslated = untranslated();
    let line = |ui: &mut Ui, ok: bool, text: String| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(if ok { "✓" } else { "✗" }).color(if ok { icons::GREEN } else { icons::RED }).font(theme::bold(13.0)));
            ui.label(text);
        });
    };
    line(ui, missing.is_empty(), format!("{} {}", missing.len(), tl!("ribbon controls without a UI-kit entry")));
    for m in &missing {
        ui.label(egui::RichText::new(crate::ui_kit::artwork(&format!("    {m}"))).small());
    }
    line(ui, untranslated.is_empty(), format!("{} {}", untranslated.len(), tl!("explanations without an Arabic translation")));
    for m in &untranslated {
        ui.label(egui::RichText::new(crate::ui_kit::artwork(&format!("    {m}"))).small());
    }
    line(ui, true, format!("{} {}", INVENTORY.len(), tl!("ribbon controls in the inventory")));
    let entries = crate::ui_kit::entries().count();
    line(ui, entries > 0, format!("{entries} {}", tl!("entries in the UI kit")));
    line(ui, true, format!("{} {}", Icon::ALL.len(), tl!("icons in the set")));
    line(ui, true, format!("{} {}", deckcraft_engine::command_specs().len(), tl!("commands in the engine registry")));
    let _ = t;
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    #[test]
    fn the_self_test_runs_every_command_without_a_panic() {
        let rep = self_test();
        let expected = deckcraft_engine::command_specs().iter().filter(|s| !SKIPPED.contains(&s.id)).count();
        assert_eq!(rep.runs.len(), expected);
        assert!(rep.runs.len() > 100, "{} ran", rep.runs.len());
        for r in &rep.runs {
            assert!(!r.answer.is_empty(), "{}: no answer", r.id);
            assert!(!r.answer.contains("panic"), "{}: {}", r.id, r.answer);
        }
    }

    #[test]
    fn inventory_keys_are_unique_per_tab_and_the_checklist_runs() {
        let mut seen = std::collections::HashSet::new();
        for (tab, key, _) in INVENTORY {
            assert!(seen.insert((tab, key)), "{tab} › {key}: listed twice");
        }
        let missing = unexplained();
        assert!(missing.is_empty(), "{} inventory rows without visual help:\n{}", missing.len(), missing.join("\n"));
        let missing = untranslated();
        assert!(missing.is_empty(), "{} explanations without Arabic:\n{}", missing.len(), missing.join("\n"));
    }

    #[test]
    fn the_report_round_trips_through_json() {
        let rep = Report { runs: vec![Run { id: "slide.new".into(), answer: "✓".into(), ok: true, millis: 0.5 }], slides: 3 };
        let json = serde_json::to_value(&rep).unwrap();
        assert_eq!(serde_json::from_value::<Report>(json).unwrap(), rep);
    }

    /// `app.uiKit` opens the dialog on its first tab, `app.selfTest` on the Self-Test tab, and
    /// one frame of each draws without a panic.
    #[test]
    fn the_ui_commands_open_the_dialog_on_the_right_tab() {
        use serde_json::json;
        let mut harness = egui_kittest::Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_ui_state(
            |ui, app: &mut SlideApp| {
                app.logic(ui.ctx());
                app.ui(ui);
            },
            SlideApp::new(deckcraft_engine::Session::with_new(), crate::Services::default()),
        );
        harness.run_steps(2);
        harness.state_mut().run("app.uiKit", json!({})).unwrap();
        harness.run_steps(2);
        let d = harness.state().dialog.clone().expect("the UI Kit dialog is open");
        assert_eq!(d.id, "uiKit");
        assert_eq!(d.fields.get("tab").map(String::as_str), Some("UI Kit"));
        harness.state_mut().run("app.selfTest", json!({})).unwrap();
        harness.run_steps(2);
        let d = harness.state().dialog.clone().expect("the dialog is open");
        assert_eq!(d.fields.get("tab").map(String::as_str), Some("Self-Test"));
        for tab in ["Icons", "Checklist"] {
            harness.state_mut().run("app.uiKit", json!({"tab": tab})).unwrap();
            harness.run_steps(2);
            assert_eq!(harness.state().dialog.as_ref().and_then(|d| d.fields.get("tab")).map(String::as_str), Some(tab));
        }
        assert!(harness.state_mut().run("app.uiKit", json!({"tab": "Nope"})).is_err());
    }
}
