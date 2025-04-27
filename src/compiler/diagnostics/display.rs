use std::fmt::{Debug, Display};

use crate::compiler::Path;

use super::{DiagnosticData, DiagnosticLevel, DiagnosticsFile};

impl Display for DiagnosticsFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.diagnostics
                .iter()
                .map(|diagnostic| diagnostic.to_string(&self.path))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
impl Debug for DiagnosticsFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl DiagnosticData {
    fn to_string(&self, path: &Path) -> String {
        let level = &self.level;
        let title = &self.title;
        let position = match &self.position {
            Some(p) => format!(
                ":{}:{}-{}:{}",
                p.start.line, p.start.column, p.end.line, p.end.column
            ),
            None => String::new(),
        };

        let span = self
            .spans
            .iter()
            .map(|note| format!("{}", note.message))
            .collect::<Vec<String>>()
            .join("\n");

        format!("{level}: {title}\n\t--> {path}{position}\n{span}")
    }
}

impl Display for DiagnosticLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Error => "Error",
                Self::Warning => "Warning",
                Self::Note => "Note",
                Self::Help => "Help",
            }
        )
    }
}
