use std::path::PathBuf;

use super::{DiagnosticData, DiagnosticLevel, DiagnosticResult, DiagnosticSpan};
use common::position::PositionRange;

#[allow(unused)]
impl DiagnosticData {
    fn new(level: DiagnosticLevel) -> Self {
        Self {
            level,
            ..Default::default()
        }
    }
    fn level(mut self, level: DiagnosticLevel) -> Self {
        self.level = level;
        self
    }
    pub fn title(mut self, title: impl ToString) -> Self {
        self.title = title.to_string();
        self
    }
    pub fn position(mut self, position: PositionRange) -> Self {
        self.position = Some(position);
        self
    }
    pub fn span(mut self, span: DiagnosticSpan) -> Self {
        self.spans.push(span);
        self
    }
    pub fn error() -> Self {
        Self::new(DiagnosticLevel::Error)
    }
    pub fn warning() -> Self {
        Self::new(DiagnosticLevel::Warning)
    }
    pub fn help() -> Self {
        Self::new(DiagnosticLevel::Help)
    }
    pub fn note() -> Self {
        Self::new(DiagnosticLevel::Note)
    }
    pub fn to_err<T>(self) -> DiagnosticResult<T> {
        self.into()
    }
}
impl<T> Into<DiagnosticResult<T>> for DiagnosticData {
    fn into(self) -> DiagnosticResult<T> {
        Err(self)
    }
}

#[allow(unused)]
impl DiagnosticSpan {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.into(),
            ..Default::default()
        }
    }
    pub fn position(mut self, position: PositionRange) -> Self {
        self.position = Some(position);
        self
    }
    pub fn path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }
}
