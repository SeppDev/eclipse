use std::collections::HashMap;

use crate::parser::Path;

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
    pub fn get_type(&self, path: &Path) {
        return self.data_structures.;
    }
}
pub struct Function {}

pub struct Enum {}
pub struct Struct {}

pub enum DataStructure {
    Struct(Struct),
    Enum(Enum),
}
