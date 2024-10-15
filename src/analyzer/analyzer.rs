use crate::{
    analyzer::types::functions::get_function_types, ASTNode, AnalyzeResult, Module, Node, Type,
};

use super::{nodes::Variables, types::functions::ModuleTypes};

pub fn analyze(module: Module) -> AnalyzeResult<()> {
    println!("{:#?}", module);

    let types = get_function_types(&module)?;
    println!("{:#?}", types);

    handle_module(module, &types);

    todo!()
}

fn handle_module(module: Module, types: &ModuleTypes) {
    for ast in module.body {
        match ast.node {
            Node::Function {
                export,
                is_unsafe,
                name,
                generics,
                parameters,
                return_type,
                body,
            } => {
                let mut variables = Variables::new();
                handle_scope(types, &mut variables, &parameters, &return_type, body).unwrap();
            }
            _ => continue,
        }
    }
}

fn handle_scope(
    types: &ModuleTypes,
    variables: &mut Variables,
    parameters: &Vec<(String, Type)>,
    return_type: &Option<Type>,
    body: Vec<ASTNode>,
) -> AnalyzeResult<()> {
    for ast in body {
        match ast.node {
            Node::Scope { is_unsafe, body } => {
                handle_scope(types, variables, parameters, return_type, body)?
            }
            Node::Return(expression) => {

            }
            // Node::SetVariable(, )
            _ => continue,
        }
    }
    return Ok(());
}

