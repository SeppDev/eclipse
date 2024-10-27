use std::collections::HashMap;

use crate::{ASTModule, AnalyzeResult, Node, Path, Type};

pub fn get_function_types(module: &ASTModule) -> AnalyzeResult<FunctionTypes> {
    let mut function_types = FunctionTypes::default();

    for (name, (export, submodule)) in &module.submodules {
        let types = get_function_types(submodule)?;

        function_types
            .submodules
            .insert(name.clone(), (export.clone(), types));
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

    return Ok(function_types);
}

#[derive(Debug, Default)]
pub struct FunctionTypes {
    submodules: HashMap<String, (bool, FunctionTypes)>,
    functions: HashMap<String, (bool, Function)>,
}
impl FunctionTypes {
    pub fn get_function(&self, relative: &Path, to: Path) -> AnalyzeResult<(Path, Function)> {
        // super, root
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
        let (_, function) = types.functions.get(&name).unwrap();

        return Ok((new_path, function.clone()))
    }

    fn get_relative_function(&self, path: &mut Vec<String>, full_path: &Path) -> &FunctionTypes {
        match path.pop() {
            Some(a) => {
                let (_, types) = match self.submodules.get(&a) {
                    Some(a) => a,
                    None => panic!("Could not find {}", full_path),
                };

                return types.get_relative_function(path, full_path);
            },
            None => return self
        };
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    // pub f_unsafe: bool
    pub parameters: Vec<(String, Type)>,
    pub return_type: Type,
}
