use std::fmt::{Debug, Display};

use super::{DiagnosticData, DiagnosticLevel};

impl DiagnosticData {
    fn to_string(&self) -> String {
        let path: String = self.path.clone().to_str().unwrap().into();
        let level = &self.level;
        let title = &self.title;
        let span = String::new();

        format!("{level}: {title}\n\t--> {path}\n{span}")
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
