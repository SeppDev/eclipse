use std::fmt::{Debug, Display};

use super::{DiagnosticData, DiagnosticLevel};

impl DiagnosticData {
    fn to_string(&self) -> String {
        // let path: String = self.path.clone().to_str().unwrap().into();
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

        format!("{level}: {title}\n\t--> {position}\n{span}")
    }
}

impl Debug for DiagnosticData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
impl Display for DiagnosticData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
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
                Self::Hint => "Hint",
                Self::Info => "Info",
                Self::Note => "Note",
            }
        )
    }
}
