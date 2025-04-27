use crate::{common::position::PositionRange, compiler::Path};

pub type DiagnosticResult<T = ()> = Result<T, Option<DiagnosticData>>;
pub enum BaseDiagnosticResult<T> {
    Ok(T),
    Err(Option<DiagnosticData>),
}
impl<T> Into<BaseDiagnosticResult<T>> for DiagnosticResult<T> {
    fn into(self) -> BaseDiagnosticResult<T> {
        match self {
            Ok(k) => BaseDiagnosticResult::Ok(k),
            Err(e) => BaseDiagnosticResult::Err(e),
        }
    }
}
impl<T> BaseDiagnosticResult<T> {
    pub fn capture(self, diagnostics: &mut DiagnosticsFile) -> DiagnosticResult<T> {
        let mut error = match self {
            BaseDiagnosticResult::Ok(r) => return Ok(r),
            BaseDiagnosticResult::Err(error) => error,
        };
        match error.take() {
            Some(err) => diagnostics.diagnostics.push(err),
            None => panic!("Already took the diagnostics data!"),
        };
        Err(None)
    }
}
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
