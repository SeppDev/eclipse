use crate::compiler::Path;

use super::{DiagnosticData, DiagnosticResult, DiagnosticsFile};

impl DiagnosticsFile {
    pub fn capture<T>(&mut self, result: DiagnosticResult<T>) -> DiagnosticResult<T> {
        let error = match result {
            Ok(r) => return Ok(r),
            Err(error) => error,
        };
        self.extract_error(error);
        Err(None)
    }
    pub fn new(relative_path: Path) -> Self {
        Self {
            path: relative_path,
            diagnostics: Vec::new(),
        }
    }

    pub fn extract_error(&mut self, mut option: Option<DiagnosticData>) {
        match option.take() {
            Some(err) => self.diagnostics.push(err),
            None => panic!("Already took the diagnostics data!"),
        };
    }
}
