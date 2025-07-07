use std::collections::HashMap;

use common::path::Path;
use syntax::ast::{Parameter, Type};

#[derive(Debug)]
pub struct FunctionType {
    // generics: Vec<String>,
    parameters: Vec<Parameter>,
    return_type: Type,
}

#[derive(Debug, Default)]
pub struct ModuleTypes {
    // pub types: HashMap<String, Type>,
    pub functions: HashMap<String, FunctionType>,
}

#[derive(Debug, Default)]
pub struct Types {
    pub modules: HashMap<Path, ModuleTypes>,
}
impl Types {
    pub fn new() -> Self {
        Self::default()
    }
}
