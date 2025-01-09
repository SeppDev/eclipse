use super::{analyzer::AnalyzedModule, errors::CompileCtx, nodes::hlir};

pub fn optimize(ctx: &mut CompileCtx, module: &mut AnalyzedModule) {
    for function in &mut module.functions {
        function.check_redundant_variables();
    }
}

impl hlir::Function {
    fn check_redundant_variables(&mut self) {
        
    }
}