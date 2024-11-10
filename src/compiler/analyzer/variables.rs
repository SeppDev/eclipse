use std::collections::HashMap;

use crate::compiler::types::Type;

#[derive(Debug, Clone)]
pub struct Variable {
    pub mutable: bool,
    pub data_type: Type,
    pub name: String,
}

#[derive(Debug)]
pub struct Variables {
    count: usize,
    states: Vec<Vec<String>>,
    variables: HashMap<String, Variable>,
    parameters: HashMap<String, Variable>,
}
impl Variables {
    pub fn new(parameters: Vec<(String, Type)>) -> Self {
        let mut vars = Self {
            parameters: HashMap::new(),
            states: Vec::new(),
            variables: HashMap::new(),
            count: 0,
        };

        for (key, t) in parameters {
            let name = vars.generate();
            vars.parameters.insert(
                key,
                Variable {
                    name,
                    mutable: false,
                    data_type: t,
                },
            );
        }

        return vars;
    }
    pub fn generate(&mut self) -> String {
        self.count += 1;
        return self.count.to_string();
    }
    pub fn insert(&mut self, key: String, mutable: bool, data_type: Type) -> Result<(), Variable> {
        let name = self.generate();
        let current_state = self.states.last_mut().unwrap();

        match self.parameters.get(&key) {
            Some(_) => panic!("Duplicate key found '{}'", key),
            None => {}
        }

        let result = self.variables.insert(
            key.clone(),
            Variable {
                name,
                mutable,
                data_type,
            },
        );
        self.count += 1;

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
    pub fn pop_state(&mut self) {
        let state = self.states.pop().unwrap();
        for key in state {
            self.variables.remove(&key);
        }
    }
    pub fn get(&self, key: &String) -> Option<&Variable> {
        return match self.parameters.get(key) {
            Some(t) => Some(t),
            None => self.variables.get(key),
        };
    }
}
