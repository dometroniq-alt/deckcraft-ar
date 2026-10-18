//! Starting a sibling Craft app (WordCraft AR, GridCraft AR) on a `.craftx` file, as
//! `docs/ar-exchange.md` § *Launching the sibling app* lays down: the executable beside ours, then
//! `..\<Sibling> AR\` under the same parent folder, then (Windows) the `App Paths` registry keys
//! the AR installers register, then the plain name on `PATH`. Nothing here is fatal: when no
//! sibling is found the file is still saved and the caller tells the user where.

use std::path::{Path, PathBuf};

/// The sibling apps DeckCraft AR exchanges files with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sibling {
    WordCraft,
    GridCraft,
}

impl Sibling {
    /// The product name, as shown to the user.
    pub fn name(self) -> &'static str {
        match self {
            Sibling::WordCraft => "WordCraft AR",
            Sibling::GridCraft => "GridCraft AR",
        }
    }
    /// The executable's file stem.
    pub fn exe_stem(self) -> &'static str {
        match self {
            Sibling::WordCraft => "WordCraft AR",
            Sibling::GridCraft => "gridcraft",
        }
    }
    /// The sibling's install folder name under the same Program Files parent.
    pub fn folder(self) -> &'static str {
        match self {
            Sibling::WordCraft => "WordCraft AR",
            Sibling::GridCraft => "GridCraft AR",
        }
    }
    /// The `App Paths` keys the AR installer registers, the AR one first, then the upstream one.
    pub fn app_paths(self) -> [&'static str; 2] {
        match self {
            Sibling::WordCraft => ["WordCraft AR.exe", "wordcraft.exe"],
            Sibling::GridCraft => ["gridcraft-ar.exe", "gridcraft.exe"],
        }
    }
    /// The plain command name looked up on `PATH`.
    pub fn path_name(self) -> &'static str {
        match self {
            Sibling::WordCraft => "wordcraft",
            Sibling::GridCraft => "gridcraft",
        }
    }
}

/// What [`launch`] did.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Launched {
    /// The sibling was started from this executable with the file as its only argument.
    Started(PathBuf),
    /// No sibling was found (or starting it failed): the file is saved, nothing was opened.
    NotFound,
}

fn exe_name(stem: &str) -> String {
    if cfg!(windows) { format!("{stem}.exe") } else { stem.to_string() }
}

/// The executables to try next to our own: `<exe dir>/<sibling>` and `<exe dir>/../<Sibling AR>/<sibling>`.
pub fn neighbour_candidates(app: Sibling, our_exe: &Path) -> Vec<PathBuf> {
    let Some(dir) = our_exe.parent() else { return vec![] };
    let name = exe_name(app.exe_stem());
    let mut out = vec![dir.join(&name)];
    if let Some(parent) = dir.parent() {
        out.push(parent.join(app.folder()).join(&name));
    }
    // The upstream name beside ours too (`WordCraft AR.exe` is also shipped as `wordcraft.exe`).
    let upstream = exe_name(app.path_name());
    if upstream != name {
        out.push(dir.join(&upstream));
    }
    out
}

/// The `REG_SZ` default value of an `App Paths` key as `reg query <key> /ve` prints it, or `None`
/// for anything that doesn't look like one. Odd output is never a panic: the first line holding
/// `REG_SZ` is split at it and the rest trimmed.
pub fn parse_reg_query(output: &str) -> Option<PathBuf> {
    for line in output.lines() {
        let Some(at) = line.find("REG_SZ") else { continue };
        let value = line.get(at + "REG_SZ".len()..)?.trim();
        if !value.is_empty() {
            return Some(PathBuf::from(value.trim_matches('"')));
        }
    }
    None
}

