use std::collections::HashMap;

use crate::{ASTModule, AnalyzeResult, Node, Path, Type};

use super::variables::RandomString;

pub fn get_function_types(
    module: &ASTModule,
    random: &mut RandomString,
) -> AnalyzeResult<ModuleTypes> {
    let mut function_types = ModuleTypes::new();

    for (name, (export, submodule)) in &module.submodules {
        let types = get_function_types(submodule, random)?;

        function_types.submodules.insert(name.clone(), types);
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
                let fname = if module.name == "main" && name == "main" {
                    String::from("main")
                } else {
                    random.generate()
                };

                let function = Function {
                    name: fname,
                    parameters: parameters.clone(),
                    return_type: return_type.clone(),
                };
                function_types.functions.insert(name.clone(), function);
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
    pub name: String,
    pub parameters: Vec<(String, Type)>,
    pub return_type: Type,
}
#[derive(Debug)]
pub enum Types {
    Enum,
    Struct,
}

#[derive(Debug, Default)]
pub struct ModuleTypes {
    types: HashMap<String, Type>,
    submodules: HashMap<String, ModuleTypes>,
    functions: HashMap<String, Function>,
}
impl ModuleTypes {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_function(
        &self,
        relative: &Path,
        to: Path,
        name: &String,
    ) -> AnalyzeResult<&Function> {
        let mut new_path = convert_path(relative, to);
        new_path.components.reverse();

        let mt = match self.get_relative_submodule(&mut new_path.components) {
            Some(mt) => mt,
            None => panic!("Failed to find {}/{}", new_path, name),
        };

        match mt.functions.get(name) {
            Some(f) => Ok(f),
            None => todo!(),
        }
    }
    pub fn get_type(&self, relative: &Path, to: Path, name: &String) -> AnalyzeResult<&Type> {
        let mut new_path = convert_path(relative, to);
        new_path.components.reverse();

        let mt = match self.get_relative_submodule(&mut new_path.components) {
            Some(mt) => mt,
            None => panic!("Failed to find {}/{}", new_path, name),
        };

        match mt.types.get(name) {
            Some(t) => Ok(t),
            None => todo!(),
        }
    }

    fn get_relative_submodule(&self, to: &mut Vec<String>) -> Option<&ModuleTypes> {
        match to.pop() {
            Some(key) => match self.submodules.get(&key) {
                Some(mt) => mt.get_relative_submodule(to),
                None => None,
            },
            None => Some(self),
        }
    }
}

fn convert_path(relative: &Path, to: Path) -> Path {
    let mut new_path = relative.clone();

    for key in &to.components {
        if key == "super" {
            new_path.components.pop();
        } else {
            new_path.add(key.clone());
        }
    }

    return new_path;
}
