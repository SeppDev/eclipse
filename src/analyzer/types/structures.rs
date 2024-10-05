use std::collections::HashMap;

use crate::{
    parser::{Path, Type},
    AnalyzeResult, CompileError,
};

#[derive(Debug)]
pub struct Types {
    functions: HashMap<Path, Function>,
    data_structures: HashMap<Path, DataStructure>,
}
impl Types {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            data_structures: HashMap::new(),
        }
    }
    pub fn push_function(
        &mut self,
        path: Path,
        generics: Vec<String>,
        parameters: Vec<(String, Type)>,
        return_type: Option<Type>,
    ) -> AnalyzeResult<()> {
        let function = Function {
            generics: match generics.len() == 0 {
                true => None,
                false => Some(generics),
            },
            parameters,
            return_type,
        };
        let result = self.functions.insert(path.clone(), function);
        match result {
            Some(_) => return Err(CompileError::new(format!("{:?} is already defined", path.name()), 0)),
            None => {}
        }

        Ok(())
    }
    pub fn get_type(&self, path: &Path) -> AnalyzeResult<&DataStructure> {
        return match self.data_structures.get(path) {
            Some(data) => Ok(data),
            None => Err(CompileError::new(
                format!("Type: {:?} was not found", path),
                0,
            )),
        };
    }
}

#[derive(Debug)]
pub struct Function {
    pub generics: Option<Vec<String>>,
    pub parameters: Vec<(String, Type)>,
    pub return_type: Option<Type>,
}

#[derive(Debug)]
pub struct Enum {}
#[derive(Debug)]
pub struct Struct {}

#[derive(Debug)]
pub enum DataStructure {
    Struct(Struct),
    Enum(Enum),
    Type(Type),
}
