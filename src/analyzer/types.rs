use std::collections::HashMap;

use crate::parser::Path;

pub struct Types {
    pub function: HashMap<Path, Function>
}
impl Types {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }
    pub fn get(&mut self) {

    } 
}
pub struct Function {

}

pub struct Enum {

}
pub struct Struct {

}