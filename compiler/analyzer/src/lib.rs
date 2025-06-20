use diagnostics::DiagnosticResult;

mod semantics;
mod types;

pub struct Analyzer {}

pub fn analyze(modules: &ASTModules) -> DiagnosticResult {
    let analyzer = Analyzer {};
    for (_, module) in &modules.files {
        analyzer.semantics(&module.body)?;
    }
    // let types = self.get_types(modules)?;
    // self.log(format!("{types:#?}"));

    Ok(())
}

impl Analyzer {
    // pub(super) fn check_name_collision(&self) {}
}

pub struct Symbol {}
pub struct SymbolTable {}
