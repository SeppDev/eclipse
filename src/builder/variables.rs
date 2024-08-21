use std::collections::HashMap;

use crate::parser::Type;

#[derive(Debug, Clone)]
pub struct Variable {
    pub var_type: Type,
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
    pub fn remove(&mut self, key: &String) {
        match self.variables.remove(key) {
            Some(_variable) => {
                // self.offset -= variable.size;
            }
            None => panic!("'{}' is not defined", key),
        }
    }
    pub fn read(&mut self, key: &String) {
        // return self.variables.get(key).unwrap().offset;
        todo!()
    }
    pub fn write(&mut self, key: &String, var_type: &Type) -> Variable {
        let variable = Variable {
            var_type: var_type.clone(),
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