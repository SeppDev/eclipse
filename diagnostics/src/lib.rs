use common::position::PositionRange;
use std::{collections::HashMap, path::PathBuf};

pub type DiagnosticResult<T = ()> = Result<T, DiagnosticData>;

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
    path: Option<PathBuf>,
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
    // relative_path: Path,
    diagnostics: Vec<DiagnosticData>,
}

#[derive(Default)]
pub struct Diagnostics {
    files: HashMap<PathBuf, DiagnosticsFile>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn check(&self) {
        if !self.has_errors() {
            return;
        }
        self.display();
        std::process::exit(0)
    }
    pub fn file(&mut self, relative_path: &PathBuf) -> &mut DiagnosticsFile {
        self.files
            .entry(relative_path.clone())
            .or_insert_with(DiagnosticsFile::new)
    }
    pub fn insert(&mut self, relative_path: &PathBuf, diagnostic: DiagnosticData) {
        let file = self.file(relative_path);
        file.insert(diagnostic);
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
