use crate::common::position::PositionRange;
use std::path::PathBuf;
pub mod builder;
mod display;

pub type DiagnosticResult<T> = Result<T, DiagnosticData>;

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
    title: String,
    notes: Vec<DiagnosticSpan>,
}

pub struct Diagnostics {}
