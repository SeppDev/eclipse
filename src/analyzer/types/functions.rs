use std::collections::HashMap;

use crate::{AnalyzeResult, Module, Node, Path, Type};

pub fn get_function_types(module: &Module) -> AnalyzeResult<ModuleTypes> {
    let mut module_types = ModuleTypes::default();

    for (name, (export, submodule)) in &module.submodules {
        let types = get_function_types(submodule)?;

        module_types.submodules.insert(
            name.clone(),
            (export.clone(), types),
        );
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
    pub submodules: HashMap<String, (bool, ModuleTypes)>,
    pub functions: HashMap<String, (bool, Function)>,
}
impl ModuleTypes {
    pub fn get_type(&self, path: Path) -> AnalyzeResult<()> {

        todo!()
    }
}

#[derive(Debug)]
pub struct Function {
    pub parameters: Vec<(String, Type)>,
    pub return_type: Option<Type>,
}
