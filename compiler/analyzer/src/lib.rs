use context::CompilerCtx;
use syntax::{ast, hlir};

mod semantic;
mod types;

struct Analyzer<'ctx> {
    pub compiler: &'ctx mut CompilerCtx,
}

pub fn analyze(
    compiler: &mut CompilerCtx,
    collection: ast::ModuleCollection,
) -> hlir::ModuleCollection {
    let mut analyzer = Analyzer { compiler };
    analyzer.analyze(collection)
}
