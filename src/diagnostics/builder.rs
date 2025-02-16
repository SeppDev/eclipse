use std::path::PathBuf;

use crate::common::position::PositionRange;

use super::{DiagnosticData, DiagnosticLevel, DiagnosticSpan};

impl DiagnosticData {
    pub fn new(
        title: impl Into<String>,
        path: PathBuf,
        note: impl Into<String>,
        position: PositionRange,
    ) -> Self {
        Self::basic(title, path.clone()).span(note, path, position)
    }
    pub fn basic(title: impl Into<String>, path: PathBuf) -> Self {
        Self {
            path,
            level: super::DiagnosticLevel::default(),
            title: title.into(),
            notes: Vec::new(),
        }
    }
    pub fn span(
        mut self,
        title: impl Into<String>,
        path: PathBuf,
        position: PositionRange,
    ) -> Self {
        self.notes.push(DiagnosticSpan {
            path,
            message: title.into(),
            position,
        });

        self
    }
    pub fn warning(mut self) -> Self {
        self.level = DiagnosticLevel::Warning;
        self
    }
}
