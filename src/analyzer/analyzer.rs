use crate::{analyzer::types::functions::get_function_types, AnalyzeResult, CompileError, Module, Node};

pub fn analyze(module: Module) -> AnalyzeResult<()> {
    println!("{:#?}", module);

    let types = get_function_types(&module)?;
    println!("{:#?}", types);
    
    todo!()
}
 