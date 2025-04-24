use super::{DiagnosticData, DiagnosticLevel, DiagnosticSpan};
use crate::common::position::PositionRange;

impl DiagnosticData {
    fn new() -> Self {
        Self {
            level: DiagnosticLevel::Error,
            position: None,
            title: String::new(),
            spans: Vec::new(),
        }
    }
    fn level(mut self, level: DiagnosticLevel) -> Self {
        self.level = level;
        self
    }
    fn title(mut self, title: &str) -> Self {
        self.title = title.into();
        self
    }
    pub fn error() -> Self {
        Self {
            level: DiagnosticLevel::Error,
            ..Default::default()
        }
    }
    pub fn position(mut self, position: PositionRange) -> Self {
        self.position = Some(position);
        self
    }
    pub fn basic(title: impl Into<String>) -> Self {
        Self {
            position: None,
            level: super::DiagnosticLevel::default(),
            title: title.into(),
            spans: Vec::new(),
        }
    }
    pub fn span(mut self, title: impl Into<String>, path: Path, position: PositionRange) -> Self {
        self.spans.push(DiagnosticSpan {
            path,
            message: title.into(),
            position,
        });

        self
    }
}

impl DiagnosticSpan {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn position(mut self, position: PositionRange) -> Self {
        self.position = Some(position);
        self
    }
}