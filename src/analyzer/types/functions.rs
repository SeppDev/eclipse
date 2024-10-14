use std::collections::HashMap;

use crate::{AnalyzeResult, Module, Node, Type};

type Function = (&'static Vec<(String, Type)>, &'static Option<Type>);

pub fn get_function_types(module: &Module) -> AnalyzeResult<HashMap<&String, &Function>> {
    // for (name, (public, submodule)) in module.submodules {

    // }
    let mut function_types: HashMap<&String, &Function> = HashMap::new();

    for ast in &module.body {
        match &ast.node {
            #[allow(unused)]
            Node::Function {
                export,
                is_unsafe,
                name,
                generics,
                parameters,
                return_type,
                body,
            } => {
                let function = (parameters, return_type);
                function_types.insert(name, function);
            }
            _ => panic!("Function expected got: {:?}", ast),
        }
    }

    return Ok(function_types);
}
