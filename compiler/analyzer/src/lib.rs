use context::CompilerCtx;
use diagnostics::DiagnosticResult;

mod semantics;
#[allow(unused)]
mod types;

pub struct Analyzer<'ctx> {
    compiler: &'ctx mut CompilerCtx,
}

pub fn analyze(compiler: &mut CompilerCtx) -> DiagnosticResult {
    Ok(())
}

// pub fn analyze(modules: &ASTModules) -> DiagnosticResult {
//     let analyzer = Analyzer {};
//     for (_, module) in &modules.files {
//         analyzer.semantics(&module.body)?;
//     }
//     // let types = self.get_types(modules)?;
//     // self.log(format!("{types:#?}"));

//     Ok(())
// }

impl Analyzer {
    // pub(super) fn check_name_collision(&self) {}
}

pub struct Symbol {}
pub struct SymbolTable {}
