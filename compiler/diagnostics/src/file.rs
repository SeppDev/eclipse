use super::{DiagnosticData, DiagnosticsFile};

impl DiagnosticsFile {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }
    pub fn insert(&mut self, diagnostic: DiagnosticData) {
        self.diagnostics.push(diagnostic);
    }
    // pub fn capture<T>(&mut self, result: DiagnosticResult<T>) -> Option<T> {
    //     let error = match result {
    //         Ok(r) => return Some(r),
    //         Err(error) => error,
    //     };
    //     self.extract_error(error);
    //     None
    // }
    // pub fn extract_error(&mut self, mut option: Option<DiagnosticData>) {
    //     match option.take() {
    //         Some(err) => self.diagnostics.push(err),
    //         None => panic!("Already took the diagnostics data!"),
    //     };
    // }
}