#[cfg(all(windows, not(target_arch = "wasm32")))]
fn registry_candidates(app: Sibling) -> Vec<PathBuf> {
    let mut out = vec![];
    for key in app.app_paths() {
        let full = format!(r"HKLM\Software\Microsoft\Windows\CurrentVersion\App Paths\{key}");
        let Ok(o) = std::process::Command::new("reg").args(["query", &full, "/ve"]).output() else { continue };
        if let Some(p) = parse_reg_query(&String::from_utf8_lossy(&o.stdout)) {
            out.push(p);
        }
    }
    out
}

#[cfg(not(all(windows, not(target_arch = "wasm32"))))]
fn registry_candidates(_app: Sibling) -> Vec<PathBuf> {
    vec![]
}

/// Start the sibling `app` with `path` as its only argument, trying the places the spec lists in
/// order. Under `cfg!(test)` nothing is started (tests must never open other programs).
#[cfg(not(target_arch = "wasm32"))]
pub fn launch(app: Sibling, path: &Path) -> Launched {
    if cfg!(test) {
        return Launched::NotFound;
    }
    let mut candidates: Vec<PathBuf> = std::env::current_exe().map(|exe| neighbour_candidates(app, &exe)).unwrap_or_default();
    candidates.extend(registry_candidates(app));
    for exe in candidates.into_iter().filter(|c| c.is_file()) {
        if std::process::Command::new(&exe).arg(path).spawn().is_ok() {
            return Launched::Started(exe);
        }
    }
    // Last, the plain name on PATH.
    let name = app.path_name();
    if std::process::Command::new(name).arg(path).spawn().is_ok() {
        return Launched::Started(PathBuf::from(name));
    }
    Launched::NotFound
}

#[cfg(target_arch = "wasm32")]
pub fn launch(_app: Sibling, _path: &Path) -> Launched {
    Launched::NotFound
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbours_are_tried_beside_and_under_the_same_parent() {
        let exe = Path::new("/apps/DeckCraft AR").join(exe_name("deckcraft"));
        let c = neighbour_candidates(Sibling::WordCraft, &exe);
        assert_eq!(c.first(), Some(&Path::new("/apps/DeckCraft AR").join(exe_name("WordCraft AR"))));
        assert_eq!(c.get(1), Some(&Path::new("/apps/WordCraft AR").join(exe_name("WordCraft AR"))));
        let g = neighbour_candidates(Sibling::GridCraft, &exe);
        assert_eq!(g.first(), Some(&Path::new("/apps/DeckCraft AR").join(exe_name("gridcraft"))));
        assert_eq!(g.get(1), Some(&Path::new("/apps/GridCraft AR").join(exe_name("gridcraft"))));
        assert!(neighbour_candidates(Sibling::GridCraft, Path::new("")).is_empty());
    }

    #[test]
    fn reg_query_output_is_parsed_and_odd_output_is_ignored() {
        let out = "\r\nHKEY_LOCAL_MACHINE\\...\\App Paths\\wordcraft.exe\r\n    (Default)    REG_SZ    C:\\Program Files\\WordCraft AR\\WordCraft AR.exe\r\n\r\n";
        assert_eq!(parse_reg_query(out), Some(PathBuf::from(r"C:\Program Files\WordCraft AR\WordCraft AR.exe")));
        assert_eq!(parse_reg_query("    (Default)    REG_SZ    \"C:\\x\\y.exe\""), Some(PathBuf::from(r"C:\x\y.exe")));
        for odd in ["", "ERROR: The system was unable to find the specified registry key or value.", "REG_SZ", "REG_SZ   ", "   REG_EXPAND_SZ  x"] {
            assert_eq!(parse_reg_query(odd), None, "{odd:?}");
        }
    }

    #[test]
    fn tests_never_start_a_program() {
        assert_eq!(launch(Sibling::WordCraft, Path::new("/nonexistent/x.craftx")), Launched::NotFound);
        assert_eq!(Sibling::GridCraft.name(), "GridCraft AR");
        assert_eq!(Sibling::WordCraft.app_paths(), ["WordCraft AR.exe", "wordcraft.exe"]);
    }
}
