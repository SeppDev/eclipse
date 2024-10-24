use crate::{
    analyzer::types::functions::get_function_types, ASTNode, AnalyzeResult, ASTModule, Path, Type,
};

use super::{nodes::IRModule, types::functions::FunctionTypes, variables::Variables};

pub fn analyze(module: ASTModule) -> AnalyzeResult<()> {
    let types = get_function_types(&module)?;

    let main_path = Path::from(String::from("main"));
    let main = handle_module(module, &types, main_path)?;

    println!("{:#?}", main);

    todo!()
}

fn handle_module(
    module: ASTModule,
    types: &FunctionTypes,
    current_path: Path,
) -> AnalyzeResult<IRModule> {
    println!("{:#?}", types);
    
    todo!()
    // return Ok(ir_module);
}

fn handle_scope(
    types: &FunctionTypes,
    variables: &mut Variables,
    return_type: &Option<Type>,
    body: Vec<ASTNode>,
) -> AnalyzeResult<()> {
    variables.create_state();

    variables.pop_state();
    return Ok(());
}
