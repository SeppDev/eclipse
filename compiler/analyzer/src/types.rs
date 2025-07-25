use std::{collections::HashMap, path::PathBuf};

use syntax::ast::{Parameter, Type, Module};

use crate::Analyzer;

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
    pub modules: HashMap<PathBuf, ModuleTypes>,
}
impl Types {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Analyzer<'_> {
    pub fn get_types(&self, module: &Module) {
        
    }
}