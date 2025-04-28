use crate::{common::position::PositionRange, compiler::Path};

pub type DiagnosticResult<T = ()> = Result<T, Option<DiagnosticData>>;
impl DiagnosticsFile {
    pub fn capture<T>(&mut self, result: DiagnosticResult<T>) -> DiagnosticResult<T> {
        let mut error = match result {
            Ok(r) => return Ok(r),
            Err(error) => error,
        };
        match error.take() {
            Some(err) => self.diagnostics.push(err),
            None => panic!("Already took the diagnostics data!"),
        };
        Err(None)
    }
}

pub mod builder;
mod display;

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
    path: Path,
    diagnostics: Vec<DiagnosticData>,
}
impl DiagnosticsFile {
    pub fn new(relative_path: Path) -> Self {
        Self {
            path: relative_path,
            diagnostics: Vec::new(),
        }
    }
    pub fn then<T, F>(&mut self, func: F) -> DiagnosticResult<T>
    where
        F: FnOnce() -> DiagnosticResult<T>,
    {
        self.capture(func())
    }
}

#[derive(Default)]
pub struct Diagnostics {
    files: Vec<DiagnosticsFile>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn file(&mut self, relative_path: Path) -> &mut DiagnosticsFile {
        self.files.push(DiagnosticsFile {
            path: relative_path,
            diagnostics: Vec::new(),
        });
        self.files.last_mut().unwrap()
    }
    pub fn insert(&mut self, diagnostics: DiagnosticsFile) {
        self.files.push(diagnostics)
    }
    pub fn display(&self) {
        println!(
            "{}",
            self.files
                .iter()
                .map(|f| format!("{f}\n"))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }
    pub fn has_errors(&self) -> bool {
        for file in &self.files {
            for diagnostic in &file.diagnostics {
                if diagnostic.level == DiagnosticLevel::Error {
                    return true;
                }
            }
        }
        false
    }
}
