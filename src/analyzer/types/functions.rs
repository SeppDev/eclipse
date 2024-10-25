use std::{collections::HashMap, path::Components};

use crate::{AnalyzeResult, ASTModule, Node, Path, Type};

pub fn get_function_types(module: &ASTModule) -> AnalyzeResult<FunctionTypes> {
    let mut function_types = FunctionTypes::default();

    for (name, (export, submodule)) in &module.submodules {
        let types = get_function_types(submodule)?;

        function_types.submodules.insert(
            name.clone(),
            (export.clone(), types),
        );
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
                    return_type: return_type.clone()
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
    pub submodules: HashMap<String, (bool, FunctionTypes)>,
    pub functions: HashMap<String, (bool, Function)>,
}
impl FunctionTypes {
    pub fn get_function(&self, at: &Path, to: Path) -> AnalyzeResult<Option<Type>> {
        // super, root
        let mut new_path = at.clone();

        for name in at.components.clone() {
            if name == "super" {
                new_path.components.pop();
            } else {
                new_path.add(name);
            }
        }

        println!("{:?}", new_path);

        todo!()
    }
} 


#[derive(Debug)]
pub struct Function {
    // pub f_unsafe: bool
    pub parameters: Vec<(String, Type)>,
    pub return_type: Type,
}
