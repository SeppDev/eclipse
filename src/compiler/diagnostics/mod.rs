use crate::{common::position::PositionRange, compiler::Path};

pub type DiagnosticResult<T = ()> = Result<T, DiagnosticData>;

pub mod builder;
mod display;
pub mod file;

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

#[allow(unused)]
pub struct DiagnosticsFile {
    path: Path,
    diagnostics: Vec<DiagnosticData>,
}

#[allow(unused)]
#[derive(Default)]
pub struct Diagnostics {
    files: Vec<DiagnosticsFile>,
}

#[allow(unused)]
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
    pub fn check(&self) {
        let has_errors = self.has_errors();
        if has_errors {
            todo!()
        }
    }
    fn has_errors(&self) -> bool {
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
