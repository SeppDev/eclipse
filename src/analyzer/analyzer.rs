use std::collections::HashMap;

use crate::{
    analyzer::{nodes::Function, types::functions::get_function_types},
    ASTModule, ASTNode, AnalyzeResult, Node, Path, Type,
};

use super::{
    nodes::{IRModule, IRNode},
    types::functions::FunctionTypes,
    variables::Variables,
};

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

    let mut submodules: HashMap<String, (bool, IRModule)> = HashMap::new();
    let mut functions = Vec::new();

    for node in module.body {
        match node.node {
            Node::Function {
                export,
                is_unsafe,
                name,
                generics,
                parameters,
                return_type,
                body,
            } => {
                let mut variables = Variables::new(parameters.clone());
                let nodes = handle_scope(types, &mut variables, &return_type, body)?;
                let function = Function {
                    parameters,
                    return_type,
                    nodes,
                };
                functions.push(function);
            }
            _ => continue,
        }
    }

    for (name, (export, ast_module)) in module.submodules {
        let mut sub_path = current_path.clone();
        sub_path.add(name.clone());

        let ir_module = handle_module(ast_module, types, sub_path)?;
        submodules.insert(name, (export, ir_module));
    }

    return Ok(IRModule {
        submodules,
        body: functions,
    });
}

fn handle_scope(
    types: &FunctionTypes,
    variables: &mut Variables,
    return_type: &Option<Type>,
    body: Vec<ASTNode>,
) -> AnalyzeResult<Vec<IRNode>> {
    let mut nodes = Vec::new();
    variables.create_state();

    for node in body {
        match node.node {
            Node::DefineVariable {
                mutable,
                name,
                data_type,
                expression,
            } => {
                
            }
            Node::Return(expression) => {}
            _ => continue,
        }
    }
    
    variables.pop_state();
    return Ok(nodes);
}
