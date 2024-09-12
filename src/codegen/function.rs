use std::collections::HashMap;

use crate::parser::Type;

#[derive(Debug, Default)]
pub struct FunctionInfo {
    pub stack_size: usize,
    pub variables: HashMap<String, (Type, usize)>
}
impl FunctionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
