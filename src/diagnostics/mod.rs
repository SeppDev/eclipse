use crate::common::position::PositionRange;
use std::path::PathBuf;
pub mod builder;
mod display;

pub type DiagnosticResult<T> = Result<T, DiagnosticData>;

#[derive(Default)]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
    Hint,
    #[default]
    Note,
}

pub struct DiagnosticSpan {
    file: PathBuf,
    message: String,
    position: PositionRange,
}

#[derive(Default)]
pub struct DiagnosticData {
    level: DiagnosticLevel,
    title: String,
    notes: Vec<DiagnosticSpan>,
}
impl DiagnosticData {
    pub fn path(&self) -> &PathBuf {
        &self.notes.first().unwrap().file
    }
}

pub struct Diagnostics {}
