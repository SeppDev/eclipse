use std::collections::HashMap;

use crate::parser::node::{type_size, Type};

#[derive(Debug, Clone)]
pub struct Variable {
    pub offset: usize,
    pub var_type: Type,
    pub size: usize,
}

pub struct Variables {
    offset: usize,
    variables: HashMap<String, Variable>,
}
impl Variables {
    pub fn new() -> Self {
        Self {
            offset: 0,
            variables: HashMap::new(),
        }
    }
    pub fn get(&mut self, key: &String) -> usize {
        return self.variables.get(key).unwrap().offset;
    }
    pub fn remove(&mut self, key: &String) {
        match self.variables.remove(key) {
            Some(_variable) => {
                // self.offset -= variable.size;
            }
            None => panic!("'{}' is not defined", key),
        }
    }
    pub fn create(&mut self, key: &String, var_type: &Type) -> Variable {
        let size = type_size(var_type);
        self.offset += size;

        let variable = Variable {
            var_type: var_type.clone(),
            offset: self.offset,
            size: size.clone(),
        };

        match self.variables.insert(
            key.clone(),
            variable.clone(),
        ) {
            Some(_) => panic!("'{}' is already defined", key),
            None => return variable,
        }
    }
}
