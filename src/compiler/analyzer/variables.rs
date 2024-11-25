use std::collections::HashMap;

use crate::compiler::{errors::Location, types::Type};

#[derive(Debug, Clone)]
pub struct Variable {
    pub mutable: bool,
    pub data_type: Type,
    pub name: String,
    pub location: Location,
    pub mutated: bool,
    pub read: bool,
}

#[derive(Debug)]
pub struct Variables {
    count: usize,
    states: Vec<Vec<String>>,
    variables: HashMap<String, Variable>,
}
impl Variables {
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
            variables: HashMap::new(),
            count: 0,
        }
    }
    pub fn increment(&mut self) -> String {
        self.count += 1;
        return self.count.to_string();
    }
    pub fn insert(
        &mut self,
        key: &String,
        mutable: bool,
        data_type: Type,
        location: Location,
    ) -> Result<(), Variable> {
        let name = self.increment();
        let current_state = self.states.last_mut().unwrap();

        let result = self.variables.insert(
            key.clone(),
            Variable {
                location,
                name,
                mutable,
                data_type,
                mutated: false,
                read: false,
            },
        );

        match result {
            Some(var) => return Err(var),
            None => {}
        }
        current_state.push(key.clone());

        return Ok(());
    }
    pub fn create_state(&mut self) {
        self.states.push(Vec::new());
    }
    pub fn pop_state(&mut self) -> Vec<(String, Variable)> {
        let state = self.states.pop().unwrap();
        let mut vars = Vec::new();
        for key in state {
            vars.push((key.clone(), self.variables.remove(&key).unwrap()));
        }
        return vars;
    }
    pub fn get(&mut self, key: &String, mutate: bool) -> Option<&Variable> {
        return match self.variables.get_mut(key) {
            Some(t) => {
                if mutate {
                    t.mutated = true
                } else {
                    t.read = true;
                }
                Some(t)
            }
            None => None,
        };
    }
}
