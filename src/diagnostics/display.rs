use std::fmt::Display;

use super::DiagnosticData;

impl Display for DiagnosticData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self.path().as_os_str();
        write!(f, "{path:?}")
    }
}
