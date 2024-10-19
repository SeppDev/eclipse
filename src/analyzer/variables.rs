use std::collections::HashMap;

use crate::{AnalyzeResult, CompileError, Type};

#[derive(Debug, Clone)]
pub struct Variable {
    pub mutable: bool,
    pub data_type: Option<Type>,
}

#[derive(Debug)]
pub struct Variables {
    states: Vec<Vec<String>>,
    variables: HashMap<String, Variable>,
    parameters: Parameters,
}
impl Variables {
    pub fn new(parameters: Vec<(String, Type)>) -> Self {
        Self {
            parameters: Parameters::new(parameters),
            states: Vec::new(),
            variables: HashMap::new(),
        }
    }
    pub fn insert(
        &mut self,
        key: String,
        mutable: bool,
        data_type: Option<Type>,
    ) -> AnalyzeResult<()> {
        let current_state = self.states.last_mut().unwrap();

        let result = self
            .variables
            .insert(key.clone(), Variable { mutable, data_type });

        match result {
            Some(_) => {
                return Err(CompileError::new(
                    format!("{:?} is already defined", key.clone()),
                    0,
                ))
            }
            None => {}
        }

        current_state.push(key);

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
    pub fn get(&self, key: &String) -> AnalyzeResult<Variable> {
        match self.parameters.get(key) {
            Some(t) => {
                let variable = Variable {
                    mutable: false,
                    data_type: Some(t.clone()),
                };
                return Ok(variable);
            }
            None => {}
        }
        return match self.variables.get(key) {
            Some(var) => Ok(var.clone()),
            None => return Err(CompileError::new(format!("{:?} is not defined", key), 0)),
        };
    }
    pub fn change_type(&mut self, key: &String, new_type: Type) -> AnalyzeResult<()> {
        let variable = self.variables.get_mut(key).unwrap();
        variable.data_type = Some(new_type);

        return Ok(());
    }
}

#[derive(Debug)]
struct Parameters {
    values: HashMap<String, Type>,
}
impl Parameters {
    pub fn new(parameters: Vec<(String, Type)>) -> Self {
        let mut map = HashMap::new();
        for (key, data_type) in parameters {
            map.insert(key, data_type);
        }

        Self { values: map }
    }
    pub fn get(&self, name: &String) -> Option<&Type> {
        return self.values.get(name);
    }
}
