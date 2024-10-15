use std::collections::HashMap;

use crate::{AnalyzeResult, Module, Node, Type};


pub fn get_function_types(module: &Module) -> AnalyzeResult<ModuleTypes> {
    let mut module_types = ModuleTypes::default();
    
    for (name, (_export, submodule)) in &module.submodules {
        module_types
        .submodules
            .insert(name.clone(), get_function_types(submodule)?);
    }

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
                let function = Function {
                    parameters: parameters.clone(),
                    return_type: return_type.clone(),
                };
                module_types
                    .functions
                    .insert(name.clone(), (export.clone(), function));
            }
            Node::Struct {
                export,
                name,
                generics,
                body,
            } => {}
            Node::Enum {
                export,
                name,
                generics,
                body,
            } => {}
            _ => panic!("Function, Struct or Enum expected got: {:#?}", ast),
        }
    }
    
    return Ok(module_types);
}

#[derive(Debug, Default)]
pub struct ModuleTypes {
    pub submodules: HashMap<String, ModuleTypes>,
    pub functions: HashMap<String, (bool, Function)>,
}

#[derive(Debug)]
pub struct Function {
    pub parameters: Vec<(String, Type)>,
    pub return_type: Option<Type>,
}