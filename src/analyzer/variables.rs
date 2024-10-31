use std::collections::HashMap;

use crate::{AnalyzeResult, CompileError, Type};

use super::{Random, RandomString};

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub mutable: bool,
    pub data_type: Option<Type>,
}

#[derive(Debug)]
pub struct Variables {
    random: RandomString,
    states: Vec<Vec<String>>,
    variables: HashMap<String, Variable>,
    parameters: HashMap<String, Variable>,
}
impl Variables {
    pub fn new(parameters: Vec<(String, Type)>) -> Self {
        let mut vars = Self {
            random: RandomString::new(),
            parameters: HashMap::new(),
            states: Vec::new(),
            variables: HashMap::new(),
        };

        for (key, t) in parameters {
            let name = vars.random.generate();
            vars.parameters.insert(
                key,
                Variable {
                    name,
                    mutable: false,
                    data_type: Some(t),
                },
            );
        }

        return vars;
    }

    pub fn insert(
        &mut self,
        key: String,
        mutable: bool,
        data_type: Option<Type>,
    ) -> AnalyzeResult<&Variable> {
        let new_name = self.random.generate();
        let current_state = self.states.last_mut().unwrap();

        match self.parameters.get(&key) {
            Some(_) => panic!("Duplicate key found '{}'", key),
            None => {}
        }

        let result = self.variables.insert(
            key.clone(),
            Variable {
                name: new_name,
                mutable,
                data_type,
            },
        );

        match result {
            Some(_) => {
                return Err(CompileError::new(
                    format!("{:?} is already defined", key),
                    0,
                ))
            }
            None => {}
        }

        current_state.push(key.clone());

        return Ok(self.variables.get(&key).unwrap());
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
    pub fn get(&self, key: &String) -> AnalyzeResult<&Variable> {
        return match self.parameters.get(key) {
            Some(t) => Ok(t),
            None => match self.variables.get(key) {
                Some(var) => Ok(var),
                None => return Err(CompileError::new(format!("{:?} is not defined", key), 0)),
            },
        };
    }
    pub fn change_type(&mut self, key: &String, new_type: Type) -> AnalyzeResult<()> {
        let variable = self.variables.get_mut(key).unwrap();
        variable.data_type = Some(new_type);

        return Ok(());
    }
}
