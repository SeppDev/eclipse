use std::collections::HashMap;

use crate::{Path, Type};

#[derive(Debug, Default)]
pub struct Builder {
    functions: HashMap<Path, Box<ScopeBuilder>>
}
impl Builder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn function(&mut self, path: Path) -> &mut ScopeBuilder {
        let sbuilder = Box::new(ScopeBuilder::new());
        let entry = self.functions.entry(path).or_insert(sbuilder);
        entry.as_mut()
    }
}


#[derive(Debug, Default)]
pub struct ScopeBuilder {
    scopes: Vec<Box<ScopeBuilder>>
}
impl ScopeBuilder {
    fn new() -> Self {
        Self::default()
    }
    pub fn create(&mut self) -> &mut Self {
        let sbuilder = Box::new(ScopeBuilder::new());
        self.scopes.push(sbuilder);
        self.scopes.last_mut().unwrap()
    }
    pub fn define_variable(&mut self, name: String, data_type: Type) {

    }
}

