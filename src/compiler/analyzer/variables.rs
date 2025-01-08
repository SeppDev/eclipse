use std::collections::{HashMap, HashSet};

use crate::compiler::{errors::Location, nodes::hlir};

pub struct Variable {
    location: Location,
    mutable: bool,
    data_type: hlir::Type,
}

#[derive(Default)]
pub struct Variables {
    states: Vec<HashSet<String>>,
    variables: HashMap<String, Variable>,
}
impl Variables {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(
        &mut self,
        name: String,
        mutable: bool,
        data_type: hlir::Type,
        location: Location,
    ) -> Option<Variable> {
        if name == "_" {
            return None;
        }
        
        let variable = Variable {
            location,
            mutable,
            data_type,
        };
        
        return self.variables.insert(name, variable)
    }
    pub fn push_scope(&mut self) {
        self.states.push(HashSet::new());
    }
    pub fn pop_scope(&mut self) {
        let state = self.states.pop().unwrap();
        for key in state {
            self.variables.remove(&key);
        }
    }
    
}
