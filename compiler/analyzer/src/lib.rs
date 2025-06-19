use super::{diagnostics::DiagnosticResult, parser::ASTModules, CompilerCtx};

mod semantics;
mod types;

pub struct Analyzer {}

impl CompilerCtx {
    // pub(super) fn check_name_collision(&self) {}
    pub fn analyze(&mut self, modules: &ASTModules) -> DiagnosticResult {
        let analyzer = Analyzer {};
        for (_, module) in &modules.files {
            analyzer.semantics(&module.body)?;
        }
        // let types = self.get_types(modules)?;
        // self.log(format!("{types:#?}"));

        Ok(())
    }
}

pub struct Symbol {}
pub struct SymbolTable {}
