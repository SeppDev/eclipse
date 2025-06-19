use std::fmt::{Debug, Display};

use common::path::Path;

use super::{DiagnosticData, DiagnosticLevel, DiagnosticsFile};

impl DiagnosticsFile {
    pub fn display(&self, path: &Path) -> String {
        self.diagnostics
            .iter()
            .map(|d| d.display(path))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl DiagnosticData {
    fn display(&self, path: &Path) -> String {
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
impl Debug for DiagnosticData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display(&Path::single("?")))
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
