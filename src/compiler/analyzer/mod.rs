use super::{diagnostics::DiagnosticResult, parser::ASTModules, CompilerCtx};

mod types;

impl CompilerCtx {
    // pub(super) fn check_name_collision(&self) {}
    pub fn analyze(&mut self, modules: &ASTModules) -> DiagnosticResult {
        let types = self.get_types(modules)?;
        self.log(format!("{types:#?}"));

        Ok(())
    }
}

pub struct Symbol {}
pub struct SymbolTable {}
