use std::collections::HashMap;

use crate::{AnalyzeResult, CompileError};

pub struct Fields {
    fields: HashMap<String, usize>
}
impl Fields {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new()
        }
    }
    pub fn insert(&mut self, key: String, line: usize) -> AnalyzeResult<()> {
        match self.fields.insert(key.clone(), line) {
            Some(defined_line) => {
                return Err(CompileError::new(
                    format!(
                        "{}:{} is already defined on line: {}",
                        &key, line, defined_line
                    ),
                    line,
                ));
            },
            None => return Ok(())
        } 
    }
}