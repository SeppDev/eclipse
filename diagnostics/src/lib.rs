use common::path::Path;
use common::position::PositionRange;
use std::collections::HashMap;

pub type DiagnosticResult<T = ()> = Result<T, Option<DiagnosticData>>;

pub mod builder;
mod display;
mod file;

#[derive(Default, PartialEq)]
pub enum DiagnosticLevel {
    #[default]
    Error,
    Warning,
    Help,
    Note,
}

#[derive(Default)]
pub struct DiagnosticSpan {
    path: Option<Path>,
    position: Option<PositionRange>,
    message: String,
}

#[derive(Default)]
pub struct DiagnosticData {
    level: DiagnosticLevel,
    position: Option<PositionRange>,
    title: String,
    spans: Vec<DiagnosticSpan>,
}

pub struct DiagnosticsFile {
    diagnostics: Vec<DiagnosticData>,
}

#[derive(Default)]
pub struct Diagnostics<'a> {
    files: HashMap<&'a Path, DiagnosticsFile>,
}

impl<'a> Diagnostics<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn file(&mut self, relative_path: &'a Path) -> &mut DiagnosticsFile {
        self.files.insert(relative_path, DiagnosticsFile::new());
        self.files.get_mut(relative_path).unwrap()
    }
    pub fn display(&self) {
        println!(
            "{}",
            self.files
                .iter()
                .map(|(p, f)| f.display(p))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
    pub fn has_errors(&self) -> bool {
        for (_, file) in &self.files {
            for diagnostic in &file.diagnostics {
                if diagnostic.level == DiagnosticLevel::Error {
                    return true;
                }
            }
        }
        false
    }
}
