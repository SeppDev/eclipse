use std::path::PathBuf;

use crate::common::position::PositionRange;

use super::{DiagnosticData, DiagnosticSpan};

impl DiagnosticData {
    pub fn new(
        title: impl Into<String>,
        file: PathBuf,
        note: impl Into<String>,
        position: PositionRange,
    ) -> Self {
        Self {
            level: super::DiagnosticLevel::default(),
            title: title.into(),
            notes: Vec::new(),
        }
        .span(note, file, position)
    }
    pub fn span(
        mut self,
        title: impl Into<String>,
        file: PathBuf,
        position: PositionRange,
    ) -> Self {
        self.notes.push(DiagnosticSpan {
            file,
            message: title.into(),
            position,
        });

        self
    }
}
