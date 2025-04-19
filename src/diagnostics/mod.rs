use crate::common::{exit::exit, position::PositionRange};
use std::path::PathBuf;

mod result;
pub use result::DiagnosticResult;

pub mod builder;
mod display;

#[derive(Default)]
pub enum DiagnosticLevel {
    #[default]
    Error,
    Warning,
    Info,
    Hint,
    Note,
}

pub struct DiagnosticSpan {
    path: PathBuf,
    message: String,
    position: PositionRange,
}

#[derive(Default)]
pub struct DiagnosticData {
    level: DiagnosticLevel,
    path: PathBuf,
    position: Option<PositionRange>,
    title: String,
    notes: Vec<DiagnosticSpan>,
}
impl DiagnosticData {
    pub fn exit(self) -> ! {
        exit(format!("{}", self))
    }
}

#[derive(Default)]
pub struct Diagnostics {
    diagnostics: Vec<DiagnosticData>,
}
impl Diagnostics {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }
    pub fn collect_error<T>(&mut self, result: DiagnosticResult<T>) -> Result<T, ()> {
        let error = match result {
            Ok(t) => return Ok(t),
            Err(err) => err,
        };
        self.diagnostics.push(error);

        Err(())
    }
}
