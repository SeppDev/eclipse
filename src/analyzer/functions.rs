use std::collections::HashMap;

use crate::{ASTModule, AnalyzeResult, Node, Path, Type};

pub fn get_function_types(module: &ASTModule) -> AnalyzeResult<ModuleTypes> {
    let mut function_types = ModuleTypes::default();

    for (name, (export, submodule)) in &module.submodules {
        let types = get_function_types(submodule)?;

        function_types
            .submodules
            .insert(name.clone(), types);
    }

    for ast in &module.body {
        match &ast.node {
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
                function_types
                    .functions
                    .insert(name.clone(), function);
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

    return Ok(function_types);
}

#[derive(Debug, Clone)]
pub struct Function {
    // pub f_unsafe: bool
    pub parameters: Vec<(String, Type)>,
    pub return_type: Type,
}
#[derive(Debug)]
pub enum Types {
    Enum,
    Struct
}

#[derive(Debug, Default)]
pub struct ModuleTypes {
    submodules: HashMap<String, ModuleTypes>,
    types: HashMap<String, Type>,
    functions: HashMap<String, Function>,
}
impl ModuleTypes {
    pub fn get_function(&self, relative: &Path, to: Path) -> AnalyzeResult<(Path, Function)> {
        let mut new_path = relative.clone();
        let mut components = to.components.clone();

        for name in components {
            if name == "super" {
                new_path.components.pop();
            } else {
                new_path.add(name);
            }
        }

        let mut find_path = new_path.clone().components;
        find_path.reverse();
        find_path.pop();
        let name = find_path.pop().unwrap();

        let types = self.get_relative_function(&mut find_path, &new_path);
        let function = types.functions.get(&name).unwrap();

        return Ok((new_path, function.clone()))
    }

    fn get_relative_function(&self, path: &mut Vec<String>, full_path: &Path) -> &ModuleTypes {
        match path.pop() {
            Some(a) => {
                let types = match self.submodules.get(&a) {
                    Some(a) => a,
                    None => panic!("Could not find {}", full_path),
                };

                return types.get_relative_function(path, full_path);
            },
            None => return self
        };
    }
}

