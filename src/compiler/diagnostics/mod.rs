use crate::{common::position::PositionRange, compiler::Path};

pub type DiagnosticResult<T = ()> = Result<T, DiagnosticData>;

pub mod builder;
mod display;
pub mod file;

#[derive(Default)]
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
}
