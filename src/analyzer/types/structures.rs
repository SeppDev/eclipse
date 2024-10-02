use std::collections::HashMap;

use crate::{parser::{Path, Type}, AnalyzeResult, CompileError};

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
    pub fn push_function(&mut self, path: &Path, generics: Vec<String>, parameters: Vec<>) -> AnalyzeResult<()> {
        match self.functions.insert(path, Function {  }) {
            Some(p) => {},
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
pub struct Function {
    pub parameters: Vec<(String, Type)>
}

pub struct Enum {}
pub struct Struct {}

pub enum DataStructure {
    Struct(Struct),
    Enum(Enum),
}
