use crate::diagnostics::DiagnosticResult;

use super::CompilerCtx;

impl CompilerCtx {
    pub fn analyze(&mut self) -> DiagnosticResult<()> {
        let body = self.parse()?;
        Ok(())
    }
}
